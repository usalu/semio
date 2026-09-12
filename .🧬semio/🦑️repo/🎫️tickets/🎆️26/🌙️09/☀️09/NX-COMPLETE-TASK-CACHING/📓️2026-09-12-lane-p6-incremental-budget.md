# Lane P6 — Incremental Cargo Budget (2026-09-12)

Scope: `⚡️caching/🧹️pruning/**`, `cache-report`/`cache-prune`/`scanCacheAreas` in `⚡️caching/📜️script.ts`, `policy.json`
`storage` + schema, cache-prune fixtures/tests. Builds on lane S7 (2026-09-11), which gave the shared build-dir one
combined Cargo budget across compiled units and incremental state.

## Problem

The shared build-dir was 113G (later observed 130G, live/concurrent activity) against an 80 GiB budget, and 94G of
it was rustc incremental state. Today's pruner treated incremental crate dirs and compiled units under one combined
budget with a 48h guard, so on a busy day (lots of recent touches) it could never evict enough to get under budget —
incremental state needs its own, much tighter policy since it is a pure rebuild accelerator (safe to delete; costs
only a slower next incremental build), unlike compiled units.

## Observed on-disk shapes (verified before coding)

```
.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/incremental/<crate>-<hash>/
  s-<ts>-<id>-<svh>        finalized session directory
  s-<ts>-<id>.lock         0-byte lock file, same <ts>-<id> prefix, kept even after finalizing
  s-<ts>-<id>-working      in-progress session (present for 3 real crate dirs at scan time, all several
                           hours stale — confirmed no rustc/cargo process was running, i.e. genuinely
                           orphaned by a crashed/interrupted build, not just "look old")
```

- A crate dir can hold **more than one finalized session** simultaneously (found real examples, e.g.
  `compiler-2l2v37kytmhot` with two `s-*` dirs + two matching `.lock` files) — rustc does not always clean up
  the previous one itself.
- Lock file name is always the session dir's first three `-`-separated segments (`s`, timestamp, id) plus
  `.lock` — verified against multiple real session/lock pairs, including `-working` ones.
- `debug/incremental` alone was 57G, `wasm32-wasip2/wasm-dev/incremental` 20G, `wasm32-unknown-unknown/debug/incremental`
  12G, `wasm32-wasip2/debug/incremental` 4.5G.

## Design (schema-first)

`⚡️caching/🔣️policy.json` `storage.cargo` gains a nested `incremental` policy, schema in `⚡️caching/🧬️schema/🔣️.json`:

```json
"cargo": {
  "budgetBytes": 85899345920,
  "unusedAgeMs": 604800000,
  "incremental": { "budgetBytes": 25769803776, "unusedAgeMs": 86400000, "guardAgeMs": 3600000 }
}
```

24 GiB budget / 24h unused-age / **1h guard** for incremental (vs. the existing 80 GiB / 7 day / 48h guard for
compiled units) — much tighter because incremental state is disposable.

### Pure planner extension (`⚡️caching/🧹️pruning/🟦️.ts`)

- `CacheAreaInput` gained an optional `guardAgeMs`; `planCachePrune`'s budget pass now uses
  `area.guardAgeMs ?? guardAgeMs` (falls back to the shared cross-area guard when an area doesn't set its own —
  fully backward compatible with the existing `vite`/`agents`/`cargo` areas).
- New `scanCargoIncrementalUnits(buildDir, signal, onUnit?)` returns `{ units, staleSessions }`:
  - `units` — one `cargo-incremental` unit **per crate dir**, sized to only its **newest finalized session**
    plus any in-progress `-working` session. `lockHeld: true` whenever a `-working` session exists, **regardless
    of its age** — a crate dir must never be deleted while one is present, since a crashed build can leave it
    looking stale for hours without meaning it is safe to remove (this is why detecting `-working` explicitly,
    not just relying on recency, matters — three real examples on this machine were hours old with no process
    holding them).
  - `staleSessions` — every finalized session that is **not** the newest, as independent `cargo-incremental-session`
    units. rustc itself only needs the newest one, so this is unconditionally safe to reclaim regardless of the
    crate dir's own age or lock state.
- `scanCargoBuildUnits` no longer descends into `incremental/` (split out, matching the new area).
- `deleteUnit` gained a `cargo-incremental-session` case: removes the stale session directory **and** its sibling
  `.lock` file (derived by the same first-three-segments rule).
- Refactored the "find `build`/`incremental` dirs at any depth" walk into a shared `findNamedDirs` helper used by
  both `scanCargoBuildUnits` and `scanCargoIncrementalUnits`.

### Wiring (`⚡️caching/📜️script.ts`)

`scanCacheAreas` now returns five areas instead of three:

- `cargo` — compiled `build/<pkg>/<hash>` units + uplifted target files, unchanged 80 GiB/7day/48h policy.
- `cargo-incremental` — post-compaction crate-dir units, 24 GiB/24h/**1h guard** (its own `guardAgeMs`).
- `cargo-incremental-sessions` — stale finalized sessions, `budgetBytes: null`, `unusedAgeMs: 0` (always
  age-eligible — an invariant compaction, not a tunable retention policy, so it isn't itself a policy knob).
- `vite`, `agents` — unchanged.

`cache-report`/`cache-prune` iterate `plan.areas` generically, so the new areas get their own report/delete lines
("separate incremental line") with no changes needed to those two classes beyond the import and `scanCacheAreas`/
`areaUnitRoot` (which now maps all three `cargo*` area names back to `cargoDirectories().build`).

## Fixtures and tests

- `⚡️caching/🧫️fixtures/cache-prune/🔣️.json` (+ schema): added `cargo-incremental` (demonstrates `lockHeld`
  crate-dir protection surviving both age and budget passes, and a per-area `guardAgeMs` override of 500ms that is
  *tighter* than the fixture's global 2000ms guard, showing the override actually changes the outcome — a unit that
  would be guarded under the global value is evicted under the area's own shorter one) and
  `cargo-incremental-sessions` (unconditional reclaim, `unusedAgeMs: 0`) areas; schema allows an optional per-area
  `guardAgeMs` and the new `cargo-incremental-session` unit kind.
- `⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts`:
  - Python plan oracle updated: `guardCutoff = now - area.get('guardAgeMs', guard)`.
  - Disk-scan test rebuilt with a realistic incremental tree: a single-session crate, a fresh single-session crate,
    a two-finalized-session crate (`crate-multi`, proves compaction keeps only the newest and reports the older one
    as a `cargo-incremental-session`), and a crate with one finalized session *and* a stale-looking `-working`
    session (proves `lockHeld` survives the age pass even though every file in it is 10 days old).
  - New invariant check against an independent Python `os.walk` byte oracle: raw bytes under `incremental/` must
    exactly equal `sum(units.bytes) + sum(staleSessions.bytes)` — compaction repartitions bytes, never gains or
    loses them.
  - Real deletion: age-evicts the old crate dir entirely, evicts only the stale session (+ its `.lock`) out of
    `crate-multi` while the crate dir and its newest session survive, and confirms the `-working` crate dir and its
    working session directory are untouched.

### Test run (standalone, `bun` importing `testCachePrune` directly)

```
[DEBUG] Cache eviction planner matches fixture and independent Python oracle for age, budget, guard (including per-area override) and lock protection PASS
[DEBUG] Real Cargo build/target/incremental/vite/agents scanning, Python byte oracle, cancellation, session compaction, deletion and empty-parent pruning PASS
ALL PASS
```

Schema/policy self-check (Ajv against the real `policy.json`): `valid= true`.

## Live cache: before / cache-report / cache-prune / after

Build-dir size before this lane's prune run: **130G** (grew from the 113G noted in the plan doc — this is a live,
actively-building repo with many concurrent lanes/dev sessions).

Real `cache-report` (dry run, no deletions):

```
[cache-report] cargo: 43.05 GiB across 3708 units; would delete 0 unused units (0 B) and 0 over-budget units (0 B); retains 43.05 GiB
[cache-report] cargo-incremental: 72.33 GiB across 923 units; would delete 0 unused units (0 B) and 418 over-budget units (29.85 GiB); retains 42.47 GiB; 42.47 GiB over budget but within the guard age
[cache-report] cargo-incremental-sessions: 39.28 GiB across 408 units; would delete 408 unused units (39.28 GiB) and 0 over-budget units (0 B); retains 0 B
[cache-report] vite: 117.12 MiB across 8 units; ...
[cache-report] agents: 0 B across 0 units; ...
```

Real `cache-prune` (one run, as instructed): deleted **833 units, 70.06 GiB** (almost entirely stale finalized
incremental sessions — the unconditional compaction pass — plus the LRU-evictable portion of `cargo-incremental`
that was past its 1h guard).

Before / after:

| | build-dir | `debug/incremental` | `debug/build` | `df -h /` avail |
| --- | ---: | ---: | ---: | ---: |
| before | 130G | 57G (original baseline; grew further under concurrent builds) | 11G | 235Gi |
| after | **78G** | **27G** | 19G | **286Gi** |

Re-running `cache-report` immediately after shows the design working live: `cargo-incremental` still reports
46.49 GiB "over budget but within the guard age" — expected on this heavily concurrent repo, where many crate dirs
are touched every few minutes by other lanes' builds; the 1h guard is deliberately conservative and will let the
next prune (auto-triggered ≤1h after any `native cargo build|check|test`, per lane S7's `scheduleThrottledCachePrune`)
evict more as those touches age past the guard. `cargo-incremental-sessions` had already regrown to 3.97 GiB from
new compiles in the few minutes between the two report runs — confirming the unconditional-compaction area is doing
real, continuous work independent of the LRU budget pass.

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts` — `CacheAreaInput.guardAgeMs`
  (optional per-area override), `findNamedDirs` helper, `scanCargoBuildUnits` (incremental branch removed),
  new `scanCargoIncrementalUnits`/`CargoIncrementalScan`/`sessionLockName`/`measureSession`, `deleteUnit`
  `cargo-incremental-session` case, new `cargo-incremental-session` `CacheUnitKind`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts` — `scanCacheAreas` (5 areas instead
  of 3), `areaUnitRoot` (`CARGO_AREAS` set), import of `scanCargoIncrementalUnits`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` — `storage.cargo.incremental`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/🔣️.json` — nested `incremental` object
  under `storage.cargo`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/cache-prune/🔣️.json`,
  `🧫️fixtures/cache-prune/🛂️schema/🔣️.json` — new areas, optional per-area `guardAgeMs`, new unit kind.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts` — Python oracle
  per-area guard, rebuilt real-disk incremental scan/compaction/deletion coverage.

No changes to `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` were needed — it already imports and calls `testCachePrune`
generically (lane S7).
