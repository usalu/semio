# Shared Native decode ledger role separation

Read-only current source inspection following actual original After66676 (270/271; Base complete full/empty remaining failure). No replay or runtime success claim.

The shared Base Native `decode` currently constructs `NativeDecodeControl::new(remaining.min(limits.max_value_bytes), ...)` inside `SqliteSnapshotControl::allocation_stage`. This conflates relational semantic cell bytes with Rust owned allocation requests. A typed Vec request includes Rust element layout, enum padding and allocated capacity, which need not fit the semantic SQLite INTEGER/TEXT/BLOB sum. The existing Value owner already constructs its Native control from `remaining` alone while retaining its explicit semantic document gate.

| Owner | Current borrowed semantic gate before owned child fields |
|---|---|
| ✉️base | Root `subset_limits` checks before root schema copy; selected child callback gates before child fields |
| 🌊️flow | Binary and Document gates at callback entry |
| 🎞️animation | Binary and Document gates at callback entry |
| 🎬️video | Binary and Document gates at callback entry |
| 🏛️model | Binary and Document gates at callback entry |
| 📊️table | Binary and Document gates at callback entry |
| 📐️cad | Binary and Document gates at callback entry |
| 📑️document | Binary and Document gates at callback entry |
| 📦️object | Binary and Document gates at callback entry |
| 📽️presentation | Binary and Document gates at callback entry |
| 🔊️audio | Binary and Document gates at callback entry |
| 🔢️value | Document gate, including Binary borrowed UTF-8 route |
| 🔤️text | Binary and Document gates at callback entry |
| 🔺️mesh | Binary and Document gates at callback entry |
| 🕸️graph | Binary and Document gates at callback entry |
| 🖊️drawing | Binary and Document gates at callback entry |
| 🖼️image | Binary and Document gates at callback entry |
| 🧊️brep | Binary and Document gates at callback entry |
| 🧰️kit | Binary and Document gates at callback entry |

All 18 selected child owners have explicit `admit_binary`/`admit_document` callback entry checks (Value has only its canonical document grammar, used for both payload dialects). Actual admit wrappers delegate to borrowed semantic Census readers, retaining copied layout/rows/value limits. Base checks complete 183-table/54-column schema layout, subtracts its real root row and `16 + schema UTF-8 + tag UTF-8` bytes in `subset_limits`, then delegates the adjusted semantic limits to the selected child callback. Root schema copying intentionally precedes child gate to preserve the original paid cancellation premise; its own root semantic budget check precedes that copy.

The narrow clean change is shared Native `NativeDecodeControl::new(remaining, &mut callback)`. Preserve the `limits` argument to every Binary/Document callback, Base `subset_limits`, borrowed child Census, file-size gate, controlled wrapper/preamble parsing, final Owned guard/checkpoint, and `allocation_stage` returned `native.owned_bytes()` settlement. This leaves actual owned requests bounded by the caller's explicit allocation ledger; copied `max_value_bytes` remains enforced by semantic cell admission rather than doubling as a Rust heap ceiling. Do not change `reconstruction_remaining_bytes` indiscriminately: that is a distinct relational reconstruction route, not this allocation-stage Native decode route.

`allocation_stage` obtains `allocation_remaining_bytes`, invokes the operation under forwarded cancellation/progress, and admits the reported owned request count even on typed refusal. Thus removing only the implicit min does not remove aggregate payment or cancellation settlement. Original exact-allocation/minus-one tests continue to constrain actual heap through max_allocation_bytes; semantic minus-one/columns/rows tests continue to constrain the borrowed gates. Their current repaired whole-owner rerun is necessary evidence; this static audit cannot qualify them.
