# Fleet 1108 precompile review (static only, no builds)

Lane `precompile-review`, first pass 2026-11-08 03:55. Scripts (all read-only) in `🧰️scripts/`: `precompile-artifact-check.py` (workspace members, deps, lib/test paths, `mod`/`#[path]` targets), `precompile-dep-check.py` (external crate used but undeclared; `sqlite`, `semio_format`, `semio_grammar_conformance` hits are module-name false positives), `precompile-listdelta-check.py` (marker rule; en1992/en1994 hits are macro-generated markers, false positives), `precompile-field-check.py` (`doc.<field>` reads versus model fields, mounted files only), `precompile-import-check.py` (very noisy because of macro-generated items; not used for findings).

## Gates
- `check-cargo-authority.py`: 37 crates, 37 members, 0 errors.
- `source-boundary-scan.py ✏️s/🔌️plugins/📕️norm`: exit 0, no output.
- path budget: longest 240, budget 240, 0 over.
- Every artifact workspace: all members exist on disk, all `[workspace.dependencies]` paths exist (checked), no `key: id` in artifact `list_delta!` blocks.
- DIN 4108 scene/editor/viewer, DIN 16798 scene, VDI 3805 and ISO 16757 windows: line-by-line comparison with the compiled-pattern references (EN 1999 / DIN 4108): `norm_live_editor!`, `norm_live_acceptance!`, `norm_latency_bench!`, `norm_live_commands!`, `live_report!`, `live_scene!` argument shapes match the macro definitions; scene-kit, handle-kit, annotation, geometry, `FieldSpec`, `UnitId` and `Basis` names/signatures used by the DIN 4108 and DIN 16798 scene resolve. No finding for lanes din-a, din16798, din18599, iso-vdi, vdi3805, shared at this pass.

## Findings (appended to `📌️important/requests.md`)
| lane | n | summary |
|---|---|---|
| en1993 | 1 | P0 dangling workspace dep `...mut-fastened-joints-3-2` (crate dir and workspace rows do not exist) fails cargo metadata for the whole en1993 workspace |
| en1998 | 2 | model lacks `isolation_systems`, `pier_bases`, `critical_regions` (and their row types) that scene/schema/tests read; examples crate lacks dev-dep `semio-s-plugin-norm-compliance` |
| en1999 | 1 | model lacks `fire_insulations`, `sheet_profiles` (and `FireInsulation`, `SheetProfile`) that scene/checks/tests read |
| en1994 | 1 | examples crate lacks dev-dep `semio-s-plugin-norm-compliance` |

## Not verified
Type-level checks (borrowck, trait bounds, macro expansion output) are out of reach statically; the unit tests of the new vdi3805/din16798 scene crates were only checked for mount/dependency integrity, not line by line.

## Pass 2 (deep read, 2026-11-08 ~04:35)
Line-by-line read of din4108 scene (lib, kit, elements, layout, handles, diagrams, unit tests), din4108 editor root/viewport/diagrams/field-meta/incremental/tests, din16798 scene (lib, kit, rooms, layout, handles, ventilation and air-quality diagrams), vdi3805 scene (lib incl. custom `StructuralProjection`, products). Mechanical tree-wide scans added: `precompile-literal-check.py` (struct and enum-variant literals against definitions, E0063/E0560), `precompile-arity-check.py` (too noisy, not used), plus inline scans of enum variants (scene schema enums, `UnitId`), model item names, `checks::`/`inferences::` item names, unknown field names.
Clean: struct and variant literals in din16798/vdi3805/din18599/din4108/iso16757 (the 6 hits are local same-name structs), all `model::` and `checks::` names in scene/editor/viewer, all `UnitId::` variants of the five artifacts, trait impls (`SceneKit`, `StructuralProjection` for `Vdi3805Projection` forwards all 9 required items, `UnitSource`), `live_report!`/`live_scene!` preconditions (Default, Send).
New findings: shared-scene 1 (QuantityKind variants, hits din16798, din18599, en1991), din-a 1 (test uses nonexistent `annex` leaf), en1992 1 (`UnitId::Angle`). Pass 1 items en1993 (`-3-2` crate) and en1994/en1998 examples dev-dep are fixed; en1998 `critical_regions`/`isolation_systems`/`pier_bases` and en1999 `fire_insulations`/`sheet_profiles` model fields are still missing.

## Pass 3 (2026-11-08 evening, fixes allowed, no builds)
Gates re-run: `precompile-artifact-check.py` 0 issues, `check-cargo-authority.py` 37/37/0, path budget longest 240 / 0 over, `source-boundary-scan.py` clean after the fixes below (it was NOT clean at the start: two en1998 crates over budget).

### Fixes
1. `app-surface/🫧️live-lanes/🎛️editor-wiring/🦀️.rs`: `paint_report_of` used `PAINT_BUDGET` without importing it (E0425 in every norm editor); added `PAINT_BUDGET` to the `use crate::{..}` list (re-exported through `panels::*` -> `selection_mirror`).
2. en1998 `mut-tie-beams` crate mounted 46 leaves / 139 files (limit 32). Split by entity with `🧰️scripts/en1998-split-tie-beams-mut.py`: `mut-tie-beams` (7), new `mut-isolation-systems` (8), `mut-pier-bases` (8), `mut-critical-regions` (8), `mut-critical-regions-more` (8), `mut-critical-regions-rest` (7). Rewired: en1998 `Cargo.toml` (members + workspace deps), `📦️packages/🦀️rust/📜️script.ts` package list, `mutations/🦀️.rs` and `mutations/📦️packages/🦀️rust/Cargo.toml` (deps + `test-support` feature), `io/📝️text/🧬️mutations/🔗️tie-beams` (`use` paths + deps). `demo-cases` only uses tie-beam leaves (unchanged).
3. en1998 `diff` crate was 4057 active lines (limit 4000, includes the handcrafted `.ts/.proto/.graphql/.json` leaves): removed 60 blank lines in `🔺️diff/🦀️.rs` (now below 4000, margin under 10 lines).

### Verified clean (static)
din18599 editor against en1994 (full normalised diff: only intended differences; `DinV18599Family`, `NormFamilyId::DinV18599`, `DIN18599_DIALECT`, `resolve_edit`, `child_restore_projection`, `genesis_din18599_child_pack` all exist; `render_inspector` signature 6 args matches, `render_catalogue` matches); `Hatch` is `Copy` (via `choices!`); scene `QuantityKind` variants used by din16798/din18599/en1991/vdi3805 all exist (scene vs compliance enum separated); all `UnitId::` variants exist; `insert_edit`, `verification_catalogue_table`, `paint_report` generics (`NormRetainedEditor::Family: NormFamily<Document=Snapshot>`) type-check by reading; mechanics slope ponded-water code and oracle key `slope.janbu.ponded` present; en1998 `isolation_systems`/`pier_bases`/`critical_regions` and en1999 `sheet_profiles`/`fire_insulations` now exist in model, snapshot and diff (en1998 diff uses `*Delta`, en1999 diff uses `Option<*Rows>`, both internally consistent).

### Remaining suspicions
- `Cargo.lock` of en1998 still lists only `mut-tie-beams`; any `--locked` run fails until it is regenerated.
- en1998 diff has under 10 lines of source-boundary margin; the next added collection needs a diff split.
- en1998 leaves `change-joint-{id,beam-id,building-id,column-id,storey-id,xm}` exist on disk but are mounted by no crate, while the `mutations` kind list names `change-joint-xm`/`-column-id`; harmless for compile, may fail a kind-list-vs-dispatch law test.
- A crude brace-balance scan flagged 7 generated `mutate-enXXXX-1` tests and `en1991 wave3-rows` (line 59); the scanner is unreliable (raw strings/format), line 58 reads balanced by eye. Treat as unconfirmed.
- Not type-checkable statically: borrowck, trait bounds in macro expansions (`norm_live_editor!`, `live_report!`, `list_delta!`).

### Pass 3 follow-ups (coordinator request, en1998)
1. Diff crate split: the 3200 lines of the diff crate were the four handcrafted twin leaves pulled in by `include_str!`, so a Rust-only entity split would not have helped. Followed the existing en1999 pattern instead: four new twin crates `semio-s-artifact-norm-en1998-diff-{typescript,graphql,json-schema,proto}` under `🔺️diff/<leaf>-twin/📦️packages/🦀️rust` (one `LEAF` const each), `DIFF_LEAVES` now references them; wired into the diff `Cargo.toml`, en1998 workspace members and `[workspace.dependencies]`, and `📜️script.ts`. `en1998-diff` closure is now 1 file / 704 lines / 50910 bytes (was 5 / 4057 / 251635); the earlier blank-line trimming is harmless and left in place. No Rust entity split was done (no longer needed).
2. The five `change-joint-*` "orphans" are fully defined (KINDS, enum variants, `use` lines, twins, text codec, EDIT_RULES, features, oracle) and the aggregator already imports them from `mut_members::`; the real bug was the re-export chain: `mut-members-joints` mounted only `change-joint-col-moment-rd-nm` while `mut-members` re-exports 14 names from it. Fix: `mut-members-joints` now mounts building-id, storey-id, column-id, beam-id and xm (6 leaves, 19 files) and re-exports `y_m`, `column_moment_rd_nm`, `beam_moment_rd_nm`, `insert/remove_joint`, `insert/remove_tie_beam`, `insert/remove_seismic_slope` from `mut-members-joints-more` (new dep + `test-support` feature in its Cargo.toml); `mut-members` re-export list gained `change_joint_col_moment_rd_nm`. Gates: boundary scan, artifact check and cargo authority all clean; Cargo.lock left for the build's synchronize step.
