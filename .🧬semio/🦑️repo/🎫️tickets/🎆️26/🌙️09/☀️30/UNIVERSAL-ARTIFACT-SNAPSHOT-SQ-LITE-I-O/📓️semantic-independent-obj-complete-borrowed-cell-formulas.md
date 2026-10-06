# OBJ Complete Borrowed Cell Formulas

Read-only authority: [single authored SQL visitor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:57), [native structs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/🦀️.rs), [native Pack descriptors](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/📦️pack/🦀️.rs). Existing preflight at SQL line56 is a physical expansion forecast; it does not enforce all five copied SQL limits. Preserve its constants through file_only separately from complete semantic admission.

Canonical schema extent6231, tables14, maximum columns15. Independent original full34 rows/1830 bytes, metadata-preserving empty2/60, original Source empty2/41 remain literal authorities in [independent input](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/stl-obj-original-carriers-independent-null-zero-demand-blueprint.json). NULL costs0, integers8. Define F(x)=8 bits+UTF8 numeric-class+8 query value unless NaN; NaN query NULL costs0. Thus finite22, infinity32, NaN11. Optional absent coordinate costs0 for all three SQL companion cells.

| Table | Width | Complete per-row semantic bytes, including identity |
|---|---:|---|
| obj_document |3|8+schema UTF8+present mtllib UTF8|
| obj_vertex |15|24+F(x)+F(y)+F(z)+present F(w)|
| obj_texcoord |12|24+F(u)+F(v)+present F(w)|
| obj_normal |12|24+F(x)+F(y)+F(z)|
| obj_face |3|24|
| obj_face_vertex |9|24+8 literal vertex+8 each present literal texcoord/normal+8 each resolved vertex/texcoord/normal|
| obj_group / obj_object |4|24+name UTF8|
| obj_group_face / obj_object_face |6|40+8 if resolved face exists|
| obj_face_boundary |4|24+8 except terminal boundary|
| obj_material_range |7|40+8 if resolved boundary exists+material UTF8|
| obj_smoothing_range |7|40+8 if resolved boundary exists+8 if group present|
| obj_unknown_statement |6|40+raw UTF8|

Rows comprise document1; all authored vertices/texcoords/normals/faces; every face vertex; every group/object and membership; faces.len()+1 boundaries; every material/smoothing range and unknown statement. Split u64 positions/indices into two SQL INTEGER cells, each8, rather than decimal text. Resolved links exist exactly when literal index<count (material/smoothing boundary count faces.len()+1). Literal unresolved words are supported: preserve nullable links and do not reject them merely for being out of range. Source is zero based; stored identity is ordinal+1.

Required native root11 IDs: 0 schema Text; 1 vertices List<Record>; 2 texcoords List<Record>; 3 normals List<Record>; 4 faces List<Record>; 5 groups List<Record>; 6 objects List<Record>; 7 mtllib Optional Text; 8 usemtl List<Record>; 9 smoothing_groups List<Record>; 10 unknown_statements List<Record>.

Nested exact fields: Vertex4 x0/y1/z2 Float,w3 Optional Float; TexCoord3 u0/v1 Float,w2 Optional Float; Normal3 x0/y1/z2 Float; Face1 vertices0 List<Record>; FaceVertex3 vertex0 UInt u32,texcoord1/normal2 Optional UInt u32; Group/Object2 name0 Text,faces1 List<UInt u64>; Material2 face_index_from0 UInt u64/material1 Text; Smoothing2 face_index_from0 UInt u64/group1 Optional UInt u32; Unknown2 line_index0 UInt u64/raw1 Text. There are no enum or DslValue domain wrappers. Controlled unsigned fields accept UInt with checked width; Float accepts FieldValue::Float, not Int/UInt. Optional roles require explicit Absent or their actual primitive. Missing fields do not become defaults in controlled DslRecord binding; require every declared ID and exact record count. Present-empty Text differs from Absent even when byte cost coincides.

Pretyped visitor should borrow these actual Record/List/Text cells, accumulate the same table rows and cell formulas, and validate schema/table/column ceilings before typed construction. Use bounded UTF8 checkpoints (at most256 borrowed bytes), checked additions, and a scoped native traversal workload restored before binder; retain cumulative ownership control. Do not construct Snapshot, private SQL database, native-record mirror, or a fresh private control. Typed encode/preflight should reuse existing write_sqlite_rows with semantic RowWriter, then retain separate original physical forecast and actual encode allocation/file enforcement.

## Actual Cohort Receipt

[Retained log](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/🗑️generated/root-zip-png-after-tiff-stl-obj-before-native.log) is terminal Nx failure for five projects. ZIP After15/15,69 skipped at15984; PNG After8/8,55 skipped at64089. TIFF Before4/2/2,117 skipped at32057; OBJ Before17/15/2,44 skipped at47853; STL Before13/11/2,44 skipped at79815. Each red owner has independent native semantic-limit positive failing value-bytes OwnershipLimit and copied-columns preflight assertion failing Binary. OBJ exact locations test228:173 and235:717. These are genuine runtime receipts, not compiler-only; no whole-owner or fleet qualification inferred.
