# Artifact Plan Neutral Contract Review

Current actual corpus/schema live under process/artifacts/native-build/plan/tests. Eight identity vectors independently distinguish same plan and changes in Source, mode, toolchain, compiler, product, root and configuration. Eight lifecycle rows cover producer, join, distinct plan, consumer cancellation, producer settlement and matching publication. Closed objects/enums/hashes are neutral DATA only; no Original, control, lease or admission token is fabricated or accepted by this fixture. No production implementation or test execution is inferred.

Two actionable gaps sent to backend before production factoring:

- Both root schemas require ^/, excluding native Windows drive and UNC coordinates. Add actual supported canonical root forms and a Windows identity vector, or prove an existing canonicalizer produces the declared portable representation. A POSIX-only synthetic root is not cross-platform input validation.
- cancel-producer-still-owned is currently byte-for-byte the same lifecycle observations as same-plan-joins, aside from id. An explicit producer-cancellation-request observation is needed to demonstrate that cancellation alone is not settled/released custody and cannot authorize takeover. ProducerSettled must remain tied to real lifecycle/custody in eventual implementation, not trusted fixture authority.

Boolean published is understood as an already-matching/rechecked model observation, not permission to consume arbitrary output. Real implementation must still bind actual admitted Source graph/unit.sourceInputs, compiler/toolchain/configuration/product identities and each caller Original/control independently. Schema/oracle success alone will not establish cross-process joining or artifact acceptance. No Source edits, tests or process changes were made by this audit.

## Corrected Corpus and Actual RED Scope

Actual updated corpus has10identity and14lifecycle vectors. Windows drive/UNC slash-equivalent facts are present without case-folding. Lifecycle now separates producer cancellation request, settled completion and released custody; matching/rechecked, matching/unrechecked, stale and foreign publication; failed-but-unsettled versus released failure. This resolves the earlier duplicate cancellation observation and publication boolean gaps. Closed DATA remains authority-free.

Actual RED reports12tests:11pass1fail35assertions. It calls existing cargoTargetRecipeCanonicalV1 using a synthetic recipe that intentionally includes configuration but omits the separate model Source field. Changed-source therefore collides. This demonstrates the distinction between recipe and the proposed full admitted plan; it does not prove a defect in recipe semantics, and changing recipe to accept unproven model Source would not establish admission. Final new plan tests must consume actual captured Cargo graph plus admitted unit.sourceInputs and compare genuine resulting identities.

Current SQLite lifecycle test compares SQL decision against expected DATA only; it does not compare a production lifecycle implementation or prove actual multiprocess joining, consumer cancellation isolation, Original ownership, receipt recheck or settled-custody takeover. Those remain future implementation/physical laws.

A remaining test-only concern was sent backend: root normalization unconditionally performs Unicode NFC in addition to slash replacement. That is not demonstrated filesystem canonical identity and can conflate distinct POSIX names. Use the existing actual canonical root boundary/facts or remove unsupported Unicode normalization; preserve case/Unicode distinctions rather than adding equality by assumption. This concern does not turn the current neutral RED into authority evidence. No Source edits or execution performed by this review.

## First Production Factoring Review

Actual plan owner now projects actual captured CargoCompilationGraph after Source admission plus the existing recipe into descriptive plan facts. It preserves received root verbatim, host platform, mode/product/compiler/toolchain/configuration/source identities; no invented NFC or slash canonicalization is applied in this production projection. Neutral/public provider and repository layer retain same Original/control, compiler store/input/network and normal qualified consumer path. No DevScript coalescing wiring or physical multiprocess proof exists yet.

Two concrete closure blockers reported immediately from native-build/TS withCargoArtifactPlanV1:

1. produce checks live/control/storage and recipe freshness but does not internally recheck the admitted Source graph/plan identity before compilation. A callback can wait while Source changes and then produce newer bytes under its old identity. A compiler validation of newly current Source does not bind those bytes to the earlier admitted plan. Enforce the actual Source/receipt identity within production, not as optional caller recheck.
2. produce is asynchronous and the body can start it without awaiting, then return. The finally only sets live=false; no pending operation roster/drain settles started compilation before outer input/store scopes unwind. Register/drain owned async work before releasing custody, preserve primary error/cancellation and refuse new operations after callback completion. A one-shot flag alone is insufficient.

These findings concern actual authored factoring, not hypothetical coalescing redesign. No Source edits or execution performed by this audit.

## Stable Core Follow-Up

Current Source structurally resolves the two reported blockers. Async plan facts stream actual admitted graph/recipe with finite byte accounting, chunked strings, issued control checks/publication/yields and received root verbatim. Production recheck freshly captures the Cargo graph and re-admits Source, rather than merely hashing the old graph. Produce requires this before recipe lease, after lease acquisition, before qualified consumer and after capture. Completed provenance inputs must match every admitted workspace Source path/digest before original consume.

Started recheck/produce operations now enter genuine issued operation tracking, with bounded pending roster, callback acceptance cutoff, drain before live=false and outer scope release. Body primary failure is preserved before secondary operation failures; newly attempted operations after cutoff are refused. No additional concrete blocker found in the bounded current review. Actual scope lifecycle laws, ordinary semantics and physical multiprocess coalescing remain pending; pure13identity/14lifecycle and public import/parser success are not those proofs. No execution/Source edits performed by this audit.
