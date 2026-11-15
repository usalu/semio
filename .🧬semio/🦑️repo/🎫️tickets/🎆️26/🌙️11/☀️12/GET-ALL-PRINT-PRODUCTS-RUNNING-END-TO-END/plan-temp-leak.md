# Plan temp leak in the per-host graph service

## Lifecycle found
- Writer: `createHostServiceV1.spill()` in `build/resident/service/host/.mjs` writes `<TMPDIR>/semio-repo/<uid>/plans-<tag>/<sha256>.json`, content-addressed. It holds the graph slice of every freshly computed plan (and a task list above `maximumInlinePlanTasks`). A memo hit reuses the slice reference without writing.
- Readers: the requesting CLI (`invocation/plan` `readSliceV1`, `client/plan` `resolvePlanV1`), and every process the invocation starts: the handoff (path, hash, size) goes through the environment (`planHandoffEnvironmentV1`), so child and nested tasks read the file at their own start. The service never learns when the invocation ends.
- Retention before: only a count bound (`planMemoEntries`=256, newest by mtime), run on each spill. No byte bound, no age bound, no sweep at start or on a timer, never removed on exit. With ~275 MB per plan that allows ~70 GB; in practice 66 files = 14 GB.

## Root cause
Count-only retention over very large files, never swept except when a new plan spills. The service was long-lived and nothing bounded bytes or age.

## Fix
- Pure decision `planRetentionV1({files,live,policy})` (new module `resident/service/host/retention/.mjs`): keeps live names and files younger than `planRetainMilliseconds` (6 h: later processes of a running invocation still read them); beyond that removes files older than `planMaximumAgeMilliseconds` (24 h), then oldest unprotected files until total bytes <= `planMaximumBytes` (4 GiB) and count <= `planMemoEntries`; removes stale `.tmp` leftovers; never touches other names. Protected files count toward the bounds, so unprotected ones give way first.
- Host service: `sweepPlans()` runs at start, every `planSweepMilliseconds` (10 min, unref'd timer cleared on drain) and after each spill (the just-written digest is live). A memo hit refreshes the slice file's mtime (reuse extends its life) and, if the slice file was swept, drops the memo entry and re-plans instead of serving a dangling reference.
- Not done: removal on invocation end. The service has no signal for it (descendants read via env); the age window stands in for it. The service is not restarted; the fix takes effect on its next start.

## Files changed
All under `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/`:
- `🏛️service/🌐️host/🟨️.mjs` (sweep wiring, memo guard)
- `🏛️service/🌐️host/🧹️retention/🟨️.mjs` (new, pure retention)
- `🏛️service/🌐️host/🧹️retention/🟨️.d.mts` (new, types)
- `🧬️schema/🔣️.json` (HostPolicy: planRetainMilliseconds, planMaximumAgeMilliseconds, planMaximumBytes, planSweepMilliseconds)
- `📨️protocol/🎚️policy/🟨️.d.mts` (policy keys)
- `🧪️tests/🧫️fixtures/🧹️plan-retention.json` (new language-agnostic fixture, 8 cases: ages, sizes, live refs, bounds, tmp)
- `🧪️tests/🌐️host/🟦️.ts` (fixture test + timer-sweep test)

## Tests
`repo tools exec --light bun -- test ./…/🧪️tests/🌐️host/🟦️.ts`: 32 pass, 1 fail. The failure ("a held connection keeps the idle rule ... retires after it is released") is unrelated to this change: the idle rule is deferred by `runInFlightV1` while other agents' runs hold live capacity grants, so the service does not retire; not re-verified against the unmodified tree (the shared tree cannot be reverted).

## Follow-ups
- Plan size: one slice is `{graph:{projects,dependencies,namedInputs,externalNodes,admission}}`; sampled file (273 MB on disk, ~430 MB when parsed and re-serialized with default formatting): `projects` is 427 MB for 674 projects (~630 KB each), `dependencies` 3 MB (10343 edges), `externalNodes` 0.2 MB. The slice embeds the whole per-project graph, not a minimal slice for the target run, and distinct per-invocation digests prevent sharing. Redesign (reference the sealed graph generation / per-project content addressing, or share projects across slices) is a follow-up.
- `repo doctor` / `repo cache gc`: no storage-area registration mechanism found (searched doctor, caching/environment/inspection, cas); wiring the plans directory into both is not a small change, so not done. Minimum follow-up: add `<TMPDIR>/semio-repo/<uid>/plans-*` as a named area in the doctor storage section and let `cache gc` call `planRetentionV1` on it with a tightened policy.
- The disk gate could name the largest directory it cannot free.
- Ticket not closed by this agent (`ticket_close` is the caller's).
