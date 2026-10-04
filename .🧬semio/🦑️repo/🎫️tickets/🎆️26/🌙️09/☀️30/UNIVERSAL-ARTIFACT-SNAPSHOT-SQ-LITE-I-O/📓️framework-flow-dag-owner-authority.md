# Framework Flow and DAG Snapshot Authority

These are two persisted owners omitted by app-only enumeration. They remain unimplemented for owned SQLite and are not covered by the separate plugin Flow/DAG capabilities. No factory or runtime proof can be inferred from those same-named plugin tests.

## Flow

The actual Framework FlowHostSnapshot persists schema:String, CameraJson{x,y,zoom:f64}, widgets:Vec<Widget>, synapses:Vec<SynapseSpec>, layout:OrderedMap<WidgetLayout{x,y:f64}>. Widget has nine variants: Neuron(id,neuron_kind,Dictionary params,ordered input/output port names,preview flag); InputSlider(id,label,value,min,max,step:f64); InputNote(id,text); InputImage(id,src); Variable(id,name,schema); OutputPreview(id,Dictionary preview,OrderedSet expanded); OutputAction(id,action); OutputExport(id,format); Cluster(id,name,Neural Tree,FlowGui). The nested FlowGui retains camera, ordered node GUI entries with typed NodeChrome, and preview entries with optional source/layout, mode, dictionary and expanded paths. Schema and line/identifier text are literal fields. Binary64 requires explicit bits and numeric class companions; recursive dictionary/tree/native GUI relationships require full typed entities and ownership edges.

FlowHostSnapshot's public native methods currently delegate through the private FlowHostSnapshotDsl structural mirror, with flow.flow v1 envelopes. The mirror contains explicit fields rather than an opaque snapshot JSON string, but its eager conversion functions and nested custom constructors lack required controlled construction. These native paths must be audited and brought under the same cumulative bounds and cancellation authority before native erased I/O can be claimed. FlowArtifact is specifically documented and used as a computed rendering projection from the persisted host snapshot, so it must not be invented as another independent persisted root solely because its type is named Artifact.

## DAG

The real Framework DagSnapshot in the VCS owner persists schema:String, nodes:Vec<DagNodeSpec> and edges:Vec<DagHostSnapshotEdge>; camera and selection are ephemeral host fields. Each node retains literal id/name/abbreviation/icon, x/y/width/height:f64, optional operator_kind, PropertyBag and an eleven-variant DagNodeKind. Select.selected is full u64 and cannot be truncated into signed SQL INTEGER. Kinds carry input/output IoPortSpec records, slider scalars, ordered selections, optional media, typed preview content, expanded strings and app instance identities. Preview Tree.json is a first-party typed DslValue despite its name and needs semantic value/entity relationships, not a JSON text column.

DagSnapshot delegates native text/Pack to private DagSnapshotDsl and reports that structural record specification. It currently has no owned ArtifactSqliteSnapshot/provider/Pack hook. The document schema is dag.host_snapshot; its actual private DSL declaration separately publishes dag.dag v1 native envelopes. Native-derived DTOs are not authoritative persisted schema definitions. Exact metadata, field and variant audit plus individual authored SQL and neutral independent fixtures remain required before mounting an implementation.

## Status

Read-only owner-source examination, no tests or compilation were run for these two new scopes. No implementation or completion is claimed. Refer to the latest universal remaining-native audit for their factory locations.
