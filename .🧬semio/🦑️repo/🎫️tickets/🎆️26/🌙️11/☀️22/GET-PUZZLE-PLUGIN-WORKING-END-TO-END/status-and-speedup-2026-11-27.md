# 🧭️ Puzzle Plugin: Status, Problems and Speed-Up Proposals (27.11. 11:30)

## Where we are

| Area | Green this session | Red / open |
| --- | --- | --- |
| Browser | Vitest 58/58, Storybook catalog 60/60, story typecheck 0 errors, fresh wasm32 build + wasm-bindgen package (fold split fixed 59,755 locals), Playwright Chromium 19/20 | lasso story fix written, rerun pending; test-browser-* targets |
| WASI guest | component built (226 MB, max 907 locals), JCO describe: all 6 apps, descriptor regenerated | all 3 editor probes stall at Original source close (lane lclose); test-guest-io queued |
| 2D | first full run: 1,099 passed / 40 failed (30 suites) | class A example-load hang (lane larch), class B cursors > 4 KiB, class C fixes written, unverified |
| 3D | mutation_latency measured (13/16 turns vs 8), 8 of 9 latency tests pass, 3d-import-a green | no production fix yet; panels/66-case/settlement not run |
| 5D | first full run: 835 passed / 33 failed (60 suites); verify-optional-fields green | 17 UI-arena registration, 2 fill, 2 asset, ~12 mutation-schema |
| Registered targets | all TS/script targets green (architecture, fixtures-lint, composition, snapshot-sqlite source, ownership) | cargo-based verify-*/snapshot-native batch still running |

Expected finish: not today on this host. Each heavy cargo run waits 30–120 min for memory and compiles 30–120 min; each lane gets about 2–3 runs per 8 h. Remaining work needs several iterations per lane (2D A/B/C, 3D latency, 5D four classes, guest close stall). Realistic: 1–2 more days on this machine alone, hours if builds move to the fleet.

## Problems hit and fixed

- Disk 3.8 GB → freed ~350 GB total over the night (stale generated trees, old builder, wasm incremental 44 GB, per-crate hash dirs ~120 GB).
- My own mistakes, fixed: `cp -al` seeding shared cargo locks and ~34,000 artifacts between lanes (hours of phantom E0599/E0425 and lock waits); a bfs `-newermt '-120 minutes'` filter that silently matched everything (killed one browser build); pruning during a live build (killed 5D run 18).
- Foreign half-finished framework edits broke every lane repeatedly (job-context split 162 errors, job payload constant, store engine witness, LocalizedLabel, test-support refactor missing in ~10 manifests). Fixed or waited out: reactor app path, job-context facade re-export, test-support dev-deps (2D v1, text-inverse, editor unit tests, pdf presence, 6 generation3d crates), store engine witness delegation, 5D job import.
- Real production defects fixed: browser module too many locals (fold split), `artifact_with` re-export missing (2D v1), capacity gate waiters dying on unrecognized grants (version skew; fixed with test, lane lcap), fixtures-lint scanned the whole repo, 2D delete-inverse private field, selection inverses reversed, canvas testkit root, stale fixtures/assertions.

## Resource use

Not saturated, and that is the main problem. At 11:27: 7.8 GB available, 5.8 GB spare, load 9 on 12 cores, but only 2 grants run (1.3 GB observed) while 8 requests wait, because the strict FIFO barrier holds everything behind an 8,000 MB head that cannot fit. Agents: 8 lanes (Sonnet) + 1 audit (Haiku) were used; more agents would only queue. The 15 fleet workers (remote execution, all idle) are unusable for plain cargo, and the repo graph path stalls in planning.

## Anticipated

- larch finds the replay-initializer bind-genesis stall; that likely unblocks most 2D class A and the 5D fill/asset failures.
- 5D UI-arena registration (17 failures) is one shared fixture/registration fix.
- The guest close stall is one framework fix that turns all three editor probes green.
- The 3D latency law needs real production changes in retirement/publication; it is the longest pole.

## Clean long-term proposals for development speed

1. **Gate admission by observed memory, with backfill.** Reservations are 2–8× above observed RSS (8,000 reserved / 1,300 observed). Admit by a learned per-crate peak, let small requests backfill past a large head when the head's start time is not delayed (EASY backfill), and give each agent sub-session its own budget instead of one session for all lanes.
2. **Make the fleet run cargo.** 15 idle remote workers exist. A tool-neutral `repo cargo test` that ships crate compilations through the remote-execution API (sccache-style rustc wrapper against the REAPI CAS) would turn 2 h local compiles into minutes.
3. **Shared, content-addressed build cache instead of per-lane target dirs.** Per-lane dirs cost 80 GB each, and seeding them is dangerous. A rustc wrapper with a shared CAS (keyed by inputs, not timestamps) gives every lane warm artifacts without lock or staleness hazards, and garbage-collects old configurations automatically.
4. **Never let the shared worktree be uncompilable.** Most lost hours came from half-finished edits replicated to every machine. Options: edit in a private overlay and publish only after `cargo check` of the touched crates passes (pre-publish gate in the replica daemon), or atomic multi-file publication per change set.
5. **Version-skew tolerance as a rule.** Every long-running reader (gate, warden, graph) must tolerate newer schemas (ignore unknown fields, keep waiting). The lcap fix should become a schema convention with a test template.
6. **Split oversized test crates.** 2D/5D lib-test crates need about 8 GB to compile; carving them along the taxonomy (the warden already asks for it) makes 3–4 run in parallel on 16 GB machines.
7. **One-command lane setup.** A `repo lane open <name>` that creates an isolated target, scratch dir and capacity sub-session, and `repo lane close` that prunes it, would remove the hand-made scripts (current-native-tests.sh, builder.sh, …) every coordinator re-invents.
8. **Agent hygiene in the brief.** Debug prints must be env-guarded and allocation-free (one print broke a heap-audited test), and long waits should be armed on log markers rather than agent turns.

## Update 27.11. 18:00

| Area | State |
| --- | --- |
| Browser | FINAL: Vitest 58, catalog 60, typecheck, fresh wasm (max locals 7,648), Playwright 20/20 (4 load flakes on rerun). test-browser-* targets blocked by the repo graph service restarts. |
| WASI guest | component built, JCO describe 6/6, descriptor regenerated; editor probes stall at Original source closure (lane lclose: first fix written, native test queued); test-guest-io never admitted. |
| 2D | full run 1,249 passed / 23 failed (was 1,099/40). Example-load livelock gone (larch fixed persisted-binary registration). Panels 10/10. Remaining: fill work-quote refusals, fill jobs, agent-lane close quote, remote cancel, replay progress (lane l2d2); cursor sizes (lcore2), inverse/prep tests (linv). |
| Framework | RetainedCloneBinding 976 → 144 B via compact close (lcore2, ladder-model law test green); connect-preparation 4,080 B and inverse 4,016 B fit; candidate cursor still 16,416 B. Capacity gate waiter crash fixed with test (lcap). |
| 3D | latency 12/14 turns (was 13/16, limit 8). The retirement continuation broke receipt accounting: editor package 199 passed / 204 failed (197 "Close receipt differs from its original caller authority", 65 vs 64 items). Lane l3dfix must fix or revert. |
| 5D | 835 passed / 33 failed; mutation registration fixed like 2D; reruns queued (measured peak 7.0 GB). |
| Registered targets | TS/script targets green; cargo items green: 3d-import-a, 3d-lifecycle, 3d-wire, 3d-snapshot-sqlite-native, 5d-import, 5d-optional; most others refused by the gate (8,000 MB not reachable while other agents' graph workers hold 7.5 GB) and being rerun at right-sized reservations. |

## Update 28.11. 00:00

| Area | State |
| --- | --- |
| Browser | FINAL (unchanged): all browser suites green; 3 test-browser-* targets blocked by the repo graph service. |
| WASI guest | close-stall root cause found and fixed natively (lclose: unacknowledged settled return-escrow row pinned the Source close; regression test passes, 2474-step stall reproduced before the fix). Component rebuild (5.2 GB) queued; then the 3 editor probes and test-guest-io. 3 other guest-contract tests fail in untouched paths (not compared to baseline). |
| 2D | 1,249/23 at last full run. Fill close: yield fix moved the failure to a receipt assertion (lane l2d2). Inverse/prep lib now 7/9; the two inverse tests stall in the cursor close ladder (lane linv, traced run queued). Custody tests in that lib pass. All 4 cursors ≤ 4096 B. |
| 3D | editor package 392/11 after the receipt fix; frontier guard restored (editor4 queued); brush-verb fix written; latency law 11–12/12–13 turns (limit 8), store-record compact close next (lane l3d3). |
| 5D | light per-package runs; found a latent defect: excluding a member drops dev-dependency feature unification so mut-any-5 misses testkit serde derives (lane l5d). |
| Registered targets | all TS/script targets and 3D cargo items green (import-a/b, lifecycle, fill-native, wire, snapshot-sqlite), 5D import and optional green; 2D/5D items re-queued at right-sized memory. |
| Host | memory guard (warden) not running since ~18:00; capacity barrier stalls when 8 GB heads wait; other agents' tooling holds ~7 GB. |
