# Actor Lifecycle Turn R3 RED

Exact canonical actor turn filter before implementation. Actual captured output:

```text

> nx run @semio-tech/framework-actor-rs:test --args=--lib actor_instance_lifecycle_turn_ -- --nocapture

> bun ./📜️script.ts test --lib actor_instance_lifecycle_turn_ -- --nocapture

error[E0560]: struct `component::TurnResult` has no field named `lifecycle_receipt`
error[E0599]: no method named `expect` found for unit type `()` in the current scope
error[E0599]: no method named `is_err` found for unit type `()` in the current scope
error: could not compile `semio-framework-actor` (lib test) due to 3 previous errors; 1 warning emittedWarning: command "bun ./📜️script.ts test --lib actor_instance_lifecycle_turn_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-actor-rs failed

Failed tasks:

- @semio-tech/framework-actor-rs:test

Hint: run the command with --verbose for more details.


```
