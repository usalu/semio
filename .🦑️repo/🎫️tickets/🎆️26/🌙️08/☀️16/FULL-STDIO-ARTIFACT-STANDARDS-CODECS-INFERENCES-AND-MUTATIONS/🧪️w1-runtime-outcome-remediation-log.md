# Runtime Outcome Remediation Gate Log

## Commands

All commands ran from `/Users/ueli/Documents/semio` with an isolated ticket-local Cargo target and `RUSTC_WRAPPER=''`.

| Command | Result |
| --- | --- |
| `cargo test -p semio-framework-os-kernel --lib quarantined_accept_is_atomic_when_a_later_envelope_remains_fatal -- --nocapture` | Passed: 1 passed, 0 failed. |
| `cargo test -p semio-framework-os-kernel --lib empty_store_snapshot_merge -- --nocapture` | Passed: 2 passed, 0 failed. |
| `cargo test -p semio-framework-os-kernel --lib empty_store_snapshot_policy_rejection -- --nocapture` | Passed: 1 passed, 0 failed. |
| `cargo test -p semio-framework-os-kernel --lib decoder_rejects -- --nocapture` | Passed: 3 passed, 0 failed. |
| `cargo test -p semio-framework-os-kernel --lib spr_parse_rejects_history_without_authoritative_operation_metadata -- --nocapture` | Passed: 1 passed, 0 failed. |
| `cargo test -p semio-framework-os-kernel --lib spr_round_trip_preserves_edit_messages_and_conflicts -- --nocapture` | Passed: 1 passed, 0 failed. |
| `cargo test -p semio-framework-os-kernel --lib -- --nocapture` | Passed: 942 passed, 0 failed, 0 ignored. |
| `cargo check -p semio-framework-os-kernel --lib --features sync` | Passed. |
| `cargo test -p semio-framework-os-kernel --features sync --lib wire_fixtures_stay_byte_identical_across_rust_and_ts -- --nocapture` | Passed: 1 passed, 0 failed. |

## Diagnostic Notes

The kernel emits existing compiler warnings and fixture-sweep `[DEBUG] soft-skip` diagnostics for unrelated stdio examples. They did not fail either gate. This remediation added no temporary runtime debug logging.
