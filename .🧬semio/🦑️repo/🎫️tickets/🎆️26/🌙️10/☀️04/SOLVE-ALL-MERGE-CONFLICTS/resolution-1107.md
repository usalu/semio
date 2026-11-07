# Resolve Stash Conflicts On 2026-11-07

Ticket `26/10/04/SOLVE-ALL-MERGE-CONFLICTS` reopened under goal `🎯consistentrepohistory`.

## State

Branch `🐙ueli/⛳wip` is not in a merge or rebase. The index had 303 unmerged paths from a stash pop: `Updated upstream` is index stage 2, `Stashed changes` is index stage 3.

Upstream already uses current crate names (`semio_framework_*`) and current retirement/receiving signatures. The stash still uses the previous names and signatures (`semio_plugin`, `RetainedCloneGrant` close steps, `RetainedWorkOutcome`).

## Decision

- When every stash function and `use` path already exists upstream, the resolved file is stage 2. That keeps the current API and does not drop unique upstream code. 137 Rust files were written from stage 2 and `git add`ed. None of them still contain conflict markers.
- When the stash declares function names or imports that stage 2 does not have, those items are ported onto the upstream file. Same-named items stay on the upstream body.
- Cargo manifests keep stage 2 dependency names. A stash dependency is added only when its path exists and stage 2 has no entry for that same path under the current package name.
- Lockfiles follow the resolved manifests. A lockfile whose manifests match stage 2 is restored from stage 2.
- Modify/delete paths where stage 2 exists keep stage 2. Paths deleted by stage 2 stay deleted unless the stash file is the only copy of code that is not present anywhere in stage 2.

## Batch

Remaining work is listed in `batches-1107.json` (71 Rust files with stash-only symbols) and `rest-1107.txt` (manifests, locks, TypeScript, JSON, WIT, and modify/delete paths).

Subagent model slug `Grok 4.7 Non-Fast Medium` is not in the session catalogue. Agents inherit this session's Grok 4.7. The fast slug `grok-4.7-high-fast` was not used.

## Value rest

All 11 value-rest files are stage 2, staged, and free of conflict markers. Stash-only names (`retirement`, `take_value`, the paged-text helpers, `staged_receipt`, `output_published`) are absent from `HEAD` as definitions, including `encode_with_inline` and `write_encoding_format`. Call sites for those two names already exist on `HEAD` outside this file. The resolved blob is `HEAD` (`f97d668bdbae1381bfdf21a1be266e99301b6a62`). Unmerged count after this batch: 106.

## Other

All six `other` files are staged and marker-free. `jit_budget`, `begin_close`, `decode`, and `encode` stayed dropped. The job context had dropped `consume_receiving_retained`, `receiving_issued_grant`, `receiving_child_progress`, `settle_receiving_scope`, `replacement_capture`, `replacement_is_stale`, and `with_replacement_current` even though callers still use them. Those methods and their fields are restored on the current `StepContext`, and `with_native_receiving` now records the issued grant. `with_native_decode_progress` stays the existing re-export.

## Manifests

All 47 paths in `rest-1107.txt` are staged and free of conflict markers. `framework/Cargo.toml` gained the try-map, job-replacement, and tests-20 members. The persistent-list laws gained `closeLaw` and `readLaw`. The reactor WIT gained `stage-command-page` and `stage-cold-pair-page`. The arc-custody test and the I/O refusal receiver stayed deleted. `framework/Cargo.lock` has no conflict markers and does not yet name `semio-framework-job-replacement`. `cargo metadata` stops in the repository compiler ingress before it can refresh that lock. Unmerged count after this batch: 54.

## OS batch 1

All 15 os-1 files are staged and marker-free. The space-history work-loan test is present in the resolved snapshot tests. Unmerged count after this batch: 39.

## OS batch 0

All 12 os-0 files are staged and marker-free. The resolved tree contains `member_open_request_ignores_the_stepping_caller_identity_and_follows_the_slot_identity`, `close_blockers_debug`, and `birth_identity`. Unmerged count after this batch: 27.

## Value decode

All 12 value-decode files are staged and marker-free. The resolved sources contain `spent_bytes`, `install_work_recipient`, the array `source` projection, `fragment_probe`, and `dynamic_map_keys_require_an_actual_borrowed_name`. Schema helpers such as `fixed`, `invalid`, `receipt`, `nested`, `child_grant`, and `add_receipt` stay re-exported from `semio_framework_value_native_schema_engine` through `owned`'s `tree` module. `git diff --diff-filter=U` reports no unmerged paths after this batch. The 15 os-2 paths are also staged and free of conflict markers in the index.

## OS batch 2

All 15 os-2 files are staged and marker-free. The resolved sources contain the four original host wake treasury tests, `close_rung_now`, and `whole_receipt`. Dropped stash names have no remaining callers: host-event routing delegates to `semio_plugin_instance_dispatch`, and the cold-job file keeps the upstream test `portable_frontier_respects_protocol_events_before_output_ack`. A scan of framework and hub sources found no conflict markers. `framework/Cargo.lock` still does not name `semio-framework-job-replacement` because the repository compiler ingress refuses a raw `cargo metadata` refresh.
