# Eighty Held Rust API and Trait Audit

Fresh absence-aware readback: 80 of 80 actual files match current before images. Single-object shared helper after SHA256 is 09c8f1a4a9cfc33b903f92e56bd250202daf8326519e71968f3cd60a6e3ff7c0. This review writes only this ticket Markdown, with no production or held input mutation and no compiler/runtime invocation.

## API Checks

Actual RowIndex grouped_by accepts Fn(FloatRow<'a>) returning (u8, Option<i64>); reviewed relationship closures use exactly that fixed scalar key, including CAD None owner and Presentation tagged owner. parent_positions returns Vec<(Option<usize>,u8)> and cycles accepts its mutable slice. take/get/take_index return optional borrowed FloatRow and callers explicitly handle refusal/missing rows. row returns FloatRow directly. indices returns &[usize]; the previously reported Model extra reference has been removed in the current held after.

No mutable control is stored in RowIndex, so helper borrows of row/index data remain tied to the database instead of the temporary control. Disjoint entity/order/detail fields allow CAD, BRep and Presentation source-range iteration while another detail index is consumed. Their grouped/range keys use scalar copied tuples, not retained temporary string ownership. FloatRow is used as a borrowed view; no Owned<FloatRow> trait fiction is introduced.

The loop scan found no remaining `in &owner.indices()` form or direct iteration over an Owned wrapper in the reviewed reconstruction paths. Value and Document `for root in roots` operate on actual input slices. Guarded collections use get_mut().iter/drain/pop or transfer their plain collection through take. Option geometry/completion slots use the actual Option and Vec retirement implementations.

## Typed Ownership Checks

RetireOwned requires Send + 'static. BRep generic geometry containers state that bound explicitly, which satisfies the existing Owned bound; curve, p-curve and surface variants have genuine domain implementations. Object's borrowed child helper stores actual ArtifactChild<S>; its Send + 'static generic requirement matches the existing ArtifactChild retirement authority. Domain guard types for Mesh/Image/Table/Graph/Drawing have actual root RetireOwned definitions. Scalars and arrays inside those owners use the existing scalar and collection implementations. Borrowed RowIndex/FloatRow/task buffers remain ordinary paid scratch rather than being forced into a 'static typed owner.

All 19 snapshots have explicit retire_sqlite_snapshot hooks in the current held after. These call the existing base Native Owned authority; they do not rely on inherited drop(self). This establishes intended hook dispatch statically, not a measured physical retirement limit.

Base's 17 non-Value child Native binary callbacks retain body/control/limits three-argument signatures. The Value branch is deliberately special. Root metadata/schema handling remains within the guarded actual snapshot after child reconstruction. No callback adapter, public signature change, or new retirement trait is required by these checks.

## Result and Limits

No new definite compile blocker was found in this pass. The Model &&slice finding is repaired in held input; signed RowIndex is refreshed and qualified by Physical's separate owning receipt. Rust type inference, borrow checking for the complete integrated crate and Native law behavior still require Root's actual owning compiler/run. Static API review is not compiler success credit, and no 4096-byte source backing release claim is made.

## Signed Text Continuity

Immutable joined the three Signed caller regions under registered handle36696 exit0 following Physical shared Native46/46 and Source58/58 receipts. This qualifies the separate shared helper and held join, not Semio Native runtime. Subsequent actual original29778 produced55 compiler errors in existing production prerequisites; the new canonical proposals are documented separately.
