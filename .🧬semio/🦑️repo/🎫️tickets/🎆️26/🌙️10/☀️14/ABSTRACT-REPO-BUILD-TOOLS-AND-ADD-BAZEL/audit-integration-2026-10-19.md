# Integration Audit — 2026-10-19

## Result

No reproducible P1/P2 correctness defect found in the audited entrypoint, IDE routing, selected Bazel inventory, or unpublished birth retention changes. This is bounded source/integration review, not qualification of native Nx/Bazel execution.

## Runtime Validation Actually Run

- `bun test ./🧪️tests/🧭️entrypoint/🟦️.ts`: 3 passed, 39 expectations.
- `bun ./📜️script.ts test entrypoint`: 3 passed, 39 expectations; logs show loading/running/complete for the original workspace command.
- Bazel selected inventory suite: 1 passed, 1 skipped, 18 expectations. Real filesystem/cancellation case is intentionally skipped without SEMIO_TEST_ARTIFACT_DIR.
- Dedicated birth suite without SEMIO_TEST_ARTIFACT_DIR: 2 failed on the explicit artifact-directory harness precondition. No storage behavior failed in that invocation; no storage success is claimed.
- Native backend remains unqualified because Mount-DiskImage needs privilege on this host, per the storage execution report.

## Source Review

- Root repo route calls the build CLI public entry (`📜️script.ts:168-171`) rather than wrapping RepoScript in a second original caller. All remaining domain command classes are lazy. The entrypoint test verifies runtime help for both backend selections and an esbuild-independent eager-import boundary.
- New test registration exists as workspace `test-command-entrypoint`, root `test entrypoint`, and IDE Command Startup Tests. Birth suite is imported by the fixed aggregate at line 2. No omission found in those registrations.
- IDE build/run/test/dev/task routes pass backend and configuration explicitly. Referenced input ids are present. The bounded entrypoint paths are comfortably below 256 code units under this workspace.
- Bazel inventory roots eliminate redundant traversal prefixes. Final task fingerprints filter collected paths by each task's own included/excluded patterns, so conservative directory traversal does not by itself add unrelated sources to a fingerprint. The new minimatch oracle validates selected fixture outputs.
- Windows file publication stages and retains originals through nonreplacing descriptor-relative renames, then validates named identities and bytes after close. New code checks bounded leaf paths, custody and cancellation. Full native failure-path coverage remains dependent on the issued artifact test harness.
- Unpublished birth retention refuses a published receipt, foreign roster, active association, changed backing/ancestry or preexisting retained destination. Windows rename uses MoveFileExW with WRITE_THROUGH and without replacement. Physical identities are rechecked after the rename.

## Remaining Qualification

Run the issued public native/cache and birth test harness with a ticket-owned artifact directory, then rerun backend integration on a host allowed to mount the fixed image. Raw Bun tests lacking that harness must not be counted as native acceptance.

## Final Follow-up: Windows Synchronous Reads and Platform Admission

Reviewed the Windows synchronous read helper in the filesystem descriptor owner, its test registration, current source-owner store platform admission schema, and the storage host-admission report. No concrete memory, handle-lifetime or undefined-primary defect found. The nested synchronous descriptor scopes preserve thrown undefined through discriminated Outcome values, invalidate receiver access before closing, retain unresolved close state, and avoid unloading the DLL after unresolved descriptor closure. Reads validate original identity before and after the bounded native call.

Actual follow-up validation:

- Native Windows synchronous file capability test: 1 passed, 9 expectations. Debug output confirms exact fixture bytes and closed handles. Tests used the existing ticket-generated artifact directory; their temporary folder was removed by their own finally block.
- Updated root entrypoint suite: 4 passed, 41 expectations, including developer command loading without the unrelated application gateway.

The source-owner schema explicitly admits win32 alongside darwin/linux as implementation support. This does not establish successful mounted backing production. The storage report states the current process lacks SeManageVolumePrivilege and that the public dependency sync now refuses before cache generation effects. Full mounted backing remains unverified.
