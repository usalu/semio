# Durable Group Borrowed Descriptor Compiler Repair

The actual original whole Semio Native Before21282 stopped in Rust with four E0277 diagnostics for `DurableOwnedGroupAnchorV1` and `DurableOwnedGroupMemberV1`, before Nextest. The retained original log is `🗑️generated/root-semio-resumed-original-before-native.log`.

Mounted two explicit `BorrowedDslField` implementations immediately after their existing handwritten `value_field!` declarations. Both declare `BorrowedShape::Value`, matching the original `DslField::shape()` authority exactly. These types use the existing Value codecs; no derived record metadata is invented and no generic fallback is added. The unique two-declaration anchor and exact replacement are captured in `📥️inputs/child-paged-complete-family/store-durable-value-borrowed-schema-narrowed-guard.json`.

This is source repair only. The original whole Semio owner must rerun after the two separate Mutation generic-inference repairs owned by Root. No runtime pass or semantic Native closure is claimed.
