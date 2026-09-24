# GJ3 — inference_run wfc hang root cause

Slice: GJ3 · Session 9 · 2026-09-23
Scope: Root-cause and fix `inference_run` of `s.wfc.bitmap.solve` hanging after first
`notifications/progress` (0.05); prove solved bitmap via MCP in seconds; client-e2e 38/38;
host laws stay green; language-agnostic regression fixture.

## 0. Headline

**Root cause:** after GJ1's one-shot `pump_process_worker_pool()` (UI-frame bounds: 64 pumps /
`PROCESS_POOL_WALL_MS=2`, and `break` when `default_now_ms()` is `None`), a `semio.infer` Pump
state action could leave `WorkerJobPoll::Submitted`. Every later `step-job` re-observed Submitted
without finishing the worker step. MCP emitted progress 0.05 (contract check, host-side) then
blocked in `run_inference` for 600s / client-e2e 240s. GJ1 host watchdog laws stay green because
the guest never trapped — it starved.

**Fix (landed in source):** `pump_process_worker_pool_for_inference()` — count-bounded (65536),
no Instant wall, clock fallback `unwrap_or(pumps as u64)` so a missing clock never skips
`WorkerPool::pump`. `InteractiveInferenceJob::pump` calls it. Language-agnostic fixture
`inference-pool-pump-law.json` + Rust law + TS oracle.

**Proof status:** source + fixture + host laws green; WFC wasm rebuild queued on fleet wasm mutex
behind `tc5` (trusted-catalog-bootstrap) then `p1` then `wp-o3`. Live solve / client-e2e pending
rebuilt artifact + descriptor `hashes.wasmSha256` patch.

## 1. Inherited evidence

| source | takeaway |
| --- | --- |
| GJ1 | epoch/watchdog + one-shot pool pump + Infer-headless; laws green |
| GJ2 | host laws green; solve hangs 600s after progress 0.05; client-e2e stops at 30 PASS; wasm-dev (22 Sep 23:37, 129383519 B, MATCHES descriptor) vs newer component-dev (hash mismatch) |

## 2. Hypotheses under test

1. Stale wasm-dev vs component-dev — contributes (rebuild needed) but not the logic bug; MCP loads
   matching `cargo/target/.../wasm-dev` by `hashes.wasmSha256`
2. Job starved (pool pump never enough / clock skip) — **CONFIRMED**
3. Guest solve non-terminating — ruled out as primary (Submitted never becomes Outcome)
4. Stale MCP binary missing GJ1 host fixes — ruled out (laws green; progress 0.05 proves dispatch)
5. Progress fires once then pump/result path dead — **CONFIRMED** (0.05 is pre-guest)

## 3. Instrumentation path

gateway `inference_run_handler` (progress 0.05) -> `ActionAdapter.run_inference` ->
`PluginArtifactChannel::infer_real` -> `ArtifactInferenceRouter` -> `run_job_on_worker` /
`GuestColdRelayJob` step-job loop -> guest `InteractiveInferenceJob::pump` ->
`process_worker_pool` + `pump_process_worker_pool*` -> Outcome -> result bytes -> stdio

## 4. Measurements

| check | result | capture |
| --- | --- | --- |
| GJ2 solve run3 | FAIL 609.8s after channel=headless | wp-gj2/generated/gj2-wfc-solve-run3.txt |
| GJ2 client-e2e | 30 PASS then tools/call 240s timeout | wp-gj2/generated/gj2-client-e2e.txt |
| `pump_process_worker_pool` | wasm only; 64 pumps / 2ms; `None` clock breaks | reactor-turn.rs |
| InteractiveInferenceJob | single pump then Running on Submitted | infer-job.rs (pre-fix) |
| plugin `cargo check -p semio-framework-plugin` | ok | generated/gj3-check-plugin.txt |
| TS fixture oracle | PASS | `bun gj3-pump-law.ts` |
| host law `pump_law` | PASS 1/1 | generated/gj3-host-laws.txt |
| host laws `watchdog` | PASS 4/4 | generated/gj3-host-watchdog.txt |
| WFC cargo build | queued behind tc5/p1/wp-o3 | generated/gj3-wfc-cargo-build2.txt |

## 5. Root cause

The inference lane reused the reactor-turn pool pump (frame-sized). One `Submitted` worker step
that needs more than one 2 ms window (or that hits a `default_now_ms()==None` break) never reaches
`Outcome`/`Terminal`. Host `GuestColdRelayJob` keeps stepping; guest keeps answering Running with
scheduled progress; MCP never gets a terminal job result.

## 6. Fix

| file | change |
| --- | --- |
| `plugin/.../reactor/turn` | `pump_process_worker_pool_for_inference` + shared bounded driver; turn pump uses `Some(2)` wall; inference uses count-only + clock fallback |
| `plugin/.../jobs/infer` | call `pump_process_worker_pool_for_inference` after Submitted |
| `plugin-host/.../fixtures/inference-pool-pump-law.json` | language-agnostic law |
| `plugin-host/.../wasmtime-runtime` tests | fixture law test |
| `.tmp-ticket/wp-gj3/gj3-pump-law.ts` | third-party JSON.parse oracle |

## 7. Proof

| check | result | capture |
| --- | --- | --- |
| wfc solve via MCP | pending rebuild | gj3-wfc-solve.ts ready |
| client-e2e 38/38 | pending | |
| host laws | PASS (pump + watchdog) | gj3-host-laws.txt / gj3-host-watchdog.txt |
| regression fixture | PASS (TS + Rust) | gj3-pump-law.ts / host-laws |

## 8. Files changed

See §6. Scratch helpers: `gj3-wfc-solve.ts`, `gj3-patch-wasm-hash.py`.

## 9. Honest gaps

1. Live solve not yet re-proven — waiting on fleet wasm mutex to rebuild `semio_s_plugin_wfc.wasm`
   into shared `cargo/target/.../wasm-dev` and patch `hashes.wasmSha256` (full `describe` blocked by
   taxonomy peer breakage; manual hash patch is the interim path).
2. Mutex owner `tc5` held since 14:16 on `trusted-catalog-bootstrap --packages all` (gis 0/8).

## 10. Live proof log (2026-09-23 evening)

- Root fix landed; host laws PASS (pump + watchdog + wasmtime_runtime 9/9).
- First WFC wasm rebuild under gj3 finished (`Finished wasm-dev` in 1m34s) but cargo did **not**
  uplift into `cargo/target/.../wasm-dev/`; staged manually from build-dir out + patched
  `hashes.wasmSha256` → `98ad3a21…`.
- Symbol `pump_process_worker_pool_for_inference` present in staged wasm; MCP solve still timed
  out at 177s after `channel=headless` (same symptom). Strengthened infer-job to **while**
  Submitted: pump+repoll until settled or pool idle; clock fallback `1_000_000+pumps`.
- Second rebuild (`gj3b`) queued behind tc5 trusted-catalog-bootstrap (gis derive complete,
  gis build restarting). Waiting for mutex.

## 11. Live proof (continued)

### Standalone MCP solve — PASS

`bun ./gj3-wfc-solve.ts` with `GJ3_BUDGET_MS=600000` (capture `generated/gj3-wfc-solve-run5.txt`):

- `[5.4s] channel=headless`
- `[42.6s] inference_run isError=false` status=SUCCEEDED, contradiction=false, pixels present
- `[42.6s] PASS solved bitmap via MCP`

Staged wasm sha256 `98ad3a21…` (manual uplift from build-dir; descriptor patched).

### client-e2e — 34/37 (blocked on empty-artifact genesis)

Progress step reached `last progress=0.35` then `inference_run` timed out at 240s; subsequent
`notifications/cancelled` ping also timed out (server still busy). Root cause of the e2e hang:
default `BitmapInput`/`BitmapOutputSpec` were **1×1** while `pattern_size` defaults to **2**, so
overlapping extract never admits a window and the solve stalls. Fixed:

- Default genesis → 2×2 checkerboard
- Admission rejects `pattern_size > min(input.width, input.height)`
- Infer job: while-Submitted pump+repoll loop (strengthening)

Awaiting `gj3b` wasm rebuild (queued behind s14/o3b) to stage the genesis+admission fix, then
re-run client-e2e.
