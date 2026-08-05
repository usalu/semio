# 📋️ Per-plugin migration template

The ordered recipe for merging ONE plugin's 7-crate constitutional layout into ONE crate with a taxonomy
source tree. Written by the W1 pilot (🌊️flow, ticket
`26/08/05/FLOW-PLUGIN-PILOT-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`) from what actually happened,
not from the plan's projection. Read the master doc's **Discovery contract** and **Registrar Protocol**
sections first; this file is the how, they are the what.

Read 🌊️flow as the worked example while following this — it is the simplest case (one app, one artifact),
so anything it does NOT cover (multi-app, multi-artifact, a `🫀️core` cross-artifact kernel, a
`🦀️wasm.rs` bridge) is flagged inline below.

---

## 0. Before you touch anything

1. **Open your own ticket** (`ticket_open`, goal `🎯aioptimizedrepo`). Scratch files go in your ticket
   folder, never in the plugin tree.
2. **Inventory the plugin.** `find ✏️s/🔌️plugins/<p> -name Cargo.toml` and
   `find ✏️s/🔌️plugins/<p> -name '*.rs' | xargs wc -l | sort -n`. Note which crates exist — not every
   plugin has all seven, and some have extra `🔨️modules/*` or `🧩️extensions/*` crates.
3. **Record the test baseline.** Run `cargo test -p <each old crate> --lib` and write down the per-crate
   pass counts. That total is your correctness floor: every one of those tests must still exist and pass
   in the merged crate.
4. **Capture the wire baseline (do NOT skip this).** Temporarily add a test to the old `📡️protocol`
   crate that constructs one value per `<X>Command` variant and prints
   `print_op(&c) | bytes.len() | hex(bytes)`; run it with `-- --nocapture`, save the output into your
   ticket folder. After the merge, run the equivalent test on the new crate and `diff` the two. This is
   the ONLY check that proves the command decomposition did not silently rewrite the wire format — the
   round-trip laws are self-consistent and will happily pass on a changed format. The pilot's
   `🧪️wire-baseline-before.txt` / `🧪️wire-after.txt` show the shape.
5. **Find external dependents.** `grep -rn "<old-crate-name>" --include=Cargo.toml .` — anything outside
   your plugin dir is a cross-cutting edit you must plan for (see §8).

---

## 1. Create the package files

```
✏️s/🔌️plugins/<p>/📦️packages/🦀️rust/{Cargo.toml, 📋️project.json, 📜️script.ts}
```

`📜️script.ts` — copy the pilot's; it is 13 lines and only names the crate:

```ts
#!/usr/bin/env bun
/** 🌊️ `@semio-tech/<p>-plugin` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-<p>"], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
```

`📋️project.json` — name `@semio-tech/<p>-plugin`; **all four leveled test targets**
(`test`/`test-quick`/`test-long`/`test-exhaustive`) or root `verify gate`'s `checkLeveledTestTargets`
fails; `options.cwd` = the package dir; and a `namedInputs.default` override listing
`{workspaceRoot}/✏️s/🔌️plugins/<p>/**/*.rs` — the sources now live OUTSIDE `projectRoot`, so without it
nx caches stale results.

---

## 2. Write `📦️lib.rs` at the plugin root — the `#[path]` mechanics

This is the part that eats a day if you guess. Verified empirically by the pilot:

* A `#[path]` on a module declared at the **top level of a file** resolves relative to that file's
  directory.
* A `#[path]` on a module declared **inside an inline `mod` block** resolves relative to
  `<file dir>/<inline mod name>/…` — the inline module's own name is spliced into the base. Nesting
  `pub mod artifacts { pub mod flow { #[path = "🗿️artifacts/🌊️flow/…"] … } }` therefore looks for
  `artifacts/flow/🗿️artifacts/🌊️flow/…` and hard-errors with
  `couldn't read artifacts/./🗿️artifacts/…: No such file or directory`.
* **`#[path = "."]` on the inline module resets that base back to the file's own directory**, and it
  composes at any depth.

So: put `#[path = "."]` on **every grouping module**, and a **full path from the plugin root** on every
leaf. The full paths are not cosmetic — the taxonomy validator does a literal
`join(pluginRoot, <#[path] value>)` and reports both "component file not declared" and "declared path
does not exist" if you use relative-to-parent paths.

```rust
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod flow {
        #[path = "🗿️artifacts/🌊️flow/🦀️component.rs"]
        mod component;
        pub use component::*;                       // the node's own file IS the module's contents

        #[path = "🗿️artifacts/🌊️flow/🔧️op/🦀️component.rs"]
        pub mod op;
        // … 🔺️diff 🗣️dsl 🎒️pack 📡️spr ⚙️engine
    }
}
```

Consequences of the `mod component; pub use component::*;` idiom, all of which you will hit:

* The component file is a **child** module, so siblings are never in scope — every cross-node reference
  is an absolute `crate::apps::<app>::…` / `crate::artifacts::<a>::…` path. Write them that way from the
  start; `super::` mostly does not mean what you want.
* Anything a sibling node needs must be `pub` — including `app_labels!`'s struct, which defaults to
  private (`app_labels! { pub struct XLabels { … } }`; the macro already takes a `$vis`).

`📦️lib.rs` ends with the `semio_plugin!{}` invocation and contains **no logic at all** — the
`TaxonomyLibShape` lint and the taxonomy validator both treat anything else as an inlining regression
(see the master doc's Single-File-Repo hazard ruling).

---

## 3. Cargo.toml — the union recipe

```toml
[package]
name = "semio-s-plugin-<p>"          # PRESERVE the old bundle crate's name — component identity,
version.workspace = true             # registry rows, launch entries and wasm filenames all key on it
edition.workspace = true
rust-version.workspace = true
description = "…"

[lints]
workspace = true

[package.metadata.component]         # ← copied VERBATIM from the old bundle crate
package = "semio:<p>"

[package.metadata.semio]
role = "plugin"                      # ← the ONE new key (discovery contract). Additive: keep every
                                     #   existing key in this table (contributes/consumes/host/assets)
[[package.metadata.semio.playground]]
…                                    # ← verbatim, ports UNCHANGED (so launch.json regen is a no-op)

[lib]
crate-type = ["cdylib", "rlib"]
path = "../../📦️lib.rs"

[dependencies]
# union of all old crates' deps, five `../` up to the repo root, intra-plugin path deps DELETED
# (they are `crate::` paths now) and ALL dev-dependencies deleted (the dev-dep cycle the old 7-crate
# split needed is gone — tests are plain `crate::` references)
…
semio-framework-plugin = { path = "…", features = ["component-guest"], package = "semio-framework-plugin" }
serde.workspace = true
serde_json.workspace = true
```

* `[target.'cfg(…)'.dependencies]` tables: union them verbatim.
* `[workspace.dependencies]` adoption: use it for `version`/`edition`/`rust-version` and for deps whose
  key equals the crate name (`serde`, `serde_json`). Do **not** use it for the renamed internal path deps
  (`flow_core = { package = "semio-framework-os-kernel-flow-core", default-features = false }`) — the
  rename + `default-features = false` + workspace-inheritance combination is a trap, and the root table's
  paths are repo-root-relative which will not match while you verify in isolation.

### The chicken-and-egg, and how to verify anyway

The new crate is **not a workspace member** until the registrar edits root `Cargo.toml`, and it cannot be
added early because the OLD bundle crate with the same package name is still a member. Cargo refuses to
build a manifest that sits inside a workspace without being one of its members.

The pilot's solution: append a clearly-marked **temporary `[workspace]` overlay** to the new
`Cargo.toml`, making it its own workspace root for the duration of verification, then delete the overlay
at handoff. Copy `[workspace.package]`, the `[workspace.dependencies]` rows you use,
`[workspace.lints.rust]`/`[workspace.lints.clippy]` and the `[profile.dev*]` blocks **verbatim from root
`Cargo.toml`** so artifact fingerprints match and the shared `target/` stays warm — **also copy the
`cargo-features = ["trim-paths"]` line that sits above `[workspace]` in root Cargo.toml** (found by
gis's agent): omit it and cargo hard-errors before doing anything, since the profiles below reference a
feature the manifest never declared. Then everything runs via `--manifest-path`:

```
cargo check   --manifest-path ✏️s/🔌️plugins/<p>/📦️packages/🦀️rust/Cargo.toml
cargo check   --manifest-path … --target wasm32-wasip2
cargo clippy  --manifest-path … --all-targets -- -D warnings
cargo test    --manifest-path …
cargo build   --manifest-path … --target wasm32-wasip2
```

At handoff: delete the overlay block, and delete the `target/` and `Cargo.lock` the nested workspace
created inside `📦️packages/🦀️rust/`.

---

## 4. Move the code — what goes where

| From (old crate / region) | To |
|---|---|
| bundle `🛂️manifest/🗿️artifact` lib.rs | plugin-root `📦️lib.rs` (`semio_plugin!` only); its `register_*_exports()` becomes artifact `⚙️engine::register()` |
| app facade crate (`FlowFixture`/schema re-exports) | `🗿️artifacts/<a>/🦀️component.rs` + a new `artifact_kind()` fn lifted out of the manifest's `.artifact_kind(…)` |
| `⚙️engine` — document-side compute | `🗿️artifacts/<a>/⚙️engine/🦀️component.rs` |
| `⚙️engine` — the `Config` struct | app `🦀️config.rs` (view state, not document) |
| `🔧️op` — `*Diff` structs + `OperationDiff` impls | `🗿️artifacts/<a>/🔺️diff/🦀️component.rs` |
| `🔧️op` — `Operation` enum + apply/backwards | `🗿️artifacts/<a>/🔧️op/🦀️component.rs` |
| `🔧️op` — `ConfigOperation` | app `🦀️config.rs` (next to the `Config` it patches) |
| `🗣️dsl`, `🎒️pack` | `🗿️artifacts/<a>/{🗣️dsl,🎒️pack}/🦀️component.rs`, verbatim |
| `📡️protocol` — `encode_op`/`decode_op` | `🗿️artifacts/<a>/📡️spr/🦀️component.rs` (**renamed**; no `📡️protocol` segment may survive) |
| `📡️protocol` — the `Command` enum | **rebuilt** by `app_commands!` in app `🦀️component.rs` — see §5 |
| `🖱️ui` Constants | the node that owns them (a window's body/surface keys go in that window's file; app-wide ids stay in app `🦀️component.rs`) |
| `🖱️ui` Locale/Terminology | app `🦀️terminology.rs` — ONE `app_labels!` block, never split |
| `🖱️ui` DocumentHelpers | **one consumer → that consumer's file; two or more → artifact `⚙️engine`, UNLESS the helper takes an app-only view-state type (e.g. `<X>Config`) as a parameter — then it stays at app level (`🦀️component.rs` or a shared app-level helper file) no matter how many app-level consumers it has, because artifacts must never depend on apps.** State the rule in the engine's module doc so the next reader knows where to add |
| `🖱️ui` Panels (`build_*_tree`) | `📌️panels/<panel>/🦀️component.rs`, each exporting `definition() -> PanelTabDefinition` + `render(…) -> UiNode` |
| `🖱️ui` WindowMeasures | `🎭️modes/<m>/🪟️windows/<w>/🎚️options/<o>/🦀️component.rs`, one file per `WindowMeasure`/`Group` root, each exporting `measure(config, labels)` |
| `🖱️ui` Render (`render_*`) | the matching window's `render()` |
| `🖱️ui` PlayApp + `DocumentApp` impl | app `🦀️component.rs`, **dispatch-only** |
| `🖱️ui` Manifest builder chain | app `🦀️component.rs`'s slim `🔖️Manifest` region — see §6 |
| `🖱️ui` WasmBridge (if present) | app `🦀️wasm.rs` |
| `🖱️ui` Tests | split per node (§7) |
| `🧩️extensions/*` | **untouched** |
| plugin-level TS-only `🔨️modules/*` | leave in place (this migration is the Rust crate merge only) |

Adopt-as-you-go improvements the pilot made and you should copy: extract a `<verb>_operations()` helper
next to a command when two commands share it (`reorganize` and the `auto-layout` extension effect), and
give each window a `window_measures()` that *collects* from its `🎚️options/*` rather than re-listing them.

---

## 5. Command decomposition with `app_commands!`

Read the macro in `semio-framework-plugin` (`//#region 🔖️AppCommands`) — it has three arms. Use the
**keyed** form, and add `, ctx = <T>` if your handlers need app-struct state:

```rust
semio_framework_plugin::app_commands! {
    /// doc comment (the macro forwards `$(#[$meta])*` onto the generated enum)
    pub enum XCommand for XProjection, XOperation, XConfig, XConfigOperation, ctx = XSession {
        "addWidget" as "add-widget" => add_widget::AddWidget,
        …
    }
}
```

Rules, each of which cost the pilot a debugging cycle:

1. **Two literals per row.** `"commandId"` is the camelCase manifest ACTION id (`command_id()`, what
   `.operation()`/`.view_action()` declared); `"wire-key"` is the kebab `#[dsl(key = …)]` the codec uses.
   They are different vocabularies. Copy each from the old enum's `#[dsl(key)]` attribute and the old
   `command_id()` match arm respectively — do NOT derive one from the other; flow has
   `"setLocale" as "locale"`.
2. **Row order is the binary variant ordinal.** Keep the old enum's order exactly. Appending is safe;
   reordering is a format break that no round-trip test will catch.
3. **Every payload struct needs `#[dsl(keyword = "<same as the `as` literal>")]`.** A single-field tuple
   variant delegates its whole `RecordSpec` to the inner type, and a `DslRecord` struct's keyword defaults
   to `None` — without the attribute the op prints with no leading keyword at all and fails to re-parse
   (`unknown operation line 'kind=inputSlider x=10'`). This is the single highest-value line in this file.
4. **Field names and order are the record shape.** Copy them verbatim off the old variant.
5. **Unit variants become fieldless structs** (`pub struct DeleteSelection {}`). Verified: the DSL text
   and binary bytes are identical to the unit variant's (`01 <ordinal> 00 00`). Serde's JSON shape does
   change (`"DeleteSelection"` → `{"DeleteSelection":{}}`); flow's Command serde is not used on any wire
   (the codec is `dsl::DslOps`), but check your plugin before assuming the same.
6. **One module per payload, addressed by a single ident.** The macro takes `$module:ident::$Payload`, so
   group several payloads per component file as nested `pub mod`s and `use` them flat at the invocation
   site. Flow's 41 commands live in 13 files this way, not 41.
7. **Avoid `commands::<group>::<same-name>`** — clippy's `module_inception` fires. Flow renamed the
   `evaluate`/`reorganize` groups to `eval`/`layout`.
8. **Handler signature** is uniform:
   `pub fn handle(payload: &P, doc: &DocumentView<'_, Proj>, cfg: &ConfigView<'_, Cfg>, ctx: &mut Ctx) -> Result<Emit<Op, CfgOp>, Fault>`
   with the old match arm's body moved in verbatim. `&mut Ctx` reborrows to `&Ctx` at call sites that
   only read.
9. `DocumentApp::handle` then collapses to two lines (`lock the ctx; command.dispatch(doc, cfg, &mut ctx)`)
   and `command_id` to one (`command.command_id()`).

### 5.1 When `app_commands!` does NOT apply — untyped-payload command enums

Found by 🧩️puzzle: some plugins' `Command` enum is generated by a local `macro_rules!` where **every**
variant has the identical shape `{ window_id: Option<String>, args: Option<serde_json::Value> }` and a
hand-written `serde_json`-based `OpBinary` — the args are opaque JSON, not a typed-per-variant DSL
record. `app_commands!` assumes one Rust type per row with a real `#[dsl(keyword)]`; forcing an untyped
`Value` payload through it would silently rewrite the wire format (exactly the failure mode §0.4's
wire-baseline check exists to catch — run that check before concluding you can use the macro at all).

If you hit this shape: **do not use `app_commands!`. Keep the existing enum/macro and its variant list
byte-identical** (still one file per command-GROUP under `🎮️commands/<group>/🦀️component.rs`, each
holding its own slice of the match arms as `handle(...)` functions) — you are only decomposing
`handle_action_impl`'s match body across files, not rebuilding the enum. `DocumentApp::handle` stays a
dispatch match calling into each group's `handle`, same shape as before the migration, just relocated.

---

## 6. The manifest stitch

Each taxonomy node exports its own `definition()`; app `🦀️component.rs` keeps a slim `🔖️Manifest` region
that calls one passthrough per node:

```rust
App::builder(APP_ID, LocalizedLabel::native(…))
    .artifact_kind(crate::artifacts::<a>::artifact_kind())
    .mode_def(edit::definition())
    .window_kind_def(main::definition())
    .panel_tab_def(panels::document::definition())
    .default_layout(edit::layout())
    .named_layout(generate::layout())
    …
```

The scalar leaf declarations that have no `_def` passthrough (`.operation()`, `.view_action()`,
`.action_with()`, `.action_args()`, `.keybinding()`, `.config()`) stay written out inline — that is
intended, not a gap. Note `WindowKindDefinition.options.measures` stays **empty**: measures are
config-derived per frame by `DocumentApp::window_measures`, never frozen into the manifest.

---

## 7. Tests

Every component file gets its own `//#region 🧪️Tests` covering just that node. Cross-cutting tests (full
manifest build, undo/redo round trip, multi-instance convergence, unknown-body-key fallback, the
whole-command-surface laws) stay in app `🦀️component.rs`.

Add a `#[cfg(test)] pub(crate) mod testkit` to app `🦀️component.rs` first — every other node's tests need
it and will otherwise each re-derive the harness:

```rust
pub type XApp = VcsDocumentApp<XPlayApp>;
pub fn app() -> XApp                      // new_app: no registry
pub fn app_with_registry() -> XApp        // new_app_with_registry(create_x_app): enforces kind discipline
pub fn dispatch(app: &mut XApp, c: XCommand) -> InvocationResult
pub fn render(app: &mut XApp, body_key: &str) -> String
pub fn main_window_measures(app: &mut XApp) -> Vec<WindowMeasure>
```

`VcsDocumentApp` exposes `projection()` but **no config accessor** — assert config-only effects through
`render(…)` output or `window_measures()`, the way the pre-migration tests already did. Empty vectors are
often `skip_serializing_if`-omitted, so assert `!contains("\"selection\":[\"id\"]")` rather than
`contains("\"selection\":[]")`.

Don't probe selection/config state by grepping rendered-scene substrings — a feature/pin id appears in
the scene descriptor whether or not it's selected, so that test passes for the wrong reason. Call the
payload's `handle` directly with a constructed `DocumentView`/`ConfigView` and assert on the returned
`Emit`'s `config_operations` instead. Also note `InvocationResult::operations` is type-erased
(`KernelOperation`) — it cannot be pattern-matched against your app's own `Operation` enum, so route
around it the same way (found by gis's agent).

For asserting a manifest's declared actions actually reach a command handler, use the framework's own
`testkit::assert_declared_actions_bridge_to_commands::<A>(manifest)` rather than hand-rolling the loop —
it already knows which framework-injected action ids to skip (`recordTutorial`, `startIntroduction`,
`setActiveUtility`, …), which are `ActionKind::View` like some of your own actions so they can't be
filtered out by kind alone. Same trap for panel-tab counts: the framework injects its own tabs, so assert
your tabs are *present*, never pin a total count (found by gis's agent, after gis3d-ui's entire
`{action,args}` host wire turned out to be silently dead — see §12 below).

Keep these permanent wire guards (both are pilot additions that paid for themselves immediately):

* `every_command()` returning one value per row, feeding an `assert_op_text_binary_equivalence` loop and
  a "printed line starts with the row's wire keyword" assertion;
* an `optional_field_rows_keep_their_pre_migration_bytes` test pinning the exact hex for rows whose
  `Option` fields make `None`/`Some` distinct wire cases (copy the hex out of your §0.4 baseline).

Delete the temporary `[DEBUG]` dump test once the diff is clean.

---

## 8. Delete the old crates — and only then

Order matters. Steps 1–4 of §9 must be green on the NEW crate first, because the moment you delete the
old crate directories the root workspace goes red (its member lines dangle) and you can no longer run any
root-level cargo command.

1. `rm -rf` the old `🛂️manifest/`, `🎛️apps/<app>/⚡️implementations/`, `🎛️apps/<app>/🔨️modules/`.
   Relocate any stray data file first (flow had an orphan `🛂️manifest.jsondag.manifest.json`, moved to
   `🗿️artifacts/<a>/🛂️manifest.json`).
2. Fix external dependents you found in §0.5. The repo-wide one is
   `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/…/Cargo.toml`, which dev-depends on
   **every** plugin's app facade crate — every W2 agent will hit it. Prefer repointing the alias at the
   real owner of the type (for flow: the kernel crate that actually implements `DocumentDsl`), which is a
   one-line change with no `lib.rs` ripple; only repoint it at the new plugin crate if the type genuinely
   lives there, and then fix the `use` path too.
3. Remove the temporary `[workspace]` overlay and the nested `target/`+`Cargo.lock` (§3).
4. Report the registrar handoff (§10) and stop. Do not try to "fix" the dangling members yourself.

---

## 9. Verification sequence

| # | Command | Notes |
|---|---|---|
| 1 | `cargo check --manifest-path <new>/Cargo.toml` | must be warning-free **for your crate** — dependency-crate warnings are pre-existing |
| 2 | `cargo check --manifest-path … --target wasm32-wasip2` | |
| 3 | `cargo clippy --manifest-path … --all-targets -- -D warnings` | see the known-findings list below |
| 4 | `cargo test --manifest-path …` | ≥ your §0.3 baseline count, all green |
| 4b | wire diff against the §0.4 baseline | the real correctness proof |
| 4c | your ticket's copy of the taxonomy audit | mirrors `validateTaxonomyTree`; the real `registry check` bails on staleness until the registrar regenerates |
| 5 | delete old crates (§8) | only now |
| 6 | `cargo build --manifest-path … --target wasm32-wasip2` then `bunx @bytecodealliance/jco transpile <artifact> -o <tmp> --name <base>_component --map "semio:framework/host=./🟨️host-shim.js"` | the manual equivalent of `framework-os-dev:plugin -- <p>`, which needs a healthy workspace + regenerated registry and therefore only runs after the registrar |
| 7 | *(post-registrar)* `bun 🧰️framework/…/📇️registry/📜️script.ts check`, `bun nx run @semio-tech/framework-os-dev:plugin -- <p>`, `bun ./📜️script.ts dev <p>`, `bun ./📜️script.ts verify gate` | orchestrator/registrar gate, not yours |

### Clippy findings the split itself creates

* `result_large_err` × one per handler. The old code had a single `DocumentApp::handle`, and the lint
  skips trait impls; free-function handlers are not skipped. The error type is framework-owned (`Fault`)
  and the signature is fixed by the trait, so a documented crate-level
  `#![allow(clippy::result_large_err)]` in `📦️lib.rs` is the right answer.
* `module_inception` — rename the group module (§5.7).
* `unused_qualifications` — inside a `pub mod x { use super::*; }` payload module, call the parent's
  helpers unqualified; `super::helper(…)` now lints.
* Pre-existing `map_unwrap_or` / `redundant_clone` findings ride along with the moved code and must be
  fixed (they are mechanical: `map_or`, `map_or_else`, `is_some_and`, dropping a dead clone) because
  `-D warnings` is the gate.

---

## 10. Registrar handoff — what you must NOT touch

**Never edit**, no matter what your migration seems to need:

* root `Cargo.toml` / `Cargo.lock`
* root `📜️script.ts`
* `🧰️framework/…/📇️registry/📜️script.ts` **and its `🤖️generated/` output**
* `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`
* `.dependency-cruiser.cjs`, `nx.json`, `🧪️vitest.config.ts`, `.storybook/scopes.ts`
* any other plugin's directory

Instead, end your report with a **registrar handoff** block stating literally:

```
Remove these member lines from root Cargo.toml:
    "✏️s/🔌️plugins/<p>/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/<p>/🎛️apps/<app>/⚡️implementations/🦀️rust",
    … one per old module crate …
Add:
    "✏️s/🔌️plugins/<p>/📦️packages/🦀️rust",
Also remove from [workspace.dependencies] (if present):
    <old-crate-name> = { path = "…" }
```

plus: any cross-cutting file you did edit (§8.2), any framework gap you fixed, and the exact commands
that are still un-run because they need a healthy workspace. The registrar then prunes the members, runs
`cargo metadata`, and regenerates the registry + `launch.json` in one serialized pass.

---

## 11. Framework gaps the pilot found and fixed

All in `semio-framework-plugin`; additive, existing behavior untouched. If a later plugin needs more,
extend in the same style and record it here.

1. **`app_commands!` keyed rows** (`"id" as "wire-key" => mod::Payload`). The original single-literal
   arm conflates the manifest action id with the wire keyword and silently rewrites the format of any
   existing app it is applied to.
2. **`app_commands!` dispatch context** (`, ctx = <T>`), for handlers needing app-struct state that is
   neither document nor config (flow's `Mutex<FlowEvalSession>`).
3. **`app_commands!` attribute passthrough** (`$(#[$meta:meta])*`) — the original arm rejected a doc
   comment on the generated enum.
4. **`testkit::assert_constitutional_crates` is now taxonomy-aware.** `semio_plugin!` generates a
   sanity test that asserted the old seven per-app crate slots; a migrated plugin has none of them and
   the test fails at the very end of an otherwise-green merge. It now detects the new shape by the same
   marker the discovery contract uses (a plugin-root `📦️lib.rs` beside `🗿️artifacts/`) and checks the
   taxonomy components instead — the Rust twin of `validateTaxonomyTree`. Both shapes pass while the
   migration is in flight.
5. **Registry `validateConstitutionalCrates` now exempts migrated plugins.** The orchestrator found this
   one right after the flow registrar step: the legacy gate walked every plugin's `🎛️apps/` dir looking
   for the seven old crate slots, and correctly-but-uselessly flagged a freshly-migrated plugin as
   "missing all seven" — it isn't missing anything, it moved on. Fixed by passing the set of
   `findNewContractPluginRoots` plugin ids into `validateConstitutionalCrates` and skipping them; those
   plugins are validated by `validateTaxonomyTree` instead. If `bun …/📇️registry/📜️script.ts check`
   reports your plugin as missing constitutional slots right after you finish, that fix has already
   landed — pull the current file, don't reimplement it.

---

## 12. Batch-1 findings (from cad, vcs, shooting, sourcing — read before you start)

1. **NEVER background your own verification commands.** Several batch-1 agents ran `cargo check`/`clippy`/
   `test` with `run_in_background: true` and then stopped, waiting — and were never woken up, because that
   auto-resume mechanism only exists for the main orchestrating session, not for a subagent. If you
   background a long cargo command, your task silently stalls forever until the orchestrator notices and
   resends you a message. **Always run cargo/verification commands as plain synchronous Bash calls.** If a
   cold build is slow, raise the `timeout` parameter (e.g. 300000–600000ms) instead of backgrounding it.
2. **Shared options across multiple windows in one mode: one `🎚️options` set at the mode level, not one
   per window.** cad has 4 windows in its one edit mode that all expose the same 3 option kinds
   (projection/sun/dislocate). Duplicating them under every `🪟️window/🎚️options/` is needless — put them
   once at `🎭️modes/<mode>/🎚️options/<option>/🦀️component.rs` and have each window's `definition()` bind
   the same option ids. `validateTaxonomyTree` only constrains what's *directly under* a window dir, so
   this passes cleanly; it also passed the root `📜️script.ts` taxonomy lints as-is. Apply this to any
   plugin with more than one window sharing option kinds (watch for it in puzzle, gis, norm).
3. **A pre-existing, repo-wide broken commit affects some old ui crates.** cad's old `…-app-cad-ui` crate
   was flat-out non-compiling at HEAD (`fa51b5c82f`) — an automated repo-wide edit had wrapped `handle`'s
   match arms in `Ok(` without closing the paren, left some `Emit::amend_config` arms unwrapped, and never
   imported `Fault`. vcs and shooting independently hit the *same* corruption pattern in their own old ui
   crates. If your plugin's old ui crate fails to even `cargo check` before you've changed anything, this
   is very likely why — read the `handle` match arms for unbalanced `Ok(`/missing `Fault` imports before
   assuming you introduced a bug. Fixing it as part of the port (which is what step 4 of §0's baseline
   naturally forces you to do, since you can't get a passing baseline otherwise) is correct; just call it
   out plainly in your final report so the pattern's spread across plugins is visible to the orchestrator.
4a. **Shared options can live one level higher than the mode, when no single mode owns both sharing
   windows.** cad's pattern (§12.2) put shared options at `🎭️modes/<mode>/🎚️options/` because one mode
   owned all the windows sharing them. 💠️lowpoly hit the next case up: its Model window is reused
   byte-identically by two different modes (`edit` and `paint`), so no single mode can own the shared
   `🎚️options`/engagement — they moved to app-level `🎚️options/` instead. General rule: put shared
   options at the shallowest taxonomy node that is a common ancestor of every window sharing them
   (window own level → mode → app), never duplicate them per window.
4. **Cross-plugin test-only dependents aren't just demonstrator.** cad's agent found that 💠️lowpoly's
   engine crate `dev-depends` on `semio-s-app-cad-engine` for one test module — nothing in the plan or
   TEMPLATE flagged this. Do the §0 step 5 dependent-search thoroughly (including `[dev-dependencies]`,
   not just `[dependencies]`) — a hit outside your plugin dir is still yours to report, even if it's just
   a test-only dependency that won't break the dependent's `lib` build, only its `cargo test`.

---

## 13. Scope-extension addendum (from the approved plan `the-codebase-still-has-spicy-umbrella.md`)

The initiative's target has grown to *zero* `⚡️implementations` dirs anywhere in the repo, with an
upcoming os-kernel merge (`store`/`protocol`→`spr`/`dsl`/`pack`/…) that has **313 dependents on `store`
alone**. Two mandates every plugin agent from here on must follow so that merge stays cheap:

1. **Adopt `{ workspace = true }` for every dependency already listed in root `[workspace.dependencies]`**,
   not just the plugin's own name-key deps. When your new crate's `[dependencies]` table lists a framework
   crate (`semio-framework-os-kernel-*`, `semio-framework-plugin`, `semio-framework-core`, …) by an
   explicit `path = "…"`, check whether root `[workspace.dependencies]` already has an entry for it (or the
   crate it's about to be renamed to) and use `dep = { workspace = true }` instead. This is opt-in per the
   Registrar Protocol (requires your crate to already be a workspace member, so in practice happens in the
   SAME registrar pass that adds your member line, or right after) — but **write your `Cargo.toml` in the
   workspace-true form from the start** so the registrar's job is a rename, not a rewrite. The payoff: when
   the kernel merge lands, its ~313 dependents become ~10 root-file edits instead of ~300 scattered
   per-crate edits.
2. **Single-crate plugins are a fast path, not a special case.** 🔋️energy is already one crate on disk —
   its "migration" is a taxonomy-tree reshape (extract component files, write `📦️lib.rs` wiring, add the
   `role = "plugin"` metadata key, move to `📦️packages/🦀️rust`) with no multi-crate union step. Don't
   over-apply the 7-crate union recipe (§3) where there's nothing to union.
3. **Shared-cache test-serialization lesson (from 🌀️procedural).** Running `cargo test` for your new crate
   while other plugins' cargo processes are live can produce spurious incremental-compilation races on the
   shared `target/` dir. If a test run fails with a linker or rmeta error that doesn't reproduce on retry,
   re-run once before treating it as a real regression — it is very likely lock contention, not your code.
4. **`workspace = true` + a renamed `package = "…"` does NOT work** (confirmed empirically by ➗️mathematical,
   matches the framework prerequisite's own doc comment): Cargo resolves workspace-dependency inheritance by
   the LOCAL key, and a `package =` override does not redirect that lookup — `dsl = { workspace = true,
   package = "semio-framework-os-kernel-dsl" }` errors with "dependency.dsl was not found in
   workspace.dependencies". Only use `{ workspace = true }` when your local dependency key is spelled
   IDENTICALLY to the root `[workspace.dependencies]` key (e.g. `serde`, `serde_json`,
   `semio-framework-core`, `semio-framework-plugin`). Every renamed/aliased internal dep
   (`dsl`/`store`/`protocol`/`mathematical_graph`/…) stays a plain `path =` + `package =` pair — this is not
   a workaround to fix later, it is the correct permanent form.
5. **`app_commands!` forces `Serialize`/`Deserialize` on the generated Command enum** (from ➗️mathematical) —
   if any payload type nested inside your command enum wraps a framework `dsl::Wire`-family type that does
   NOT implement `Serialize`/`Deserialize` (some don't, by design), the macro-generated `#[derive]` will fail
   to compile. Fix by hand-rolling `Serialize`/`Deserialize` for that inner type entirely within your own
   plugin files (do not modify the framework `dsl::Wire` type or the `app_commands!` macro itself for this —
   it's a one-off payload-shape mismatch, not a framework bug).
