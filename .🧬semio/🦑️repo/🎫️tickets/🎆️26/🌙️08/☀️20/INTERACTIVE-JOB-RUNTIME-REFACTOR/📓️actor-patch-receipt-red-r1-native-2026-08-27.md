# Actor Patch Receipt — Native Compile RED R1

Command: `bun x nx run @semio-tech/framework-actor-rs:test --skip-nx-cache --args='--lib actor_ui_patch_receipt_ -- --nocapture'`, existing ticket-owned target and artifacts.

Actual exit 1 before tests. Fifteen errors: missing `ActorUiPatchReceipt`, missing `ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES`, and two `TurnResult` constructors requiring the not-yet-implemented `ui_patch_receipt` field. All three authored laws are unexecuted. Actor production was held unchanged for this RED snapshot.

```text
error[E0425]: cannot find value `ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES` in this scope
error[E0425]: cannot find type `ActorUiPatchReceipt` in this scope
error[E0422]: cannot find struct, variant or union type `ActorUiPatchReceipt` in this scope
error[E0433]: cannot find type `ActorUiPatchReceipt` in this scope
error[E0560]: struct `component::TurnResult` has no field named `ui_patch_receipt`
error[E0560]: struct `component::TurnResult` has no field named `ui_patch_receipt`
error: could not compile `semio-framework-actor` (lib test) due to 15 previous errors; 1 warning emitted
NX Running target test for project @semio-tech/framework-actor-rs failed
```

Raw captured list: `🧪️member-actor-patch-receipt-red-r1-native-2026-08-27.txt`. Production implementation is delegated to the lifecycle owner after this checkpoint; no codec or lifecycle runtime completion is inferred.
