# Native receiver recovery seam

NativeReceiver advance accepts NativeDecodeControl, but begin_close and close_step do not; the generic close_native_receiver_backing helper supplies no control. NativeReceivingEffect resolution, work settlement, receipt and take require the same original control and construction recipient. NativeSharedAdmission resume is specific to its own effect and generic finish_receiving is private. Retiring a pending effect explicitly refuses UnsupportedOwner.

Therefore the current generic receiver trait has no recovery hook for an internally held pending effect during close. The owning path must expose a pre-close recovery seam with the original control and authority, or evolve the close API. Cancellation behavior must be established in that owner; no runtime claim.