# Seven Current Held Reconstruction Audit

Read-only audit of current canonical pair after images, including the later Value entry guard. Production was not modified and no owning runtime was launched. Earlier stdin rustfmt parsing returned exit 0 for all seven images; this establishes syntax only, not Rust type checking or execution.

## Current Scope

| Owner | Current SQL after SHA-256 |
| --- | --- |
| 🔤️text | `2f4311cb018432dbad9fb0cbf70f3b603ce266cad83781e14b92305b838721e7` |
| 🔊️audio | `cf7e6987062a13dc2cd0a58e36c02c8342dd2747f03d332dad3790be610b1e4d` |
| 🎬️video | `3d799d4aa946cdf8ba76d554949671e2c3f60fba76c7e80e982e4e8a4ff52918` |
| 📦️object | `57e0ce60ac97bce5f4b49a6414d68c92751d9fca0b9ee6c152fcf4acc9fd3aac` |
| 🔢️value | `04fe20e2e3fd054e42571b4114124d29ea3af702843c1384f9c18f93322b0b55` |
| 🌊️flow | `80c2a084ba167712f33275651a8978f0db40b2dd70da7aee385c33614b1b7828` |
| 🎞️animation | `dd5d9bbbf26bdc2aaceb3bc7305147481897d1b92101a3a8a65a74a5cadb5412` |

## Findings

The known Text signed-row policy remains a definite semantic integration blocker: its original identity check accepted signed primary identities, while the inspected shared `RowIndex::new` rejects `rowid <= 0`. The proposed signed API and three Text caller corrections must be joined before this owner can claim original acceptance parity. This is independent of the paid allocation and partial-owner changes.

The seven callers otherwise use the actual shared RowIndex API: borrowed FloatRow views, paid source positions and consumption marks, controlled heap sorting, owner/ordinal grouping, and explicit consumption. No FloatRow retirement implementation is assumed. Scalar index buffers remain ordinary paid scratch. The shared helper admits both buffers before allocation and checks identity duplicates after sorting.

Text retains run and mark fields in actual Owned builders; Audio retains channel/sample/tag fields; Video retains stream/sample fields; Flow retains node/parameter/edge fields; Animation retains timeline/channel/keyframe and weight owners. Root schema is copied while the typed snapshot remains guarded, followed by the final checkpoint before taking it. Object's child builder retains the original ArtifactChild and ArtifactRef strings rather than replacing the typed owner with a projection. These are static ownership observations, not allocator or bounded physical retirement results.

Value now passes `Option<&[(i64, &str)]>` to the shared forest. Its name buffer is built from `nodes.indices()`, which the actual RowIndex sorts by rowid, so `binary_search_by_key` has the required ordering. Graph/Table callers using None preserve literal text references. The later map-entry guard holds the copied key in Owned<SemioValueEntry> while child lookup and take remain fallible; its moved child is assigned before the entry is transferred. List children and completed roots similarly remain guarded through later refusal paths. The iterative forest detects repeated/cyclic ownership and uses paid link, row, completion and traversal buffers.

One cancellation follow-up remains in the Value forest: the linear contiguous-link ordinal validation walk has no checkpoint inside the walk. Its heap sort is controlled, but cancellation arriving during that subsequent walk is deferred until a later checkpoint. Add periodic existing ReconstructSnapshot checkpoints if the operation's cancellation contract must cover this pass; this does not require changed grants or another owner representation.

## Limits

No new definite API/type blocker was established beyond Text's recorded signed identity policy. Static review does not certify generic bounds, allocator equality, refusal cleanup execution, or a 4096-byte physical retirement limit. Owned logical close and eventual whole String/Vec backing release are distinct from physical one-allocation retirement. Preserve the independent semantic corpus and unchanged grants when running the owning Native demands.

The prior Audio/Presentation import report described Value's pre-merge map signature. That historical observation is superseded by the current names-slice signature audited here; no compatibility overload is needed.

## Later Continuity

Immutable joined the Value ordinal-loop checkpoint correction with value-ordinal-held-region-guard.json, registered handle 22529 exit 0. The former cancellation follow-up is superseded for the current held image; no Native runtime credit follows.
