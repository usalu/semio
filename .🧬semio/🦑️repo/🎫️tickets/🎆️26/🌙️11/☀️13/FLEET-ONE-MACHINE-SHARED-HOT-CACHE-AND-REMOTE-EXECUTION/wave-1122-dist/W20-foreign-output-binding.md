# W20: foreign package output binding on g1-m08/g1-m10, and a stage waiting on stale gate code (2026-11-25, 09:31 to 10:52)

`A` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/`, `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, `R` = `B🌐️remote-execution/🛰️sidecar/🚦️ready/`, `O` = `/Users/Shared/semio-store/v1/origins/`. Work folder `W20/` (`closure.py`: import closure of the service revision files; sha files). No sub-agent, nothing restarted by hand, no scratch build, one test file at a time.

## 1. The binding refusal: a lost receipt, proven
- W17's diagnostic appeared on g1-m08 at 09:57 (`O/g1-m08/ready-log-workspace.txt`):
  `…/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🟦️typescript/dist is a link -> /Users/ueli/Documents/semio/.cache/b/p/m/a/ts/e6e47bd925ccef535dc6774fe84e3921/prototype/out; receipt absent`.
- The link text is exactly the protocol output of this owner (`a/ts/<sha256(owner)[0:32]>/prototype/out` under the products store). The receipt `a/ts/<hash>/binding.json` lives in the store, the links in the package folder. g1-m10 also logged `[store-receipt] re-executing @semio-tech/norm-din4108:build:prototype: invalid (size .cache/b/p/m/a/ts/e6e47…/prototype/out)`: its store content of that owner was gone too. m08/m10 are the two workers with small, later-born products images (14,335 / 9,215 MB; m08's store free space jumped 7,307 -> 11,370 MB in its logs). Store content and receipt went, the package links stayed, and `bindPackage` refused every attempt because ownership was read from the receipt only (`A🟦️.ts`, old `old=plan.links.map(...)`: `!prior` => refuse).
- Fix (`A🟦️.ts`): new pure `typeScriptPackageLinkModeV1` decides which mode an existing fixed link is bound to: the receipt's mode when it records one, otherwise the mode whose private output the link text names exactly (`dist`: either mode, `📤️dist/<mode>`: that mode); anything not a link stays foreign (the W17 message is unchanged for those). `bindPackage` uses it; an own link whose output is gone (dangling) is replaced, an output that exists must still be the link's realpath (otherwise "Foreign package output target preserved and refused"). Rollback restores the same protocol target. The receipt is rewritten by the bind as before, so readers work again.
- Language-agnostic test: 12 `links` cases in `A🧪️tests/📋️plans.json` (schema `A🧪️tests/🧬️schema/🔣️.json`), checked against the TS function and the Python oracle `A🧪️tests/🐍️oracle/🐍️.py` (`link_mode`).

## 2. g1-m10's stage on stale gate code: would the supervisor end it? No (two defects), fixed
Measured from `O/g1-m10/ready*.json`, its log and the code:
- **Stall clock never ran.** The wait line `Reading the project graph slice (N s): waiting for host capacity (...); 2 active grants, 1 queued requests; the gate reports 4015 MB available ...` changes its megabytes every 2 s. `normalisePatterns` strip seconds but not MB, so `readyStallTrackerV1` saw a "new line" every few seconds = progress for 7+ h.
- **Code-change restart never fires** for this: it only follows the two `semanticsSources` pointers (`R🌲️process-tree/🧬️schema/🔣️.json`), and a capacity-decision edit is not one. The step-overdue rule does not match the wait line. The only clock left was the progress ceiling: 24 x 3,600 s = 24 h.
- **The graph service never reloaded either**: its identity `hostServiceRevisionV1` hashes the hand list `HostPolicy.serviceRevisionFiles` (`B🏡️resident/🧬️schema/🔣️.json`). The list did not name `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts` (nor the gate), and its process-capacity entry `../../../../../../🔨️modules/…/📡️sampling/🟦️.ts` was one folder too high (hashed as "absent" forever; the test never checked existence). So W17's decision landing changed no service identity; the waiting slice read stayed on the old `session-cap` rule. `W20/closure.py`: the listed files' import closure has 42 host-capacity modules, 2 were listed.

Fixes:
- `R🌲️process-tree/🧬️schema/🔣️.json` `normalisePatterns` + `waiting for host capacity \(.*` and `warte auf Rechnerkapazität \(.*`: a gate wait is one line whatever the gate reports; docstring of `readyNormaliseLineV1`; fixture case in `R🌲️process-tree/🧪️tests/🔣️.json` (TS + Python oracle).
- `B🧭️invocation/🛰️plan/🟦️.ts` new `hostServiceRevisionPathsV1`: a `serviceRevisionFiles` name ending in `/` stands for every .ts/.mjs/.json below it outside `🧪️tests`/`📖️texts` (UTF-8 byte order); the two capacity entries are now the folders `../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/` and `🌡️host-capacity/`. `B🧭️invocation/🔄️reload/🟦️.ts` syntax-checks the same expanded list. Test `B🏡️resident/🧪️tests/🔖️revision/🟦️.ts`: Python oracle walks folders, every named file/folder must exist, decision/gate/sampling are in the identity.

Effect observed: both sidecars restarted on the new supervisor code at 09:51 and adopted their stages. **g1-m10's stage was ended by the supervisor at ~10:04** (`stage-timeout`, 31,020 s, SIGTERM, exit receipt): with the wait no longer counting as progress it was past the 8 x 3,600 s hard cap with no progress inside the stall limit. Next attempt 10:24 starts fresh processes on current code (new client identity → service on the current decision code).

## Tests (`repo test-file`, one at a time)
- `B🏡️resident/🧪️tests/🔖️revision/🟦️.ts` 4/0; `B🏡️resident/🧪️tests/🔄️reload/🟦️.ts` 12/0; `B🧭️invocation/🧪️tests/🛰️plan/🟦️.ts` 17/0.
- `R🌲️process-tree/🧪️tests/🟦️.ts` 16/0 (includes the Python oracle).
- `A🧪️tests/🟦️.ts` 5/0.
- `R🧪️tests/🟦️.ts` (ready supervisor) 58/0.
- `…/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts`: 1 pass / 15 fail, the same fixture cause as before W17 and W20 (10 x "Actual artifact repository authority required", 1 x "Exact owned cancellation fixture data required", 4 follow-on expects; every test after the first fails in < 2 ms). The physical binding path (dangling own link replaced) is therefore not exercised by a real-image test yet.
- One slip: the binding test file ran at load 61 (the wait used the wrong `vm.loadavg` column once); every other heavy step ran at load <= 30.

## Watch
- 09:51 both sidecars restarted on the new supervisor code and adopted their stages.
- ~10:04 g1-m10's stage ended `stage-timeout` (31,020 s). 10:24 a fresh attempt 26 started a new graph service ("Waiting for the repo graph service"), **got past the gate** (no more `session-cap` wait), read the slice in ~600 s ("taken 607 of 609") and executed.
- **`✔ nx run @semio-tech/norm-din4108:build:prototype` on g1-m10 (~10:39) and on g1-m08 (~10:47)**: the binding refusal is gone on both.
- Neither worker was Ready at 10:52:
  - Both fail `framework:cargo` with `process.source.crate-refused {"crate":"semio-framework-value-native-receiver-protocol","reason":"Cargo Source outside admitted owner"}`. Cause: a peer's temporary debug module in `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧬️protocol/📦️packages/🦀️rust/🦀️.rs:2`, `#[path="/Users/ueli/…/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs"] mod dbg;`. The source admission is right to refuse it. Not touched (peer work); the same kind of leftover W17 saw in `…/🧳️original/🦀️.rs`.
  - g1-m08 (attempt 19, still the process tree from 09:13) is in a second slice read and waits in the gate again: "this agent session already holds its share …; 2 active grants, 1 queued; 490 MB available above the floor". With 490 MB it would wait on memory anyway. The records do not show whether its service already reloaded or a second session really competes.

## Open items
1. Remove the peer's `mod dbg` `#[path]` line (owner: ticket GET-PROCESS-WORKING-END-TO-END) or let it land; until then `framework:cargo` fails on every worker that builds this stage.
2. g1-m08's current wait: if it stays on `session-cap`, its supervisor now ends it (the wait is no longer progress; past the hard cap within 10 min, before it after the stall limit plus backoff). The next attempt runs fresh processes on the current code.
3. The old graph service of a worker reloads only through `newerSibling`/drain: its in-memory list hashed the resident schema I edited, so its wanted identity is neither the old nor the new one. A fresh client (new attempt) starts the service for the current identity, which is what happened on m10. A reload of a live service whose revision-file list itself changed is not proven.
4. Real-image test for "a dangling own link is replaced and the receipt rewritten" waits for the `📥️physical` harness fixture ("Actual artifact repository authority required") to be repaired.
5. Why m08/m10's products stores lost their content (re-born smaller images) is not established; the binding no longer depends on it.
