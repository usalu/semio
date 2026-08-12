# W1b — `semio-s-plugin-puzzle` → `ArtifactDeclaration` conversion

`apa-status: clean` — three declarations landed (one per artifact: `◻2d`, `🖐️5d`, `🧊️3d`), the umbrella
`register()`/`register_io()` escape hatches are gone, a real pre-existing bug (puzzle3d/puzzle5d
grammars never registered) was fixed as a side effect, one cross-plugin compile dependency
(`🎪️demonstrator`) was discovered and preserved rather than broken, and `cargo check -p
semio-s-plugin-puzzle --all-targets` (`RUSTC_WRAPPER=""`) compiles with **0 errors** — real,
compiler-verified.

## Clearance

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`.
`🧩️puzzle` appears under **RELEASED** (SMO's own mutation-facet lane, not APA's) — not HELD, not
listed as another session's. Per that file's own wording ("ABSENCE FROM THIS FILE MEANS FREE"),
FREE either way. Proceeded.

## Changes (file:line)

### `.artifact()` wiring — plugin root

- **`✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs`** — `plugin()` now calls `.artifact(…::declaration())`
  three times (one per artifact) instead of `.setup(crate::artifacts::puzzle2d::engine::register)`.
  Added a private `setup()` free function (:19, called via one `.setup(setup)`) that combines the
  two surviving no-declaration-field escape hatches — `PluginBuilder::setup` is a single
  `Option<fn()>`, not repeatable, so four separate `.setup(...)` calls would have silently kept only
  the last and dropped the other three; caught and fixed before it became a real regression.

### Three declarations — one per artifact engine

- **`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`**
  — `register()` (:51, old) → `declaration()` (:59): `.schema(puzzle2d_artifact_schema_descriptor())`,
  `.composers(…standards::v1::engine::io_registry::entries())`, `.languages(pilot_languages())`,
  `.document_codec::<Puzzle2dPlayApp>()`, kind `"s.puzzle2d"`. `register_media_io()` (:73) and
  `register_app_schemas()` (:83) kept, now called from the plugin root's `setup()`.
  `register_pilot_languages()` (void, called) → `pilot_languages()` (:91, `&'static` via `OnceLock`,
  same pattern as `🗒️note`'s exemplar).
- **`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`**
  — `register_io()` (:648, old) → `declaration()` (:659): `.schema(...)`,
  `.inferences([puzzle3d_artifact_inference_descriptor()])`, `.composers(...)`,
  `.languages(pilot_languages())`, `.document_codec::<Puzzle3dPlayApp>()`, kind `"s.puzzle3d"`.
  `register_mesh_io()` (:677, renamed from the old `register_io()`'s mesh-only half) kept, called from
  `setup()`. `register_pilot_languages()` (:568, void, **dead — zero call sites before this change**)
  → `pilot_languages()` (:572), now actually wired into `.languages(...)` — puzzle3d's own grammars
  were never registered before this conversion; fixed as a direct consequence of the M1 pattern, not
  separately.
- **`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`**
  — same shape: `register_io()` (:285, old) → `declaration()` (:296), kind `"s.puzzle5d"`,
  `register_mesh_io()` (:313). `register_pilot_languages()` (:206, void, **dead**) →
  `pilot_languages()` (:210) — same bug, same fix.

### App-file cleanup (dead wrapper deletion + one restore)

- **`✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs`** — deleted `register_puzzle2d_exports()`
  (its only caller, puzzle2d's `register()`, is gone; zero other callers repo-wide, grep-verified).
  `.document_codec::<Puzzle2dPlayApp>()` on the declaration now does the same work declaratively.
- **`✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs`** — same deletion for
  `register_puzzle5d_exports()`, same reasoning, zero other callers confirmed.
- **`✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs`** — **`register_puzzle3d_exports()` was NOT
  deleted**, unlike its 2d/5d siblings. Repo-wide grep found
  `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🧩️aggregator/🦀️component.rs` imports and calls it directly
  (`use puzzle::apps::puzzle3d::{…, register_puzzle3d_exports, …}`) as its one cross-plugin
  host-export entry point. Deleting it would have broken `🎪️demonstrator`'s compile — a different
  plugin, out of this ticket's edit scope. Kept `pub`, body unchanged, doc comment updated to explain
  why it survives alongside the declaration's own `.document_codec::<Puzzle3dPlayApp>()`. The
  resulting double-registration when both `🧩️puzzle` and `🎪️demonstrator`'s aggregator pane load in
  the same process is not new — the old umbrella `register()` and demonstrator's pane already called
  this same function independently before this change.

## `.setup()` status and why

Two calls survive on `.setup()`, combined into one `setup()` free function
(`✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs:22-27`) because `PluginBuilder::setup` holds a single
`fn()`, not a list:

1. **`register_app_schemas()`** (app-scope config/presence schema for all three play apps) — the
   documented, by-design exception. `ArtifactDeclaration` has no field for
   `register_app_schema_descriptor`; not a finding.
2. **`register_media_io()` / `register_mesh_io()` × 2** (2d SVG/DWG bridge, 3d and 5d mesh
   exporter/importer/dwg bridges via `semio_framework_os::register_2d_export_handlers` /
   `register_dwg_import_handler` / `register_mesh_exporter` / `register_mesh_importer` /
   `register_mesh_dwg_export_handler` / `register_mesh_dwg_import_handler`) — **a genuine finding,
   reported loudly rather than silently kept.** These are NOT part of the nine §6 registrars
   `ArtifactDeclaration` covers. They are the OS media-host registry, a separate 14-function family
   (census: `📓️w0-census.md`/`📓️w0-d-sdk-surface.md`, `register_2d_export_handlers`,
   `register_dwg_import_handler`, `register_mesh_exporter`, `register_mesh_importer`,
   `register_mesh_dwg_export_handler`, `register_mesh_dwg_import_handler`, `register_solid_exporter`,
   `register_solid_importer`, `register_app_io`) predating and orthogonal to M1's `ArtifactDeclaration`
   mechanism. No field exists to move them into; `ArtifactDeclaration` would need a new field (or this
   family needs its own declarative wrapper) before these can leave `.setup()`. Puzzle is the plugin
   with the largest share of this family (6 of the 8 functions, per the W0 census: `2d.puzzle` ×2,
   `3d.puzzle` ×4 mesh, `5d.puzzle` ×4 mesh) — worth flagging to whoever owns that follow-on wave.

No other `.setup()` calls survive; every other §6-covered registration moved into one of the three
`declaration()`s.

## `kit.catalog` — confirmed, not touched

Puzzle **does** declare `kit.catalog`: `kit_catalog_artifact_kind()` at
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️component.rs:522`, registered via
`.artifact_kind(crate::artifacts::puzzle3d::kit_catalog_artifact_kind())` inside
`create_puzzle3d_app()` at `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs:2417`. This is a
DIFFERENT builder call from anything `ArtifactDeclaration` touches — `.artifact_kind()` is an
`ArtifactApp`/app-builder method registering an `ArtifactKindSpec` (the older OS catalog vocabulary),
not one of the nine §6 `ArtifactDeclaration` registrars — so it was left exactly as found, per the
dispatch's explicit "do NOT rename" instruction. Its own doc comment already notes it is
"harmless if a producer, e.g. block3d, declares an identical spec" — i.e. the duplicate-with-block
sharing is acknowledged and intentional in the existing code, not something this conversion
introduced or needs to resolve. Flagging here per the dispatch's "UCAS needs the map" instruction so
UCAS's roster work knows both `🧩️puzzle` and `🧱️block` declare `kit.catalog`.

## Inventory (Step 5 — NOT fixed, reported only)

- **`thread_local!` holding the entire app** (the largest draft-lane debt named in the dispatch):
  - `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs:1876-1879` —
    `PUZZLE3D_PLAY_SESSION: RefCell<Puzzle3dPlayApp>`.
  - `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs:1295-1298` —
    `PUZZLE5D_PLAY_SESSION: RefCell<Puzzle5dPlayApp>`.
  - Both hold live user-gesture app state (the whole `Puzzle{3d,5d}PlayApp`), not a derived cache —
    matches the dispatch's own framing exactly. Not touched.
- **Other interior-mutable statics, minor, likely-benign** (not host/engine handles):
  `PUZZLE3D_MESH_REGISTRY: LazyLock<Mutex<HashMap<…>>>` and `PUZZLE3D_ID_COUNTER`/
  `PUZZLE5D_ID_COUNTER: AtomicU32` (both in the respective `🎛️apps/…/🦀️component.rs` root files) —
  id counters and a mesh-bytes cache, not framework/host handles. The `OnceLock<Vec<ComposerEntry>>`
  and `LazyLock<ExampleSource>`/`LazyLock<String>` statics scattered through `🗿️artifacts/…` and
  `📚️examples/…` are legitimate build-once derived-data caches (composer tables, parsed example
  fixtures), not flagged.
- **`std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]`**: only
  `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs` (a Cargo build script — icon asset copying,
  `CARGO_MANIFEST_DIR`/`OUT_DIR` reads) — the sanctioned location for this, not a runtime violation.
  Zero other hits.

## Verification

1. **`#[path]` resolution in `📦️glue.rs`**: 502 non-`"."` `#[path]` attributes, all resolved against
   the real filesystem relative to `📦️glue.rs`'s own directory — **0 missing**.
2. **`include_str!`/`include_bytes!`**: none of the 7 files this conversion touched contain any
   (schema `FacetLeaves` descriptors — the files that DO carry `include_str!` — were read, not
   edited, so nothing to re-resolve there).
3. **`cargo metadata --no-deps --format-version 1`**: exits 0, empty stderr → **OK**.
4. **`RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-puzzle
   --all-targets`**: **0 errors.** Took 3 attempts to get real signal — not because of my changes,
   but because of extreme concurrent lock contention on this ticket's shared `CARGO_TARGET_DIR`
   (13–23 other plugins' W1b sessions running `cargo check` against the same target dir
   simultaneously — confirmed via `lsof`/`ps`, a different plugin's process held the exclusive
   `.cargo-build-lock` at one point). Attempts 1 and 2 each ran to real completion but stopped short
   of ever reaching `semio-s-plugin-puzzle` because `semio-s-plugin-stdio` (a dependency, owned by
   UCAS's in-flight roster work — same documented pattern as the `🗒️note` exemplar's own W1 report)
   was red; both attempts' full error lists had **zero mentions of any `🔌️plugins/🧩️puzzle` path**.
   Attempt 3 (the one pasted below) finally caught stdio green and ran all the way through:

```
    Checking semio-s-plugin-puzzle v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust)
[... 70 lib warnings + 82 test warnings, all pre-existing (unused-variable `app` in test blocks,
     unused-extern-crate on glue.rs, etc.) — none on any line this conversion touched, confirmed by
     grepping the full output for declaration()/register_media_io/register_mesh_io/
     pilot_languages/setup() and finding zero warnings or errors on any of them ...]
warning: `semio-s-plugin-puzzle` (lib) generated 70 warnings (run `cargo fix --lib -p semio-s-plugin-puzzle` to apply 56 suggestions)
warning: `semio-s-plugin-puzzle` (lib test) generated 82 warnings (68 duplicates) (run `cargo fix --lib -p semio-s-plugin-puzzle --tests` to apply 11 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 8m 08s
```

   `grep -c "^error"` on the full attempt-3 log → **0**. Full log:
   `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-w1b-puzzle-check-3.txt`.

## sharedFileRequests

None requiring another session's action. One cross-plugin **read-only** dependency noted for the
record: `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🧩️aggregator/🦀️component.rs` imports
`puzzle::apps::puzzle3d::register_puzzle3d_exports` — not edited, not broken; documented above so
whoever eventually retires that function (once `🎪️demonstrator` is APA'd) knows both sides.

## Honest pass/fail

- `.setup()` → `.artifact()` for all three artifacts: **done, three declarations, kind strings
  verified against each artifact's own `Dialect.artifact_kind`** (`s.puzzle2d`/`s.puzzle3d`/
  `s.puzzle5d` — NOT the `ArtifactSchemaDescriptor.id` values, which are the differently-shaped
  `s.puzzle.puzzle2d` etc.; same distinction the `🗒️note` exemplar established).
- `.setup()` narrowed to exactly two justified escape hatches, combined into one callback because
  `PluginBuilder::setup` is a single slot — caught before it silently dropped three of four calls.
- Plugin root closed to only `🦀️component.rs`/`AGENTS.md`/`README.md`/`🎛️apps`/`🗿️artifacts`/
  `📦️packages` — **already true before this session**, confirmed, nothing to delete.
- `kit.catalog`: **confirmed declared, location reported, untouched** per instruction.
- Inventory (thread_local, static handles, std::fs/env/process): **done, reported, not fixed**, per
  instruction.
- One real bug fixed as a side effect: puzzle3d's and puzzle5d's own DSL grammars were dead code
  (defined, never called) — now wired through `.languages(pilot_languages())`.
- One cross-plugin compile break avoided: `register_puzzle3d_exports()` kept alive for
  `🎪️demonstrator`.
- `cargo metadata`: **OK**. `#[path]` census: **502/502 resolve**. `cargo check --all-targets`:
  **0 errors, real compiler output pasted in Verification §4** (3rd attempt; the first two were
  blocked by swarm-wide `CARGO_TARGET_DIR` lock contention and pre-existing, unrelated
  `semio-s-plugin-stdio` churn, never by anything in `semio-s-plugin-puzzle` itself).
