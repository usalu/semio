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
