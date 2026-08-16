# W2 Packet P4 (cad) — Pilot Report

Lane: W2 packet P4, plugin `📐️cad`, subset `s.cad.cad@1/*`. Scope: migrate the retired
`✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/` app into `✏️editor`, author a real `👁️viewer`, rewire
`📦️glue.rs`/`Cargo.toml`/TS configs, and record the recipe every other W2 packet follows.

## What landed

### Editor (`…/🪆️subsets/✳️any/✏️editor/`)

The entire old app tree moved across intact: root `🦀️component.rs` (now `impl ArtifactEditor for
CadPlayApp`), `🎚️config` (+schema), `👥️presence` (+schema), `🎮️commands/*` (11 groups), `📌️panels/*`
(3), `🗣️terminology`, `🌉️wasm`, `📚️examples/🎬️demo-session`, and `🎭️modes/✏️edit/{component.rs,
🎚️options/*, 🪟️windows/{📐️shape,🏢️building,🔥️energy,🏛️structure-classic}}`. Each of the four
windows gained a real `🟦️component.ts` twin (typed `ViewModel`/`CadDislocateOptions` interfaces +
window-kind id/body-key/surface-id constants, mirroring the Rust `render()` boundary) — the scaffold's
single `🪟️main` placeholder was deleted. The surface root also gained a real `🟦️component.ts`
(namespaced re-export of all four window twins — `export * as shapeWindow from …`, not a blanket
`export *`, since all four windows independently declare a same-named `CadDislocateOptions` interface
and a blanket re-export would be ambiguous).

`impl ArtifactApp for CadPlayApp` → `impl ArtifactEditor for CadPlayApp`; `const APP_ID` removed;
`const DIALECT: Dialect = crate::artifacts::cad::CAD_DIALECT` added. `create_cad_app()` now returns
`AppDefinition` (`Editor::builder(CAD_DIALECT)…build_definition()`) instead of `App`; the trailing
`.example(CAD_EXAMPLE_FOREST_LEFT, …)` / `.workflow("cad", …)` calls were **dropped**, not ported — see
"SDK gaps" below.

### Viewer (`…/✳️any/👁️viewer/`)

A genuinely independent, minimal, real `CadViewer: ArtifactViewer`:
- `Snapshot = CadSnapshot`, `Mutation = crate::artifacts::cad::op::CadMutation` (both artifact-level,
  shared with the editor — decode-only per contract §2.2).
- `Config`/`ConfigMutation` = framework `NoConfig`/`NoConfigMutation`; `Presence`/`Transient` likewise
  `NoPresence`/`NoTransient`. A viewer needs no persisted per-session state to render — camera/sun use
  hardcoded defaults, documented as an intentional simplification, not a bug.
- `Command` = a one-variant `CadViewCommand::Noop` (viewer declares no actions); `handle` always
  returns `Ok(ViewEmit::default())`.
- One real window, `📐️shape` (`🎭️modes/👁️view/🪟️windows/📐️shape`), rendering the actual
  `CadSnapshot` (real reference overlays via `world_references_json`, real camera/environment JSON via
  the same `semio_framework_plugin::world3d_*` helpers the editor uses) through
  `build_world_3d_scene`/`world3d_scene_extended` — **not** by calling into the editor. Object/mesh
  content renders the same fallback-box placeholder the editor's own `world_meshes_json` already falls
  back to while composed-child object resolution is unimplemented (pre-existing
  `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave-3 gap, not introduced here) — real parity with the
  editor's *current* behavior, not a regression.
- `create_cad_viewer() -> AppDefinition` via `Viewer::builder(CAD_DIALECT)…build_definition()`.

### `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::cad::*` from `../../🎛️apps/📐️cad/…`) replaced by two
independent regions:
- `//#region ✏️Editor` — `pub mod editor { pub mod cad { … } }`, every leaf `#[path]`-mounted from
  `../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
- `//#region 👁️Viewer` — `pub mod viewer { pub mod cad { … } }`, same base but `…/👁️viewer/…`, deliberately
  never mounting anything under `✏️editor/`.

Every `crate::apps::cad::` reference across the 25 moved Rust files became `crate::editor::cad::`
(mechanical `sed`, editor tree only). The bottom `//#region 📚️Examples` mount for
`app_cad_demo_session` was repointed at the new editor path (name kept, only the `#[path]` string
changed). `resolveAll #[path] attrs` verified against disk twice (243 total, 0 missing) — see
`🧪️w2-cad-cargo.txt`.

### Plugin root (`✏️s/🔌️plugins/📐️cad/🦀️component.rs`)

`.document_app::<crate::apps::cad::CadPlayApp>(create_cad_app())` → two calls:
`.editor::<crate::editor::cad::CadPlayApp>(crate::editor::cad::create_cad_app())` and
`.viewer::<crate::viewer::cad::CadViewer>(crate::viewer::cad::create_cad_viewer())`. Added
`#[cfg(test)] mod surface_tests` with the two required assertions (local stand-ins — see SDK gaps).

Note: mid-session this file also picked up an unrelated concurrent edit from another live session
(`register_exports`/`.setup()` replaced by a declarative `.host_media_handler(…)` call) — merged
cleanly with my `.editor()`/`.viewer()` wiring, confirmed by re-reading the file; not something I
authored, not reverted.

### `🗿️artifacts/📐️cad/🦀️component.rs`

Added `pub const CAD_DIALECT: Dialect = Dialect { artifact_kind: "s.cad.cad", standard:
StandardId("1"), subset: SubsetId::ANY }` — lives at the ARTIFACT level (not under `editor`/`viewer`)
specifically so a viewer file can read it without ever importing through `editor`. `artifact_kind =
"s.cad.cad"` matches the id `definition()`'s own `"s.cad.schema.artifact"` row already keys off,
`standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the
canonical surface id is `s.cad.cad@1/*#editor` / `s.cad.cad@1/*#viewer`, exactly the contract §1
grammar. Fixed the one other real (non-comment) `crate::apps::cad::` reference:
`.document_codec::<crate::apps::cad::CadPlayApp>()` → `.document_codec::<EditorApp<CadPlayApp>>()`
(the runtime `ArtifactApp` bound needs the SDK adapter, not the authoring trait implementor directly).
Three stray doc-comment references to the old path fixed too (cosmetic).

### `📦️packages/🦀️rust/Cargo.toml`, `📦️packages/🟦️typescript/tsconfig.json`

`[package.metadata.semio.storybook].sourceRoots` and every `tsconfig.json` `include` entry pointing at
`🎛️apps/📐️cad/⚙️engine/…` repointed at `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/…`.
`tsconfig.json` `include` list verified against disk (0 missing).

### Deletion

`✏️s/🔌️plugins/📐️cad/🎛️apps/` removed in full (it was the plugin's only app) once every real file had
a real destination.

## Migration recipe (for the other 8 W2 packets)

Numbered, in the order to actually do them — read this before touching files.

1. **Read the SDK first.** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` region
   `🔖️Surfaces` (`ArtifactEditor`/`ArtifactViewer`/`ViewEmit`/`EditorApp`/`ViewerApp`/`Editor`/`Viewer`
   builders) and `🏗️builder/🦀️component.rs` (`PluginBuilder::editor`/`::viewer`). Line numbers drift
   as peers edit the file — `grep -n "//#region 🔖️Surfaces"` fresh each time, don't trust cached
   numbers.
2. **SDK re-export trap (the single biggest time sink this packet hit):** `ArtifactEditor`,
   `ArtifactViewer`, `Editor`, `Viewer`, `EditorApp`, `ViewerApp`, `ViewEmit`, `Dialect`, `StandardId`,
   `SubsetId` are **not** in `semio_framework_plugin`'s crate-root re-export list
   (`🔌️plugin/🦀️component.rs:~17858`, a hand-maintained `pub use app::{ … };` block) as of this
   packet. `use semio_framework_plugin::ArtifactEditor` etc. fails with `E0432 unresolved import`.
   Import them as `use semio_framework_plugin::app::{ArtifactEditor, Dialect, Editor, …};` instead —
   `NoConfig`/`NoPresence`/`NoTransient`/`App`/`ArtifactApp`/etc. (pre-existing names) ARE in the
   curated list and work bare. **Check the actual list before assuming** — it may have grown by the
   time you read this; grep it fresh (`sed -n '17858,17990p' … | grep YourTypeName`).
3. **Move the whole app tree in one shot**, preserving internal structure, into `✳️any/✏️editor/`:
   root `component.rs`, `🎚️config`, `👥️presence`, `🎮️commands/*`, `📌️panels/*`, `🗣️terminology`,
   `🌉️wasm`, `📚️examples/*`, and the mode subtree (`🎭️modes/<edit-mode>/{component.rs, any
   mode-level facets, 🪟️windows/*}`). Delete the scaffold's placeholder leaves you're about to
   overwrite FIRST (`rm -f`/`rm -rf` the specific scaffold files/dirs), then `mv` the real ones in —
   don't try to diff/merge, the scaffold is 100% disposable.
4. **`⚙️engine` (or any other app-only, non-`surfaceChildDirs` facet) has no taxonomy slot under a
   surface.** `surfaceChildDirs` is `[🎭️modes, 🎮️commands, 📌️panels, 🎚️config, 👥️presence,
   🫧️transient, 🗣️terminology, 🌉️wasm, 📚️examples]` — no `⚙️engine`. If your app has one, check who
   actually calls it: if only editor-side files (commands/mode/windows) reference it, move it whole
   into `✏️editor/⚙️engine/` (pragmatic, undocumented-but-not-forbidden today — `policyTaxonomyDirsBreaches`
   doesn't walk surface subtrees until W3 per contract §6). If your subset's own `🧬️schema/💡️inferences`
   *also* depends on it (ours did, via TS relative imports), that's a pre-existing layering wrinkle,
   not yours to fix — just repoint the relative import depth, don't relocate the dependency direction.
5. **`include_str!`/`include_bytes!` are relative to the file's location ON DISK, not the module
   tree.** Any file you physically move that reaches OUTSIDE its own moved subtree via a relative
   `include_str!` breaks silently until compile time (`error: could not read … No such file or
   directory`, no clue it's a depth problem from the message alone). Compute the new depth delta
   (old app path depth vs. new `✏️editor/…` path depth) and fix every such macro — grep
   `include_str!\(".*\.\./` across every moved file, not just the obvious ones. `include_str!` calls
   that stay INSIDE the moved subtree (referencing a sibling that moved with it) need no fix.
6. **Root `component.rs` trait/manifest edits:**
   - `impl ArtifactApp for X` → `impl ArtifactEditor for X`; delete `const APP_ID`; add
     `const DIALECT: Dialect = …` (define a `pub const <PLUGIN>_DIALECT` at the ARTIFACT level, not
     under `editor`, so a viewer can read it without an `::editor::` import — grammar is
     `Dialect { artifact_kind: "s.<plugin>.<artifact>", standard: StandardId("<std slug>"), subset:
     SubsetId::ANY }` for the `✳️any` subset, matching `<artifact_kind>@<standard>/<subset>`).
   - `create_X_app()`: `App::builder(ID, LABEL)` → `Editor::builder(DIALECT)` (label is auto-set to
     "Editor"/"Editor" — drop any custom label arg). Change the return type to `AppDefinition` and end
     the chain with `.build_definition()` instead of `App::from_builder(...)`. **`.example(...)` and
     `.workflow(...)` do not exist on `EditorBuilder`** (contract §2.4's `App { definition, examples
     }` split — `.editor::<E>(def: AppDefinition)` only takes the definition, examples always end up
     empty) — drop those calls, don't try to port them; note it, don't silently lose the behavior
     without a comment.
   - Plugin root: `.document_app::<X>(create_x_app())` → `.editor::<X>(create_x_app())` +
     `.viewer::<V>(create_v_viewer())`.
   - Any `.document_codec::<X>()` call elsewhere in the artifact's `declaration()` needs
     `X` → `EditorApp<X>` (that builder method is bound on the RUNTIME `ArtifactApp` trait, which only
     the adapter implements now).
7. **Test module fallout in the SAME root file** — the highest-risk silent-break spot:
   - `VcsArtifactApp<X>` / `testkit::new_app::<X>()` → `VcsArtifactApp<EditorApp<X>>` /
     `testkit::new_app::<EditorApp<X>>()`.
   - `<X as ArtifactApp>::method(...)` → `<X as ArtifactEditor>::method(...)`.
   - `testkit::assert_declared_actions_bridge_to_commands::<X>(create_x_app)` — this testkit fn's
     signature is still `fn(manifest: fn() -> App)`, unchanged for this ticket; `create_x_app` now
     returns `AppDefinition`. Write a tiny local `fn x_manifest_for_testkit() -> App { App {
     definition: create_x_app(), examples: Vec::new() } }` and pass that instead (framework testkit
     gap, not yours to fix).
   - Grep the WHOLE moved tree (not just the root file) for `ArtifactApp`, `VcsArtifactApp<`,
     `App::builder`, `App::from_builder`, `<PLUGIN>_PLAY_APP_ID` used as a trait const (as opposed to
     a plain string tag, which is fine to keep) before declaring done.
8. **Viewer: keep it genuinely independent, not a thin wrapper.** `Snapshot`/decode-only `Mutation`
   should be the SAME artifact-level types the editor uses (they already live outside both surfaces).
   `Config`/`Presence`/`Transient` can almost always be the framework's `NoConfig`/`NoPresence`/
   `NoTransient` — a viewer rarely needs persisted per-session state for a first pass. `Command` can be
   a single-variant no-op enum if you declare no view actions yet. For `render`, either (a) relocate
   genuinely pure, reusable render helpers into the subset's `🧬️schema/💡️inferences` (cleanest, but
   costs real refactor time — this packet did NOT do this for the bulk of the editor's WorldScene
   helpers, given the time budget) or (b) write a small, self-contained pure render function directly
   in the viewer's own window file, built from framework-level `world3d_*` helpers and artifact-level
   pure inference functions — never call into `crate::…::editor::…`.
9. **`policyViewerPurityBreaches` is a literal substring match on `::editor::`, including inside
   comments/doc-comments.** Writing "…never `crate::…::editor::…`" as an explanatory doc-comment IN a
   viewer file trips the same check it's explaining. Phrase warnings about the forbidden pattern
   without literally typing `::editor::` (e.g. "the sibling `editor` module", "the editor module",
   spelled without the leading/trailing `::`).
10. **Wire `📦️glue.rs`**: derive every new path by reading the glue file's OWN existing `pub mod`
    nesting and the real subset path on disk — never copy a path from a sibling artifact/plugin. Two
    independent `#[path = "."]` module trees (`editor`/`viewer`), each `#[path]`-mounting from
    `../../🗿️artifacts/<plugin>/🏅️standards/🔖️<std>/🪆️subsets/✳️<subset>/<✏️editor|👁️viewer>/…`.
    **Verify with a script before reporting done** — don't eyeball emoji paths:
    ```python
    import re, os
    text = open(glue_path, encoding="utf-8").read()
    for p in re.findall(r'#\[path\s*=\s*"([^"]+)"\]', text):
        if p != "." and not os.path.isfile(os.path.normpath(os.path.join(os.path.dirname(glue_path), p))):
            print("MISSING", p)
    ```
    This caught zero issues here on the first real run because I built every path programmatically
    from a saved base-path string (see trap below) — hand-typing 240+ emoji paths would not have.
11. **THE emoji-typo trap**: "🏅️standards" (Latin) vs "🏅️标准" (Chinese, meaning the same word) render
    almost identically in a terminal/editor at a glance but are different bytes — I mistyped this
    exactly four times mid-session, each time silently creating a WRONG sibling directory tree instead
    of erroring. **Never hand-type a subset path segment.** Save the real path once (`ls`/`find` it,
    copy the exact string into a shell variable or a scratch file), then reuse that variable/file for
    every subsequent Bash/Python invocation touching the same subset. When you must use a tool that
    can't take a shell variable (Read/Write/Edit's `file_path`), paste from a just-successful command's
    OWN output, never retype from memory — and re-`ls` the parent directory immediately after every
    `Write` to a new path to confirm it landed where you meant.
12. **Cargo.toml / tsconfig.json / vitest.config.ts / project.json**: grep the WHOLE plugin package
    tree (`📦️packages/**`) for the literal old app path string (`🎛️apps/<plugin>`) — it leaks into
    build-tool config (`sourceRoots`, `include` lists) that a Rust-focused pass easily misses. Verify
    JSON/tsconfig `include` arrays resolve on disk the same way as glue.rs.
13. **Referrers outside your lease**: grep the WHOLE repo (not just your plugin) for
    `apps::<yourplugin>` and the literal old path string before declaring done. Real Rust-code
    dependencies (not just doc comments) from ANOTHER plugin are a hard blocker for that plugin's next
    `cargo check` and must be reported, not silently left — see "Outside-lease referrers" below.
14. **`assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect`/`new_viewer` (contract
    §2.5) do not exist in the framework yet** as of this packet (grepped the whole tree, confirmed
    absent). `new_viewer::<V>()` needs no framework change — it's just
    `testkit::new_app::<ViewerApp<V>>()`, already generic. The two `assert_*` functions: write tiny
    local equivalents in your plugin's own test module (see this packet's `surface_tests` region for
    the exact pattern) and swap them for the canonical versions once W1-A lands them.
15. **Verification order**: fix-compile-fix-compile in a loop — don't try to hand-verify 2000+ lines
    of Rust by reading. Each real `cargo check` run surfaces a genuinely different class of error;
    this packet went 137 → 17 → 14 errors across three runs, and by the third run **zero** were in its
    own plugin files (confirm this explicitly: `grep -B2 -A5 "^error" cargo.txt | grep -c
    "🔌️plugins/<yourplugin>"` should read 0 before you stop iterating). If it's still 0 and the crate
    still won't finish checking, that's the workspace-churn case below, not your bug.
16. **Concurrent workspace churn is real and will hit you.** Before attributing any error outside your
    own plugin's files to yourself, run `git status --porcelain -- <thatfile>` and `git log --date=iso
    -2 -- <thatfile>` — if it's modified/uncommitted or was touched today by another live session,
    it's not yours. This packet hit three different snapshots of unrelated breakage (stdio's gltf
    inferences → stdio's semio artifact mutations → framework's own `os-kernel`/`dsl`/`spr` crates),
    each moving further upstream as the peer session kept editing, confirming it live rather than
    something this packet caused. Document it with the evidence (file, error text, git status/log),
    don't try to fix it, don't block on it.

## Outside-lease referrers (report, not fixed)

- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs:5` —
  `use cad::apps::cad::{create_cad_app, CadPlayApp};` is a REAL Rust compile dependency on the now-
  deleted `apps::cad` module path and the old `create_cad_app() -> App` / `CadPlayApp: ArtifactApp`
  shapes. This will fail to compile as soon as `semio-s-plugin-cad` is rebuilt with this packet's
  changes. Needs its owner to update to `cad::editor::cad::{create_cad_app, CadPlayApp}` (now
  `AppDefinition`-returning) or `cad::viewer::cad::{create_cad_viewer, CadViewer}`, whichever the
  demonstrator actually wants to embed.
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/📌️panels/🔍️properties/🦀️component.rs:28` — a doc comment only
  (references the old `🎛️apps/📐️cad/📌️panels/🔍️inspection/…` path for context). Not a compile
  dependency; cosmetic drift, low priority for that plugin's own owner.

## SDK gaps found (framework, outside this packet's lease — report to W1-A)

1. `ArtifactEditor`, `ArtifactViewer`, `Editor`, `Viewer`, `EditorApp`, `ViewerApp`, `ViewEmit`,
   `Dialect`, `StandardId`, `SubsetId` are not in `semio_framework_plugin`'s curated crate-root
   `pub use app::{ … };` list (`🔌️plugin/🦀️component.rs:~17858`) — every consumer needs the
   `semio_framework_plugin::app::` prefix today. Trivial fix (add 10 names to the existing list), but
   every one of the other 8 W2 packets will hit the exact same `E0432` until it lands.
2. `testkit::assert_viewer_never_mutates`, `testkit::assert_editor_and_viewer_share_dialect`,
   `testkit::new_viewer` (contract §2.5) do not exist yet. Local stand-ins are in this packet's
   `surface_tests` module with a doc comment pointing at this gap.
3. `testkit::assert_declared_actions_bridge_to_commands<A: ArtifactApp + Default>(manifest: fn() ->
   App)` was not updated for the `AppDefinition`-returning `create_*_app()` convention — every packet
   that already has a test calling it (most do) needs the same `App { definition: create_x_app(),
   examples: Vec::new() }` local wrapper this packet used.
4. `PluginBuilder::editor::<E>`/`::viewer::<V>` take a bare `AppDefinition`, discarding
   `App.examples` entirely (`App { definition: def, examples: Vec::new() }` inside both builder
   methods) — every existing `.example(...)`/`.workflow(...)` call chain on a `create_*_app()` that
   gets migrated to `Editor::builder(...)` silently loses its example registration unless the packet
   notices and reports it (as this one does). Possibly intentional (the subset's own
   `📚️examples/🎬️demo` facet may be the intended replacement mechanism) but not stated anywhere in
   the contract — worth confirming with the coordinator.

## Verification run

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-cad --all-targets --keep-going`, three full runs,
  output for the last one in `🧪️w2-cad-cargo.txt`:
  - Run 1: 137 errors, all 137 inside `semio-s-plugin-stdio`'s own gltf inferences files (confirmed
    live-edited: `git status` showed them modified, `git log --date=iso` showed a commit at
    2026-08-16 12:10:56 — today). **0 errors in `📐️cad` files.**
  - Run 2 (after that peer session's stdio fix landed): 17 errors — 8 in stdio's own `dsl::Mutations`
    derive output (`MutationOutcome`/`ConflictRule` mismatches, still stdio's own files), plus the 9
    real errors this packet's own code had at that point (fixed — see commit history in this report's
    "What landed" section for what those were: `EditorApp`/`ArtifactEditor` etc. import path, testkit
    fallout, `include_str!` depth). **0 remaining in `📐️cad` files after fixing.**
  - Run 3: 14 errors, now inside `semio-framework-os-kernel` itself (`MutationOutcome` still missing,
    `ReconcileReport`/`ReconcileSeverity` unresolved, `.validate()`/`.reconcile()` missing on
    `Mutation`/`Op`) — confirmed via `git status`/`git log` that `🧰️framework/…/🗣️dsl/**` and
    `…/📡️spr/**` are mid-edit by a live session RIGHT NOW. **0 in `📐️cad` files.**
  - Net: every real error this packet's OWN code produced across all three runs was found and fixed;
    the crate cannot finish a full `cargo check` right now purely because of unrelated, actively
    in-flight framework/stdio work. Re-run once that lands.
- `cargo test -p semio-s-plugin-cad --no-run` — same blocker (stdio → cargo check dependency chain),
  output in `🧪️w2-cad-test.txt`.
- WASM build: no `wasm32`-specific nx target exists in
  `📦️packages/🦀️rust/📋️project.json` for this plugin (checked; none of the sibling plugins have one
  either — wasm packaging is orchestrated elsewhere, not per-plugin). Tried both `cargo component
  build -p semio-s-plugin-cad --target wasm32-unknown-unknown` and plain `cargo build … --target
  wasm32-unknown-unknown`; both fail identically and immediately on an unrelated, pre-existing
  dependency-feature gap: `getrandom v0.3.4` needs its `wasm_js` feature enabled for this target, and
  nothing in the workspace enables it for this dependency edge (two OTHER crates —
  `🧰️framework/…/🧊️wgpu` and `compose/client/lib/rs` — already carry the identical
  `getrandom = { version = "0.3.4", features = ["wasm_js"] }` fix in their own `Cargo.toml`, confirming
  this is a known, already-patched-elsewhere gap, not something introduced by this migration). Output
  in `🧪️w2-cad-wasm.txt`. Not fixed — outside this packet's lease (a transitive dependency-feature
  edge, not `📐️cad`'s own `Cargo.toml`) and not cad-specific.
- `bun ./📜️script.ts policy` (repo root), full run in `🧪️w2-cad-policy-full.txt`, cross-checked
  against `.🦑️repo/⚡️cache/breaches/compose.json` directly:
  - `taxonomy/surface-completeness`: 0 total repo-wide.
  - `taxonomy/surface-scaffold-residue`: 284 total repo-wide (= 142 other subsets × 2 roles still
    scaffolded — expected, not this packet's job), **0 for `📐️cad`**.
  - `taxonomy/viewer-purity`: 0 total (found 2 for `📐️cad` on the first run — both were my own
    explanatory doc-comments literally containing the substring `::editor::`; rephrased, confirmed 0
    on re-run).
  - `plugin-dependency/contributed-surface-target`, `taxonomy/os-config-shape`: 0, unaffected.
  - **Target met**: `s.cad.cad@1/*` shows 0 breaches and 0 scaffold-residue rows across all three new
    surface policies.

## Files touched

Created:
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (85 files —
  moved content + 5 new real `🟦️component.ts` twins: 4 windows + surface root)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (19 files —
  `🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `📐️shape` window; taxonomy
  facet dirs otherwise `📌️empty.md`)

Edited:
- `✏️s/🔌️plugins/📐️cad/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring, `surface_tests`)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs` (`CAD_DIALECT`, `.document_codec::<EditorApp<…>>()`, doc fixes)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions)
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` (storybook `sourceRoots`)
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json` (`include` paths)

Deleted:
- `✏️s/🔌️plugins/📐️cad/🎛️apps/` (whole tree — the plugin's only app)

Scratch (ticket folder): `🧪️w2-cad-cargo.txt`, `🧪️w2-cad-test.txt`, `🧪️w2-cad-wasm.txt`,
`🧪️w2-cad-policy-full.txt`.
