# Staged Dependency Progress Candidate Review

This review covers schema/corpus/feature authoring in progress, before final tests/runtime author milestone. Only filesystem reads and report writing occurred; no Source edit, helper, command, validator or test execution. Runtime/test omissions at this stage are not findings. Final Source hashes are awaited.

## Contract Readback

The pinned decision and interface addendum require a shared 5000 ms frame, eleven fixed slots, observation ranks 0–10, first entitlement per frame, due-slot replacement before drain, original raw/human retention, actual old-scheduler emission barriers, family-specific resets and dequeue-before-first-write. Falsy primary failure identity must survive a failing final flush. Ordering is within a drain, not global callback chronology.

## Current Schema/Corpus/Feature

The dependencyWork policy pins 5000 ms, 11 pending rows, maximum rank10, closed slot/family lists, safe bounded counters, explicit unopened/paced/unclocked states, finite rebase/nonfinite recovery, family reset and sink-failure semantics. Ownership shares a family across seven operations; scan and ledger have independent families; replay requires elapsed zero. The feature explicitly preserves existing schedulers and family-only pending evidence.

The 28 authored neutral vectors cover due replacement; dense all-eleven slot admission; unknown barriers without renewed entitlement; provider/cache/tooling interleaving; family reset retaining unrelated older rows; independent scan/ledger/unit counters; owner/command/Script-stage boundaries; backward and nonfinite clocks; 12 malformed record variants; maximum safe counter; raw/human/due partial write failures; repeated flush; and repeated eleven-slot rank compression. The family-reset vector intentionally produces non-global index order (0,3,4,2,7,1,5,6), matching the interface exception. Three old-family vectors pin that callbacks remaining pending do not flush new rows before actual old-scheduler emission.

No blocking staged contract finding. Some vectors deliberately opt out of the dependency-only SQLite oracle (old schedulers, malformed classification and partial sinks); final tests must still independently derive accepted/attempted indices from row facts and policy for covered scheduling cases and directly check every opted-out fixture against actual writer behavior. This is a review checkpoint, not a claim of execution or completed implementation.

## Final Review Still Required

Read exact final five Source hashes; inspect classifier closure and every existing AST extraction; inspect bounded live rank arithmetic and retained state; verify old scheduler emit barrier placement; verify family counter resets, clock state recovery, observation-time messages and attempted-row removal before either sink write; inspect independent SQLite derivation and Original checkpoints/yields/signal/falsy failure assertions. Retain prior16 test titles plus exact2 new titles.
