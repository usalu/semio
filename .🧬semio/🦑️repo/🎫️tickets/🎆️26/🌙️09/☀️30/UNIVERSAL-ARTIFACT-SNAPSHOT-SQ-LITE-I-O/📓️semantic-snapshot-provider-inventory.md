# Semantic Snapshot Provider Inventory

Retained planning input: `🔣️semantic-snapshot-inventory.json`. This inventory records exact source ownership, verified native coordinates, existing schema sources, local snapshot declarations and entity fields. It is not a runtime schema generator.

## Coverage Counts

- 96 artifact roots across stdio and nonstdio plugins.
- 149 subset schema roots: 89 stdio and 60 nonstdio.
- 149 distinct native dialect coordinates; all mapped, no unresolved coordinate.
- All149roots have language-neutral JSON schemas. Snapshot source inventory includes789named entity declarations and3700Rustfields; this is planning data, not automatic SQL mapping.
- 118 roots contain dedicated snapshot source; 120 named snapshot declarations include nested sum/alias types and therefore do not imply 120 independent native providers.
- 31 subset roots lack their own snapshot source; another DWG source reexports AC1024 snapshot. Conformance subsets intentionally share typed native snapshots.
- 88 typed stdio editor rows; GeoJSON schema is the additional stdio subset outside that typed shipped catalog.

## Handwritten Provider Architecture

Define a framework-owned relational document contract (tables, columns with INTEGER/REAL/TEXT/BLOB domain scalar types, rows, foreign keys, constraints, ordered ownership). Implement every artifact native snapshot provider by hand beside its owning subset in existing `🚪️io/🦀️.rs` and a handwritten SQL schema under the owning schema tree. Register its exact dialect with native snapshot assembly, and reject missing providers. Shared subsets may use one identical typed provider with per-coordinate conformance validation; do not regenerate a generic schema from DslValue or pack metadata.

A semantic snapshot exports domain entities, columns and relationships directly. CSV uses csv_record and csv_field with record/order keys and quoted Boolean; SemioKit uses kit_type, design, piece(type_id FK), connection(two piece IDs and ports), representations and owned child tables; mesh uses vertices/faces/face_vertex ordering; graph uses nodes/edges/source-target FKs; media uses track/frame/sample/channel/pixel entities with individual typed sample values; compressed bitmap/audio/video payload must not stand in for domain rows. JSON/XML require handwritten syntax-domain node/member/attribute relationships, not a generic cross-artifact property bag. Native binary artifact can use ordered byte INTEGER rows; embedding any artifact pack wholesale is forbidden.

Import validates schema shape, SQLite affinity/scalars, unique keys, foreign keys, nullability, finite numbers, ordered rows, and typed domain invariants; reconstruct typed snapshot directly. Include file pages/row batching limits, cancellation/progress in both directions and deterministic writes. No external runtime DB library should leak into the provider API. Rust and TypeScript use the same handwritten schema and independent domain vectors with a third-party SQLite implementation as oracle.

## Existing SQL Implementations

No existing per-artifact semantic SQL schema was found under `✏️s/🔌️plugins`. Existing domain relational examples are `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` (users, spaces, memberships, document descriptors, FKs), `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` (jobs/events/progress/approvals), and `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/🧬️schema/🗄️.sql` (repo/folder/file/section hierarchy). These show handwritten schema and row patterns but are not snapshot converters. OS DB snapshot_generation.bytes and the current IO artifact_snapshot.snapshot are opaque BLOB persistence, so they do not meet the updated task.

## Complete Subset Ownership Roster

| Dialect | Local Snapshot Types / Shared Type References | Subset Source Root |
| --- | --- | --- |
| `s.writer.writer@1/*` | WriterSnapshot | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.mathematical.equation@1/*` | EquationSnapshot, EquationExprSnapshot | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.wfc.wfc2d@1/*` | Wfc2dSnapshot | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.wfc.grid2d@1/*` | Grid2dSnapshot | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.wfc.bitmap@1/*` | BitmapSnapshot | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.wfc.wfc3d@1/*` | Wfc3dSnapshot | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.wfc.grid3d@1/*` | Grid3dSnapshot | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.procedural.generation2d@1/*` | Generation2dSnapshot | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.procedural.generation3d@1/*` | Generation3dSnapshot | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.flow.flow@1/*` | FlowSnapshot | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.gis.gisterrain@1/*` | GisTerrainSnapshot | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.gis.gismap@1/*` | GisMapSnapshot | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.vcs.vcs@1/*` | VcsSnapshot | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.animate.presentation@1/*` | PresentationSnapshot | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.shooting.shooting@1/*` | ShootingSnapshot | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.demonstrator.playground@1/*` | PlaygroundSnapshot | `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.sequence.sequence@1/*` | SequenceSnapshot, SequenceHostSnapshot | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.fem.fem2d@1/*` | Fem2dSnapshot | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any` |
| `s.fem.fem3d@1/*` | Fem3dSnapshot | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any` |
| `s.architect.program@1/*` | ProgramSnapshot | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.process.process3d@1/*` | Process3dSnapshot | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.lowpoly.lowpoly@1/*` | LowpolySnapshot | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.reasoning.wires@1/*` | WiresSnapshot | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.forms.forms@1/*` | FormsSnapshot | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.layout.layout@1/*` | LayoutSnapshot | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.cad.cad@1/*` | CadSnapshot | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1990@1/*` | En1990Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.din18599@1/*` | Din18599Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1997@1/*` | En1997Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.din16798@1/*` | Din16798Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1991@1/*` | En1991Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1992@1/*` | En1992Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.vdi3805@1/*` | Vdi3805Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.iso16757@1/*` | Iso16757Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1993@1/*` | En1993Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1994@1/*` | En1994Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.din4108@1/*` | Din4108Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1996@1/*` | En1996Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1995@1/*` | En1995Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1999@1/*` | En1999Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.norm.en1998@1/*` | En1998Snapshot | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.playbook.playbook@1/*` | PlaybookSnapshot | `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.imperative.procedure@1/*` | ProcedureSnapshot | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.remodel.remodeling@1/*` | RemodelingSnapshot | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.energy.model@1/*` | EnergyModelSnapshot | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.trinity.rewriting@1/*` | RewritingSnapshot | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.trinity.jack@1/*` | JackSnapshot | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.dag.dag@1/*` | DagSnapshot | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.draw.drawing@1/*` | DrawingSnapshot | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.raster.raster@1/*` | RasterSnapshot | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.stdio.las@1.0/*` | LasSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header` |
| `s.stdio.html@5/*` | HtmlSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any` |
| `s.stdio.epw@energyplus/*` | EpwSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any` |
| `s.stdio.zip@2.0/iso21320` | shared: SetSnapshot, ZipSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320` |
| `s.stdio.zip@2.0/*` | ZipSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base` |
| `s.stdio.gif@87a/*` | GifSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any` |
| `s.stdio.gif@89a/*` | GifSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base` |
| `s.stdio.mp4@isobmff/*` | Mp4Snapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any` |
| `s.stdio.svg@1.1/tiny` | shared: EditSnapshot, SetSnapshot, SvgSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny` |
| `s.stdio.svg@1.1/basic` | shared: EditSnapshot, SetSnapshot, SvgSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic` |
| `s.stdio.svg@1.1/*` | SvgSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base` |
| `s.stdio.mp3@mpeg1-layer3/*` | Mp3Snapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any` |
| `s.stdio.ifc@4/*` | IfcSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any` |
| `s.stdio.ifc@2x3/cobie` | shared: EditSnapshot, Ifc2x3Snapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie` |
| `s.stdio.ifc@2x3/cv20` | shared: EditSnapshot, Ifc2x3Snapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20` |
| `s.stdio.ifc@2x3/sav` | shared: EditSnapshot, Ifc2x3Snapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav` |
| `s.stdio.ifc@2x3/*` | Ifc2x3Snapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base` |
| `s.stdio.bcf@2.1/*` | BcfSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup` |
| `s.stdio.binary@raw/*` | BinarySnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any` |
| `s.stdio.csv@rfc4180/*` | CsvSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any` |
| `s.stdio.step@ap214/cc1` | shared: EditSnapshot, SetSnapshot, StepSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1` |
| `s.stdio.step@ap214/cc2` | shared: EditSnapshot, SetSnapshot, StepSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2` |
| `s.stdio.step@ap214/cc3` | shared: EditSnapshot, SetSnapshot, StepSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3` |
| `s.stdio.step@ap214/cc4` | shared: EditSnapshot, SetSnapshot, StepSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4` |
| `s.stdio.step@ap214/cc5` | shared: EditSnapshot, SetSnapshot, StepSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5` |
| `s.stdio.step@ap214/cc6` | shared: EditSnapshot, SetSnapshot, StepSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6` |
| `s.stdio.step@ap214/*` | StepSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base` |
| `s.stdio.tsv@iana/*` | TsvSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any` |
| `s.stdio.xlsx@ecma-376/transitional` | shared: SetSnapshot, XlsxSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional` |
| `s.stdio.xlsx@ecma-376/strict` | shared: SetSnapshot, XlsxSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict` |
| `s.stdio.xlsx@ecma-376/*` | XlsxSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base` |
| `s.stdio.pdf@1.4/x` | shared: PdfSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x` |
| `s.stdio.pdf@1.4/a` | shared: PdfSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a` |
| `s.stdio.pdf@1.4/*` | PdfSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base` |
| `s.stdio.pdf@1.7/ua` | shared: PdfSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua` |
| `s.stdio.pdf@1.7/h` | shared: PdfSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h` |
| `s.stdio.pdf@1.7/e` | shared: PdfSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e` |
| `s.stdio.pdf@1.7/x` | shared: PdfSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x` |
| `s.stdio.pdf@1.7/a` | shared: PdfSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a` |
| `s.stdio.pdf@1.7/*` | PdfSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base` |
| `s.stdio.pdf@1.7/vt` | shared: PdfSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt` |
| `s.stdio.docx@ecma-376/strict` | shared: DocxSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict` |
| `s.stdio.docx@ecma-376/transitional` | shared: DocxSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional` |
| `s.stdio.docx@ecma-376/*` | DocxSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base` |
| `s.stdio.md@commonmark/*` | MdSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any` |
| `s.stdio.xml@1.0/valid` | shared: EditSnapshot, SetSnapshot, XmlSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid` |
| `s.stdio.xml@1.0/*` | XmlSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base` |
| `s.stdio.png@1.2/*` | PngSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any` |
| `s.stdio.jpg@jfif-1.01/baseline` | shared: EditSnapshot, JpgSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline` |
| `s.stdio.jpg@jfif-1.01/*` | JpgSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document` |
| `s.stdio.avi@1.0/*` | AviSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl` |
| `s.stdio.pptx@ecma-376/transitional` | shared: PptxSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional` |
| `s.stdio.pptx@ecma-376/strict` | shared: PptxSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict` |
| `s.stdio.pptx@ecma-376/*` | PptxSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base` |
| `s.stdio.wav@riff-pcm/*` | WavSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any` |
| `s.stdio.txt@utf-8/*` | TxtSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any` |
| `s.stdio.stl@ascii/*` | StlSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any` |
| `s.stdio.dwg@ac1018/*` | shared: DwgSnapshot, EditSnapshot, SetSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any` |
| `s.stdio.dwg@ac1024/*` | DwgSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any` |
| `s.stdio.dxf@r12/*` | DxfSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header` |
| `s.stdio.tiff@6.0/baseline` | shared: EditSnapshot, SetSnapshot, TiffSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline` |
| `s.stdio.tiff@6.0/*` | TiffSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document` |
| `s.stdio.deflate@rfc1950/*` | DeflateSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any` |
| `s.stdio.obj@3.0/*` | ObjSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry` |
| `s.stdio.gltf@2.0/*` | GltfSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any` |
| `s.stdio.ply@1.0/*` | PlySnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any` |
| `s.stdio.json@rfc8259/geojson` | shared: JsonSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson` |
| `s.stdio.json@rfc8259/i-json` | shared: EditSnapshot, JsonSnapshot, PatchSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json` |
| `s.stdio.json@rfc8259/*` | JsonSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base` |
| `s.stdio.semio@v1/*` | SemioSubsetSnapshot, SemioSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base` |
| `s.stdio.semio@v1/flow` | SemioFlowSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow` |
| `s.stdio.semio@v1/animation` | SemioAnimationSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation` |
| `s.stdio.semio@v1/video` | SemioVideoSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video` |
| `s.stdio.semio@v1/model` | SemioModelSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model` |
| `s.stdio.semio@v1/table` | SemioTableSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table` |
| `s.stdio.semio@v1/cad` | SemioCadSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad` |
| `s.stdio.semio@v1/document` | SemioDocumentSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document` |
| `s.stdio.semio@v1/object` | SemioObjectSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object` |
| `s.stdio.semio@v1/presentation` | SemioPresentationSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation` |
| `s.stdio.semio@v1/audio` | SemioAudioSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio` |
| `s.stdio.semio@v1/value` | SemioValueSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value` |
| `s.stdio.semio@v1/text` | SemioTextSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text` |
| `s.stdio.semio@v1/mesh` | SemioMeshSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh` |
| `s.stdio.semio@v1/graph` | SemioGraphSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph` |
| `s.stdio.semio@v1/drawing` | SemioDrawingSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing` |
| `s.stdio.semio@v1/image` | SemioImageSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image` |
| `s.stdio.semio@v1/brep` | SemioBrepSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep` |
| `s.stdio.semio@v1/kit` | SemioKitSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit` |
| `s.stdio.bmp@v3/*` | BmpSnapshot | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any` |
| `s.note.note@1/*` | NoteSnapshot | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.puzzle.puzzle2d@1/*` | Puzzle2dSnapshot | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.puzzle.puzzle5d@1/*` | Puzzle5dSnapshot | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.puzzle.puzzle3d@1/*` | Puzzle3dSnapshot | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.block.block2d@1/*` | Block2dSnapshot | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.block.block5d@1/*` | Block5dSnapshot | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.block.block3d@1/*` | Block3dSnapshot | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.space.home@1/*` | SHomeSnapshot | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.space.space@1/*` | SSpaceSnapshot | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `s.sourcing.curation@1/*` | CurationSnapshot | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any` |

## Domain Allocation Counts

| Plugin | Subset Schemas |
| --- | --- |
| `✒️writer` | 1 |
| `➗️mathematical` | 1 |
| `🀄️wfc` | 5 |
| `🌀️procedural` | 2 |
| `🌊️flow` | 1 |
| `🌍️gis` | 2 |
| `🌿️vcs` | 1 |
| `🎞️animate` | 1 |
| `🎥️shooting` | 1 |
| `🎪️demonstrator` | 1 |
| `🎬️sequence` | 1 |
| `🏗️fem` | 2 |
| `🏛️architect` | 1 |
| `🏭️process` | 1 |
| `💠️lowpoly` | 1 |
| `💡️reasoning` | 1 |
| `📋️forms` | 1 |
| `📏️layout` | 1 |
| `📐️cad` | 1 |
| `📕️norm` | 15 |
| `📖️playbook` | 1 |
| `📜️imperative` | 1 |
| `📸️remodel` | 1 |
| `🔋️energy` | 1 |
| `🔱️trinity` | 2 |
| `🕸️dag` | 1 |
| `🖍️draw` | 1 |
| `🖨️raster` | 1 |
| `🗄️stdio` | 89 |
| `🗒️note` | 1 |
| `🧩️puzzle` | 3 |
| `🧱️block` | 3 |
| `🪐️space` | 2 |
| `🪵️sourcing` | 1 |
