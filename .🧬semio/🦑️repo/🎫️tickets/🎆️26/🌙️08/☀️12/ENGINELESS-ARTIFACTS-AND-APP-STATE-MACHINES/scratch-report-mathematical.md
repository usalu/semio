# Dissolve `➗️mathematical` artifact-tree `⚙️engine` — Report

Ticket: 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES (#2553)
Scope: `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` (1 file, 313 LOC, `🦀️component.rs`)

Pre-flight `git log -3 --oneline` on the engine dir showed only old, already-committed history (`382ace1b27`, `fd01661f06`) — no unexpected concurrent churn on this specific file.

## (a) Summary table

| Engine dir | LOC before | Destinations |
|---|---|---|
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` | 313 | `🎛️apps/➗️mathematical/🦀️component.rs` (Io, Scene, GraphAlgorithms, Geometry regions + tests); `🚪️io/🦀️component.rs` (DerivedIoRegistry region); `MathematicalEngine` struct deleted outright (fossil, 0 external refs, no `ArtifactEngine` trait impl anywhere) |

## (b) Every region and where it went

| Engine region | Contents | Destination | Reasoning |
|---|---|---|---|
| `//#region 🔖️Io` | `mathematical_io() -> AppIo` | `🎛️apps/➗️mathematical/🦀️component.rs`, new `//#region 🔖️Io` | Rule 4: returns `AppIo` |
| `//#region 🔖️Scene` | `empty_component_scene(...)` | `🎛️apps/➗️mathematical/🦀️component.rs`, new `//#region 🔖️Scene` | Rule 4: references app type `MATH_APP_ID`; consumed by both window renderers, mirrors the app being the new shared home |
| `//#region 🔖️GraphAlgorithms` | `algorithm_overlay(&MathematicalGraph)`, `workflow_json(&MathematicalGraph)` | `🎛️apps/➗️mathematical/🦀️component.rs`, new `//#region 🔖️GraphAlgorithms` | Rule 8: pure over a Graph substructure, but reachable from BOTH the graph window renderer AND the app top-level's `export_media` — neither io-codec-only nor inference-only, so app behaviour; not stateful so rule 7's `⚙️engine` stub not needed |
| `//#region 🔖️Geometry` | `geometry_layers_json(&MathematicalGeometry)` | `🎛️apps/➗️mathematical/🦀️component.rs`, new `//#region 🔖️Geometry` | Same reasoning as GraphAlgorithms — single consumer (geometry window) but for consistency with the dissolved engine's own grouping stayed alongside the other derived-render helpers in the app root |
| `//#region 🧪️Tests` | 6 tests covering `mathematical_io`, `algorithm_overlay` (x3), `workflow_json`, `geometry_layers_json` | `🎛️apps/➗️mathematical/🦀️component.rs`'s existing `mod tests`, new sub-regions `//#region 🔖️MathematicalIo`, `//#region 🔖️GraphAlgorithms`, `//#region 🔖️Geometry` | Rule 9: travel with the code they test |
| `//#region 🔖️ArtifactEngine` | `MathematicalEngine` struct + `impl MathematicalEngine { fn new }` | **Deleted outright** | Rule 1: repo-wide grep for `MathematicalEngine` found only its own file; grep for `trait ArtifactEngine` / `impl … ArtifactEngine for …` found 0 hits repo-wide (only a comment in an unrelated file noting its absence). Fossil. |
| `//#region 🚪️DerivedIoRegistry` | `pub mod io_registry { ENTRIES, MATHEMATICAL_DIALECT, MATHEMATICAL_JSON_BRIDGE_DIALECT, rebuild_native_snapshot, EXPORT_MD_DIALECT/compose_export_md, EXPORT_JSON_DIALECT/compose_export_json, entries() }` | `🚪️io/🦀️component.rs`, new `//#region 🚪️DerivedIoRegistry` (after the existing `//#region 🎹️DerivedComposition`) | Rule 5: mirrors the `◻2d` exemplar exactly — `io_registry`/`ComposerEntry`/serializer wrappers land in `🚪️io` |

## (c) Unqualified paths found and how they were qualified

The `io_registry` module's own body used only fully-qualified `crate::...` paths already (`crate::artifacts::mathematical::io::import::deserializers::...`, `crate::artifacts::mathematical::io::export::serializers::...`) — no bare-path hazard inside the moved block itself, so it moved verbatim into `🚪️io/🦀️component.rs`.

Two call sites elsewhere pointed at the OLD `engine::io_registry` path and needed fixing (both fully qualified already, just wrong target module — not bare-path shadowing, but the exact hazard class the ticket warns about since the artifact root's OWN thin `io_registry` wrapper module lives in the same file as the `declaration()` fn):

1. `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🦀️component.rs`, `declaration()`'s `.composers(...)` call:
   - Before: `crate::artifacts::mathematical::standards::v1::engine::io_registry::entries()`
   - After: `crate::artifacts::mathematical::standards::v1::subsets::any::io::io_registry::entries()` (fully qualified, points at the REAL registry now in `🚪️io`, not the artifact root's own local `&'static [&'static ComposerEntry]` wrapper defined further down in the same file)
2. Artifact root's own local `io_registry` wrapper module's import:
   - Before: `use crate::artifacts::mathematical::standards::v1::engine::io_registry as v1;`
   - After: `use crate::artifacts::mathematical::standards::v1::subsets::any::io::io_registry as v1;`

All three call sites inside `🎛️apps/➗️mathematical/🦀️component.rs` that referenced `crate::artifacts::mathematical::engine::{mathematical_io, algorithm_overlay}` were rewritten to bare local calls (`mathematical_io()`, `algorithm_overlay(...)`) since the functions now live in that same file — no ambiguity, no other symbol of that name exists there.

Two window-renderer files' imports were repointed from the dissolved `crate::artifacts::mathematical::engine::{...}` to the new home `crate::apps::mathematical::{...}`:
- `🎛️apps/➗️mathematical/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️component.rs`: `use crate::apps::mathematical::{empty_component_scene, workflow_json};`
- `🎛️apps/➗️mathematical/🎭️modes/✏️edit/🪟️windows/📐️geometry/🦀️component.rs`: `use crate::apps::mathematical::{empty_component_scene, geometry_layers_json};`

`📦️glue.rs` had both the real `#[path=".../⚙️engine/component.rs"] pub mod engine;` mount (under `standards::v1`) and a legacy shim `pub mod engine { pub use super::standards::v1::engine::*; }` — both removed (mirrors `◻2d`'s glue.rs, which carries neither).

## (d) Assertion count before vs after

- Engine file (`⚙️engine/🦀️component.rs`) before deletion: **17** assertions (`rg -c "assert"`).
- `🎛️apps/➗️mathematical/🦀️component.rs`: **14** (before) → **31** (after) — delta of exactly +17, all engine assertions landed here (the engine file had no tests on the deleted `MathematicalEngine` struct, so nothing was lost there).
- `🗿️artifacts/➗️mathematical/🦀️component.rs`: **5** → **5** (unchanged, no tests moved here).
- `🚪️io/🦀️component.rs`: **0** → **0** (the moved `io_registry` module carries no `#[cfg(test)]` tests of its own).

Total: 17 before across all sources that will vanish with the engine file; 17 additional assertions present after, all in the app top-level's test module. **No assertions lost.**

## (e) Verbatim compiler output / error attribution

Command run twice (identical result both times):
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/🎯️target" cargo check -p semio-s-plugin-mathematical --all-targets
```

Result: `semio-framework-plugin` and all upstream framework crates compiled clean (warnings only, all pre-existing/unrelated). The build then reached the workspace path-dependency `semio-s-plugin-stdio` (declared in `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml:28`) and failed there, BEFORE `semio-s-plugin-mathematical` itself was ever checked:

```
    Checking semio-s-plugin-stdio v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust)
error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/./././././././././../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`: No such file or directory (os error 2)
    --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:7024:37
     |
7024 | ...                   pub mod inverse;
     |                       ^^^^^^^^^^^^^^^^

error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
```

**Attribution: pre-existing/upstream, NOT a file I touched.** Evidence:
- `git status --porcelain -- "✏️s/🔌️plugins/🗄️stdio"` shows **646** currently-uncommitted changed paths (M/A) across the stdio plugin, including `M ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` itself and multiple `⚙️engine` dirs mid-dissolution (`☁️las`, `☁️ply`, …) — a live, in-progress refactor by another concurrent session.
- `git log -3 --oneline -- "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs"` shows only old committed history (`382ace1b27`, `20252aa16d`, `fd01661f06`) — the breakage is in the other session's uncommitted working tree, not something I introduced.
- The referenced directory `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📄set-snapshot` does not exist on disk at all — the other session's `glue.rs` edit is ahead of the files it wires in.
- `✏️s/🔌️plugins/🗄️stdio` is explicitly out of scope (CLAUDE.md: "Do NOT edit ... anything under `✏️s/🔌️plugins/🗄️stdio`") and outside this ticket's assignment (`Work ONLY inside ✏️s/🔌️plugins/➗️mathematical/**`).
- This matches a known recurring pattern (repo-wide cargo build failures caused by another session's in-progress multi-hundred-file refactor) — polling/retrying immediately is not expected to resolve it quickly.

Consequently `semio-s-plugin-mathematical` itself was never reached by `cargo check` in either run — **its own compile status could not be directly observed**, but every framework crate it depends on ahead of `stdio` in the build graph compiled clean, and all edits described above were re-read after writing and are structurally consistent (correct region markers, no leftover bare `engine::` paths, all call sites updated). This is reported honestly as blocked, not claimed as green.

## (f) Deviations from plan and why

- The four pure/derived-compute regions (`Io`, `Scene`, `GraphAlgorithms`, `Geometry`) all landed in the app's own top-level `🦀️component.rs` rather than being split across `🧬️schema/💡️inferences` — none of them are `fn(&Snapshot) -> Value`-shaped projections; `empty_component_scene` explicitly references the app type `MATH_APP_ID`, and `algorithm_overlay`/`workflow_json`/`geometry_layers_json` are consumed only by app-side window renderers and the app top-level's `export_media`, never by any artifact-schema or io-codec path — rule 8 routes exactly this case to "app behaviour," and since none of it is stateful, rule 7's `⚙️engine` stub (which exists as an empty reserved dir at `🎛️apps/➗️mathematical/⚙️engine`, confirmed empty, left untouched) was not needed.
- No content was moved into `🧬️schema/🦀️component.rs` or `🧬️schema/💡️inferences/🦀️component.rs` — nothing in the engine file was a pure document helper (`empty_x_snapshot()`/`next_id()`-shaped) or a `Snapshot`-level projection distinct from the existing `MathematicalInference`/`topology` inference.
- `MathematicalEngine` struct/impl deleted outright per rule 1 — confirmed 0 external references and 0 `ArtifactEngine` trait definitions/impls repo-wide; this is the expected/normal case per the ticket, not the "first-of-its-kind" exception.
- Compile verification is blocked by unrelated concurrent churn in `🗄️stdio` (see section e) — this is a genuine external blocker, not a deviation I chose; documented rather than worked around, since editing `🗄️stdio` is explicitly forbidden.

## (g) Files touched

Updated:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🦀️component.rs` — added `//#region 🔖️Io`, `//#region 🔖️Scene`, `//#region 🔖️GraphAlgorithms`, `//#region 🔖️Geometry`; rewired 3 internal call sites off the old `engine::` path; updated file-level doc comment; extended `mod tests` with 6 moved tests across 3 new sub-regions; added imports (`MathematicalGeometry`, `MathematicalGraph`, `SurfaceKind`, `UiComponentSceneNode`, `UiPresence`, `serde_json::{json, Value}`, `ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord}`)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — added `//#region 🚪️DerivedIoRegistry` (the real `io_registry` module) after the existing `//#region 🎹️DerivedComposition`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🦀️component.rs` — fixed `declaration()`'s `.composers(...)` call and the local `io_registry` wrapper's `use ... as v1;` import to point at `standards::v1::subsets::any::io::io_registry` instead of the deleted `standards::v1::engine::io_registry`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️component.rs` — import repointed to `crate::apps::mathematical::{empty_component_scene, workflow_json}`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎭️modes/✏️edit/🪟️windows/📐️geometry/🦀️component.rs` — import repointed to `crate::apps::mathematical::{empty_component_scene, geometry_layers_json}`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` — removed the `#[path=".../⚙️engine/🦀️component.rs"] pub mod engine;` mount under `standards::v1`, and removed the `pub mod engine { pub use super::standards::v1::engine::*; }` legacy shim

Removed:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` (directory, containing only `🦀️component.rs`, 313 LOC) — confirmed no `Cargo.toml` inside before removal

Created: none (no new files — all destinations were existing files, per the exemplar pattern).

## Structural verification

```
grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/➗️mathematical
```
→ 0 hits (empty output).

```
find ✏️s/🔌️plugins/➗️mathematical -path "*🗿️artifacts*" -name "⚙️engine" -type d
```
→ 0 hits (empty output). The app-side `🎛️apps/➗️mathematical/⚙️engine` reserved stub (empty, no files) was left untouched as instructed.

## Verdict

**Structural verification: PASS.** **Compile verification: BLOCKED** by a genuinely pre-existing/unrelated failure in `semio-s-plugin-stdio` (646 uncommitted paths mid-refactor by another concurrent session, out of scope per CLAUDE.md and this ticket) — `semio-s-plugin-mathematical` itself was never reached by `cargo check` in either of two runs, so its own compile status is unconfirmed pending that other session finishing or committing its work.
