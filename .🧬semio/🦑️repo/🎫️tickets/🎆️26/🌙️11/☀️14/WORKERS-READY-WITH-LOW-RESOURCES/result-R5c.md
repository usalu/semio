# R5c progress notes (successor session, 2026-11-15 from 05:35)

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Scratch: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R5c/` (`inspect.py`, `subsume.py`).

## Status
Nothing landed, no lock taken, no LAND-OK written, no live file edited by this session (only `T/🧰️scripts/R5/lab/stale-probe.py`: the stop condition now waits for the real "leave K projects stale" line instead of "refreshing").

## Finding 1: the pointer/index design of R5b is much larger than needed, because most of it exists
`script-inputs/🔌️provider-closures` already is the "one immutable record per tool, owners hold a pointer" design, in Nx-native form:
- a node holds `{input:"toolClosure:cargo-command:native", projects:["@semio-tech/repo-script-providers"]}`, the group lives ONCE in the providers project node (tokens `SEMIO_SOURCE_CLOSURE_<sha>` of content-addressed records); a membership change of the tool moves exactly that one node.
- travel (B1): the group is part of the generation fragments, the closure leaves it names travel with the generation since R8 (`uploadClosuresV1`/`restoreClosuresV1`). No separate index, nothing to restore.
- refresh (B2): the group is a normal shared closure entry (`primeSharedClosuresV1`, `sharedClosureRecipesV1`), re-inferred with the providers project; no second step, no fault policy beyond the existing one.
- queries (B3): `fact()` in `closureApiV1` records the answer of `some`/`excludedPaths` as a derived fact; unchanged.
So blockers B1, B2, B3 of R5 do not arise for this route. The nine identity-check places stay untouched because tokens stay immutable content hashes.

## Finding 2: what keeps the churn is NOT a missing tool record, it is duplicates and undeclared tools
Measured on generation 235 (5 100 owners, fragments decoded, closure leaves read from `.cache/b/g/m/build-input-closures`, 8 667 leaves):
- A Rust node (example `semio-s-plugin-energy-engine-zone-air`) holds in `commandSources0`: 10 reference items (nine `scriptProvider:*:native`, `toolClosure:cargo-command:native`) AND 10 `SEMIO_SOURCE_CLOSURE_*` tokens (chunks of 52 to 348 files). The 7 chunk leaves present on this machine hold 930 files; **928 of them are in the shared `toolClosure:cargo-command:native` group (1 593 files)**, the other two are `bunfig.toml`, `rustfmt.toml`. I.e. the per-node tokens are a pure duplicate of what the node already references. They come from the target's own walk of its command script (`targetScriptSources` -> `boundedScriptInputsV1` of the entries `cargo/📜️script.ts` with router command words), which is the same script the declared tool covers. 3 of the 10 leaves (`3eb91d84`, `7ee0751a`, `bcc68376`) are not on this machine, so "all 10 are subsumed" is shown for 7 of 10 only.
- Distribution: 5 827 command-source groups carry tokens; 3 195 have 10 tokens, 1 898 have 11, 315 have 14 (the Rust and component targets, about 5 400 groups), 4 454 groups hold no token. These are the groups whose tokens change when the cargo driver closure gains a file; this is the 3 118 holders of R5.
- TS/generic nodes (`@semio-tech/framework-os-mcp`): 4 groups of 3 to 11 tokens each next to 4 to 9 `scriptProvider:*:scheduling` references; token files lie in the graph inference, bazel, resident, remote-exchange and budget trees. These are walks of generic targets' scripts that no declared tool covers.

## Corrected design (the smallest sound R5c)
1. **Subsumption by declared tool (static, no walk):** in `nativeTargetCommandInputs` (script-inputs) compute the target's script entries the way `targetScriptSources` does (extract that first half into a helper, no walk), and when every entry is an entry of a declared tool whose view the call reads (`cargo-command`, `cargo-lock-owner` from the policy table `provider-closures/🔣️.json`) and the target adds no other entry (no module literals, no `closure.modules`/`resources`/`runtimeSourceEntries`), return `fallback` (which already holds `toolReferencesV1("cargo-command")`) without walking. Result: the owner records no walk fact, the node holds no token for it, and an added file in the driver closure moves the single providers-project node. Soundness: the tool group is the over-approximation (no router word selection, `walk(entries ⊆ T) ⊆ walk(T)`); the only files the node loses are those the native view leaves out by the policy's own contract.
2. **Same shortcut for the generic path** (`genericTargetCommandInputs`) with more declared tools (data only, table `tools.closures`): one tool per distinct script entry set of the generic targets (bun/vite/tsc/bazel wrappers). The set has to be enumerated from the inference records (`np` records are binary, a small bun script on the `closureRecipes` table of the project records does it); not done yet.
3. Law tests: (a) tool closure ⊇ target walk for every shortcut target on the live tree (esbuild oracle already used by `⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs`); (b) the node of a shortcut target holds no `SEMIO_SOURCE_CLOSURE` for the driver; (c) an added imported file moves exactly the group of the providers project.

## Why this is not landed yet
- It still re-stales all Rust nodes once (the nodes shrink), and only the Rust half (about 3 100 of the 5 100 owners) is covered by tool declarations that exist today; the generic half needs step 2. Landing the Rust half alone would cost a second full refresh later, which the package forbids ("no half state").
- The three leaves not on this machine and the generic tool set are unmeasured, so a soundness claim for "all targets" would be a guess.
- Graph reads on this machine take 15+ minutes at load 15 to 20 (see probe lines below).

## Probe log (graph reads, one at a time)
(see appended lines)

## Probe result
Baseline `repo show projects` (limit 1 700 s, load 15 to 20, another agent's read was running too): no graph summary line within the limit, so NO real before/after number for L1 to L5 exists. The second probe (test file) was therefore not started.

## Coordinator notes handled
1. Drain (R5b) removed from the package (moved to `T/🧰️scripts/R5/withdrawn-R5b-drain/` for R17). `payload.py` no longer applies `extra_edits`; `land.py` tests list shrunk; `payload.py` re-run against the live tree; `land.py --dry-run` is clean: 3 edited files (nx.json, build policy, scheduling table), 4 new boundary-test files. Scheduling-domains-only landing is ready and independent.
2. R13 fleet fallback in `project-nodes`: NOT wired. A correct wiring needs (a) the implementation digest (`graphKey`) and the stale `roots`, which only the host has (`🕰️host/🟦️.ts:188 fleetRead`, `🤝️prefetch fleetReadStaleGenerationV1`), threaded as an option into `discoverBuildProjectsV1`; (b) in the owner loop of `project-nodes` a catch of `GraphOwnerFleetErrorV1` that re-runs `inferProject` outside the governor so the project-record gate seeds it from the fleet (`createProjectFleetPortV1().gate`); neither can be validated without a graph read, and a wrong catch would drop projects from the graph. Left for the agent that can run a read.

---
# FINAL STATE (07:35, successor session; supersedes everything above except the R5/R5b/R5c measurements of the predecessor)

Scripts: `T/🧰️scripts/R5c/` (edits `edits_si.py` code, `edits_policy.py` policy/schema/stamp/fixtures; `apply_overlay.py`, `run.sh`, `runtest.sh`, `suite.sh`, `harness/{laws,cover,enum,hub,pkg}.ts`, `summarize.py`, `equiv.py`, `new-tool-cover.ts`). `T/🧰️scripts/R5/payload.py` folds them into `edits.json`/`baseline.json`; one `land.py` = scheduling domains (R5) + tool commands (R5c). The drain of R5b was withdrawn to R17 (`R5/withdrawn-R5b-drain/`). The R13 fleet fallback in `project-nodes` is NOT wired (needs the implementation digest and stale roots from the host plus a validated catch; no graph read available): left for the agent that can run a read.

## Design that was built (it is smaller than the pointer/index design of R5b, and B1/B2/B3 do not arise)
The "tool record + pointer" already exists in Nx-native form: a node names `{input:"toolClosure:<tool>:<view>", projects:[@semio-tech/repo-script-providers]}` and the group lives once in the providers project (its closure leaves travel with the generation since R8, it is refreshed by the ordinary inference of that project, `some`/`excludedPaths` stay derived facts). What was missing: nodes still embedded a duplicate walk of the same files (928 of 930 files of the per-node tokens of a Rust node are in the `cargo-command` group). R5c removes the duplicates:
1. **Entry cover** (`toolCoverV1`): a command whose script entry is the entry module of a declared tool (and selects no router words, or the tool says `"selection":"included"`) is not walked; the closure names the tool. Not applied to a target with module globs / TypeScript project / runtime entries / a resource no covering tool carries; a generic command takes only tools whose view carries the external packages.
2. **Subtraction** (`toolFinishV1`): for every declared tool whose entry module the walk of the remaining entries reached, the closure names the tool and drops the files of its group. A generic command computes its external packages over the dropped files too (`covered` field), so packages are never lost.
3. Tool groups with a view that carries packages now carry `{externalDependencies}` (`toolClosureGroupV1`; `cargo-*` tools are native view: unchanged). Entries outside the workspace are ignored (fixture workspaces).
4. Declared tools: `cargo-command` (+`selection:included`), `cargo-lock-owner` (unchanged), new `driver-stages`, `process-router`, `testing-framework` (all view scheduling, selection included). Schema: optional `selection`. Inferrer stamp `script-inputs/3` -> `/4` (schema document + output-stamps fixture). `targetScriptEntriesV1` is the entry collection of `targetScriptSources` as its own export; closure-facts fixture rows follow the function split.
5. Callers: `nativeTargetCommandInputs` ("native"), `genericTargetCommandInputs` ("generic"), recipe `target-sources` ("queries"); every other caller (owner discovery) unchanged.

## Tool declarations enumerated (from 41 000 recipes of the closure-entry store, `entries.json`/`entries-ts.json`)
Script entries by distinct owners (projectRoot): `⚡️caching/🦀️cargo/📜️script.ts` 4 340 (Rust and component owners) -> tool `cargo-command`; plugin `driver-stages` script 71 -> `driver-stages`; process module router script 7 and every package script that loads the testing framework (1 849-file closure, the shared 1 533 files of all Go/TypeScript/.NET package scripts) -> `process-router`, `testing-framework`. 710 distinct package scripts (one owner each: the router of one package) stay per-node; their shared framework part is subtracted through `testing-framework`/`cargo-command`, so each keeps only its own 4 to 60 files (median own list of the 370 sampled owners: 4 files, before 1 525). Not declarable as entry tools and not subtracted: nothing observed in the sample.

## Proof (real inference functions, live tree, virtual file world for the counterfactuals, frozen read snapshot, control world; 370 owners = 150 native-target, 120 generic-target, 100 target-sources; `scratchpad/R5c/laws-old2.json` before, `laws-final3.json` after; control "base2 vs base": 0 nodes, 0 keys, 0 groups differ)
| law | before (live code) | after (R5c) |
|---|---|---|
| L1 new unreferenced test file under the build module (+ a lone file beside the driver): nodes changed / task keys changed | 0 / 0 | 0 / 0 |
| L2 an imported file added to the cargo-command tool: nodes changed | native 147/150, generic 45/120, target-sources 91/100 | 0/150, 0/120, 0/100 |
| L2 task keys changed (tasks that execute the tool) | 150, 113, 98 | 150, 113, 98 (the 7 generic and 2 target-sources owners that do not execute the tool are unchanged, correct) |
| L2 groups changed | 20 provider groups | 21 of 23 groups, all in the single providers project |
| L4 replay of P14 (portable/platform-class imports) | 0 nodes | 0 nodes |
| L4 replay of R2 (memory settle/owner-scope imports) | native 147, generic 46, target-sources 91 | 0, 0, 0 |
| L3 body edit in a neutral-domain file | boundary tests of R5 (key law), unchanged | same |
| never under-key: key inputs after vs before on the unchanged tree | | files: 0 owners lose a file; 369 owners strictly larger (1 file = a peer's landing in between; 21 generic owners +224 native-build modules because their cargo-driver walk is now the native-view group) |
| packages after vs before | | none lost for hashed kinds (target-sources lists are query-only) |
Units: new `provider-closures/🧪️tests/🪡️tool-cover` 6 pass (esbuild oracle; add/imported file/unreferenced file/body edit/late equals direct/subtraction). Touched-module suites on the overlay equal the baseline except the rig artifact below: provider-closures 5/5, closure-facts 22/22 (baseline 21/1), output-stamps 4/4, late-closures 4/4, command-inputs 2/2, script-inputs 4/4, router 25/25, walk-views 5/5, project-assembly 2/2. R8 `closure-leaves` (restore on a second store) 4 pass at the live location. Every changed `.mjs`/`.ts` parses (`bun build --no-bundle`), the `.d.mts` parses (TypeScript API, 0 diagnostics), the 7 changed JSON files parse.
Rig artifacts (overlay copies load twice where a test imports by absolute path): the scheduling test "fallback command inputs of native targets…" and 2 stamps tests compare an overlay module with the live one; `land.py` runs them live.

## L5 (generation restored on a second store)
Not simulated with a scratch store of this change: the change adds no new document type. A node now holds references; the groups live in the providers project fragment, and its leaves are named by that fragment and travel with the generation (R8 `closure-leaves` 4 pass). A restored generation therefore carries them; a missing leaf is refetched or re-derived (R8).

## Not verified
- No whole-graph read before landing (machine load 12 to 20, reads take 30+ min); the effect on real late-mode owner records (`np` store) is proven by the late==direct unit law and the sample above, not by a generation.
- Windows/Linux: the new code uses no path separators of its own.

## LANDED 07:25 (T/landings.md "Package R5 and R5c landing")
`land.py --land` took the lock, wrote 16 files, ran 14 test files: all "as expected" (the 2 reds are the known ones: engine-digest spawn-only test and the stamps "keyed by bytes" test, both red before). `N/LAND-OK` removed, lock released. Originals for `land.py --rollback`: `T/🗑️generated/R5-landing-072537`. The landing re-stales every project once (stamp `script-inputs/4`, policy rows); the server's refresh is running (build-agent.log). Next: wait for that refresh to settle, then ONE real graph read for L1 (add a file under `🏗️build/🧪️tests/`, read, remove the file) - result below when done.

## Coordinator rule change (06:50 UTC): no further landings
R5c was ALREADY landed at 06:25 UTC (07:25 CET) with R5, before the rule; it finished cleanly (16 files, tests as expected, LAND-OK removed, lock released). I did not land again. R5c is complete and in the live tree; its one full refresh is running. The real L1 graph read after the refresh was NOT done (the watcher `settle.py` was stopped; machine busy with the refresh). To run it later: wait until `.cache/b/g/m/generations/m` shows a generation with an implementation digest other than `b1f7adc64de3` and no new generation for 15 min, add a file under `🏗️build/🧪️tests/`, run `SEMIO_GRAPH_PHASES=1 repo show projects` (`T/🧰️scripts/R5/lab/stale-probe.py <label> 1800`), expect 0 stale projects beyond ordinary churn, remove the file.
Rollback if ever needed: `python3 -I T/🧰️scripts/R5/land.py --rollback T/🗑️generated/R5-landing-072537` (takes the lock).
