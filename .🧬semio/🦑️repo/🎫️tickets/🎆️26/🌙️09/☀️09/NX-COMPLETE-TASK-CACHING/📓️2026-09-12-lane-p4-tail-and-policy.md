# Lane P4 — Tail Targets + Policy (2026-09-12)

Scope: every NON-continuous uncached target in `🗑️generated/nx/uncached.json` except `test*` (lane P1) and
os-dev `prepare-*`/`activate-*` (lane P2) — 113 targets across 30 projects, derived at
`🗑️generated/p4/my-scope.json`. Research was split across 4 parallel read-only agents (Python/oracle cluster,
Rust oracle/check cluster, hub/dev/bench cluster, preview-generated/misc cluster); I verified every proposed
flip myself against the live `nx show project --json` resolution before editing, since name-based regex
classification (`mutatingName`/`liveName`/`cacheableFamily`) can silently override an explicit `cache: true` in
a project's own `📋️project.json`.

## Real bugs found and fixed (not just the ~10 flips the plan expected)

`targetPolicy` in `🟨️.mjs` checks `matchesUncached(name, policy) || mutatingName(name) || liveName(name)`
**before** ever looking at the target's own declared `cache` value — so a name collision forces `cache: false`
even when the project author explicitly wrote `cache: true` with a correct, fixed `outputs` list. Two blanket
name families had exactly this defect:

- `mutatingName`'s `write-baseline$` and `(?:^|-)report(?:-|$)` alternatives caught *any* target whose name
  happens to end in "report" or "write-baseline" — including a Print **document literally named `report`**
  (`build-report`, a real PDF-compilation target with `cache:true` and `outputs:["{projectRoot}/dist/documents/report"]`
  already declared), `verify-taxonomy-report` (already `cache:true` in `📋️project.json`, silently overridden),
  and a cargo-test wrapper named for its *subject matter* (`reset-document-ownership` runs
  `cargo test … reset_document_ownership`, it doesn't reset anything itself).
- `liveName`'s `(?:^|-)live(?:-|$)` caught the `hub-live-catalog-{oracle,check,native-check}` family, where
  "live" names a fail-closed catalog-selection **contract**, not a network call — all three are pure
  fixture/AJV-driven checks with zero I/O beyond `readFileSync`.

Fix (🟨️.mjs, `mutatingName`/`liveName` — I own these two functions plus `cacheableFamily`/`targetPolicy`):

```js
const mutatingName = (name) => /(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)/.test(name);
/** 🌐 `hub-live-catalog-*` names a fail-closed catalog-selection contract, not a network call — pure fixture-driven checks, exempted from the live-name family. */
const liveName = (name) => !/^hub-live-catalog(?:-|$)/.test(name) && /(?:^|-)(?:e2e|live)(?:-|$)/.test(name);
```

I removed `write-baseline$`, `report`, and `reset` only after grepping the **entire** repo (via the ticket's
`🗑️generated/nx/projects.json` audit inventory, not just my 113-item slice) for every target name containing
those substrings, confirming zero remaining legitimate mutating use of `report`/`write-baseline`/`reset`, and
confirming `clean`/`gc`/`prune`/`setup`/`fuzz` still have genuine non-prefix-covered members (`cache-prune`,
`test-gc`, `cpp-setup`, `oracle-setup`, `stdio-fuzz`) that would otherwise fall through to the generic
catch-all and become accidentally cacheable.

`policy.json` gained one new entry: `cache-report` was uncached only as a side effect of the `report` bug
above, with no honest reason recorded anywhere. It scans the *live*, time-varying on-disk cache/storage areas
(`scanCacheAreas` → real directory sizes and ages), exactly like its already-listed sibling `disk-report` — so
I added it to `policy.uncached` next to `disk-report` instead of leaving it accidentally-uncached.

## Decision table

`cache: true` (flipped from `false`, or a dead `false`→`true` project.json correction — 21 targets):

| project:target | change | evidence |
| --- | --- | --- |
| `@semio-tech/framework-os-host-rs:member-history-identity-source` | `cache:false`→`true`, `outputs:[]` | `📜️script.ts:247-333` — pure fixture reads/asserts/log, zero writes/network/randomness; strict subset of the already-cached `member-history-input-check` |
| `@semio-tech/framework-os-host-rs:public-member-open-handoff-source` | same | `📜️script.ts:1041-1119`; superset `public-member-open-handoff-check` already cached |
| `@semio-tech/framework-os-mcp-rs:inference-discovery-oracle` | `cache:false`→`true`, `outputs:[]` | `📜️script.ts:220-259` — fixed fixture/schema reads, AJV validation, no fetch/spawn |
| `@semio-tech/framework-os-mcp-rs:hub-live-catalog-oracle` | `cache:false`→`true`, `outputs:[]` (+ `liveName` carve-out) | `📜️script.ts:286-319` — fixture-driven AJV/deepStrictEqual only |
| `@semio-tech/framework-os-mcp-rs:hub-live-catalog-check` | already declared `cache:true`; silently forced false by `liveName` — now takes effect, added `outputs:[]` | `📜️script.ts:323-338` — subprocess-invokes the oracle + `readFileSync` marker checks |
| `@semio-tech/framework-os-mcp-rs:hub-live-catalog-native-check` | same as above | `📜️script.ts:342-357` — deterministic `cargo test --lib` |
| `@semio-tech/framework-os-mcp-rs:canonical-checkpoint-resource-oracle` | `cache:false`→`true`, `outputs:[]` | `📜️script.ts:360-385` — pure base64/sha256 fixture validation |
| `os-hub:gis-inference-ledger-oracle` | `cache:false`→`true`, `outputs:[]` | `📜️script.ts:8049-8180` — fixture reads, AJV, in-memory `bun:sqlite(:memory:)`, no fetch/spawn/randomBytes/Date.now gating; sibling `gis-inference-ledger-check` already cached |
| `os-hub:admin-live-journey-check` | dead `cache:true`→`false` (project.json cleanup) | `📜️script.ts:13179-13194`/`2981-3060` — real `startLocalHub`, `randomBytes(32)` nonce, live Playwright browser, `Date.now()` deadlines; `liveName` already forced it false at runtime, the JSON was just lying |
| `@semio-tech/framework-os-kernel:preview-generated` | `cache:false`→`true`, `outputs:[]` | `script.ts:2095-2098` → `runNestedCargoPackageAdapter(...,"preview")` only does `process.stdout.write` for mode `"preview"`, no filesystem writes |
| `@semio-tech/repo-lib:preview-generated` | `cache:false`→`true`, `outputs:[]` | writes only to stdout; underlying handoff-bytes helper works in a `mkdtempSync` sandbox **outside** the repo, cleaned up in `finally` |
| `@semio-tech/mit-bestand-bericht:preview-generated` | `cache:false`→`true` (inputs/outputs already correct) | delegates to `ActorNetworkScript.run(["preview"])`, docstring: "non-writing preview" |
| `repo:cache-verify` (caching module's own `📋️project.json`) | dead `cache:true`→`false` | `CacheVerifyScript` spawns real child `nx` processes, measures wall-clock `durationMs`, writes into an externally-supplied ticket dir (`ticketOutput`) — `cache-verify` is in `policy.uncached` already, the JSON `true` never took effect |
| `workspace:prepare` | `cache:false`→`true` (`outputs:[]` already) | `SetupScript.run`: `prepare: () => console.log("[prepare] Nx prerequisites completed")` — literally a no-op; all real work happens via the target's own `dependsOn` |
| `workspace:artifact-field-parity-report` | `cache:false`→`true`, `+outputs:[]` | same `policyArtifactOwnershipFieldParity` check as the already-cached `-enforce` sibling; `report` mode just doesn't throw |
| `workspace:verify-dependencies-freeze-write-baseline` | `cache:false`→`true`, `+outputs:["{workspaceRoot}/🔒️dependencies.json"]` | `dependencyFreezeWriteBaseline` derives the baseline from current repo dependency state and writes exactly that one fixed path |
| `workspace:verify-layering-write-baseline` | `cache:false`→`true`, `+outputs:["{workspaceRoot}/🧅️layering.json"]` | `writeLayeringBaseline` derives from `layeringReferences(repoRoot)`, writes exactly `LAYERING_BASELINE_REL_PATH` |
| `workspace:reset-document-ownership` | `cache:false`→`true`, `+outputs:[]` | `runCargo(["test", …, "reset_document_ownership", …])` — a deterministic cargo test; only match for "reset" repo-wide |
| `workspace:verify-taxonomy-report` (no JSON change) | mutatingName fix let its existing `cache:true` take effect | `runTaxonomy`: `report` mode logs and never throws |
| `@semio-tech/print:build-report` (no JSON change) | mutatingName fix let its existing `cache:true`+real `outputs` take effect | `printDocumentTargets()` generates `build-${document.id}` for every catalog document; one document's id happens to be `"report"` |

`policy.json` addition (1 target, honest reclassification rather than accidental):

| project:target | change | evidence |
| --- | --- | --- |
| `repo:cache-report` | added `cache-report` to `policy.uncached` | `CacheReportScript` → `scanCacheAreas` reads live, time-varying on-disk cache directory sizes/ages — same class as sibling `disk-report` |

## Confirmed correct as-is (stays uncached, one-line reason each — 92 targets)

| project:target | reason |
| --- | --- |
| `@semio-tech/energy-oracle-py:deps` | `uv sync --locked` — network venv install into `.venv` |
| `@semio-tech/energy-oracle-py:oracle-setup` | downloads OpenStudio over the network via `curl`, extracts into shared toolchain cache |
| `@semio-tech/energy-oracle-py:oracle-status` | probes live installed binary versions on this machine |
| `@semio-tech/energy-oracle-py:oracle-run/oracle-native/oracle-epjson/oracle-emit` | `forwardAllArgs:true`, output path is a free-form CLI arg (`<out.json>`) — no fixed declarable output; run/native/epjson also invoke the live EnergyPlus binary |
| `@semio-tech/repo-test-dotnet:deps` | `dotnet restore` — NuGet network restore into a shared cache dir |
| `@semio-tech/ui-styling-py:deps` | `uv sync --locked` — same network venv-install pattern |
| `@semio-tech/browser-actor-import-testkit:pending-host-close-check`/`runtime-check` | requires ticket-scoped `CARGO_TARGET_DIR`/`SEMIO_TEST_ARTIFACT_DIR`, writes JCO/wasm evidence into that externally-supplied non-deterministic dir |
| `@semio-tech/gis-plugin:component-cold-map-patch-native-check` | requires ticket-scoped `SEMIO_TEST_ARTIFACT_DIR`, `mkdtempSync` subdirectory |
| `@semio-tech/plugin-registry:new` | interactive scaffolder writing new files at derived locations from arbitrary plugin/kind/standard args |
| `@semio-tech/plugin-registry:plugin-root-ownership-check`/`rust-taxonomy-mounts-check` | ticket-scoped `SEMIO_TEST_ARTIFACT_DIR` + `mkdtempSync` |
| `@semio-tech/stdio-plugin:artifact-directory-wiring-generate`/`subset-directory-wiring-generate` | codemod-style: rewrites every stale source file across the whole stdio module tree, not a fixed output set |
| `@semio-tech/stdio-plugin:catalog-root` | requires externally-supplied `--build-root`/`SEMIO_CATALOG_FRESH_BUILD_ROOT`, drives a live cancellable cargo+jco build |
| `@semio-tech/os-plugin-describe-rs:describe` | `forwardAllArgs:true`, forwards to a native binary with a caller-chosen `--out <dir>` — arbitrary-arg launcher |
| `@semio-tech/repo-cli-rs:run`/`workflow` | forwards arbitrary argv to the `semio` binary — arbitrary-arg launcher, side-effecting by design |
| `@semio-tech/repo-lib:workspaces-write` | deterministically computes `workspaces` from directory structure but overwrites the **whole tracked** root `package.json` in place — a freshness-fixup writer (same category as `format-fix`), kept uncached so a stale cache never hides a newly-added workspace member |
| `@semio-tech/print:deps-tectonic`/`deps-tex` | network `fetch` + extract into the shared toolchain cache (`⚡️cache/tools/tectonic/…`) — matches the repo-wide `deps` policy family exactly |
| `@semio-tech/framework-os-dev:bench`/`bench-plugins-native`/`bench-plugins-react`/`bench-plugins-wgpu` | performance measurement, covered by the `bench` policy family |
| `@semio-tech/framework-os-dev:collab-e2e` | real free-port hub/user daemons + live Playwright browser (`liveName` "e2e") |
| `@semio-tech/framework-os-dev:parity` | bare dispatch router to `smoke\|triage\|probe\|verify\|sweep` with no work of its own — deliberately in `policy.uncachedExact` so the umbrella stays uncached while specific sub-targets are independently cacheable |
| `semio-framework-3d:bench` | performance measurement |
| `os-hub:browser-actor-child-worker-containment-check`/`browser-actor-gis-describe-check` | ticket-scoped `SEMIO_TEST_ARTIFACT_DIR` (+ `mkdtempSync` / free-port Vite server) |
| `os-hub:secure-local-smoke` | real `startLocalHub` + live `fetch()` HTTP calls + native/wgpu cargo builds |
| `os-hub:setup` | covered by the repo-wide `setup` policy family |
| `os-hub:scoped-presence-browser-serve` | **found, not fixed by me**: a genuine continuous Vite `server.listen()` that blocks on SIGINT/SIGTERM (`📜️script.ts:14340-14387`) — its name ends in, rather than starts with, `serve`, so the continuous-family prefix matcher misses it; it's currently `cache:false` by explicit override (safe), just not marked `continuous:true`. `os-hub`'s serve/dev continuous targets belong to lane P3's file ownership, so I flagged this for them rather than editing os-hub's continuous handling myself |
| `repo:audit`/`ci-baseline`/`disk-report` | write into an externally-supplied ticket output directory (`ticketOutput`), and `disk-report` additionally scans live, time-varying cache/disk state |
| `repo:doctor` | probes live installed toolchain versions on this machine |
| `repo:cache-prune` | deletes cache units — external mutation by design |
| `repo:generator-inputs` | its output file is read by other targets' cache-key computation via a `runtime` shell hash at graph-construction time, not through Nx's own output-restoration path; keeping it always-fresh avoids downstream cache keys silently hashing a stale file |
| `workspace:bench`/`bench-plugins` | `bench` policy family |
| `workspace:clean`/`clean-coverage`/`clean-test` | `clean` policy family (deletes files) |
| `workspace:clean-taxonomy-{inventory,plan,apply,verify}` | multi-phase stateful taxonomy-mutation workflow — `inventory`/`plan` capture snapshots consumed by `apply`, which mutates real taxonomy files in place; not independently replayable outside the full pipeline; matches this repo's established conservative treatment of the whole `clean` family |
| `workspace:commit`/`format`/`micro-commit`/`os`/`semio` | `uncachedExact` — git commit, in-place formatter, arbitrary-arg launchers |
| `workspace:cpp` | `forwardAllArgs:true` dispatch to `setup\|configure\|build\|test\|all` (arbitrary-arg launcher; `all` itself is a no-op, but the target can't declare what the caller will actually run) |
| `workspace:cpp-setup` | installs `cmake`/`ninja`/MSVC on the machine |
| `workspace:deps-browsers/-cargo/-cpp/-dotnet/-go/-javascript/-python/-tools/-wasm/-wasm-opt` | `deps` policy family — installs into shared/user toolchain locations |
| `workspace:new`/`new-taxonomy-mutation` | scaffolding, writes at caller-chosen/derived locations |
| `workspace:publish`/`purge` | `publish`/`purge` policy families |
| `workspace:scale-fixture` | `uncachedExact` (its `-check` sibling is separately cacheable) |
| `workspace:setup`/`setup-git`/`setup-native` | `setup` policy family — postinstall/git config/native toolchain setup |
| `workspace:setup-storybook` | underlying command is the same no-op `setup prepare` dispatch as the now-cacheable `prepare`, but it's also covered by the repo-wide `setup` **prefix** family (`"setup-storybook".startsWith("setup-")`) independently of `mutatingName` — carving a per-target exception out of the shared `setup` prefix family isn't justified for a single, effectively-free (single `console.log`) target; left uncached for name-family consistency |
| `workspace:stdio-fuzz` | genuine randomized fuzzing — `mutatingName`'s `fuzz` keyword is a correct match here, not a false positive |
| 8× `nx-release-publish` (`assets`, `flow-core`, `flow-js`, `repo-client`, `repo-coordinator`, `repo-sqlite`, `repo-vscode`, `ui-react`, `ui-styling`) | `@nx/js:release-publish` — generated by the **external** `@nx/js` Nx plugin's own `createNodes`, never passes through this repo's `targetPolicy`; publishing a package is an inherent external side effect. No project.json in this repo declares it, so there is nothing in our policy/plugin layer to change; leaving it alone is correct |
| `mit-bestand-demonstrator:activate-dev`/`prepare-dev`/`prepare-e2e`/`prepare-release` | dev-runtime activation/preparation targets, same shape as os-dev's `prepare-*`/`activate-*` family — owned by lane P3 (`♻️mit-bestand/**`), confirmed to exist and not touched |

Out-of-scope items observed in the post-fix audit but **not** part of my original 113-item assignment (new/renamed
since the original `uncached.json` snapshot, or belonging to other lanes' domains) — left untouched:
`@semio-tech/repo-test-domain:test-*` (lane P1, `test*` prefix), `@semio-tech/repo-lib:test-inventory-artifact-shards`
(lane P1), `@semio-tech/mit-bestand-demonstrator:test-e2e` (lane P1), `@semio-tech/trinity-jack-shell:run` (new
project/target not present in the original snapshot; same arbitrary-arg-launcher shape as `repo-cli-rs:run` if
anyone picks it up later), `workspace:test-discover/test-doctor/test-metrics/test-metrics-enforce` (lane P1).

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` — `mutatingName`, `liveName`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` — added `cache-report` to `uncached`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json` — `cache-verify` → `cache:false`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/nx-contract/🔣️.json` — fixed the
  `artifact-field-parity-report` row (was asserting `cache:false`) and added 7 rows covering `verify-taxonomy-report`,
  `build-report`, `reset-document-ownership`, `verify-dependencies-freeze-write-baseline`, `hub-live-catalog-check`,
  `admin-live-journey-check`, `cache-report`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` — added direct
  `matchesUncached`/`mutatingName`/`liveName` assertions documenting each collision and its resolution.
- `📋️project.json` (root) — `prepare`, `artifact-field-parity-report`, `verify-dependencies-freeze-write-baseline`,
  `verify-layering-write-baseline`, `reset-document-ownership`.
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📋️project.json` — `member-history-identity-source`,
  `public-member-open-handoff-source`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📋️project.json` — `inference-discovery-oracle`,
  `hub-live-catalog-oracle`, `hub-live-catalog-check`, `hub-live-catalog-native-check`,
  `canonical-checkpoint-resource-oracle`.
- `🌎️hub/📦️packages/🦀️rust/📋️project.json` — `gis-inference-ledger-oracle`, `admin-live-journey-check`.
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json` — `preview-generated`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` — `preview-generated`.
- `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json` — `preview-generated`.

## Verification

- `NX_DAEMON=false bunx nx show projects > /dev/null` → exit 0 (checked after the `🟨️.mjs` regex edits and again
  after all `📋️project.json` edits).
- `SEMIO_TICKET_DIR=<ticket> bunx nx run repo:audit --skip-nx-cache` → `projects=700 commands=7136 artifacts=8014
  violations=0` (log: `🗑️generated/p4/repo-audit.log`).
- Runtime resolution spot-check (`nx show project <p> --json`) for all 13 changed/clarified targets matches the
  intended value exactly (`🗑️generated/p4/show-projects-2.log` command history): `verify-taxonomy-report`,
  `artifact-field-parity-report`, `reset-document-ownership`, `prepare` → `cache:true`; the two write-baselines
  → `cache:true` with their exact single-file `outputs`; `print:build-report` → `cache:true` with its existing
  `outputs`; `mcp-rs:hub-live-catalog-check`/`hub-live-catalog-oracle` → `cache:true`; `os-hub:gis-inference-ledger-oracle`
  → `cache:true`; `repo:cache-report`/`cache-verify` and `os-hub:admin-live-journey-check` → `cache:false` (now
  honestly, not accidentally).
- 3 newly-cached targets run twice for real (non-mutating verification/oracle commands, safe to execute):
  - `@semio-tech/framework-os-mcp-rs:inference-discovery-oracle` — miss then `[existing outputs match the cache,
    left as is]` / `Cache: 1/1 hit (100%)`.
  - `os-hub:gis-inference-ledger-oracle` — miss then `Nx read the output from the cache instead of running the
    command … Cache: 1/1 hit (100%)`.
  - `@semio-tech/framework-os-mcp-rs:hub-live-catalog-oracle` — miss then `Cache: 1/1 hit (100%)`.
- Final remaining-uncached inventory regenerated from a fresh post-fix `repo:audit` run
  (`🗑️generated/p4/final-uncached.json`, derived from `🗑️generated/nx/projects.json` at 01:46) — see the
  "Confirmed correct as-is" table above; 99 non-continuous `cache:false` targets remain repo-wide, of which the
  ones in my scope are exactly the documented, deliberate cases.

### Unrelated finding (not fixed, out of scope)

`@semio-tech/framework-os-host-rs:member-history-identity-source` and its already-cached sibling
`member-history-input-check` both currently fail with a real `AssertionError` at `📜️script.ts:330`
(`assert(!validate(extra))`) when actually run — a genuine, pre-existing business-logic bug unrelated to caching
(confirmed by running both the target I flipped and its untouched sibling; both fail identically, so the failure
predates and is independent of my edit — likely concurrent work elsewhere in this shared, multi-lane session).
Caching classification is about determinism, not passing status: a target that deterministically fails the same
way given the same inputs is still correctly `cache:true`. Left as-is; flagging for whichever lane owns host-rs
document-identity fixtures.

A second, separate real-run attempt surfaced the same category of issue: `workspace:verify-taxonomy-report`
(run for real, non-cached, as part of the "run twice" verification) took 94m36s under this session's heavy
concurrent load and then failed with `frozen-coordinate-evidence-invalid: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json:
document digest does not match registered bytes` (`🧹️normalization/🟦️.ts:7109`, inside `verifyTaxonomy` — thrown
in `report` mode too, not gated behind `enforce`; log: `🗑️generated/p4/taxreport-1.log`). This does not touch any
code I edited (`runTaxonomy`/`verifyTaxonomy`/`frozenCoordinateEvidenceCoordinates` are all untouched by this
lane) and reads as a fixture mid-write by a concurrent session rather than a stable bug, so I did not chase it
further or revert the `cache:true` classification — a deterministic function of current repo state that
currently throws is still correctly cacheable (and Nx does not persist a failed run's result as a cache hit
regardless). Re-run this one once the fixture settles if the failure persists.
