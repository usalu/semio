# Design H102: incremental, per-owner graph publication (the seal stops being a gate)
Deutsch: Entwurf H102: inkrementelle Veröffentlichung des Graphen pro Owner (das Siegel ist kein Tor mehr). Prototyp + Orakel + Simulation: `T/🧰️scripts/H102/` (`run.sh`). Nothing landed; H101 lands, this lane designs and stages only.

## 1. Why the seal cannot converge today (measured, not guessed)
- Read 10 log (`scratchpad/H84/graph.log`, 53k lines): stale-edge lines `1, 1, 48, 29, 518` (first read) and `1, 59, 219, 292` (after the 16:00 landings); 4 `refreshing 5093 projects (generation 261)` starts; 5 code hand-overs ("this read runs code X but the library is at Y; it hands over after 5358 of 10403 owners"). Every restart re-ran the first wave (`inferring 311 projects`, `storing 639 fragments`) again.
- Mechanism in code: an owner row carries ONE stamp `edgesEnv` = digest of the whole environment (`🕰️generations/🟨️.mjs` lines 349-351, 374). `staleEdgeOwnersV1` compares by string equality, `requireEdgeEnvironmentV1` refuses publication while any non-pending row differs. The environment moves whenever ANY owner row, package, module, workspace manifest, lock or the taxonomy moves, so every wave re-stales every row stamped earlier; the "carry" step re-stamps non-consumers, but only inside a read, at one instant, and the check wants zero stale at that instant.
- The second global gate is the code digest: "library at ?" is the digest of the whole build library; a landing in `⚡️caching` or `🏃️execution` hands over a graph read although the inference closure does not import them (`🧰️scripts/H102/impl-closure-probe.mjs`: static closure of `🕸️graph/🕰️generations/🟨️.mjs` = 156 files, 55 inside `🕸️graph`, 0 in `⚡️caching`, 0 in `🏃️execution`, 0 in `🌍️environment`; build library has 231 non-test `.mjs`).
- Third gate: the structural custody claim of the whole read and the single manifest CAS (H38): one writer, hours long, one publication.
- Already good and kept: the edge reads recorder (`🧵️edge-reads`: owners, paths, packages, modules, workspaces, lock) and `diffEnvironmentV1`/`edgeConsumersV1` are a decomposition of the environment per owner; they are only used to decide what to REFRESH at a read, never as the validity rule of a row. This design promotes them to the validity rule.

## 2. Principle
A published owner record is valid for a reader iff the components of the environment it actually read, and the code components that actually produced it, have the digests it saw. Validity is a pure function of (record, pinned environment); nothing is stamped, nothing is re-stamped, no read is needed to "make it consistent".

## 3. Environment decomposition (prototype: `environment-components.mjs`)
Component keys and what they digest (code-point order, sha256 of canonical JSON array):
| key | digest of | read by a record when |
|---|---|---|
| `T` | taxonomy digest | always (taxonomy moved = everything stale, as today `all`) |
| `o|name` | the row `[name, root, nativeRoot]` or `-` | self, and every owner its edges resolved to (`reads.owners`) |
| `t|root` | rows whose root lies strictly inside `root` (its territory: a new owner takes files) | own roots, roots of resolved owners |
| `c|path` | rows whose root is related (ancestor, equal, descendant) to a directory it looked up without finding an owner | `reads.paths` |
| `p|pkg`, `m|mod` | owners the package / Go module name maps to | `reads.packages`, `reads.modules` |
| `w|dir` | manifest digests at the workspace directory | `reads.workspaces` |
| `W|root` | workspace manifests related to the owner's roots (a nearer manifest appears) | owner has `reads.workspaces` |
| `L` | lock digest | `reads.lock` |
| `*` | the whole table | no recorded reads (as today: "a candidate without recorded reads is affected") |
`record.keys` is fixed when the record is made (sorted list), `record.readsDigest = H([[key, digest(key)]...])`. The reader recomputes the digests of exactly those keys under its pinned environment: O(keys) lookups on a once-built index (`indexEnvironmentV1`), memoised; no owner table scan per record (binary search on the sorted root list).
Re-stale rule: `stale(record, E) = readsDigest(record.keys, E) != record.readsDigest`. An environment change re-stales exactly the owners whose recorded components changed.
Implementation components (same scheme, second vector `record.implementation`): `analysis` (provider ids + revisions, already versioned), `facts` (codec + schema), `edges` (edge deriver), `closure` (closure walker, module resolution), per-language inference (`rust`, `go`, `ts`), each = digest of the static import closure of its entry module (probe above), NOT of the whole library. A landing outside the closure moves no component, so no hand-over and no freeze.

## 4. Record schema (content addressed, two layers)
Layer A, inference record (expensive; keyed by what inference reads): `{kind:"inference", name, inputs, impl, facts, fragment}`; `inputs` = digest of the owner's observed fact rows + targets + config (the per-owner hold already vouched by journal-clean, H21), `impl` = digests of the components that inference uses. `id = H(name, inputs, impl)`.
Layer B, edge record (cheap; seconds): `{kind:"edges", name, inference:<A.id>, keys, readsDigest, edges, fragment, seq, by}`; `id = H(name, A.id, readsDigest)`. Edges never depend on inference facts (documented in `edge-heal`), so an environment change creates only new B records (the existing "edge-only refresh"), never new A records.
Store: `S/graph-records/<name>/<id>.json.br` (immutable, atomic temp+rename) + per-owner head `S/graph-records/<name>/head.json` = newest B id with its `seq` (lamport); a head update is a CAS on one owner. Two builders producing the same record write the same bytes (idempotent); different valid records for one owner are resolved by newest `seq`. No structural custody: the custody claim of a whole read disappears, H38's overlay becomes the only mode (everything publishes at owner granularity).
Bilingual texts (all en then de): `graph: owner <n> verified (record <id8>)` / `graph: Owner <n> geprüft (Eintrag <id8>)`; `graph: <k> of <n> owners usable (<p> %) under environment <e8>` / `graph: <k> von <n> Ownern nutzbar (<p> %) unter Umgebung <e8>`; `graph: owner <n> is stale: <component> moved` / `graph: Owner <n> ist veraltet: Komponente <component> hat sich bewegt`.

## 5. Consistency model for readers
1. The reader pins ONE environment `E` (cheap: owner table, packages, modules, workspaces, lock, taxonomy from the live scan or the newest published parts) and its own implementation vector.
2. Per owner it takes the newest B record valid under `E` (`assembleViewV1`). Every chosen record was derived from exactly the components `E` shows, so the set is a consistent snapshot of `E` by construction (an edge to owner X was derived against X's row; if X's row differs in `E` the record is invalid and not chosen). No cross-owner transaction needed.
3. Owners with only invalid records: `stale`, the reader derives the edges itself in memory (edges-only, as H38's overlay) and may publish the result; owners without records: `missing`, inferred narrowly on demand (overlay) and published. A narrow read therefore never waits for anything, whatever the background verifier is doing.
4. Generation id = `H(E.tableDigest, sorted leaves H(name,id))` (merkle over owner record ids; the prototype uses a flat digest of the sorted leaves, a tree is a pure refinement for O(log n) incremental update). Two readers with equal `E` and equal chosen records have equal generation ids without coordination; the fleet status shows coverage `usable/total` instead of "generation null".
5. Request closures: a command needs only the records of its owner closure (`graph_policy=current` today). It is ready as soon as ITS closure is valid; the whole-graph `coverage` is informational. This also removes the "stale closure waits for the whole read" case of H38/H56.
Anomalies excluded: a record chosen for owner A can never mix environments (all valid under `E`); a record valid under `E` but computed by a later environment is fine (digest equal means components equal). Rollback of an edit simply re-validates older records (their digest matches again): no recompute.

## 6. Whole read = background verifier
It iterates owners in priority order (stale of live commands' closures, then stale, then missing, then least recently verified), each verified owner is published immediately (head CAS). It holds no custody, can be killed any time (progress is the store), may run on several machines at once (fleet-fetched records dedupe by id), and is restarted by nothing: an environment or code change just makes some records stale, the verifier's queue is recomputed by `staleOwnersV1` (O(records × keys)). Verifier completion = `usable == total`, reported, never awaited. A periodic full audit (every N hours) re-verifies a random sample of "valid" records against fresh facts to catch a wrong recorder (reads under-reported = silent staleness), see risks.

## 7. Fleet-fetched owner records
Records are self-describing and content addressed, so the existing `🌐️remote-exchange` (now all-or-nothing: "the fleet exchange of the whole workspace is not usable (claims)") is replaced by per-owner exchange: a machine asks the store index for `head(name)`, fetches the B record by id, verifies `id == H(content)`, recomputes `readsDigest` under ITS pinned `E` (so a record from a machine with a different environment is accepted only if the components it read agree), and verifies the A record's `inputs` through its own journal-vouched fact hold (H21; facts differ per checkout if the tree differs). Valid -> used and cached locally, invalid -> stale/missing as above. No trust in the publisher beyond content: the implementation vector in `impl` must equal the reader's, else the record is ignored (no cross-version leakage). Fetch is lazy per owner closure, so a 16 GB worker never downloads the whole graph; all machines on one hot cache see each other's records within the store latency.

## 8. Migration-free cutover (CLAUDE.md: greenfield, no legacy)
- Generation 260's manifest already stores per owner `{root, kind, fragment, edgesEnv, targets, reads}` and the manifest stores `envParts`. A record view over a manifest row is computed on read: `keys = recordKeysV1(row, reads, E260)`, `readsDigest` from `envParts` of that manifest. No rewrite, no script, no dual format: the first new-code read finds valid records (rows with `reads`) or treats rows without `reads` as `*` (stale as today), then the verifier publishes records and heads.
- The `edgesEnv` stamp, `carry.healed/edgeTried/healInferred`, `edgeHealPlanV1`, `requireEdgeEnvironmentV1` and the hand-over-on-library-digest are DELETED in the same changeset (no compatibility layer); `generation` in fleet status becomes the merkle id; the generation manifest becomes a derived snapshot written by the verifier for bootstrap/export only.
- One changeset in the landing train: (1) `🕸️graph/🧬️schema` record + head schemas, (2) store + head CAS member, (3) `environment-components` + oracle test as `🧪️tests` (the Python oracle and fixture go in unchanged), (4) reader assembly in `🔌️load`, (5) verifier replaces the whole-read driver, (6) delete the stamp code, (7) implementation vector with import-closure digests (the probe becomes a member), (8) fleet status/MCP texts. Land order for H101: 3 (tests only, any time), then 1+2, then 4+5+6+7 together.

## 9. Evidence from the prototype
`run.sh` (oracle -> check -> simulation), real numbers:
- JS rule == CPython oracle on 300 random worlds / 12,000 records: keys 0 mismatches, readsDigest 0, flagged set 0 mismatches, generation id order-independent (0 mismatches).
- Soundness vs ground truth (toy edge function reading the same lookups as the recorder): 3,619 truly changed edges, 0 missed by the component rule. Stale per environment change: global stamp 100 %, today's consumers rule 39.8 %, component rule 39.0 %, ground truth 30.2 %. So the precision of the per-owner read set equals today's consumer rule; the gain is NOT fewer stale owners per change but that they are stale individually and keep their record instead of forcing a whole-graph stamp.
- Today's rule vs the toy truth missed 18 changed owners in worlds with workspace/lock edits (UNVERIFIED lead: toy recorder semantics may differ from the real recorder; check `edgeConsumersV1` workspace + lock branches against the real recorder before landing).
- Simulation over the read-10 log (`simulate.py`, 100 runs x 24 h; stale-set sizes sampled from the real series 1,1,48,29,518,1,59,219,292; N=5093; assumptions: env move every 20 min, library landing every 100 min of which 10 % touch a component and 30 % of owners, edge refresh 100 owners/min, inference 6/min, 18 min fixed cost per heal round (a round re-enters verification), 25 min verification):
  | scenario | today: publications/day, runs without any | owners usable (mean) | per-owner: owners usable (mean), time >= 99 % |
  |---|---|---|---|
  | base | 1.0, 62/100 | 0.39 | 0.92, 48 % |
  | env 2x faster | 0.0, 99/100 | 0.18 | 0.86, 20 % |
  | env 2x slower | 4.8, 17/100 | 0.60 | 0.93, 57 % |
  | code 2x faster | 0.3, 83/100 | 0.31 | 0.87, 35 % |
  | 30 % of landings relevant | 1.0, 62/100 | 0.31 | 0.78, 18 % |
  | 100 % relevant | 1.0, 62/100 | 0.19 | 0.43, 0.3 % |
  | no relevant landing | 1.0, 62/100 | 0.43 | 0.97, 67 % |
  | inference 3x faster | 6.7, 1/100 | 0.72 | 0.98, 83 % |
  | edges 10x slower | 0.3, 82/100 | 0.34 | 0.91, 39 % |
  Base reproduces the observed "no seal in ~20 h" (62 % of simulated days never publish). The gain depends on code relevance: with whole-library digests each landing is relevant; with import-closure components only the few that touch inference are. If every landing changes inference semantics (100 % row) per-owner publication is still better than 0.19 but needs inference speed: the real fix there is inference throughput, not the publication model.

## 10. Risks / open points (nothing here is proven live)
1. Under-reported reads = a record that stays valid when it should not. Mitigations: `*` for rows without reads, periodic sample audit, differential mode in CI (recompute edges for a sample of "valid" records and compare), the oracle tests.
2. Record count: 5093 owners x versions; retention = keep the newest valid per owner + the ones referenced by a pinned view; sweep by `seq`.
3. Head CAS across machines on the shared store needs the atomic rename guarantees H93/H94 rely on; HFS+ has no RENAME_SWAP (memory: native-cache images), use link/rename of unique files + directory listing newest `seq`, not in-place overwrite.
4. Tagged owners (tests, policies) are rediscovered as a group today (`staleEdgeOwnersV1` `tagged`); the design needs their group records (`kind:"group"`, key = member ids) — not prototyped.
5. Simulation parameters are assumptions (log has no timestamps / per-owner reads); only stale-set sizes, owner count and wave sizes come from the log.
6. Implementation-component closure by static imports misses dynamic imports/worker entries; the member must include declared dynamic entries (list) and the probe must fail closed on unresolved specifiers.
