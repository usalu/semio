# WASI 1044 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":229,"warning":0},"cargoWarnings":["warning: build failed, waiting for other jobs to finish..."],"startedAt":"2026-09-09T11:31:03.127Z","finishedAt":"2026-09-09T11:50:12.772Z","packages":160}

## E0425 — cannot find function `puzzle3d_window_transient_retained_bytes` in this scope

error[E0425]: cannot find function `puzzle3d_window_transient_retained_bytes` in this scope
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:142:26
    |
142 |     let retained_bytes = puzzle3d_window_transient_retained_bytes(transient).ok_or_else(|| "Puzzle 3D window transient footprint ov...
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope


## clippy::vec_init_then_push — calls to `push` immediately after creation

error: calls to `push` immediately after creation
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:113:9
    |
113 | /         let mut mutations = Vec::with_capacity(20);
114 | |         mutations.push(En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex { new_annex: snapshot.annex }));
115 | |         mutations.push(En1995Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: snapshot.m_ed_knm }));
116 | |         mutations.push(En1995Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: snapshot.n_ed_kn }));
...   |
132 | |         mutations.push(En1995Mutation::ChangeAVertMS2(change_a_vert_m_s2::ChangeAVertMS2 { new_a_vert_m_s2: snapshot.a_vert_m_s2 }));
133 | |         mutations.push(En1995Mutation::ChangeNCyclesBridge(change_n_cycles_bridge::ChangeNCyclesBridge { new_n_cycles_bridge: snapshot.n_cycles_bridge }));
    | |___________________________________________________________________________________________________________________________________________________________^ help: consider using the `vec![]` macro: `let mutations = vec![..];`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push
    = note: `-D clippy::vec-init-then-push` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::vec_init_then_push)]`


## unused_imports — unused import: `JackEditorWindowConfigMutation`

error: unused import: `JackEditorWindowConfigMutation`
  --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:10:48
   |
10 | use crate::editor::jack::query_window_config::{JackEditorWindowConfigMutation, JackEditorWindowConfigOwner};
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_imports)]`


## unused_qualifications — unnecessary qualification

error: unnecessary qualification
  --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/../🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/🦀️.rs:92:78
   |
92 |     Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: std::mem::size_of::<JackEditorWindowTransient>() })
   |                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-qualifications` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_qualifications)]`
help: remove the unnecessary path segments
   |
92 -     Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: std::mem::size_of::<JackEditorWindowTransient>() })
92 +     Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: size_of::<JackEditorWindowTransient>() })
   |


## clippy::large_enum_variant — large size difference between variants

error: large size difference between variants
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs:187:1
    |
187 | / pub enum QueryPreparationStep {
188 | |     Pending,
    | |     ------- the second-largest variant carries no data at all
189 | |     Complete(QueryExecution),
    | |     ------------------------ the largest variant contains at least 568 bytes
190 | | }
    | |_^ the entire enum is at least 568 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    = note: `-D clippy::large-enum-variant` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
189 -     Complete(QueryExecution),
189 +     Complete(Box<QueryExecution>),
    |


## clippy::field_reassign_with_default — field assignment outside of initializer for an instance created with Default::default()

error: field assignment outside of initializer for an instance created with Default::default()
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs:987:13
    |
987 |             metadata.name = std::mem::take(&mut self.graph.name);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: consider initializing the variable with `standards::v1::subsets::any::schema::snapshot::component::JackSnapshot { name: std::mem::take(&mut self.graph.name), manifest_id: self.graph.manifest_id.take(), manifest: std::mem::take(&mut self.graph.manifest), camera: self.graph.camera.clone(), root_node_id: self.graph.root_node_id.take(), ..Default::default() }` and removing relevant reassignments
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs:986:13
    |
986 |             let mut metadata = JackSnapshot::default();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#field_reassign_with_default
    = note: `-D clippy::field-reassign-with-default` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::field_reassign_with_default)]`


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../🦀️.rs:457:5
    |
457 |     pub fn with_content(schema: String, name: String, manifest_id: Option<String>, manifest: Manifest, camera: Camera, nodes: Vec<Node>, edges: Vec<Edge>, root_node_id: Option<String>) -> Self {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::large_enum_variant — large size difference between variants

error: large size difference between variants
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs:751:1
    |
751 | / enum JackSnapshotDecodeState {
752 | |     AwaitToken,
    | |     ---------- the second-largest variant carries no data at all
753 | |     Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    | |     -------------------------------------------------------------- the largest variant contains at least 320 bytes
754 | |     Ready,
...   |
757 | |     Complete,
758 | | }
    | |_^ the entire enum is at least 320 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
753 -     Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
753 +     Decode(Box<store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>>),
    |


## clippy::large_enum_variant — large size difference between variants

error: large size difference between variants
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs:868:1
    |
868 | / enum JackMutationDecodeState {
869 | |     AwaitToken,
    | |     ---------- the second-largest variant carries no data at all
870 | |     Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    | |     -------------------------------------------------------------- the largest variant contains at least 320 bytes
871 | |     Ready,
...   |
874 | |     Complete,
875 | | }
    | |_^ the entire enum is at least 320 bytes
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
    |
870 -     Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
870 +     Decode(Box<store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>>),
    |


## clippy::new_without_default — you should consider adding a `Default` implementation for `JackSnapshotCloneAuthority`

error: you should consider adding a `Default` implementation for `JackSnapshotCloneAuthority`
    --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs:1066:5
     |
1066 | /     pub fn new() -> Self {
1067 | |         Self::with_local_owner(true)
1068 | |     }
     | |_____^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#new_without_default
     = note: `-D clippy::new-without-default` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::new_without_default)]`
help: try adding this
     |
1065 + impl Default for JackSnapshotCloneAuthority {
1066 +     fn default() -> Self {
1067 +         Self::new()
1068 +     }
1069 + }
     |


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:326:1
    |
326 | / fn jack_retained_window_config_reduce(
327 | |     command: &TrinityJackCommand,
328 | |     _snapshot: &JackSnapshot,
329 | |     _config: &NoConfig,
...   |
334 | |     _operation: &AppOperationContext,
335 | | ) -> Result<Emit<TrinityGraphMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    | |_________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


## E0277 — the trait bound `std::boxed::Box<editor::cad::config::component::CadConfig>: dsl::DslField` is not satisfied

error[E0277]: the trait bound `std::boxed::Box<editor::cad::config::component::CadConfig>: dsl::DslField` is not satisfied
   --> ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:248:17
    |
248 |         config: Box<CadConfig>,
    |                 ^^^^^^^^^^^^^^ the trait `dsl::DslField` is not implemented for `std::boxed::Box<editor::cad::config::component::CadConfig>`
    |
    = help: the following other types implement trait `dsl::DslField`:
              CadCamera
              CadNode
              CadPaneId
              CadProjectionDsl
              CadReference
              [T; N]
              bool
              dsl::ArtifactChild<S>
            and 445 others
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_cad_cad-5bfafee17a9b6bfc.long-type-7035434274395263872.txt'
    = note: consider using `--verbose` to print the full type name to the console


## E0277 — the trait bound `std::boxed::Box<editor::cad::config::component::CadConfig>: dsl::DslField` is not satisfied

error[E0277]: the trait bound `std::boxed::Box<editor::cad::config::component::CadConfig>: dsl::DslField` is not satisfied
   --> ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:248:9
    |
243 | #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
    |                                                       ----------- required by a bound introduced by this call
...
248 |         config: Box<CadConfig>,
    |         ^^^^^^ the trait `dsl::DslField` is not implemented for `std::boxed::Box<editor::cad::config::component::CadConfig>`
    |
    = help: the following other types implement trait `dsl::DslField`:
              CadCamera
              CadNode
              CadPaneId
              CadProjectionDsl
              CadReference
              [T; N]
              bool
              dsl::ArtifactChild<S>
            and 445 others
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_s_artifact_cad_cad-5bfafee17a9b6bfc.long-type-7035434274395263872.txt'
    = note: consider using `--verbose` to print the full type name to the console


## clippy::needless_update — struct update has no effect, all the fields in the struct have already been specified

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:45:133
   |
45 | ... snapshot.imported_features_json, mesh: snapshot.mesh, ..Self::default() }
   |                                                             ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:144:1
    |
144 | / fn gis3d_retained_reduce(
145 | |     command: &Gis3dCommand,
146 | |     snapshot: &GisTerrainSnapshot,
147 | |     config: &Gis3dConfig,
...   |
152 | |     operation: &AppOperationContext,
153 | | ) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation, NoDraftMutation>, Fault> {
    | |__________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::needless_update — struct update has no effect, all the fields in the struct have already been specified

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs:21:100
   |
21 |             MutationOutcome::new(Gis3dConfigDelta { camera_json: Some(self.camera_json.clone()), ..Default::default() }.into())
   |                                                                                                    ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update


## clippy::empty_line_after_outer_attr — empty line after outer attribute

error: empty line after outer attribute
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:22:1
   |
22 | / #[artifact_schema(id = "s.procedural.generation2d")]
23 | |
   | |_^
24 |   pub struct Generation2dArtifact {
   |   ------------------------------- the attribute applies to this struct
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr
   = note: `-D clippy::empty-line-after-outer-attr` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::empty_line_after_outer_attr)]`
   = help: if the empty line is unintentional, remove it


## clippy::empty_line_after_outer_attr — empty line after outer attribute

error: empty line after outer attribute
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:11:1
   |
11 | / #[artifact_schema(id = "s.procedural.generation2d")]
12 | |
   | |_^
13 |   pub struct Generation2dSnapshot {
   |   ------------------------------- the attribute applies to this struct
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr
   = help: if the empty line is unintentional, remove it


## clippy::empty_line_after_outer_attr — empty line after outer attribute

error: empty line after outer attribute
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:11:1
   |
11 | / #[artifact_schema(id = "s.procedural.generation2d")]
12 | |
   | |_^
13 |   pub struct Generation2dDiff {
   |   --------------------------- the attribute applies to this struct
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr
   = help: if the empty line is unintentional, remove it


## clippy::empty_line_after_doc_comments — empty line after doc comment

error: empty line after doc comment
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:29:1
   |
29 | / /// would align them; not editable here — glue.rs is shared with the sibling `generation3d` artifact).
30 | |
   | |_^
...
33 |   pub fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
   |   ------------------- the comment documents this function
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_doc_comments
   = note: `-D clippy::empty-line-after-doc-comments` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::empty_line_after_doc_comments)]`
   = help: if the empty line is unintentional, remove it
help: if the doc comment should not document function `widget_index` then comment it out
   |
26 ~ // /// 🧵 Sibling triad-leaf modules wired by `🦀️.rs` under eight pre-existing (pre-semantic)
27 ~ // /// directory slots — their directory/module names are leftovers of the generic slots each was
28 ~ // /// repurposed from (`sharedFileRequests` in this ticket's wave2 report has the glue.rs rename that
29 ~ // /// would align them; not editable here — glue.rs is shared with the sibling `generation3d` artifact).
   |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or_else(<g>)` on an `Option` value

error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../../../🫀️core/🖼️semantic-ui/🦀️.rs:179:35
    |
179 | ... = value.as_array().map(<[DslValue]>::to_vec).unwrap_or_else(|| question.fields.as_deref().unwrap_or_default().iter().map(|field| DslValue::float(field.value.unwrap_or(0.0))).collect());
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `value.as_array().map_or_else(|| question.fields.as_deref().unwrap_or_default().iter().map(|field| DslValue::float(field.value.unwrap_or(0.0))).collect(), <[DslValue]>::to_vec)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
    = note: `-D clippy::map-unwrap-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_unwrap_or)]`


## clippy::map_unwrap_or — called `map(<f>).unwrap_or_else(<g>)` on an `Option` value

error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../../../🫀️core/🖼️semantic-ui/🦀️.rs:180:47
    |
180 |                       let labels: Vec<String> = question
    |  _______________________________________________^
181 | |                         .fields
182 | |                         .as_deref()
183 | |                         .map(|fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect())
184 | |                         .unwrap_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect());
    | |_________________________________________________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: try
    |
180 ~                     let labels: Vec<String> = question
181 +                         .fields
182 ~                         .as_deref().map_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect(), |fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect());
    |


## clippy::derivable_impls — this `impl` can be derived

error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:32:1
   |
32 | / impl Default for Generation2dArtifact {
33 | |     fn default() -> Self {
34 | |         Self { fixture: FlowFixture::default(), generation: GenerationPlayRoot::default() }
35 | |     }
36 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
24 + #[derive(Default)]
25 | pub struct Generation2dArtifact {
   |


## clippy::needless_update — struct update has no effect, all the fields in the struct have already been specified

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:46:78
   |
46 |         Self { fixture: snapshot.fixture, generation: snapshot.generation, ..Self::default() }
   |                                                                              ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


## clippy::derivable_impls — this `impl` can be derived

error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:21:1
   |
21 | / impl Default for Generation2dSnapshot {
22 | |     fn default() -> Self {
23 | |         Self { fixture: FlowFixture::default(), generation: GenerationPlayRoot::default() }
24 | |     }
25 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
help: replace the manual implementation with a derive attribute
   |
13 + #[derive(Default)]
14 | pub struct Generation2dSnapshot {
   |


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:420:5
    |
420 |     fn begin_dsl(&mut self) -> Result<bool, &'static str> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
    = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
    |
420 -     fn begin_dsl(&mut self) -> Result<bool, &'static str> {
420 +     fn begin_dsl(&mut self) -> bool {
    |
help: ...and then remove the surrounding `Ok()` from returning expressions
    |
422 ~             return true;
423 |         }
424 ~         let Some(parent) = self.stack.len().checked_sub(1) else { return false };
425 |         let field = match self.stack.get(parent) {
426 |             Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field: Some(field @ (2 | 3)), .. }) if widget.keyword == "cluster" => *field,
427 ~             _ => return false,
428 |         };
429 |         self.dsl_destination = Some((parent, usize::from(field - 2)));
430 ~         true
    |


## clippy::single_match — you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`

error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:789:66
    |
789 |   ...   Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
    |  ____________________________________________________________^
790 | | ...       Some(
791 | | ...           Generation2dMountedContainerOwner::Synapses { field, present, next, .. } | Generation2dMountedContainerOwner::Gener...
792 | | ...       ) => {
...   |
797 | | ...       _ => {}
798 | | ...   },
    | |_______^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_match
    = note: `-D clippy::single-match` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::single_match)]`
help: try
    |
789 ~             Token::Unsigned { role: Role::TableField, value } => if let Some(
790 +                     Generation2dMountedContainerOwner::Synapses { field, present, next, .. } | Generation2dMountedContainerOwner::Generations { field, present, next, .. } | Generation2dMountedContainerOwner::Dictionary { field, present, next, .. },
791 +                 ) = self.stack.last_mut() {
792 +                 *field = Some(u16::try_from(value).map_err(|_| "generation2d-mounted.table-field")?);
793 +                 present.fill(false);
794 +                 *next = 0;
795 ~             },
    |


## clippy::collapsible_match — this `if` can be collapsed into the outer `match`

error: this `if` can be collapsed into the outer `match`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:871:21
    |
871 | /                     if value == 0 {
872 | |                         present.fill(true);
873 | |                     }
    | |_____________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_match
    = note: `-D clippy::collapsible-match` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::collapsible_match)]`
help: collapse nested if block
    |
870 ~                 Some(Generation2dMountedContainerOwner::Synapses { present, .. } | Generation2dMountedContainerOwner::Generations { present, .. } | Generation2dMountedContainerOwner::Dictionary { present, .. }) if rows as usize == present.len()
871 ~                     && value == 0 => {
872 |                         present.fill(true);
873 ~                     }
    |


## clippy::single_match — you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`

error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:877:56
    |
877 |   ...   Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
    |  __________________________________________________^
878 | | ...       Some(Generation2dMountedContainerOwner::Synapses { present, .. } | Generation2dMountedContainerOwner::Generations { pre...
879 | | ...           for bit in 0..8 {
880 | | ...               let row = first_row as usize + bit;
...   |
886 | | ...       _ => {}
887 | | ...   },
    | |_______^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_match
help: try
    |
877 ~             Token::TableBitmap { first_row, value } => if let Some(Generation2dMountedContainerOwner::Synapses { present, .. } | Generation2dMountedContainerOwner::Generations { present, .. } | Generation2dMountedContainerOwner::Dictionary { present, .. }) = self.stack.last_mut() {
878 +                 for bit in 0..8 {
879 +                     let row = first_row as usize + bit;
880 +                     if row < present.len() {
881 +                         present[row] = value & (1 << bit) != 0;
882 +                     }
883 +                 }
884 ~             },
    |


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:155:72
    |
155 | pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, cam...
    |                                                                        ^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
155 | pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: &WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation2dDiff {
    |                                                                        +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:155:95
    |
155 | pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, cam...
    |                                                                                               ^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
155 | pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: &SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation2dDiff {
    |                                                                                               +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:155:117
    |
155 | pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, cam...
    |                                                                                                                     ^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
155 | pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: &LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation2dDiff {
    |                                                                                                                     +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:155:137
    |
155 | ...f, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation2dDiff {
    |                                                           ^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
155 | pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<&CameraJson>, schema: Option<String>) -> Generation2dDiff {
    |                                                                                                                                                +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:155:165
    |
155 | ... layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation2dDiff {
    |                                                             ^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
155 | pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<&String>) -> Generation2dDiff {
    |                                                                                                                                                                            +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:161:67
    |
161 | pub fn diff_generation_from_ops(base: &Generation2dSnapshot, ops: Vec<GenerationMutation>) -> Generation2dDiff {
    |                                                                   ^^^^^^^^^^^^^^^^^^^^^^^ help: consider changing the type to: `&[GenerationMutation]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:300:1
    |
300 | / pub fn generation2d_admit_publication_authority(
301 | |     operation: semio_framework_job::OperationId,
302 | |     generation: semio_framework_job::Generation,
303 | |     base_revision: u64,
...   |
308 | |     maximum_controls: usize,
309 | | ) -> Result<(), &'static str> {
    | |_____________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:535:63
    |
535 |                 Generation2dReplayDisplaced::Camera(value) => drop(value),
    |                                                               ^^^^^^^^^^^
    |
note: argument has type `semio_framework_artifact_flow_flow::CameraJson`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:535:68
    |
535 |                 Generation2dReplayDisplaced::Camera(value) => drop(value),
    |                                                                    ^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop
    = note: `-D clippy::drop-non-drop` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::drop_non_drop)]`


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Option`

error: this function's return value is unnecessarily wrapped by `Option`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:556:1
    |
556 | fn generation2d_retire_displaced(value: Generation2dReplayDisplaced) -> Option<Box<dyn store::ErasedSnapshotRetirement>> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Option` from the return type...
    |
556 - fn generation2d_retire_displaced(value: Generation2dReplayDisplaced) -> Option<Box<dyn store::ErasedSnapshotRetirement>> {
556 + fn generation2d_retire_displaced(value: Generation2dReplayDisplaced) -> std::boxed::Box<(dyn dsl::ErasedSnapshotRetirement + 'static)> {
    |
help: ...and then remove the surrounding `Some()` from returning expressions
    |
557 -     Some(Box::new(Generation2dReplayRetirement { value: std::mem::ManuallyDrop::new(Some(value)), domain: semio_framework_artifact_flow_flow::retained::FlowRetirement::default() }))
557 +     Box::new(Generation2dReplayRetirement { value: std::mem::ManuallyDrop::new(Some(value)), domain: semio_framework_artifact_flow_flow::retained::FlowRetirement::default() })
    |


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1186:5
     |
1186 |     fn begin_dsl(&mut self) -> Result<bool, &'static str> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
1186 -     fn begin_dsl(&mut self) -> Result<bool, &'static str> {
1186 +     fn begin_dsl(&mut self) -> bool {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
1188 ~             return true;
1189 |         }
1190 ~         let Some(parent) = self.stack.len().checked_sub(1) else { return false };
1191 |         let field = match self.stack.get(parent) {
1192 |             Some(Generation2dMutationFrame::Widget { field: Some(field @ (2 | 3)), owner }) if owner.keyword == "cluster" => *field,
1193 ~             _ => return false,
1194 |         };
1195 |         self.dsl_destination = Some((parent, usize::from(field - 2)));
1196 ~         true
     |


## clippy::single_match — you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`

error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1393:66
     |
1393 |               Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
     |  __________________________________________________________________^
1394 | |                 Some(Generation2dMutationFrame::Dictionary { field, present, next, .. }) => {
1395 | |                     *field = Some(u16::try_from(value).map_err(|_| "generation2d-mutation.dictionary-field")?);
1396 | |                     present.fill(false);
...    |
1399 | |                 _ => {}
1400 | |             },
     | |_____________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_match
help: try
     |
1393 ~             Token::Unsigned { role: Role::TableField, value } => if let Some(Generation2dMutationFrame::Dictionary { field, present, next, .. }) = self.stack.last_mut() {
1394 +                 *field = Some(u16::try_from(value).map_err(|_| "generation2d-mutation.dictionary-field")?);
1395 +                 present.fill(false);
1396 +                 *next = 0;
1397 ~             },
     |


## clippy::collapsible_match — this `if` can be collapsed into the outer `match`

error: this `if` can be collapsed into the outer `match`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1513:21
     |
1513 | /                     if value == 0 {
1514 | |                         present.fill(true);
1515 | |                     }
     | |_____________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_match
help: collapse nested if block
     |
1512 ~                 Some(Generation2dMutationFrame::Dictionary { present, .. }) if rows as usize == present.len()
1513 ~                     && value == 0 => {
1514 |                         present.fill(true);
1515 ~                     }
     |


## clippy::single_match — you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`

error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1519:56
     |
1519 |               Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
     |  ________________________________________________________^
1520 | |                 Some(Generation2dMutationFrame::Dictionary { present, .. }) => {
1521 | |                     for bit in 0..8 {
1522 | |                         let row = first_row as usize + bit;
...    |
1528 | |                 _ => {}
1529 | |             },
     | |_____________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_match
help: try
     |
1519 ~             Token::TableBitmap { first_row, value } => if let Some(Generation2dMutationFrame::Dictionary { present, .. }) = self.stack.last_mut() {
1520 +                 for bit in 0..8 {
1521 +                     let row = first_row as usize + bit;
1522 +                     if row < present.len() {
1523 +                         present[row] = value & (1 << bit) != 0;
1524 +                     }
1525 +                 }
1526 ~             },
     |


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1630:9
     |
1630 |         drop(self.layout.take());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<semio_framework_artifact_flow_flow::WidgetLayout>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1630:14
     |
1630 |         drop(self.layout.take());
     |              ^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1631:9
     |
1631 |         drop(self.camera.take());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<semio_framework_artifact_flow_flow::CameraJson>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1631:14
     |
1631 |         drop(self.camera.take());
     |              ^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::manual_range_contains — manual `!RangeInclusive::contains` implementation

error: manual `!RangeInclusive::contains` implementation
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1694:12
     |
1694 |         if expected_bytes < 3 || expected_bytes > GENERATION2D_OWNER_BYTES || maximum_items == 0 {
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `!(3..=GENERATION2D_OWNER_BYTES).contains(&expected_bytes)`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_range_contains
     = note: `-D clippy::manual-range-contains` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::manual_range_contains)]`


## clippy::collapsible_match — this `if let` can be collapsed into the outer `if let`

error: this `if let` can be collapsed into the outer `if let`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1784:21
     |
1784 | /                     if let store::mounted_pack_rt::RetainedRecordBodyToken::Value(token) = event {
1785 | |                         let complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
1786 | |                         let body = self.body.as_ref().expect("P2 retained mutation body");
1787 | |                         self.owner.as_mut().expect("P2 retained mutation owner").accept(token, body)?;
...    |
1792 | |                     }
     | |_____________________^
     |
help: the outer pattern can be modified to include the inner pattern
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1783:29
     |
1783 | ...   if let Some(event) = self.body.as_mut().ok_or("generation2d-mutation.body-owner")?.grant().map_err(|_| "generation2d-mutatio...
     |                   ^^^^^ replace this binding
1784 | ...       if let store::mounted_pack_rt::RetainedRecordBodyToken::Value(token) = event {
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ with this pattern
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_match


## clippy::clone_on_copy — using `clone` on type `Number` which implements the `Copy` trait

error: using `clone` on type `Number` which implements the `Copy` trait
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:2343:63
     |
2343 |         dsl::DslValue::Number(value) => dsl::DslValue::Number(value.clone()),
     |                                                               ^^^^^^^^^^^^^ help: try dereferencing it: `*value`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy
     = note: `-D clippy::clone-on-copy` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::clone_on_copy)]`


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:2997:21
     |
2997 |                     drop(self.candidate_disposer.take());
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<standards::v1::subsets::any::schema::snapshot::component::Generation2dSnapshot, standards::v1::subsets::any::schema::mutations::component::Generation2dMutation>>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:2997:26
     |
2997 |                     drop(self.candidate_disposer.take());
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::map_unwrap_or — called `map(<f>).unwrap_or_else(<g>)` on an `Option` value

error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3157:34
     |
3157 |                           let id = entry
     |  __________________________________^
3158 | |                             .mutation_meta
3159 | |                             .get(index)
3160 | |                             .and_then(|meta| meta.mutation_id.as_ref())
3161 | |                             .map(|id| protocol::MutationId(generation2d_copy_string(&id.0).unwrap_or_default()))
3162 | |                             .unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
     | |_____________________________________________________________________________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: try
     |
3157 ~                         let id = entry
3158 +                             .mutation_meta
3159 +                             .get(index)
3160 ~                             .and_then(|meta| meta.mutation_id.as_ref()).map_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)), |id| protocol::MutationId(generation2d_copy_string(&id.0).unwrap_or_default()));
     |


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3314:21
     |
3314 |                     drop(self.initial_digest.take());
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3314:26
     |
3314 |                     drop(self.initial_digest.take());
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3315:21
     |
3315 |                     drop(self.edit_digest.take());
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3315:26
     |
3315 |                     drop(self.edit_digest.take());
     |                          ^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3356:9
     |
3356 |         drop(self.initial_digest.take());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3356:14
     |
3356 |         drop(self.initial_digest.take());
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3357:9
     |
3357 |         drop(self.edit_digest.take());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3357:14
     |
3357 |         drop(self.edit_digest.take());
     |              ^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3377:17
     |
3377 |                 drop(self.initial_digest.take());
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3377:22
     |
3377 |                 drop(self.initial_digest.take());
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3378:17
     |
3378 |                 drop(self.edit_digest.take());
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3378:22
     |
3378 |                 drop(self.edit_digest.take());
     |                      ^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::explicit_auto_deref — deref which would be done by auto-deref

error: deref which would be done by auto-deref
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:40:71
   |
40 |                         AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
   |                                                                       ^^ help: try: `t`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref
   = note: `-D clippy::explicit-auto-deref` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::explicit_auto_deref)]`


## clippy::explicit_auto_deref — deref which would be done by auto-deref

error: deref which would be done by auto-deref
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:41:75
   |
41 |                         AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
   |                                                                           ^^ help: try: `b`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Option`

error: this function's return value is unnecessarily wrapped by `Option`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:131:1
    |
131 | fn generation2d_bounded_extent(_command: &Generation2dCommand, _snapshot: &Generation2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Option` from the return type...
    |
131 - fn generation2d_bounded_extent(_command: &Generation2dCommand, _snapshot: &Generation2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
131 + fn generation2d_bounded_extent(_command: &Generation2dCommand, _snapshot: &Generation2dSnapshot, _interaction: &protocol::InteractionState) -> usize {
    |
help: ...and then remove the surrounding `Some()` from returning expressions
    |
132 -     Some(1)
132 +     1
    |


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:135:1
    |
135 | / fn generation2d_retained_reduce(
136 | |     command: &Generation2dCommand,
137 | |     snapshot: &Generation2dSnapshot,
138 | |     config: &Generation2dConfig,
...   |
143 | |     operation: &AppOperationContext,
144 | | ) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation, NoDraftMutation>, Fault> {
    | |___________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on a `Result` value

error: called `map(<f>).unwrap_or(<a>)` on a `Result` value
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs:21:20
   |
21 |         let body = store::semio_format::split_text_preamble(text).map(|(_, rest)| rest).unwrap_or(text);
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
   |
21 -         let body = store::semio_format::split_text_preamble(text).map(|(_, rest)| rest).unwrap_or(text);
21 +         let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, rest)| rest);
   |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs:64:41
   |
64 |                 ("generationId".into(), payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null)),
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
   |
64 -                 ("generationId".into(), payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null)),
64 +                 ("generationId".into(), payload.generation_id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String)),
   |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs:71:61
   |
71 | ...ue::object([("id".into(), payload.id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null))]);
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
   |
71 -             let args = dsl::DslValue::object([("id".into(), payload.id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null))]);
71 +             let args = dsl::DslValue::object([("id".into(), payload.id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String))]);
   |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️select-generation/🦀️.rs:18:53
   |
18 |     let args = dsl::DslValue::object([("id".into(), payload.id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null))]);
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
   |
18 -     let args = dsl::DslValue::object([("id".into(), payload.id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null))]);
18 +     let args = dsl::DslValue::object([("id".into(), payload.id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String))]);
   |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️update-generation-values/🦀️.rs:21:33
   |
21 |         ("generationId".into(), payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null)),
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
   |
21 -         ("generationId".into(), payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null)),
21 +         ("generationId".into(), payload.generation_id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String)),
   |


## clippy::unnecessary_to_owned — unnecessary use of `to_string`

error: unnecessary use of `to_string`
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs:31:99
   |
31 | ...ets.iter().map(|widget| tree_item(widget_id(widget).to_string(), widget_id(widget).to_string())))?;
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `widget_id(widget)`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_to_owned
   = note: `-D clippy::unnecessary-to-owned` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::unnecessary_to_owned)]`


## clippy::empty_line_after_outer_attr — empty line after outer attribute

error: empty line after outer attribute
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:24:1
   |
24 | / #[artifact_schema(id = "s.procedural.generation3d")]
25 | |
   | |_^
26 |   pub struct Generation3dArtifact {
   |   ------------------------------- the attribute applies to this struct
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr
   = note: `-D clippy::empty-line-after-outer-attr` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::empty_line_after_outer_attr)]`
   = help: if the empty line is unintentional, remove it


## clippy::empty_line_after_outer_attr — empty line after outer attribute

error: empty line after outer attribute
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:13:1
   |
13 | / #[artifact_schema(id = "s.procedural.generation3d")]
14 | |
   | |_^
15 |   pub struct Generation3dSnapshot {
   |   ------------------------------- the attribute applies to this struct
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr
   = help: if the empty line is unintentional, remove it


## clippy::empty_line_after_outer_attr — empty line after outer attribute

error: empty line after outer attribute
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:11:1
   |
11 | / #[artifact_schema(id = "s.procedural.generation3d")]
12 | |
   | |_^
13 |   pub struct Generation3dDiff {
   |   --------------------------- the attribute applies to this struct
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr
   = help: if the empty line is unintentional, remove it


## clippy::map_unwrap_or — called `map(<f>).unwrap_or_else(<g>)` on an `Option` value

error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../../../🫀️core/🖼️semantic-ui/🦀️.rs:179:35
    |
179 | ... = value.as_array().map(<[DslValue]>::to_vec).unwrap_or_else(|| question.fields.as_deref().unwrap_or_default().iter().map(|field| DslValue::float(field.value.unwrap_or(0.0))).collect());
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `value.as_array().map_or_else(|| question.fields.as_deref().unwrap_or_default().iter().map(|field| DslValue::float(field.value.unwrap_or(0.0))).collect(), <[DslValue]>::to_vec)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
    = note: `-D clippy::map-unwrap-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_unwrap_or)]`


## clippy::map_unwrap_or — called `map(<f>).unwrap_or_else(<g>)` on an `Option` value

error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../../../🫀️core/🖼️semantic-ui/🦀️.rs:180:47
    |
180 |                       let labels: Vec<String> = question
    |  _______________________________________________^
181 | |                         .fields
182 | |                         .as_deref()
183 | |                         .map(|fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect())
184 | |                         .unwrap_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect());
    | |_________________________________________________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: try
    |
180 ~                     let labels: Vec<String> = question
181 +                         .fields
182 ~                         .as_deref().map_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect(), |fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect());
    |


## clippy::needless_update — struct update has no effect, all the fields in the struct have already been specified

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:69:78
   |
69 |         Self { fixture: snapshot.fixture, generation: snapshot.generation, ..Self::default() }
   |                                                                              ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:433:5
    |
433 |     fn begin_dsl(&mut self) -> Result<bool, &'static str> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
    = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
    |
433 -     fn begin_dsl(&mut self) -> Result<bool, &'static str> {
433 +     fn begin_dsl(&mut self) -> bool {
    |
help: ...and then remove the surrounding `Ok()` from returning expressions
    |
435 ~             return true;
436 |         }
437 ~         let Some(parent) = self.stack.len().checked_sub(1) else { return false };
438 |         let field = match self.stack.get(parent) {
439 |             Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(widget), field: Some(field @ (2 | 3)), .. }) if widget.keyword == "cluster" => *field,
440 ~             _ => return false,
441 |         };
442 |         self.dsl_destination = Some((parent, usize::from(field - 2)));
443 ~         true
    |


## clippy::single_match — you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`

error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:802:66
    |
802 |   ...   Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
    |  ____________________________________________________________^
803 | | ...       Some(
804 | | ...           Generation3dMountedContainerOwner::Synapses { field, present, next, .. } | Generation3dMountedContainerOwner::Gener...
805 | | ...       ) => {
...   |
810 | | ...       _ => {}
811 | | ...   },
    | |_______^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_match
    = note: `-D clippy::single-match` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::single_match)]`
help: try
    |
802 ~             Token::Unsigned { role: Role::TableField, value } => if let Some(
803 +                     Generation3dMountedContainerOwner::Synapses { field, present, next, .. } | Generation3dMountedContainerOwner::Generations { field, present, next, .. } | Generation3dMountedContainerOwner::Dictionary { field, present, next, .. },
804 +                 ) = self.stack.last_mut() {
805 +                 *field = Some(u16::try_from(value).map_err(|_| "generation3d-mounted.table-field")?);
806 +                 present.fill(false);
807 +                 *next = 0;
808 ~             },
    |


## clippy::collapsible_match — this `if` can be collapsed into the outer `match`

error: this `if` can be collapsed into the outer `match`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:884:21
    |
884 | /                     if value == 0 {
885 | |                         present.fill(true);
886 | |                     }
    | |_____________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_match
    = note: `-D clippy::collapsible-match` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::collapsible_match)]`
help: collapse nested if block
    |
883 ~                 Some(Generation3dMountedContainerOwner::Synapses { present, .. } | Generation3dMountedContainerOwner::Generations { present, .. } | Generation3dMountedContainerOwner::Dictionary { present, .. }) if rows as usize == present.len()
884 ~                     && value == 0 => {
885 |                         present.fill(true);
886 ~                     }
    |


## clippy::single_match — you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`

error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:890:56
    |
890 |   ...   Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
    |  __________________________________________________^
891 | | ...       Some(Generation3dMountedContainerOwner::Synapses { present, .. } | Generation3dMountedContainerOwner::Generations { pre...
892 | | ...           for bit in 0..8 {
893 | | ...               let row = first_row as usize + bit;
...   |
899 | | ...       _ => {}
900 | | ...   },
    | |_______^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_match
help: try
    |
890 ~             Token::TableBitmap { first_row, value } => if let Some(Generation3dMountedContainerOwner::Synapses { present, .. } | Generation3dMountedContainerOwner::Generations { present, .. } | Generation3dMountedContainerOwner::Dictionary { present, .. }) = self.stack.last_mut() {
891 +                 for bit in 0..8 {
892 +                     let row = first_row as usize + bit;
893 +                     if row < present.len() {
894 +                         present[row] = value & (1 << bit) != 0;
895 +                     }
896 +                 }
897 ~             },
    |


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:154:72
    |
154 | pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, cam...
    |                                                                        ^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
154 | pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: &WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation3dDiff {
    |                                                                        +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:154:95
    |
154 | pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, cam...
    |                                                                                               ^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
154 | pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: WidgetsDiff, synapses: &SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation3dDiff {
    |                                                                                               +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:154:117
    |
154 | pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, cam...
    |                                                                                                                     ^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
154 | pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: &LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation3dDiff {
    |                                                                                                                     +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:154:137
    |
154 | ...f, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation3dDiff {
    |                                                           ^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
154 | pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<&CameraJson>, schema: Option<String>) -> Generation3dDiff {
    |                                                                                                                                                +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:154:165
    |
154 | ... layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation3dDiff {
    |                                                             ^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
154 | pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<&String>) -> Generation3dDiff {
    |                                                                                                                                                                            +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:160:67
    |
160 | pub fn diff_generation_from_ops(base: &Generation3dSnapshot, ops: Vec<GenerationMutation>) -> Generation3dDiff {
    |                                                                   ^^^^^^^^^^^^^^^^^^^^^^^ help: consider changing the type to: `&[GenerationMutation]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:305:1
    |
305 | / pub fn generation3d_admit_publication_authority(
306 | |     operation: semio_framework_job::OperationId,
307 | |     generation: semio_framework_job::Generation,
308 | |     base_revision: u64,
...   |
313 | |     maximum_controls: usize,
314 | | ) -> Result<(), &'static str> {
    | |_____________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:540:63
    |
540 |                 Generation3dReplayDisplaced::Camera(value) => drop(value),
    |                                                               ^^^^^^^^^^^
    |
note: argument has type `semio_framework_artifact_flow_flow::CameraJson`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:540:68
    |
540 |                 Generation3dReplayDisplaced::Camera(value) => drop(value),
    |                                                                    ^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop
    = note: `-D clippy::drop-non-drop` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::drop_non_drop)]`


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Option`

error: this function's return value is unnecessarily wrapped by `Option`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:561:1
    |
561 | fn generation3d_retire_displaced(value: Generation3dReplayDisplaced) -> Option<Box<dyn ErasedSnapshotRetirement>> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Option` from the return type...
    |
561 - fn generation3d_retire_displaced(value: Generation3dReplayDisplaced) -> Option<Box<dyn ErasedSnapshotRetirement>> {
561 + fn generation3d_retire_displaced(value: Generation3dReplayDisplaced) -> std::boxed::Box<(dyn dsl::ErasedSnapshotRetirement + 'static)> {
    |
help: ...and then remove the surrounding `Some()` from returning expressions
    |
562 -     Some(Box::new(Generation3dReplayRetirement { value: std::mem::ManuallyDrop::new(Some(value)), domain: semio_framework_artifact_flow_flow::retained::FlowRetirement::default() }))
562 +     Box::new(Generation3dReplayRetirement { value: std::mem::ManuallyDrop::new(Some(value)), domain: semio_framework_artifact_flow_flow::retained::FlowRetirement::default() })
    |


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1192:5
     |
1192 |     fn begin_dsl(&mut self) -> Result<bool, &'static str> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
1192 -     fn begin_dsl(&mut self) -> Result<bool, &'static str> {
1192 +     fn begin_dsl(&mut self) -> bool {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
1194 ~             return true;
1195 |         }
1196 ~         let Some(parent) = self.stack.len().checked_sub(1) else { return false };
1197 |         let field = match self.stack.get(parent) {
1198 |             Some(Generation3dMutationFrame::Widget { field: Some(field @ (2 | 3)), owner }) if owner.keyword == "cluster" => *field,
1199 ~             _ => return false,
1200 |         };
1201 |         self.dsl_destination = Some((parent, usize::from(field - 2)));
1202 ~         true
     |


## clippy::single_match — you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`

error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1399:66
     |
1399 |               Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
     |  __________________________________________________________________^
1400 | |                 Some(Generation3dMutationFrame::Dictionary { field, present, next, .. }) => {
1401 | |                     *field = Some(u16::try_from(value).map_err(|_| "generation3d-mutation.dictionary-field")?);
1402 | |                     present.fill(false);
...    |
1405 | |                 _ => {}
1406 | |             },
     | |_____________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_match
help: try
     |
1399 ~             Token::Unsigned { role: Role::TableField, value } => if let Some(Generation3dMutationFrame::Dictionary { field, present, next, .. }) = self.stack.last_mut() {
1400 +                 *field = Some(u16::try_from(value).map_err(|_| "generation3d-mutation.dictionary-field")?);
1401 +                 present.fill(false);
1402 +                 *next = 0;
1403 ~             },
     |


## clippy::collapsible_match — this `if` can be collapsed into the outer `match`

error: this `if` can be collapsed into the outer `match`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1519:21
     |
1519 | /                     if value == 0 {
1520 | |                         present.fill(true);
1521 | |                     }
     | |_____________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_match
help: collapse nested if block
     |
1518 ~                 Some(Generation3dMutationFrame::Dictionary { present, .. }) if rows as usize == present.len()
1519 ~                     && value == 0 => {
1520 |                         present.fill(true);
1521 ~                     }
     |


## clippy::single_match — you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`

error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1525:56
     |
1525 |               Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
     |  ________________________________________________________^
1526 | |                 Some(Generation3dMutationFrame::Dictionary { present, .. }) => {
1527 | |                     for bit in 0..8 {
1528 | |                         let row = first_row as usize + bit;
...    |
1534 | |                 _ => {}
1535 | |             },
     | |_____________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_match
help: try
     |
1525 ~             Token::TableBitmap { first_row, value } => if let Some(Generation3dMutationFrame::Dictionary { present, .. }) = self.stack.last_mut() {
1526 +                 for bit in 0..8 {
1527 +                     let row = first_row as usize + bit;
1528 +                     if row < present.len() {
1529 +                         present[row] = value & (1 << bit) != 0;
1530 +                     }
1531 +                 }
1532 ~             },
     |


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1635:9
     |
1635 |         drop(self.layout.take());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<semio_framework_artifact_flow_flow::WidgetLayout>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1635:14
     |
1635 |         drop(self.layout.take());
     |              ^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1636:9
     |
1636 |         drop(self.camera.take());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<semio_framework_artifact_flow_flow::CameraJson>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1636:14
     |
1636 |         drop(self.camera.take());
     |              ^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::manual_range_contains — manual `!RangeInclusive::contains` implementation

error: manual `!RangeInclusive::contains` implementation
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1699:12
     |
1699 |         if expected_bytes < 3 || expected_bytes > GENERATION3D_OWNER_BYTES || maximum_items == 0 {
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `!(3..=GENERATION3D_OWNER_BYTES).contains(&expected_bytes)`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_range_contains
     = note: `-D clippy::manual-range-contains` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::manual_range_contains)]`


## clippy::collapsible_match — this `if let` can be collapsed into the outer `if let`

error: this `if let` can be collapsed into the outer `if let`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1789:21
     |
1789 | /                     if let store::mounted_pack_rt::RetainedRecordBodyToken::Value(token) = event {
1790 | |                         let complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
1791 | |                         let body = self.body.as_ref().expect("P3 retained mutation body");
1792 | |                         self.owner.as_mut().expect("P3 retained mutation owner").accept(token, body)?;
...    |
1797 | |                     }
     | |_____________________^
     |
help: the outer pattern can be modified to include the inner pattern
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1788:29
     |
1788 | ...   if let Some(event) = self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.grant().map_err(|_| "generation3d-mutatio...
     |                   ^^^^^ replace this binding
1789 | ...       if let store::mounted_pack_rt::RetainedRecordBodyToken::Value(token) = event {
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ with this pattern
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_match


## clippy::clone_on_copy — using `clone` on type `Number` which implements the `Copy` trait

error: using `clone` on type `Number` which implements the `Copy` trait
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:2348:63
     |
2348 |         dsl::DslValue::Number(value) => dsl::DslValue::Number(value.clone()),
     |                                                               ^^^^^^^^^^^^^ help: try dereferencing it: `*value`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy
     = note: `-D clippy::clone-on-copy` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::clone_on_copy)]`


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3001:21
     |
3001 |                     drop(self.candidate_disposer.take());
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<standards::v1::subsets::any::schema::snapshot::component::Generation3dSnapshot, standards::v1::subsets::any::schema::mutations::component::Generation3dMutation>>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3001:26
     |
3001 |                     drop(self.candidate_disposer.take());
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::map_unwrap_or — called `map(<f>).unwrap_or_else(<g>)` on an `Option` value

error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3161:34
     |
3161 |                           let id = entry
     |  __________________________________^
3162 | |                             .mutation_meta
3163 | |                             .get(index)
3164 | |                             .and_then(|meta| meta.mutation_id.as_ref())
3165 | |                             .map(|id| protocol::MutationId(generation3d_copy_string(&id.0).unwrap_or_default()))
3166 | |                             .unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
     | |_____________________________________________________________________________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: try
     |
3161 ~                         let id = entry
3162 +                             .mutation_meta
3163 +                             .get(index)
3164 ~                             .and_then(|meta| meta.mutation_id.as_ref()).map_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)), |id| protocol::MutationId(generation3d_copy_string(&id.0).unwrap_or_default()));
     |


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3318:21
     |
3318 |                     drop(self.initial_digest.take());
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3318:26
     |
3318 |                     drop(self.initial_digest.take());
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3319:21
     |
3319 |                     drop(self.edit_digest.take());
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3319:26
     |
3319 |                     drop(self.edit_digest.take());
     |                          ^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3360:9
     |
3360 |         drop(self.initial_digest.take());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3360:14
     |
3360 |         drop(self.initial_digest.take());
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3361:9
     |
3361 |         drop(self.edit_digest.take());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3361:14
     |
3361 |         drop(self.edit_digest.take());
     |              ^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3381:17
     |
3381 |                 drop(self.initial_digest.take());
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3381:22
     |
3381 |                 drop(self.initial_digest.take());
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::drop_non_drop — call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes

error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3382:17
     |
3382 |                 drop(self.edit_digest.take());
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: argument has type `std::option::Option<dsl::ArtifactStoreInitializationDigest>`
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3382:22
     |
3382 |                 drop(self.edit_digest.take());
     |                      ^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_non_drop


## clippy::explicit_auto_deref — deref which would be done by auto-deref

error: deref which would be done by auto-deref
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:42:71
   |
42 |                         AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
   |                                                                       ^^ help: try: `t`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref
   = note: `-D clippy::explicit-auto-deref` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::explicit_auto_deref)]`


## clippy::explicit_auto_deref — deref which would be done by auto-deref

error: deref which would be done by auto-deref
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:43:75
   |
43 |                         AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
   |                                                                           ^^ help: try: `b`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Option`

error: this function's return value is unnecessarily wrapped by `Option`
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:221:1
    |
221 | fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Option` from the return type...
    |
221 - fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
221 + fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> usize {
    |
help: ...and then remove the surrounding `Some()` from returning expressions
    |
222 -     Some(1)
222 +     1
    |


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:344:1
    |
344 | / fn generation3d_retained_reduce(
345 | |     command: &Generation3dCommand,
346 | |     snapshot: &Generation3dSnapshot,
347 | |     config: &Generation3dConfig,
...   |
352 | |     operation: &AppOperationContext,
353 | | ) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>, Fault> {
    | |___________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


## clippy::redundant_closure — redundant closure

error: redundant closure
    --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1007:120
     |
1007 | ...ap(|value| dsl::json::to_json_string(value))).unwrap_or_else(|| "[]".into()),
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace the closure with the function itself: `dsl::json::to_json_string`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure
     = note: `-D clippy::redundant-closure` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::redundant_closure)]`


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs:57:33
   |
57 |             let generation_id = payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null);
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
   |
57 -             let generation_id = payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null);
57 +             let generation_id = payload.generation_id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String);
   |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️update-generation-values/🦀️.rs:35:25
   |
35 |     let generation_id = payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null);
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
   |
35 -     let generation_id = payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null);
35 +     let generation_id = payload.generation_id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String);
   |


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
    --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs:2310:5
     |
2310 |     fn push_item(&mut self, page: usize, strings: [Option<&str>; 4], numbers: [f64; 16], number_len: u8, indexes: [u32; 8], index_len: u8, flags: u16) -> Result<(), Vec<u8>> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
     = note: `-D clippy::too-many-arguments` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::derivable_impls — this `impl` can be derived

error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs:33:1
   |
33 | / impl Default for Fem3dArtifact {
34 | |     fn default() -> Self {
35 | |         Self {
36 | |             nodes: Default::default(),
...  |
47 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
10 + #[derive(Default)]
11 | pub struct Fem3dArtifact {
   |


## clippy::needless_update — struct update has no effect, all the fields in the struct have already been specified

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs:77:15
   |
77 |             ..Self::default()
   |               ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:173:1
    |
173 | / fn fem3d_retained_reduce(
174 | |     command: &Fem3dCommand,
175 | |     snapshot: &Fem3dSnapshot,
176 | |     config: &Fem3dConfig,
...   |
181 | |     operation: &AppOperationContext,
182 | | ) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, NoDraftMutation>, Fault> {
    | |_____________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../🦀️.rs:102:31
    |
102 | ...ks_json = node.params.iter().find(|param| param.key == "blocksJson").map(|param| param.value.as_str()).unwrap_or("[]");
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
    = note: `-D clippy::map-unwrap-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_unwrap_or)]`
help: use `map_or(<a>, <f>)` instead
    |
102 -             let blocks_json = node.params.iter().find(|param| param.key == "blocksJson").map(|param| param.value.as_str()).unwrap_or("[]");
102 +             let blocks_json = node.params.iter().find(|param| param.key == "blocksJson").map_or("[]", |param| param.value.as_str());
    |


## clippy::type_complexity — very complex type used. Consider factoring parts into `type` definitions

error: very complex type used. Consider factoring parts into `type` definitions
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../🦀️.rs:273:15
    |
273 |     let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity
    = note: `-D clippy::type-complexity` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::type_complexity)]`


## clippy::needless_update — struct update has no effect, all the fields in the struct have already been specified

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:44:160
   |
44 | ...tle, document: snapshot.document, flow: snapshot.flow, ..Self::default() }
   |                                                             ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


## clippy::manual_is_multiple_of — manual implementation of `.is_multiple_of()`

error: manual implementation of `.is_multiple_of()`
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:112:8
    |
112 |     if s.len() % 2 != 0 {
    |        ^^^^^^^^^^^^^^^^ help: replace with: `!s.len().is_multiple_of(2)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_is_multiple_of
    = note: `-D clippy::manual-is-multiple-of` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::manual_is_multiple_of)]`


## clippy::field_reassign_with_default — field assignment outside of initializer for an instance created with Default::default()

error: field assignment outside of initializer for an instance created with Default::default()
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:256:5
    |
256 |     snapshot.schema = read_str_lp(&mut reader)?;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: consider initializing the variable with `standards::v1::subsets::any::schema::snapshot::component::PlaybookSnapshot { schema: read_str_lp(&mut reader)?, id: read_str_lp(&mut reader)?, version: read_str_lp(&mut reader)?, title: read_opt_str(&mut reader)?, document: read_child(&mut reader)?, flow: read_child(&mut reader)? }` and removing relevant reassignments
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:255:5
    |
255 |     let mut snapshot = PlaybookSnapshot::default();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#field_reassign_with_default
    = note: `-D clippy::field-reassign-with-default` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::field_reassign_with_default)]`


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs:86:28
   |
86 | fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> PlaybookTopology {
   |                            ^^^^^^^^^^^ help: consider changing the type to: `&[String]`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
   = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs:86:48
   |
86 | fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> PlaybookTopology {
   |                                                ^^^^^^^^^^^^^^^^^^^^^ help: consider changing the type to: `&[(String, String)]`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


## clippy::explicit_auto_deref — deref which would be done by auto-deref

error: deref which would be done by auto-deref
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:36:71
   |
36 |                         AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
   |                                                                       ^^ help: try: `t`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref
   = note: `-D clippy::explicit-auto-deref` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::explicit_auto_deref)]`


## clippy::explicit_auto_deref — deref which would be done by auto-deref

error: deref which would be done by auto-deref
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:37:75
   |
37 |                         AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
   |                                                                           ^^ help: try: `b`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:140:150
    |
140 | ...tes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
    |                                                     ^^^^^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:140:149
    |
140 | ...:deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
    |                                                                   ^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone
    = note: `-D clippy::redundant-clone` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::redundant_clone)]`


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:149:151
    |
149 | ...(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                     ^^^^^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:149:150
    |
149 | ...serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                                   ^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:157:148
    |
157 | ...(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                     ^^^^^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:157:147
    |
157 | ...serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                                   ^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:165:155
    |
165 | ...(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                     ^^^^^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:165:154
    |
165 | ...serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                                   ^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:173:155
    |
173 | ...(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                     ^^^^^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:173:154
    |
173 | ...serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                                   ^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:181:154
    |
181 | ...(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                     ^^^^^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:181:153
    |
181 | ...serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
    |                                                                   ^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone


## clippy::useless_format — useless use of `format!`

error: useless use of `format!`
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs:10:23
   |
10 |     snap.title = Some(format!("Imported pdf"));
   |                       ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"Imported pdf".to_string()`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#useless_format
   = note: `-D clippy::useless-format` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::useless_format)]`


## clippy::useless_format — useless use of `format!`

error: useless use of `format!`
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs:10:23
   |
10 |     snap.title = Some(format!("Imported docx"));
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"Imported docx".to_string()`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#useless_format


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:115:1
    |
115 | / fn playbook_retained_reduce(
116 | |     command: &PlaybookCommand,
117 | |     snapshot: &PlaybookSnapshot,
118 | |     config: &PlaybookConfig,
...   |
123 | |     operation: &AppOperationContext,
124 | | ) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, NoDraftMutation>, Fault> {
    | |___________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::redundant_clone — redundant clone

error: redundant clone
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️steps/🦀️.rs:45:39
   |
45 |             TreeNodeView { id: step.id.clone(), label, children }
   |                                       ^^^^^^^^ help: remove this
   |
note: this value is dropped without further use
  --> ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️steps/🦀️.rs:45:32
   |
45 |             TreeNodeView { id: step.id.clone(), label, children }
   |                                ^^^^^^^
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone


## clippy::empty_line_after_doc_comments — empty line after doc comment

error: empty line after doc comment
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../🦀️.rs:60:1
   |
60 | / /// pair of regenerated content-addressed child handles, exactly like every other composed plugin.
61 | |
   | |_^
62 |   //#region 🔖️ChildTypes
63 |   pub type FormsStructureChild = store::ArtifactChild<SemioValueSnapshot>;
   |   ---------------------------- the comment documents this type alias
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_doc_comments
   = note: `-D clippy::empty-line-after-doc-comments` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::empty_line_after_doc_comments)]`
   = help: if the empty line is unintentional, remove it
help: if the doc comment should not document type alias `FormsStructureChild` then comment it out
   |
39 ~ // /// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): the document's
40 ~ // /// `steps: Vec<FormStep>` tree (each step's id-keyed `blocks`, each block a `FormQuestion` with
41 ~ // /// 15+ optional config fields plus a recursive `condition` expression tree) is no longer an inline
42 ~ // /// `FormsSnapshot` field — it composes stdio's `s.stdio.semio.value`/`table` subsets as two fixed
43 ~ // /// child slots (`structure`/`results`). `structure` (`value`) is the LOSSLESS source of truth: the
44 ~ // /// full step/block tree folded into one structured `SemioValue::Map`, honestly reflecting that a
45 ~ // /// form question's config (`default`/`params`/`condition`/`options`/`fields`) is exactly
46 ~ // /// "structured/computed values," not prose or a flat table. `results` (`table`) is a DERIVED,
47 ~ // /// non-reconstructive projection — one row per block, flattened in step order (`id`/`stepId`/
48 ~ // /// `label`/`kind`/`required`) — for tabular scan/display convenience; it is always regenerated
49 ~ // /// alongside `structure` from the SAME steps (never an independent source), so the two never
50 ~ // /// diverge. Reconstruction (`forms_steps_from_structure`) reads `structure` only.
51 ~ // ///
52 ~ // /// Per this ticket's own corrected precedent (norm/mathematical round 2): composing these two
53 ~ // /// children does NOT regress this plugin's already-granular per-field mutation triads
54 ~ // /// (`create-step`/`delete-step`/`reorder-step`/`rename-step`/`change-step-description`/
55 ~ // /// `create-block`/`delete-block`/`move-block-to-step`/`replace-block`/`change-form-title`) into a
56 ~ // /// whole-blob replace. Every triad's mutation PAYLOAD shape is untouched; `FormsStepsDelta`/
57 ~ // /// `FormsStepPatch` (`🔺️diff/🦀️.rs`) stay the id-keyed sparse delta types they always
58 ~ // /// were, applied via `apply_steps_delta` against the WORKING-SCENE steps (`forms_steps`, not a
59 ~ // /// snapshot field) — only the diff's own OUTER wire representation of "what changed" becomes a
60 ~ // /// pair of regenerated content-addressed child handles, exactly like every other composed plugin.
   |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../🦀️.rs:151:50
    |
151 | ...r::Const { value: semio_value_map_get(value, "value").map(dsl_from_semio_value).unwrap_or(dsl::DslValue::Null) }),
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
    = note: `-D clippy::map-unwrap-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_unwrap_or)]`
help: use `map_or(<a>, <f>)` instead
    |
151 -         "const" => Some(FormExpr::Const { value: semio_value_map_get(value, "value").map(dsl_from_semio_value).unwrap_or(dsl::DslValue::Null) }),
151 +         "const" => Some(FormExpr::Const { value: semio_value_map_get(value, "value").map_or(dsl::DslValue::Null, dsl_from_semio_value) }),
    |


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../🦀️.rs:425:109
    |
425 | ..., title: Option<String>, steps: Vec<FormStep>) -> FormsSnapshot {
    |                                    ^^^^^^^^^^^^^ help: consider changing the type to: `&[FormStep]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`


## clippy::type_complexity — very complex type used. Consider factoring parts into `type` definitions

error: very complex type used. Consider factoring parts into `type` definitions
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../🦀️.rs:468:15
    |
468 |     let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity
    = note: `-D clippy::type-complexity` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::type_complexity)]`


## clippy::needless_update — struct update has no effect, all the fields in the struct have already been specified

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:56:168
   |
56 | ...ucture: snapshot.structure, results: snapshot.results, ..Self::default() }
   |                                                             ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs:85:28
   |
85 | fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> FormsTopology {
   |                            ^^^^^^^^^^^ help: consider changing the type to: `&[String]`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs:85:48
   |
85 | fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> FormsTopology {
   |                                                ^^^^^^^^^^^^^^^^^^^^^ help: consider changing the type to: `&[(String, String)]`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


## clippy::manual_is_multiple_of — manual implementation of `.is_multiple_of()`

error: manual implementation of `.is_multiple_of()`
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs:24:8
   |
24 |     if s.len() % 2 != 0 {
   |        ^^^^^^^^^^^^^^^^ help: replace with: `!s.len().is_multiple_of(2)`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_is_multiple_of
   = note: `-D clippy::manual-is-multiple-of` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::manual_is_multiple_of)]`


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs:61:37
   |
61 | pub fn forms_diff_from_delta(delta: FormsStepsDelta, base: &FormsSnapshot) -> FormsDiff {
   |                                     ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
   |
61 | pub fn forms_diff_from_delta(delta: &FormsStepsDelta, base: &FormsSnapshot) -> FormsDiff {
   |                                     +


## clippy::needless_borrow — this expression creates a reference which is immediately dereferenced by the compiler

error: this expression creates a reference which is immediately dereferenced by the compiler
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:148:30
    |
148 |                 raw.push_str(&chunk);
    |                              ^^^^^^ help: change this to: `chunk`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow
    = note: `-D clippy::needless-borrow` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_borrow)]`


## clippy::unnecessary_lazy_evaluations — unnecessary closure used with `bool::then`

error: unnecessary closure used with `bool::then`
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:211:5
    |
211 |     (payload.question_kind == kind).then(|| QuestionKindRoute { app_id: payload.app_id, params_body_key: payload.params_body_key, preview_body_key: payload.preview_body_key })
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_lazy_evaluations
    = note: `-D clippy::unnecessary-lazy-evaluations` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_lazy_evaluations)]`
help: use `then_some` instead
    |
211 -     (payload.question_kind == kind).then(|| QuestionKindRoute { app_id: payload.app_id, params_body_key: payload.params_body_key, preview_body_key: payload.preview_body_key })
211 +     (payload.question_kind == kind).then_some(QuestionKindRoute { app_id: payload.app_id, params_body_key: payload.params_body_key, preview_body_key: payload.preview_body_key })
    |


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Option`

error: this function's return value is unnecessarily wrapped by `Option`
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:414:1
    |
414 | fn forms_bounded_extent(_command: &FormsCommand, _snapshot: &FormsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
    = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Option` from the return type...
    |
414 - fn forms_bounded_extent(_command: &FormsCommand, _snapshot: &FormsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
414 + fn forms_bounded_extent(_command: &FormsCommand, _snapshot: &FormsSnapshot, _interaction: &protocol::InteractionState) -> usize {
    |
help: ...and then remove the surrounding `Some()` from returning expressions
    |
415 -     Some(1)
415 +     1
    |


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:418:1
    |
418 | / fn forms_retained_reduce(
419 | |     command: &FormsCommand,
420 | |     snapshot: &FormsSnapshot,
421 | |     config: &FormsConfig,
...   |
426 | |     operation: &AppOperationContext,
427 | | ) -> Result<Emit<FormMutation, FormsConfigMutation, NoDraftMutation>, Fault> {
    | |____________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::len_without_is_empty — struct `FormsTryValues` has a public `len` method, but no `is_empty` method

error: struct `FormsTryValues` has a public `len` method, but no `is_empty` method
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:215:5
    |
215 |     pub fn len(&self) -> usize {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#len_without_is_empty
    = note: `-D clippy::len-without-is-empty` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::len_without_is_empty)]`


## clippy::type_complexity — very complex type used. Consider factoring parts into `type` definitions

error: very complex type used. Consider factoring parts into `type` definitions
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:173:38
    |
173 | static ACTIVE_TRY_VALUE_GENERATIONS: OnceLock<Mutex<BTreeMap<(String, String, String), u64>>> = OnceLock::new();
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity


## clippy::result_large_err — the `Err`-variant returned from this function is very large

error: the `Err`-variant returned from this function is very large
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:209:63
    |
209 | fn put_session(key: FormsJobKey, session: TryValueSession) -> Result<(), TryValueSession> {
    |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `Err`-variant is at least 384 bytes
    |
    = help: try reducing the size of `editor::forms::commands::set_try_value::TryValueSession`, for example by boxing large elements or replacing it with `Box<editor::forms::commands::set_try_value::TryValueSession>`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err
    = note: `-D clippy::result-large-err` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::result_large_err)]`


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
   --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:309:30
    |
309 |                 local_end += chunk[local_start..].chars().next().map(char::len_utf8).unwrap_or(1);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
    |
309 -                 local_end += chunk[local_start..].chars().next().map(char::len_utf8).unwrap_or(1);
309 +                 local_end += chunk[local_start..].chars().next().map_or(1, char::len_utf8);
    |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on an `Option` value

error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
    --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:1142:32
     |
1142 |         next = end.min(start + source[start..].chars().next().map(char::len_utf8).unwrap_or(1));
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
     |
1142 -         next = end.min(start + source[start..].chars().next().map(char::len_utf8).unwrap_or(1));
1142 +         next = end.min(start + source[start..].chars().next().map_or(1, char::len_utf8));
     |


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
    --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:1155:45
     |
1155 | fn continuation_emit(generation: u64, next: SetTryValueStep) -> Emit<FormMutation, FormsConfigMutation> {
     |                                             ^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
     |
1155 | fn continuation_emit(generation: u64, next: &SetTryValueStep) -> Emit<FormMutation, FormsConfigMutation> {
     |                                             +


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
    --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:1170:47
     |
1170 | fn finish_try_value(generation: u64, session: TryValueSession, mutations: Vec<FormsConfigMutation>) -> Emit<FormMutation, FormsCon...
     |                                               ^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
help: consider taking a reference instead
     |
1170 | fn finish_try_value(generation: u64, session: &TryValueSession, mutations: Vec<FormsConfigMutation>) -> Emit<FormMutation, FormsConfigMutation> {
     |                                               +


## clippy::obfuscated_if_else — this method chain can be written more clearly with `if .. else ..`

error: this method chain can be written more clearly with `if .. else ..`
    --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:1354:25
     |
1354 |                   source: current_content_id
     |  _________________________^
1355 | |                     .is_none()
1356 | |                     .then(|| {
1357 | |                         let fallback = match &rewrite {
...    |
1363 | |                     })
1364 | |                     .unwrap_or_default(),
     | |________________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#obfuscated_if_else
     = note: `-D clippy::obfuscated-if-else` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::obfuscated_if_else)]`
help: try
     |
1354 ~                 source: if current_content_id
1355 +                     .is_none() { {
1356 +                         let fallback = match &rewrite {
1357 +                             TryValueRewrite::Vector(_) => "null",
1358 +                             TryValueRewrite::Container(ContainerRewrite { edit: ContainerEdit::Option { .. }, .. }) => "[]",
1359 +                             TryValueRewrite::Container(ContainerRewrite { edit: ContainerEdit::Object { .. }, .. }) => "{}",
1360 +                         };
1361 +                         ChunkedSource::from_text(fallback.into())
1362 ~                     } } else { Default::default() },
     |


## clippy::map_unwrap_or — called `map(<f>).unwrap_or_else(<g>)` on an `Option` value

error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
    --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:1425:17
     |
1425 |     let chunk = payload.value_json.as_ref().map(ChunkAddressableJson::owner).unwrap_or_else(|| std::sync::Arc::from("false"));
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `payload.value_json.as_ref().map_or_else(|| std::sync::Arc::from("false"), ChunkAddressableJson::owner)`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or


## clippy::type_complexity — very complex type used. Consider factoring parts into `type` definitions

error: very complex type used. Consider factoring parts into `type` definitions
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-try-values/🦀️.rs:17:33
   |
17 | static ACTIVE_BULK_GENERATIONS: OnceLock<Mutex<BTreeMap<(String, String, String), (u64, String)>>> = OnceLock::new();
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity


## clippy::type_complexity — very complex type used. Consider factoring parts into `type` definitions

error: very complex type used. Consider factoring parts into `type` definitions
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-try-values/🦀️.rs:75:33
   |
75 | fn active_bulk_generations() -> &'static Mutex<BTreeMap<(String, String, String), (u64, String)>> {
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/▶️try/🦀️.rs:51:57
   |
51 | fn read_only_field(question: &FormQuestion, value_text: String) -> UiAssemblyResult<ui::BuiltNode> {
   |                                                         ^^^^^^ help: consider changing the type to: `&str`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value


## E0433 — cannot find `NoTransientStoreDisposer` in `semio_framework_plugin`

error[E0433]: cannot find `NoTransientStoreDisposer` in `semio_framework_plugin`
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1816:47
     |
1816 |         Some(Box::new(semio_framework_plugin::NoTransientStoreDisposer::new()))
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `NoTransientStoreDisposer` in `semio_framework_plugin`


## E0433 — cannot find `NoTransientStoreDisposer` in `semio_framework_plugin`

error[E0433]: cannot find `NoTransientStoreDisposer` in `semio_framework_plugin`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:82:47
   |
82 |         Some(Box::new(semio_framework_plugin::NoTransientStoreDisposer::new()))
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `NoTransientStoreDisposer` in `semio_framework_plugin`


## E0425 — cannot find type `FlowMainWindowConfig` in this scope

error[E0425]: cannot find type `FlowMainWindowConfig` in this scope
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:25:57
   |
25 | pub fn evaluate_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, N...
   |                                                         ^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
 3 + use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
   |


## E0425 — cannot find type `FlowMainWindowConfig` in this scope

error[E0425]: cannot find type `FlowMainWindowConfig` in this scope
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏁️flow-eval-resolve/🦀️.rs:25:57
   |
25 | pub fn evaluate_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, N...
   |                                                         ^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
 3 + use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
   |


## E0425 — cannot find type `FlowMainWindowConfig` in this scope

error[E0425]: cannot find type `FlowMainWindowConfig` in this scope
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:25:57
   |
25 | pub fn evaluate_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, N...
   |                                                         ^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
 3 + use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
   |


## E0425 — cannot find type `FlowMainWindowConfig` in this scope

error[E0425]: cannot find type `FlowMainWindowConfig` in this scope
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:30:60
   |
30 | fn node_graph_edit_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, operations: &[FlowNodeGr...
   |                                                            ^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
 3 + use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
   |


## E0425 — cannot find type `FlowMainWindowConfig` in this scope

error[E0425]: cannot find type `FlowMainWindowConfig` in this scope
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️spotlight-commit/🦀️.rs:30:60
   |
30 | fn node_graph_edit_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, operations: &[FlowNodeGr...
   |                                                            ^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
 3 + use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
   |


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1500:123
     |
1500 | ...w, &ConfigView { snapshot: &payload.config, window: None }, session),
     |                               ^^^^^^^^^^^^^^^ expected `&NoConfig`, found `&Arc<FlowMainWindowConfig>`
     |
     = note: expected reference `&semio_framework_plugin::NoConfig`
                found reference `&std::sync::Arc<editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig>`


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1501:129
     |
1501 | ...w, &ConfigView { snapshot: &payload.config, window: None }, session),
     |                               ^^^^^^^^^^^^^^^ expected `&NoConfig`, found `&Arc<FlowMainWindowConfig>`
     |
     = note: expected reference `&semio_framework_plugin::NoConfig`
                found reference `&std::sync::Arc<editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig>`


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connect-media-ports/🦀️.rs:20:54
     |
  20 |     Ok(Emit::mutations(host_operations(doc.snapshot, cfg.snapshot, session, |host| host.connect_ports(&payload.source_node_id, &pa...
     |                        ---------------               ^^^^^^^^^^^^ expected `&FlowMainWindowConfig`, found `&NoConfig`
     |                        |
     |                        arguments to this function are incorrect
     |
     = note: expected reference `&editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig`
                found reference `&semio_framework_plugin::NoConfig`
note: function defined here
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2054:8
     |
2054 | pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut...
     |        ^^^^^^^^^^^^^^^                          -----------------------------


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs:31:52
     |
  31 |     let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| {
     |                      ---------------               ^^^^^^^^^^^^ expected `&FlowMainWindowConfig`, found `&NoConfig`
     |                      |
     |                      arguments to this function are incorrect
     |
     = note: expected reference `&editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig`
                found reference `&semio_framework_plugin::NoConfig`
note: function defined here
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2054:8
     |
2054 | pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut...
     |        ^^^^^^^^^^^^^^^                          -----------------------------


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️disconnect/🦀️.rs:17:54
     |
  17 |     Ok(Emit::mutations(host_operations(doc.snapshot, cfg.snapshot, session, |host| host.disconnect(&payload.synapse_id).is_ok())))
     |                        ---------------               ^^^^^^^^^^^^ expected `&FlowMainWindowConfig`, found `&NoConfig`
     |                        |
     |                        arguments to this function are incorrect
     |
     = note: expected reference `&editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig`
                found reference `&semio_framework_plugin::NoConfig`
note: function defined here
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2054:8
     |
2054 | pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut...
     |        ^^^^^^^^^^^^^^^                          -----------------------------


## E0599 — no variant named `SetDuplicateWidgetProgress` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetDuplicateWidgetProgress` found for enum `semio_framework_plugin::NoConfigMutation`
   --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:182:56
    |
182 | ...[NoConfigMutation::SetDuplicateWidgetProgress { json }], effects: vec![queue(&step)], ui_scope: UiDirtyScope::Full, ..Default::d...
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0599 — no variant named `SetDuplicateWidgetProgress` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetDuplicateWidgetProgress` found for enum `semio_framework_plugin::NoConfigMutation`
   --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:197:50
    |
197 |         config_mutations: vec![NoConfigMutation::SetDuplicateWidgetProgress { json: String::new() }],
    |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0609 — no field `duplicate_widget_progress_json` on type `&semio_framework_plugin::NoConfig`

error[E0609]: no field `duplicate_widget_progress_json` on type `&semio_framework_plugin::NoConfig`
   --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:205:21
    |
205 |     if cfg.snapshot.duplicate_widget_progress_json.len() > MAX_CHECKPOINT_BYTES {
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unknown field


## E0609 — no field `duplicate_widget_progress_json` on type `&semio_framework_plugin::NoConfig`

error[E0609]: no field `duplicate_widget_progress_json` on type `&semio_framework_plugin::NoConfig`
   --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:220:79
    |
220 |     if dsl::os_pack::json::from_json_str::<DuplicateWidgetStep>(&cfg.snapshot.duplicate_widget_progress_json).ok().as_ref() != Some...
    |                                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unknown field


## E0599 — no variant named `CancelDuplicateWidget` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `CancelDuplicateWidget` found for enum `semio_framework_plugin::NoConfigMutation`
   --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:230:67
    |
230 | ...c![NoConfigMutation::CancelDuplicateWidget { generation: payload.generation }], ..Default::default() });
    |                         ^^^^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0599 — no variant named `SetDuplicateWidgetProgress` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetDuplicateWidgetProgress` found for enum `semio_framework_plugin::NoConfigMutation`
   --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:237:85
    |
237 | ...[NoConfigMutation::SetDuplicateWidgetProgress { json: String::new() }], ..Default::default() }),
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0609 — no field `duplicate_widget_progress_json` on type `&semio_framework_plugin::NoConfig`

error[E0609]: no field `duplicate_widget_progress_json` on type `&semio_framework_plugin::NoConfig`
   --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:263:67
    |
263 |     if let Some(generation) = checkpoint_generation(&cfg.snapshot.duplicate_widget_progress_json) {
    |                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unknown field


## E0599 — no variant named `CancelDuplicateWidget` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `CancelDuplicateWidget` found for enum `semio_framework_plugin::NoConfigMutation`
   --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:264:59
    |
264 |         emit.config_mutations.insert(0, NoConfigMutation::CancelDuplicateWidget { generation });
    |                                                           ^^^^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:48:53
     |
  48 |     let mut host = host_from_snapshot(doc.snapshot, cfg.snapshot, session);
     |                    ------------------               ^^^^^^^^^^^^ expected `&FlowMainWindowConfig`, found `&NoConfig`
     |                    |
     |                    arguments to this function are incorrect
     |
     = note: expected reference `&editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig`
                found reference `&semio_framework_plugin::NoConfig`
note: function defined here
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2045:8
     |
2045 | pub fn host_from_snapshot(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> FlowHost {
     |        ^^^^^^^^^^^^^^^^^^                         -----------------------------


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs:27:48
     |
  27 |     match focus_selection_camera(doc.snapshot, cfg.snapshot, session, &nodes) {
     |           ----------------------               ^^^^^^^^^^^^ expected `&FlowMainWindowConfig`, found `&NoConfig`
     |           |
     |           arguments to this function are incorrect
     |
     = note: expected reference `&editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig`
                found reference `&semio_framework_plugin::NoConfig`
note: function defined here
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2080:8
     |
2080 | pub fn focus_selection_camera(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, selected_node_ids:...
     |        ^^^^^^^^^^^^^^^^^^^^^^                         -----------------------------


## E0599 — no variant named `SetCamera` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetCamera` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs:28:64
   |
28 |         Some(camera) => Ok(Emit::config(vec![NoConfigMutation::SetCamera { camera }])),
   |                                                                ^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-media-node/🦀️.rs:19:52
     |
  19 |     let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| {
     |                      ---------------               ^^^^^^^^^^^^ expected `&FlowMainWindowConfig`, found `&NoConfig`
     |                      |
     |                      arguments to this function are incorrect
     |
     = note: expected reference `&editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig`
                found reference `&semio_framework_plugin::NoConfig`
note: function defined here
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2054:8
     |
2054 | pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut...
     |        ^^^^^^^^^^^^^^^                          -----------------------------


## E0599 — no variant named `SetCamera` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetCamera` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs:18:44
   |
18 |     Ok(Emit::config(vec![NoConfigMutation::SetCamera { camera: payload.camera.clone() }]))
   |                                            ^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-widget/🦀️.rs:22:52
     |
  22 |     let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| host.remove_widget(target_id).is_ok());
     |                      ---------------               ^^^^^^^^^^^^ expected `&FlowMainWindowConfig`, found `&NoConfig`
     |                      |
     |                      arguments to this function are incorrect
     |
     = note: expected reference `&editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig`
                found reference `&semio_framework_plugin::NoConfig`
note: function defined here
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2054:8
     |
2054 | pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut...
     |        ^^^^^^^^^^^^^^^                          -----------------------------


## E0308 — mismatched types

error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🦀️.rs:19:35
     |
  19 |     host_operations(doc.snapshot, cfg.snapshot, session, |host| host.reorganize(REORGANIZE_OPTIONS_JSON).is_ok())
     |     ---------------               ^^^^^^^^^^^^ expected `&FlowMainWindowConfig`, found `&NoConfig`
     |     |
     |     arguments to this function are incorrect
     |
     = note: expected reference `&editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig`
                found reference `&semio_framework_plugin::NoConfig`
note: function defined here
    --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2054:8
     |
2054 | pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut...
     |        ^^^^^^^^^^^^^^^                          -----------------------------


## E0599 — no method named `automation_enabled` found for reference `&semio_framework_plugin::NoConfig` in the current scope

error[E0599]: no method named `automation_enabled` found for reference `&semio_framework_plugin::NoConfig` in the current scope
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-extension-action/🦀️.rs:38:22
   |
38 |     if !cfg.snapshot.automation_enabled().get(*id).copied().unwrap_or(false) {
   |                      ^^^^^^^^^^^^^^^^^^ method not found in `&semio_framework_plugin::NoConfig`


## E0599 — no variant named `SetCatalogueSections` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetCatalogueSections` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-catalogue-sections/🦀️.rs:16:44
   |
16 |     Ok(Emit::config(vec![NoConfigMutation::SetCatalogueSections { sections_json: payload.sections_json.clone() }]))
   |                                            ^^^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0599 — no variant named `SetGridFactor` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetGridFactor` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️set-grid-factor/🦀️.rs:18:44
   |
18 |     Ok(Emit::config(vec![NoConfigMutation::SetGridFactor { value: payload.value.clamp(0.5, 50.0) }]))
   |                                            ^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0599 — no variant named `SetGridSnapEnabled` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetGridSnapEnabled` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs:16:44
   |
16 |     Ok(Emit::config(vec![NoConfigMutation::SetGridSnapEnabled { value: payload.pressed.unwrap_or(!cfg.snapshot.grid_snap_enabled) }]))
   |                                            ^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0609 — no field `grid_snap_enabled` on type `&semio_framework_plugin::NoConfig`

error[E0609]: no field `grid_snap_enabled` on type `&semio_framework_plugin::NoConfig`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs:16:112
   |
16 |     Ok(Emit::config(vec![NoConfigMutation::SetGridSnapEnabled { value: payload.pressed.unwrap_or(!cfg.snapshot.grid_snap_enabled) }]))
   |                                                                                                                ^^^^^^^^^^^^^^^^^ unknown field


## E0599 — no variant named `SetGridVisible` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetGridVisible` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs:16:44
   |
16 |     Ok(Emit::config(vec![NoConfigMutation::SetGridVisible { value: payload.pressed.unwrap_or(!cfg.snapshot.grid_visible) }]))
   |                                            ^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0609 — no field `grid_visible` on type `&semio_framework_plugin::NoConfig`

error[E0609]: no field `grid_visible` on type `&semio_framework_plugin::NoConfig`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs:16:108
   |
16 |     Ok(Emit::config(vec![NoConfigMutation::SetGridVisible { value: payload.pressed.unwrap_or(!cfg.snapshot.grid_visible) }]))
   |                                                                                                            ^^^^^^^^^^^^ unknown field


## E0599 — no variant named `SetLodMode` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetLodMode` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️set-lod-mode/🦀️.rs:20:48
   |
20 |         Ok(Emit::config(vec![NoConfigMutation::SetLodMode { value: payload.value.clone() }]))
   |                                                ^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0609 — no field `preview_off_node_ids` on type `&semio_framework_plugin::NoConfig`

error[E0609]: no field `preview_off_node_ids` on type `&semio_framework_plugin::NoConfig`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🙈️set-preview-off/🦀️.rs:17:33
   |
17 |     let mut next = cfg.snapshot.preview_off_node_ids.clone();
   |                                 ^^^^^^^^^^^^^^^^^^^^ unknown field


## E0599 — no variant named `SetPreviewOff` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetPreviewOff` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🙈️set-preview-off/🦀️.rs:27:44
   |
27 |     Ok(Emit::config(vec![NoConfigMutation::SetPreviewOff { node_ids: next }]))
   |                                            ^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0599 — no variant named `SetProximityDistance` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetProximityDistance` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️set-proximity-distance/🦀️.rs:16:44
   |
16 |     Ok(Emit::config(vec![NoConfigMutation::SetProximityDistance { value: payload.value.max(0.0) }]))
   |                                            ^^^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## E0599 — no method named `automation_enabled` found for reference `&semio_framework_plugin::NoConfig` in the current scope

error[E0599]: no method named `automation_enabled` found for reference `&semio_framework_plugin::NoConfig` in the current scope
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🦀️.rs:17:32
   |
17 |     let mut map = cfg.snapshot.automation_enabled();
   |                                ^^^^^^^^^^^^^^^^^^ method not found in `&semio_framework_plugin::NoConfig`


## E0599 — no variant named `SetAutomationEnabled` found for enum `semio_framework_plugin::NoConfigMutation`

error[E0599]: no variant named `SetAutomationEnabled` found for enum `semio_framework_plugin::NoConfigMutation`
  --> ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🦀️.rs:19:44
   |
19 |     Ok(Emit::config(vec![NoConfigMutation::SetAutomationEnabled { json: serde_json::to_string(&map).unwrap_or_default() }]))
   |                                            ^^^^^^^^^^^^^^^^^^^^ variant not found in `semio_framework_plugin::NoConfigMutation`


## unfulfilled_lint_expectations — this lint expectation is unfulfilled

error: this lint expectation is unfulfilled
  --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️retry-stage/🦀️.rs:18:10
   |
18 | #[expect(clippy::unnecessary_wraps, reason = "app_commands! requires every concrete handler to return Result<Emit, Fault>.")]
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: app_commands! requires every concrete handler to return Result<Emit, Fault>.
   = note: `-D unfulfilled-lint-expectations` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unfulfilled_lint_expectations)]`


## unfulfilled_lint_expectations — this lint expectation is unfulfilled

error: this lint expectation is unfulfilled
    --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:1018:10
     |
1018 | #[expect(clippy::unnecessary_wraps, reason = "app_commands! requires every concrete handler to return Result<Emit, Fault>.")]
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: app_commands! requires every concrete handler to return Result<Emit, Fault>.


## unfulfilled_lint_expectations — this lint expectation is unfulfilled

error: this lint expectation is unfulfilled
  --> ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-stage/🦀️.rs:18:10
   |
18 | #[expect(clippy::unnecessary_wraps, reason = "app_commands! requires every concrete handler to return Result<Emit, Fault>.")]
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: app_commands! requires every concrete handler to return Result<Emit, Fault>.


## E0277 — the `?` operator can only be applied to values that implement `std::ops::Try`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
    --> ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:4517:26
     |
4517 |             return match preflight.close_step(maximum_bytes)? {
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `dsl::SnapshotRetirementStep`
     |
     = help: the nightly-only, unstable trait `std::ops::Try` is not implemented for `dsl::SnapshotRetirementStep`


## unused_variables — unused variable: `base`

error: unused variable: `base`
  --> ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️change-camera/🦀️.rs:19:20
   |
19 |     fn diff(&self, base: &DagConfig) -> protocol::MutationOutcome<DagConfig> {
   |                    ^^^^ help: if this is intentional, prefix it with an underscore: `_base`
   |
   = note: `-D unused-variables` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_variables)]`


## unused_qualifications — unnecessary qualification

error: unnecessary qualification
   --> ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🦀️.rs:107:78
    |
107 | ...nt { work_items: 1, retained_bytes: std::mem::size_of::<Block3dWorldWindowTransientMutation>() })
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unused-qualifications` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(unused_qualifications)]`
help: remove the unnecessary path segments
    |
107 -     Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: std::mem::size_of::<Block3dWorldWindowTransientMutation>() })
107 +     Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: size_of::<Block3dWorldWindowTransientMutation>() })
    |


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../🦀️.rs:149:133
    |
149 | ...rtexKindExtra>, kinds: Vec<Block3dVortexKind>) {
    |                           ^^^^^^^^^^^^^^^^^^^^^^ help: consider changing the type to: `&[Block3dVortexKind]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:294:1
    |
294 | / fn block3d_retained_reduce(
295 | |     command: &Block3dCommand,
296 | |     snapshot: &Block3dSnapshot,
297 | |     config: &Block3dConfig,
...   |
302 | |     operation: &AppOperationContext,
303 | | ) -> Result<Emit<Block3dMutation, Block3dConfigMutation, NoDraftMutation>, Fault> {
    | |_________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
   --> ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🦀️.rs:106:1
    |
106 | fn preview_footprint(_: &Block3dWorldWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
    = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
    |
106 - fn preview_footprint(_: &Block3dWorldWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
106 + fn preview_footprint(_: &Block3dWorldWindowTransientMutation) -> dsl::ArtifactStoreOneItemFootprint {
    |
help: ...and then remove the surrounding `Ok()` from returning expressions
    |
107 -     Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: std::mem::size_of::<Block3dWorldWindowTransientMutation>() })
107 +     store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: std::mem::size_of::<Block3dWorldWindowTransientMutation>() }
    |


## unused_qualifications — unnecessary qualification

error: unnecessary qualification
   --> ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:143:26
    |
143 |     let retained_bytes = std::mem::size_of::<NoteCompositeWindowTransient>().checked_add(transient.engagement_input.len()).ok_or_el...
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unused-qualifications` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(unused_qualifications)]`
help: remove the unnecessary path segments
    |
143 -     let retained_bytes = std::mem::size_of::<NoteCompositeWindowTransient>().checked_add(transient.engagement_input.len()).ok_or_else(|| "Note composite window transient footprint overflowed".to_string())?;
143 +     let retained_bytes = size_of::<NoteCompositeWindowTransient>().checked_add(transient.engagement_input.len()).ok_or_else(|| "Note composite window transient footprint overflowed".to_string())?;
    |


## dead_code — associated function `default_ui` is never used

error: associated function `default_ui` is never used
  --> ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:97:8
   |
55 | impl NoteArtifact {
   | ----------------- associated function in this implementation
...
97 |     fn default_ui() -> Self {
   |        ^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
   = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`


## clippy::derivable_impls — this `impl` can be derived

error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:14:1
   |
14 | / impl Default for NoteCompositeWindowConfig {
15 | |     fn default() -> Self {
16 | |         Self { camera: NoteCamera::default() }
17 | |     }
18 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
10 + #[derive(Default)]
11 | pub struct NoteCompositeWindowConfig {
   |


## clippy::manual_range_patterns — this OR pattern can be rewritten using a range

error: this OR pattern can be rewritten using a range
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:236:23
    |
236 |             [2, edge, 0 | 1 | 2] => {
    |                       ^^^^^^^^^ help: try: `0..=2`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_range_patterns
    = note: `-D clippy::manual-range-patterns` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::manual_range_patterns)]`


## clippy::manual_range_patterns — this OR pattern can be rewritten using a range

error: this OR pattern can be rewritten using a range
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:240:31
    |
240 |             [2, edge, slot @ (0 | 1 | 2), 0] => {
    |                               ^^^^^^^^^ help: try: `0..=2`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_range_patterns


## clippy::manual_range_patterns — this OR pattern can be rewritten using a range

error: this OR pattern can be rewritten using a range
   --> ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:259:32
    |
259 |             [1, _, 0] | [2, _, 0 | 1 | 2] => &["value"],
    |                                ^^^^^^^^^ help: try: `0..=2`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_range_patterns


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:2871:5
     |
2871 |     fn close_json_cursor(cursor: &mut Option<JsonValidationCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage) -> Result<PluginCloseStep, Fault> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
     = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
     |
2871 -     fn close_json_cursor(cursor: &mut Option<JsonValidationCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage) -> Result<PluginCloseStep, Fault> {
2871 +     fn close_json_cursor(cursor: &mut Option<JsonValidationCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage) -> semio_framework_plugin::PluginCloseStep {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
2874 ~                 return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
2875 |             }
 ...
2879 |         *stage = next;
2880 ~         PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
     |


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:2883:5
     |
2883 |     fn close_typed_cursor(cursor: &mut Option<TypedJsonCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
2883 -     fn close_typed_cursor(cursor: &mut Option<TypedJsonCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
2883 +     fn close_typed_cursor(cursor: &mut Option<TypedJsonCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> semio_framework_plugin::PluginCloseStep {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
2889 ~                     return PluginCloseStep::Pending { released_items: usize::from(released != 0), released_bytes: released };
2890 |                 }
2891 |                 drop(cursor.stack.pop());
2892 ~                 return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
2893 |             }
 ...
2897 |         *stage = next;
2898 ~         PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
     |


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:2901:5
     |
2901 |     fn close_optional_string(value: &mut Option<String>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
2901 -     fn close_optional_string(value: &mut Option<String>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
2901 +     fn close_optional_string(value: &mut Option<String>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> semio_framework_plugin::PluginCloseStep {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
2904 ~                 return PluginCloseStep::Pending { released_items: usize::from(released != 0), released_bytes: released };
2905 |             }
 ...
2909 |         *stage = next;
2910 ~         PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
     |


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
    --> ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:2913:5
     |
2913 |     fn close_required_string(value: &mut String, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
2913 -     fn close_required_string(value: &mut String, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
2913 +     fn close_required_string(value: &mut String, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> semio_framework_plugin::PluginCloseStep {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
2915 ~             return PluginCloseStep::Pending { released_items: usize::from(released != 0), released_bytes: released };
2916 |         }
 ...
2919 |         *stage = next;
2920 ~         PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
     |


## dead_code — associated function `default_ui` is never used

error: associated function `default_ui` is never used
  --> ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:75:8
   |
64 | impl EquationArtifact {
   | --------------------- associated function in this implementation
...
75 |     fn default_ui() -> Self {
   |        ^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
   = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`


## clippy::unusual_byte_groupings — digits of hex, binary or octal literal not in groups of equal size

error: digits of hex, binary or octal literal not in groups of equal size
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/../../🦀️.rs:360:103
    |
360 |     SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_nanos() as u64).unwrap_or(0xC0FF_EE00_D15E_A5E)
    |                                                                                                       ^^^^^^^^^^^^^^^^^^^^ help: consider: `0x0C0F_FEE0_0D15_EA5E`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unusual_byte_groupings
    = note: `-D clippy::unusual-byte-groupings` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unusual_byte_groupings)]`


## clippy::map_unwrap_or — called `map(<f>).unwrap_or(<a>)` on a `Result` value

error: called `map(<f>).unwrap_or(<a>)` on a `Result` value
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/../../🦀️.rs:360:5
    |
360 |     SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_nanos() as u64).unwrap_or(0xC0FF_EE00_D15E_A5E)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
    = note: `-D clippy::map-unwrap-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::map_unwrap_or)]`
help: use `map_or(<a>, <f>)` instead
    |
360 -     SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_nanos() as u64).unwrap_or(0xC0FF_EE00_D15E_A5E)
360 +     SystemTime::now().duration_since(UNIX_EPOCH).map_or(0xC0FF_EE00_D15E_A5E, |duration| duration.as_nanos() as u64)
    |


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/../../🦀️.rs:492:91
    |
492 |         operator_info("math.add", "Add", "Add", "Adds numbers, points, or vectors", scalar.clone(), sum_output.clone()),
    |                                                                                           ^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/../../🦀️.rs:492:85
    |
492 |         operator_info("math.add", "Add", "Add", "Adds numbers, points, or vectors", scalar.clone(), sum_output.clone()),
    |                                                                                     ^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone
    = note: `-D clippy::redundant-clone` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::redundant_clone)]`


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/../../🦀️.rs:511:115
    |
511 | ...racts numbers, points, or vectors", subtract_scalar.clone(), vec![difference_out()]),
    |                                                       ^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/../../🦀️.rs:511:100
    |
511 | ...   operator_info("math.subtract", "Subtract", "Sub", "Subtracts numbers, points, or vectors", subtract_scalar.clone(), vec![diff...
    |                                                                                                  ^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone


## clippy::redundant_clone — redundant clone

error: redundant clone
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/../../🦀️.rs:569:166
    |
569 | ...annelSpec::list("list", &["math.sum"])], sum_output.clone()), Sum, vec!["list"], &["number"]);
    |                                                       ^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/../../🦀️.rs:569:156
    |
569 | ...ctionary", vec![ChannelSpec::list("list", &["math.sum"])], sum_output.clone()), Sum, vec!["list"], &["number"]);
    |                                                               ^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone


## clippy::needless_borrow — this expression creates a reference which is immediately dereferenced by the compiler

error: this expression creates a reference which is immediately dereferenced by the compiler
  --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/../../🦀️.rs:47:36
   |
47 |         let indices = list_indices(&list);
   |                                    ^^^^^ help: change this to: `list`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow
   = note: `-D clippy::needless-borrow` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_borrow)]`


## clippy::needless_borrow — this expression creates a reference which is immediately dereferenced by the compiler

error: this expression creates a reference which is immediately dereferenced by the compiler
  --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/../../🦀️.rs:88:33
   |
88 |         let next = list_indices(&list).len();
   |                                 ^^^^^ help: change this to: `list`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow


## clippy::needless_borrow — this expression creates a reference which is immediately dereferenced by the compiler

error: this expression creates a reference which is immediately dereferenced by the compiler
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/../../🦀️.rs:101:67
    |
101 |         Ok(channel_output("count", number_dictionary(list_indices(&list).len() as f64)))
    |                                                                   ^^^^^ help: change this to: `list`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow


## clippy::needless_borrow — this expression creates a reference which is immediately dereferenced by the compiler

error: this expression creates a reference which is immediately dereferenced by the compiler
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/../../🦀️.rs:114:53
    |
114 |         Ok(channel_output("list", remove_list_index(&list, index)))
    |                                                     ^^^^^ help: change this to: `list`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow


## clippy::needless_borrow — this expression creates a reference which is immediately dereferenced by the compiler

error: this expression creates a reference which is immediately dereferenced by the compiler
   --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/../../🦀️.rs:144:36
    |
144 |         let indices = list_indices(&list);
    |                                    ^^^^^ help: change this to: `list`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow


## clippy::needless_pass_by_value — this argument is passed by value, but not consumed in the function body

error: this argument is passed by value, but not consumed in the function body
  --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/../../🦀️.rs:10:28
   |
10 | fn map_kernel_error(error: DrawingError) -> EvalError {
   |                            ^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
   = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
   |
10 | fn map_kernel_error(error: &DrawingError) -> EvalError {
   |                            +


## clippy::unnecessary_wraps — this function's return value is unnecessarily wrapped by `Result`

error: this function's return value is unnecessarily wrapped by `Result`
  --> ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/../../🦀️.rs:61:1
   |
61 | fn read_rgba(input: &Dictionary, key: &str) -> Result<[f64; 4], EvalError> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
   = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
   |
61 - fn read_rgba(input: &Dictionary, key: &str) -> Result<[f64; 4], EvalError> {
61 + fn read_rgba(input: &Dictionary, key: &str) -> [f64; 4] {
   |
help: ...and then remove the surrounding `Ok()` from returning expressions
   |
62 ~     [
63 +         read_channel_number(input, &format!("{key}R")).unwrap_or(0.0),
64 +         read_channel_number(input, &format!("{key}G")).unwrap_or(0.0),
65 +         read_channel_number(input, &format!("{key}B")).unwrap_or(0.0),
66 +         read_channel_number(input, &format!("{key}A")).unwrap_or(1.0),
67 +     ]
   |


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../././⚙️engine/🪐️space/🦀️.rs:359:1
    |
359 | / fn space_bounded_reduce(
360 | |     command: &SpaceCommand,
361 | |     snapshot: &WorkflowSnapshot,
362 | |     config: &SpaceConfig,
...   |
367 | |     operation: &AppOperationContext,
368 | | ) -> Result<Emit<WorkflowMutation, SpaceConfigMutation, NoDraftMutation>, Fault> {
    | |________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


