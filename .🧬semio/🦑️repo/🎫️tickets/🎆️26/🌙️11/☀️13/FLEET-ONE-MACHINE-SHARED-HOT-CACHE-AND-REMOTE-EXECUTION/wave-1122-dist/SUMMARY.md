# 🧾️ Wave 1122-dist summary: semio-tech fully distributable, one fleet cache

Goal (user): every compile unit of a semio-tech build (native + wasm32, libs + component finals) can run on any ready worker; ONE fleet cache keyed only by the coordinator's synced sources + toolchain/flags, so no crate compiles twice anywhere; stale entries cleared; proven with numbers.

Timeline, decisions and every lane verdict: `coordination.md`. Per-lane detail: `<lane>.md` in this folder. Every landing with sha256 before/after: `T/landings.md`.

## Proof (P1, live fleet, dedicated proof workspace `proof/`)

| Phase | Where | Units | Compiled | Fleet hits | Result |
|---|---|---|---|---|---|
| A build | pinned to g1-m01 | 17 rustc + 4 build-script RUN | 17 remote, 0 local | 0 | all published to the fleet cache |
| B rebuild, fresh target + empty local store | pinned to g1-m08 | 17 + 4 | **0** | **17/17 + 4/4** | binary byte-identical to A |
| C edit `p-leaf` | g1-m11 | 17 + 4 | 3 (p-leaf, p-mid, p-top) | 14 + 4 | only the crate + dependents recompiled |
| D retention dry-run (scratch copy) | coordinator | 28 entries | – | – | exactly the 3 pre-edit entries stale (R2: 21 live kept) |

## What was wrong and is fixed

| Area | Defect (measured) | Fix (lane) |
|---|---|---|
| Key stability | cargo `-C metadata` is workspace-relative → same crate+slot re-keyed per workspace (301 transitions), "0 reused" on retries | wrapper rewrites `-C metadata` to a slot-level identity (K1 → K4, namespace v3); stable across workspaces AND edits |
| Portability | CARGO_HOME/rustup/sysroot/second checkout roots, `env!(OUT_DIR)`, `DEP_*` build paths, SDK symlink names in keys | portable tokens (K2), OUT_DIR/`⟨build⟩` neutral (K5, K6, P1 fix), SDK = `SDKSettings.json` CanonicalName (S1) |
| Build scripts | RUN outputs never fleet-shared; keys machine-bound | portable RUN keys + fleet lookup/claim/publish (K2), lane-neutral (K6) |
| Determinism | 23 `diverged` entries: incremental session CGU names | session-compiled units never offered/uploaded (K3) |
| Play build | native-build driver lane `g` never engaged the fleet tier (503 + 169 lookups without fleet) | one wrapper-env builder for every cargo entry point (N1) |
| Offload coverage | pressure-only `auto`; native cdylib/dylib/staticlib, `-l`, finals, build-script RUN never offloaded; unrelated env paths (NATIVE_PLAY_OUT: 214 declines/20 min); module-reach gaps; `inputs-too-large` on unreadable dirs | `distribute` default + `pin:<origin>` (O1), linked units + validator (O2), generic command offload for wasm-opt/transpile/probe/describe (O3, O4), build-script RUN offload (O5), read-env only + reach fixes (O6), exact extern closure (O9) |
| Parallelism | cargo `-j` = local grant (-j4) while ~40 fleet cores idle | `-j` = local grant + fleet idle slots, light slots for remote units (O7) |
| Robustness | lost remote action wedged the release build 15 min | client deadlines (4× cost), liveness watchdog, scheduler worker-loss requeue, `repo remote-execution cancel` (O8) |
| Stale = cleared | "current" = newest store; deleted crates never retired | staleness vs coordinator tree (source digests, toolchain, transitive edges incl. proc-macro/build-script/stage), stale-on-arrival 422, reachability for registry units, undeclared entries never judged (R1, R2) — LIVE in dry-run |
| Disk | host CAS 36–50 GiB vs 32 GiB cap | write admission at cap, continuous 2 GiB collection above 90 %, transient offload-input leases (CAS1) — LIVE |
| Stage finals | incomplete transpile/optimize keys; fleet hits hidden | complete keys, local vs fleet stats (D2) |
| Fundamentals fixed on the way | bare `bun test` scans whole checkout (ENFILE killed a build); wrapper seal closure 2,212 files; test budgets overflow schema under load; `incremental prepare --fresh` order + fleet facts; `repo cargo` capture walk serial; build-script RUN ignored SIGTERM; suites without Python binding | G1, F1, F2/F4, F3, C2, PY1 |
| Visibility | – | `repo fleet status`: CAS, retention, offload (X1) |

## Open (see coordination.md for detail)

- Retention `enforce`: flip after the play release chain exits and the live dry-run report is reviewed (the play session asked; X1's status shows "no report from cache host" until the sidecar runs the reader-compatible code).
- Test sweep T2a/T2b results (many files were starved under load 45–75 all night).
- `repo cargo` still spends ~250 s in command-stage setup and ~213 s inside `cargo metadata` before compiling (C2 open).
- Local compile-slot wait for build-script RUN has no time bound (progress + cancellable, not hung).
- Linux/Windows workers (g1-m04 disk, g1-m05 replica) not Ready; platform-split sharing applies.

## Day 2 (2026-11-23) addendum — state at 20:06

Full timeline: `coordination.md`. New lane reports: GR1–GR13 (graph), H1–H3 + O10–O12 (headroom, component jobs, memory charging), CAS2–CAS4 + R3 (store), SY1/SY2 (play snapshot sync lists), TI1/TI2 + PY1 + F2/F4 (test infrastructure), RP1 (replica bootstrap packs), W1/W2 (worker readiness).

### Regressions this wave caused and then fixed
- Continuous CAS collection (CAS1) + server unpinning (R3) deleted the blobs of local unit-cache entries → every build attempt recompiled from zero. Fixed in three steps: unit-cache roots are registered holders (CAS2), held blobs carry keeper hard links so old-code collectors skip them (CAS3), partition pressure never evicts held entries + `repo cache cas gc --unreferenced` (CAS4).
- A lane probe sealed the rustc wrapper into `units/` INSIDE the checkout (349 files, replicated) → 81 % of the graph's "workspace is stale" lines. Removed; relative cache roots now resolve to absolute; `sealed` excluded from graph inputs (GR5).
- The `taskpolicy -b` throttle rule slowed disk reads ~60× (1,124 s vs 33 s for one test file) and starved the wave's own tests. Withdrawn.
- ~70 tool edits invalidated graph owner records fleet-wide (records are valid only while a ~1,900-file tool closure is unchanged). Mitigated: equal re-derived answer keeps a record valid (GR12). Narrowing the closures themselves is DEFERRED (changes task hashes).

### Live numbers (20:06)
- Scheduler: 24 units executing remotely; play release build 1,748 units compiled in 115 min with 4 on the fleet at that moment; no `inputs-too-large` declines since the snapshot sync (O9).
- Host store 21.7 of 27.8 GiB; retention dry-run: 3,489 entries, 2,954 judged, 440 stale, 27 unreachable, 535 undeclared, 0 retired.
- Graph: fleet inference 2.4 → 6.25 owners/min, fleet-record adoption 70 % → 80 %; **Generation: none published** (no read has finished since 2026-11-20).
- Workers: **0 Ready**, 11 working (setup stages blocked on the graph).

### Not done
1. No graph head/generation published → workers still not Ready; `repo run` on a worker still blocked. (`repo cargo` needs no graph: warm 29–44 s on the coordinator.)
2. Component finals do not run on workers: finals need up to 6.5 GB (cad), 16 GB workers report 0–4.5 GB spare while their own setup stage runs; single charging (O12) is live on the scheduler, workers switch over by themselves; not yet observed.
3. Retention is dry-run; `enforce` is the user's decision (recommended after the play release passes once).
4. Play release has not completed a build; concurrent component jobs proven only with stub jobs (TI2).
5. Deferred: closure narrowing (GR10 options 1–2), SDK identity + key namespace v3 in the play snapshot, driver test file 46/24 on causes not traced to this wave, 6 pre-existing unbounded module Maps, Linux/Windows workers.

## Night 2→3 addendum — state at 01:15 on 2026-11-24

New lane reports since 20:06: GR14, GR15, W3, W4, W5 (worker readiness), R4 (graph heads on the cache server), O13–O17 (placement, local jobs, one-thread finals, remote codegen, expensive-unit rule), P2/P3 (final-on-worker proof attempts), SC1 (stage cache), RP1 (replica packs).

### Found and fixed
- Hour-long component finals: `semio-build-scheduler` added `-Z no-parallel-backend` to every cdylib final (codegen + ThinLTO on one thread). Now only at a grant of one job (O15, applied in the play snapshot 23:55); same on the remote path with a worker-side thread grant (O16). Output bytes proven identical at 1/2/4/9 threads.
- Offload slower than local (7 vs 25 units/min): cargo ran with 4 jobs and `distribute` sent all 4 remote. Cargo now gets the fleet-sized job count in every pass; placement is local-first; cost records time only the compiler (O13). Expensive units (≥120 s or ≥4 GB predicted) prefer a worker (O17).
- Component jobs were serial without remote slots; now also planned from local memory (O14).
- A graph head for the workers' 605-owner selection is published (first since 11-20) after two defects (≥24 GB machine waiting behind a 16 GB claimant; 318 MiB admission document over a 256 MiB limit, refused silently) (GR14). The cache server kept evicting it; heads/records are now a protected `graph` class with an eviction journal (R4).
- Worker read convergence: taken-owner memo lost under memory pressure, whole-tree inventory per round, seal wait (W4); per-pass re-verification of ~100k directory rows → stamp memo (W5).

### Not achieved
1. **Workers Ready: 0 of 11.** Fixes W4/W5 reach a worker only when its graph child restarts; 5 of 8 were still on older code at 01:10. Not observed working.
2. **A component final on a worker: unproven.** Two proof runs were stopped for disk (P2, P3). P3 measured 802 dependency units remote on g1-m12 at a 3.6 s median.
3. **A full release cache does not fit:** host store cap in force 27.75 GiB, full of in-use data; 2,202 of the play build's entries evicted for capacity between 20:24 and 00:40 → each restart recompiles about two thirds; offload uploads refused (507).
4. Root constraint: coordinator disk at 96–97 %. `/Users/ueli/Documents/semio.old-20261114/.cache` (146 GiB, retired checkout) is the largest reclaimable item — USER decision, asked by the play session.
5. Play release: not finished. Retention: dry-run (user decision). Server journal shows 2 small graph entries evicted for budget → R4 protection to re-check. Whole-graph generation head not in the protected class.
