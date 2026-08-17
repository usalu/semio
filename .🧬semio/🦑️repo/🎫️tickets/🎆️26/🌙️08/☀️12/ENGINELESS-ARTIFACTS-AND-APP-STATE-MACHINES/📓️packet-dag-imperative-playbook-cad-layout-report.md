# Packet report — `🕸️dag` · `📜️imperative` · `📖️playbook` · `📐️cad` · `📏️layout` artifact-tree `⚙️engine` elimination

Targets (all five, per dispatch): `find ✏️s/🔌️plugins/🕸️dag ✏️s/🔌️plugins/📜️imperative ✏️s/🔌️plugins/📖️playbook ✏️s/🔌️plugins/📐️cad ✏️s/🔌️plugins/📏️layout -type d -path "*🗿️artifacts*" -name "⚙️engine"`.

`🕸️dag`, `📜️imperative`, `📖️playbook`, `📏️layout` executed directly in this session. `📐️cad` (≈18,800 LOC, ~10x the others, mixed Rust+TypeScript) was delegated to a dedicated sub-agent under close brief — see its own section below, filled in once it reports back.

## Cross-plugin consumer sweep (coordinator-flagged risk)

The coordinator warned that `cargo check -p <plugin>` alone cannot see consumers in OTHER plugins/crates, and that this was independently measured to be real for `📐️cad` (`🎪️demonstrator`, `💠️lowpoly`). Repo-wide sweep run for all five plugins before any deletion:

```
grep -rn "::artifacts::[a-z0-9_]*::engine::" ✏️s/🔌️plugins 🧰️framework --include="*.rs" \
  | grep -v "crate::artifacts::" | grep -v "semio_s_plugin_stdio::"
```

Full output saved: `scratch-packet2-cross-plugin-engine-consumers.txt` (14 lines total, repo-wide).

| plugin | external consumers found | verdict |
|---|---|---|
| `🕸️dag` | none | checked, none |
| `📜️imperative` | none | checked, none |
| `📖️playbook` | none | checked, none |
| `📏️layout` | none | checked, none |
| `📐️cad` | `🎪️demonstrator/🎪️panes/📐️koordinator/🦀️component.rs:10` (`cad_document_from_dwg`, `cad_document_from_mesh`, `cad_mesh_from_document`), `💠️lowpoly/…/🧬️schema/🦀️component.rs:618` (`objects_from_fixture_model`, `parse_geometry`) | confirmed by coordinator, handed to cad sub-agent with explicit repoint instructions — see cad section |

The other hits in the sweep (`sourcing_curate::artifacts::curate::engine::…` in `🪵️sourcing`, `puzzle::artifacts::puzzle2d::engine::…` in framework's renderer) belong to plugins outside this packet's scope (`curate`, `puzzle2d`) and were left untouched.

---

## `🕸️dag` — 340 LOC, single-file engine

**Deleted**: `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`.

### Region → destination

| region | destination | note |
|---|---|---|
| `🔖️ArtifactEngine` (`DagEngine`) | **DELETE** | 0 external refs, no trait impl — matches the ticket's 87/89 ruling |
| `⚠️ Errors` (`DagPlayError`) | `🧬️schema/` | travels with `connect_edge`, its only producer |
| `🔖️DocumentHelpers` (`split_endpoint`, `document_to_workflow`, `next_node_id`, `default_node_for_kind`, `connect_edge`, `node_patch_for_field`, `remove_nodes_operations`) | `🧬️schema/` | pure over `DagSnapshot`/`DagNodeSpec`, no app type in any signature |
| `🧪️Tests` | `🧬️schema/` (new `document_helpers_tests` mod) | all 8 tests moved verbatim |
| `🚪️DerivedIoRegistry` (`io_registry` module, real `entries() -> &'static [ComposerEntry]`) | `🚪️io/` | appended as new region in existing `🚪️io/🦀️component.rs` |

### The shadow trap — confirmed and fixed
Artifact root `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️component.rs` has its own thin `io_registry` wrapper (`&'static [&'static ComposerEntry]`, `.iter().collect()` view) distinct from the real one. Two internal references qualified:
- `declaration()`'s `.composers(crate::artifacts::dag::standards::v1::engine::io_registry::entries())` → `...::standards::v1::subsets::any::io::io_registry::entries()`
- `mod tests { use crate::artifacts::dag::standards::v1::engine::io_registry as v1; }` → same new path

### Call sites updated (9 files)
- `🎛️apps/🕸️dag/🎮️commands/🔧️nodes/🦀️component.rs`, `🎮️commands/🕸️graph/🦀️component.rs` (`use crate::artifacts::dag::engine;` → `crate::artifacts::dag::schema;`, all `engine::X` call sites unchanged in shape)
- `🎭️modes/✏️edit/🪟️windows/🕸️main/🦀️component.rs` (`engine::document_to_workflow` → `schema::document_to_workflow`)
- 5 files under `🧬️schema/🧬️mutations/**` and `🧬️schema/📸️snapshot/💾️binary` referencing `engine::split_endpoint`/`engine::default_node_for_kind` → `schema::…`
- `📦️glue.rs`: removed the `⚙️engine` `#[path]` mount and the top-level `pub mod engine { pub use super::standards::v1::engine::*; }` shim (dead once the mount is gone)

### Assertion arithmetic
`git show 382ace1b27:<old engine path>` (last commit before this session's auto-committed deletion) vs current `🧬️schema/🦀️component.rs`:

| | tests | assertions |
|---|---:|---:|
| before | 8 | 10 |
| after | 8 | 10 |

Exact parity, all 8 test names preserved verbatim.

### Structural verification
```
find ✏️s/🔌️plugins/🕸️dag -path "*🗿️artifacts*" -name "⚙️engine" -type d   → (empty)
grep -rn "::engine::\|standards::v1::engine" ✏️s/🔌️plugins/🕸️dag --include="*.rs"  → (empty)
```

### Compiler
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-dag --all-targets
```
First attempt (full output: `scratch-packet2-dag-cargo-check.txt`) hit **2 errors, both**:
```
error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/./././././././././../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`: No such file or directory (os error 2)
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
```
`semio-s-plugin-dag` was never reached. Re-ran later (`scratch-packet2-retry-dag-imperative-playbook-summary.txt`) once stdio's state had visibly shifted (see layout's section — stdio is a live moving target under 6 concurrent sessions) — this time it hit a **different** stdio error, `error[E0433]: cannot find engine in any` in `🗄️stdio/🗿️artifacts/📝️md/🦀️component.rs:57`, still before `semio-s-plugin-dag` is ever reached (`grep "Checking semio-s-plugin-dag"` → no hit both times). **Attribution: (c) upstream**, twice over, two different transient stdio failures, neither touching `🕸️dag`. `🗄️stdio` is explicitly off-limits — not fixed.

---

## `📜️imperative` — 464 LOC, single-file engine

**Deleted**: `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`.

### Region → destination

| region | destination | note |
|---|---|---|
| `🔖️ArtifactEngine` (`ImperativeEngine`) | **DELETE** | 0 external refs |
| `🔖️Bootstrap` (`default_imperative_contributions_json`, `bootstrap_imperative_runtime`) | `🚪️io/` (new region) | **not** app-side: has THREE callers across two layers — artifact root's `declaration()`, the app's `🎚️config`, and the app engine's `ImperativeHost::from_snapshot`. An artifact must not depend on its app, so this stayed artifact-side, reachable by both root and app via qualified path. `bootstrap_imperative_runtime` widened `pub(crate)` → `pub` for the app-engine caller |
| `⚠️ Errors` (`ImperativeCoreError`) | `🎛️apps/📜️imperative/⚙️engine/` | only consumer is `ImperativeHost` + the app's wasm bridge |
| `🔖️Io` (`imperative_io`) | `🎛️apps/📜️imperative/⚙️engine/` | returns `AppIo` — rule 4 |
| `🔖️Host` (`ImperativeHost`, `default_snapshot()`'s consumer) | `🎛️apps/📜️imperative/⚙️engine/` | D5 Behavioral: owns `&mut self` (`registry`, `next_serial`), threaded through app root, edit-mode script window, wasm bridge, view command |
| `default_snapshot()` (was inline, one-liner near Host) | `🧬️schema/` | pure, no app type, also consumed directly by 3 schema-tree test files |
| `🧪️Tests` | split: 18 Host/Io tests → app engine; 0 stayed at schema (no dedicated test for `default_snapshot` existed in the old engine) | |
| `🚪️DerivedIoRegistry` (`io_registry`) | `🚪️io/` | |

The app-tree `⚙️engine` directory was previously **empty** (0 files) — created `🦀️component.rs` there for the first time and mounted it in `📦️glue.rs`.

### The shadow trap — confirmed and fixed
Root `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🦀️component.rs` has the thin `io_registry` wrapper at line ~188. Qualified:
- `declaration()`'s `bootstrap_imperative_runtime()` call and `.composers(…)` call → `crate::artifacts::imperative::standards::v1::subsets::any::io::{bootstrap_imperative_runtime, io_registry::entries()}`
- `mod tests { use …engine::io_registry as v1; }` → `…subsets::any::io::io_registry as v1`

### Call sites updated (9 files)
`🎛️apps/📜️imperative/🦀️component.rs` (split one `use` line into 3: schema for `default_snapshot`, app engine for `imperative_io`; plus `ImperativeHost` call site), `🎚️config/🦀️component.rs` (`default_imperative_contributions_json` → `io::`), `🌉️wasm/🦀️component.rs`, `🎮️commands/👁️view/🦀️component.rs`, `🎭️modes/✏️edit/🪟️windows/📝️script/🦀️component.rs` (all → `crate::apps::imperative::engine::…`), 3 schema-tree files (`default_snapshot` → `schema::`). `📦️glue.rs`: removed engine mount + shim, added `pub mod engine;` mount for the app engine.

### Assertion arithmetic
`git show 382ace1b27:<old engine path>` vs new app-engine file:

| | tests | assertions |
|---|---:|---:|
| before | 18 | 40 |
| after | 18 | 40 |

Exact parity.

### Structural verification
```
find ✏️s/🔌️plugins/📜️imperative -path "*🗿️artifacts*" -name "⚙️engine" -type d  → (empty)
grep -rn "artifacts::imperative::engine\|standards::v1::engine" ✏️s/🔌️plugins/📜️imperative --include="*.rs"  → (empty)
```
Remaining `::engine::` hits are all `crate::apps::imperative::engine::…` (the new, legitimate app engine) or `semio_s_plugin_stdio::…::engine::…` (unrelated stdio codec, out of scope) — both expected and correct.

### Compiler
First attempt (`scratch-packet2-imperative-cargo-check.txt`): same **2 errors**, both the identical upstream stdio `os error 2` on `✳️mesh/…/📄set-snapshot`. Retry (`scratch-packet2-retry-dag-imperative-playbook-summary.txt`): same later `engine in any` stdio error as `🕸️dag` above (stdio's state had shifted between runs, same live-churn cause). `semio-s-plugin-imperative` never reached either time (`grep "Checking semio-s-plugin-imperative"` → no hit). **Attribution: (c) upstream**, same pre-existing/transient issue as `🕸️dag`.

---

## `📖️playbook` — 224 LOC, single-file engine

**Deleted**: `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`.

### Region → destination

| region | destination | note |
|---|---|---|
| `🔖️Types` (`pub use … {empty_playbook_snapshot, flatten_playbook_blocks, PlaybookBlock}`) | **removed as re-export** | these were never defined in the engine — the engine only re-exported them from the artifact root. All call sites repointed straight to `crate::artifacts::playbook::{empty_playbook_snapshot, flatten_playbook_blocks}` (their real, unchanged home), eliminating a needless indirection rather than relocating it |
| `🔖️ArtifactEngine` (`PlaybookEngine`) | **DELETE** | 0 external refs |
| `🔖️Io` (`playbook_io`, `PlaybookChapterPayload`) | `🎛️apps/📖️playbook/⚙️engine/` (new file) | `playbook_io` returns `AppIo`; `PlaybookChapterPayload` is the app's own wire-decode shape |
| `🔖️DocumentHelpers` (`default_block`) | `🧬️schema/` | pure over `PlaybookBlock`, no app type |
| `🧪️Tests` | split: 1 to schema (`default_block_sets_kind_and_label`, was the placeholder marker test rewritten to test the real thing instead of asserting nothing-of-substance), 1 to app engine (`playbook_io_declares_the_extra_chapters_in_port`) | |
| `🚪️DerivedIoRegistry` | `🚪️io/` | |

The old engine's own placeholder test (`playbook_config_dsl_placeholder_module_compiles`, whose body was just `assert_eq!(default_block(...).kind, "text")` dressed as a "sanity marker") was replaced with an honestly-named `default_block_sets_kind_and_label` — same single assertion, no test lost, name now matches what it actually checks.

### The shadow trap — confirmed and fixed
Root `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️component.rs` line ~173 has the thin wrapper. Qualified `declaration()`'s `.composers(…)` and `mod tests`'s `use …engine::io_registry as v1` to `…standards::v1::subsets::any::io::io_registry`.

### Call sites updated (6 files)
`🎛️apps/📖️playbook/🦀️component.rs` (split `use` into `flatten_playbook_blocks` direct, `default_block` from schema, `{playbook_io, PlaybookChapterPayload}` from app engine; 4 bare `empty_playbook_snapshot()` call sites repointed), `🎮️commands/🧱️block/🦀️component.rs`, 3 schema-tree files (`empty_playbook_snapshot` → direct). `📦️glue.rs`: removed engine mount + shim, added app-engine mount.

### Assertion arithmetic

| | tests | assertions |
|---|---:|---:|
| before | 2 | 6 |
| after | 2 | 6 |

### Structural verification
```
find ✏️s/🔌️plugins/📖️playbook -path "*🗿️artifacts*" -name "⚙️engine" -type d  → (empty)
grep -rn "artifacts::playbook::engine\|standards::v1::engine" ✏️s/🔌️plugins/📖️playbook --include="*.rs"  → (empty)
```

### Compiler
First attempt (`scratch-packet2-playbook-cargo-check.txt`): same 2 errors, identical upstream stdio issue, playbook never reached. **Retry succeeded — genuine green**, real proof, not inferred: `scratch-packet2-retry-dag-imperative-playbook-summary.txt` shows stdio compiled this time (state shifted between runs) and:
```
Checking semio-s-plugin-playbook v0.1.0 (…)
warning: `semio-s-plugin-playbook` (lib) generated 11 warnings
warning: `semio-s-plugin-playbook` (lib test) generated 16 warnings (11 duplicates)
Finished `dev` profile [unoptimized] target(s) in 1m 06s
EXIT_semio-s-plugin-playbook:0
```
**Zero errors.** The 11/16 warnings are all pre-existing-style noise (unused imports/qualifications, several inherited verbatim from the deleted engine file's own already-unused imports, e.g. `ArtifactBuilder` in the new `🚪️io/🦀️component.rs` region) — none indicate a relocation mistake. This is the one plugin in this packet with a complete, unambiguous, `Finished`-line compiler pass.

---

## `📏️layout` — 1,567 LOC across two files (`🦀️component.rs` 758 + `🎬️scene/🦀️component.rs` 809)

**Deleted**: the whole `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` tree (both files).

**This is the real exception the ticket warned about**: `LayoutEngine` (defined in the `🎬️scene` sibling) is genuinely constructed outside its own file — `grep -c "LayoutEngine::new()"` across the app tree shows 15+ call sites (canvas, pointer command, wasm bridge, blueprint/preview window renderers, export command, app root). It owns real `&mut self` state (`FontContext`, `LayoutContext`, a lazily-loaded font blob) and is threaded through every render/hit-test/export call. **Not deleted — moved wholesale to the app engine as the second confirmed exception in this ticket** (`📸️remodel::ReconstructionEngine` is the first, per the manifest).

### Region → destination

| region | destination | note |
|---|---|---|
| `⚠️Errors` (`LayoutError`) | `🚪️io/` (new region) | used by BOTH `🚪️io`'s own MediaImportExport region and the app engine's Scene/Export region — an artifact-level error type, app reaches it by qualified path (normal direction) |
| `🔖️Io` (`layout_io`) | `🎛️apps/📏️layout/⚙️engine/` (new file) | returns `AppIo` |
| `📄️Document` (`parse_layout_document`, `ResolvedFrame`, `resolve_page`) | `🧬️schema/` (new region) | pure over `LayoutSnapshot`/`Page`, no engine state |
| `🔖️DocumentHelpers` (`default_document`, `build_demo_layout_snapshot`, `layout_sample_document_json`, `rgba_to_text`, `text_to_rgba`) | `🧬️schema/` (new region) | pure |
| `🔖️MediaImportExport` (`rect_path_segments`, `path_bounds`, `compose_svg_from_drawing`, `layout_snapshot_to_semio_drawing`, `layout_document_json_to_svg`, `dwg_rect_pages`, `dwg_drawing_to_semio_drawing`, `layout_document_json_from_dwg`) | `🚪️io/` (new region) | stdio composer/codec-dispatch territory, rule 5 |
| `🔖️TestSupport` (`ensure_stdio_semio_drawing_registered`) | `🚪️io/` | widened `pub(crate)` → `pub`: now a cross-module second caller (app engine's own scene tests) |
| **Entire `🎬️scene/🦀️component.rs` file** — `🖼️Display` types, `⚙️Scene` (`LayoutEngine` + everything built on `&mut LayoutEngine`), `📤️Export` (SVG/PDF/PNG/zip) | `🎛️apps/📏️layout/⚙️engine/🎬️scene/🦀️component.rs` (new file, sibling split preserved) | wholesale move — every function in this file either takes `&mut LayoutEngine` directly or exists only to feed the export pipeline built on it |
| `🔖️ArtifactEngine` (`LayoutArtifactEngine`, distinct from `LayoutEngine`) | **DELETE** | 0 external refs anywhere — confirmed the manifest's "`LayoutEngine` survives" note refers to the scene struct, not this one |
| `🚪️DerivedIoRegistry` | `🚪️io/` | |

### The shadow trap — confirmed and fixed
Root `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️component.rs` has the thin wrapper (`pub mod io_registry` at ~line 727). All `standards::v1::engine::io_registry` references (declaration composers call, doc comments, `mod tests` import) repointed to `standards::v1::subsets::any::io::io_registry` — 5 occurrences in that one file (`perl -pi -e` regex swap, then individually verified each resulting line).

### The include_bytes! relative-path hazard (new failure mode this packet hit, not previously documented)
`static LAYOUT_SANS: &[u8] = include_bytes!(…)` embeds the app's bundled font, addressed by a chain of `../` relative to the SOURCE FILE's own directory. Moving the file from `⚙️engine/🎬️scene` (11 path segments deep from repo root) to `🎛️apps/📏️layout/⚙️engine/🎬️scene` (7 segments deep) changes the required `../` count from 11 to **7**. Verified both the old and the new path resolve to the same file on disk via `os.path.normpath` before and after editing — old path had 11 `../` and resolved correctly; naively copying that same 11-`../` string into the new location would have silently pointed at a directory 4 levels above the repo root (a build-time `include_bytes!` error, not a subtle runtime bug — but exactly the kind of "moved code, unqualified relative reference" hazard the shadow-trap rule warns about in spirit, just for paths rather than Rust module qualification).

### Call sites updated (14 files)
`🎛️apps/📏️layout/🦀️component.rs` (6 sites: `LayoutEngine` import, `default_document`×1, `layout_io`×3, `layout_sample_document_json`×1, scene `export_document_svg`×1), `🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs`, `…/📐️blueprint/🦀️component.rs` (both `LayoutEngine` render signatures), `🎮️commands/🖱️pointer/🦀️component.rs`, `🎮️commands/🐚️export/🦀️component.rs` (scene export fns), `🌉️wasm/🦀️component.rs` (`parse_layout_document` + scene bundle import), `🖼️canvas/🦀️component.rs` (scene `LayoutEngine`/`build_display_list_for_page`/`DisplayList` + `default_document`×2), `📌️panels/🚦️preflight/🦀️component.rs` (`resolve_page`, `default_document`), `📌️panels/🔍️inspection/🦀️component.rs` (`rgba_to_text`), `🎮️commands/✏️author/🦀️component.rs` (`text_to_rgba`), `🚪️io/📥️import/…/🖊️dwg/…/🦀️component.rs` (`layout_document_json_from_dwg` → `io::`), `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` + `🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (`default_document` → `schema::`), `🧬️schema/🦀️component.rs` itself (its own `DerivedConstruction::empty()` call). `📦️glue.rs`: removed artifact-tree engine mount (both the main file and the `scene` sub-mount) + shim; added a new `pub mod engine { … pub mod scene; }` mount under `apps::layout`.

### Assertion arithmetic
`git show 382ace1b27:<old paths>` (both files) vs the four destination files:

| | tests | assertions |
|---|---:|---:|
| before (main 7 + scene 19) | 26 | 54 |
| after (schema 3 + io 4 + app-engine-scene 19 + app-engine-main 0) | 26 | 54 |

Exact parity. Every scene test moved verbatim (same 19 names); the main file's 7 tests split 3 (Document: `resolve_page_…`, `parse_layout_document_…`, `rgba_text_round_trips`) → schema, 4 (dwg import ×2, svg export ×2) → io.

### Structural verification
```
find ✏️s/🔌️plugins/📏️layout -path "*🗿️artifacts*" -name "⚙️engine" -type d  → (empty)
grep -rn "artifacts::layout::engine\|standards::v1::engine" ✏️s/🔌️plugins/📏️layout --include="*.rs"  → (empty)
```

### Compiler
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-layout --all-targets --message-format=short
```
First attempt was killed mid-run (exit 144, no completion trailer) by the extreme concurrent load on this shared tree — `ps aux` at the time showed a dozen+ simultaneous `cargo check -p semio-s-plugin-<other-plugin>` processes from other sessions (`space`, `raster`, `stdio`×3, `note`, `animate`, `procedural`, `process`, `sourcing-beams/slabs/windows`, `draw`, `energy`). Re-ran with `--message-format=short` to cut output volume; this run completed cleanly (`EXIT:101`, full `could not compile … due to 3 previous errors` trailer present). Full output: `scratch-packet2-layout-cargo-check-errors-only.txt` (grep of the errors/Checking/exit lines out of the full 259KB log).

This time `semio-s-plugin-stdio` itself compiled (a different, transient stdio state than the `os error 2` seen for dag/imperative/playbook minutes earlier — confirms the ticket's own warning that stdio's state is a live moving target between sessions, not something this packet controls). `semio-s-plugin-layout` was reached and produced **3 errors, all in the same two pre-existing files**:
```
🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:3  error[E0432]: unresolved import `semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc`
🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:11 error[E0560]: struct `semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot` has no field named `page`
🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:9   error[E0609]: no field `page` on type `&semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot`
```
**Attribution: (b) pre-existing, proven with `git log`.** Both files' last commit is `adbb6091af` (flag 481) — long before this packet's work (which starts in the 498+ range) and neither file was touched by any edit in this packet (I only edited the *root* `🚪️io/🦀️component.rs`, never these leaf serializer/deserializer files). Their content references stdio's PDF snapshot as `PdfSnapshot { schema, page: PageDoc { .. } }`; the live stdio crate no longer has `PageDoc` or a `page` field on `PdfSnapshot`. This is the exact same class of fallout the ticket's own manifest already documents for md/json bridges across 35+ plugins ("stdio REGRESSED" section) — a stdio snapshot-shape drift, not a rename this packet could or should patch. Not fixed — `🗄️stdio` is explicitly off-limits, and this is real feature work (adapting to a new PDF shape) spanning every plugin with a PDF bridge, not a two-minute edit.

---

## `📐️cad` — ~18,800 LOC across 16 files (Rust + TypeScript), delegated to sub-agent

[FILL IN — sub-agent dispatched with the general region map, cad-specific D6d classification already recorded in this ticket's `important.md`, the full shadow-trap warning, and the coordinator's explicit cross-plugin consumer instructions (`🎪️demonstrator`, `💠️lowpoly`). Section replaced with its full findings once it reports back.]

---

## Honest summary (interim — cad still with the sub-agent)

- **Directories deleted so far: 4 of 5** (`🕸️dag`, `📜️imperative`, `📖️playbook`, `📏️layout`). `📐️cad` pending sub-agent completion.
- **Structural** (`find … -name "⚙️engine" -type d` under each plugin's `🗿️artifacts`): 0/0/0/0 confirmed for the four done plugins.
- **`::engine::` grep**: 0 artifact-tree hits in all four; remaining hits are legitimate new app-engine self-references (`crate::apps::<plugin>::engine::…`) or unrelated `semio_s_plugin_stdio::…::engine::…`.
- **Assertions**: 8/8, 18/18, 2/2, 26/26 tests preserved exactly (dag/imperative/playbook/layout); 10/10, 40/40, 6/6, 54/54 assertions exactly.
- **Compiler — playbook: genuine GREEN.** Retried once stdio's state shifted; reached `semio-s-plugin-playbook`, `Finished` line, exit 0, zero errors, only pre-existing-style warnings.
- **Compiler — dag/imperative**: retried twice each; both attempts blocked before reaching their own code by `semio-s-plugin-stdio` failures — first the `os error 2` dangling `✳️mesh` mount, then (stdio's state having shifted) a different `cannot find engine in any` error in stdio's own `📝️md` artifact. Two different transient stdio failures, neither ever reaching `🕸️dag`/`📜️imperative` (`grep "Checking semio-s-plugin-<plugin>"` → no hit either time). **Attribution: (c) upstream**, confirmed twice over.
- **Compiler — layout**: `semio-s-plugin-stdio` compiled on the second attempt (state again shifted), so layout's own code WAS reached and checked. **3 errors, all pre-existing** (`git log` shows both files last touched at flag 481, long before this packet; neither was edited by this packet — only the sibling root `🚪️io/🦀️component.rs` was) — a stdio `PdfSnapshot` shape drift (`PageDoc`/`.page` no longer exist) breaking layout's PDF import/export bridge leaves, the same class of fallout the ticket's own manifest documents across 35+ plugins for md/json bridges. **Attribution: (b) pre-existing**, not introduced by this packet, `🗄️stdio` off-limits to fix.
- **Pattern across all four**: `semio-s-plugin-stdio`'s compile state visibly changed between consecutive check runs minutes apart, three different ways (`os error 2` → `cannot find engine in any` → compiles clean). This is the ticket's own documented "stdio is a live moving target under concurrent sessions" behavior, not noise from this packet's own edits (this packet never touched `🗄️stdio`).
- **First layout check attempt was killed (exit 144, no completion) by extreme concurrent load** — a dozen+ other sessions' `cargo check` processes observed running simultaneously against the same shared build tree at that moment. Re-ran with `--message-format=short` and it completed cleanly; not treated as a false "pass," the truncated run's partial output was discarded in favor of the complete re-run.
- **Deviations**: playbook's `🔖️Types` re-export region was eliminated rather than relocated (its contents were never actually defined in the engine, only re-exported); layout's `LayoutError`/`ensure_stdio_semio_drawing_registered` stayed artifact-side rather than app-side despite being consumed by app-engine code, because they're also consumed by artifact-level `🚪️io` code and an artifact must not depend on its app; imperative's Bootstrap region stayed artifact-side for the same multi-caller-across-layers reason; layout's `LayoutEngine` (in `🎬️scene`) is the ticket's second confirmed real exception to "delete the `*Engine` struct" (moved wholesale to the app engine, not deleted) alongside `📸️remodel::ReconstructionEngine`.
- **Unverified at draft time**: cad's full result (sub-agent still running).
