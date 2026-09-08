# Stdio Owned Representation Audit

Pass 356 groups current large-enum and by-value diagnostics before selecting representation changes. No representation edit is applied by this audit.

{
  "clippy::needless_pass_by_value:🔺️diff": 55,
  "clippy::needless_pass_by_value:🧬️mutations": 258,
  "clippy::needless_pass_by_value:📸️snapshot": 1,
  "clippy::large_enum_variant:🧬️mutations": 123,
  "clippy::needless_pass_by_value:💡️inferences": 1,
  "clippy::large_enum_variant:📸️snapshot": 1,
  "clippy::needless_pass_by_value:other": 11,
  "clippy::needless_pass_by_value:⚙️engine": 2
}

```text
error: large size difference between variants
  --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/././././././././../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:34:1
   |
34 | / pub enum Ifc2x3Mutation {
35 | |     SetSnapshot(set_snapshot::SetSnapshot),
   | |     -------------------------------------- the largest variant contains at least 252 bytes
36 | |     UpsertInstance(upsert_instance::UpsertInstance),
37 | |     RemoveInstance(remove_instance::RemoveInstance),
38 | |     SetHeader(set_header::SetHeader),
   | |     -------------------------------- the second-largest variant contains at least 36 bytes
39 | | }
   | |_^ the entire enum is at least 256 bytes
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
   = note: `-D clippy::large-enum-variant` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
   |
35 -     SetSnapshot(set_snapshot::SetSnapshot),
35 +     SetSnapshot(Box<set_snapshot::SetSnapshot>),
   |

```
```text
error: large size difference between variants
  --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/././././././../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🧬️schema/🧬️mutations/🦀️.rs:77:1
   |
77 | / pub enum Ifc2x3Cv20Mutation {
78 | |     SetSnapshot(set_snapshot::SetSnapshot),
   | |     -------------------------------------- the largest variant contains at least 252 bytes
79 | |     SetViewDefinition(set_view_definition::SetViewDefinition),
80 | |     SetStructuralEntity(set_structural_entity::SetStructuralEntity),
   | |     --------------------------------------------------------------- the second-largest variant contains at least 48 bytes
81 | |     SetProjectUnits(set_project_units::SetProjectUnits),
82 | |     SetProductPlacement(set_product_placement::SetProductPlacement),
83 | | }
   | |_^ the entire enum is at least 256 bytes
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
   |
78 -     SetSnapshot(set_snapshot::SetSnapshot),
78 +     SetSnapshot(Box<set_snapshot::SetSnapshot>),
   |

```
```text
error: large size difference between variants
  --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/././././././././../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🏠️default-scene/🔗️bind/🦀️.rs:20:1
   |
20 | / pub enum BindDefaultSceneMutation {
21 | |     Apply(GltfBindDefaultScenePayload),
   | |     ---------------------------------- the second-largest variant contains at least 4 bytes
22 | |     Restore(crate::artifacts::gltf::schema::diff::GltfDiff),
   | |     ------------------------------------------------------- the largest variant contains at least 656 bytes
23 | | }
   | |_^ the entire enum is at least 656 bytes
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
   |
22 -     Restore(crate::artifacts::gltf::schema::diff::GltfDiff),
22 +     Restore(Box<crate::artifacts::gltf::schema::diff::GltfDiff>),
   |

```
```text
error: large size difference between variants
  --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/././././././././../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/🔗️bind/🦀️.rs:19:1
   |
19 | / pub enum BindMorphTargetAttributeMutation {
20 | |     Apply(GltfBindMorphTargetAttributePayload),
   | |     ------------------------------------------ the second-largest variant contains at least 28 bytes
21 | |     Restore(crate::artifacts::gltf::schema::diff::GltfDiff),
   | |     ------------------------------------------------------- the largest variant contains at least 656 bytes
22 | | }
   | |_^ the entire enum is at least 656 bytes
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
   |
21 -     Restore(crate::artifacts::gltf::schema::diff::GltfDiff),
21 +     Restore(Box<crate::artifacts::gltf::schema::diff::GltfDiff>),
   |

```
```text
error: large size difference between variants
  --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/././././././././../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📷️node-camera/🔗️bind/🦀️.rs:19:1
   |
19 | / pub enum BindNodeCameraMutation {
20 | |     Apply(GltfBindNodeCameraPayload),
   | |     -------------------------------- the second-largest variant contains at least 8 bytes
21 | |     Restore(crate::artifacts::gltf::schema::diff::GltfDiff),
   | |     ------------------------------------------------------- the largest variant contains at least 656 bytes
22 | | }
   | |_^ the entire enum is at least 656 bytes
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
   |
21 -     Restore(crate::artifacts::gltf::schema::diff::GltfDiff),
21 +     Restore(Box<crate::artifacts::gltf::schema::diff::GltfDiff>),
   |

```

## Pass 359 — Repeated Rejection Conversion

120 private rejection_outcome helpers form 1 exact whitespace-normalized bodies. Centralizing an identical owned error conversion can remove needless String cloning/parameter ownership while preserving each mutation’s error classification.


## Pass 360 — Shared Rejection Conversion

Replaced all 120 byte-equivalent private rejection helpers with one crate-private conversion in top-level mutation support. The converter borrows code/path and consumes the detail String. Every caller preserves its exact error fields, classification order, target splitting and output messages. This addresses 240 needless-pass-by-value diagnostics without changing mutation schemas, payload layouts or failure semantics.

Fresh Stdio compiler and mutation runtime laws are pending.


## Pass 363 — Mutation Carrier Contract Before Representation Edit

Added committed language-neutral JSON for forward/restore carriers, an inline size budget, and six rejection classifications. Tests decode/encode with the first-party codec and compare against the serde_json oracle, execute the forward mutation and its restored inverse, and assert exact rejection messages. These tests are written before changing Restore representation. Execution is pending.


## Pass 364 — Compact Owned glTF Restore Variants

119 diagnosed mutation enums now store their large owned GltfDiff restore payload in Box. Their Apply payloads stay inline. Every inverse constructor, restore apply and cloned outcome was updated explicitly. The compact node-name restore already uses its own three-field schema and is unchanged. A repo-wide scan found no external explicit constructors for these 119 Restore variants. The first-party Box ToValue/FromValue implementations transparently delegate to the payload. Existing ownership remains exclusive and clones remain deep; each restore adds one allocation while common apply entries no longer reserve 656 WASI bytes for an unused restore field. No serialized fields or mutation descriptors changed.

The pass363 JSON/forward/inverse laws and fresh native/WASI compiler checks are required before claiming parity.


## Pass 371 — Remaining Owned Parameters

Current-source matching leaves 83 distinct diagnosed functions after shared rejection conversion.

{
  "absorb_array_diff": 2,
  "absorb_attrs_diff": 3,
  "absorb_children_diff": 3,
  "diff_pack_err": 5,
  "op_pack_err": 17,
  "absorb_blocks_diff": 1,
  "absorb_list_items_diff": 1,
  "step_snapshot_from_part21": 1,
  "absorb_point_diff": 1,
  "diff_set_point": 1,
  "gltf_bin_err": 1,
  "empty_indicators": 1,
  "absorb_triangle_diff": 1,
  "finish_r2010_object_frame": 1,
  "absorb_weak_index_triple": 1,
  "contribute": 1,
  "absorb_pages_diff": 1,
  "absorb_law_pages_diff": 1,
  "absorb": 1,
  "absorb_indexed": 8,
  "absorb_named": 11,
  "diff_set_shape_text": 1,
  "step_err": 1,
  "map_err": 1,
  "map_step": 1,
  "absorb_vertex_diff": 1,
  "absorb_canvas_diff": 1,
  "absorb_style_diff": 1,
  "absorb_material_diff": 1,
  "walk": 1,
  "absorb_frame": 1,
  "diff_set_slide_notes": 1,
  "diff_set_textbox_blocks": 1,
  "node_view": 8
}


## Pass 373 — Borrowed Pack Error Formatting

22 private binary error-formatting helpers now borrow PackError; 90 map_err call sites pass a reference while owning the error inside their closure. Error categories, offsets and rendered details are unchanged. Every executable reference to each helper was accounted for before editing. Fresh compiler verification remains required.


## Pass 374 — Borrowed Tree Paths

Eight private JSON/XML editor and viewer tree builders now borrow a path slice. Their root calls use an empty slice, recursive calls borrow the existing child path, and child-path expansion retains the same owned elements through to_vec. Node IDs, labels, ordering and view structure are unchanged. Current-source function bodies and every local call were validated before editing. Compiler/runtime validation remains pending.


## Pass 375 — Collection Absorb Borrow Review

Proposed 29 private absorb helpers whose diagnosed immutable parameters and every direct local call were identified. No ownership edit is applied by this audit. Skipped: [{"name":"absorb_point_diff","reason":"noncall/generic reference"},{"name":"absorb_named","reason":"noncall/generic reference"},{"name":"absorb_named","reason":"noncall/generic reference"},{"name":"absorb_vertex_diff","reason":"noncall/generic reference"},{"name":"absorb_style_diff","reason":"noncall/generic reference"},{"name":"absorb_indexed","reason":"noncall/generic reference"},{"name":"absorb_named","reason":"noncall/generic reference"},{"name":"absorb_material_diff","reason":"noncall/generic reference"},{"name":"absorb_indexed","reason":"noncall/generic reference"},{"name":"absorb_frame","reason":"noncall/generic reference"}]


## Pass 376 — Borrowed Collection Absorb Inputs

{"functions":28,"parameters":30,"calls":68,"clones":2,"files":21}

Private collection absorb helpers now borrow compiler-proven unconsumed immutable inputs; all direct local calls were updated. Two callers no longer clone a diff solely to read it. Parameters carrying owned first-stage state keep their ownership. Generic function-pointer consumers and PNG’s vector-triple API are deferred for separate review. Fresh compiler and collection-law validation remain required.


## Pass 383 — Named Collection Parts and Encoder Contracts

Introduced domain-local names for removed/modified/added collection parts, optional mutation payload encoders, and artifact assembly factories. These aliases preserve tuple order, representation, and callable contracts. Updated 40 references across 17 files. No new runtime implementation or dependency. Source-matched pre-write checks passed; fresh compilation pending.


## Pass 384 — Domain Names for Codec and Geometry Results

Named existing tuple results and intermediate collections according to their codec or geometry role in 17 files. Layout, ownership, tuple destructuring, and output fields remain identical. Type aliases are local to their owning modules and introduce no external dependencies. Compiler verification remains pending.


## Pass 390 — One Flow Decode Admission Entry Point

Moved the identical Flow decode admission body into its existing MemberSnapshotOpenOperation::begin implementation and removed the redundant inherent constructor. Six existing identity/progress/cancellation law call sites now use that canonical trait entry point. No error carrier allocation, admission semantics, request ownership, or retirement behavior changed. Also removed the compiler-reported redundant std::mem qualification from the glTF carrier size law (pass 388). Fresh compiler and Flow runtime laws pending.


## Pass 392 — Remaining Error Formatter Borrows

Four compiler-diagnosed glTF, STEP, and BREP error helpers now borrow the value they format. All direct STEP formatting calls and map_err users were updated; details and error categories remain identical. Fresh compiler verification pending.


## Pass 393 — Borrowed Snapshot and Content Diff Inputs

Eight STEP, LAS, glTF, DWG, PNG, PPTX, and presentation helpers now borrow immutable document, byte writer, and content inputs. Five mutation dispatchers no longer clone content merely to compare it. The DWG handle writer remains owned and mutable; its data writer is borrowed. All callers were enumerated with repository source search and exact edits were guarded. Fresh compilation and artifact oracle laws pending.


## Pass 409 — Remaining Function Inventory

Captured complete current function bodies and local references for the remaining ownership, slice, infallibility, and argument-count diagnostics. Callback signatures and external constructors require caller audits before changing them.


## Pass 410 — Consumed Diff Additions and Borrowed Scalar Helpers

Five generic diff absorbers now consume their owned second-stage additions, removing payload clones while preserving transport and order. Scalar diff callbacks borrow immutable inputs at their existing owned callback boundary. PNG second-stage removal/modification slices and XML/SVG prolog slices avoid unnecessary container ownership. IFC model traversal passes a borrowed optional ancestor id, creating strings only for emitted model rows. All local references and full bodies were inspected; guarded edits applied, compilation pending.


## Pass 411 — Borrowed STEP Documents and JPEG Diff Inputs

The STEP snapshot constructor takes an immutable Part-21 document reference; repository source search enumerated and updated every qualified constructor call, including dialect tests. JPEG component and quantization diffs borrow their merge input, removing two caller clones. Function bodies and caller ownership were checked before guarded edits. Fresh compiler and codec-law results pending.


## Pass 412 — Infallible Codec and Geometry Leaves

Fourteen helpers that cannot produce errors or absence now return their values directly. Fallible generic mutation callbacks and public surface-intersection dispatch retain their result contracts at the caller boundary; actual parsing, indexing, and topology failures remain fallible. PDF name/Unicode handling and HTML text extraction retain their original byte traversal. Every affected helper body and call site was inspected; source guards passed, fresh compilation pending.


## Pass 414 — Related Geometry Parameter Groups

Grouped existing related parameters: indexed diff triples, mesh/primitive identity, traversal output, SVG radii and gradient focus, grid spacing, fillet support faces/surfaces/domain, boolean bounds and triangle output, 2D line coordinates, Bezier segments, Newton seeds, cylinder supports, topology callbacks, neutral plane, orbit endpoints, and helix axis. The same scalar/reference values reach the original calculations in the original order. Borrowed diff and traversal groups have local named type aliases. All 24 definitions and their repository callers are guarded; behavior and strict compilation remain to be verified.


## Pass 417 — Remaining Large Carrier Audit

Enumerated IFC2x3 base/CV2.0 and DWG snapshot mutation fields and EPW record mutation fields, keeping match patterns distinct from construction. Aggregate mutation derives require direct leaf contracts; storage indirection must remain inside the affected leaf payload or ordinary object-body variant. Existing artifact oracle fixtures remain the wire authority.


## Pass 418 — Large Mutation and DWG Object Carriers

Stored IFC2x3 base/CV2.0 and DWG full-snapshot leaf payloads, EPW inserted records, and DWG table-style object bodies behind owned boxes. Mutation leaf identities and first-party value/DSL schemas stay at their existing tags and fields; explicit DWG table-style tag 23 and payload field 24 are unchanged. Repository-wide constructors, dialect oracle tests, and borrowed match patterns were audited separately. Existing exact IFC, DWG, and EPW mutation/fixture laws are selected for the next native executable verification. Strict WASI run 416 began before this storage edit, so it cannot verify its final state.
