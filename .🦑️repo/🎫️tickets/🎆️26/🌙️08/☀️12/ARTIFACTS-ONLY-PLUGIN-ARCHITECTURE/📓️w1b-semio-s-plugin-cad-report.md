# W1b — `semio-s-plugin-cad` → `.artifact(declaration())` conversion

`apa-status: partial` — mechanism steps (1–2) landed and are compiler-verified false-positive-free
by the `#[path]`/`include_str!` resolution scripts below, `cargo metadata` is clean, and a real
`cargo check -p semio-s-plugin-cad --all-targets` got all the way through
`semio-framework-plugin` (where `ArtifactDeclaration` itself lives — 0 errors) into
`semio-s-plugin-stdio` (a cad dependency), where it hit a **confirmed unrelated, concurrent**
compile error — zero mentions of any `📐️cad` path anywhere in that output. **The crate's own
compile result (does `declaration()`/the plugin root actually type-check) was never reached** —
see "Verification" §4 for the full evidence trail and the exact re-run command. This machine is
running 40+ concurrent `cargo check`/`test` processes from sibling W1b sessions sharing one target
dir at time of writing (`ps aux | grep -c cargo` → 41), so lock contention on top of that is
expected, not a red build.

## Clearance (Step 0)

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`.
`📐️cad` appears in **neither** the RELEASED nor the HELD sections — per that file's own explicit
wording ("ABSENCE FROM THIS FILE MEANS FREE, NOT HELD"), cad was FREE to edit. Proceeded.

## What changed

### 1. `declaration()` replaces `register()` — `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- **Deleted** `pub fn register()` (old :901-910), `pub fn register_artifact_schema()` (old
  :916-918), `pub fn register_artifact_inferences()` (old :923-925) — grep-confirmed zero other
  call sites repo-wide before deleting (`register_pilot_languages()`, `register_artifact_schema(`,
  `register_artifact_inferences(` all returned only their own definition line).
- **Added** `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` (new :909-924),
  following `🗒️note`'s exemplar shape exactly:
  ```rust
  pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
      semio_framework_plugin::ArtifactDeclaration::builder("s.cad")
          .schema(crate::artifacts::cad::schema::cad_artifact_schema_descriptor())
          .inferences([crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_artifact_inference_descriptor()])
          .composers(crate::artifacts::cad::standards::v1::engine::io_registry::entries())
          .languages(pilot_languages())
          .document_codec::<crate::apps::cad::CadPlayApp>()
          .build()
  }
  ```
  `kind` is `"s.cad"`, matching the composer table's own `CAD_DIALECT.artifact_kind` (verified by
  reading the `io_registry` module's `const CAD_DIALECT: Dialect = Dialect { artifact_kind: "s.cad", ... }`
  at :1149 before writing the `.composers()` call — every entry in that 9-row table either writes
  `"s.cad"` (import direction, the `composer_entry_of::<CadAnyComposer>()` row) or reads it (export
  direction, the other 8 rows: ifc/step/png/json/dwg/stl/gltf/obj), so `register_all`'s ownership
  check passes for all of them). `document_codec::<CadPlayApp>()` reproduces the deleted call's
  behavior exactly — `CadPlayApp::DOCUMENT_SCHEMA` is `CAD_DOCUMENT_SCHEMA` ("cad.scene"), the same
  constant the deleted call passed explicitly (verified at `🎛️apps/📐️cad/🦀️component.rs:983`:
  `const DOCUMENT_SCHEMA: &'static str = CAD_DOCUMENT_SCHEMA;`).
- **Converted** `pub fn register_pilot_languages()` (old :797-848, imperative, five
  `dsl::register_language(...)` calls) into a private `fn pilot_languages() -> &'static
  [dsl::LanguageSpec]` (new :801-859) — `OnceLock`-backed, built once and leaked, mirroring note's
  own `pilot_languages()` helper 1:1 (same reason: `dsl::passthrough_hooks` isn't `const fn`, so the
  five `LanguageSpec`s can't be a `const` array). Same five languages, same ids
  (`cad.document`/`cad.op`/`cad.diff`/`cad.pack`/`cad.spr`), same grammar/protocol wiring — only the
  registration mechanism changed (imperative call → data handed to `.languages()`).
- No subset validators, no dialect migrations, no format descriptors exist for cad (grep-confirmed:
  `register_subset_validator|register_dialect_migration|register_format_descriptors` had zero hits
  anywhere in the plugin) — those `ArtifactDeclarationBuilder` methods are simply not called, which
  is correct, not an omission.
- Module doc comment at the top of the file updated to describe `declaration()` instead of the
  deleted `register()`.

### 2. Plugin root — `✏️s/🔌️plugins/📐️cad/🦀️component.rs`

```rust
pub fn plugin() -> Plugin {
    Plugin::builder("cad")
        .label("CAD")
        .version("0.1.0")
        .setup(crate::apps::cad::config::schema::register_app_schema)
        .artifact(crate::artifacts::cad::engine::declaration())
        .register_document_app::<crate::apps::cad::CadPlayApp>(crate::apps::cad::create_cad_app())
        .build()
}
```
`crate::artifacts::cad::engine::declaration()` resolves through the pre-existing shim module at
`📦️glue.rs:575-577` (`pub mod engine { pub use super::standards::v1::subsets::any::engine::*; }`),
the same shim the deleted `.setup(crate::artifacts::cad::engine::register)` call used — zero new
module wiring needed.

## `.setup()` survival — kept for exactly one call, same as note

`.setup(crate::apps::cad::config::schema::register_app_schema)` survives. It registers
`CadPlayApp`'s own config/presence `AppSchemaDescriptor` (`🎛️apps/📐️cad/🎚️config/🧬️schema/🦀️component.rs:102-104`,
calling `::schema::register_app_schema_descriptor`) — app-scope, not artifact-scope, and per the W1
mechanism report `ArtifactDeclaration` deliberately has no field for
`register_app_schema_descriptor` (§6 census: it's not one of the 9 artifact-scoped registrars). No
other reason for `.setup()` to survive was found — every other call the old `register()` made
(schema descriptor, inference descriptor, composer entries, languages, document codec) now flows
through `.artifact(declaration())`.

## Step 3 — plugin root already closed (no action needed)

Confirmed before starting: `find ✏️s/🔌️plugins/📐️cad -maxdepth 1` shows only `AGENTS.md`,
`README.md`, `🦀️component.rs`, plus dirs `🎛️apps`, `📦️packages`, `🗿️artifacts`, `🧩️extensions`. Per
the dispatch's plugin-specific note, `🧩️extensions` is pre-cleared as inventory-only (crates —
confirmed: `find 🧩️extensions -iname Cargo.toml` finds 4 hits, one per extension:
`🏢️aec-building`, `📐️spatial-shape`, `🏛️aec-building-structure`, `🔥️aec-building-energy`, each at
`📦️packages/🦀️rust/Cargo.toml` — Cargo.toml check confirms these are real crate members, correctly
left untouched). No `🛂️manifest/`/`🎟️capabilities/`/`🔧️setup/` dirs exist at the root. Nothing to
delete or relocate for this step.

## Step 4 — escape hatches and deps (measured, nothing found)

- `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_"` across
  the whole plugin: **zero hits**. Cad owns `"3d.cad"` (the `ArtifactKindSpec.id` at
  `🗿️artifacts/📐️cad/🦀️component.rs:379`) but registers no IO for that kind, matching the
  dispatch's explicit instruction not to write any — the only place `"3d.cad"` is wired at all is
  `.artifact_kind(artifact_kind())` on the **app** builder inside `create_cad_app()`
  (`🎛️apps/📐️cad/🦀️component.rs:1144`, app-scope media/window-kind capability, not touched — out of
  `ArtifactDeclaration`'s scope by design, that field lives on `AppBuilder` not
  `ArtifactDeclarationBuilder`). The plugin's real composer table (`s.cad`, 9 entries, see above) is
  a separate dialect and was already artifact-owned before this conversion.
- `grep -rln "semio_framework_os::"` across the plugin: **zero hits**. The plugin's only
  `semio-framework-os*` dependency in `📦️packages/🦀️rust/Cargo.toml:49` is
  `semio-framework-os-kernel` (imported as `dsl`/`store`/`protocol` in `📦️glue.rs:13-15`, and
  directly as `semio_framework_os_kernel` in two files) — a **different crate** from
  `semio-framework-os`, actively used, out of scope for the "purge `semio-framework-os` only if
  empty" instruction (that instruction is about the `register_mesh_*`-family crate, not this one).
  Nothing purged.
- Top-level `🗿️artifacts/📐️cad/🦀️component.rs`'s own `pub mod io_registry { entries()/compose()/register() }`
  (:468-491) is now **fully orphaned** — its `register()` was the only call site of the top-level
  wrapper and that call site is gone (replaced by `.composers()` pointing straight at
  `crate::artifacts::cad::standards::v1::engine::io_registry::entries()`, the real 9-entry table this
  wrapper only re-exported references to). Left in place, exactly matching the W1 report's own
  precedent on note's identically-shaped orphaned `io_registry` module — deleting it is unrelated
  cleanup outside this wave's scope. Flagged here for whoever next touches cad.

## Step 5 — inventory only (not fixed, catalogued)

1. **`static HOST: OnceLock<BrepEngineHost>`** —
   `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:92-95`
   (`pub fn cad_brep_host() -> &'static BrepEngineHost { static HOST: OnceLock<BrepEngineHost> = OnceLock::new(); HOST.get_or_init(|| BrepEngineHost::new(CAD_BREP_CACHE_BUDGET_BYTES)) }`).
   This is the ticket's own named exemplar violation: a plugin holding a **host-owned engine
   handle** (`BrepEngineHost`, the shared brep kernel session) in a process-global `OnceLock`, read
   by `cad_brep_kernel()` (:97-99) to lock the kernel for every synchronous `BrepKernel` call in the
   file (tessellation, solid construction, geometry import, etc. — dozens of call sites). This is a
   distinct violation class from ordinary interior mutability: it is the plugin reaching into and
   caching a handle that conceptually belongs to the framework/OS host process, not plugin-local
   state, and no declarative field on `ArtifactDeclaration` could express "the artifact needs a
   shared kernel handle" — the fix is cross-session (the ticket explicitly says do not fix it here).
   Not touched.
2. **`thread_local! { static CAD_PREVIEW_SEQ: RefCell<u64> }`** —
   `🎛️apps/📐️cad/🦀️component.rs:947-949`. Documented in its own preceding comment as "the sole
   surviving interior-mutable field" backing `gesture_preview`'s "never-VCS'd, never-config'd live
   rubber-band tick counter" — user-gesture state (a live, ephemeral, per-tab preview staleness
   counter), not a derived cache and not app state proper. Inventory-only per the dispatch's own
   distinction (user-gesture state vs. derived cache); not fixed.
3. **`static SLOT: OnceLock<Mutex<String>>`** (via `last_cad_computer_contributions_json()`) —
   `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:862-863`, read/written
   by `sync_cad_computer_contributions()` (:877-895) to dedupe host-pushed `cad.computer` topic
   contributions (skips re-processing when the incoming JSON string is byte-identical to the last
   seen). This is closer to a derived cache (memoized "have I already synced this payload") than
   user-gesture state; per the dispatch, a derived cache belongs in an inference — not built here,
   catalogued only.
4. **`std::fs::read_dir` / `std::fs::read_to_string`** —
   `🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs:640,655`. Both calls are inside
   `#[cfg(test)]` (a regression test walking the `📚️examples/🖼️assets/🏗️modelDefinitions/**/🎬️interactions/*.json`
   fixture tree to assert every on-disk interaction asset parses as `InteractionSpec`) — excluded by
   the dispatch's own "outside `#[cfg(test)]`" carve-out. No other `std::fs`/`std::env`/`std::process`/
   `Command::new` usage exists anywhere in the plugin (grep-confirmed empty for the other three).

## Verification (Step 6)

1. **`#[path]` mount resolution** — scripted against the real filesystem (not pattern-substituted):
   parsed every `#[path = "..."]` attribute in `📦️packages/🦀️rust/📦️glue.rs`, resolved each
   relative to that file's own directory, checked `os.path.isfile`.
   **147 non-`"."` mounts, 0 missing.**
2. **`include_str!`/`include_bytes!` target resolution** — same discipline, walked every `.rs` file
   in the plugin (excluding `node_modules`), resolved each literal path relative to its own file's
   directory, checked `os.path.isfile`. **117 targets, 0 missing.**
3. **`cargo metadata --no-deps --format-version 1 >/dev/null && echo OK`** — ran from repo root:
   ```
   OK
   ```
4. **`cargo check -p semio-s-plugin-cad --all-targets`**, `RUSTC_WRAPPER=""`,
   `CARGO_TARGET_DIR=".../ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/🎯️target"` — run twice, real output
   pasted from both attempts, neither reached a verdict on cad's own crate:

   **Attempt 1** (`scratch-w1b-cad-check-1.txt`, ~9.5 min, mostly spent blocked on
   `Blocking waiting for file lock on package cache` / `...on build directory` before another
   session released it): once unblocked, compiled clean through
   `semio-framework-os-kernel` → `semio-framework-ui` → `semio-framework-schema` →
   `semio-framework-math` → `semio-framework` → **`semio-framework-plugin`** (0 errors — this is
   the crate `ArtifactDeclaration`/`.artifact()` itself live in, so this is real evidence the M1
   mechanism still compiles) → started `Checking semio-s-plugin-stdio`, then:
   ```
   error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/./././././././././../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`: No such file or directory (os error 2)
       --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:6028:37
   error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
   warning: build failed, waiting for other jobs to finish...
   ```
   Confirmed **not mine**: `find` on that exact `📄set-snapshot` dir shows it no longer exists on
   disk (another session mid-rename, consistent with this ticket's own hard rule never to write
   `SetSnapshot` under `✏️s/`), `stat -f '%Sm'` on stdio's `📦️glue.rs` showed a mtime seconds before
   this check, and re-grepping that file moments later showed the `set_snapshot` mount already
   rewritten — stdio was being actively fixed by another session in real time. Zero occurrences of
   any `🔌️plugins/📐️cad` path anywhere in this output (grep-verified).

   **Attempt 2** (`scratch-w1b-cad-check-2.txt`, started fresh once attempt 1's stdio error
   resolved the build): the package-cache lock cleared quickly this time but the process was still
   sitting on `Blocking waiting for file lock on build directory` after 3:47 elapsed when this
   report was finalized — the same shared `🎯️target` dir is being hammered by dozens of concurrent
   sibling sessions (`ps aux | grep -c "cargo check\|cargo test\|cargo nextest"` → 41 at the time).
   Never got far enough to re-attempt stdio, let alone reach cad.

   **Net verification state**: static resolution (147 `#[path]` mounts, 117 `include_str!`/
   `include_bytes!` targets, `cargo metadata`) is fully clean; `semio-framework-plugin` (the
   mechanism itself) is compiler-confirmed 0-errors; **cad's own crate has not yet been
   compiler-verified** — the only blocker both times was infrastructure external to this diff
   (lock contention, then an unrelated in-flight stdio rename). **Re-run exactly this command** to
   get the real pass/fail once the shared machine quiets down:
   ```
   RUSTC_WRAPPER="" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/🎯️target" cargo check -p semio-s-plugin-cad --all-targets
   ```
   Full raw output for both attempts is at
   `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-w1b-cad-check-1.txt`
   and `...-check-2.txt` in this ticket folder.

## sharedFileRequests

None. Everything touched is inside `✏️s/🔌️plugins/📐️cad/` (the two files listed under "What
changed"). No framework, host, or other-plugin files were edited.

## apa-status

`partial` — steps 1-2 (declaration + plugin-root wiring) are done and self-consistent: path/include
resolution (147+117 targets, 0 missing) and `cargo metadata` are both clean, and
`semio-framework-plugin` (the `ArtifactDeclaration`/`.artifact()` mechanism itself) compiler-checked
0 errors mid-way through attempt 1. **cad's own crate has not yet gotten a compiler verdict** — both
`cargo check -p semio-s-plugin-cad --all-targets` attempts were blocked by infrastructure external
to this diff (cargo lock contention from 40+ concurrent sibling sessions, then an unrelated in-flight
`semio-s-plugin-stdio` rename with zero `📐️cad` paths in its error) rather than by anything in cad's
own code. Re-run the command in "Verification" §4 once the shared machine quiets down to get the
real pass/fail. Steps 3-5 required no code changes (root already closed except pre-cleared
`🧩️extensions`; no escape-hatch calls found to remove; inventory items catalogued, not fixed, per
the dispatch's own instructions).
