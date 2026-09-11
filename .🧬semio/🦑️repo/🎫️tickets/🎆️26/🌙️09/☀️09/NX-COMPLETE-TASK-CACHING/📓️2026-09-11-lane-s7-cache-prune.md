# Lane S7 — Cache Prune/Report for the Shared Cache Root (2026-09-11)

Scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/**`. Builds on Wave A (`.cargo/config.toml`
`target-dir`/`build-dir` under the cache root, `nx.json` `cacheDirectory` moved, `cargoDirectories()`/
`repoCacheDirectory()` resolvers already in place).

## Design

### Schema-first budgets (`🔣️policy.json` `storage`, schema in `🧬️schema/🔣️.json`)

```json
"storage": {
  "guardAgeMs": 172800000,
  "cargo": { "budgetBytes": 85899345920, "unusedAgeMs": 604800000 },
  "vite": { "unusedAgeMs": 1209600000 },
  "agents": { "unusedAgeMs": 604800000 }
}
```

80 GiB / 7 days / 48 h guard for Cargo (one combined budget across `build-dir` + `target-dir`, matching the plan's
"cargo build+target budget 80 GiB"), 14-day age-only rule for Vite consumer caches, 7-day age-only rule for stray
agent scratch dirs. `guardAgeMs` is a single cross-area safety floor (never delete anything touched more recently
than 48h, even over budget) rather than duplicated per area. Nx's own `nx/` dir is intentionally **not** a pruned
area — `nx.json` `maxCacheSize` already governs it (task instruction: don't duplicate that governance).

### Pure eviction core (`⚡️caching/🧹️pruning/🟦️.ts`)

`planCachePrune(areas, nowMs, guardAgeMs)` is a pure function over an abstract `CacheUnit[]` tree — no filesystem
access, so it's identical in every language and directly fixture-testable:

1. **Age pass**: any unit not `lockHeld` whose `recencyMs <= nowMs - unusedAgeMs` is deleted.
2. **Budget pass** (only if the area declares a `budgetBytes`, e.g. Cargo): if still over budget, evict remaining
   units oldest-first, but skip (and report in `guardedOverBudgetBytes`) any unit that is `lockHeld` or younger
   than `nowMs - guardAgeMs` — "never delete anything newer than the guard, even over budget; report instead"
   from the plan's Key Evidence #6.

`CacheUnit.recencyMs` is `max(atime, mtime)` over **files only**, never directory metadata — a directory's own
mtime moves on unrelated sibling churn and would mask a genuinely stale unit.

### Disk adapter (same module)

- `scanCargoBuildUnits(buildDir, signal, onUnit?)` — walks the evidenced build-dir shapes: a unit is either
  `.../build/<package>/<hash>/` or `.../incremental/<crate>-<hash>/`, discovered generically (recurse until a
  directory literally named `build` or `incremental` is found, so an optional target-triple level in between is
  handled without guessing).
- `scanCargoTargetUnits(targetDir, signal, onUnit?)` — under the new build-dir layout `target-dir` holds only
  uplifted deliverable **files** (no per-unit subdirs), so each file is its own unit; `CACHEDIR.TAG` and
  `.cargo-*` sentinels are skipped.
- `scanDirectoryUnits(areaRoot, signal, exclude?, onUnit?)` — generic "immediate child directory = one unit"
  walker for Vite consumer caches and stray agent scratch dirs (`agents/resource-leases`, the lease store itself,
  is passed in `exclude` so the pruner can never delete its own locking infrastructure).
- `deleteUnit(areaRoot, unit)` — `rmSync` recursive for a build/incremental/directory unit; for a flat target
  file, `unlinkSync` then walks parents upward with `rmdirSync` until a non-empty directory is hit (Bun's
  `rmSync(path, { recursive: false })` on an empty dir throws `EFAULT` — confirmed by a failing probe during
  development — so empty-parent pruning uses `rmdirSync` instead).
- Cancellation: every walker calls `signal.throwIfAborted()` per directory/unit; both CLI commands wire
  `SIGINT`/`SIGTERM` to an `AbortController` and check the signal again immediately before each deletion, so a
  prune stops cleanly between deletions as required.
- Progress: every unit found and every unit deleted is streamed to stdout (`[cache-report] scanned …`,
  `[cache-prune] deleting …`).

### CLI commands (`⚡️caching/📜️script.ts`, router style)

- `cache-report [--json]` — dry run only, never deletes. Scans the three areas, runs `planCachePrune`, prints
  per-area size/unit-count/would-delete summary (or the full `PrunePlan` as JSON).
- `cache-prune [--dry-run]` — acquires an **exclusive lease** (`⚡️caching/🔒️leases`, resource `cache-prune`,
  directory `⚡️cache/agents/resource-leases` — the same store other tools already use) so concurrent prunes
  serialize instead of racing; then scans, plans, and deletes age- then budget-evicted units one at a time with
  progress, honoring cancellation between deletions. `--dry-run` runs the identical plan without calling
  `deleteUnit`, for safe verification.
- Both registered in the same `ScriptRouter` as the existing `disk-report`/`disk-prune`/`cache-verify` commands.

### Nx targets (`⚡️caching/📋️project.json`, project `repo`)

`cache-report` / `cache-prune`, `"cache": false`, calling only `bun ./📜️script.ts <name>` — matching the
`disk-report`/`disk-prune` pattern exactly. Confirmed both are uncached **by the plugin's own policy**, not just
the explicit `false`: `cacheInternals.mutatingName("cache-report")` and `("cache-prune")` are both `true` (the
regex matches `-report`/`-prune` as a suffix), so `targetPolicy()` forces `cache: false` regardless of what a
caller passes — verified directly against the plugin (`bun -e '... cacheInternals.targetPolicy(...)'`, see
Verification). `bunx nx show project repo --json` itself could not be run this session — an unrelated concurrent
peer's in-progress edit (`🖨️tectonic-template-compilation/📚️bundle/📜️script.ts` missing, `@repo/emoji-project-json`
plugin's `createDependencies` throwing) breaks Nx's project-graph construction repo-wide; this is outside `⚡️caching/**`
and not something this lane touched or should chase (matches the repo's own "ignore unrelated concurrent churn"
guidance). The direct plugin-level check above is the authoritative substitute — it's the exact function Nx calls.

### Automatic throttled bounding (`⚡️caching/🦀️cargo/📜️script.ts`)

`scheduleThrottledCachePrune(repoRoot)` runs after every successful `native cargo build|check|test` and
`native component dev|release` (not after `native cargo metadata`, which isn't a build). It reads/writes a stamp
file at the cache root (`⚡️cache/🧭️prune-stamp.json`, deliberately **outside** the `agents/`/`vite`/`cargo` areas
so it's never itself treated as a prunable unit), no-ops if fired within the last hour, and otherwise spawns
`bun 📜️script.ts cache-prune` **detached** (`spawn(..., { detached: true, stdio: "ignore" }).unref()`) so the
build's own result is never delayed. The whole function is wrapped in `try { … } catch {}` — a prune failure (or
even a throw while reading a corrupt stamp file) can never fail the build.

**Operational note for the coordinator**: this means the *next* native cargo build anyone runs after this lands
will, within an hour, autonomously run a real `cache-prune` against the live cache root. That is the intended
behavior (this is the "automatic bounding" the ticket asked for) but is called out explicitly since this session
deliberately never triggered a real prune against the live cache itself.

### Registry/fixture consistency (task item 5)

- `📦️artifacts/📇️registry/🟦️.ts`: `"nx"` store path `.nx/cache` → `${cache}/nx`; `"cargo-browser"` (`${cache}/cargo`)
  replaced with `"cargo-target"` (`${cache}/cargo/target`) and `"cargo-build"` (`${cache}/cargo/build`); added a
  `"vite"` store (`${cache}/vite`). The broad `"repo-cache"` entry already rejects any path under the cache root
  regardless of these specific sub-store names, so this is a correctness/clarity fix, not a behavior change to
  `createArtifactRegistry`'s CACHE-04 detection.
- `🧫️fixtures/artifact-registry/🔣️.json`: `invalid` list's `.🧬semio/🦑️repo/⚡️cache/cargo/browser` entry replaced
  with `.../cargo/target` and `.../cargo/build`; added `.../nx` alongside the existing `.nx/cache` entry (both
  still independently invalid — `.nx/cache` via the generic `.nx` path-segment guard, the cache-root ones via the
  `repo-cache` prefix guard).
- `📦️artifacts/🐳️containers/🧫️fixtures/🐳️devcontainer-context/🔣️.json`: the two illustrative `ignored` example
  paths updated from `.nx/cache/task/output` / `.../cargo/browser/target/...` to the current
  `.🧬semio/🦑️repo/⚡️cache/nx/task/output` / `.../cargo/target/debug/binary` (the test only checks these match the
  Docker `**` ignore pattern via `minimatch`, so this is example-data hygiene, not a behavior change).
- `🧫️fixtures/dependency-bootstrap/🔣️.json` (`tooling.storage.cache: ".nx/cache"`): **left unchanged**. Traced its
  only consumer (`🧪️tests/📦️dependencies/🟦️.ts` `testNxTooling`) — `storage.data`/`storage.cache` are arbitrary
  sandbox subdirectory *names* joined under a freshly `mkdtempSync`-ed `root`
  (`NX_CACHE_DIRECTORY: join(root, fixture.tooling.storage.cache)`), never resolved against the real repo's
  `nx.json`. This is exactly the documented exception ("Temp-workspace tests that set `NX_CACHE_DIRECTORY` for
  isolated sandboxes may stay").

## Files changed

- `⚡️caching/🔣️policy.json`, `⚡️caching/🧬️schema/🔣️.json` — `storage` budgets (schema-first).
- `⚡️caching/🧹️pruning/🟦️.ts` **(new)** — pure planner + disk adapter (`planCachePrune`, `scanCargoBuildUnits`,
  `scanCargoTargetUnits`, `scanDirectoryUnits`, `deleteUnit`, `formatBytes`).
- `⚡️caching/📜️script.ts` — `CacheReportScript`/`CachePruneScript`, `scanCacheAreas`/`areaUnitRoot` helpers,
  router registration.
- `⚡️caching/📋️project.json` — `cache-report`/`cache-prune` targets.
- `⚡️caching/🦀️cargo/📜️script.ts` — `scheduleThrottledCachePrune`, wired after build/check/test/component success.
- `⚡️caching/📦️artifacts/📇️registry/🟦️.ts` — store table updated to the new cache-root layout.
- `⚡️caching/🧫️fixtures/artifact-registry/🔣️.json`,
  `⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🐳️devcontainer-context/🔣️.json` — example paths updated.
- `⚡️caching/🧫️fixtures/cache-prune/🔣️.json` **(new)**, `⚡️caching/🧫️fixtures/cache-prune/🛂️schema/🔣️.json`
  **(new)** — language-agnostic eviction fixture (3 areas, boundary-equality case, lock-protection in both the
  age and budget phases, a guarded-over-budget case).
- `⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts` **(new)** — `testCachePrune`: fixture+schema validation, a Python
  oracle reimplementing the exact same age/budget/guard algorithm, real-disk scanning of a temp Cargo-shaped
  tree cross-checked against a Python `os.walk` byte oracle, cancellation, real deletion + empty-parent pruning.
- `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` — one import + one call wiring `testCachePrune` into
  `testCommandInputs`; did **not** touch lines ~602-603 (owned by lane S1 — confirmed via `git diff --cached`,
  those two lines were already changed by S1 before this lane started and were left untouched).

## Test results

Standalone run (`bun` importing `testCachePrune` directly):

```
[DEBUG] Cache eviction planner matches fixture and independent Python oracle for age, budget, guard and lock protection PASS
[DEBUG] Real Cargo build/target/vite/agents scanning, Python byte oracle, cancellation, deletion and empty-parent pruning PASS
ALL PASS
```

This exercises, against real files in a scratch temp dir (self-cleaned):
- The pure planner against a hand-authored fixture **and** an independent Python re-implementation of the same
  age/budget/guard algorithm (third-party-language oracle) — both agree exactly, including a unit at the exact
  cutoff boundary (`<=`), a locked unit that survives both the age pass and a budget pass that would otherwise
  evict it, and a guarded-over-budget case where eviction stops once budget is satisfied without ever touching
  the guarded unit.
- Real `scanCargoBuildUnits`/`scanCargoTargetUnits`/`scanDirectoryUnits` against an on-disk tree shaped exactly
  like the evidenced Cargo layout, with byte totals cross-checked against a Python `os.walk` + `os.lstat` oracle.
- `scanDirectoryUnits` correctly excludes `agents/resource-leases` (the lease store) from ever becoming a
  prunable unit.
- Cancellation: an already-aborted `AbortSignal` makes every scanner throw immediately.
- Real deletion: `deleteUnit` removes age-eligible build/incremental/target-file/directory units, leaves fresh
  and locked/guarded ones untouched, and prunes the now-empty `target/debug/deps` parent directory while leaving
  `target/debug` itself (still holds `examples/` and the sentinel) — caught and fixed a real bug in the process
  (see below).

Bug found and fixed during verification: Bun's `rmSync(path, { recursive: false })` on an **empty** directory
throws `EFAULT: bad address in system call argument` (reproduced in isolation with a 3-line repro) — switched
the empty-parent-pruning step in `deleteUnit` to `rmdirSync`, which works correctly.

Schema/policy self-check:
```
valid= true
storage= {"guardAgeMs":172800000,"cargo":{"budgetBytes":85899345920,"unusedAgeMs":604800000},"vite":{"unusedAgeMs":1209600000},"agents":{"unusedAgeMs":604800000}}
```

## Real `cache-report` against the live cache (dry run, no deletions)

```
[cache-report] cargo: 28.43 GiB across 1688 units; would delete 0 unused units (0 B) and 0 over-budget units (0 B); retains 28.43 GiB
[cache-report] vite: 924 B across 4 units; would delete 0 unused units (0 B) and 0 over-budget units (0 B); retains 924 B
[cache-report] agents: 9.20 GiB across 2 units; would delete 1 unused units (74.68 MiB) and 0 over-budget units (0 B); retains 9.13 GiB
```

`cache-report --json` produced a valid, small (597 B) `PrunePlan` — small because empty `ageDeletions`/
`budgetDeletions` arrays contribute nothing; a real stale cache would produce a proportionally larger payload.
Confirms: Cargo's combined build+target usage (28.43 GiB) is well under the 80 GiB budget so no LRU eviction
triggers yet; `agents/` already has one dir (`⚡️cache/agents/local` or `.../plugin-host-lifecycle-sol`, 74.68 MiB)
past the 7-day age rule; `vite/` already has real content from other lanes' work (`os-mcp`, `framework-kernel`,
`os-shell`, `framework-schema`), confirming the shared `vite/<consumer>` layout from the plan is already in use.

**No real `cache-prune` was run against the live cache** (per instructions — the coordinator does that after
review). A real prune (actual deletions, not `--dry-run`) was only run against the temp scratch tree inside
`testCachePrune`, as required.

## Left undone / handed to the coordinator

- A real `bunx nx show project repo --json` / `bun nx run repo:cache-report` end-to-end run through the Nx CLI
  itself could not complete this session — blocked by an unrelated concurrent peer edit breaking project-graph
  construction (`🖨️tectonic-template-compilation` module, `@asamuzakjp/css-color`). The plugin-level substitute
  check (`cacheInternals.targetPolicy`/`mutatingName` — the exact function Nx's graph construction calls) confirms
  the targets are correctly uncached; worth a quick `bunx nx show project repo --json` re-check once that
  unrelated breakage clears.
- The real `cache-prune` run against the live cache root is intentionally left for the coordinator, as instructed.
- `cargo/browser` (the old 54 GiB second target directory) is out of this lane's scope (not part of
  `cargoDirectories()`'s resolved `{target, build}`, and removing it is Wave B lane S1/coordinator territory per
  the plan) — it will sit untouched by `cache-prune` until something either deletes it directly or a future
  change folds it into a scanned area.
