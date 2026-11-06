# Native receiver migration census

At this source snapshot handwritten implementations still need required pending_recovery and begin_close_recovered across owned list, mapped, box, tree scalar/string/option/sequence, bulk, tuple, array, variant/unit, ordered map/set and sealed-shared receiver. Handwritten test implementations include owned/🧪️tests and header/work tests. NativeReceiveSlot has its own pending forwarder; its callers must propagate fallible begin_close. Generated record/enum were separately migrated.

These files may change concurrently. No compiler or runtime result claimed.