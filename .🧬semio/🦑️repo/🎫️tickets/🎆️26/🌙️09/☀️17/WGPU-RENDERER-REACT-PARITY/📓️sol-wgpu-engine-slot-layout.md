# Engine Surface Slot Layout Audit

The root full native suite measured `EngineSurfaceSlot` at 82656 bytes, against the committed fixture's 81600. Registry capacity remains 256 and the registry owner remains 32 bytes; the other EngineCanvas tables retain their declared shapes (352/24 and 384/72 element/owner bytes). The failing comparison precedes bounded-stack construction, so this failure alone does not show a stack regression.

The first-party registry stores `Box<[EngineSurfaceSlot; 256]>` and constructs every slot through `semio_framework_async::boxed_fixed_slots`; it does not construct a temporary inline array. Each slot separately owns optional live and optional retiring surfaces. Both carry their node graph/map/board/editor/raster owners and sync caches, while retirement additionally carries the component retirement states. The owner fields are a box pointer, a text-editor receipt counter, an outbox cursor, and a fault flag.

The exact recorded element byte count is a measurement contract. The safety budgets are separate: 1048576 bounded thread stack bytes, 2097152 worker thread stack bytes, and the 65536 conversion threshold. Those budgets and the registry capacity are unchanged. Read-only source history shows no current EngineCanvas live-owner production diff; the existing fixture measurement was itself updated from 81320 to 81600 on September 30. This audit does not invent a new media field as the cause of the 1056-byte difference.

A temporary `[DEBUG]` component-size witness was inserted before the existing assertion so the next coordinated failed native law records live/retirement and major component sizes. Updating the measurement awaits that witness. The current coordinated attempt stopped at unrelated frame-worker source ownership generation before Cargo, so no component trace has yet been obtained.

## Executed Layout Witness and Exact Measurement Repair

The coordinated native law executed and recorded these sizes in bytes: slot 82656, live surface 28016, retiring surface 54368, registry 32, node-graph enum 15712, Flow host 15712, DAG host 11520, board host 6712, map host 792, editor host 432, raster host 1128, editor delivery state 832, node-graph sync cache 320, map sync cache 392, editor sync cache 72. The slot accounts for both live and retiring payloads plus 272 bytes of slot identity/generation/alignment metadata: `28016 + 54368 + 272 = 82656`. Exact witness: `🗑️generated/sol-theme-guidance-viewer-native-red-compiled.txt`.

The declaration now records this measured 82656-byte element shape. This is a stale exact-layout record repair, not an increase to any safety bound. The 256-entry heap table is 21159936 bytes; the 32-byte registry owner still holds it through a box constructed directly on the heap. Capacity, owner size, the other two slot shapes, the 1 MiB bounded construction stack, the 2 MiB worker stack, and the 65536 conversion threshold retain their previous values. No EngineCanvas production field or construction path changed. The temporary diagnostic was removed after its values were retained here.

The existing neutral TypeScript budget arithmetic/shape suite passed all four selected laws after this declaration update (22 other tests filtered); log `🗑️generated/sol-2026-09-29/engine-slot-neutral-current.log`. Native green, including actual construction on the 1 MiB thread, is pending the coordinated union.

## Coordinated Native Receipt

The executed union `🗑️generated/sol-native-theme-viewer-green-display-introduction-red.txt` ran 17 selected laws: 15 passed and two unrelated Display/introduction laws failed as intended. The viewer offer, admitted example picker, boot announcement and heap-first registry laws passed. This confirms the concrete production fixes; it does not establish full renderer parity. No duplicate Cargo process was started by this lane.

The coordinated follow-up `🗑️generated/sol-native-theme-display-introduction-green-graph-repaired.txt` executed 34 laws and passed all 34, including these viewer, admitted picker/boot and heap-first laws again.
