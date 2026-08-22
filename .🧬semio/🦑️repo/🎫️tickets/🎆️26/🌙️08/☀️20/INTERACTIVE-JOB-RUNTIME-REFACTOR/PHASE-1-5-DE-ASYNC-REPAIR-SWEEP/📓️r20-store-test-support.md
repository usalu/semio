# R20 Store Test-Support De-Async Repair

## Outcome

Seven pure Store law helpers now execute synchronously instead of returning futures that callers could silently discard:

- `assert_dsl_round_trip`
- `assert_config_round_trip`
- `check_dsl_fixture_text_laws`
- `assert_pack_round_trip`
- `assert_dsl_pack_equivalence`
- `assert_op_line_round_trip`
- `assert_op_text_binary_equivalence`

The helpers perform only deterministic in-memory parsing, printing, encoding, decoding, and equality assertions. Their internal helper-to-helper calls and every outer caller await in the non-compose Rust tree were migrated to the value-returning contract.

## Census and audit

The source census covers 1,103 symbol references across 282 Rust files.

- Before: seven `async fn` definitions and 44 outer call-site awaits.
- After: zero `async fn` definitions and zero outer call-site awaits, using a balanced recursive-PCRE call matcher so awaits inside async arguments are not misclassified.
- The mechanical call-site pass initially crossed one doc-comment reference and removed the genuine `print_command(command).await` in the next function. It was restored from a compiler-exact E0599 diagnostic.
- Reconstructing the exact transformation against every helper-bearing file at `HEAD` proves that this command-printer await was the only non-helper await selected by the pass; all other 44 removals were the intended outer helper awaits.
- Scoped `git diff --check` exits 0.

Evidence:

- `📝️r20-store-test-support-census-before.txt`
- `📝️r20-store-test-support-census-after.txt`
- `📝️r20-store-helper-regex-head-reconstruction.txt`
- `📝️r20-store-test-support-diff-lines.txt`
- `📝️r20-store-diff-check.txt`

## Verification

| Gate | Result |
| --- | --- |
| `cargo check -p semio-framework-os-kernel --lib --message-format=json` | exit 0, zero diagnostics |
| `cargo test -p semio-framework-os-kernel --lib demo_dsl_ -- --nocapture` | 2 passed, 0 failed |
| `cargo test -p semio-framework-os-kernel --lib demo_op_text_round_trips -- --nocapture` | 1 passed, 0 failed |
| `cargo check -p semio-s-plugin-stdio --lib --message-format=json` | stopped upstream with 36 E0277 diagnostics, all in framework plugin component test-support code |
| `cargo test -p semio-s-plugin-stdio --lib schema_keys_and_runtime_factories_are_exact -- --nocapture` | stopped at the same 36-error framework-plugin dependency wall before stdio test execution |

The OS gates used the ticket-local `🧪️target-r20-store` target. The stdio gates reused the isolated Phase-9 stdio target and do not contend with the shared workspace target.

Logs:

- `📝️r20-os-kernel-check-2.json`
- `📝️r20-os-kernel-check-2.stderr.txt`
- `📝️r20-os-store-dsl-tests.txt`
- `📝️r20-os-store-op-test.txt`
- `📝️r20-stdio-native-check.json`
- `📝️r20-stdio-native-errors.tsv`
- `📝️r20-stdio-registry-test.txt`

## Blocker

The Store packet itself has zero compiler diagnostics. The representative stdio boundary cannot currently reach stdio because the concurrently changing framework-plugin test-support surface contains 36 stale awaits on newly synchronous byte helpers, all outside this packet's ownership.
