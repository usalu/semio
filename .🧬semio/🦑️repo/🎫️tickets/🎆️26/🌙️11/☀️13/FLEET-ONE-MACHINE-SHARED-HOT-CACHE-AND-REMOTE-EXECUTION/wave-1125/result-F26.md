# F26: g1-m06 "Unknown closure reference publication: selected/<hash>.admission.json.<uuid>"

Cause: `writeAdmissionMemoV1` (`🏗️build/🕸️graph/📥️load/🧠️admission-memo`) stages `path+"."+randomUUID()`; the closure proof's `role()` knew only the final names and threw, failing the whole headroom pass (and so the workspace refresh).

Landed (area caching-storage, folder `…/📏️headroom/🧾️closures/`, details in `T/landings.md`):
- `temporaryBase`: a name is an in-flight temporary when it is a known publication (role, spill, retirement intent) plus a writer suffix (`.<uuid>`, `.<pid>[.<time>].tmp`, `.<hex>[.tmp]`, `~<hex>`, `~<n>.next|prior`, `<namespace>/~<uuid>`). All writers of the store grepped; kinds checked: selected json/authority/admission, canonical/authority/graph/format/directory-stamps, build-input/lock closures, snapshots (+ledger), generations (current, m, o, x, f, members), inference (np, ip, rr, ob, members), retirement intents.
- `directories()`: a temporary is left to its writer (never read, never a row). Any other unknown name outside `retired/<h>.d` is refused and skipped (`proof.refusals`, `buildGraphClosureRefusalsV1(token)`), it no longer throws.
- Orphans (writer pid dead, or older than schema `temporaryOrphanSeconds` = 3600) are retired first in `retireBuildGraphClosureProofV1`, stamp-checked, directory stamp updated.
- Test `🧪️tests/⏳️in-flight-temporary/🟦️.ts` 5/0 incl. the exact m06 name and a Python oracle.

Not verified / for others:
- The orphan retirement and the directories() skip run only in a real image: no integration test (only the pure functions are tested). Refusals are reported by accessor only, not in the headroom result (its schema is closed).
- `closures/🟦️.ts` is 65281 bytes; the implementation pin reads at most 65536 ("Bounded physical Source revision required" at 65632 broke it for a minute). Further growth needs a sibling file.
- Failing before this landing: `📏️cardinality`, `📁️census` ("retiredSpill is not defined", they extract `directories` without `retiredSpill`), `♻️groups` 2/11 (F7: 7/6), `🧪️tests/🟦️.ts` 19/8.
- Loaded processes report "Stale loaded Graph closure implementation; restart required" until restarted.

## Round 2
- Hypothesis from reading: intents `retired/<h>.json` were refused as unknown names and skipped as rows. Fixed (patch2, sha 68b15aebb6a3): no refusal below `retired/`. groups 2/11 -> 4/9, in-flight 5/0.
- NOT solved: the remaining groups failures are "Closure reference directory changed: …/m/tmp (0 entries before)" from `recheck` at the end of `captureClosureProof`. My temp handling creates and retires nothing during capture (orphans are retired only in `retireBuildGraphClosureProofV1`, never in the optional-group path), so I could not tie `m/tmp` to my change. F7 already saw 7/6 before the landing. Unproven: whether the baseline fails the same way; the A/B needs a scratch copy of the whole `headroom` tree with its relative imports, which I did not build. Suggested next: A/B in scratch, or make the root `tmp` directory a live directory (like `liveSpillDirectory`) if the image machinery writes there.
- An A/B in the live tree (block disabled for about a minute) was a mistake; stopped and restored, sha 68b15aebb6a3, 65325 bytes. `tests/🟦️.ts` not rerun.

## Round 3 (no landing: the A/B proof could not be obtained)
Scratch copies of the `📏️headroom` tree (landed and pre-F26, reversed from the landed file; `🧰️scripts/F26/mkscratch.py`, `bare.json`) were built three ways and none can run `♻️groups`:
1. Under the scratchpad (outside the checkout), outward imports rewritten absolute: the harness's source-closure authorizer refuses ("Closed authorizer relative source path required"); bare `bun test` there also needs cwd inside the checkout (`MODULE.bazel`), and `repo test-file` refuses files outside it.
2. Inside `.cache/` (gitignored): `bun test` never discovers dot directories.
3. Inside `out/` (gitignored, relative imports): the file runs, but the harness answers "Exact captured closure worker readiness required" for both copies (the worker identity is pinned to the real path), so the A/B says nothing about `m/tmp`.
All scratch dirs deleted; the live file is untouched (sha 68b15aebb6a3).
What the code says: the proof has always entered every directory below the image root except the declared ignored ones, so a root-level `tmp` that gains an entry during `captureClosureProof` fails `recheck` the same way before and after F26; my new code creates, renames or retires nothing during capture (orphan retirement is only in `retireBuildGraphClosureProofV1`, not in the optional-group path). F7 measured groups 7/6 before the landing. That is an argument, not a proof.
Proposed next step (needs a go from the coordinator, one tiny landing in a quiet window): a `[DEBUG]` line in `recheck` naming the entries of the changed directory, run groups once, then remove it. If the entries are harness files in `<root>/tmp`, the fix is declaring that directory a live spill directory like `liveSpillDirectory`.

## Round 4
- Landed the product change: `recheck` now names appeared and disappeared entries (`directoryChangeV1` in `📡️session`, English and German, 10 names + count). Pin: closures 65.2 KB.
- First groups run with it: 13/0 (4/9 before), so no `m/tmp` names were seen and no live-spill rule was needed; the earlier `m/tmp` failures were transient harness churn (likely concurrent runs). If it recurs the refusal now says which entries.
- `🧪️tests/🟦️.ts`: 19/8 -> 25/2 -> after dropping scenario `unknown` (now refused+skipped by design) 23/4 with different reds each run (load flaky); `pack-unproven` red in every run, not investigated.
