# 2026-10-12 raw suites: `test-retained-raw-source` red on four TS suites

Executor: raw-suites (Sonnet). No git-modifying command, no ticket operation, no sub-agent, no Rust or product TypeScript edited (release build #9 running). Only TS oracle files under `🧪️tests` were edited. Rust ran in the foreground chain of `🚦️cargo-slot.sh` (`SLOTS=2`) with a private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/raw-suites/target`; that directory, the two copied test binaries and all logs were deleted at the end.

## Result

| suite (alone) | before | after | verdict |
| --- | --- | --- | --- |
| `🧵️retained-command/🧪️tests/♻️raw-allocation-close` | 1 pass / 1 fail | 1 pass / 1 fail | PRODUCT is wrong, oracle and Rust law agree with each other |
| `🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests` | 3 / 1 | **4 / 0** | oracle obsolete, rewritten |
| `♻️lifecycle/🏪️stores/🧪️tests` (also imports `🧬️context/🧪️tests`) | 3 / 5 (8 tests with the imports, 3 / 5 failing) | **8 / 0** | oracle obsolete, 5 tests repaired |
| `♻️cancellation/📦️lease/🧪️tests` | 3 / 1 | 3 / 1 | 2 of 3 stale assertions repaired, the third is a PRODUCT defect (read from code) |
| `🎟️admission/🧪️tests` (same list) | 2 / 0 | 2 / 0 | untouched |

`bun test` on a repo-relative path crashes Bun 1.3.14 (segfault while scanning the repo from the cwd); the script passes absolute paths, so did I.

`NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-plugin:test-retained-raw-source --skip-nx-cache`:
- before: exit 1, 8 failing tests (raw 1, frontier 1, context 1, stores 4, lease 1) in the one `bun test` over the five files, so the `tsc --noEmit` step after it never ran.
- after: exit 1, **2 failing tests**: `raw retirement preserves metadata copy zero and independent original backing release` and `yielded window refresh keeps the original mutation and receipt before ACK`. Both are product findings below; the oracles were deliberately not weakened.
- standalone `tsc --noEmit` (the script's flags) over the lease and frontier test files printed no diagnostic.

## Findings that need a product decision after build #9

### 1. raw-allocation-close: the retained command job drains initialized raw bytes through COPY (product deviates from fixture v3, the TS oracle and the Rust law)

Evidence (run): the Rust law `component::plugin_runtime::plugin_builder_contract_tests::retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement` is red with the binary built from today's tree: cases `empty-zero-capacity` and `empty-large-capacity` pass, case `partial-large-capacity` (8193 initialized bytes, capacity 65536) panics at `🧵️retained-command/🦀️.rs:961` `assert_eq!(job.next_close_copy_byte_demand().unwrap(),0)` with `left: 1`. The same red is in `pl-tests/r10..r15`, `fw-tests/plugin-each`, `wire-src`, `typed-close/plugin-run`: it has been red since the law was introduced (git `cede6fc227b`), never fixed.

Cause: `ArtifactRetainedCommandJob::close_demands` answers `copy_bytes:1` for a non-empty `raw`, and `close_step` truncates `raw` by `min(len, grant.maximum_copy_bytes)` charging `copied_bytes`. A `Vec<u8>` has no drop work: this is a logical drain of a flat backing, which is exactly what correction #63 (String retirement copy 0, one deallocation) says must not exist. It also costs `ceil(len / copy grant)` turns per command (32 turns for 1 MiB at the shipped 32 KiB copy grant), and with a zero copy grant the step returns an empty receipt forever (the unfundable-demand shape of #72a). The fixture (`maximumCopyBytes: 0`, `expectedCopiedBytes: 0`), the TS oracle and the Rust law all say copy 0.

Why not an oracle update: updating fixture and TS to the product would leave the Rust law red, and the law's hard-coded `==0` lives in `🧵️retained-command/🦀️.rs` (`#[cfg(test)] test_raw_allocation_close`), a product file build #9 compiles.

Proposed patch (NOT applied, NOT run), `🧵️retained-command/🦀️.rs`:
- `close_demands`: `if !self.raw.is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}`
- `close_step`: `if !self.raw.is_empty(){self.raw.clear();return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}`

These are the exact strings the TS oracle greps for; `clear()` keeps the capacity, so the following release-of-capacity branch (unchanged) still frees 65536 in one step, which is what the law asserts.

### 2. lease: the mounted WindowConfig lane has no re-entry for a yielded refresh (read from code, NOT run)

The oracle asserts `if let Some(receipt)=mounted.pending_window_config_receipt`. The product (`🔌️plugin/🦀️.rs`, `publish_mounted_typed_operation_unit`, `TypedOperationResultLane::WindowConfig` arm) stores `mounted.pending_window_config_receipt=Some(receipt)`, calls `window_config_store.refresh(authority,grant)` and on `WindowAuthorityRefreshStep::Pending(_)` does `return Ok(())` without queueing the result page. `🪟️window/🎚️config/🦀️.rs` `refresh` ALWAYS returns `Pending(copied_items:1)` on its first call (`pending_snapshot` starts `None` and is only captured there). On the next unit the publication is `AwaitingAck`, `advance_apply_batch` answers `AwaitingAck(receipt)` and the host maps that to `Err("artifact-store publication awaits an ACK without its retained host page")`. Nothing reads `pending_window_config_receipt` on re-entry (its only other readers are `retirement_demands`/`retirement_step`, i.e. after the operation already failed). Same code at HEAD. Real editors declare this lane (gis, cad, layout, equation, puzzle2d, flow). No plugin lib law drives a mounted typed operation through this arm (`a_published_window_config_closes_through_its_own_published_physical_demand` uses `dispatch_emit` and is red at baseline for a different reason: `snapshot registry is unavailable after exact owned close`, store `🦀️.rs:749`).

Fix direction: before `advance`, consume the retained receipt: if `pending_window_config_receipt` is `Some`, continue `refresh` and queue the page from the retained receipt; then the oracle goes green with no change.

## Oracle repairs (each checked against the current Rust text; the greps are source twins, so each is a structural claim only)

1. `🧵️retained-command/🧬️context/🧪️tests` (stores imports it): the job's `context_retirement` field was removed by pub-perf (#72, root cause 2); the context is now retired by `input_retirement` through `store::artifact_retirement_admit_owned(&mut self.context,&mut self.input_retirement,grant)`. The oracle now asserts that call, `context: Option<ArtifactOwnedContextHandle<A>>`, `&& self.context.is_none()`, `&& self.input_retirement.is_none()` and the absence of `context_retirement`.
2. stores, "transient detach": the replacement-free detach lives in `TransientStore::begin_close_retirement` (takes `current`, `reads`, `active_returned_read`, no `Self::new`, no `mem::replace`); the disposer calls it and checks `close_terminal_is_empty()`. `begin_retirement` (replace with a terminal root) is a different, unused API. Oracle rewritten to those.
3. stores, "rejected read return": `try_return_original_snapshot_read(&mut self.owner, &mut self.lease)` has one call site (`SnapshotRead::try_return_to_registry_witness`), the erased read returns through `try_into_typed`; count 2 -> 1 plus an anchor on the method (count already 1 at HEAD).
4. stores, "export source handback": the draw export `Work` builds with `Work::new(request.snapshot_read,birth_grant(),handoff_grant())` and passes `self.birth_grant` to `io_begin_owned_serialization`; `request.retained_birth_grant` no longer exists.
5. stores, "registered owned serializers": the request no longer carries `retained_birth_grant` / `retained_handoff_grant`; the host owns `fn artifact_owned_tool_birth_grant()` / `..handoff_grant()` (4 and 2 occurrences). Asserted, and the removed fields asserted absent.
6. frontier, "mounted publication source captures before close": it grepped `PendingPublicationOutcome` machinery that nothing ever sets (phase stays `Uncaptured`, `-1 < index` passed vacuously, and `terminal_fault.get_or_insert(report)` is not in the file). The live mechanism is `reject_cancelled_publication`: first statement of the publish unit, `pending.begin_close()` before `queue_page`, fault page from the existing `terminal_fault` else the cancellation lane, retirement only after the page ACK (`granted_retirement_ready` requires `result_page.is_none()`). Test renamed and rewritten to that.
7. lease, "yielded window refresh": two assertions were obsolete and are updated (`window_config_mutations.push(mutation)` pending arm -> `window_config_store.admit_begin(...)`; `copy_bytes:std::mem::size_of::<store::LaneItemReceipt>()` -> with the space rustfmt left). The receipt re-entry assertion is kept (finding 2).

## Rust laws run (plugin lib and os-kernel lib built into the private target; one process per test, 6 parallel, 150 s alarm)

- plugin `--lib`: 332 laws matching raw/admission/frontier/retained_command/window/transient/read_return/lease/cancell/publication/export/serializ/context/store: 252 ok, 79 FAILED, 1 EXIT133. All 80 non-ok were already non-ok in `typed-close/plugin-run/results.tsv` (05:56 today): **0 new reds**. Green and relevant: `component::retained_command::admission::tests::retained_raw_admission_has_no_failure_destructor_or_undeclared_allocation` (fixture v2 agrees with the TS oracle), `metadata_retirement`, `checkpoint_cursor`, `component::app::tool_run_tests::original_mounted_child_frontier_closes_after_actual_ack_under_original_policy`, `component::window_mutation::tests::original_window_refresh_admits_displaced_snapshot_under_actual_currencies`.
- os-kernel `--lib` (150 laws matching read/return/registry/transient/lease): 134 ok, 13 FAILED, 3 TIMEOUT, all already non-ok in `fw-tests/kernel-each`: 0 new reds. Green and relevant to repairs 3-5: `retained_read_return_retirement::tests::original_read_return_refusals_preserve_payload_and_lease`, `original_erased_read_return_retains_source_on_every_refusal_and_returns_the_same_registry`, `registered_owned_serializer_rejections_preserve_exact_read_and_options`, `store_close_releases_a_returned_read_before_its_displaced_root`, `store_close_waits_for_a_live_snapshot_read_then_retires_its_returned_owner`.
- No Rust law consumes the context fixture; the draw crate (repair 4) was not built, so its own tests did not run.

## Other things noticed (not fixed)

- `component::transient_retirement::tests::empty_transient_retirement_waits_for_read_release_and_matches_neutral_vectors` is red (baseline too): neutral vector wants `heldReadBlocks: true`, the disposer returns `heldReadBlocks: false`. With a read held, `TransientStoreRetirement::close_step` answers an empty `Progress` instead of `Blocked`: a silent spin, the #72a livelock shape. Product or vector, owner of the transient disposer decides.
- Dead product code: `PendingPublicationOutcome` (frontier `🦀️.rs`, never leaves `Uncaptured`; still referenced by `🧪️tests/♻️publication-retirement-authority` and two struct literals), `PendingArtifactStorePublication::retirement_turn` with the `PendingArtifactStorePublicationRetirement` enum (no caller), `TransientStore::begin_retirement`. The TS twin `PublicationOutcomeCursor` + its corpus (frontier test 3, passing) models the removed capture/deliver machinery and has no Rust counterpart.
- `RetainedRawScript` native law names are `app::plugin_builder_contract_tests::retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement` and `retained_command::admission::tests::retained_raw_admission_...`; `--list` prints `component::plugin_runtime::plugin_builder_contract_tests::...` and `component::retained_command::admission::tests::...`. Whether `runRepositoryExactCargoLaws` resolves the shorter names was not run (`test-retained-raw-native` untouched), so it may select zero laws.
- Draw `handoff_grant()` is `{items 1, depth 1}` while the host's `artifact_owned_tool_handoff_grant()` funds 64 copy bytes for the alias handle: duplicated, not equal.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️context/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🏪️stores/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/📦️lease/🧪️tests/🟦️.ts`
- this report. Not touched: raw-allocation-close test and fixture, any Rust or product TS, `🗑️generated`.
