# Transaction Command Close Six-Law Native GREEN R3

## Actual Outcome

**6 passed, 524 skipped**, 0.159s, Nx exit0. Every exact six-law selector from R2 executed in the fresh compiled binary; no abort. Source holds released immediately after terminal. The exact two command-release cases now report Pending/1 item/1 byte; zero and short bytes preserve both owners with Blocked/0/0. Before-begin and zero-items remain Blocked/0/0.

The actual command value layout is one byte, so short and zero-byte rows both use zero. This proves neither allocator overhead nor completion allocation/final-owner retirement. No full Plugin, timing, or publication credit. The original R2 2PASS/4FAIL remains preserved.

## Source Boundary

[867 selected nested inputs](./📓️txn-command-close-r3-selected-inputs-2026-08-28.md), not an atomic complete closure. Actual transaction parent `88d8ac707cec6330e09ff76af2d94599c46f245f9be38093244b90e74e3757e3`. Unchanged native test `15438e324d90b46ea3b9964c93a7c980bdd2825b04d7f15a989857802b6f470f`, fixture `e3f6b8a4c7236e52f17bbd0e637599a09730af718cad6c53d354cffebd118398`, schema `ad076a4cf63443b31143082f68f19baadf3ba4ca5d224dc487e9b801529d9848`.

Same retained target/jobs2/exhaustive/no-fail-fast; no budget, stack, thread or profile changes. Nx printed a flaky-task advisory after its successful footer; no repeat run was substituted.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive txn_command_close_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-txn-command-close-six-r3-2026-08-28.md'
```

## Complete Captured Tool Output

```text

> nx run @semio-tech/framework-plugin:test --args=exhaustive txn_command_close_ --no-fail-fast -- --nocapture

> bun 📜️script.ts test exhaustive txn_command_close_ --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] plugin-runner-oracle cases=6
────────────
[32;1m Nextest run[0m ID [1mfae89772-c486-427c-8c1c-28c171b64baa[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m6[0m tests across [1m1[0m binary ([1m524[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_external_completion[0m

running 1 test
[DEBUG] txn-command-close id=exact-external-completion commandBytes=1 grantItems=1 grantBytes=1 step=pending releasedItems=1 releasedBytes=1
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_external_completion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.045s] (1/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_external_completion[0m
[32;1m       START[0m [         ] (2/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_pending_completion[0m

running 1 test
[DEBUG] txn-command-close id=exact-pending-completion commandBytes=1 grantItems=1 grantBytes=1 step=pending releasedItems=1 releasedBytes=1
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_pending_completion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.031s] (2/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_pending_completion[0m
[32;1m       START[0m [         ] (3/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_requires_begin_close[0m

running 1 test
[DEBUG] txn-command-close id=before-begin-close commandBytes=1 grantItems=1 grantBytes=1 step=blocked releasedItems=0 releasedBytes=0
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_requires_begin_close ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.025s] (3/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_requires_begin_close[0m
[32;1m       START[0m [         ] (4/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_short_bytes_preserves_owners[0m

running 1 test
[DEBUG] txn-command-close id=short-bytes commandBytes=1 grantItems=1 grantBytes=0 step=blocked releasedItems=0 releasedBytes=0
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_short_bytes_preserves_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (4/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_short_bytes_preserves_owners[0m
[32;1m       START[0m [         ] (5/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_bytes_preserves_owners[0m

running 1 test
[DEBUG] txn-command-close id=zero-bytes commandBytes=1 grantItems=1 grantBytes=0 step=blocked releasedItems=0 releasedBytes=0
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_bytes_preserves_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (5/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_bytes_preserves_owners[0m
[32;1m       START[0m [         ] (6/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_items_preserves_owners[0m

running 1 test
[DEBUG] txn-command-close id=zero-items commandBytes=1 grantItems=0 grantBytes=1 step=blocked releasedItems=0 releasedBytes=0
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_items_preserves_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (6/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_items_preserves_owners[0m
────────────
[32;1m     Summary[0m [   0.159s] [1m6[0m tests run: [1m6[0m [32;1mpassed[0m, [1m524[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-HxDB3i[0m



 NX   Successfully ran target test for project @semio-tech/framework-plugin



 NX   Nx detected a flaky task

  @semio-tech/framework-plugin:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

