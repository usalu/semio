# R69: g1-m04 `tools` stage, `Exact selected dependency fields required`

Status: **staged (round 2), not landed, exact offending row of g1-m04 still not seen; the plan now survives it structurally.** Package: `…/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R69/` (`land.py`, `stage/`, `closure.py`).

## Throw site (proven)

`captureJavascriptDependencySelectedFieldsV1(value, keys?)` in
- `LIB/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (line 191)
- `LIB/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🏷️roles/🟦️.ts` (line 6, a copy)

It throws when a plain DATA row has more than 16 fields or, when `keys` is given, a different field set than `keys`. The message named neither the row, the missing field nor the unexpected one. About 40 call sites use it, so the log alone cannot tell which row it was. The literal exists nowhere else.

## What the log and the code prove about where it happens

- Stage `tools` runs `repo tools setup all`. After the provider check it runs `retireObsoleteRepositoryDependencyGenerationsV1` (`💾️storage/🔒️enforcement/…/📤️publication/⚙️setup/♻️retirement/🟦️.ts`), which calls `planSelectedRepositoryDependencyGenerationsV1`. That plan publishes `dependencies:selected-plan:<n>` once per candidate directory of the dependency store `j/` (`captureRepositoryDependencySelectedPlanV1`), in prefix order (capsules `.p…` first, then 20-hex generations).
- g1-m04 prints 1..5 and then the error, every time (round 24, 27, 31 notes and 8 attempts, identical). A Mac (g1-m08, `workers/g1-m08/round-16-tools-setup-console.txt`) prints 1..6 and goes on to the next ownership walk. So the failure is deterministic and depends on what is in the worker's own dependency store, not on the lock file or on `process.platform`.
- Nothing in the selection/ownership code reads `process.platform`, `os`, `cpu` or `libc` (grep of `🗂️workspaces/🟦️bun`, the storage module and the retirement module: only producer recipe backends and link-target helpers use the platform). The "optional/os/cpu/libc" hypothesis has no support in this code path.
- Sites that run per candidate and take data from the store (so differ per machine): the unready intent `i` of a retained capsule (`javascriptDependencyUnreadyRetentionIntentV1`, 16 exact fields) and the ready receipt `bindings.json` of a retained generation (`readJavascriptDependencyRetainedGenerationDataV1` → `readJavascriptDependencySourceOwnerReadyV1`, 11 exact fields, member rows of 5, binding rows of 2, owner references of 2, singleton rows of 3). Either the 6th candidate, or a step right after the 5th, throws.
- I could not reproduce it: the failing store lives only on g1-m04 (no access from here), the Macs pass, and driving the selection code with `platform=linux` on the real `bun.lock` changes nothing because the platform is not an input of that code. Most likely cause (hypothesis, not proven): a retained candidate of the Linux store (the worker went through 24+ rounds of failed producers at older code versions) holds a receipt or intent with a field set different from the current exact set, and one such candidate aborts the whole plan instead of being named. The retirement design says no legacy receipts, so the right fix depends on what the row is; guessing a "classify as obsolete" rule without seeing it would be a compatibility layer on a guess.

## Change staged (diagnosis, so that the next attempt on g1-m04 names the row)

1. `captureJavascriptDependencySelectedFieldsV1` (both copies, same text): the error now reads `Exact selected dependency fields required: missing [...], unexpected [...], found N of M allowed` (at most 16 names each). The old prefix is kept.
2. `describeRepositoryDependencyCandidateV1` (new export, publication owner, doc-emoji 🔬️): `captureRepositoryDependencySelectedPlanV1` wraps the intent read (`i`) and the receipt read (`bindings.json`) so the refusal reads `Retained dependency generation <prefix> file <file>: Exact selected dependency fields required: missing …`. That is the "which package/which field" answer for this site (the row is a retained generation, not a package; no package field is involved here).
3. Test `💾️storage/🧪️tests/🧾️field-diagnosis/🟦️.ts` (4 tests): intent with a missing and an extra field, observe command without `expected`, 17 fields, the wrapped plan error.
   - Before (live files via a bun preload mapping): 0 pass, 4 fail. After (staged files): 4 pass, 10 expects.
   - No existing test asserts the old message (grep over the library).
- User-facing texts: these are internal `Error` messages (English only, like the 40 neighbours), not entries of the bilingual caching texts; I did not add DE text. Open question for the owner: if they should become `refuseCachingV1` texts, that is a larger change at 40 sites.

## Tool closure verdict (decides landing)

`closure.py` (static import walk from the entries of `🔌️provider-closures/🔣️.json` `tools.closures`, over-approximates dynamic imports) finds that `💾️storage/🟦️.ts` and `🧹️retirement/🏷️roles/🟦️.ts` are reachable from the closure **`driver-stages`** (2348 files). A landing would re-stale every package record on the fleet, so I did **not** land. `land.py --dry-run` is clean (3 modified files at the staged base, 1 new test). `land.py --land` additionally refuses unless `R69_CLOSURE_OK=1`. R51's real preflight test (`🛃️preflight`) only checks generated imports, it does not list membership, so it was not used for this verdict; it is in `land.py`'s test list.

## Not verified

- The actual offending row on g1-m04 (needs the improved message, or a copy of its `j/` listing and the receipts).
- That the error comes from the 6th candidate and not from a step after the 5th (the staged wrapper only covers the two reads in the candidate loop; if the throw is later, the new field list still names the row but not the prefix).
- The static closure verdict is an approximation; the group read's own walk is the authority.
- g1-m04 attempt 9 was running (tools, backoff state) while I worked; no change of mine reached it.

## Next step for whoever owns the closure decision

Accept the re-stale and land with `R69_CLOSURE_OK=1 python3 land.py --land`, or hand the 3 file edits to a landing that is anyway in `driver-stages`. After g1-m04's next failure, its `ready-log-tools-failed.txt` names the prefix, file and fields; then fix the root cause (retire or reject that exact kind of row) with a law on that row.


## Round 2 (coordinator: the plan must survive a stale retained generation)

### What step `selected-plan:6` reads (from the code)
Candidates are the entries of `j/` sorted by bytes, so `.p<8 hex>` capsules come before 20-hex generations. `selected-plan:<n>` is published after candidate n is fully read, so a stop after 5 means candidate 6 threw while being read. What the read of a candidate does:
- a capsule `.p…`: lists its roles; with `p` present it reads the intent file `i` through `javascriptDependencyUnreadyRetentionIntentV1` (16 exact fields);
- a generation with `bindings.json`: `readJavascriptDependencyBindingReceiptReadV1` then `readJavascriptDependencyRetainedGenerationDataV1` -> `readJavascriptDependencySourceOwnerReadyV1` (11 exact fields, version 4; rows of 5, 2, 2, 3 fields).
The Mac prints `selected-plan:6` and goes on, so all six of its rows match the current sets. I cannot tell from the code whether candidate 6 on g1-m04 is a capsule or a generation; the law covers the receipt read (the structural cases: one missing field, one extra field, wrong version) and the intent read keeps its named hard error (below).

### Change
- `storage/🟦️.ts`: new `JavascriptDependencyRowMismatchV1` (carries `missing`, `unexpected`); thrown by the field-set capture and by the version/domain check of the ready receipt.
- Publication plan (`captureRepositoryDependencySelectedPlanV1`): a retained generation that is NOT the prefix of a `current.scopes` generation and whose `bindings.json` throws that mismatch is classified `unready` (generation and fingerprint null, roles listed) instead of throwing; it is therefore in the retire set (`selectRepositoryDependencyRetirementCandidateV1` picks it). A bilingual line `dependency-store.stale-generation` (EN+DE, level warn, texts file) names prefix, file, missing and unexpected fields. A mismatching receipt of a current generation stays a hard error, prefixed with prefix and file (`describeRepositoryDependencyCandidateV1`).
- Retiring such a generation had to be allowed by three safety checks that treated `bindings.json` as proof of readiness: `unreadyPayloadRoles` gets `bindings.json` (retirement schema), the fresh admission no longer refuses it for `unready-generation`, and the never-ready role reader accepts it at the root of an `unready-generation` (still refused inside a retained-unready capsule payload). Deletion already removes `bindings.json` last, and the maintain pass re-plans with the same classification, so an interrupted retire resumes.
- Fixtures: unready roles corpus (`unready-generation-ready`, admission `ready-receipt` now accepted for `unready-generation` only) and its SQLite oracle updated.

### Proof
- New law `⚙️setup/♻️retirement/🧪️tests/🪦️stale-generation` (2 tests, real `bindings.json` files in a temp store, the real receipt reader and plan function extracted from source, real retirement candidate selector): store with one current generation plus one whose receipt has an extra field and misses `singletons` -> plan succeeds, current is `active`, odd is `unready` and chosen as the retire candidate, the diagnostic line names prefix, file, `["singletons"]`, `["platform"]`, EN and DE texts exist; the same receipt on the current generation throws `Retained dependency generation <prefix> file bindings.json: Exact selected dependency fields required: missing ["singletons"] ...`.
- Before the change (live files): the plan throws `Exact selected dependency fields required`, both tests red. After (staged files via bun preload): 6 pass (this law plus the 4 `🧾️field-diagnosis` tests). Retirement busy-driver and roster tests with staged files: 21 pass. Unready roles corpus against the staged role reader: 22 rows, 0 mismatch (the SQLite test file itself needs `typescript`, absent here). Texts test: with staged files only the "every message is used" check is red until the plan file lands (the key is used only there); the placeholder checks pass.

### Not covered / not verified
- A capsule `.p…` with payload whose intent `i` mismatches stays a hard error naming prefix and file: retiring it needs the intent as identity (pin, `retained-unready` selection), so it cannot be retired without a new protocol. If g1-m04's candidate 6 is such a capsule this change only improves the message.
- The real retire of a stale generation through the native owners (observe then retire, delete with `bindings.json` last) was not run: the retirement driver is rehearsed with model owners, the maintain pass reads the payload through native custody. The payload reader/role reader changes are verified on the role corpus only.
- Still NOT landed; `land.py` (9 files, `R69_CLOSURE_OK=1` required, because storage `🟦️.ts` is in `driver-stages`) is ready for the combined batch; `--dry-run` is clean at the current live bases.
