# Flow Artifact Contract Extraction Read-Only Audit

Actual local Rust/manifests/schema/test source inspected. No Cargo, tests, deletion, modifications or generated witnesses. Proposed extraction is design evidence, not runtime acceptance.

All Product paths below share `🧰️framework/🛍️products/💻️os/🔨️modules/`; General paths share `🧰️framework/🔨️modules/`.

## Actual Imports and Responsibilities

- Flow package implementation `🌊️flow/📦️packages/🦀️rust/🦀️.rs:27–29` imports **another concrete playbook artifact** plus flow-flow artifact `artifact` and `retained` modules. Its aliases expose OS kernel store/DSL/pack/SPR/VCS and infinite directed DAG board. Dependency removal is therefore more than replacing two Cargo entries.
- `🌊️flow/🖥️host/🦀️.rs:7,19` imports DAG snapshot literal projection, execution-row projection, DagHostSnapshot/Edge, DagNodeKind/Spec, EdgeRouteStyle, IoPortSpec. Lines 465/2244 construct artifact DagCamera; lines 5386/5393 call normalize_node_display.
- Host line 20 imports concrete FlowStore, FlowMutation, Add/Change/RemoveWidget/Synapse, ChangeLayout, FlowLayoutEntry, retire_flow_mutation and FLOW_DOCUMENT_SCHEMA. These are document event/store/codec and retirement ownership implementations, not just generic graph types.
- Host line 3313 retains boxed FlowHostSnapshotIntake from the artifact crate; 3636 exposes artifact FlowSnapshotCopyIntakeSource in a public failure enum; 3781/3785 size/account and construct FlowHostSnapshotCopy and intake source. Any extraction must preserve original admission, exact source ownership, progress/cancellation and retirement rather than copying snapshots into an uncredited wrapper.
- `🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs:7–29` reexports artifact extension implementation and exposes its FlowHostSnapshot/Widget/widget_id_for in dependency JSON projection. Host retirement test also imports DagCamera and IoPortSpec. Source and tests both depend on concrete artifact owners.
- Flow artifact `🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs:11` imports artifact DAG normalization, node/spec/preview/port types; 453 onward constructs IoPortSpec for variables, clusters and neural channels. This is the confirmed artifact-to-artifact incoming edge.

## Existing Defining Owners

General `🕸️graph/🗣️dsl/🦀️.rs:122` defines QueryableGraph; 379/388 define owned WireNode/WireEdge and 481/512 serialize/parse wire literals. These already support generic graph topology and execution rows. Existing graph crate is `🕸️graph/📦️packages/🦀️rust`, with graph manifest admission schema and portable fixtures under its own tree. Reuse it rather than copying graph row types.

DAG artifact snapshot Rust owner is `♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs`: normalization30, PortShape49, EdgeRouteStyle59, IoPortSpec69, media/preview/node variants169–208, DagNodeSpec306, snapshot621, camera631, edge641, execution-row projection690. Defining JSON schema is that artifact's `🧬️schema/🔣️.json`. Its root module also owns demo text and VCS/document implementation. Port metadata and graph canvas presentation contracts can outlive that specific document producer; demo/document/VCS factories remain artifact-owned.

Flow artifact root `🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs` mounts snapshot schema, retained ownership, VCS, parameter intent, extension implementations, registry/catalogue and package/sqlite laws. Snapshot owner defines WidgetLayout64, FlowArtifact73, FlowUi83, FlowNodeGui98, NodeChrome108, FlowPreviewGui143, FlowChannelRef160, CameraJson168, SynapseSpec186, Widget202, FlowHostSnapshot276. Its defining JSON schema is `🧬️schema/🔣️.json`, with separate parameter/intent, cluster-tree inference and mutation/diff schemas. This domain schema belongs Flow's document contract owner, not a domain-neutral General graph API.

## Concrete Clean Boundary

1. Keep General graph topology/query/DSL contracts in the existing graph owner. Move only reusable port metadata and canvas DAG presentation contracts plus their schema/native twins to an appropriate existing/general graph or Product infinite directed-port contract owner according to domain neutrality. Handcraft one defining owner; artifact adapters import it. Do not lift Flow Widget/neural schema into General.
2. Move Flow's reusable Widget/Synapse/HostSnapshot domain contracts, defining schema and retained intake interface to the Flow module contract taxonomy. Preserve one implementation and its original ownership types; artifact public constructors/factories and document codecs remain under the artifact producer.
3. Flow host receives a first-party document execution capability declaring required operations (snapshot intake, command/event admission, schema/codec boundary and controlled mutation retirement) and a first-party graph presentation capability. The artifact factory supplies concrete FlowStore/mutation/retirement implementation when its child-owned declaration is mounted. Host must not expose concrete artifact intake source in its public failure enum.
4. Move artifact-specific extension registry/catalogue/wire implementation behind that child contribution. Keep generic request/result/failure/turn schemas under existing `🌊️flow/🧩️extensions/🕸️wasm/🧬️schema` owners. Resolve optional mounted producer at assembly; do not retain unconditional artifact crate dependency in surviving parent host.
5. Apply the same boundary to the discovered playbook import rather than claiming deletion independence after only DAG/flow changes. Root membership globbing cannot repair these required imports/dependencies.

## Existing Coverage to Preserve

DAG has host-kind admission, label tiers, demo ownership and SQLite snapshot fixtures/laws. Flow has cluster-tree inference, parameter intent, parameter semantics and SQLite snapshot fixtures/laws; source root mounts package and SQLite snapshot test modules. Flow package dev dependency enables artifact `protocol-laws` feature. General graph has portable cold-properties/output/record-owner fixtures and manifest admission schemas. These are source-presence facts; none were run.

Relocate tests with their defining contracts, retain artifact producer law coverage separately, and add portable independent graph/snapshot/retirement capability vectors plus actual removed-artifact host assembly probes. Verify SQL/binary/graph oracle behavior already represented by existing laws before inventing a second implementation. Current native and graph processes were untouched; Native whole-producer deletion behavior remains unqualified.
