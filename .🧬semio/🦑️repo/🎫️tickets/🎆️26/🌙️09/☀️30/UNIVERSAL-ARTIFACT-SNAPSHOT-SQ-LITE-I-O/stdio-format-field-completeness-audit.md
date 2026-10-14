# Stdio Format Field Completeness Audit

Read-only source audit; no builds/runtime qualifications. Bases below are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`. Schema source is each base/🧬️schema/📸️snapshot/🦀️.rs; implementation base/🚪️io/🪶️sqlite/📸️snapshot. Complete nested legal-value qualification remains separate from top-level coverage.

| Reviewed base | Actual declared snapshot fields | Source traversal finding |
|---|---|---|
| 🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any | schema,document,buffers,source_form | 📄️document writes/reads source_form json/glb, schema and GltfDocument; buffers owned separately; mounted split helpers/DDL include57 semantic tables; no top-level omission found |
| 🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry | schema,vertices,texcoords,normals,faces,groups,objects,mtllib,usemtl,smoothing_groups,unknown_statements | Explicit write/read traversals include source index high/low words, smoothing optional group and unknown raw statements; no omitted11-field collection found |
| 🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any | schema,solid_name,triangles | Root semantic implementation models document identity/name and triangles; nested exact scalar/normal/vertex variant proof remains required |
| 🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base | schema,doc | Delegates actual XML document projection/reconstruction through named XmlSqliteTables mapping13tables; root/attribute policy checks apply; doc is not JSON blob |
| 📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base | schema,pages | Small root semantic projection exists; page content branches not fully reviewed here |
| 📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base |32 fields | Actual 📄️document constructor includes all32, including shorthand document_id copied from two Blob cells and admitted_stream_roles through pdf_document_role helper; no omitted top-level field |
| 📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any | schema,has_header,records | Explicit receiving Projection/Reconstruction retain logical records/fields and quoted flag; prior original binder audit covers typed seam |
| 📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any | schema,records,trailing_newline,line_ending | Actual document schema includes flag/ending; explicit binder/receiving frames preserve all4 rather than inferring them from raw body |
| 🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base | schema,value | Relational value tree with typed null/bool/number lexeme/string/array/object child tables, not opaque serialized JSON snapshot |

## Real provider expansion

glTF ArtifactPack is in actual binary snapshot/📦️pack/🦀️.rs:101, with explicit sqlite_snapshot_codec at102; searching only binary snapshot root would falsely claim absence. Norm common compliance macro @binary emits provider when actual sqlite= argument present; earlier roster now distinguishes verified macro providers from unresolved entries. This pass does not assert every Norm nested value variant is complete merely because provider macro exists.

PDF1.7 restores objects/trailer/catalog_extra using semantic COS tables, document metadata/ID/encryption/info and role admission tables. Opaque-looking intrinsic stream bytes are real document stream owners; relational extensions/extras IDs in glTF point to a structured JSON tree. Neither is evidence of an opaque whole-snapshot carrier. Literal raw_text/script or unknown statement content must remain literal source authority, not be split because its String contains braces.

## Concrete Scope Hole and Remaining Work

No new exact top-level semantic omission comparable to BIM16 was proven in these reviewed formats. The actual whole-snapshot versus raw-native payload scope hole applies beyond TXT: schema/metadata/lexical source distinctions must travel through the new direct typed file API rather than assume native source file encoding is injective. Existing typed P/R availability does not prove original physical parser/writer prefix custody or Native runtime passes.

Unreviewed: all57 glTF nested fields/tag variants; OBJ every nested face/source-index legality; STL exact scalar classes; SVG/XML attribute/entity/doctype legal cases; PDF1.4 page branches and PDF1.7 all135-table tagged navigation/content/COS/resource alternatives; JSON exact source-number/map duplicate policy; all Norm nested field/macro codecs and alias subsets. YAML was not found among the36 selected stdio contribution calls and no YAML artifact snapshot owner was found by this path scan; do not call it a missing installed provider without an actual selected declaration.

Prioritize schema-first independent fixtures with nondefault/optional/empty/UTF8/NUL/source-form and scalar bit variants per legal field, then actual new direct supplied-owner typed file export/import and cancellation/cleanup laws. Never equate broad test labels/table counts or this source audit with runtime qualification.
