# result-x-graph: why `repo run <one target>` pays the whole closure every time, and the staged fix

`T` = this ticket folder. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `B` = `L/🏗️build`. `G` = `B/🕸️graph`. `C` = ticket 26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN. Tools and staged set: `T/x-graph-tools/`. Peer handoff: `T/handoff-graph-closure.md`. Status: ANALYSIS DONE, FIX STAGED AND VERIFIED IN A MIRROR (14 + 5 + 13 mutation checks, tsc clean), NOTHING APPLIED LIVE (quiet tree, peer freeze), LIVE TIMING NOT TAKEN (no dry-run allowed since 16:39, load 41-46).

## 0. Verdict

1. **The selected closure already exists; the problem is that nothing about it is ever persisted.** `repo run X` selects `{kind:"tasks"}` and `closeBuildOwnerAdmissionV1` (`G/🚪️admission/🟦️.ts:71`) admits only owners reachable from the selected tasks through `dependsOn`, generated-task producers, file ownership, package and native dependencies and catalog activation. The 429 s of `dry-boot-2.log` are that closure being derived from nothing: 258 owner observations for 162 admitted owners (of 3,732 in `📋️build-owners.json`), user 367 s, sys 51 s, i.e. CPU bound at about 1.4 CPU-s per observation. Wall minus CPU is the ingress (x-ingress).
2. **Why a warm unchanged tree recomputes (three independent reasons, all in `G/🟦️.ts`).**
   - `inventory()` (`:324`) derives the full closure on EVERY call, before the cached graph receipt is even looked at, because it needs `admission.files` to compute the membership digest. A valid graph receipt therefore saved only the later project assembly, never the closure. The cold path derives the closure TWICE (`initial` `:325`, `final` `:349`).
   - Neither the closure nor the per-owner observations are cached across processes. Stage 2 (project assembly, `G/📥️inference/📦️project-nodes/🟨️.mjs:143`) already has a content-keyed per-project record (`np/<sha256(root)[:24]>.json`, key = implementation revision + registry digest + manifest json + selected targets + inputs); stage 1 (`discoverBuildOwnerAdmissionFactsV1`, `G/📥️inference/🔎️owner-discovery/🟨️.mjs:156`) has none.
   - The selected graph and authority receipts (`selected/<scope>.json`, `.authority.json`) are written only at the very end of a SUCCESSFUL attempt. This machine has never completed one for the course: `.cache/b/g/m/selected/` and every `inference/selections/<scope>/` directory are empty. The coordinator's run died on `Generated task cannot consume its own output` for `@semio-tech/framework-os-shell-rs:typegen` (see 1.4), so every retry starts cold.
3. **The refusal is not an over-approximated closure.** The error is thrown inside the observation of an owner that IS in the closure (a consumer's compile import of `🧰️framework/🔨️modules/🐚️shell/🟦️.ts` makes `shell-rs:typegen` a required producer task through `generatedDependencies`; typegen's own input `📜️script.ts` closure reaches the same file). The peer is changing that rule (coordination 16:39). What changes here: the error now names the requester chain, and a failed closure no longer loses the 161 finished owners.
4. **Fix (staged, 21 anchored operations, 3 live files + 5 new test files):** (a) every owner observation is a persisted, content-keyed record that is reused while its consumed facts hold (resumable after cancellation or a failing owner); (b) the finished closure is held in memory for the second `inventory()` call and persisted next to the graph as `selected/<scope>.admission.json`, so a warm run verifies facts (O(distinct facts)) instead of deriving anything; (c) progress (`build:selected-records`) and cancellation per owner; (d) everything sits below `loadBuildGraphV1`, which both the Nx and the Bazel path call with the same selection, so the neutral graph is one object.
5. **Not measured live** (frozen tree). Offline numbers: receipt for 37,263 consumed facts = 13.6 MB, create+validate 0.22 s, parse+validate 0.49 s, `observationsHold` 0.41 s at load 46 (`bench-receipt.ts`). Expected warm cost of the whole admission phase: 1-3 s versus 367 CPU-s. Acceptance procedure and harness in section 6.

## 1. Profile (evidence)

### 1.1 What the log shows (`T/🗑️generated/dry-boot-2.log`, `x-graph-tools/profile-log.py`)

| fact | value |
|---|---|
| total | real 429.21 s, user 366.92 s, sys 50.73 s, rc 1 |
| ingress before the first observation | log lines 1-52: 40 `dependencies:generation-verify` events twice, one wait for `repository-cache-charge:.cache/b/d/m` (exclusive, position 2 of 2) then `build:selected-declaration` starts at line 53 |
| owner observations (`build:selected-declaration`) | 258 |
| of them with targets or inputs (`build:selected-normalization`) | 96; base only (`[]`,`[]`): 162 = 162 distinct admitted owners at the time of the failure |
| per observation | one `selected-declaration, catalog-targets, command-sources, native-closure, compile-closure` cycle (+ `normalization, native-preparation, source-inputs, source-closure` when targets exist); 638 `selected-compile-imports` ticks, 414 `selected-source-inputs`, 188 `command-source-imports` |
| failure | observation of an owner with selected task `typegen`, importer `🧰️framework/🔨️modules/🐚️shell/🟦️.ts`, phase `compile`, thrown at `🔎️owner-discovery/🟨️.mjs:213` (`generated`) |

The log has no timestamps. `x-graph-tools/graphtime.ts` prefixes elapsed milliseconds and tabulates the JSON events (`build:selected-*`) for the next real run; `profile-log.py` reads plain and prefixed logs.

### 1.2 Which graph phases run for `repo run <one target>` (tasks selection, file:line in the live tree)

1. `B/📜️script.ts:188` `loadBuildInvocationGraphV1` -> `loadBuildGraphV1` (`G/🟦️.ts:361`) with `buildInvocationGraphSelectionV1(args)` = `{kind:"tasks",tasks:[{project,target,configuration?}]}`. The Nx path calls the same loader with the same selection (`L/⚡️caching/🚀️bootstrap/📜️script.ts:405`). Scope key `buildGraphScopeKeyV1 = sha256([selection, registry])` (`G/🚪️admission/🟦️.ts:66`): no tool in it.
2. `withBuildGraphStateV1(..., scope)`: exclusive per-scope writer lease inside `withRepositoryBuildGraphSelectionV1`; the optional-cache namespace of that writer is `inference/selections/<scope>/` and isolated (`buildGraphWriterOptionalPathAdmittedV1`: a selection writer may only write `selections/<its scope>/...`; `buildGraphWriterDirectoryAdmittedV1` refuses the `admission` namespace).
3. `loadBuildGraphAttemptV1` (`:317`):
   1. `inventory()` #1 (`:324`): `captureObservedSourceInputsV1(settleObservedComputationV1(closeBuildOwnerAdmissionV1(... discoverBuildOwnerAdmissionFactsV1 ...)))` then `buildGraphMembershipV1` (lstat of the admitted files). The closure is a fixpoint: every admitted owner is observed once with `([],[])`, and again every time its selected targets or requested input groups grow (`closeBuildOwnerAdmissionV1` re-observes on a changed `[targets,inputs]` revision).
   2. cached receipt `selected/<scope>.authority.json` (`:333`): compared to the membership digest of step 1 and to `observationsHold(receipt.observations)`; on a hit the graph file is read and returned. This is the only warm shortcut and it comes AFTER step 1.
   3. on a miss: `discoverBuildGraphV1` over `admission.files` only (`discoverBuildProjectsV1` per admitted project through the np record of stage 2, `discoverBuildDependenciesV1`, metadata, portable inputs).
   4. `inventory()` #2 (`:349`): the closure derived a second time to detect divergence.
   5. publish `selected/<scope>.json` + `.authority.json` (`:357`).
4. Rust preparation (`withSelectedRustBuildGraphV1`), pin to the retained graph store, then the tool (Nx or Bazel adapter). Not part of this task.

### 1.3 What each of the 258 observations does (and why it is 1.4 CPU-s)

`discoverBuildOwnerAdmissionFactsV1` (`🔎️owner-discovery/🟨️.mjs:156`): reads the manifest, `cargoTargets` + `componentBuildTargets` + playground targets for the owner, `targetScriptSources` per selected target (command source closure), the `package.json`, the native Cargo closure, `projectWithDefaults` + `withNativePreparation` when targets exist, then the source-input walk (`readBuildSourceInputMatchesV1`) and the TypeScript compile-import closure (`selected-compile-imports`, up to 638 ticks). Shared files (command sources, shared TS modules) are parsed once per process thanks to in-process memos, but nothing survives the process except the 13.5k content-addressed `build-input-closures/*.json` and the per-record source-analysis packs.

### 1.4 The `typegen` refusal is a closure-internal cycle

`framework-os-shell-rs` declares `typegen` with input group `schemaModule` (`{projectRoot}/📜️script.ts` among them) and output `🧰️framework/🔨️modules/🐚️shell/🤖️generated`. The thrown message has `importer: 🐚️shell/🟦️.ts` (which re-exports `./🤖️generated/🟦️.js`) and `phase: compile`: the module that imports typegen's own output is part of a closure typegen consumes. `for(const name of targets)` in `generated()` rejects it for every SELECTED target of the observed owner, a check that exists only in the selected path (the all-graph path has no such rule), which is why `show projects` never saw it. This is a rule decision for the peer (it owns it per coordination 16:39). Consequence for this design: a deterministic failure of ONE owner must not discard the others, and it must tell the developer why that owner is in their closure (both done).

## 2. Design

### 2.1 What the peers already staged (checked, extended not duplicated)

- `C/result-graph-perf.md`: cold CPU -40 percent (shared command-input closures, row JSON cache, async brotli), seal fact table, `bootstrap` double-checked install. Touches `script-inputs`, `inputs`, `observation`, `seal`, `bootstrap`. Does NOT touch `G/🟦️.ts`, `G/🚪️admission/🟦️.ts`, the admission schema. Its open item 4 ("serialize the first shared walk") is a different walk (dependencies ingress).
- `C/result-show-perf.md`: `show projects` GC bound, no quadratic code. Not related to the selected path.
- `C/result-bzl-lazy.md`: laziness of the Bazel workspace per selection; consumes the graph this task produces. Its closure-versus-universe finding is about Bazel analysis, not inference.
- `T/result-x-ingress.md` / `handoff-ingress.md`: the generation receipt and exclusive lease ingress. Independent; the generation receipt makes the ingress seconds, this makes the graph seconds.

### 2.2 (a) Per-owner records keyed by content

`observeBuildOwnerAdmissionV1(ports, owner, targets, inputs, compute)` (appended to `G/🚪️admission/🟦️.ts`) wraps each `inspect(owner,targets,inputs)` of the closure in the existing `observeProjectInferenceV1` primitive, exactly as stage 2 does for projects:

- leaf: `np/<sha256("owner-admission" 0x00 root 0x00 JSON([targets,inputs]))[0:24]>.json` inside the scope's isolated writer directory. It matches the existing optional-cache grammar (`np/[a-f0-9]{24}.json`), so no storage schema change, and it is domain separated from the project leaf `sha256(root)`.
- key: `[1, implementationRevision, scope, sha256(registry), owner.name, owner.root, owner.manifest, targets, inputs]`. `implementationRevision` = `producer.currentImplementationRevisionV1()` (64 hex, the inference modules' revision); `observeProjectInferenceV1` additionally folds `OBSERVATION_REVISION` and `inferenceAuthorityDigestV1()` (the sealed module-load observations) into the record identity. The record's header stores every consumed fact (file stamps + sha256, directory listings, resolutions, source analyses); a hit requires each to hold. Manifest bytes and every inference input are therefore covered by what the computation actually read, not by a declared list.
- value: `slimBuildOwnerAdmissionFactsV1`: only `project.name`, `project.targets[*].dependsOn`, `files`, `dependencies`, `dependencyInputs`, `generatedDependencies`, round-tripped through JSON (what a hit returns equals what a miss returns). The closure reads nothing else of an owner's facts; generated Cargo target bodies (1,200 command inputs each) no longer inflate records.
- resumability: a record is written when its owner finishes (inside `observeInferenceComputationV1`), before the closure ends. A cancelled closure or a failing owner keeps all finished owners.
- sharing: `settleObservedComputationV1` gives the closure a shared validation scope (`inferenceValidationScopes`), so a fact consumed by many owners is probed once per run (`sharedObservationsHoldV1`).

### 2.3 (b) Closure only, and a receipt so that the closure is not re-derived

- Closure selection is `closeBuildOwnerAdmissionV1` unchanged in meaning: an owner is observed only after something in the closure admitted it; an unrelated broken owner is never inspected. The new law test proves it against an independent fixpoint and a full-graph-then-filter reference (section 4) and pins that `inventory()` never calls `workspaceBuildFilesV1` for a tasks selection other than the `admission?.files??` fallback.
- Diagnostics: `closeBuildOwnerAdmissionV1` records who first admitted each owner (`requestedBy`: nothing = selection, `{owner,target?}`, `catalog`) and `annotateBuildOwnerAdmissionFailureV1` appends `Build owner admission chain: shell-gen <- consumer:build <- selection` ONCE to the failing error's message. Identity, class and `code` are preserved (the error is mutated, not wrapped); cancellation and abort errors are left alone.
- Warm path: `selected/<scope>.admission.json` = `{version, scope, module{implementation,publication}, startedAt, admission, observations}` (schema `admissionReceipt` in `B/🧬️schema/🚪️admission.json`, closed, 512 MB cap). `inventory()` first tries `reuseAdmission()`: in-memory memo (the second call of a cold attempt) or the disk receipt; it requires scope + loaded module equality, `observationsHold(receipt.observations)`, the registry still hashing to `scope`, and a fresh membership digest of the admitted files. Anything else falls through to the normal derivation. A stale or foreign receipt can only cost the derivation it replaces.
- Publication: the receipt is written only after the graph and authority receipt were renamed (`:357`), best effort (a failure is swallowed unless the invocation was cancelled), and only when no consumed file is younger than 2 s before the closure started (racy rule, same constant as the observation layer). In-memory memoization obeys the same rule. It is NOT written early: `🔄️candidate` models "no side effect before inference" (`effects` = 0) and the per-owner records already provide resumability.
- Cold run: one closure derivation instead of two (`final` reuses the memo after `observationsHold`).

### 2.4 (c) Progress and cancellation

Per owner: `ports.check()` (= `checkScriptInvocation`) before and after, and `advanceScriptInvocation(invocation,"build:selected-records","running")` (publish + yield) so a pure cache-hit pass still prints a heartbeat per owner. Inside a miss the existing `yieldInferenceV1` ticks are unchanged. `settleObservedComputationV1` keeps the invocation signal. Verified: a closure cancelled after 2 owners leaves exactly those 2 records and the next run computes only the rest (cross-process test).

### 2.5 (d) One neutral graph for Nx and Bazel

All of it lives below `loadBuildGraphV1(root, invocation, phaseControl, {selection})`. `B/📜️script.ts:188` (every tool, Bazel included) and `L/⚡️caching/🚀️bootstrap/📜️script.ts:405` (Nx) call it with `buildInvocationGraphSelectionV1`, the scope key contains no tool, and the graph published and pinned afterwards (`SEMIO_BUILD_GRAPH_PATH`) is the same file. Test: both entry points are pinned by source law, and the selection and scope key are identical with `SEMIO_BUILD_TOOL=nx` and `bazel` (checked with `crypto.sha256`).

### 2.6 Why not more

- Cross-selection sharing (course `test` then `build`): records live in the selection's isolated writer directory by design (`buildGraphWriterOptionalPathAdmittedV1`), and the key includes the scope (the selected task's configuration enters `discoverBuildOwnerAdmissionFactsV1`). A first run of each distinct selection is cold. A shared base-facts store for `([],[])` observations (162 of 258 here) would need a non-isolated writer; left as follow-up F1.
- Cheaper base observations (162 owners admitted only by file ownership still run `cargoTargets`, `componentBuildTargets`, `readTaxonomyProjectionV1`): changes closure semantics; F2.
- Optional-cache quota: 1 GiB max / 512 MiB low water, shared by all scopes; the all-graph `np` store holds about 855 MB today and is retired oldest-first. Expect about 30-60 MB per selection for these records (one brotli header of shared observation rows each).

## 3. Staged files (all under `T/x-graph-tools/`, nothing live was edited)

| file | purpose |
|---|---|
| `apply.py` | `--check` (exit 1 pending, 0 applied, 2 refusal), `--live` (writes nothing unless EVERY anchor occurs exactly once or the edit is already present; then temp file + `os.replace` per file), `mirror DIR [--bare]` (scratch tree: byte copies of the library and framework modules along the patched paths, symlinks elsewhere, never a hard link) |
| `check.sh` | check on live, apply atomicity tests, fixture reproducibility, mirror, oracle `--verify`, bun suites, mutation proof, tsc |
| `test_apply.py` | 5 tests: pending writes nothing, live idempotent, one missing anchor refuses before any write, duplicated anchor refuses, partial tree completes |
| `gen_fixture.py` | deterministic fixture (7 hand written worlds, 30 seeded random worlds, 104 closure cases, key/chain/receipt/publishable vectors, 2 cache scenarios) + frozen oracle answers |
| `mutation-check.py` | 13 mutations of the mirror, each must turn the named test red |
| `profile-log.py`, `graphtime.ts`, `measure.sh`, `bench-receipt.ts` | log census, per-phase timer, two-pass live acceptance, receipt cost bench |
| `staged/admission.append.ts` | appended to `G/🚪️admission/🟦️.ts`: record key/leaf, `observeBuildOwnerAdmissionV1`, `slimBuildOwnerAdmissionFactsV1`, receipt create/read/publishable/path, chain + annotation |
| `staged/admission.defs.txt` | `$defs` `admissionCache` (constants), `observationRow`, `admissionReceipt` inserted into `B/🧬️schema/🚪️admission.json` (already in the graph module authority list) |
| `staged/graph.inventory.head.ts`, `graph.port.txt` | the `G/🟦️.ts` glue (reuse, memo, lazy ports, records) |
| `staged/closure-cache.{test.ts,fixture.json,schema.json,oracle.py,step.ts}` | new member `G/🚪️admission/🧪️tests/🗃️closure-cache/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json,🐍️oracle.py,🏃️step/🟦️.ts}` |

Live edits (21 operations): `G/🚪️admission/🟦️.ts` (7 anchored edits + append), `B/🧬️schema/🚪️admission.json` (1 insert), `G/🟦️.ts` (7 anchored edits: 2 imports, inventory head, closure port, memo, persist-with-graph, staging sweep regex `(?:\.authority|\.admission)?`), 5 new test files. Longest new path: 118 code points relative to the repository root, 146 absolute on this machine (256 rule).

## 4. Tests (test-first: written against the fixture before the glue; all green in the mirror, red on mutation)

Run: `sh x-graph-tools/check.sh` (about 60 s unloaded). Suite `closure-cache`: 14 tests, 3,300+ expects.

| law | independent check |
|---|---|
| fixture is closed under its schema | Ajv strict (2020) and Python `jsonschema` |
| closure = full graph filtered to the selection: owners, files, targets, inputs, providers, generated producers for 104 cases (nested roots, native roots, optional `^target`, object dependency forms, cycles, catalog providers, `all`) | Python naive least-fixpoint (`🐍️oracle.py`, no shared code, hashlib/jsonschema), `graphlib.alg.preorder` reachability (every admitted owner has a path from the selection), Ajv on the result |
| never inspects an owner outside the closure; an unrelated broken owner (2+ cases) never fails the run; a reachable broken owner (`shell-gen` through `generatedDependencies`, `bad` through `other1`) fails and names its chain | call log vs oracle owner set |
| record leaves/keys, domain separation from the project leaf, grammar `np/[a-f0-9]{24}.json` | hashlib sha256 over the same canonical JSON |
| requester chains incl. cycle and the 12-hop limit | Python chain walk |
| receipts: 9 vectors closed/open (version, scope, module, extra property, unknown kind, open admission, negative start); foreign scope/module refused; racy publication rule (8 vectors, boundary 2,000 ms) | Ajv + Python `jsonschema` agree with our reader on every vector |
| facts slimmed to what the closure reads; hit equals miss | JSON round trip |
| cross-process reuse (fresh `bun` per step, real observation module, real files): cold = every owner computed; warm = nothing; edit of `lib` = exactly `lib`; new dependency owner = `lib` + `extra`; registry digest change = all; implementation revision change = all; closure cancelled after 2 owners then resumed = only the rest, no key twice | disk state only; records, facts and rows written by earlier processes |
| graph attempt with mocks (real source of `loadBuildGraphAttemptV1`, transpiled): cold = 1 closure derivation, 1 inference, writes in order graph, authority, admission; warm = 0 derivations, 0 inference, 0 writes; stale facts or changed membership = 1 derivation; young facts = no memo, no receipt | operations log |
| Nx/Bazel share loader, selection and scope key | source law + sha256 |

Mutation proof (`mutation-check.py`, 13/13 killed): no persisted records, bodies kept in records, implementation / registry / targets missing from key or leaf, closure admits everything, ctime ignored, chain not appended, receipt scope unchecked, glue never reuses / forgets the memo / skips the fact check / persists before the graph.

Existing suites, control (unpatched mirror) versus patched mirror, same failing sets: `🚪️admission/🧪️tests` 1/0; `🛂️admission`, `🎯️source-demand`, `🔄️candidate`, `🔭️observed-facts`, `🧊️optional-cache`, `📌️pinned-publication` fail IDENTICALLY in both ("Original parent Script process invocation required": they need the repo test runner's process envelope, which a scratch mirror cannot provide; `🔄️candidate` transpiles `loadBuildGraphAttemptV1` into a `Function` whose dynamic `import()` calls resolve relative to the test file, so it is most likely stale already). They must be run by the peer through the normal runner after apply. `tsc --noEmit` over `G/🟦️.ts`, `G/🚪️admission/🟦️.ts`, the suite and the step harness: 0 diagnostics.

## 5. Measurements

| what | value | how |
|---|---|---|
| baseline cold dry-run | 429 s real, 367 s user, 258 observations, 162 owners, rc 1 | `dry-boot-2.log`, `profile-log.py` |
| receipt for 37,263 consumed facts (tree under load 46) | 13.6 MB; create + validate 218 ms; parse + validate 485 ms; `observationsHold` 410 ms; capture 23.6 s cold read | `bench-receipt.ts` against `🧰️framework` |
| estimated warm admission phase | 1-3 s (receipt read + validate + hold + membership lstat of the admitted files) | extrapolation, not measured end to end |
| estimated resumed cold run after a failing owner | finished owners at hit cost (record decode about 25 ms + shared fact probes), remaining owners at 1.4 CPU-s | extrapolation from the np record decode figure of `C/result-graph-perf.md` (10 s per 400 records) |
| NOT measured | real warm and cold dry-runs with the set applied | frozen; use `measure.sh` |

## 6. Apply order and acceptance

1. Prerequisite: none of the three live targets changed since 03:16/11:24; the set is independent of x-ingress (bootstrap), graph-perf (script-inputs, inputs, observation, seal) and bzl-lazy (adapter).
2. `python3 x-graph-tools/apply.py --check` (expect 21 pending, rc 1), `--live` (refuses with exit 2 and writes nothing if a peer moved an anchor), `--check` again (rc 0).
3. `sh x-graph-tools/check.sh` (also proves the live tree against the mirror it builds).
4. Run the existing graph suites through the normal runner (section 4, last paragraph).
5. After the peer lifts the dry-run ban and the typegen rule is resolved: `sh x-graph-tools/measure.sh` (pass 1: cold or partially warm, pass 2: warm). Acceptance: pass 2 prints `owner observations computed = 0`, `owner records consulted` may be 0 (receipt reuse), no `build:selected-declaration` events, graph phase in seconds; pass 1 prints at most one `selected-declaration` cycle per distinct `(owner,targets,inputs)` and never twice the closure. A second selection (`course:build`) is expected to be cold again (F1).
6. If the optional quota refuses writes (silently skipped publication), `owner records consulted` stays high on pass 2; check `used_bytes` of the optional ledger.

## 7. Open items

- F1 cross-selection base-facts sharing (needs a non-isolated writer or a seed step); F2 cheaper base observations; F3 taxonomy admission of the new test member and emoji (`🗃️`, `🏃️`, `🐍️`) is NOT verified (as for x-ingress); F4 `🔄️candidate` should be rewritten against the dynamic-import form of the attempt (the new "graph attempt" tests show the technique); F5 Windows: `check.sh` and the suite call `python3`/`bun` by name (use `py -3` there); F6 the typegen own-output rule is the peer's decision (coordination 16:39); until then no run reaches Bazel.

## Backend edits

None live. (Brief rule: generic backend only when blocking; this set is staged because `G/📥️inference/**`, the graph-adjacent storage layers and the admission schema are under the peer freeze.)
