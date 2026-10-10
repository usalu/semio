# Current Preparation Generator Review

Read-only hash cut in 📥️cargo-preparation-generator.json; no tests/native execution.

Shared preparationPlan retains dependency-first traversal, visiting/finished cycle control, exact script+argv deduplication, post-recipe cache invalidation and final closure re-discovery. Executed key is marked before child acquisition, but any failed child aborts the whole plan, so no current successful retry skips that failure within the same call. Final closure still detects changed unexecuted recipes. Named-package selection uses mutable selectedPackages after current correction; no separate duplicate algorithm observed.

Concrete recipe custody hole: controlled recipe branch calls runOwnedCommand directly and does not invoke current directory/physical guards on operation.path or operation.cwd. The generator previously validated script via physicalPlan, but its final state advance is awaited; a continuation may replace script or cwd ancestry before recipe spawn. Raw readFileSync(script) for before hash and synchronous capture occurs on resume, also outside the reported physical operation stream. Recheck physical script and cwd immediately before spawn, and bind actual current source hash to the captured before value. A symlink-swap at final script state is a meaningful original test, with no recipe marker/native child.

Program capture and resolver custody remain synchronous child calls; recipe await does not make those stages cancellation-responsive. Source observation parsing/currentness after recipe also performs synchronous raw reads and resolver subprocesses. Preserve actual custody identities, but route those defining operations through original control where expensive.

Recipe progress currently onProgress prints console.error only, so long running recipe does not advance original control's progress/check/yield port except after completion; original signal is forwarded and owned tree/drain is genuine. Connect child progress to admitted original control without fabricating physical-completed counters or allowing machine stdout contamination.

Final result has no fresh relevant-currentness fence after final physical advance; cached documents can precede that await. Preserve pair-level stable source/manifest/roster/recipe checks before publication/native update, and define what prepare result promises. Add final-continuation manifest mutation refusal only if current snapshot semantics are claimed. No global freeze or reduced dependency inputs.
