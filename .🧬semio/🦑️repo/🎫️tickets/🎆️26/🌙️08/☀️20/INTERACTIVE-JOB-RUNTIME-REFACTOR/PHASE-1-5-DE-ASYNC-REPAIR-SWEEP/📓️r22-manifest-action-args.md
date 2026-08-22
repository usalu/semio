# R22 Manifest Action Arguments and Interaction References

## Result

The manifest's pure action-argument, command-argument, and interaction-reference surfaces are synchronous end to end. They no longer manufacture ready-only futures in declaration builders or force editor/plugin callers to await in-memory construction.

## Synchronous Surface

- `ActionArgOption::new`.
- The complete `ActionArgDef` builder and schema family: constructors, required/default/description modifiers, and JSON-schema rendering.
- Recursive argument-format/schema helpers.
- `ActionDefinition::with_args` and `CommandDefinition::with_args`.
- `InteractionRef::new` and `InteractionRef::as_str`.
- Every manifest-local call and test was updated to the value-returning contract.

All changes preserve serialization, defaults, action identities, typed arguments, and interaction ids; only decorative suspension points were removed.

## Verification

- `cargo check -p semio-framework --lib --message-format=short`: passed before the final caller cleanup.
- `cargo test -p semio-framework action_arg_ -- --nocapture`: passed after the complete cleanup; 3/3 focused tests passed and the full `semio-framework` library-test target compiled.
- Source census: no `pub async fn` remains in the listed pure argument/reference methods and no stale `.await` remains on their manifest-local call sites.

Warnings emitted during the focused test are pre-existing workspace lints in unrelated OS store, I/O, kernel, and media-converter surfaces; no warning was introduced by this packet.
