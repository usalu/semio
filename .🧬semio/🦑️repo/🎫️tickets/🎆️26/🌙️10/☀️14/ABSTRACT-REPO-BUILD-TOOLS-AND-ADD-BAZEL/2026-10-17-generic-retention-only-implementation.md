# Generic Ordinary Retention Without Binding

The existing ordinary filesystem receiver now exposes `retainPreservation(expected, intent)`. It consumes only an actual same-live-root `capturePreservation` expectation for a regular file or directory, pins the original descriptor, creates an exclusive `.p` plus eight intent-SHA hex capsule, durably writes `i`, and atomically moves the exact original basename to capsule `p` with no replacement. It durably writes `r`, syncs both directories, and returns frozen nominal completion DATA with the now-absent source name, final parent seven facts, and the retained seven-fact/hash record. This grants no Native, installer, binding, readiness, adoption, inactivity or later absence authority.

## Closed Authority and Failure Behavior

The existing 32 combined expectations, 32 descriptor owners, 256-unit paths, minimum12-unit source basename, 4096-byte exact UTF8 intent/receipt and one-capsule-per-original-root rules remain. Copied, foreign, nested, escaped, consumed, concurrently pending or later operations refuse. Existing Windows preservation capture and the new Windows retention method explicitly refuse before physical operations; NTFS retention is not implemented or qualified.

Darwin uses the actual fixed-signature `renameatx_np` with `RENAME_EXCL=4`; Linux uses `renameat2` with `RENAME_NOREPLACE=1`. Unsupported operations refuse without a fallback. There is no unlink, rollback, old capsule reuse, payload traversal or old file-content read in the operation. Original descriptor custody spans intent, move, moved-inode proof and durable receipt; original and moved seven facts must match except nondecreasing own-rename ctime. Unexpected moved or recreated objects remain retained and refuse completion.

The first real Generated positive exposed APFS directory link counts: moving a regular file reduces the old parent's nlink by1 and increases the new parent's nlink by1. The independently executed Python ctypes syscall confirms this for both file and directory moves. The closed schema permits paired Darwin file delta0/1 (Linux file0; directory1), requires the same chosen opposite deltas for the two parents, exact complete rosters, unchanged dev/ino/mode, monotonic timestamps and unchanged retained nlink/size/mtime. It does not separately choose unrelated deltas.

A private same-frame final-roster record exists only after exact successful move/record verification and original descriptor/library close. The existing outer exclusive-create owner still rechecks actual full parent roster and exact capsule seven facts after child close and after parent sync. The record is cleared on all frame settlements. Default creation deltas remain unchanged. New fault laws exercise foreign insertion at the pinned source close, actual capsule close, and final parent sync; none returns a genuine completion.

## Actual Source Evidence

Schema-first missing-method semantic RED is saved as `🗑️generated/bazel-planner/retention-only-semantic-red.log`. Transient parser errors during implementation were not counted as semantic RED. The strict final gate is `retention-only-final-strict.log`: genuine finite Source helper, TypeScript strict on the actual owner/test/helper roots with zero diagnostics, then three passing laws with751 assertions. The schema has62 portable cases:61 actual POSIX fixture cases plus the explicit Windows early-refusal case. Eight cases allow outer success; one is accepted unawaited capture with forbidden late mutation and seven complete retention.

The actual POSIX owner AST executes genuine Bun system-library syscalls on this agent's fresh Generated fixtures. Fault hooks wrap real operations only. Cases cover regular files, opaque nested directories and symlinks without following them, external hardlinks,20-hex generation names, partial writes, zero writes,4096-byte intent, original/record/capsule/library close ambiguity, exact arbitrary undefined/frozen primary and distinct close failures, cancellation before/after move, no-replace collisions, parent/leaf drift, copied and foreign proofs, shared capacity, unawaited drain and late foreign/source recreation. Successful moved seven facts and payload test bytes are independently matched by Python `os.lstat`/hashlib; SQLite checks accepted classification and durable intent-before-move ordering; Ajv validates portable inputs and closed receipts.

Old suites ran on the final production bytes:

- Preservation:4 laws589 assertions, `retention-regression-preservation.log`.
- Strict symbolic binding and actual Windows-owner ABI model:3 laws7239 assertions, `retention-regression-binding.log`.
- Exclusive creation:5 laws4726 assertions, `retention-regression-create.log`.
- Relative read/remove/path and synchronous descriptor laws:7 laws193944 assertions, `retention-regression-relative.log`.

Together with the new three laws this is22 laws207249 assertions. All completed naturally with status0. The old41 method bodies have identical ordered AST-body SHA values (`retention-prior-method-preservation.json`); the prior19 read/write/remove method surfaces are included. Only the adjacent new operation and narrowly private creation-close verification were added. No new Source input file or runtime dependency was introduced; closure counts/caps are not reasserted from these tests.

## Reproduction and Binding

Use the existing generic helper launcher with `stdio-phase --relative-retention-strict`, or the Source-only equivalent with `SEMIO_WORKSPACE_ROOT` equal to workspace and `SEMIO_TEST_ARTIFACT_DIR` equal to this ticket's Generated directory. No new script or launcher format was added. Existing helper modes still select the old suites.

Exact five input paths/bytes/SHA and stable nofollow FD/name seven stamps are in `🗑️generated/bazel-planner/retention-only-five-manifest.json`. The independent two-kind actual ctypes ABI receipt is `retention-python-rename-oracle.json`. The Source contract/model review precedes implementation in `2026-10-17-generic-retention-only-contract.md`.

No protected asset, Native image, installer, existing workspace payload or Source binding was operated on by this agent. Native use remains for the parent-controlled domain issuer after independent review. Result absence is an observed completed-operation fact only; the domain must immediately recheck actual current state under its own live custody before fresh Core staging. Direct alias exclusion and whole funding remain domain responsibilities. Actual Windows and Linux filesystem runtime qualification are not claimed from the Darwin tests.
