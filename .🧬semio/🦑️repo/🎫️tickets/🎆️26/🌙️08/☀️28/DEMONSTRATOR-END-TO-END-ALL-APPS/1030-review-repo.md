# 1030 Review: repo tooling diff (waves 1029-1030)

Read-only review. Scope: uncommitted diff under `🧰️framework/🛍️products/🦑️repo/🔨️modules` plus the surrounding committed code (headroom, launcher, elastic/pinned capacity, source-owner resolution). Pressure GC, lock/frozen check and cache-recovery targets were already committed and are not in the working diff; only their callers were read. Nothing was built or run. Confidence is stated per finding.

## Findings, most severe first

### 1. Cargo.lock does not contain the new build-graph crate or the two new dependency edges (owner: deps) - confidence high
- `Cargo.toml:561` lists `.../📚️library/🏗️build/🕸️graph/🦀️rust/📦️packages/🦀️rust` as a member, but `Cargo.lock` has no `semio-framework-repo-library-build-graph` package.
- The `semio-framework-repo-dashboard-daemon` lock entry (`Cargo.lock:21173`) lacks the new `dashboard_build_graph` dependency, and the registry entry lacks it too.
- The crate directory is untracked (`??`).
- Failure: any `--locked` or frozen resolution fails. Otherwise the first cargo to run rewrites `Cargo.lock` while peers write it concurrently.
- Fix: `repo dependencies lock` run by the coordinator, then verify `Cargo.lock` contains the package.

### 2. Source-owner resolution semantics changed but the fingerprint profile and version stayed (owner: deps/policy) - confidence medium
- `💾️storage/🟦️.ts:362-374`: `sourceProvider` no longer walks the ancestors of a source path. It only tries `<stage>/w/<slot>`, then the stage root. The owner-closure check and the "Unowned Source module fallback retained" refusal are gone.
- `🧬️schema/📥️install.json:4901-4904` now says `hoisted-scope-root-owner-bindings`, `unownedFallback: not-consulted` and `ambientFallback: not-consulted`.
- Nothing in code or tests reads `sourceResolution`, `unownedFallback` or `ambientFallback` (grep finds only the schema), so the schema change is not enforced. That is schema-first drift.
- `SourceOwnerExecutionProtocol.profile` is still `declared-provider-identity` and `version` is still 1. Both feed the ready fingerprint at `💾️storage/🟦️.ts:361`.
- Failure: a ready receipt written under the old "refuse ambient node_modules" contract stays valid when the provider addresses coincide. The weaker guarantee is silently accepted across generations.
- Runtime risk: Node and Bun still walk real ancestors. A stray `node_modules` above the checkout, or between a package and the stage, can shadow the declared provider without the guard noticing.
- Fix: bump `profile` or `version`, and add a test that reads and enforces the three fields.

### 3. Headroom learns only lane growth, but the reservation covers all image bytes (owner: actinfra) - confidence medium-low
- `🚪️launcher/🟦️.ts:101-103`: `grown = measureLaneBytes(after) - measureLaneBytes(before)`. It covers only the lane's target and build directories.
- The shared unit cache objects, the incremental sessions pruned by GC, and component outputs written outside the lane are not counted.
- The lane is measured after admission, and the unit cache is written during the build.
- Failure: after 3 samples (`minimumSamples`) the quantile estimate replaces the 8 GiB component prior and the 2 GiB cold prior. Reservations can fall below the real image writes. Several cold builds are then admitted at once and the image fills.
- Also, `grown` can go negative when GC trims the lane during the build. The sample is clamped to 0, which biases the estimate further low.
- Fix: measure the cache root (or the image free-space delta, excluding GC) around the command, or record the lane and unit-cache bytes under separate samples.

### 4. effectiveCapacityV1 and pinnedCapacityV1 disagree on receipts (owner: actinfra) - confidence medium
- `📐️elastic/🟦️.ts:18-23` (elastic): an existing receipt is accepted when `>= minimumBytes` with no upper bound.
- `📐️elastic/🟦️.ts:25-30` (pinned): accepted only when `33554432 <= bytes <= declared`.
- Elastic: a receipt smaller than `minimumBytes` returns `minimumBytes`, which is larger than the physical image. Budgets and headroom then compute with a capacity the image does not have.
- Pinned: a smaller image is honoured down to 32 MiB.
- Elastic: no receipt means the size is computed from the host's free space at that moment. Two processes reading the policy at different times get different capacities until the image is born.
- Fix: share one receipt parser. Cap the elastic receipt at `rule.maximumBytes`. Clamp downward instead of snapping up.

### 5. Admission "alone" rule can stall behind a live but stuck lease (owner: actinfra) - confidence low
- `🏦️headroom/🟦️.ts` `decideHeadroomV1`: the `alone` path needs `running.length===0 && ahead.length===0 && reclaimed`.
- Heartbeats are `unref`'d intervals and keep a record fresh while the process is alive. A hung body that never calls `release` keeps a "running" row forever, although its held bytes decay to 0 after the ramp (up to 15 min for components).
- Failure: if free space is below `reserve + floor`, every later command waits with reason `space` and reclaims every interval. Only a real shortfall resolves this, so it is bounded by the image size, not a hard deadlock.
- A cancelled wait does remove its record. The monitor's reclaim guard `inFlight` is fine.
- Fix: let a row older than its ramp plus a grace period stop counting in `running.length` for the `alone` test.

### 6. Warm/cold temperature is decided before admission, the lane after (owner: actinfra) - confidence low
- `🚪️launcher/🟦️.ts:98` picks `warm` by planning the pool, then waits in the headroom queue, then `acquirePooledLaneV1` picks a lane.
- During a long wait another process can take the warm lane. A cold build is then admitted under the warm prior (0.75 GiB or 3 GiB for components) and the sample is recorded under `#warm`, polluting that class.
- Fix: re-plan after admission and re-record under the actual temperature.

### 7. Two workspace walks, two skip rules (owner: policy) - confidence medium-high, low impact
- `🗂️workspaces/🟦️.ts:57-58` uses only the `OutputDirectories` set.
- `🗂️workspaces/🟦️bun/🟨️.cjs:13-14` also applies the `/^target(?:-[a-z0-9]+)*$/` regex.
- The TS generator therefore descends into `target-foo` directories that the Bun ownership walk ignores, and can generate workspace entries the ownership check will refuse.
- `pkg` and `🕸️bindings` are new global skips by name. An authored package directory called `pkg` anywhere is silently dropped.
- Fix: move the regex into the schema and share it. Scope `pkg` and `🕸️bindings` to a path rule.

### 8. recycleReasonV1 relies on an unchecked baseline (owner: policy) - confidence low
- `🏊️pool/🟨️.mjs:75-76` and `:187`. If the first reply carries no `rssMegabytes`, `baseline` is `undefined`. The growth term `rss > undefined` is then false, so only the hard `rss > megabytes` cap can recycle the worker.
- Fix: validate the number with `Number.isFinite`, or fall back to the old ratio rule.

## Checked and found sound
- `physicalMemo` and `candidateMemo` live per call to `readJavascriptDependencySourceOwnerCandidatesV1`. Every memoized lookup, including a missing path, is recorded in `observations` and re-verified by the double `replay()`, so there is no stale result across generations.
- `artifactPhase` (publication): `accept` is called once, outside the per-scope loop. The `InputGroup` read and funding functions exist in the native platform module.
- `headroomPriorV1` and the Python oracle use the same prefix rule. The 15% share ceiling bounds the component prior (7.2 GiB on a 48 GiB image), so a component command can always be admitted once it runs alone after a reclaim.
- The build-graph crate's `include_str!` paths resolve from the same source file as before.
- `.🧬semio` is still skipped by the `startsWith(".")` check at `🗂️workspaces/🟦️.ts:112`.
- The removed taxonomy member `📬️publication-authority` has no directory on disk.
