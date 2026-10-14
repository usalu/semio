# Native Jack Ownership and Deletion Audit

Read-only source audit on 2026-10-14. Root, s, and Trinity AGENTS were read. The only written file is this report. No native build/test/runtime was executed; no passing native or deletion result is claimed.

## Required Ownership

The TypeScript JackGraphQuery schema is the correct minimal inference consumer port: selected child identity, literal node IDs, literal edge IDs/source/target, and numeric displacement. Rust needs that same behavioral contract under the existing graph-query taxonomy, with borrowed iteration and a retained query owner. Admission must compare childId, artifactId, artifactKind, standard and subset with the parent's selected handle; the intrinsic port must not decide which dialect is Semio. The concrete binding belongs in specific installation composition alongside `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🟦️.ts`, with its native counterpart and tests.

Do not stop at inference: Jack currently publicly exports the complete content implementation through `pub use content::*`, fixes its parent child type/schema to Semio, fixes editor/viewer Members and aggregate app constraints to SemioMembers, and interprets mutation op bytes in reorganize. Removing those requires explicit consumer behavior ports for inference, query admission/projection, UI graph reads, effect preparation, layout move publication/checkpoint interpretation and child genesis pack provision. Define schema contracts before their Rust implementations. Keep each behavior close to its consumer rather than constructing a replacement structural snapshot of nodes/edges/properties.

The actual rich Semio graph remains owned by its original artifact store with its full clone, pack, mutation, retained-clone and retirement implementations. A bound borrowed/retained view keeps that concrete owner alive and exposes only required behaviors. The composition owns exact Semio selection, numeric lexeme interpretation, six-family query projection refusals and concrete mutation translation. The intrinsic Jack parent retains a neutral typed child-address marker, not SemioGraphSnapshot. Runtime Members composition must be parameterized or provided by the authored specific installation, including registered editor/viewer assembly and VcsArtifactApp conversion; moving imports while leaving a default concrete Members alias is not completion.

Framework evidence: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🦀️.rs` already separates persisted childId/target from serialization-skipped Any local-owner and PhantomData; clone aliases the owner. `…/🔌️plugin/📜️declaration/🪆️child/🦀️.rs` exposes ChildContentView typed_read at 563 and dialect at 575. Existing published reads retain store leases; composition must bind that published read, not reuse parent genesis/local owner as live scene. A live capability registration facility may need framework work: typed_read alone cannot intrinsically discover a Jack behavioral trait from an erased concrete child. Keep framework facilities domain neutral and composition responsible for trait binding.

## Concrete Leak and Caller Taxonomy

`🪆️content/🦀️.rs` is not just an inference utility. Lines 10–15 own/ expose ArtifactChild<SemioGraphSnapshot> and its retained owner. Lines 61–95 interpret Semio values, geometry, ports, labels and endpoint separator conventions. Lines 97–126 derive Semio child identity/pack, construct/materialize owners, encode genesis and attach bundled empty content. Lines 151–172 select exact dialect, typed-read live child, and submit concrete child preparations. Lines 188–242 translate GraphEffect into ordered concrete mutations; node deletion cuts incident edges first; property insertions choose sorted indexes; sequential effects observe previous leaves; working snapshot retirement is explicit. All these behaviors must remain available through authored composition/ports, with actual Semio content retained intact.

Snapshot schema `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` has `#[child(kind = "s.stdio.semio")]` and default construction through concrete content; changing only inference leaves schema and storage coupled. Keep parent JSON/SQLite/binary serialization as address-only content. Full child storage travels independently.

Query preparation `🧬️schema/🧮️executor/🪜️execution/🦀️.rs` computes byte/depth/item admission before projecting one retained node or edge per step (217 onward). Move concrete byte admission/projection into composition-backed behavior, preserve refusal before allocation, bounded pending steps, cancellation, close demand, terminal-empty and result ownership. Do not replace this with an eager whole graph clone or preprojected vector.

UI consumers need names, geometry, ports and edge endpoint/label views separately from inference IDs. Viewer/editor currently intentionally render rich content directly; reading through the lossy six-family query projection would regress InOut ports, rich labels and arbitrary stored values. Reorganize needs IDs/geometry/edges plus composition-owned encoding/decoding of Semio MoveNode op bytes. GraphEffect is Jack-owned; concrete SemioGraphMutation is not a valid public intrinsic return type.

Rust public imports from concrete JSON IO also remain in the format-addressed import/export serializers. The artifact Cargo manifests declare CSV, JSON, MD, PNG, SVG, TXT, Semio, Semio base, graph and value. Full Stdio deletion criteria must cover those even though only JSON and Semio appear in current non-test Rust use sites. Transport dialect selection and serializer/deserializer assembly belong to authored composition; do not leave them as unused Cargo dependencies.

## Native Parity Traps

Current Rust topology uses BTreeMap frontier order plus newly-zero sorting; TypeScript contract must determine the exact agreed sorting/FIFO behavior. Current Rust flat-position edges are BTreeMap sorted by edge ID, whereas the current TS evidence calls for stable retained edge order. Native parity requires consuming the same language-neutral vectors rather than assuming existing algorithm order is equivalent. Rust Int offset currently uses parse::<f64>(), accepting noncanonical numeric forms that the specific TS composition rejects. Keep canonical integer/float/NaN word validation owned by composition, preserve negative zero, infinities, NaN and payload behavior where fixtures require it.

Query projection deliberately refuses Semio Int/Bytes/Ref, duplicate property keys, edge labels, InOut ports, and literal @ endpoint components. These are query capability limitations, not reasons to mutate/normalize the actual owner. Inference must continue accepting those unrelated rich fields.

## Verification and Deletion Gate

Supported existing Nx package: `@semio-tech/trinity-jack-rs`. Its `📦️packages/🦀️rust/📋️project.json` routes check/test through `bun ./📜️script.ts check|test` and graph-generate. Existing targets: `verify-jack-document-contract`, `verify-jack-query-ownership`, `verify-physical-codecs`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `graph-wire-check`. Invoke as `bun nx run @semio-tech/trinity-jack-rs:<target>` through the parent's supported bootstrap if the contended root graph remains unavailable. verify-jack-query-ownership runs its language-neutral TS oracle then native lib filter query_ownership with nocapture; the oracle-only suffix is not native verification. Do not run long builds during this audit.

Extend permanent script/Nx/launch registration for native graph-query and specific composition verification if existing targets cannot express them. Test first using alternate neutral graph owner and exact identity mismatch on all coordinates; full retained clone/pack/child mutation plus UI/query/layout runtime must continue working. Assert original owner identity and rich content survive binding, mutations, encode/decode, cancellation and retirement.

Deletion requires: no Stdio concrete imports, public type/signature leaks, schema fixed kind, Members requirements, Cargo dependency edges, direct fixture imports or serializer routing under intrinsic Jack; independent native and TS intrinsic builds with Stdio unavailable; native specific installation binds real published Semio children and participates in editing/viewing/query/reorganize; full rich owner clone/pack/mutations remain verified, not only inferred topology. Intrinsic tests need neutral owners; Semio integration tests belong to composition. Whole-plugin deletion also includes Rewriting and other dependents and is outside what a Jack-only test can prove.

## Exact Production Native Leak Inventory

Paths relative to the Jack artifact root shown below; lines were read from current files, not a cached copy.

### 🦀️.rs

```rust
9: //! is NOT graph-shaped in the `SemioGraphSnapshot` sense (that subset is an INSTANCE graph: nodes with
596: + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::jack::TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
597: + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::jack::TrinityJackViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
603: + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::jack::TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
604: + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::jack::TrinityJackViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
```

### 🪆️content/🦀️.rs

```rust
5: use semio_s_artifact_stdio_semio_base::schema::geometry::{SemioPoint2, binary64_lexeme, read_binary64_lexeme};
6: use semio_s_artifact_stdio_semio_graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
7: use semio_s_artifact_stdio_semio_value::schema::snapshot::{SemioValue, SemioValueEntry};
10: pub type JackContentChild=store::os_store::artifact_child::ArtifactChild<SemioGraphSnapshot>;
12: pub struct JackContentOwner { snapshot:Option<SemioGraphSnapshot> }
14: pub fn new(snapshot:SemioGraphSnapshot)->Self{Self{snapshot:Some(snapshot)}}
15: pub fn snapshot(&self)->&SemioGraphSnapshot{self.snapshot.as_ref().expect("live Jack Semio child")}
82: pub fn jack_content_snapshot_from_working(nodes:&[Node],edges:&[Edge])->SemioGraphSnapshot{
83: SemioGraphSnapshot{schema:STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(),nodes:nodes.iter().map(|node|SemioGraphNode{id:GraphNodeId::new(node.id.clone()),kind:node.kind.clone(),label:node.name.clone(),position:SemioPoint2{x:node.x,y:node.y},width:node.width,height:node.height,ports:node.ports.iter().map(|port|SemioGraphPort{name:port.id.clone(),kind:match port.direction{PortDirection::In=>SemioGraphPortKind::In,PortDirection::Out=>SemioGraphPortKind::Out},category:port.kind.clone(),properties:properties_to_semio(&port.properties)}).collect(),properties:properties_to_semio(&node.properties)}).collect(),edges:edges.iter().map(|edge|{let(source,source_port)=edge.source.split_once('@').map_or((edge.source.as_str(),None),|(node,port)|(node,Some(port.to_string())));let(target,target_port)=edge.target.split_once('@').map_or((edge.target.as_str(),None),|(node,port)|(node,Some(port.to_string())));SemioGraphEdge{id:GraphEdgeId::new(edge.id.clone()),source:GraphNodeId::new(source),target:GraphNodeId::new(target),kind:edge.kind.clone(),label:String::new(),source_port,target_port,properties:properties_to_semio(&edge.properties)}}).collect()}
86: pub fn working_from_jack_content_snapshot(content:&SemioGraphSnapshot)->Result<(Vec<Node>,Vec<Edge>),ValueError>{
92: pub fn jack_node_from_content(node:&SemioGraphNode)->Result<Node,ValueError>{let ports=node.ports.iter().map(|port|Ok(Port{id:port.name.clone(),kind:port.category.clone(),direction:match port.kind{SemioGraphPortKind::In=>PortDirection::In,SemioGraphPortKind::Out=>PortDirection::Out,SemioGraphPortKind::InOut=>return Err(invalid("Jack query projection requires a single port direction"))},properties:properties_from_semio(&port.properties)?})).collect::<Result<_,ValueError>>()?;Ok(Node{id:node.id.value.clone(),kind:node.kind.clone(),name:node.label.clone(),x:node.position.x,y:node.position.y,width:node.width,height:node.height,ports,properties:properties_from_semio(&node.properties)?})}
95: pub fn jack_edge_from_content(edge:&SemioGraphEdge)->Result<Edge,ValueError>{if !edge.label.is_empty(){return Err(invalid("Jack query projection cannot represent an intrinsic edge label"))}Ok(Edge{id:edge.id.value.clone(),kind:edge.kind.clone(),source:endpoint(&edge.source.value,edge.source_port.as_deref())?,target:endpoint(&edge.target.value,edge.target_port.as_deref())?,properties:properties_from_semio(&edge.properties)?})}
97: pub fn jack_content_handle(snapshot:&SemioGraphSnapshot)->JackContentChild{use pack::snapshot::ArtifactPack;let bytes=SemioGraphSnapshot::encode_pack(snapshot);let child_id=store::content_id("jack-content",&bytes);let target=semio_framework_artifact_reference::ArtifactRef{artifact_id:child_id.clone(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"graph".into()}};store::os_store::artifact_child::ArtifactChild::new(child_id,target)}
99: pub fn jack_content_child_with_snapshot(snapshot:SemioGraphSnapshot)->JackContentChild{jack_content_handle(&snapshot).with_local_owner(Arc::new(JackContentOwner::new(snapshot)))}
101: pub fn materialize_jack_snapshot(handle:&mut JackContentChild,snapshot:SemioGraphSnapshot){handle.set_local_owner(Arc::new(JackContentOwner::new(snapshot)))}
109: pub fn genesis_jack_child_pack(snapshot:&JackSnapshot,slot:&str,child_id:&str)->Result<Option<Vec<u8>>,ValueError>{use pack::snapshot::ArtifactPack;if slot!="content"||child_id!=snapshot.content.child_id{return Ok(None)}let owner=jack_content_for_handle(&snapshot.content)?;Ok(Some(SemioGraphSnapshot::encode_pack(owner.snapshot())))}
112: fn bundled_jack_content(child_id: &str) -> Option<SemioGraphSnapshot> {
128: use {semio_s_artifact_stdio_semio_graph::schema::mutations::add_edge_property::AddEdgeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::add_node_property::AddNodeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::change_node_label::ChangeNodeLabel, semio_s_artifact_stdio_semio_graph::schema::mutations::create_edge::CreateEdge, semio_s_artifact_stdio_semio_graph::schema::mutations::create_node::CreateNode, semio_s_artifact_stdio_semio_graph::schema::mutations::delete_edge::DeleteEdge, semio_s_artifact_stdio_semio_graph::schema::mutations::delete_node::DeleteNode, semio_s_artifact_stdio_semio_graph::schema::mutations::move_node::MoveNode, semio_s_artifact_stdio_semio_graph::schema::mutations::remove_edge_property::RemoveEdgeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::remove_node_property::RemoveNodeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::set_edge_property::SetEdgeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::set_node_property::SetNodeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation};
151: pub fn jack_content_from_children<'a>(snapshot: &JackSnapshot, children: &'a semio_framework_plugin::app::ChildContentView) -> Result<store::os_store::snapshot_read::SnapshotReadRef<'a, SemioGraphSnapshot>, semio_framework_plugin::Fault> {
157: children.typed_read::<SemioGraphSnapshot>("content", child_id)
168: pub fn jack_child_emit<C, D>(snapshot: &JackSnapshot, leaves: Vec<SemioGraphMutation>) -> semio_framework_plugin::Emit<crate::TrinityGraphMutation, C, D> {
172: semio_framework_plugin::Emit { child_preparations: std::collections::VecDeque::from([semio_framework_plugin::app::ChildEmitPreparation::of::<SemioGraphSnapshot, _>("content", &snapshot.content.child_id, leaves)]), ..Default::default() }
188: pub fn graph_leaves(base: &SemioGraphSnapshot, effects: &[crate::GraphEffect]) -> Vec<SemioGraphMutation> {
192: let push = |work: &mut SemioGraphSnapshot, leaf: SemioGraphMutation, leaves: &mut Vec<SemioGraphMutation>| {
193: let outcome = <SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::diff(&leaf, work);
203: let SemioGraphNode { id, kind, label, position, width, height, ports, properties } = content;
204: push(&mut work, SemioGraphMutation::CreateNode(CreateNode { id, kind, label, position, width, height, ports, properties, at: None }), &mut leaves);
209: push(&mut work, SemioGraphMutation::DeleteEdge(DeleteEdge { id: edge }), &mut leaves);
211: push(&mut work, SemioGraphMutation::DeleteNode(DeleteNode { id: GraphNodeId::new(id.clone()) }), &mut leaves);
216: push(&mut work, SemioGraphMutation::CreateEdge(CreateEdge { id: GraphEdgeId::new(edge.id.clone()), source, target, kind: edge.kind.clone(), label: String::new(), source_port, target_port, properties: properties_to_semio(&edge.properties), at: None }), &mut leaves);
218: GraphEffect::DeleteEdge(id) => push(&mut work, SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new(id.clone()) }), &mut leaves),
219: GraphEffect::RenameNode { id, name } => push(&mut work, SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: GraphNodeId::new(id.clone()), new_label: name.clone() }), &mut leaves),
220: GraphEffect::MoveNode { id, x, y } => push(&mut work, SemioGraphMutation::MoveNode(MoveNode { id: GraphNodeId::new(id.clone()), new_position: SemioPoint2 { x: *x, y: *y } }), &mut leaves),
224: Some((true, _)) => SemioGraphMutation::SetNodeProperty(SetNodeProperty { node_id: GraphNodeId::new(id.clone()), key: key.clone(), value: property_to_semio(value) }),
225: other => SemioGraphMutation::AddNodeProperty(AddNodeProperty { node_id: GraphNodeId::new(id.clone()), index: other.map_or(0, |(_, index)| index), property: SemioValueEntry { key: key.clone(), value: property_to_semio(value) } }),
232: Some((true, _)) => SemioGraphMutation::SetEdgeProperty(SetEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), key: key.clone(), value: property_to_semio(value) }),
233: other => SemioGraphMutation::AddEdgeProperty(AddEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), index: other.map_or(0, |(_, index)| index), property: SemioValueEntry { key: key.clone(), value: property_to_semio(value) } }),
237: GraphEffect::RemoveProperty { entity: crate::EntityRef::Node(id), key } => push(&mut work, SemioGraphMutation::RemoveNodeProperty(RemoveNodeProperty { node_id: GraphNodeId::new(id.clone()), key: key.clone() }), &mut leaves),
238: GraphEffect::RemoveProperty { entity: crate::EntityRef::Edge(id), key } => push(&mut work, SemioGraphMutation::RemoveEdgeProperty(RemoveEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), key: key.clone() }), &mut leaves),
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
3: use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
4: use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
3: use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
4: use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs

```rust
9: use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
91: fn content_property_bytes(value:&semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValue,depth:usize,items:&mut usize,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{
92: use semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValue;
99: fn content_bag_bytes(values:&[semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValueEntry],items:&mut usize,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{for entry in values{add_owned_bytes(bytes,entry.key.len(),maximum)?;content_property_bytes(&entry.value,0,items,bytes,maximum)?;}Ok(())}
100: fn content_node_bytes(node:&semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphNode,maximum:usize)->Result<(),ValueError>{
107: fn content_edge_bytes(edge:&semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphEdge,maximum:usize)->Result<(),ValueError>{
217: pub fn step(&mut self, snapshot: &JackSnapshot, scene: &SemioGraphSnapshot, maximum_bytes: usize) -> Result<QueryPreparationStep, ValueError> {
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs

```rust
4: use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphEdge;
5: use semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValue;
14: fn offset(edge:&SemioGraphEdge,key:&str)->Result<f64,ValueError>{
19: SemioValue::Float{lexeme}=>semio_s_artifact_stdio_semio_base::schema::geometry::read_binary64_lexeme(lexeme),
44: fn extend(edges:&BTreeMap<&str,&SemioGraphEdge>,offsets:&BTreeMap<&str,(f64,f64)>,flat:&mut BTreeMap<String,(f64,f64)>,seed:&str){
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs

```rust
53: type Members = semio_s_artifact_stdio_semio::SemioMembers;
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️graph/🦀️.rs

```rust
9: use semio_s_artifact_stdio_semio_graph::schema::snapshot::{SemioGraphNode, SemioGraphPortKind, SemioGraphSnapshot};
53: fn node_to_record(node: &SemioGraphNode) -> NodeGraphNodeRecord {
63: inputs: node.ports.iter().filter(|port| matches!(port.kind,SemioGraphPortKind::In|SemioGraphPortKind::InOut)).map(|port| NodeGraphPortRecord { id: port.name.clone(), label: Some(port.name.clone()), ..Default::default() }).collect(),
64: outputs: node.ports.iter().filter(|port| matches!(port.kind,SemioGraphPortKind::Out|SemioGraphPortKind::InOut)).map(|port| NodeGraphPortRecord { id: port.name.clone(), label: Some(port.name.clone()), ..Default::default() }).collect(),
71: pub fn render(document: &JackSnapshot, child: &SemioGraphSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs

```rust
190: pub fn content_to_workflow(camera: &crate::Camera, content: &semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>, Viewport2d) {
191: use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphPortKind;
192: let ports = |node: &semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphNode, inbound: bool| -> Vec<NodeGraphPortRecord> {
193: node.ports.iter().filter(|port| matches!((&port.kind, inbound), (SemioGraphPortKind::In | SemioGraphPortKind::InOut, true) | (SemioGraphPortKind::Out | SemioGraphPortKind::InOut, false))).map(|port| NodeGraphPortRecord { id: crate::port_key(&node.id.value, &port.name), label: Some(port.name.clone()), ..Default::default() }).collect()
722: type Members = semio_s_artifact_stdio_semio::SemioMembers;
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs

```rust
4: use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
33: pub(crate) fn render(content: &SemioGraphSnapshot, _cfg: &semio_framework_plugin::NoConfig, labels: &TrinityJackLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs

```rust
11: use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
12: use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
46: pub fn layout_graph(content: &SemioGraphSnapshot) -> JackLayoutGraph {
72: use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
73: use semio_s_artifact_stdio_semio_graph::schema::mutations::move_node::MoveNode;
74: use semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphNodeId;
76: let leaf = SemioGraphMutation::MoveNode(MoveNode { id: GraphNodeId::new(node_ids[node as usize].clone()), new_position: SemioPoint2 { x: position.x, y: position.y } });
84: let moves = member_ops.iter().filter_map(|bytes| match <SemioGraphMutation as protocol::OpBinary>::decode_op(bytes) {
85: Ok(SemioGraphMutation::MoveNode(payload)) => index.get(payload.id.value.as_str()).map(|at| (*at, LayoutRunPoint::new(payload.new_position.x, payload.new_position.y))),
93: pub fn build_job(identity: ToolRunIdentity, content: &SemioGraphSnapshot, checkpoint: Option<&[u8]>, member_ops: &[Vec<u8>]) -> Result<ToolRunJob, Fault> {
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🦀️.rs

```rust
5: use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
41: pub(crate) fn render(surface_id: &str, _controller_id: &str, snapshot: &JackSnapshot, content: &SemioGraphSnapshot, config: Option<&JackGraphWindowConfig>) -> UiAssemblyResult<BuiltNode> {
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs

```rust
14: use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
200: leaves: Vec<SemioGraphMutation>,
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-nodes/🦀️.rs

```rust
6: use {semio_s_artifact_stdio_semio_graph::schema::mutations::change_node_label::ChangeNodeLabel, semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation};
7: use semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphNodeId;
32: let leaves: Vec<SemioGraphMutation> = targets.iter().map(|id| SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: GraphNodeId::new(id.clone()), new_label: value.into() })).collect();
```

### 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs

```rust
6: use {semio_s_artifact_stdio_semio_graph::schema::mutations::delete_edge::DeleteEdge, semio_s_artifact_stdio_semio_graph::schema::mutations::delete_node::DeleteNode, semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation};
16: let mut leaves: Vec<SemioGraphMutation> = content.edges.iter().filter(|edge| doomed.contains(&edge.source) || doomed.contains(&edge.target)).map(|edge| SemioGraphMutation::DeleteEdge(DeleteEdge { id: edge.id.clone() })).collect();
17: leaves.extend(doomed.into_iter().map(|id| SemioGraphMutation::DeleteNode(DeleteNode { id })));
```

## Existing Test and Fixture Paths

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🪆️record-owner/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🧫️fixtures/🧫️child-owner-isolation/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧮️executor/🪜️resumable-query/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🪜️resumable-query/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🧫️fixtures/🪆️query/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🧫️fixtures/🎞️layout-run.json`

Composition fixtures and Graphology oracle: `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/🟦️.ts` with `🧫️fixtures/🧭️topology.json`, `🎛️flat.json`, `🪆️retained.json`. The neutral Jack graph-query fixture is already language-agnostic. Reuse it in Rust rather than copying expected outputs. Existing runtime fixture-sweep dependency on Jack must remain composed after native signature changes.
