# Actor Lifecycle R2 Native Captured Output

Exact canonical `@semio-tech/framework-actor-rs:test --args='--lib actor_instance_lifecycle_wire_ -- --nocapture'`. Tail captured directly from the retained current-run text log; no fixture/source reconstruction.

```text

> nx run @semio-tech/framework-actor-rs:test --args=--lib actor_instance_lifecycle_wire_ -- --nocapture

> bun ./📜️script.ts test --lib actor_instance_lifecycle_wire_ -- --nocapture

warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID f4b91fdd-bec1-4926-8c10-473fe654fdf8 with nextest profile: fundamental
    Starting 3 tests across 1 binary (97 tests skipped)
       START [         ] (1/3) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_matches_shared_independent_leb128_vectors

running 1 test
test component::instance_lifetime::tests::actor_instance_lifecycle_wire_matches_shared_independent_leb128_vectors ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 99 filtered out; finished in 0.00s

        PASS [   0.013s] (1/3) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_matches_shared_independent_leb128_vectors
       START [         ] (2/3) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_rejects_invalid_authority_before_writing

running 1 test
test component::instance_lifetime::tests::actor_instance_lifecycle_wire_rejects_invalid_authority_before_writing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 99 filtered out; finished in 0.00s

        PASS [   0.012s] (2/3) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_rejects_invalid_authority_before_writing
       START [         ] (3/3) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_requires_exact_accepted_identity_before_terminal

running 1 test
test component::instance_lifetime::tests::actor_instance_lifecycle_wire_requires_exact_accepted_identity_before_terminal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 99 filtered out; finished in 0.01s

        PASS [   0.034s] (3/3) semio-framework-actor component::instance_lifetime::tests::actor_instance_lifecycle_wire_requires_exact_accepted_identity_before_terminal
────────────
     Summary [   0.065s] 3 tests run: 3 passed, 97 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-SdMeg5



 NX   Successfully ran target test for project @semio-tech/framework-actor-rs



```
