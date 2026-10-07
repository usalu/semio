# Nineteen Reconstruction Rust Borrow and Trait Coverage

Read-only current held after review, including Drawing's separate reconstruction helper. No production or held inputs changed; no duplicate compiler/runtime invoked. This pass addresses Rust integration, not the completed schema comparison.

| Owner | API and partial-owner types checked |
| --- | --- |
| Text | Signed RowIndex constructor; ordered/grouped/range return scalar positions; actual TextRun/TextMark builders and root, Vec<marks>. |
| Audio | Channel/sample/tag indices and ranges; AudioChannel<Vec<f32>>, AudioTag and root; direct scalar bits never borrowed into owner. |
| Video | Stream/sample grouping; VideoStream and VideoSample byte buffer guards; signed scalar fields remain plain Copy. |
| Object | Generic child<S:Send+'static> and ArtifactChild<S> retirement authority; borrowed RowIndex reference argument and optional returned child; copied ArtifactRef strings. |
| Value | names Option<&[(i64,&str)]>; Vec<Option<SemioValue>> completion slots, Vec<ValueEntry>, Vec<ValueNode> and root; forest slices borrow database, output is owned. |
| Table | Plain paid borrowed row lists; shared forest None names call; Owned<Vec<SemioValue>>, Owned<Vec<TableRow>>, per-row Owned<Vec<SemioValue>> and actual TableRow retirement. |
| Graph | Borrowed FloatRow lists and helper node_name<'a>; shared forest None; property helper mutable Option<Value> slice; GraphPort/Node/Edge and ValueEntry guards. |
| Flow | Node/edge get returns Option<FloatRow> explicitly handled; parameter ranges; FlowParam/Node/Edge with PortRef fields; actual domain retirement. |
| Image | Paid ordinary row references; ImageFrame byte buffer and MetadataEntry, root Option<Vec<u8>> ICC; actual primitive/Vec/Option retirement. |
| Animation | Channel/keyframe/weight scalar index slices; detail take Option<FloatRow>; Timeline/Channel/Keyframe including AnimValue::Weights buffer guarded within enum. |
| Mesh | Borrowed FloatRow relationship buffers remain plain scratch; Material/Mesh/Primitive/Texture guards; actual coordinate/color scalar and Vec implementations. |
| Kit | Generic child collection Vec<ArtifactChild<S>>; actual ArtifactRef/BlobRef/LinkPin/ArtifactLink retirement; KitType/Design/Piece/Connection; grouped copied tuple keys. |
| Model | Entity partitions Vec<usize>; parent_positions/cycles mutable scalar slice; direct relations.indices() iteration; Spatial/Element/PropertySet/Property/Relation guards and enum value fields. |
| CAD | Entity helper owns separate row-index fields/order vectors; source range borrowed while disjoint detail index is consumed; EntityRecord, CadEntity variants, Layer/Block and root have domain retirement. |
| Document | Forest Option<&RowIndex> lookup inputs; collection built slots Owned<Vec<Option<Vec<DocBlock>>>>; actual DocRun/DocStyle/DocImage/DocTableRow derive Default; block/run/table vectors have retirement. |
| Presentation | Forest None lookup callers; borrowed RowIndex/order fields and owned block collection slots; SlideShape, SlideTableRow and Master/Layout/Slide placeholder defaults actually exist. |
| Drawing | Existing indexed::Rows APIs rather than RowIndex; typed built Vec<Option<DrawNode>>; ordinary borrowed Task children; DrawNode variants, DrawStyle/DrawLayer and root guards. |
| BRep | BrepGeometry<T:RetireOwned> implies Send+'static; RowIndex beside Owned<Vec<Option<T>>>; NURBS sequence mutable Vec and separate parameter indices; actual Curve/Curve2/Surface and topology retirement. |
| Base | Actual owned SemioSubsetSnapshot then root guard; child calls return owned snapshots; root Reconstruction control borrow ends with checkpoint before guarded take. |

The actual common helper's grouped_by accepts Fn(FloatRow<'a>)->Result<(u8,Option<i64>)>; inspected closures and named owner functions match it. parent_positions returns Vec<(Option<usize>,u8)> and cycles consumes a mutable slice. row returns FloatRow directly; get/take/take_index return Option<FloatRow>. Each source index/ordinal distinction is explicit. The borrowed view lifetime follows the database, not the helper/control borrow, so immutable lookup views do not retain a mutable control reference.

No direct iteration over Owned was found in these reconstruction paths. Collections use get_mut().iter/drain/pop or transfer through take; roots parameters are genuine slices. The repaired Model relation loop no longer takes a second reference to indices(). No Option<RowIndex> is passed where the forest requires Option<&RowIndex>. No borrowed FloatRow/RowIndex/task type is placed into Owned requiring 'static.

Actual trait authorities are Semio root domain RetireOwned implementations, Value retirement's Vec/Option/scalar implementations, and Store's ArtifactChild/ref/link/pin definitions. The helper Owned requires RetireOwned; no weaker generic bound or imaginary FloatRow eligibility is assumed. This inspection found no new definite Rust API/trait/borrow blocker. It does not replace full-crate borrow checking or allocator/Native execution proof. Root's fresh Native29347 remains active authority after the 55-error prerequisite repair.
