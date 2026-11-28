# Result H102: incremental per-owner graph publication (design + prototype, nothing landed)
Deutsch: Ergebnis H102: inkrementelle Veröffentlichung pro Owner (Entwurf + Prototyp, nichts gelandet).

## Delivered
- `W/design-H102-incremental-graph-publication.md`: root cause of the non-converging seal (one `edgesEnv` stamp per row, whole-library code digest, one structural custody + one manifest CAS), record schema (inference record A + edge record B, content addressed, per-owner head CAS), environment decomposition into component keys, reader consistency model (pin one environment, newest valid record per owner, generation = merkle of record ids), whole read as background verifier, fleet-fetched records (per-owner, verified by id + readsDigest + local fact hold), migration-free cutover (generation 260 rows read as records, stamp code deleted, one changeset for H101), en/de texts, risks.
- Staged in `T/🧰️scripts/H102/`: `environment-components.mjs` (pure: component index, keys, readsDigest, re-stale rule, view assembly), `oracle.py` (independent CPython implementation + toy ground-truth edge function + world generator), `check.mjs` (JS vs oracle vs ground truth vs today's `diffEnvironmentV1`/`edgeConsumersV1`, imported from the real library read-only), `simulate.py` (log-calibrated simulation), `impl-closure-probe.mjs`, `run.sh`.

## Measured (ran here, nice -n 10)
- 300 random worlds / 12,000 records: JS == oracle (keys, readsDigest, flagged set, order-independent generation id: 0 mismatches); soundness: 3,619 truly changed edges, 0 missed. Stale per environment change: global stamp 100 %, today's consumers rule 39.8 %, component rule 39.0 %, truth 30.2 %.
- Static import closure of the generations module: 156 files, 0 in `⚡️caching`, 0 in `🏃️execution` (so those landings should not hand over a graph read; today they do via the whole-library digest).
- Simulation over the read-10 log series (stale sets 1,1,48,29,518,1,59,219,292; 5093 owners): today ~1 publication/day with 62/100 simulated days never publishing (matches the observed no-seal); per-owner publication keeps 0.92 of owners usable on average vs 0.39, >= 99 % usable 48 % of the time vs 2 %. Full sweep table in the design (section 9).

## Not verified / honest limits
- Not wired into the library, not run against a live store; no live proof of any benefit.
- Simulation rates (env move 20 min, landing 100 min, edge 100/min, inference 6/min, 18 min per heal round) are ASSUMPTIONS; the log has no timestamps or per-owner reads. When every landing changes inference semantics the gain shrinks (0.43 usable) and inference throughput is the lever.
- Toy ground truth: today's rule missed 18 changed owners in workspace/lock worlds; may be a toy-recorder difference, check against the real recorder before relying on it.
- Tagged-owner group records, retention, head CAS on HFS+ and the dynamic-import closure of implementation components are designed only as open points.
- H101: tests (oracle + fixture) can land any time; the runtime change is one changeset after review; nothing was handed to H101 yet.
