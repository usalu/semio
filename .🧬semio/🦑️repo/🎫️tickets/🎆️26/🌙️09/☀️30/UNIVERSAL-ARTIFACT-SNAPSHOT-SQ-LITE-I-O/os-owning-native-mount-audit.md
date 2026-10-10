# OS Owning Native Mount Audit

Read-only source inspection on 2026-10-10. No build, test, or runtime behavior was executed or confirmed. Source can change concurrently.

## Actual mounted selection

The real library owner is `semio-framework-os-kernel`, declared in `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:3`; its Rust root mounts OS IO at `🦀️.rs:228–229`. OS IO mounts the shared Core control implementation at `🔨️modules/🚪️io/🦀️.rs:2871–2872`. Core control mounts its snapshot at `🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs:3`, the ordinary control laws at 126–127 and nested laws at 129–130. Snapshot mounts its tests at `⏱️control/🛫️snapshot/🦀️.rs:60–62`.

Thus `-p semio-framework-os-kernel --lib original_receiving_ -- --nocapture` selects these six source-defined laws:

- `io::control::nested_io_tests::original_receiving_nested_io_decode_retains_child_ticket_and_same_wallet`
- `io::control::nested_io_tests::original_receiving_nested_io_encode_retains_child_ticket_and_same_wallet`
- `io::control::nested_io_tests::original_receiving_nested_io_dual_control_phase_change_refuses_without_foreign_ledger_work`
- `io::control::snapshot::tests::original_receiving_terminal_child_quotes_actual_header_before_box_release`
- `io::control::snapshot::tests::original_receiving_canceled_publication_retains_exact_io_error_fields`
- `io::control::snapshot::tests::original_receiving_publication_header_denials_retain_exact_output_backing`

These are test-name selection claims from mounted source, not a runtime listing. The ordinary control test file contains no `original_receiving_` function. The selector does not select CSV's `original_subset_validation_typed_destination_keeps_original_children` law, whose actual name differs and whose owning binary differs.

## Allocation observer boundary

OS library root installs `test_allocation::RequestedAllocator` once at `🦀️.rs:33–38`; the six selected laws call that crate-root observer. SQLite snapshot is an external dependency and is reexported at `🦀️.rs:233`, not source-mounted. Its own `#[cfg(test)]` global allocator belongs to its separate library test binary. OS Cargo's `sqlite_snapshot_native_admission` and `artifact_command_retirement` are separate integration binaries at Cargo.toml:22–28, each with their own allocator; `--lib` excludes them. Plugin SDK/host allocator declarations found by repository search belong to separately owned crates and must not be counted as OS library duplicates merely because their source lives below the OS directory. No additional global allocator was found in the directly inspected OS IO or Store root. This is a source graph qualification, not compiler proof.

At inspection time GUI launch and seed still named `@semio-tech/framework-rs:test-snapshot-receiving-native` at `.vscode/launch.json:95203` and `.vscode/🧩️launch.seed.jsonc:78340`. Both higher capability `arguments` arrays also contain the old target. Moving the visible command therefore requires moving the capability argument exactly with it, and selecting the OS owning Cargo manifest/package together. Original artifact directory, CARGO target/offline environment and independently authored policy must remain original rather than be inferred from the selected test.

## Original canceled continuation

CSV `snapshot/🧪️tests/🦀️.rs:215–224` passes the original observer into `new_forwarded`, retains the original allocation port and recipient, and immediately unwraps denied/funded close calls. A permanently canceled observer therefore cannot exercise an inspectable canceled-close continuation in this helper: close refusal panics. The helper must receive an explicit caller-owned between-operation-and-close continuation hook (or expose custody to the caller). That hook can rearm a caller-owned `Cell<bool>` captured by the very same observer; no replacement observer, native control, recipient, or wallet is needed.

The mounted Core canceled-publication law already demonstrates the correct ownership pattern at `⏱️control/🛫️snapshot/🧪️tests/🦀️.rs:25–30`: caller captures `live`, operation sets `live=false`, original publication refuses and retains custody, caller sets `live=true`, then closes through the same original native control and installed recipient. For a stronger helper law, first call close while still canceled, assert its refusal and unchanged custody/backing/receipt, then invoke the explicit original caller continuation and fund closure. A wrapper that substitutes `|_| true` after cancellation would erase the actual caller capability.
