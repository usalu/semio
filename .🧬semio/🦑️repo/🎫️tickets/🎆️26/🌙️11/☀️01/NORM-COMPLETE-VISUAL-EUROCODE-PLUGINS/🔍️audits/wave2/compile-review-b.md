# Compile review B (wave 2): shared crates, EN 1995 to EN 1999

Read-only analysis plus minimal fixes, no builds, no git writes. Scripts in `🗑️generated/` are not kept; scratch copies lived in the session scratchpad.

## Fixed (certain rustc errors)
| file:line | issue | fix |
|---|---|---|
| `🪨️en1996/…/🧬️schema/🔺️diff/🦀️.rs` (12 `list_delta!`) | `key: id` on foreign model rows implements `Keyed` for a foreign type: E0117 (the same error en1995 run 14 and en1997 run 8 hit) | local `<Row>Key` markers implementing `KeyOf`, `list: Vec<Row>, key: String = by <Row>Key` |
| `🫨️en1998/…/🧬️schema/🔺️diff/🦀️.rs` (12 `list_delta!`) | same | same |
| `🪨️en1996/…/🎬️scene/📈️diagrams/🦀️.rs:419` | `DiagramNote { id, at, text }` without `tone` (E0063) | `tone: Tone::Neutral` |
| `🪶️en1999/…/🎬️scene/📈️diagrams/🦀️.rs:493` | centroid `Marker` without `utilisation`, `bound_check`, `binding` (E0063) | `None` for all three |
| `🪵️en1995/📦️packages/🦀️rust/Cargo.toml` | root unit test file (mounted by `#[path]`) names `semio_s_artifact_norm_en1995_mut_design_cases` but the crate is no dependency (E0433, test build) | dev-dependency added (workspace entry exists) |
| `🪶️en1999/📦️packages/🦀️rust/Cargo.toml` | root mounts the diff unit test that uses `semio_s_artifact_norm_en1999_mut_member` (E0433, test build) | dev-dependency added |

## Open (for owners)
- en1998 mutations crate 5238 lines, over the 4000 budget (scan: `OVER 235674 6 5238 …en1998-mutations`).
- `NationalAnnex` moved to `compliance-core`; `dyn_enum_close!` in en1990 may need the core path (see requests.md).
- Hot spot: app-surface (foundation, live-core, live-editor, panels, viewport3d, visual-chrome) has not compiled since 06:29; 49 files changed afterwards. Read in full: device-profiles (layout, schema, gestures), themes (+schema), accessibility focus-order, manifest-and-media (first 120 lines), compliance reasons, annex-and-tables (NDP kernel API), design-case, scene-kit, projection, report-index, edit-rules (first 140 lines): no error found. Not read line by line: canvas-lowering, checks window, editor-wiring, lane-runtime, properties/controls, field-meta, viewport3d lowering, retained-commands.
- Recorded state: shared crates compiled up to the structural-scene foundation/meshing split in en1994 run 9 (18:38); the scene crate then failed only on `OwnedKind` (duplicate import, since fixed) and the `K::Snapshot: 'static` bound (fixed).

## Checks run (all clean unless listed above)
- `#[path]` targets: 0 missing across the norm tree.
- Cargo: every `workspace = true` dep exists in its workspace, every `path` dep exists, every crate is a workspace member; external crate paths used in sources vs deps (module-tree walk from `[lib] path`): only the two test-only gaps above (serde under `cfg_attr(test)` ignored).
- `use semio_s_plugin_norm_*::…` items in all five artifacts resolve (name level) and none is private.
- Struct literals vs definitions of shared structs (ViewLane, DeviceSignal, LayoutPrefs, CaseRow, Ownership, Handle, Diagram2d, Marker, …): the two errors above, nothing else.
- Exhaustive matches over `compliance::CheckStatus` (6 variants): none missing; the scene `CheckStatus` has 4 variants and its matches are complete.
- Method and associated-function names called in the five artifacts and shared crates all exist somewhere in framework or norm sources (only std names unknown to the vocabulary: cosh, tanh, repeat_n, …).
- `Type::default()` on types without `Default`: none. Duplicate top-level definitions per file: none.
- Orphan rule: only the `list_delta!` rows (fixed). The io-text/binary/sqlite crates implement compliance and pack traits for the model's snapshot type in a different crate, which the compiler rejects as E0117 unless the model re-exports an owned wrapper; this layout predates today and no build has reached those crates yet, so it is unverified.
- Source-boundary scan: compliance no longer over; over now: en1998-mutations (5238 lines), en1994-diff (4018), vdi3805-model, iso16757-model.
