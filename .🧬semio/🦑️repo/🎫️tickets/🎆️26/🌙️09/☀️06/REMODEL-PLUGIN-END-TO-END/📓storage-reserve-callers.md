# Production storage reserve callers

Source census of tracked Rust paths; no execution. Paths below are under 🧰️framework/🔨️modules unless stated.

- 🌱️value/🛬️decode/🧳️owned/📥️default/📦️slot/🦀️.rs: unreserved cursor phase already owns `self.effect` and uses `receive_allocation` with its real marker.
- 🌱️value/✨️derive/🛬️receive/🦀️.rs: generated record phase 2 stage 1 reserves partial `NativeRecordStorage`; receiver has matcher, slots and retirement but no allocation effect for this phase. Storage wrapper forwards the marker and is not a receiving frame.
- 🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs: `NativeReceiveSlot` unreserved receiver has no own effect.
- Same owned root: 📋️list/🦀️.rs phases Initialize and RowBirth reserve builder and child; 🔄️map/📦️box/🦀️.rs reserves pending-value backing; 🔗️sealed-shared-receiver/🦀️.rs phase Birth reserves child; 🧳️original/🦀️.rs reserves its receiver. These structs have nested child/shared state but no separate persistent effect for their own reserve; a child effect cannot stand in for parent allocation.
- 🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic/🦀️.rs phase 6, 🎁️wrapping/🦀️.rs BoxWrapping unreserved backing, 📦️storage/🦀️.rs OwnedStorage step, and root 🦀️.rs reserve_record_frame helper perform old post-effect `receive_retained_work`. Helpers need a genuine retained frame owned by the actual caller; exact struct ownership remains to be designed.
- 🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/✏️authoring/📦️storage/🦀️.rs `storage_ready` and 🏪️store/📬️command/🛬️receiving/🪜️cursor/🦀️.rs command and child storage reservations use job/wire contexts, not proven NativeReceivingEffect owners. They require their own genuine admission/settlement mechanism.

Paths are concurrently changing; this is a source snapshot and does not establish a passing build.
