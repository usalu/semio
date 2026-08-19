## the rule as implemented

A deadline-bounded poll of an EXTERNAL resource that emits no observable event — a TCP port or HTTP
endpoint belonging to a process we did not instrument, a lease file another `dev` invocation owns, a
filesystem lock — is "event-driven-unavailable" and acceptable. A poll of a resource we spawned and
whose handle we hold (a child's own `exit` event, a stream, a promise it already exposes) is NOT
acceptable; await the handle instead.

Written verbatim as a docstring on a new `//#region 🔖️PollHelpers` at
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:1497` (right
before `//#region 🔖️PluginBuildLease`, its first consumer), so it sits directly above the three
helpers it governs (`awaitTcpReady`, `awaitHttpOk`, `awaitChildExit`).

## the 2 event-driven fixes

Both polled `child.exitCode` in a `Bun.sleep(250|500)` loop while already holding the `SpawnDaemonHandle`
(`{ child: ChildProcess; kill }`, from the shared `spawnDaemon` in the library index — confirmed it is a
plain `node:child_process` handle, not `Bun.spawn`'s `Subprocess`, because existing call sites already do
`daemon.child.stdout?.pipe(logStream)`, a Node-`Readable`-only method).

Node's `ChildProcess` sets `exitCode` synchronously and then emits `'exit'` exactly once — no polling
needed. New helper `awaitChildExit(child, deadlineMs, opts?)` (🔖️PollHelpers) races
`child.once("exit", …)` (short-circuited via the already-set `exitCode` check, so a child that exited
before the call doesn't hang) against a `setTimeout`-based deadline, returning `"exited" | "timeout"`.

- `collabRunRestartStep` (was `~:2974`, now `~:3091`): `await Bun.sleep(250)` loop on
  `opts.hubDaemon.child.exitCode === null` → `await awaitChildExit(opts.hubDaemon.child, 30_000)`, same
  30s budget, same `spaceE2eAssert` message.
- `prebuildParityPlugin` (was `~:3859`, now `~:3980`): same shape → `await awaitChildExit(daemon.child,
  PARITY_DEV_SERVER_BOOT_BUDGET_MS)`, same budget/error message.

## the helper + its 7 call sites

Of the 7 nominated sites, only 5 are genuinely TCP/HTTP-shaped; the other 2 (lease-file, mkdir-lock) are
filesystem polls on a PID/lock we hold no handle for at all — forcing them through a TCP/HTTP-shaped
helper would misrepresent what they wait on, so those 2 got judged individually (next section + below).

`awaitTcpReady(host, port, { deadlineMs, intervalMs, mode?: "open"|"closed", isDead?, probe?, sleep?,
now? })` — polls `isDevPortInUse` (default `probe`) until open (`mode:"open"`, default) or closed
(`mode:"closed"`), optionally racing an `isDead()` predicate. `awaitHttpOk(url, { deadlineMs, intervalMs,
init?, isDead?, fetchImpl?, sleep?, now? })` — polls `fetch(url, init)` until it stops throwing (does not
check `.ok`). Both return `"ready"|"dead"|"timeout"`, never throw — callers keep their own error messages.
`probe`/`fetchImpl`/`sleep`/`now` are test-only injection points.

Call sites (deadline/interval each preserved exactly from the original loop):

1. `~:1730→~:1848` — wgpu trunk-stop port-freed wait. Was an attempt-bounded `for` loop (40 × 250ms);
   converted to `awaitTcpReady(host, port, { deadlineMs: 10_000, intervalMs: 250, mode: "closed" })`,
   same effective 10s budget. Outcome intentionally unused — original loop didn't gate on it either.
2. `~:2551→~:2661` (`collabStartHub`) — HTTP poll of `/admin/api/overview`, now `awaitHttpOk(url, {
   deadlineMs: COLLAB_E2E_HUB_BOOT_BUDGET_MS, intervalMs: 500, init: { headers: {...} }, isDead: () =>
   daemon.child.exitCode !== null })`. `"dead"`/`"timeout"` map to the original two distinct error
   messages.
3. `~:2679→~:2790` (`collabStartUserDevServer`) — port-ready wait, now `awaitTcpReady("127.0.0.1",
   opts.port, { deadlineMs: COLLAB_E2E_DEV_BOOT_BUDGET_MS, intervalMs: 500, isDead: () =>
   daemon.child.exitCode !== null })`.
4. `~:2977→~:3093` (`collabRunRestartStep`) — port-freed-after-kill wait, now `awaitTcpReady("127.0.0.1",
   opts.hubPort, { deadlineMs: 30_000, intervalMs: 250, mode: "closed" })`.
5. `~:3896→~:4013` (`startParityDevServer`) — port-ready wait, now `awaitTcpReady("127.0.0.1", port, {
   deadlineMs: PARITY_DEV_SERVER_BOOT_BUDGET_MS, intervalMs: 500, isDead: () => daemon.child.exitCode
   !== null })`.

## the mkdir-lock verdict

Left as a legitimate poll, commented, NOT given a helper (`prebuildParityPlugin`, `~:3841→~:3961`, the
`mkdirSync(lockPath)` / `EEXIST` retry loop). It is a cross-process `mkdir`-as-mutex over a shared
`target/` dir; the lock's holder may be an entirely separate `parity` invocation this process never
spawned, tracked by nothing but the lock file's mere existence — no pid, no handle, no port, no HTTP
endpoint. A TCP/HTTP helper genuinely does not fit this shape, and it's the only mkdir-lock site in the
file (one comment beats a one-call-site helper). Comment added inline citing THE RULE.

Same verdict, same reasoning, for `waitForPluginBuildLeaseReady`'s lease-file poll (`~:1586→~:1700`,
nominated as call site #1 in the brief but structurally identical to the mkdir-lock: it polls a JSON
lease file's `registryReady` flag + `isPidAlive(lease.pid)`, no TCP/HTTP/handle involved — the holder is
potentially a wholly separate `dev` process this one never spawned). Commented in place, not helperized,
for the same reason: forcing an fs+pid poll through `awaitTcpReady`/`awaitHttpOk` would misdescribe it.
So the "7" resolve as: 5 through the 2 new helpers, 2 judged-and-commented fs-based polls that were never
TCP/HTTP-shaped to begin with.

## commands + exit codes

```
$ cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript" && bun ./📜️script.ts test
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript
 Test Files  1 passed (1)
      Tests  27 passed (27)
   Start at  10:34:02
   Duration  994ms
EXIT:0
```

```
$ bun ./📜️script.ts test --reporter=verbose   (same dir)
 ✓ ... awaitTcpReady (W6 poll-census helper) > honours its deadline and reports timeout — no real sleeps: fake clock + fake probe 0ms
 ✓ ... awaitTcpReady (W6 poll-census helper) > resolves ready as soon as the injected probe reports the port open 0ms
 ✓ ... awaitTcpReady (W6 poll-census helper) > resolves closed-mode ready once the injected probe reports the port free 0ms
 ✓ ... awaitTcpReady (W6 poll-census helper) > resolves dead as soon as isDead() reports true, before the deadline 0ms
 ✓ ... awaitHttpOk (W6 poll-census helper) > honours its deadline and reports timeout — no real sleeps: fake clock + always-throwing fetch 0ms
 ✓ ... awaitHttpOk (W6 poll-census helper) > resolves ready once the injected fetch stops throwing 0ms
 ✓ ... awaitHttpOk (W6 poll-census helper) > resolves dead as soon as isDead() reports true, before attempting to fetch 0ms
 ✓ ... awaitChildExit (W6 event-driven fix — replaces polling child.exitCode) > resolves as soon as the child's own 'exit' event fires, without polling 0ms
 ✓ ... awaitChildExit (W6 event-driven fix — replaces polling child.exitCode) > resolves immediately for a child that had already exited before the call 0ms
 ✓ ... awaitChildExit (W6 event-driven fix — replaces polling child.exitCode) > still times out for a hung child that never emits 'exit' — fake deadline, no real sleep 0ms
 Test Files  1 passed (1)
      Tests  27 passed (27)
EXIT:0
```

Full verbose output archived at `📓️terra-dev-polls-test-verbose.txt` in this ticket folder.

## baseline vs after + proof tests ran by name

Baseline was **17 passed** (W5-fixed vitest config, `include: []` + `includeSource: ["📜️script.ts"]`).
After adding 10 new in-source tests directly inside the existing `if (import.meta.vitest) {...}` block
in `📜️script.ts` (already covered by `includeSource` — no second file, no config edit needed): **27
passed** = 17 + 10, all 10 new tests print by full name in the `--reporter=verbose` output above/archived
file, under 3 new `describe` blocks (`awaitTcpReady`, `awaitHttpOk`, `awaitChildExit`) in a new
`//#region 🔖️PollHelpers-tests`.

Test design, no real sleeps anywhere:
- `awaitTcpReady`/`awaitHttpOk`: `now`/`sleep`/`probe`/`fetchImpl` are all injected — deadline tests use
  a fake clock that `sleep` advances synchronously (no real `setTimeout`/`Bun.sleep`), so "honours its
  deadline" resolves in 0ms wall-clock while still exercising real deadline arithmetic.
- `awaitChildExit`: deadline test injects `timeoutAfter: () => new Promise<"timeout">(() => {})` — a
  promise that **never resolves** — proving the "resolves off the event" case can only be won by the real
  `'exit'` listener, not a timer. The hung-child case injects `timeoutAfter: async () => "timeout"`
  (resolves instantly) to prove the timeout path without a real wait.

## lease-requests

None. All edits stayed inside the owned path
(`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`); no `dist/asset/**`
or bench-region touches.

## honest gaps

- No new runtime dependency added (`node:events`' `EventEmitter` is a Node builtin, used only for the
  test's fake `ChildProcess`, not in the shipped helper code path itself, which relies solely on the real
  `ChildProcess.once`).
- `SpawnDaemonHandle["child"]` is typed as `ChildProcess` (from the shared library's `spawn`, i.e.
  `node:child_process`) but `📜️script.ts` never previously imported that type name directly — I avoided
  adding a `node:child_process` type import by indexing off the already-imported `SpawnDaemonHandle`
  type, so `awaitChildExit`'s signature has zero new external type surface.
- I did not run `bun ./📜️script.ts verify` or any collab/parity e2e paths themselves (they spawn real
  processes/dev servers and would be slow, and are out of scope for a poll-shape refactor) — only the
  package's own `test` target, as the acceptance section specifies. The 5 TCP/HTTP call sites' behavior
  under a live process is therefore verified by code-reading + unit tests of the helpers in isolation,
  not by an end-to-end run of `collab-e2e`/`parity` themselves.
- Did not run a standalone `tsc --noEmit` (no local `tsconfig.json` in this package, and the repo-root
  config wasn't explored to avoid scope creep); `bun ./📜️script.ts test`'s esbuild-based transform
  compiled and ran the file successfully, which catches syntax errors but not full type errors. I
  hand-verified types for the new code (indexed types, casts, optional fields) but this is a gap if a
  latent type error exists elsewhere in the type graph.
- Confirmed via `grep` that no other `Bun.sleep` sites remain outside the 3 helpers + the 2
  judged-and-commented fs polls (9 accounted for exactly). Also found other `while (Date.now() <
  deadline) … await page.waitForTimeout(…)` loops elsewhere in the file (Playwright DOM-condition polls,
  e.g. `waitForStudioE2eCondition`) — these are a different primitive (`page.waitForTimeout`, not
  `Bun.sleep`) and outside the census's stated scope ("9 `Bun.sleep` poll loops"), so left untouched.
