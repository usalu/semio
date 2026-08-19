# terra-web-shellhost — report

Owned path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` (6882 lines before, 7026 after). No other file was edited — three module-scope helpers (`pluginInstallConcurrency`, `runInvokeExtensionEffect`) were added INSIDE this same file rather than exported elsewhere, specifically to avoid touching `📦️index.tsx`/`🧪️vitest.config.ts`, which are outside this packet's lease. See `## lease-requests`.

## findings confirmed vs not reproduced

All five audit findings reproduced against the live file, at drifted line numbers (other packets' earlier edits shifted the file down ~60–120 lines by the time this packet started; content and shape matched the audit exactly).

1. **Confirmed** — `applyHostEffects`'s `invokeExtension` branch (found at pre-edit lines 2859–2896, audit said ~2859-2895) `await`ed `invoke()` then `completeExtensionInvoke()` inline inside the `for...of effects` loop. Line 2314 (audit-cited) is a caller — `await applyHostEffects(response.requestedEffects, nextSession)` in the boot effect — confirmed present and unchanged; it illustrates the blast radius (every caller awaiting the whole batch stalls), not itself a fix site.
2. **Confirmed** — the registry-streaming effect (pre-edit lines 2267–2281, audit said ~2197-2210) called `handlePluginAvailable` synchronously over every plugin in a `snapshot` event (potentially ~20 at cold boot) and each call fired `void installPlugin(...)`/`void reloadPlugin(...)` with no concurrency bound and no cancellation.
3. **Confirmed** — the identity-bootstrap effect (pre-edit lines 1370–1373, audit said ~1304-1307) awaited `new Promise((resolve) => { identitySnapshotResolverRef.current = resolve; setTimeout(() => resolve(null), 2000); })` — a fixed 2s wait with no early-exit on the real event and no `clearTimeout` on unmount.
4. **Confirmed** — three `fetch("/extensions/install", ...)` call sites (pre-edit lines 1793, 1872, 1957 — audit said ~1699/1777/1861 — `installExtension`, `installExtensionFromFile`, `uninstallExtension`'s DELETE) carried no `AbortSignal`.
5. **Confirmed** — the presence-beat effect (pre-edit lines 4072–4098, audit said ~3946-3981) looped `for (const [documentId, entry] of openDocumentSessionsRef.current)` and `await`ed `entry.plugin.ephemeralSnapshot?.(...)` sequentially per document, guarded by one shell-wide `publishing` boolean.

Nothing had already been fixed or made stale by a peer edit; all five reproduced cleanly.

## line ranges edited

Registrar-shared file — exhaustive list, given as pre-edit line anchors from `git diff -U0`:

- **L64, L109** (imports from `@semio-tech/framework`) — added `latestWins,` and `waitForEvent,` to the existing named-import block.
- **L475** (imports from `../PluginRuntime/🟦️component.tsx`) — added `serializePerActor` to the existing import.
- **L897–899** (new, +64 lines) — new `//#region 🧵️ConcurrencyHelpers` / `//#region 🔁️InvokeExtensionDispatch` module-scope block inserted between `resolveShellScopeStorage` and the `FrameworkOsShell` doc comment: `pluginInstallConcurrency()` (finding 2's concurrency bound) and `runInvokeExtensionEffect(...)` (finding 1's extracted extension-call body, byte-identical logic to what was inline before, just parameterized instead of closed over).
- **L1143–1149** (+11 lines) — two new refs added beside `presenceCursorRef`/`openDocumentSessionsRef`: `presenceBeatTriggersRef` (finding 5) and `extensionFetchAbortRef` (finding 4).
- **L1290 area, L1299–1355** (identity-bootstrap effect, finding 3) — replaced the fixed `setTimeout`/`Promise` wait with `waitForEvent` raced via `AbortSignal.any([identityWaitAbort.signal, AbortSignal.timeout(2000)])`; added `identityWaitAbort` `AbortController`, aborted in the effect's own cleanup alongside the pre-existing `cancelled` flag.
- **L1702, L1780, L1861** (finding 4) — added `signal: extensionFetchAbortRef.current.signal` to the three `/extensions/install` fetch calls (`installExtension`, `installExtensionFromFile`, `uninstallExtension`).
- **L2144–2146** (+3 lines) — the unmount-teardown effect now calls `extensionFetchAbortRef.current.abort()` first, before its pre-existing plugin-instance teardown.
- **L2200–2214** (finding 2, whole effect rewritten in place) — the registry-streaming effect now runs a bounded worker-pool queue (`pending`/`activeWorkers`/`pump()`, limit = `pluginInstallConcurrency()`) instead of unbounded `void installPlugin(...)` per event; `aborted` flag set in the effect's cleanup stops handing out new work (in-flight installs still settle).
- **L2863–2891** (finding 1) — the `invokeExtension` branch now dispatches `runInvokeExtensionEffect(...)` through `serializePerActor(actorKey, ...)` without `await`ing it in the loop; a `.catch()` at the dispatch site logs any residual rejection instead of it becoming an unhandled rejection.
- **L3948–3982** (finding 5, whole effect rewritten in place) — the presence-beat effect now keys a `latestWins`-wrapped trigger per open `documentId` (cached in `presenceBeatTriggersRef`, pruned each tick for closed documents) instead of one shell-wide sequential loop guarded by a single `publishing` flag.

## effect-batch ordering evidence

Real `serializePerActor` (imported from `PluginRuntime/🟦️component.tsx`, not a copy) exercised directly in the scratch suite (`terra-web-shellhost-scratch-verify.txt`, tests under "serializePerActor dispatch for invokeExtension"):

- Three calls submitted **without awaiting between them** — `serializePerActor("actor-a", a1)`, `serializePerActor("actor-b", b1)`, `serializePerActor("actor-a", a2)` — exactly how `applyHostEffects`'s loop now calls it (fire, `continue`, never block on the result).
- Observed order: `a1` and `b1` (different actors) both **start** before either settles — proves a slow extension call for one actor no longer blocks the loop from moving on to effects for a different actor.
- `a2` (same actor as `a1`) does **not** start until `a1` settles, even though it was submitted synchronously right after `a1` with no await in between — proves per-actor ordering survives the switch from inline `await` to fire-and-dispatch.
- A rejected run (`serializePerActor("actor-err", async () => { throw new Error("boom") })`) surfaces to the caller's own `.catch()` with the real error — proves failures are reported, not swallowed, matching `applyHostEffects`'s own `.catch(console.error)` at the real call site.

Within one `effects` array, every non-`invokeExtension` branch is untouched and still runs fully sequentially in the `for...of` loop exactly as before — only the `invokeExtension` branch's own extension call + completion was pulled out of the awaited path.

## baseline vs after (named sets)

**I did not measure a fresh baseline before editing** — see `## honest gaps`. The best available baseline is this ticket's own prior recorded run, `📓️terra-H1-vitest-final.txt` (`@semio-tech/framework-renderer-react`, 321 passed / 15 failed / 336), from the packet that last touched this same file for the actor-runtime rewrite.

After this packet's edits, same package, same command: **325 passed / 11 failed / 336** (see `## commands + exit codes`). Named comparison:

| baseline (15, H1) | still failing after (11) |
|---|---|
| renders selectable builder cards with selection ring | ✓ |
| interprets virtual file system component scenes | ✓ |
| isolates render faults in ShellFaultBoundary | ✓ |
| stages both args locally, dispatches nothing until Execute… | ✓ |
| gates Execute on required args… | ✓ |
| Reset restores defaults while keeping the form expanded | ✓ |
| resolveWindowActions surfaces only panel-eligible… | ✓ |
| commandCategories orders and dedupes categories by first appearance | ✓ |
| ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND introduction is app-specific… | ✓ |
| mit-bestand/demonstrator footer credits render… | ✓ |
| buildCommandCategoryTabs builds one namespaced PanelTabLeaf… | ✓ |
| auto-expands a singleton arg-carrying category… | now passes |
| an arg-carrying command row toggles expansion… | now passes |
| Execute is disabled until the required arg is staged… | now passes |
| FrameworkOsShell portal layer is unconstrained by z-tutorial… | now passes |

**The 11 still failing after this packet are an exact subset of the 15 named in the H1 baseline — zero new failure names.** The 4 that now pass are all in unrelated areas (window-action-panel arg staging, tutorial-portal z-index) with no plausible connection to any of the 5 findings; most likely another packet's fix landed in the shared tree between H1's run and this one, or the earlier run was flaky. Not claimed as this packet's doing. The one "Unhandled Rejection" (`postMessage requires 2 arguments`, jsdom quirk in `🟦️backbone-worker.ts`) is present **identically** in both the H1 baseline transcript and this run — confirmed pre-existing, unrelated to any of the 5 findings.

## commands + exit codes

**Scratch algorithm verification** (see `## honest gaps` for why this exists instead of a real repo test) — `terra-web-shellhost-scratch-verify.txt` in this folder:
```
NODE_PATH="/Users/ueli/Documents/semio/node_modules" ./node_modules/.bin/vitest run \
  --config "<scratchpad>/terra-web-shellhost-vitest.config.ts" --reporter=verbose
```
Result: **9 passed (9)**, all named individually (bounded install queue ×2, identity wait ×3, per-document latestWins ×2, serializePerActor dispatch ×2). `EXIT:0`.

**Real package suite** — `terra-web-shellhost-after1.txt` in this folder:
```
bun nx run @semio-tech/framework-renderer-react:test -- --reporter=verbose
```
Result: **325 passed | 11 failed (336)**. `EXIT:1` (nx surfaces the failed-task exit as 1; underlying vitest also reported `Tests 11 failed | 325 passed (336)`).

**Repo-wide tsc** — `terra-web-shellhost-tsc-full.txt` in this folder:
```
./node_modules/.bin/tsc --noEmit -p tsconfig.json
```
Result: `EXIT:2`, 8530 total `error TS` lines repo-wide (far more than the "~19 pre-existing" figure `📌️important.md` records — that figure comes from a differently-scoped invocation I could not reproduce; see `## honest gaps`). Cross-checked: **zero** of the 8530 lines mention any identifier I added (`waitForEvent`, `latestWins`, `serializePerActor`, `pluginInstallConcurrency`, `runInvokeExtensionEffect`, `extensionFetchAbortRef`, `presenceBeatTriggersRef`, `identityWaitAbort`), and **zero** of ShellHost's own ~90 reported error line numbers fall inside any range I edited (verified by diffing the reported line numbers against the `git diff` hunk list above — the two closest, 2865–2867, are the pre-existing `loadDocument` branch immediately above my `invokeExtension` edit, untouched).

**Syntax sanity** — `esbuild "ShellHost/🟦️component.tsx" --loader:.tsx=tsx --bundle=false --outfile=/dev/null` → `272.2kb` written, `EXIT:0`. Confirms the file parses/transforms cleanly.

## lease-requests

1. **Export `poolConcurrency` from `PluginRuntime/🟦️component.tsx`** (currently a private `function poolConcurrency(): number` at that file's line ~193). ShellHost's new `pluginInstallConcurrency()` (this file, `//#region 🧵️ConcurrencyHelpers`) duplicates that exact formula rather than importing it, specifically because I have no write access to `PluginRuntime`. Once exported, delete ShellHost's local copy and import the real one.
2. **(Lower priority) Add real repo test coverage for this packet's three ShellHost-internal fixes** (bounded install queue + abort, identity `waitForEvent` race, per-document `latestWins` presence beat) by either (a) exporting `pluginInstallConcurrency`/`runInvokeExtensionEffect` from ShellHost and wiring them through `📦️index.tsx` into `🧪️index.test.ts`, or (b) granting this packet (or a follow-up) write access to those two files plus `🧪️vitest.config.ts` to add the tests directly. See `## honest gaps` for what's covered today instead.

## honest gaps

- **Baseline was not measured before editing.** I should have run `@semio-tech/framework-renderer-react`'s suite before touching the file and did not — I only discovered this ticket's own prior recorded baseline (`📓️terra-H1-vitest-final.txt`, 321/336 with 15 names) after the fact and used it as the reference point instead. The comparison in `## baseline vs after` is sound (named-set, not count, per this ticket's own rule), but it rests on a baseline I did not personally capture pre-edit.
- **No real, wired-in repo test for three of the five fixes.** `ShellHost/🟦️component.tsx` has no test file of its own; the package's one shared `🧪️index.test.ts` only sees symbols re-exported through `📦️index.tsx`, and that package's `🧪️vitest.config.ts` sets no `includeSource`, so an `import.meta.vitest` block added to ShellHost would not run even if I added one (the exact "new test file silently doesn't run" trap `📌️important.md` warns about, one layer removed — here it's "new in-source block", not "new file"). Both files are outside this packet's lease. Instead: (a) the bounded-install-queue, identity-wait, and per-document-latestWins logic is verified against a **local reimplementation of ShellHost's exact inline code**, run under a throwaway vitest config in the session scratchpad (not the ticket folder, since this packet's scratch-file rule is `.txt`/`.md`/`.json` only) — real proof the *algorithm* is correct, not proof of the *wiring*; (b) the `invokeExtension`/`serializePerActor` dispatch (finding 1) IS tested against the real, already-exported `serializePerActor`, so that one has genuine wiring-level coverage; (c) the real package suite (`325/336`, exact named subset of the prior baseline, zero new failures) is the actual integration-level regression signal for all five fixes together, since `FrameworkOsShell` is exercised by that suite (one full-mount smoke test plus everything reachable through `📦️index.tsx`'s re-exports).
- **Could not reproduce the "~19 pre-existing" `tsc --noEmit` figure `📌️important.md` records.** My `tsc --noEmit -p tsconfig.json` run produced 8530 errors repo-wide (`EXIT:2`) — almost certainly a differently-scoped/aliased invocation than whatever produces the clean 19-error baseline (this repo's packages each carry their own path-aliased `tsconfig`/`vitest.config`, and the bare root config lacks `allowImportingTsExtensions`, producing thousands of `TS5097` false positives on top of real pre-existing drift). I verified by cross-referencing line numbers and identifiers instead of by matching the "19" figure — see `## commands + exit codes`.
- **No fake-timer test for the real 2s `AbortSignal.timeout` elapsing.** Empirically confirmed (via a throwaway probe, not kept) that this vitest version's `vi.useFakeTimers()`/`vi.advanceTimersByTime()` do **not** intercept Node's internal `AbortSignal.timeout` (it advanced 2000ms of fake time with the signal still unaborted). The scratch suite instead proves the timeout-elapsed *outcome* (falls back to `null`, matching the pre-existing behaviour) by triggering the SAME code path's `AbortController.abort()` directly rather than waiting out a real 2s delay — a `AbortSignal.any` race member, not the timeout member specifically, but the two are equivalent from `waitForEvent`'s perspective (either member firing rejects the promise the same way).
- **Did not mount `FrameworkOsShell` with a full wasm/worker/fetch harness** to exercise all five fixes together end-to-end in one integration test — this package's own precedent for that is exactly one bare smoke-mount test (`plugins: []`, no plugin loading at all); building the mocking depth this would need (fake `ShardClient`/`Worker`/`fetch`/wasm handles) was out of proportion to this packet's scope and time budget.
