# Native Fuzzy Matching Allocation

Genuine private Go751312 ended1/null09:06:11UTC: short repeated-candidate vector allocated73984bytes, exceeding32768. All4neutral vector scores matched before the optimization, including64byte boundary,65byte fallback and UTF8 byte distance.

Production now keeps one private stack distance scratch per document score, reuses it across candidate words, and retains dynamic allocation for longer rows. Width derives from existing maximum fuzziness; this is scratch capacity with unchanged semantic fallback, not a new refusal or policy limit. Query normalization, byte distance, score, conjunction, cancellation and all size guards remain. Word splitting is deferred until a fuzzy candidate comparison is actually needed. No parallelism, shared cache, GC tuning or fingerprint shortcut was added. Green and independent SQLite oracle acceptance remain pending.

Actual focused green756344 ended0/null09:08:34UTC, all4score/allocationvectors. Whole actual private Go module and independent SQLite oracle have been started. No walltime/nativequery acceptance claim. Existing neutral fixture DATA already belongs to the canonical search test-oracle closure; no globalclosure/Sourcepolicychange was required.

Scratch green756344 measured512candidates24832bytes versus73984;64byte row512bytes;65byte dynamic fallback37376bytes unchanged. Full actual private Go759967 accepted0/null09:09:29UTC with36top-level tests, no skips/failures. Independent SQLite759475 accepted0/null09:08:50UTC. These receipts qualify the scratch checkpoint.

A fifth neutral case now requires4096candidate words to remain within8KiB temporary allocation, targeting the remaining eagerly materialized word slice. The standard library [FieldsFuncSeq](https://pkg.go.dev/strings#FieldsFuncSeq) preserves FieldsFunc token splitting while yielding words without constructing the slice; actual pinned Go compilation will verify availability. New causal red is pending before this separate small change.

The streamed-word causal765642 ended1/null09:12:43UTC after all4prior score/allocation vectors passed;4096candidate word allocation exceeded8KiB. Production now uses standard-library FieldsFuncSeq with the identical predicate and unchanged scratch/dynamic fallback. Added mixed candidate lengths to exercise scratch reuse across different words; independent SQLite byte-alignment oracle now computes the best distance across all tokens. A complete Go module proof is running, with no test filter.

Final full module772696 accepted0/null09:20:06UTC with36top-level Go tests,0failures/0skips. All6neutral candidate vectors passed, including mixed lengths and4096words. Exact allocation observations:

- zz_semio_3e97354869298814_test.go:858: matched 512 candidates; allocated 0 bytes
- zz_semio_3e97354869298814_test.go:858: matched 32 candidates; allocated 0 bytes
- zz_semio_3e97354869298814_test.go:858: matched 32 candidates; allocated 36864 bytes
- zz_semio_3e97354869298814_test.go:858: matched 32 candidates; allocated 0 bytes
- zz_semio_3e97354869298814_test.go:858: matched 4096 candidates; allocated 0 bytes
- zz_semio_3e97354869298814_test.go:858: matched 8 candidates; allocated 0 bytes

Independent actual SQLite773716 accepted0/null09:19:46UTC. Public canonical786729 remains live after admission, separate from private-module/oracle acceptance. Installing the qualified Source to the container next; no native cold/warm performance acceptance yet.

Public full graph786729 ended0/null09:25:41UTC:actual Nx test-oracle9tests plus Go testtarget both successful (1prerequisite). This qualifies the repaired compiled-package/Rustup/publicsearch route, separately from remainingnativequerywalltime. Finalinstall813603 ended0/null09:20:58; receipt827948 ended0/null09:26:10:20391279B executable0755 SHA256ca6f1473311b2dc284d5804cfef6223be34b537e207c7b6f96da6d06059357ec and receipt0600. No SourceGo editsafterthisqualification.

## Formal Native Gate

Formal authoredSource/report/inputhold09:41–09:47UTC, explicitrelease09:47:37. Native843585 startedexactly09:41:00.001 afternormaladmissionat09:39:04. CourseRootreceivednotice09:39:11; allCourseownershadcontinuousREADYholds, but their finalACKprecisetimestampsare boundedonlybefore09:41:02, notindependentlyproven90snoticeforall. No intentionalauthoredwritesreported.

Gateended1/null09:43:55:lateidentitycancel54.857s cooperative0stdout; physicalsnapshotcomparisonpassed. Coldqueryforced120141ms0stdoutlastpersisting-verified62/64. No warm/completionretry. Existing fuzzyallocation/parityandpublicrouteacceptance donotqualifythiswalltimegate. RemainingworkisthefullrepositorySource/projectionpipeline andfreshphysicalreadadmission; no budgetraise/GCtuning/guardremoval.
