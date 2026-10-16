# Independent Native Preclaim Metadata Ordering Audit

Completed 2026-10-16T15:41:40.436837+00:00 as a read-only Source/evidence successor to `audit-native-funded-preclaim-successor-current.md`, which is preserved. No runtime tests, Native/provider/image/mount/compiler/cache/effect operations or Source edits ran.

## Result

No actionable defect found in this four-file successor scope. The prior P1 missing entry-parent metadata sync is resolved in Source and supported by the retained actual forwarded FileHandle.sync observations. This is an ordering conclusion, not a power-loss or actual Native constructor qualification.

## Fix Verification

At `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧾️claim/🟦️.ts:35`, creating the child directory now awaits `syncCreatedEntryV1` before appending the directory claim. Creating child/m awaits that helper before appending the root claim. The new private helper at line 37 takes the recorded parent identity and newly observed entry identity; it checks both dev/ino/mode before and after syncing the actual containing directory. Its metadata-owner root stays the captured dirname(request.directory), and the existing helper checks same-filesystem bounded ancestry and actual no-follow descriptor identity. Thus the root-stage path syncs request.directory itself, fixing the precise omitted barrier in the original audit.

The writer/control/cleanup checks remain those captured by scope.check at line 32. The helper invokes that checker around identity and metadata operations. No fresh control, arbitrary callback result, serialization field or new backend authority is introduced. Existing claim-file fsync and containing-parent sync at line 33 remain after each append. Unknown/partial states, mismatched identities/modes, extra entries and escaped callbacks continue to refuse rather than adopt paths.

At line 33 an explicit failed boolean still distinguishes success from a thrown undefined, drains pending operations, and selects descendant failure only if no body failure was captured. The new awaited helper therefore propagates its own failure while preserving the existing first-body-error/undefined rules. The inherited Error/undefined fixtures remain in actual receiver test line 59. No change to the canonical child allocation association or journal cleanup path was included; those retain the limitations in the original audit.

## Independent Evidence Review

Claim protocol metadataOrder and language-neutral creatorMetadata rows describe ordered, late-root-sync and missing-root-sync cases. Retirement tests line 75 independently compare these through Ajv exact output and SQLite positional ordering.

The observation helper at retirement tests line 72 wraps the actual FileHandle prototype sync, invokes the saved real method on the original handle, observes fstat identity and bounded actual claim bytes, and restores the original descriptor in finally. It checks real directory identity and claim stage, not merely a function name or callback Boolean. The isolated fixture at line 59 carries the actual Original/control and SQLite writer proof. The parent at line 60 requires the child result and forwards exactly one trace.

Retained RED log `🗑️generated/native-preclaim-metadata-red-prototype.log` records SQLite ordering expected 1 / received null and the genuine failed isolated creator result. Both final logs (`native-preclaim-metadata-test-prototype2.log` and `native-preclaim-metadata-test-release.log`) show the exact actual trace [directory-parent-sync, directory-claim, root-parent-sync, root-claim], six isolated status=0/signal=null records, 33 passing tests and 848 assertions. Both strict logs are empty; their actual top-level exit 0 is attributed to checkpoint handles 75996/24052, since an empty log alone does not establish status. No test was rerun by this reviewer.

## Freeze And Limits

Initial and final independent SHA-256 reads matched all four changed-file hashes and all 49 refreshed observed closure hashes (53 listed rows with repeats). No broader-roster drift was observed during this review, despite concurrent Root/Nx work; this asserts no repository-wide freeze or stability beyond the listed roster.

Actual provider constructor and compiler Volume funding remain unwired. This does not establish durable backend success, failed-birth handoff, physical teardown, power-loss recovery, Windows support or whole-cache retirement. POSIX fsync invocation/order and live observed identities are the evidence boundary. Real OS/filesystem crash behavior was not tested.
