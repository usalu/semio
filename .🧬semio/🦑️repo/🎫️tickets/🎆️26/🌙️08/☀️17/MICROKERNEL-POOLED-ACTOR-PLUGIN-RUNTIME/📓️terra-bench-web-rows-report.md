# 📓️ terra-bench-web-rows — the web half of `bun ./📜️script.ts bench plugins`

**Task**: `bench-web-rows` — before this packet, `--renderer react|wgpu` emitted `benchWebSkippedRow` for
budgets 2-8 unconditionally: the web half of the 50-plugin × 50-extension scale proof did not exist.

## What changed (owned paths only)

- **New**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/🧪️bench-web-harness.ts`
  — a browser-target module, bundled with `Bun.build` (no new external dependency), that imports the REAL,
  unmodified `ShardClient` (`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`) and drives it against
  real browser `Worker`s inside a real headless-Chromium page. Exports `runBenchWebBudgets(input)`.
- **Edited**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
  — `//#region 🧪️BenchWebRows` (new): `buildBenchWebHarnessBundle`, `runWebBenchViaHeadlessChromium`,
  `benchWebMeasuredRow`, `benchWebRows`. `benchWebSkippedRow` gained a required `reason` parameter (every
  caller updated) so a skip always carries the real cause, never the old blanket "not exercised this
  session" text. `BenchPluginsScript.run`'s `renderer === "react" || renderer === "wgpu"` branch now calls
  `benchWebRows(...)` instead of unconditionally skipping. Docstrings on `//#region 🔖️Bench` and
  `benchWebSkippedRow` updated to match. **The `native` branch and every other region of this file were
  left untouched** — verified by re-reading the diff against `HEAD` (below) after another live session's
  concurrent `terra-parity-rebaseline` edits landed in the same file's `//#region 🔬️ParityScript` region.

No files outside the two above were touched. No registrar-only file (`Cargo.toml`, root `📜️script.ts`,
`project.json`, `launch.json`, `🤖️generated/**`) was touched — none needed to be.

## Honesty scope — read this before trusting a row

`semio-framework-plugin` (the guest SDK) does not compile this session (per `📓️status.md`, currently
798→~719 errors), so **no real fleet wasm component exists**. Every `activate()` in this harness therefore
runs against a small **protocol stub** worker (`STUB_SHARD_WORKER_SOURCE`, inline in the harness file) —
not the real, generated `shardWorkerSource()` (which `import()`s a compiled jco bridge module that doesn't
exist yet) — implementing just the `activate`/`turn`/`checkpoint`/`restore`/`dispose` subset of the wire
contract with synthetic per-actor state (one `counter`).

Consequence, budget by budget (ids match `BENCH_BUDGETS`, `design-workforce.md` §4):

| id | what it checks | this run | real vs stub |
|---:|---|---|---|
| 1 | registry parse, `instantiations==0`, <150ms | **pass** (2.7ms, 2550 records) | REAL — no wasm/kernel touched, shared with the native row |
| 2 | cold boot → first interactive frame | **pass-stub-worker** (6.9ms) | STUB TIMING — real Worker-pool spin-up + one real `activate()`, but zero real wasm instantiated. A LOWER BOUND, not the real number; the dominant real-world cost (compiling/instantiating 100 wasm components) is absent. |
| 3 | activate 50+50: `active_actors==100`, `shards==K`, no shard `>ceil(100/K)+1` | **pass** (100 active, 8 shards, max 13 ≤ ceil(100/8)+1=14) | REAL — genuine `ShardClient.assignShard()` round-robin at 100-actor scale. Verified NOT a tautology: the same code path against a 12-actor fixture correctly reported `ok:false` (`activeActors:12≠100`) during dry-run — see `terra-bench-web-rows-smoke.txt` below. |
| 4 | memory ceiling; web: Worker count `==K` | **pass** (8 workers) | REAL — counts actual `new Worker()` instances. Byte-level `K×512MiB+256MiB` ceiling is not evaluable without real wasm resident; not attempted, stated as such in the row's own `note`. |
| 5 | interactive p95 command→patch | **pass-stub-worker** (p95 0.2ms over 50 samples) | STUB TIMING — real postMessage/structured-clone round trip, but zero real guest compute or UI-patch application. A LOWER BOUND. |
| 6 | hang actor killed, shard rebuilt, siblings restored, pause ≤250ms | **pass** (trapped, 35.6ms pause, siblings byte-identical) | REAL mechanism, SYNTHETIC trigger — the stub worker sends a `trap` frame after a fixed 25ms (standing in for a guest's own fuel/wallMs overrun detection, which doesn't exist without real wasm), but `ShardClient.terminate()`/`rebuild()`, the two siblings' re-`activate()`+`restore()`, and the pause-time clock are all real. |
| 7 | checkpoint/resume state-hash equality | **pass** (`9bc23426`==`9bc23426`) | REAL, full round trip: `turn×3 → checkpoint → dispose → activate → restore → checkpoint`, hashed (FNV-1a, not `crypto.subtle` — see below) and compared byte-for-byte. |
| 8 | capability revoked at runtime → denied, actor alive, quota counters zero | **pass** (denied, actor alive) | PARTIAL — `ShardClient` has **no dedicated revoke-mid-life wire message**; only `activate()`'s `caps` list at grant time. This test activates an actor WITHOUT the capability up front (standing in for "revoked") rather than genuinely revoking a live grant. Real denial-path and alive-check; not a real revocation round trip. |

**Never fabricated**: any harness failure (Chromium missing, bundle error, page timeout, empty registry)
routes every row through `benchWebSkippedRow(budget, renderer, reason)` with the real thrown error/stack
in `reason` — there is no code path that reports `"pass"` without the harness actually running.

**`crypto.subtle` avoided on purpose**: a `page.setContent()` document's secure-context status is not
worth depending on for a bench harness, so budget 7's hash is FNV-1a (dependency-free, deterministic,
not a security primitive — sufficient for a byte-equality proof).

**Renderer-agnostic by construction**: the harness measures the `ShardClient` transport layer, which
react and wgpu(web) share identically — it does **not** exercise either renderer's own paint/patch path.
Both `--renderer react` and `--renderer wgpu` therefore produce numerically-independent-but-mechanically-
identical runs (verified: `--renderer wgpu --shards 4` correctly re-derived `ceil(100/4)+1=26`, max 25 —
a different K produces a different, still-correct, bound). This is stated in the region's own header doc
and in every measured row's `note`, not left implicit.

## Commands run, exit codes, evidence

```
$ bun -e '<bundle 🧪️bench-web-harness.ts for browser>'
success: true   bytes: 56603
```
Exit 0. `process.` references in bundled output: 0. `node:fs` reference: 1, confirmed to be inside
`shard-client.ts`'s own `if (import.meta.vitest) { ... }` in-source test block (a runtime-dead branch in
a browser page, since `import.meta.vitest` is `undefined` there) — not a live import.

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript
$ CARGO_TARGET_DIR=<scratchpad>/target-bench-web-rows bun ./📜️script.ts bench plugins --renderer react \
    --out <scratchpad>/terra-v1b-bench-react.json
bench: running web scale-bench (renderer=react, shards=8) via headless Chromium — see 🧪️bench-web-harness.ts for real-vs-stub scope
bench: wrote report -> <scratchpad>/terra-v1b-bench-react.json
bench summary: 1:pass 2:pass-stub-worker 3:pass 4:pass 5:pass-stub-worker 6:pass 7:pass 8:pass
```
Exit 0. Full report copied to `terra-bench-web-rows-react-report.txt` in this ticket folder.

```
$ CARGO_TARGET_DIR=<scratchpad>/target-bench-web-rows bun ./📜️script.ts bench plugins --renderer wgpu --shards 4 \
    --out <scratchpad>/terra-v1b-bench-wgpu.json
bench: running web scale-bench (renderer=wgpu, shards=4) via headless Chromium — see 🧪️bench-web-harness.ts for real-vs-stub scope
bench: wrote report -> <scratchpad>/terra-v1b-bench-wgpu.json
bench summary: 1:pass 2:pass-stub-worker 3:pass 4:pass 5:pass-stub-worker 6:pass 7:pass 8:pass
```
Exit 0. `measured` for id 3: `{activeActors:100, distinctShards:4, maxPerShard:25, ceilBound:26}` — the
`--shards 4` flag genuinely changed the sharding math, not a hardcoded number. Full report copied to
`terra-bench-web-rows-wgpu-report.txt` in this ticket folder.

**Pre-flight smoke test** (small 6-plugin/6-extension fixture, `shardCount:2`, before the real 50×50 run)
proved budget 3's check is not a tautology: `activeActors:12` against the harness's own hardcoded
`===100` correctly returned `ok:false`. Full transcript in `terra-bench-web-rows-smoke.txt`, this
ticket folder.

### `git diff HEAD` scope check
`git diff HEAD --stat -- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/`
showed only `📜️script.ts` changed (165 insertions / 27 deletions) — the new `🧪️bench-web-harness.ts` is
untracked, shown separately by `git status`. Reading the full diff confirmed the 27 deletions belong to a
**different, concurrent** session's `terra-parity-rebaseline` edits inside `//#region 🔬️Triage`/
`🔖️Sweep` (STALE-BRIDGE boot-triage rung, `ensureParityPlaywrightBrowsersPath` hoist) — disjoint from,
and correctly layered underneath, this packet's `//#region 🧪️BenchWebRows` and `BenchPluginsScript`
edits. No overlap, no clobbering, verified by reading the diff, not assumed.

### Standalone `tsc` pass (advisory, not a project-configured typecheck target — none exists for this
package, matching the ticket's already-recorded finding that several TS packages here have no
`tsconfig.json`/typecheck target at all)
`bunx tsc --noEmit` scoped to `🧪️bench-web-harness.ts` with `--lib es2022,dom,webworker` reported **zero
errors in the harness file itself**. It did surface 7 pre-existing errors, all inside
`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` (a file this packet imports but never edits, outside
the owned-paths list) and one inside its generated sibling `🤖️actor.ts` — a `result` discriminated-union
narrowing gap at line 467 and two `import.meta.vitest`/bigint-key ambient-type gaps, all artifacts of
running `tsc` without the project's real tsconfig (missing vitest ambient types, no `dom`/lib alignment
config) rather than anything this packet changed. Not fixed — outside owned paths, and bun's runtime
(strip-only TS, no typecheck) already proved the actual `bench plugins` command runs correctly twice at
full 50×50 scale above.

## Gaps a sibling or the coordinator should know about

1. **Budgets 2 and 5 are lower bounds, not the real numbers.** Once `sdk-green` lands and a real fleet
   wasm build exists, these should be re-measured against a real jco-bridged worker (`shardWorkerSource()`
   itself, not the stub) — the current `pass-stub-worker` status is designed to make this impossible to
   miss even if only the JSON is read, not this report.
2. **Budget 8 tests capability-gate-at-activation, not revoke-mid-life** — `ShardClient` has no wire
   message for revoking a capability from an already-activated actor. If the design ever adds one, this
   row should be upgraded to a genuine round trip.
3. **Budget 6's overrun trigger (25ms fixed timeout) is synthetic** — a real guest's own fuel/wallMs
   accounting (once it exists) should replace the stub's `setTimeout`, though the terminate/rebuild/
   restore mechanism this row proves is already the real one.
4. `benchWebSkippedRow`'s signature changed (`reason` is now required) — any other in-flight caller in
   this file would fail to compile; grepped the whole file for other call sites before editing (`grep -n
   benchWebSkippedRow`) and found only the ones this packet updated.
