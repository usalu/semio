# Transaction Command Close Native Selection R1

Canonical explicit no-run inventory succeeded,25.53s,Cargo/Nx0. No behavioral test ran in that command. Actual resulting executable was then listed through workspace Nx exec; its selector contains exactly the6 authored tests below. Existing default test build profile; behavioral run uses exhaustive/no-fail-fast in the same retained target/jobs2.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive --no-run txn_command_close_' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-txn-command-close-inventory-r1-2026-08-28.md'
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' txn_command_close_ --list --format terse
```

```text
component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_external_completion: test
component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_pending_completion: test
component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_requires_begin_close: test
component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_short_bytes_preserves_owners: test
component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_bytes_preserves_owners: test
component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_items_preserves_owners: test

```

Executable: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8`. Full inventory stream is [rawR1](./🧪️member-txn-command-close-inventory-r1-2026-08-28.md); the direct polling output was truncated and is not presented as complete. A checked chunked copy will preserve the complete raw inventory separately. [867 selected source inputs](./📓️txn-command-close-r1-selected-inputs-2026-08-28.md) match Mutation's four exact released hashes.

