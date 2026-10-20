# 🛠️ Handoff: x-ingress follow-up (review items X-I1, X-I2, C1, X-I3, X-I6)

For: peer fleet (integrate). From: executor x-ingress. Reviewer: `review-release-ingress.md` (aud-review1). Base: the x-ingress set (`handoff-ingress.md`) is applied live; this follow-up is staged only, NOTHING was edited live (git status of the generation directory shows only the peer's own apply). Tools: `T/x-ingress-tools/followup/` (`T` = this ticket folder).

## Apply

```
python3 T/x-ingress-tools/followup/apply.py --check     # read only; rc 1 = edits pending, rc 0 = applied, rc 2 = refused
python3 T/x-ingress-tools/followup/apply.py --live      # two-phase
sh T/x-ingress-tools/followup/check.sh <scratch dir outside the repo>
```
`apply.py` is two-phase (X-I6): phase 1 reads all 4 targets, requires the x-ingress base set to be present, requires every anchor to occur exactly once (or the edit to be present already) and computes every new text in memory; any refusal prints `REFUSED, nothing written` and exits 2 with the tree untouched; only then phase 2 writes each file through a temporary file and `os.replace`. 13 operations on the live tree today, all anchors found. Idempotent (a second `--live` changes nothing). Six tests in `test_apply.py` pin it (check is read only; live then idempotent; one missing anchor, an ambiguous anchor, a missing base set and a missing target each refuse the WHOLE set with identical sha256 before and after, compared with `shasum`).

## What it changes

| Review item | Change | Test |
|---|---|---|
| X-I1 replay failure containment | new pure `guardGenerationReplayV1` (core); `replayJavascriptDependencyGenerationWalkV1` becomes a thin guarded wrapper over the renamed `…UnguardedV1`; any capture/verify error except a cancelled run returns `{status:"deferred",reason:"capture-failed",path:<message>}`, so `confirmGenerationWalkV1` falls back to the complete walk and the shared pass still throws "Current parser Source-owner custody changed" if the tree changed. Previously one capture error (schema, evidence capacity, missing topology) failed `ensureBuildEnvironmentV1` for every command. | fixture `guard` (6 rows, Python oracle), `decision` (6 rows: ready only when the confirmed sha equals the owner's, including a throwing replay with an unchanged and a changed tree), real replay with a walk whose bindings evidence is missing -> `capture-failed`, then `confirmGenerationWalkV1` walks |
| X-I2 settle start | `generationWalkOriginV1(sharedStartedAt, passStartedAt)` = the earlier of the two; the exclusive pass now computes `walkOriginAt=generationWalkOriginV1(cached?.startedAt,walkStartedAt)` before `cached` is cleared and passes it to the replay and to the tail publish (the shared pass is unchanged: its own start). | fixture `origin` (4 rows, oracle `min`), real replay: with the shared start the provider written 1 s ago is `provider-observation-unsettled`, with the later pass start it would have been accepted; source laws on the bootstrap |
| C1 evidence cap | generation schema `maximumEvidenceBytes` 134217728 -> 67108864 = `RelativeCreateProtocol.maximumFileBytes`; a capture above the cap now throws "Generation evidence capacity exceeded" (contained by X-I1) instead of failing mid write. | the test reads the descriptor schema with Python (`followup.oracle.py cap`) and requires equality with the generation protocol |
| X-I3 raw unlink | the files writer remembers the one leaf it was creating; after a failure (the failed scope retains the leaf and refuses further children: "Live relative child admission required", reproduced) it removes that leaf through a FRESH descriptor scope (`entry` + `remove`), swallowing cleanup errors. The raw `unlinkSync` of the leaf is gone (retire keeps its own, as before, precedent noted by the reviewer). | failed write leaves nothing (existing x-ingress test and a new one), rewrite of the same name succeeds byte-identically (`shasum -a 256`) |
| X-I6 apply | see above. | `test_apply.py` |

Not changed, with reason: X-I4 (the best-effort publish runs before the shared pass returns "ready"): it needs the store open and costs seconds on a miss only; moving it after readiness would hold the shared custody longer for no gain, so it is accepted. X-I7 gaps closed: replay that throws, exclusive pass start, ready only with a confirmed sha. Still open: no test covers the Nx path through `ensureBuildEnvironmentV1` (shared code, same fallback). The three receipts for one scope the reviewer saw at 16:25-16:29 are expected: the shared publisher retires with a 10 minute grace (`sharedRetireGraceMilliseconds`) so a verifier that read an old receipt keeps its evidence; they disappear on a later publish.

## Verification (scratch mirror outside the repo, `sh check.sh`)

- follow-up suite 7 pass 0 fail; x-ingress suite 10 pass 0 fail (its failed-write law now exercises the descriptor removal); `test_apply.py` 6 OK; peer generation suite 11 pass 1 fail, the one failure is the pre-existing `projectListing(` count law of the peer's cjs (4 instead of 2), identical in the unpatched mirror.
- `tsc --noEmit` over bootstrap, core, glue and the new test: no diagnostic in the touched files.
- Mutation checks (each fails the suite): wrapper calling the unguarded replay, evidence cap back to 128 MiB, origin ignoring the shared start.
- Test-first: the new suite failed before the edits (`Export named 'generationWalkOriginV1' not found`).
- Not run: the patched bootstrap against the real Store (quiet tree, peer `show projects` running).

## Order

After the peer's current closure edits settle (any closure edit costs one walk and one deferral): `--check`, `--live`, `check.sh`, then two consecutive `bun repo --tool=bazel run @teaching/architecture-course:test --dry-run` under `T/x-ingress-tools/timed.ts` (second run: verify only).
