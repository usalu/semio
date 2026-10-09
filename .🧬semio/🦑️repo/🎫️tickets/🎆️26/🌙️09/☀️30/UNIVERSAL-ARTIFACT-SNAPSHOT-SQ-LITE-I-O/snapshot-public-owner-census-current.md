# Public Snapshot Owner Source Census

Observed 2026-10-09T15:06:14 local source; concurrent modifications remain possible. No tests run, no runtime qualification or installed-provider denominator.

This explicit-impl census finds 154 explicit trait declarations: 137 outside test paths and 17 in test paths. These are declarations, not unique artifacts: split/cfg implementation files, generic impls and internal probes remain distinct source rows. Every explicit implementation must supply semantic SQLITE_SCHEMA/project/reconstruct by trait requirement. Presence alone does not prove independent relational fidelity. Native override and retirement evidence below is confined to each actual impl block. Macro-generated implementations can require separate expansion review.

Store trait `🏪️store/🦀️.rs:12211–12230` defaults retirement to bare drop and native decode/encode/preflight to UnsupportedOwner. Therefore an SQL schema with absent productive native override cannot qualify public native-to-SQL-to-native IO. Native override presence does not prove its internal partial construction is retained. `RetireOwned::controlled_retirement_supported` is a separate contract from the trait's retire_sqlite_snapshot method: genuine receiving-frame admission requires real supported RetireOwned for every field.

## Production / Internal Source Declarations

| Owner | File:line | Decode override | Encode override | SQL retirement override | Preflight override |
|---|---|---|---|---|---|
| ChartSnapshot | ./🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | 15 | 16 | 19 | 17 |
| FormsSnapshot | ./✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:56 | 62 | 63 | 60 | 61 |
| Block2dSnapshot | ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:314 | 316 | 331 | default | default |
| Block5dSnapshot | ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:379 | 381 | 396 | default | default |
| WorkflowSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | 35 | 38 | default | 41 |
| RunArtifact | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:25 | 27 | 30 | default | 33 |
| Generation2dSnapshot | ./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:6 | 7 | 8 | 9 | 13 |
| NoteSnapshot | ./✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:112 | 118 | 113 | 119 | 120 |
| Generation3dSnapshot | ./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:6 | 7 | 8 | 10 | 13 |
| EnergyModelSnapshot | ./✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:13 | 16 | 17 | default | 15 |
| CurationSnapshot | ./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:27 | 32 | 29 | default | 35 |
| ProcedureSnapshot | ./✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:21 | 24 | 25 | default | 23 |
| NativeSocketProbeSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🪶️sqlite/🦀️.rs:29 | 39 | 45 | default | 50 |
| SSpaceSnapshot | ./✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:10 | 13 | 12 | default | 20 |
| Block3dSnapshot | ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:361 | 365 | 380 | default | 362 |
| PlaygroundSnapshot | ./✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:15 | 18 | 19 | 45 | 17 |
| Grid2dSnapshot | ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | 36 | 37 | default | 51 |
| SHomeSnapshot | ./✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:13 | 16 | 15 | default | 17 |
| PresentationSnapshot | ./✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:15 | 20 | 17 | default | 28 |
| super::ProbeSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🪶️sqlite/🦀️.rs:5 | default | default | default | default |
| Wfc2dSnapshot | ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:19 | 21 | 22 | default | 35 |
| BitmapSnapshot | ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:25 | 27 | 28 | default | 41 |
| DagSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:15 | 19 | 20 | 22 | 21 |
| LowpolySnapshot | ./✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | 36 | 35 | default | 32 |
| CadSnapshot | ./✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️owner/🦀️.rs:226 | 233 | 243 | 228 | 229 |
| CadSnapshot | ./✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:240 | 246 | 258 | 242 | default |
| FlowHostSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛂️capability/🦀️.rs:5 | 9 | 10 | 12 | 11 |
| WriterSnapshot | ./✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:10 | 11 | 12 | default | 14 |
| Grid3dSnapshot | ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:28 | 30 | 31 | default | 48 |
| PlaybookSnapshot | ./✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:28 | 31 | 32 | default | 30 |
| SequenceSnapshot | ./✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:7 | 10 | 14 | 9 | 19 |
| SpaceHistorySnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:94 | 101 | 97 | default | 96 |
| En1997Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:42 | 44 | 46 | default | 67 |
| RasterSnapshot | ./✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:138 | 143 | 149 | 142 | default |
| Wfc3dSnapshot | ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:28 | 30 | 31 | default | 46 |
| CollectionSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:24 | 31 | 27 | default | 26 |
| SpaceSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:23 | 30 | 26 | default | 25 |
| WiresSnapshot | ./✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | 37 | 35 | 36 | 38 |
| ObjSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:152 | 161 | 153 | default | 162 |
| crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot | ./✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:291 | 297 | 296 | default | 293 |
| Vdi3805Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:90 | 92 | 93 | default | 101 |
| GisTerrainSnapshot | ./✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:12 | 20 | 14 | default | 24 |
| TsvSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:7 | 11 | 12 | 14 | 13 |
| TsvSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:8 | default | default | default | default |
| CsvSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:7 | 11 | 12 | 14 | 13 |
| RewritingSnapshot | ./✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | 15 | 18 | 23 | 22 |
| Iso16757Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:141 | 143 | 145 | 153 | 154 |
| JackSnapshot | ./✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:419 | 428 | 446 | 460 | default |
| GisMapSnapshot | ./✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:31 | 32 | 33 | 36 | 37 |
| DrawingSnapshot | ./✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:86 | 91 | 92 | 90 | default |
| CsvSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:8 | default | default | default | default |
| Fem2dSnapshot | ./✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | 34 | 35 | default | default |
| VcsSnapshot | ./✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | 35 | 42 | default | 47 |
| Mp3Snapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:60 | 61 | 63 | default | 68 |
| En1994Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:48 | 49 | 51 | default | 57 |
| BmpSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:6 | 10 | 11 | 13 | 12 |
| BmpSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:13 | default | default | default | default |
| StepSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:66 | 68 | 69 | 67 | 70 |
| En1993Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:65 | 67 | 69 | default | 104 |
| WavSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:201 | 202 | 203 | default | 204 |
| EpwSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:24 | 25 | 27 | default | 32 |
| ZipSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:56 | 58 | 59 | default | 57 |
| ModelSnapshot | ./✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:408 | 414 | 413 | default | 410 |
| GifSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:26 | 34 | 27 | default | 35 |
| Ifc2x3Snapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:71 | 73 | 74 | 72 | 79 |
| Din18599Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:52 | 53 | 55 | default | 61 |
| DeflateSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:5 | 9 | 10 | 12 | 11 |
| DeflateSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:7 | default | default | default | default |
| GifSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:32 | 40 | 33 | default | 41 |
| IfcSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:46 | 49 | 50 | 48 | 51 |
| Fem3dSnapshot | ./✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | 37 | 38 | default | default |
| En1996Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:66 | 67 | 69 | default | 75 |
| HtmlSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:49 | 50 | 51 | 52 | 53 |
| LasSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:29 | 30 | 31 | 32 | 33 |
| SemioValueSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:105 | 110 | 108 | 106 | 111 |
| Din16798Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | 35 | 37 | default | 43 |
| DxfSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:115 | 120 | 116 | default | 124 |
| En1990Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:103 | 104 | 107 | default | 113 |
| AviSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:49 | 50 | 52 | default | 57 |
| PdfSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:8 | 10 | 9 | default | 12 |
| En1992Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:67 | 69 | 70 | default | 94 |
| SemioModelSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:103 | 107 | 105 | 104 | 108 |
| StlSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:72 | 83 | 73 | default | 74 |
| GltfSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:57 | 58 | 59 | 60 | 62 |
| Mp4Snapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:47 | 48 | 50 | default | 51 |
| Process3dSnapshot | ./✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🛂️capability/🦀️.rs:6 | 10 | 11 | 13 | 12 |
| BcfSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:45 | 46 | 50 | default | 55 |
| SemioDocumentSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:154 | 159 | 157 | 155 | 160 |
| Puzzle2dSnapshot | ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:153 | 161 | 155 | 158 | 164 |
| crate::Puzzle2dPlaySnapshot | ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:170 | 177 | 186 | default | 185 |
| SemioPresentationSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:74 | 78 | 77 | 75 | 79 |
| SemioAudioSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:24 | 27 | 26 | 25 | 28 |
| En1991Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:79 | 80 | 83 | default | 89 |
| FlowSnapshot | ./✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:10 | 11 | 12 | default | 16 |
| DagSnapshot | ./✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | 62 | 77 | 82 | default |
| DocxSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:55 | 60 | 56 | 82 | 64 |
| SemioObjectSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | 37 | 36 | 35 | 38 |
| SemioFlowSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:44 | 47 | 46 | 45 | 48 |
| PlySnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:106 | 108 | 107 | 109 | 111 |
| SemioSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:31 | 35 | 34 | 32 | 36 |
| SemioMeshSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:53 | 57 | 55 | 54 | 58 |
| SemioAnimationSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:45 | 49 | 47 | 46 | 50 |
| En1998Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:56 | 57 | 59 | default | 65 |
| SemioImageSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | 33 | 32 | 31 | 35 |
| SemioDrawingSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:42 | 46 | 45 | 43 | 47 |
| SemioTableSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:32 | 37 | 35 | 33 | 38 |
| PngSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:6 | 10 | 11 | 13 | 12 |
| BinarySnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:7 | 11 | 12 | 14 | 13 |
| JsonSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | 49 | 35 | 36 | 37 |
| BinarySnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:7 | default | default | default | default |
| XlsxSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:66 | 83 | 79 | 93 | 87 |
| SemioBrepSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:109 | 113 | 112 | 110 | 114 |
| Puzzle5dSnapshot | ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:48 | 50 | 51 | default | default |
| crate::Puzzle5dPlaySnapshot | ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:151 | 155 | 156 | default | default |
| JpgSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:39 | 44 | 40 | default | 45 |
| TxtSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | 85 | 120 | default | 142 |
| SemioCadSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:92 | 96 | 94 | 93 | 97 |
| PdfSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📄️document/🦀️.rs:17 | 19 | 18 | 20 | 22 |
| Din4108Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:40 | 41 | 43 | default | 49 |
| DwgSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:59 | 61 | 60 | default | 62 |
| SemioTextSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | 38 | 37 | 36 | 39 |
| SemioKitSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:108 | 111 | 110 | 109 | 112 |
| TiffSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | 34 | 35 | default | 36 |
| MdSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:121 | 169 | 170 | 171 | 123 |
| SvgSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:16 | 17 | 18 | 19 | 20 |
| SemioGraphSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:52 | 57 | 55 | 53 | 58 |
| En1999Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:51 | 53 | 55 | default | 83 |
| XmlSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:134 | 135 | 137 | 136 | 138 |
| Puzzle3dSnapshot | ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:409 | 411 | 426 | default | default |
| crate::Puzzle3dPlaySnapshot | ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:883 | 891 | 894 | default | default |
| SemioVideoSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:23 | 26 | 25 | 24 | 27 |
| PptxSnapshot | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:63 | 68 | 64 | 90 | 72 |
| LayoutSnapshot | ./✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:86 | 91 | 95 | 90 | default |
| En1995Snapshot | ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:47 | 48 | 50 | default | 56 |
| ShootingSnapshot | ./✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:32 | 37 | 34 | default | 45 |
| EquationSnapshot | ./✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:25 | 26 | 30 | 32 | 33 |
| crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot | ./✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:165 | 169 | 170 | default | default |

## Manual Test Declarations

| Owner | File:line | Decode | Encode | Retirement | Preflight |
|---|---|---|---|---|---|
| DemoSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs:192 | default | default | default | default |
| DemoSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:1959 | default | default | default | 1961 |
| TransferSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🪶️transfer/🦀️.rs:50 | 52 | 61 | default | 60 |
| RetainedSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement/🧪️tests/🦀️.rs:29 | 35 | 31 | 47 | 50 |
| DecodedBuffers | ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs:137 | 139 | default | default | default |
| $owner | ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-encoding/🧪️tests/🦀️.rs:43 | default | default | default | default |
| NoConfig | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔬️schema-stamping/🦀️.rs:183 | default | default | default | default |
| Snapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:41 | 55 | 75 | default | 96 |
| Snapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:33 | 50 | 79 | default | 104 |
| ComposedParentSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:17 | default | default | default | default |
| RecursiveBranchSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:584 | default | default | default | default |
| TestSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs:27 | default | default | default | default |
| $snapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:72 | default | default | default | 74 |
| SurfaceSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs:137 | default | default | default | default |
| DummySnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs:29 | default | default | default | default |
| TxnSnapshot | ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs:31 | default | default | default | default |
| fn sqlite_snapshot_docx_deep_erased_native_input_output_are_interior_cancellable(){let plan:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🛫️native/🔣️.json")).unwrap();std::thread::Builder::new().stack_size(plan["smallStackBytes"].as_u64().unwrap()as usize).spawn(move||{struct Owner(Option<DocxSnapshot>);impl Drop for Owner{fn drop(&mut self){if let Some(snapshot)=self.0.take(){snapshot.retire_sqlite_snapshot();}}}let mut snapshot=fixture();snapshot.xml_parts[0].document=deep_owned_document(usize::try_from(plan["depth"].as_u64().unwrap()).unwrap());snapshot.xml_parts[1].content_type="z".repeat(plan["lateTextBytes"].as_u64().unwrap()as usize);let snapshot=Owner(Some(snapshot));let limits=SqliteDatabaseLimits::default();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.docx".into(),standard:"ecma-376".into(),subset:"*".into()};let codec=<DocxSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let db=snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let expected_database=db.clone();let payload=(codec.import)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,db,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;let projected=(codec.export)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;assert!(projected==expected_database,"complete deep native graph differs");assert_eq!(projected.table("docx_xml_element").unwrap().rows.iter().filter(|row|row.text(1).unwrap()=="node").count(),plan["depth"].as_u64().unwrap()as usize);assert_eq!(projected.table("docx_xml_text").unwrap().rows.iter().filter(|row|row.text(1).unwrap()=="leaf").count(),1);for phase in[SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{let mut reached=false;let mut callback=|event:SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=plan["cancelAfter"].as_u64().unwrap()as usize&&event.completed<event.total{reached=true;false}else{true}};let failed=if phase==SqliteSnapshotPhase::EncodeNative{let db=snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();(codec.import)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,db,encoding,&mut SqliteSnapshotControl::new(&mut callback,limits)).is_err()}else{(codec.export)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut callback,limits)).is_err()};assert!(failed);assert!(reached,"actual interior native phase {phase:?}");}}}).unwrap().join().unwrap();} | ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:102 | default | default | default | default |

## Qualification Frontiers and Publication Evidence

- MCP ProbeSnapshot genuine production document registration is `🌉️mcp/🏠️workspace/🦀️.rs:402` ArtifactCodec::of. Its actual eight-table SQL projection implementation exists at `🌉️mcp/🏠️workspace/🪶️sqlite/🦀️.rs:5`, with serde_json::Value reconstruction/projection ownership. Consult row for native overrides; document registration is not proof of productive native owner.
- SpaceSnapshot and CollectionSnapshot publish actual bare native registrations through their binary snapshot ArtifactPack hooks (Space6–11, Collection corresponding hook), plus SQL registration functions in each SQL snapshot module12. Their original registration/dialect is genuine; constructor callback migration still leaves generated partial fields unqualified.
- Workflow and RunArtifact original binary snapshot hooks register actual types and dialects; generic Plugin production `🔌️plugin/🦀️.rs:3612,3626,3646,43082` constructs bare codecs from real declared Snapshot/Mutation and schema. This route is declaration-driven, so schema-to-owner inference or a static factory catalog cannot replace installed registry enumeration.
- NativeSocketProbeSnapshot is an explicitly authored production internal probe at renderer SQLite29, with real run publication hooks. It should not be conflated with a domain artifact or used as a fabricated factory denominator. Root owns its original-control laws.
- Guest codecs are host-constructed erased ArtifactCodec callbacks, not native Rust ArtifactSqliteSnapshot implementations; WIT/SDK/Host capability and persistent denied cleanup must remain separately qualified.
- Current original three-part decode receiving callback covers completed spec/record plus registered partial T. Generated __dsl_from_record_controlled and native pack/record parser local partial producers remain a separate major unqualified frontier despite all callback arities being migrated.
- GUI `.vscode/launch.json` contains original projectTarget.test-snapshot-sqlite (100649), native100757, source100865, verify-source101025 input routes. These entries select projects; they do not establish that every owner has an executable neutral/native law or has run. Actual project.json/📜️script.ts target association must be verified per selected owner before execution.

No scope count here is promoted to the full artifact denominator. Existing actual-binding/catalog ledgers and package publication census remain the authority for completeness; this report supplies explicit trait source evidence only.

## Exact Native Signature Evidence

The following source signatures distinguish supplied snapshot owner from raw native control. These deduplicated per-file method signatures include unmounted alternatives and are not an installed census.

| Source | Method | Parameter authority |
|---|---|---|
| ./🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:15 | decode_sqlite_snapshot_native | decode-owner |
| ./🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:16 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:14 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:20 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:37 | decode_sqlite_snapshot_native | RAW native control |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | decode_sqlite_snapshot_native | decode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:38 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:143 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:149 | encode_sqlite_snapshot_native | encode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:27 | decode_sqlite_snapshot_native | decode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:153 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:161 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:12 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️owner/🦀️.rs:233 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️owner/🦀️.rs:243 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:246 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:258 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:15 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:18 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:32 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:12 | encode_sqlite_snapshot_native | encode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🪶️sqlite/🦀️.rs:39 | decode_sqlite_snapshot_native | decode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🪶️sqlite/🦀️.rs:45 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:135 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:137 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:42 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:428 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:446 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:296 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:297 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:44 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:46 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:64 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:68 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:48 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:50 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:7 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:8 | encode_sqlite_snapshot_native | encode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:19 | decode_sqlite_snapshot_native | decode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:20 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:37 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:104 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:107 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:92 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:93 | encode_sqlite_snapshot_native | encode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:97 | encode_sqlite_snapshot_native | encode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:101 | decode_sqlite_snapshot_native | decode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:68 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:69 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🛂️capability/🦀️.rs:10 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🛂️capability/🦀️.rs:11 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:69 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:70 | encode_sqlite_snapshot_native | encode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:27 | encode_sqlite_snapshot_native | encode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:31 | decode_sqlite_snapshot_native | decode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:26 | encode_sqlite_snapshot_native | encode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | decode_sqlite_snapshot_native | decode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:58 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:59 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:9 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:10 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:143 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:145 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:49 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:51 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:12 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:50 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:52 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:50 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:51 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:80 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:83 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:7 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:8 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:73 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:83 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:108 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:110 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:67 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:69 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:58 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:59 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:48 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:50 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:105 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:107 | decode_sqlite_snapshot_native | RAW native control |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛂️capability/🦀️.rs:9 | decode_sqlite_snapshot_native | decode-owner |
| ./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛂️capability/🦀️.rs:10 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:12 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:13 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:157 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:159 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:46 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:50 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:27 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:62 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:77 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:26 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:27 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:53 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:55 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:15 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:16 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:57 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:59 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:40 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:47 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:49 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:67 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:69 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:316 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:331 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:45 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:46 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:31 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:25 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:26 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:381 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:396 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:17 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:20 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:32 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:33 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:40 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:44 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:9 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:10 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:77 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:78 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:56 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:60 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:36 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:37 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:365 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:380 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:55 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:57 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:18 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:19 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:17 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:18 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:107 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:108 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:12 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:112 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:113 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:41 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:43 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:61 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:63 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:37 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:110 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:111 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:10 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:14 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:79 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:83 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:10 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:94 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:96 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:55 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:57 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:16 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:17 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:37 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:38 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:53 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:55 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:46 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:47 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:202 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:203 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:25 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:27 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:36 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📄️document/🦀️.rs:18 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📄️document/🦀️.rs:19 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:49 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:60 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:61 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:73 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:74 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:31 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:32 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:10 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:85 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:120 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:49 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:50 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:12 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:116 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:120 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:91 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:92 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:35 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:169 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:170 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:155 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:161 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:177 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:186 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:37 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:38 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:62 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:63 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:29 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:32 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:91 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:95 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:411 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:426 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:891 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:894 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:50 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:51 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:155 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:156 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:26 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:113 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:118 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:413 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:414 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:24 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:25 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:34 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:37 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:169 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:170 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:36 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:37 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:21 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:22 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:27 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:28 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:31 | encode_sqlite_snapshot_native | encode-owner |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:30 | decode_sqlite_snapshot_native | RAW native control |
| ./✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:31 | encode_sqlite_snapshot_native | encode-owner |


## Explicit Unmounted Alternatives

- ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:8
- ./✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️owner/🦀️.rs:226
- ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:8
- ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:7
- ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:13
- ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/🦀️.rs:7

## Concrete Missing Owners

MCP ProbeSnapshot uses all default native hooks despite genuine SQL projection and actual document registration. Raw NativeDecodeControl signatures listed above are not the current public trait contract and need original NativeSnapshotDecodeOwner forwarding; callback arity migration alone does not repair these signatures. Unmounted native/copy alternatives are source proposals, not productive installed owners. Generic erased OwnedSqliteSnapshot Drop is excluded from trait-implementation counts; its default-domain cleanup frontier remains separate. No controlled-retirement support is inferred from a similarly named method or derive without checking actual declared fields.
