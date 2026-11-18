# R156: a graph read that converges on a tree that never holds still (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R156/` (`land.py` + `patch_load.py` + `new/` the landing with a tested restore, `window.sh`, `battery.sh`/`tests.list`/`compare.py`, `run2.sh`/`run3.sh` the staged runs). Landing: `landings.md` "R156 landing 1" (22:22:40 to 22:22:51 UTC, hold 11 s). Scratch: `scratchpad/R156/`.

## 1. The wall and the model

R143 wall 32 / R151 section 1: the attempt of a graph read ends with "does every observed row still hold?". On a tree that 20 agents edit every few seconds the answer at the end of a 200 to 390 s start phase is almost always no, so the attempt restarts from the admission closure (R151 added up to 4 inference-only repair rounds, then the old restart). Measured on the churn model of this package (section 4): the old code restarts in **83 of 200** seeded worlds.

**Model (implemented in `loadBuildGraphAttemptV1`).** The graph of a read is the set of per-owner records the owner loop produced. Each record is valid for exactly the rows its inference observed (its own files, the foreign file facts of R141, the stamps of the inferrers that ran; the record key is untouched). A read seals when every record was valid *at its observation* and the cross-owner references resolved (the discovery that builds the graph would have thrown otherwise). It does not require one instant at which the whole tree was unchanged:

- A round follows the tree: if rows are stale, the membership moved or the owner closure moved, the next round runs on the **fresh inventory** (`basis`), not on the old one. Which owners work again is decided by the existing laws only: the record of an owner holds or does not (its rows), the admission facts memo (R151) re-observes only owners whose rows changed.
- After `repairRounds` (4) the read **seals with the records it has**. The verdict says how much changed afterwards: `[graph-seal] {mode, rounds, sealedWithLaterChanges, laterChangedOwners, membershipMoved, admissionMoved, owners}`, the same verdict plus the per-owner digests is in the authority receipt (`seal`), and `slice-memory.json` of the worker carries `sources.seal`.
- The receipt keeps the rows as they were observed. The next read compares them to the tree, finds exactly the owners whose rows differ, and works on those only (law: stale in the next read only).

### Why this is sound for build correctness

- A task never trusts the graph for its inputs. The hash of a task is computed from the files its declared inputs match **at execution** (the premise of the brief, not re-verified in the hasher code in this package); a graph node that was inferred from an older version of a file only describes *which* inputs and commands the task has, and a task whose declared inputs moved is hashed on the moved inputs. A graph that was sealed 3 s before an edit therefore never serves a wrong cache entry; it can at worst select a task or an input set that the next read corrects. This was already true for every graph (the old sealed graph is stale a second after sealing as well); the model only stops pretending the instant was shared.
- The sealed generation promises: *every owner record in this graph was valid for the rows listed in the receipt when it was taken, the membership digest is the one of the basis the graph was computed on, the cross-owner references resolved*. It does not promise: no file changed since. The receipt lists the rows, so a consumer can decide per owner.
- A graph read that sealed with later changes is not served by the `sealedOnly` reader (its receipt no longer holds), so a query never gets a graph the tree has already left without the next read repairing the stale owners first. That is the behaviour of every sealed graph a moment after sealing, and the cost is bounded (section 5).
- A **pinned implementation** (a file the process loaded, changed on disk) cannot be refreshed by repeating: R154's rule is kept: a round that leaves exactly the same stale rows throws `observation.staleSourceFactsErrorV1` and the process hands over.

### Where a stronger guarantee is needed: publication to the fleet

`seal.owners` maps every owner (holder of the file, `(workspace)` for rows outside any owner) to the sha256 of its observed rows. The generation head that a machine publishes should name these digests, so that another machine accepts **each owner independently**: it takes an owner record when the digests of the rows it would observe equal the published ones, and re-infers the others. That wiring belongs to R155 (cross-machine acceptance); this package provides the verdict and the digests in the receipt and the law that they are the digests of the rows (independent recomputation). One honest limit: the digest groups rows by the **holder** of the file (`createProjectFileIndexV1` over the roots), not by the observing owner; the observing owner is known only inside `discoverBuildGraphV1` (`onOwnerObservations`), the flat row list that reaches the attempt does not carry it. For acceptance by holder this is enough (a changed foreign fact changes the digest of the holder); a per-observer digest needs the callback threaded out (open item 1).

## 2. Every retry site, classified (grep of the primitive: `invalidated(`, `[graph-retry]`, `return undefined` of the attempt, `settleObservedComputationV1`, the attempt loop)

| site (before) | trigger | now |
|---|---|---|
| `invalidated("membership")` at the first inventory | a member vanished while the inventory was taken | in-place retry of the inventory (`settledInventory`, bounded by `repairRounds`), then the typed refusal; no restart |
| `invalidated("observed-facts")` at the end | any observed row stale after the repair rounds | **seal** with `sealedWithLaterChanges` |
| `invalidated("membership")` at the end | file list or modes moved | followed by a round on the fresh basis; at the bound, **seal** with `membershipMoved` |
| `invalidated("admission")` at the end | the owner closure moved | same, `admissionMoved` |
| `invalidated("producer-error")` | the discovery failed while facts moved | `[graph-repair] failure`, round on the fresh inventory; rethrown when the failure is genuine (facts hold, basis unchanged) or at the bound; `BUILD_AUTHORITY_CHANGED` still goes straight up |
| `invalidated("publication-facts")` before the rename | a row changed between verification and the rename | removed: that is a later change by definition |
| `return undefined` after a cached receipt whose admission rows no longer held | | falls through to discovery |
| `invalidated("owner-registry")` (3 places) and the 9-attempt outer loop | the registry file changed, so the scope key (hash of selection AND the whole registry) changed | **kept**: structural, it is a different graph object (a different scope file), not a repeat of the same read; rare (3 commits of `📋️build-owners.json` in 7 days). Open item 3: the key is the whole registry and the owner-facts memo is keyed by it, so one registry edit re-observes every owner once |
| `sealedOnly` returning undefined | no current sealed graph | kept: it is a lookup, not a retry |
| `settleObservedComputationV1` (64 rounds) | rounds of the old non-snapshot inference | not on the worker path: the graph read calls `discoverBuildGraphV1(..., true)` which sets `snapshot:true`, and the snapshot branch uses `observeInferenceSnapshotV1` (no settle). The `[graph-settle]` line of R140 comes from the non-snapshot callers |
| `loadGenerationGraphV1` (the `all` scope and the generation path) | | already per leaf (`settleClosure` marks leaves stale, one invalidation) |

After the landing the only way the attempt loop repeats is the registry; `Build source authority keeps changing during inference` can only come from nine registry changes in one read.

## 3. Tooling edits and the semantics revision (item 3)

What already existed and what this package proved:

- Records are keyed by **declared stamps** (`🔖️stamps`: 18 inferrer groups with `semio.graph-inferrer.<id>/<n>`, infrastructure fragments `none`), not by the bytes of the code; the unclassified-code law keeps every executable file of the loaded implementation claimed. New law `🔧️tooling-edit` (200 seeded worlds, the real `classifyImplementationRowsV1` over the declared protocol and the fixture rows): an edit of a covered file leaves the classified rows (and with them every record key) byte-identical; an edit of an uncovered file changes them; a stamp bump changes exactly the row `inferrer:<id>`. This law is green before the landing (it pins existing behaviour, there is no red state to show).
- The graph module's own authority (`loadedGraphModuleAuthorityV1`, the receipt and memo key) is the digest of a **bounded list** of files (`graphSelfUrlsV1` plus roster, observation, digest reader): the load library and its neighbours, not the tooling closure. It changes with every landing in those files (this one included, once).
- **Not solved (R151 open item (c)):** the admission facts and the records of native owners still observe the files of the command-source closure as content rows (they are executable inputs of the task, R141). An edit in that closure re-observes every reader once. After this landing it no longer restarts the read: it costs one bounded re-observation per reader (law: observations of an owner at most 1 + changes to its manifest + changes to the shared tooling file) and the read seals with the rest.
- **Semantics-revision law over a fixture corpus (compare results; fail if results differ and the revision did not change): NOT built.** It needs a golden-output runner per inferrer over a corpus, which only exists for the two *derived* inferrers (`input-precision`, `package-managers`: the conformance corpus settles their stamp from results). For the other 16 the stamp is bumped by hand; the law that would fail on a missed bump needs the `📦️projects` real-inference machinery (SDK, artifact namespace) that I could not run under a loaded machine in this package. Plan: one corpus workspace, run the real discovery once per commit of the stamp table, pin `sha256(canonical nodes)` next to the stamp row; law fails on (digest differs and no stamp changed).

## 4. Laws (red first) and numbers

All against the real `loadBuildGraphAttemptV1` (extracted from the source, bindings model the tree) and the real owner-facts memo; third-party cross-check: Ajv validates the verdict and the receipt against the schemas, an independent recomputation of the per-owner digests, the typescript compiler extracts the function.

- `🧪️tests/🕸️graph/🌊️seal-under-churn` (3 tests, new). A model tree of 3 to 10 owners whose files change on a logical clock (period 2 to 600 ms, a random file each time, seeded; each lookup 1 ms, an inference 12 ms, an observation 3 ms, a hold check 1 ms):
  1. 200 worlds: the read seals (`defined`), at most `1 + repairRounds` discoveries (bounded time), no `[graph-retry]`, inferences of an owner at most 1 + changes to its source, observations at most 1 + changes to its manifest + changes to the shared tooling file (work proportional to the changes), `sealedWithLaterChanges` and `laterChangedOwners` equal an independent recomputation over the receipt and the final tree.
  2. 200 worlds: a second read after the churn stops re-infers exactly the owners whose source was stale at the end of the first (each once), seals `quiet` with 0 later changes; a first read without later changes is served from its receipt with no discovery.
  3. seal and receipt satisfy their schemas, per-owner digests equal the independent recomputation, a bad mode is refused.
  Red on the unchanged code: world 1 is not sealed (`undefined`). Numbers over the same 200 worlds, old code: **83 restart, 117 seal**; new code: **200 seal** (80 `quiet`, 41 `repaired`, 79 `sealed-with-later-changes`), 581 discoveries in total, at most 5 per read, 11560 model changes.
- `🧪️tests/🕸️graph/🩹️repair-rounds` 11 (was 10): the tests that expected a restart now expect a seal; a followed membership; a membership that moves at every round seals with `membershipMoved`; the seal line and the verdict.
- `🧪️tests/🕸️graph/🧊️stale-facts` 2 (R154's law): kept (a pinned fact ends the attempt as stale implementation after one round); the second test moved to the contract (a fact that moves at every round is sealed).
- `🕸️graph/📥️inference/🔖️stamps/🧪️tests/🔧️tooling-edit` 1 (section 3).
- Battery of 11 directories under the lock: none worse; `🔄️candidate` 1/2 and `📥️birth` 7/8 are baseline reds (unchanged); `🔓️sealed-reuse` (text-order law on the attempt) 14/0 after the first try was restored on one anchor (`let captured` right after the sealed-only exit); standing preflights green after the landing; the stamps main file has one red (`no record ... keyed by the bytes of the implementation`) that reads observation/host/generations text, none of my files (not investigated).

## 5. What the workers will show (watch list)

Reads on the new code print `[graph-repair] {round, stale, staleCount, membershipMoved, admissionMoved}` per round and one `[graph-seal]` line; `slice-memory.json` of the worker has `sources.seal` (`mode`, `rounds`, `sealedWithLaterChanges`, `laterChangedOwners`, `owners`) next to `owners.pass`. Expected on a busy tree: `owners.pass` per process = 1 + number of repair rounds (at most 5) and no `[graph-retry]` except `owner-registry`. The service logs stay on the workers, the store shows `slice-memory.json`. At 23:25 CET no worker ran the new code yet (g1-m02 stood at `taken 0 of 4455` after 18763 s: a process of the old code; the stale-stage rule of R151 hands each over after 30 minutes of age). I will re-read the store after that and append to section 7 below.

## 6. Not verified, honest list

- No real read ran on the new code (machine load 40 to 58, the lab belongs to three other agents, the rules forbid the lab and a probe of a scratch edit in the live tree). The model laws drive the real function; the measurement of `owners.pass` and `[graph-seal]` on a worker is section 7.
- A round still performs the owner **lookups** of unchanged owners (the discovery iterates the roster, records hold-check their rows) and the inventory hold check of all admission rows twice per round (`inventory()` and the final hold). This is a per-round fixed cost bounded by 5 rounds, not proportional to the changes; the work that is proportional to the changes (inference, observation) is what the laws bound. The owner carry (skip unchanged owners inside a round) is still R151's open item and needs the owner loop of `📦️project-nodes` to accept the stale paths.
- Per-owner digests are by holder of the file, not by observing owner (section 1).
- `repairRounds` 4 is a declared number, not tuned on a real churn rate.
- `tsc` over the changed file was not run (the file imports through emoji paths of the whole library); only the extraction transpile and the tests ran. One type alias (`Basis`) and a definite assignment (`let captured!`) are the only new type constructs.

## 7. What others must know

- R154: your `unrefreshed-facts` rule and the `[graph-retry] reason unrefreshed-facts` line are in the new loop unchanged in effect (round counting and the signature are mine, same contract; your two tests pass; the second test now asserts a seal, not `undefined`).
- R155: the receipt has `seal.owners` (holder-grouped digests); acceptance of a record on a second machine can compare them. Machine-specific values in rows (paths below a state root) would show up as a different digest on every machine: that is yours to keep out of the row facts.
- R153/R149: three text/extraction harnesses read this function: `repair-rounds`, `stale-facts`, `🔄️candidate` (its second test is a baseline red; it does not reach the sealing code). A new free identifier in `loadBuildGraphAttemptV1` needs a binding in them (`sealVerdictV1`, `reportSealV1` are the two new ones).
- The repair bound `repairRounds` now means "rounds before sealing", not "rounds before restarting" (EN/DE text of the limit changed).
- Open items for a next package: (1) thread the per-observer rows out of `discoverBuildGraphV1`; (2) owner carry inside a round; (3) owner-granular scope/memo key instead of the whole registry; (4) the command-closure indirection (R151 c) and the corpus semantics law (section 3); (5) generation head names `seal.owners` (R155).

## 8. Watch log

(appended below after the workers pick up the new code)
