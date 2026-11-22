# Verify Open Path and Natural Source Ingress

Chain: `verify-open-chain.sh` (sequential `g5-g7-retry.sh`, logs `🗑️generated/vo-*.log`, `vo-chain.done` on end). The repo capacity gate (compiler image headroom) queues each compile for tens of minutes behind other agents.

| Test | Package | Result |
|---|---|---|
| `original_store_close_work_charges_every_unmetered_physical_receipt` (G2) | semio-plugin-store-stage-laws | PASS |
| `original_disposer_frame_has_its_own_physical_release` | semio-plugin-store-stage-laws | FAIL first run: the test binary did not link the allocator owner, so `observe_heap_allocations_on_this_thread` saw 0 bytes against receipt 392. Fix: `semio-framework-value-testing-allocator` dev-dependency, `use ... as _;` in the test root, Cargo.lock entry. Rerun `vo-stores2.log` pending |
| `original_transient_detach_allocates_no_replacement_and_retires_exact_frames` | semio-plugin-store-stage-laws | FAIL first run, same cause; rerun pending |
| `original_guest_failure_debit_interrupted_output_retirement_holds_the_exchange_until_terminal_empty` (G1/G21) | semio-plugin | pending |
| `natural_file_publication_checkpoints_follow_the_contract_quantum_not_every_pack_slice` (G3) | semio-plugin-surface-fixture | pending (first attempt hit a stale lock file, now fixed) |
| `natural_` laws incl. `natural-source-limits-tests.rs` (G6) | semio-plugin-job | pending |
| instance-jobs, instance-render, instance-natural-publication lib tests (G4/G16) | respective crates | pending |
| `natural_file_route_*` (three large-image/vector/type1 tests) | semio-s-artifact-stdio-pdf-viewer | pending (previous failure: "tool proof catalog must exactly join migrated generated declarations to live concrete factories") |

Verified by inspection: `👷️job/🏛️reserved/🦀️.rs` still holds the ordinal 22 `import-natural-source` factory, `ReservedCommitMetadata`, `setActiveTool`, `setActiveUtility` and the `natural_source_limits_tests` module.
