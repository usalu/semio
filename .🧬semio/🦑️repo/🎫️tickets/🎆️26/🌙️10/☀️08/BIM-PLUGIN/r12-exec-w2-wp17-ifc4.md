# r12 execution report: w2-wp17-ifc4 (WP-17 IFC4 and IFC coverage)

T = ticket folder, S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, E = `S/🚪️io/📤️export/🏗️ifc`, M = `S/🚪️io/📥️import/🏗️ifc`.

## 1. Result in one table

| Item | State |
|---|---|
| IFC4 (ADD2 TC1) export of every BIM family, one writer for both schemas | written; lib compiled green at the time of every verified run (see 4) |
| IFC4 import of the same families; IFC2x3 import completion for roofs, stairs, railings, curtain walls | written; round trip proven byte-stable (see 4) |
| Export as stepped job with progress and cancel (`ifc2x3`, `ifc4`) | written; staged == one-shot proven |
| M5 (exports read elevations from the inference) | done for columns, beams, slabs, ceilings, building datum |
| ifcopenshell validation (schema + WHERE rules), kernel volumes, quantities | oracle agrees for `house` and `psets`; the other six cases were re-blessed after fixes but could not be re-run (crate red by peers, see 5) |
| IFC4.3 | out of scope, justified in 6 |
| Components / MEP in IFC4 | owned by `w2-f3-assets` (uses my `Export::by` / `STAGES`); I fixed one IFC4 validity bug there |

## 2. Design

- One writer, two schemas. `E/🧰️writer`: `Schema::{Ifc2x3, Ifc4}`, `Ifc::in_schema`; IFC4 writes no owner history (author and organization travel in `FILE_NAME`), `Ifc::units()` is shared by project and library. `Export::by(v2x3, v4)` picks the attribute tail per schema.
  Every family emitter writes its IFC4 tail: `PredefinedType`, `IfcWindow` partitioning / `IfcDoor` operation (occurrence repeats its type), `IfcWindowType` / `IfcDoorType` / `IfcRoofType`, `IfcMaterialLayer.Category` = layer function, `IfcMaterial.Category`, `IfcMaterialProperties`, quantity `Formula`, `ReferenceExtent`, `IfcGrid` / `IfcZone` extra attributes; in IFC4 an `IfcGridAxis` is part of exactly one list, so a one-direction grid repeats its first axis as a second entity (the importer skips it, ids name each line once); `IfcRamp` never writes `USERDEFINED` without `ObjectType`; `IfcQuantityLength` is never negative (descending ramp height is its absolute value in IFC4).
- IFC4 bodies of meshes are `IfcTriangulatedFaceSet` (`E/🧊️brep`: `mesh_item`, `body_kind`); IFC 2x3 keeps faceted breps. Family-profile (`Profile::Family`) columns and beams, which the exporter used to skip, are now meshes with the `Column` / `Beam` record.
- IFC4 project library (`E/📚️ifc4`): `IfcProjectLibrary` declared by the project, one `IfcPropertySetTemplate` per template (id in `Description`), classification systems as chains of `IfcClassificationReference` (`ReferencedSource` = parent, `Sort` = table position, id in the system `Description`), `IfcRelAssociatesClassification` per code to every holder, `IfcRelDefinesByTemplate` for every property set named like a template (emitted at the end of `emit_links`). The standalone library serializer is gone (one IFC4 hop per dialect); `ModelIntoIfc4` replaces it.
- Staged export (`STAGES`, `StagedExport`, `Staged`): one family per step, no borrow between steps; the `exportModel` job formats are `ifc2x3`, `ifc4`, `glb`, `svg`, `csv`; analysis is the first half of the progress, writing the second half; cancel drops the writer and keeps finished inference nodes. Labels `export_ifc2x3`, `export_ifc4` (en, de), chrome toggle per format.
- M5: elevations of columns, beams, slabs and ceilings come from `x.solid(id).bounds`, the absolute building elevation from the inferred storey levels; `frames::vertical` and its test are deleted.
- Authored records for what IFC cannot express (`Semio_Authoring`, canonical JSON): `Roof`, `Stair`, `Railing`, sloped `Slab`, non-explicit space `Boundary`, tilted/family `Column`, joined/arc/family `Beam`; roofs are now typed by their roof type.
- Import (`M`): `Import.schema`, `import_document(schema, &doc)`, `Ifc2x3IntoModel` / `Ifc4IntoModel`; roofs (`⬜️horizontal`), stairs and railings (`🪜️circulation`), curtain walls (`🪟️curtain`, existed but was not mounted) restore from their record, with a best-effort reading for foreign files (flat swept roof, straight stair from its flights, railing from its polyline); IFC4: window/door types (partitioning, operation), roof types, material and layer categories, `IfcMaterialProperties`, templates, classification chains in `Sort` order, author and organization from the header, family profiles.
- stdio IFC (outside the crate): `decode_ifc4_document` / `encode_ifc4_document` in the v4 io (schema check, unique ids), unit test green; fixed the stale `IfcValue::Ref/Int` in the v4 edit-rules tests.

## 3. Files

Export: `E/🧰️writer`, `🧊️brep`, `🦀️.rs` (Schema, Staged, STAGES, StagedExport, encode, export_ifc4), `🧬️data`, `📚️ifc4`, `🏛️spatial`, `🏰️walls`, `🧷️wall-sweeps`, `🪟️curtain`, `🏗️frame`, `⬜️horizontal`, `🔲️ceilings`, `🪜️circulation`, `🛝️ramps`, `🏠️spaces`, `🏘️zoning`, `📏️grids`, `🧭️frames`, `🔬️projection` (`counted_in`, `COUNTED_4`, `projection_in`, chained classification parents), `🪑️components` (one line), tests `🧪️tests/🔭️schema4` (new, 29 tests) and the touched unit tests. Import: `M/🦀️.rs`, `🧬️data`, `🏛️spatial`, `📖️reader` (`path`), `🏗️elements`, new `⬜️horizontal`, `🪜️circulation`, tests. Editor: `🎮️commands/📤️export-model`, `🎛️chrome`, `🗣️terminology`, `🦀️.rs` bridge default `ifc2x3`. IO: `S/🚪️io/🦀️.rs` entries. stdio: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/.../4️⃣4/.../🚪️io/🦀️.rs` (+ test).
Oracle (new case, own directory): `S/🧪️tests/🏢️export-bim-1-ifc4/{🥒️.feature,🐍️.py,🦀️.rs}` (9 scenarios: house, psets, ceilings, notated, ramps, wall-depth, round trip house and psets, stepped job), oracle row `bim-1-ifcopenshell-ifc4` in `S/🔮️oracles/🔣️.json`, fixtures `S/🧫️fixtures/🏢️ifc4/<case>/<case>.ifc` + `🔬️measure/🔣️.json` for the six cases and the two shipped examples (house, office). Kept scripts in T: `r12-w2-wp17-ifc4-*` (schema probe and diff, oracle pieces, bless tests).

## 4. Commands and exact results (when the crate compiled)

- `cargo test --lib -- schema4_tests` (BIM_BLESS=1): 17 passed, 1 failed (the oracle-table test, before the python tables existed). Passing included: IFC4 house exports without skips; no owner history; attribute counts of 21 entity classes; predefined types; door/window types carry operation and partitioning; roof types; triangulated face sets only; quantity formula slot; materials and layers; psets file with templates/chains; **export -> import -> export byte stable for the whole house in both schemas and for the psets model**; IFC4 import gives roofs, stairs, railings, curtain walls, types back; staged export equals one-shot in both schemas; column length equals the inferred solid extent.
- python oracle `🐍️.py write` house and psets: `oracle agrees (ifcopenshell 0.8.4.post1)`: schema IFC4, EXPRESS validation clean, kernel volumes equal the written quantities, library tables equal the snapshot.
- python oracle on the other cases found real defects, all fixed in source afterwards: duplicate-list `IfcGridAxis` (notated), negative `IfcQuantityLength` and `USERDEFINED` without `ObjectType` (ramps), arc slabs counted as exactly measurable (example house), openings of curtain walls counted, family-profile beams skipped (example office), abstract `IfcFlowTerminalType` (office components, now `IfcAirTerminalType` in IFC4). The oracle tolerance between kernel volume and written quantity is 1e-8 relative (nanometre-rounded directions), the harness comparison stays at 1e-9.
- stdio: `cargo test -p semio-s-artifact-stdio-ifc --lib the_ifc4_document_codec`: ok.

## 5. Open items (could not be verified, crate red from peers since ~17:00)

1. Re-run after the fixes of section 4: `cargo test --lib -- schema4_tests::the_committed` with `BIM_BLESS=1` (rewrites the eight IFC4 files; the test is now one test per case, so nothing runs for minutes any more: the earlier "hang" was one test exporting eight models including the 250 kB office in a debug build on a loaded machine plus the oracle-table test failing before the tables existed), then `python 🐍️.py write <S/🧫️fixtures>` of the new case (all eight), then `cargo test --lib -- schema4_tests io::export::ifc io::import::ifc export_model`.
2. IFC 2x3 fixtures change with this package (roofs typed, new records, z from solids): `r13-integrate` or the next blesser must re-bless the committed 2x3 files and python measure tables (`house.ifc`, notated, ceilings, ramps, psets, wall-depth) once.
3. Foreign-file import is best effort for pitched roofs, curved stairs and railings without polyline (reported, not guessed).
4. Not run: wasm32-wasip2 check, the generators (`r3-f1-gen-*`; I added no mutation), `bun ./📜️script.ts oracle quick --case 🏢️export-bim-1-ifc4`, `dependency` regeneration for the new oracle id.

## 6. Exclusions

- IFC4.3: the stdio IFC artifact has `2x3` and `4` only (`IFC4X3` occurs only in fixture text); the `Schema` enum is the extension point.
- Components/MEP: written by `w2-f3-assets` on top of `Export::by`; see its report.
