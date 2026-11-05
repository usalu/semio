# Borrowed Publication Wire

A finite synchronous adapter now lives in plugin retained-command/full-operation/📃️wire. It implements the existing JsonWriteSource contract for a two-generation lane receipt, accepted count, borrowed UiDirtyScope, and borrowed Download tuple. Fault and terminal bodies use a separate raw-octet sink. No production page lifecycle, FullOp fields, publication pump, or persistence path changed.

The adapter borrows only during a synchronous call. It accepts the caller's NativeEncodeControl and already-owned mutable output slice. It never constructs a control, allocation, ToValue tree, String, or replacement ceiling. Both JSON and raw paths return the actual accepted byte count alongside success/refusal, preserving prefix custody.

The existing source-sink writer is single-pass: fixed path/scalar buffers, collection length refusal before iteration, checkpoint before output, and character-wise string processing. It does not premeasure an arbitrary UI source before a bounded sink refuses. Its written count is a byte extent, not a complete retained-work receipt; production integration must finance traversal, copying and metadata through original authority.

Three native tests were added under this owner. They compare exact neutral corpus frames with actual LaneItemReceipt/UiDirtyScope/tuple ToValue output and independent serde_json output, including omitted empty UI vectors and escaped Unicode download names. Successful writer calls require zero physical heap allocation/release. Additional laws exercise output/depth/item/cancellation refusal and raw accepted-prefix custody.

Existing first-party pack-json and pack-json-source-sink are explicit dependencies. No external/runtime dependency was added. The actual native target is package semio-framework-plugin-retained-command, --lib filter borrowed_publication_. The medium driver's job remains gated by core failures; it has not executed this target.

Latest lightweight validation: the adapter, its native tests, and the genuine original-emission regression each parse under the pinned Rust AST parser. Combined language-neutral tests pass 19 tests and 278 assertions in 274 ms (including the audit agent's page-admission corpus/schema). These are source/model checks, not native physical or browser proof.

The direct-emission regression is mounted in semio-framework-plugin-tests-16 and remains separately queued. It has preparation/retirement receipt drains, real source intake and ACK producer cleanup, and actual caller/heap comparisons. Its native semantic result is also pending. No heavy process was launched by this agent.

Exact adapter files: plugin/🧵️retained-command/🔄️full-operation/📃️wire/{🦀️.rs,🧪️tests/🦀️.rs}; registration only in full-operation/🦀️.rs; first-party dependency bindings in retained-command/📦️packages/🦀️rust/Cargo.toml. Actual zero-heap native assertions cover successful and refused writer calls.

Exact direct-emission files: plugin/🧪️tests/📤️original-emission/{🔣️.json,🧬️.json,🟦️.ts,🦀️.rs}; mount in 🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🦀️.rs; test-only first-party job binding in 🧷️companions/🥝️tests-16/📦️packages/🦀️rust/Cargo.toml.

## Bounded Native Factory Read-Only Check

The native-sized log prints only the first 512 opening indices, so its final printed511 is not an exact failing-turn attribution. Failure still records actual temporary capacity/release536 with receipt capacity/release0, copy0 and items1. No native source was changed in this audit. Factory close_children delegates registration/store/admission close owners and releases terminal boxes through quoted release. close_actor retains an inline ControlledRetirement over its actor fields; its constructor retains original fields and an empty paged cursor, while subsequent turns quote next_demand. No direct536-byte temporary allocation was identified there. The candidate Store constructor-recipient admission remains a concrete later opening seam under the root agent’s investigation.

Later root GDB attribution supersedes the provisional factory candidate above: native constructor opening completed;536-byte unreceipted temporary allocation was in app closure through lazy PeerPresenceRoot::empty_shared fallback. Root owns that repair.
