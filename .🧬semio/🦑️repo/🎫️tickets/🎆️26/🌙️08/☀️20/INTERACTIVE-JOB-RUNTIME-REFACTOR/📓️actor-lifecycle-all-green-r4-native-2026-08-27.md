# Actor Lifecycle R4 Native

Canonical actor_instance_lifecycle_ filter: five passed,97 skipped,0.050s,exit0. Includes two new outer TurnResult tests after actual three-error RED plus previous three lifecycle codec/authority tests. Actual captured tail:

```text

> nx run @semio-tech/framework-actor-rs:test --args=--lib actor_instance_lifecycle_ -- --nocapture

> bun ./📜️script.ts test --lib actor_instance_lifecycle_ -- --nocapture

warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID 17929a5f-88fd-4e26-b497-71c8702b1109 with nextest profile: fundamental
    Starting 5 tests across 1 binary (97 tests skipped)
       START [         ] (1/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_turn_rejects_invalid_receipt_without_partial_output

running 1 test
test component::instance_lifetime::tests::actor_instance_lifecycle_turn_rejects_invalid_receipt_without_partial_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 101 filtered out; finished in 0.00s

        PASS [   0.010s] (1/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_turn_rejects_invalid_receipt_without_partial_output
       START [         ] (2/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_turn_round_trips_shared_outer_vectors

running 1 test
test component::instance_lifetime::tests::actor_instance_lifecycle_turn_round_trips_shared_outer_vectors ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 101 filtered out; finished in 0.00s

        PASS [   0.010s] (2/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_turn_round_trips_shared_outer_vectors
       START [         ] (3/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_matches_shared_independent_leb128_vectors

running 1 test
test component::instance_lifetime::tests::actor_instance_lifecycle_wire_matches_shared_independent_leb128_vectors ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 101 filtered out; finished in 0.00s

        PASS [   0.010s] (3/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_matches_shared_independent_leb128_vectors
       START [         ] (4/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_rejects_invalid_authority_before_writing

running 1 test
test component::instance_lifetime::tests::actor_instance_lifecycle_wire_rejects_invalid_authority_before_writing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 101 filtered out; finished in 0.00s

        PASS [   0.009s] (4/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_rejects_invalid_authority_before_writing
       START [         ] (5/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_requires_exact_accepted_identity_before_terminal

running 1 test
test component::instance_lifetime::tests::actor_instance_lifecycle_wire_requires_exact_accepted_identity_before_terminal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 101 filtered out; finished in 0.00s

        PASS [   0.010s] (5/5) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_requires_exact_accepted_identity_before_terminal
────────────
     Summary [   0.050s] 5 tests run: 5 passed, 97 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-VBlj45



 NX   Successfully ran target test for project @semio-tech/framework-actor-rs



```
