# Current Eighty-Path Held Integration Review

Read-only review after Root prerequisite mount 82614. The roster contains 79 semantic/helper pair records plus the single-object shared RowIndex record: 80 distinct paths. Fresh absence-aware readback confirms all 79 semantic/helper before images: 60 existing files match their text and 19 new semantic files are absent with null before images. The earlier 19-change assertion was an audit bug caused by comparing absent files as empty strings against null; it is withdrawn. The shared RowIndex is already mounted, so its production file intentionally differs from the original before image.

## Cross-Owner Authorities

All 18 child SQL owners expose `pub(crate) visit_rows(&ActualChildSnapshot, &mut RowWriter<'_, '_>) -> Result<(), ValueError>`. Base delegates the selected actual child directly through these methods, using the same writer and control ledger. It does not create an intermediate child database. Base's private visitor is sufficient because only its own module invokes it.

The inspected actual shared RowIndex has borrowed FloatRow views, scalar source indices, consumption marks and explicit remaining count. Source indices are aligned to original table storage, while the sorted index vector orders identities. Therefore BRep's `values[source_index]` geometry slots and Value's sorted name slice use consistent distinct coordinate systems. Control is passed per operation; RowIndex does not retain a mutable control borrow. Paid ordering and grouping helpers retain borrowed source cells, not copied native owners.

Document's public block collection reconstruction accepts `Option<&RowIndex<'_>>` for styles/images. The supplied lookup's data lifetime belongs to the source database, so returned text does not borrow the transient mutable control. Presentation's literal-reference branch remains separate from Document's foreign-key reference branch. Value's shared forest accepts the joined name slice, and Graph/Table's None callers remain compatible.

Base's Native decoder uses the actual child SQLite Native decoding owner. The 17 non-Value child binary callbacks have the existing three arguments `(body, &mut NativeDecodeControl, SqliteDatabaseLimits)`. Value remains the deliberate special branch rather than requiring an invented callback. No callback arity or ownership adapter is required by the held reconstruction repairs.

## Typed Partial Owners and Capacity

Mesh retains the actual Material/Primitive/Mesh/Texture fields in typed Owned builders. Current material copies ID before scalar fields; primitive arrays precede its ID/material copy; mesh primitives precede mesh ID, matching the corrected original order. The arrays are reserved to actual grouped cardinalities and the root schema stays guarded through final checkpoint.

Image's Frame and MetadataEntry guards have actual RetireOwned implementations. Table's nested cells and SemioTableRow are eligible through the existing Vec and row implementations; borrowed SQL rows remain ordinary paid scratch. Graph's Port, Node, Edge and ValueEntry guards use actual domain retirement definitions, including property key retention before fallible root lookup. No borrowed FloatRow is wrapped in typed Owned.

BRep's `BrepGeometry<'a,T:RetireOwned>` stores the borrowed RowIndex beside `Owned<Vec<Option<T>>>`; RetireOwned itself requires Send + 'static, satisfying Owned's actual bound. Slots are paid and initialized before NURBS construction. Curve/Curve2/Surface have actual RetireOwned implementations. `brep_sequence` reserves exact range cardinality, takes each relationship once and decodes into the already guarded typed NURBS vector. Separate mutable parameter-family borrows do not borrow the parent curve/surface control. The geometry transfer is assigned immediately into its guarded topology owner before later fallible fields. Drawing's typed DrawNode/DrawStyle/DrawLayer retirement implementations also exist; its borrowed task/frontier scratch is not given fictional typed retirement eligibility.

## Open Authority and Validation Limits

The actual RowIndex inspected still rejects nonpositive row identities. Text's signed identity policy correction remains pending integration and is the previously reported semantic blocker. No additional definite signature, lifetime or retirement-bound blocker was established by this static review. This is not Rust compiler or runtime success credit. Root's owning Native Before 29778 remains the authority for compiler and execution results.

The seven-owner Value ordinal cancellation follow-up was subsequently joined by Immutable through `value-ordinal-held-region-guard.json` (registered 22529 exit 0). The former unchecked ordinal walk observation is superseded: each link now has an existing ReconstructSnapshot checkpoint. This receipt establishes the held input update, not Native behavior.

Source 151-law qualification is separate from these Native checks. Paid cardinality, semantic parity, typed partial retention and physical source retirement are distinct claims; no 4096-byte whole String/Vec backing release guarantee follows from Owned's bounded logical retirement loop.
