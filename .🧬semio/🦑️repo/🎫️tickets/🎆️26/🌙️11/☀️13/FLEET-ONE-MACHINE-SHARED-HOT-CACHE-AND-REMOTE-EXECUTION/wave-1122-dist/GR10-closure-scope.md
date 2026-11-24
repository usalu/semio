# GR10: what a closure entry is validated through, and why nothing seals (2026-11-23, 17:55 to 18:50 Mac clock)

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `B` = `L/🏗️build/`, `SI` = `B/🕸️graph/📥️inference/🧭️script-inputs/`. Lock `lock.graph-closure` (released). Probes kept in the session scratchpad (`measure.mjs`, `real.mjs`, `groups.mjs`, `cover*.mjs`, `sim.mjs`); nothing under the ticket except this report.

## Verdict in five lines
1. The premise is only half right. A sha change alone does NOT stale an entry (analysis rows replay with the registered parsers); measured with the real parsers, 84 of the 200 newest entries are stale and the cause is 7 files whose parse RESULT (imports/references) changed, two of them for 82 and 69 entries: `L/⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts` and `FW/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`; both were edited by this wave's own lanes.
2. The entries are not "narrow by provider": 30 of the 48 shared closure groups share ONE core of 1,901 files (union 2,275), reached through barrels and string command references from every tool entry; 29 % of the shared core is graph inference code, os, UI, cas, unit cache, replication.
3. The scheduling policy already prunes remote-execution, remote-cache, fleet-status, resident, workers from every group LIST (0 files, now pinned by a law), but not from the ROWS of an entry: a pruned file that moves still stales the entry.
4. A second, sharper cause: every component target (`🧩️component` Cargo crates, 100+ owners) lists the cargo toolchain script `L/🏃️process/🦀️cargo/🏃️execution/📜️script.ts` as an input; a module literal makes `keepsWholeWalkV1` true, so the tool boundaries are switched off and the owner's entry carries the full walk (2,381 files) and a derived `walk` fact over it.
5. The root fix (walk boundary for view-pruned domains, or tool-reach facts instead of tool rows, or a governed derive on a stale take) changes record content or key composition; I did NOT land it blind (reasons in section 4). Landed: a ceiling and exclusion law over every closure group (4 pass).

## 1. Measurement (task 1)
Store: `.cache/b/g/m/inference/selections/8d8f21d7.../ce` (40,869 entries). The 200 newest entries, 155 with more than 1,000 files, 82 with more than 1,000 files and a recorded source, replayed with the real registered parsers (`prepareSourceAnalysesV1`, `observationRowFreshV1`):

| recipe | entries | files | stale by real replay |
|---|---|---|---|
| target-sources | 105 | 0 to 2,479 | 22 |
| native-target | 47 | 0 to 1,869 | 26 |
| generic-target | 8 | 53 to 2,498 | 5 |
| tool-closure (26 tools, 1 each) | 26 | 1,866 to 2,527 for the 18 big ones | 18 of 18 big |
| script-provider (9 ports, 2 views) | 16 | 1,866 to 2,439 | 16 of 16 big |
| closure-of | 5 | 20 to 2,059 | 1 |
Total 84 stale of 200 (82 with a moved analysis row; the rest `global` generator-contract rows of the owner registry, not replayable without those registries).

Sha-only churn (GR9): 51 distinct files changed their bytes across the 200 entries; real replay moves 7 of them:
| file | entries moved | class |
|---|---|---|
| unit-cache `🔒️lock` | 82 | (b) merely reachable: cargo driver -> native-orchestration -> unit-cache launch -> gc ballast -> lock (4 hops, found by import walk) |
| native-build `🧩️component-pass` | 69 | (b) for tool/provider entries (store-custody domain, pruned from their list), (a) for native entries (it assembles the Cargo arguments, `nativeKeyed`) |
| unit-cache gc supersession | 4 | (b) |
| inference `📦️project-nodes` | 4 | (a) for the inference tools, (b) elsewhere |
| kernel plugin-graph, browser-bundle component-bridge-source, plugin driver-stages | 1 each | (a) component build path |
Classification of the 51 changed files by subsystem: graph 14, store-custody 6, remote-execution 5, cargo 5, cas 5, os 4, graph-state 2, execution 2, other 8. Class (c) data files (schemas/policies): 2 changed (`policy.json` x2), 0 moved a row. Class (b) is 40 of 51 changed files and 7 of 7 moved files except component-pass on native entries. Is the closure rooted at the repo CLI? Yes for `process-router` (the router reads its command table from its bytes: all commands), and in effect for 30 groups: the 1,901-file intersection is reached from every tool entry (a barrel such as `typescript-build` re-exports `🧩️artifact-package`; regex reach of the literal imports is 81 files, the other 1,820 come through command string references and runtime references).
Union of the 82 big entries: 2,733 files, 546 (20 %) inside declared scheduling domains (store-custody 265, resident 41, capacity 38, leases 32, process-supervision 29, remote-cache 28, remote-execution 27, diagnostics 27, worker-supervision 17), whose rows still count.

Shared groups (`sharedClosureGroups`, 22 s): 48 groups, 26 of 1,901 to 2,155 files, 4 of 116 to 285, 18 of 1 to 50. Intersection of the 30 big ones 1,901 files: process 211, graph 200, bootstrap 185, cargo 132, os 107, repo 75, tools 70, runtime 43, artifacts 39, ... cas 18, unit-cache 56, replication 15, ui 23.

## 2. Why nothing seals (what was found, with evidence)
- A stored entry is valid when all its rows hold; an analysis row holds on replay (same parse result), so only an import/reference change of ANY of the ~2,400 files stales the entry; 70 edits a wave make that certain. Verdict file of the live read (`semio-read-verdicts-501/49450-*`): 651 entry verdicts, 565 true, 86 false (the false ones are superseded entries, replaced after the verdict).
- A fleet record ships bundles derived on the worker's tree; the intake keeps a shipped bundle only when its own rows hold in the taker's tree, so a worker that derived before the last moved file is refused (`closure-entry-absent`, GR9). Live adoption (read.log, now): 87 listed, 194 looks, 98 adopted, 42 stale (closure 24), 45 missed, 18 inferred here although listed: 98 / 140 = 70 % of fetched-or-stale looks (80 % at GR9, 86 % at 17:15). Not changed by this lane (no restart).
- Component targets: probe `cover.mjs` (target without inputs): list = ONE reference `toolClosure:driver-stages:scheduling`, 10 ms. With the real inputs (cargo toolchain script as a module literal): 25 references, 14 s, whole walk, rows of 2,381 files. Same command, same tool; the literal alone switches the cover off (`keepsWholeWalkV1` in `SI/🟨️.mjs` line ~902). Expanded key files: 2,153 with the literal against 1,902 without: the 251 extra files are the native-build modules the toolchain script imports (the literal keys them on purpose).

## 3. What I tried and what is sound
- Simulation (`sim.mjs`): treat unit-cache `lock`, `gc`, `launch` as a scheduling domain and read every walk through its view without following pruned modules: stale entries 82 -> 34 (-59 %); the 34 left are component-pass (30, native view, keyed on purpose), project-nodes (4), 3 single files.
- Sound only with a soundness decision: the scheduling policy says an edit of scheduling code must not change a key, yet today a pruned file that adds an import of a keyed file adds it to the list (a violation of the stated policy). A walk that does not follow pruned modules matches the policy; it changes view answers (lists) once.

## 4. What I did NOT land and why (decisions needed)
1. Opaque walk at view-pruned prefixes (`reachWalkV1` gets a `boundary` beside `stops`; callers: tool-closure, script-provider, target walks; `walkQuestionV1` key). Changes the list of every group and target once. Conflicts with the contracts `🧮️blast-radius` ("keys on at least the files its walk reaches") and `🪡️tool-cover` ("never fewer than the walk"): they would need the walk through the view as the oracle, a policy decision.
2. Module literal covered by a tool (`native-toolchain` carries the script as a resource): keeping `stops` when the literal is carried. Measured: it changes the list of every component target (25 references -> fewer) and so the task hash of 100+ component builds, while the expanded key set stays equal only if the reached tools stay referenced. This violates the byte-identical record content requirement of this task and would rebuild every component once in the middle of the play release; not landed.
3. Take-time re-ask: a stale (not absent) entry in a take is answered by the farm verdict (GR9 item 1 / GR3 item 1a): the record stays valid when the re-derived ANSWER equals the recorded one (a `some` over the walk and `generated` are stable under these edits). This is the change with the best ratio (validity depends on the answer only, no content change, no key change) but it moves derivation into the governed child and needs a read to measure; not landed (no read can be restarted).
4. Per-entry byte-identity differential over 30 owners (task item 2): not run, because no landed change alters records. The law for it exists in spirit as `🧪️tests/🎯️read-set` (GR3).

## 5. Landed
New law `SI/🔌️provider-closures/🧪️tests/📏️closure-ceilings/` (`🟦️.ts`, `🔣️.json`, `🧬️schema/🔣️.json`): 4 tests, 4 pass, 15 s. It pins (a) the table of closure groups equals the declared tools and provider ports (a new tool needs a ceiling), (b) each group at most measured size + 5 % (2,155 -> 2,275 max), (c) no group of the scheduling view holds a file of any scheduling domain, no native-view group holds a file of a non-native-keyed domain (today 0), (d) the unrelated subsystems still reachable: cas at most 20 files per group (18 now), unit-cache at most 62 (56 now). Any change that removes files only lowers the numbers; any change that adds a subsystem (remote-execution, cas, fleet status) to a group fails.

## 6. Expected one-time cost and result check
- Landed law: no cost, no record change; result check (task 3): the law fails on an edit that makes a group reach a scheduling domain; an edit of a remote-execution module changes no group (0 files today, verified by the law); an edit of an in-closure tool file moves the groups that hold it (all 30 big ones for the core). The per-provider "edit inside invalidates only its owners" check needs the boundary of item 4.1; not available.
- If 4.3 (take re-ask) lands: every stale closure entry re-derives once per read (about 80 recipes of 1,900 files, the farm shares one derive per recipe: estimate 2 to 6 minutes warm), then owner records stay valid across tool edits as long as the answers hold; adoption should return to the pre-wave level (86 %).
- If 4.1/4.2 land: all closure entries re-derive once and the 100+ component target keys change once.

## Tests
`repo test-file` `📏️closure-ceilings`: 4 pass / 0 fail. No existing test file was touched. Landing in `T/landings.md`.
