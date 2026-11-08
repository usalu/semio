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
