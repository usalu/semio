# obsrows-g: the nine Nx-gated build-library files (19:00-19:50, resumed after the 18:50 restart)

Library edits are STAGED only (nothing live). Batch: `obsrows-g-tools/batch/batch.json` (15 files, 0 new files), applied with
`python3 -I obsrows-g-tools/stage.py check|apply|revert [--root DIR]`; `sh obsrows-g-tools/verify.sh check|selftest|suites`.
State on the live tree at 19:50: all 15 files `base` (no drift); apply/revert round trip on a scratch copy: applied == staged, revert == live.
Only single-file suites were run (no official build/test run). Raw logs: `🗑️generated/obsrows-g/` (r4..r11).

## How the files were run (important)

- Nx-gated tests need the env the Nx adapter sets (`SEMIO_NX_FIXED_STORAGE=.../fixed/m`, `NX_CACHE_DIRECTORY=.../fixed/m/nx/artifacts`).
  No genuine birth supplies it: `orchestration` (`run parent:build`) only builds capabilities, it never launches Nx
  (nx-engine: 14 pass / 5 fail under `orchestration`, 19 / 0 with the two variables). So a `fileLaunchers` entry cannot make these files green;
  they must run under the Nx route (or the route must export both variables). route2-g: do not route these via a birth.
- Staged module edits are served with a name-keyed mirror; a staged `👁️observation/🟨️.mjs` in the mirror splits the module instance from the live
  plugin (2 false reds: "Rust and native closure memo" and "observed TOML", both PASS with `SKIP_STAGED=🏗️build/👁️observation/🟨️.mjs`).
  After the batch is applied live there is one instance; no action needed.

## Status of the nine (+ reuse)

| File | live (Nx env) | with batch | Notes |
|---|---|---|---|
| 👁️observation/🧪️tests/🟦️.ts | 41 / 52 (11 red, setup-sdk, 18:30) | 52 / 52 (effective) | roster rows, decoded blocks, racy-window semantic, damaged-block message regex |
| 👁️observation/🧪️tests/📥️reuse/🟦️.ts | not run live | 12 / 12 | AST proof follows the `&&`-composed `reusable`, memo statements transpiled with the real policy |
| 🧪️tests/🕸️graph/🔌️nx-engine | 19 / 19 | 19 / 19 | no edit needed; red only without the Nx env |
| 🧪️tests/🕸️graph/📐️portable-model | 6 / 7 | 7 / 7 | projection of a non-cacheable output (`dist`) now carries `cache:false` (nxOutputsCacheableV1) |
| 🧪️tests/🕸️graph/🧭️portable-cli | 11 / 15 | 15 / 15 | cargo target moved to lane `m/u/l/d/t` (fixture x5), dashboard `run-many` has a third word, root publication wrapper now imports `🏛️sealed/📥️execution` lazily |
| 🧪️tests/🕸️graph/🛂️admission | 9 / 20 | 11 / 20 | see below, 9 open |
| 🧪️tests/🕸️graph/🧊️optional-cache | 5 / 13 | 6 / 13 | quota policy gained `retirementWindowMilliseconds` (fixture); pinned snapshot is now a record manifest (decoded with `decodeGraphRecordsV1`); then 1 text-order law + 6 lease waits, see below |
| 👁️observation/🧪️tests/⚡️incremental | 2 / 4 | 2 / 4 | ENOENT fixed (staged product edit, proven by redirect), next defect below |
| 👁️observation/🧪️tests/📦️projects | 0 / 3 | 0 / 3 | same |
| 🏃️execution/🧪️tests | red | not green | needs the `target-execution` birth (staged) and a route entry; preparation exceeds the 120 s ceiling under load |

Not green after this round: admission (9), optional-cache (7), incremental, projects, execution. Nothing is claimed green that was not run.

## Real product/test defects found

1. **Cold dataset read crashes (product, staged fix).** `observationPhysicalReadV1` tolerates an absent directory only when a path segment is
   `np`, `ip` or `rr`; `<dataset>/inference/emoji-project-json.json` (invocation script, cachePath = data dir + `inference`) has none, so the first
   read of a fresh dataset throws ENOENT instead of a cold miss (fresh machine, `.cache/b/g/m/inference/inference` never exists before the first write).
   Staged: `"inference"` added to the tolerated names (one token, `👁️observation/🟨️.mjs`). Proven effective: with a `node` shim that serves the staged mirror
   to the Node children (`🗑️generated/obsrows-g/shim/node`, `obsrows-g-tools/redirect.mjs`) incremental and projects move past ENOENT.
   NEXT failure in both children: `Error: Source input must be repository relative` from `captureRepositorySourcePathV1`
   (`📥️inputs/📋️data/🟨️.mjs:10`) in `compactSourceInputs`; the tests run the child with `cwd = <artifact temp root>` and `workspaceRoot = process.cwd()`,
   the inventory insists on a repository-relative path (no `:`/absolute/`..`). Offending value not yet printed (the module is a data: URL copy). Owner: graph/inventory lane.
2. **Type-only imports are intentionally not command inputs** (`⚡️caching/📥️inference/🟨️.mjs` scope `command`, test `runtime-type-erasure` in
   source-demand, fixture `erased.ts`). The admission fixtures still used `import type {Thing}` for the dependency edge. Converted to runtime imports in the
   fixture and inline sources (+2 green: type-only/uncached and configured original command sources). Negative law (type-only edge stays out) is covered by
   source-demand, not duplicated.
3. **Admission still red (9)**, pattern: dependency edges / `^named` input propagation do not appear although the owner and files are found:
   `facts.dependencies = []` for app although `library/api.ts` is in `files`. `commandModules` is filled from `scripts.compiledNames.get(sources.closure)`
   and stays empty, so the `compile-closure` loop never reaches `addDependency`. Open list: resource owners (inputs lib `api`), source discovery abort on
   cache replay (resolves instead of rejecting), declaration companions (`api.d.mts`, `definition.ts`), dependency input groups, Rust aggregate
   (`lib:build` vs `lib:build:release`), `targetScriptSources` called with a cache lacking `compiledNames` (test shape), playground publication
   (`Publication output escapes owner`), configured recipe owners (`dependency-release-data`), per-inventory memo (`b`,`c`,`x`).
   Same family is red in neighbours not on my list: source-demand 12, pinned-publication 7, native-scope 2 (live and with batch identical).
4. **optional-cache is hostage to the fleet**: 6 of 7 reds are `Original Script deadline refused`/`resource loan deadline exhausted` while waiting for
   `build-graph-store:.cache/b/g/m` and `repository-graph-optional-ledger:<repo>` (exclusive) behind a peer's `dependencies:generation-verify` /
   course:build. The tests use the real repository leases and the real graph image. Needs a quiet host or a private lease directory and disposable image
   (`withFixedRepositoryBuildGraphFixtureV1`). The 7th is a text-order law: `writer.indexOf(row.writerResource) < writer.lastIndexOf("await maintain(")`
   in `📥️inputs/💾️storage/🟨️.mjs`; the optional maintenance call (line ~165) now sits before the writer resource is built; decide whether maintenance
   must run under the writer lease (then fix the product) or the law is stale.
5. **execution**: `prepareTargetExecutionInventoryV1` (manifest, catalog, playground variants, components) exceeds `childTimeoutMilliseconds: 120000` at host load
   25-33 ("Original target execution preparation deadline exhausted"); the preparation schema caps the policy at 120000. Staged: `target-execution` birth
   (`preparation` field, schema, probe test, policy row). Open: a `build-execution` `fileLaunchers` row (route2-g) and a measured preparation time at a quiet host
   (or cache the inventory by producer revision).

## Order / merge notes for gate-g

- Batch is test + fixture + one product token; independent of nxlock/quietfc/conv/oracle2 batches by file (the shared fixture `🧪️tests/🧫️fixtures/🕸️graph/🔣️.json` and the observation module are the only files another batch could also touch; run `stage.py check` right before applying, `make-batch.py` regenerates the anchors against the live tree).
- Apply before route2-g's launcher rows if they reference `target-execution`.
- After apply: `sh obsrows-g-tools/verify.sh suites` (needs the Nx env by hand, printed in the script), then the official `@semio-tech/repo-lib:test-build-*` suites.

## Hand-offs

- graph/inventory lane: `Source input must be repository relative` in the Node-child project tests; admission dependency derivation (item 3).
- storage lane: item 4 text-order law; optional-cache lease isolation.
- route2-g: no birth for Nx-gated files; `target-execution` launcher for execution.
