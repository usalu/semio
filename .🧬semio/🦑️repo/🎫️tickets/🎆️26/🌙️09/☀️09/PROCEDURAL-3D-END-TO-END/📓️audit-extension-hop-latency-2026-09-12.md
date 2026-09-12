# Extension Hop Latency Audit — where the ~24 s/hop goes

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, read-only audit, 2026-09-12. Repo MCP unavailable
(`invalid initialize params`); no ticket state touched, nothing edited, no build/dev-server started.
Source probe: `🗑️generated/probe-restage-4-long/console.txt` + `hosts.json` (112.7 s convergence, both
windows `ok`/`idle`, `procedural-preview` meshes=3). A concurrent lane ("extension-invoke-door") is
editing `🔌️PluginRuntime/🟦️.tsx`; §5 records exactly what was on disk when read.

## TL;DR

The ~24 s/hop is **not** guest computation, **not** wasm re-instantiation per hop, and **not** a
lane-priority starvation. Every hop's `invoke()` round trip itself resolves in **1 guest turn**
(`extension request answered … turns: 1`). The dead time sits entirely in silent, zero-`[DEBUG]`-output
gaps between a `flowEvalTick` command's dispatch and its settle. The only mechanism in this code path
that can produce a silent multi-second gap with no logging is a **real, chained `setTimeout`**:
`scheduleDispatchAction` (`🛠️ShellHelpers/🟦️.tsx:766-775`) is the host's ONLY handler for the guest's
`Effect::DispatchAction` "run me again" request, and its default `schedule` is
`(fn, ms) => setTimeout(fn, ms)` (line 771). Every `rearm()`/`flow-eval-resolve` implementation in the
flow/procedural/generation2d/3d command set requests `delay_ms: 0` — never a backoff (confirmed by
grep across every call site, §2). On a headless, unfocused Playwright page (`console-dump-probe.mjs`
does `chromium.launch({headless:true})` + `browser.newPage()`, never `bringToFront`/focuses it —
`🐍️console-dump-probe.mjs:10-11`), Chrome's background-renderer timer floor clamps a nominally-0ms
`setTimeout` to roughly a second, and this file's own neighbor function documents the exact hazard:
`yieldPluginUiContinuation` (`🔌️PluginRuntime/🟦️.tsx:1418-1429`) was rewritten onto an unthrottled
`MessageChannel` specifically **because** "a `setTimeout(0)` chain is throttled to one tick per second
in a hidden tab" (its own doc comment, `🔌️PluginRuntime/🟦️.tsx:2098-2100`). `scheduleDispatchAction`
never got the same treatment. This is the best-supported explanation from static reading + the
measured timeline below; it is not confirmed by a source-level timestamp bracket (adding one would
require editing source, out of scope for a read-only audit) — §4 names the one-line probe that would
close that gap.

---

## 1. Per-hop timeline (from `probe-restage-4-long/console.txt`, ms since navigation)

The code logs exactly two extension-wire events per hop, both from the SAME line
(`captureExtensionCompletion().complete`, `🔌️PluginRuntime/🟦️.tsx:2583`, and `invoke()`,
`:2663`) at the SAME instant — there is no separate log for "guest emits `Effect::InvokeExtension`",
"host dispatches it", or "actor receives the `request` event": `driveInboundRequest`
(`🖼️wire-turn.ts:282-296`) submits the `request` event and polls silently until a matching `respond`
effect appears, with no intermediate `[DEBUG]` line. So the finest per-hop resolution the EXISTING
logs give is: (a) when the enclosing `flowEvalTick` command is dispatched/settles, and (b) the instant
`invoke()`/`complete()` resolves. Below, "silent gap" = elapsed ms with **zero** console lines.

| hop | capability | `flowEvalTick` dispatched | settled | extension answered / completion submitted | silent gap(s) inside the hop |
|---|---|---|---|---|---|
| install | — | `setContributions deferred effects`, 2×`dispatchAction{flowEvalTick, delayMs:0}` @ **6598** | first settle @ **23704** | — | **17 106 ms**, zero lines, between the `delayMs:0` dispatch and its first observed settle |
| 1 | `flow-extension-math` evaluate | (from install re-arm) | settled 23811, 23939, 24011 | **24261** (req 1, turns 1, 194→199 B) | included above |
| 2 | `flow-extension-brep` evaluate | 38508 (seq 16) | 49077 / 49215 (effects:1) | **49739** (req 1→2, turns 1, 121→122 B) | 24261→37572 = **13 311 ms** silent, then 38508→49077 = **10 569 ms** to settle |
| 3 | `flow-extension-brep` evaluate | 61706 (seq 20) | 73653 / 73733 (effects:1) | **73855** (req 2→3, turns 1, 123→124 B) | 49739→61706 = **11 967 ms** (interleaved with job/select noise), then 61706→73653 = **11 947 ms** silent |
| 4 | `flow-extension-brep` tessellate | 85734 (seq 25) | 96667 / 96949 (effects:2) | **97003** (req 3→4, turns 1, 363→393 B) | 73855→85452 = **11 597 ms** silent, then 85734→96667 = **10 933 ms** silent |
| 5 | `flow-extension-brep` tessellate | 110539 (seq 26) | 112564 / 112660 (effects:1) | **108422**\* / **112684** (req 4→5, 5→6, 116→131 B, 2236→2265 B) | 97003→104553 = **7 550 ms**, 104553→108422 = **3 869 ms**; then 110539→112564 = **2 025 ms** |

\* Hop 5 (req 4, 116 B) fires at 108422 without a preceding `performInvocation` line at all — its
`flowEvalTick` must be the SAME settle that's still draining from hop 4's tick (effects carried a
second `InvokeExtension`), i.e. two extension calls landed inside adjoining ticks once the chain was
close to converging. Hop 6 (req 5→6, 2236→2265 B, the FINAL tessellate that actually paints the mesh)
lands at 112684, 2025 ms after the last logged dispatch, with only ordinary render noise (`dag draw`,
`brushPreview.assemble`) in between — no silent gap at all.

**Which gap holds the ~24 s:** two back-to-back silent windows of ~11-13 s each, with **zero**
`[DEBUG]` output in either, bracketing every hop through #4. Each window starts right where a
`flowEvalTick`/`dispatchAction` effect with `delay_ms: 0` would have to re-enter the host
(`applyHostEffects`'s `"dispatchAction" in effect` branch, `🏛️ShellHost/🟦️.tsx:5153-5163`, which calls
`scheduleDispatchAction` at `:5164`) and ends the instant the next `performInvocation`/settle appears —
consistent with a throttled `setTimeout(0)` sitting silently in between, not with guest computation
(the native equivalent chain is 0.58 s wall clock per `📓️tick-arming-latch-2026-09-12.md` §3).

---

## 2. Is there a fixed timer/interval? Per-hop wasm re-instantiation? Lane starvation?

**A fixed timer exists, and it is the prime suspect — `scheduleDispatchAction`'s `setTimeout`, not a
polling interval.** Evidence trail:

- Every guest re-arm requests `delay_ms: 0`, never a backoff — checked across every call site:
  `✏️s/…/🌊️flow/…/🎮️commands/{✅️flow-eval-resolve,🧮️evaluate,⏱️flow-eval-tick}/🦀️.rs:19` and
  `✏️s/…/🌀️procedural/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs:79-80` (`pub fn rearm(...)`, `delay_ms: 0`)
  and its `⏱️flow-eval-tick/🦀️.rs:16-19` twin, and the `generation2d` mirror
  (`…/🌀️generation2d/…/⏱️flow-eval-tick/🦀️.rs:18-19`, `…/✅️flow-eval-resolve/🦀️.rs:23`). The design
  doc for the native wgpu target says as much directly: "natively, any `delay_ms` collapses to 'next
  tick' (no timer wheel…)" (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4682`) — i.e. `delay_ms:0` is meant to mean
  "immediately", and does, natively.
- On the web target, "immediately" is not free: `scheduleDispatchAction`
  (`🛠️ShellHelpers/🟦️.tsx:766-775`) is `schedule(() => { void dispatchOne(action, args)… }, delayMs)`
  with `schedule` defaulting to `(fn, ms) => setTimeout(fn, ms)` (line 771) — a genuine macrotask timer,
  called fresh every time `applyHostEffects` sees a `dispatchAction` effect
  (`🏛️ShellHost/🟦️.tsx:5153-5164`), including from INSIDE the callback of a PREVIOUS such dispatch (the
  guest's own comment above that call site: "so a plugin can chain several ticks of staged/progressive
  work … purely by re-emitting `dispatchAction` from its own handler", `🏛️ShellHost/🟦️.tsx:5155-5158`).
  That is a textbook nested-`setTimeout(0)` chain.
- The SAME FILE already diagnosed and fixed this exact hazard once, for a different loop:
  `yieldPluginUiContinuation` (`🔌️PluginRuntime/🟦️.tsx:1418-1429`) uses an unthrottled `MessageChannel`
  specifically because, per its own doc (`:2098-2100`), "a `setTimeout(0)` chain is throttled to one
  tick per second in a hidden tab (once per minute under Chrome's intensive throttling)". The probe
  (`🐍️console-dump-probe.mjs:10-11`) runs `chromium.launch({headless:true})` +
  `browser.newPage(...)` and never focuses/foregrounds the page — exactly the un-focused/backgrounded
  renderer class Chrome throttles.
- Corroborating measurement: the install's `delayMs:0` dispatch at 6598 ms does not produce its first
  observed settle until 23704 ms — a **17 106 ms** silent gap. `📓️tick-arming-latch-2026-09-12.md` §1
  independently measured, PRE-fix, "first settled tick after the install: **25.5 s** (**17 s** after
  install)" and predicted post-fix it would land "within ~1-2 s of the install" (§4 table). This probe
  is POST-fix (only ~7 `flowEvalTick` invocations total, matching that lane's own "≤16" prediction —
  the redundant-tick bug is genuinely gone) and STILL shows the same ~17 s gap. That means the
  tick-arming-latch fix removed the duplicate ticks but never touched whatever produces this fixed
  ~11-17 s floor per tick — consistent with a host-side timer the guest-side latch has no visibility
  into, and inconsistent with "still too many ticks" (there weren't).

**Per-hop wasm re-instantiation: ruled out.** `ensureRequestActor` (`🔌️PluginRuntime/🟦️.tsx:2043-2050`)
memoizes `requestActor` as a promise per plugin handle (`requestActor ??= (...)`) — activated once,
lazily, and reused for every subsequent `invoke()` call to that SAME plugin. Hops 3 and 4 both call
`flow-extension-brep`, reusing the actor hop 2 already activated, yet cost the same ~24 s and ~23 s as
hops 1-2 — if instantiation were the cost, hop 3/4 would be near-instant. They are not, which is direct
evidence against the instantiation hypothesis and additional evidence for a per-tick, not per-actor,
timer.

**Lane/queue starvation: not the dominant cause, but a real secondary risk worth fixing anyway.**
`serializeCommandIngressForActor` (`🔌️PluginRuntime/🟦️.tsx:1143-1146`) keys its mutex as
`command-ingress:${actorId}` via `serializePerActor` (`:1129-1139`), which enqueues onto the shared
`TurnScheduler` (`🎭️actor/📦️packages/🟦️typescript/🟦️.ts:71`) by `lane`. `drainTypedOperations`'s own
continuation-drain call (`🔌️PluginRuntime/🟦️.tsx:2381`) and `captureExtensionCompletion().complete`
(`:2584`) both go through this SAME per-actor key with **no explicit lane argument**, i.e. both default
to `"Interactive"` — so they are not actually racing across lanes here, they share one FIFO. That
FIFO's own pump is a **microtask** (`TurnScheduler`'s header, `🟦️.ts:8-10`: "schedules a microtask
pump… never dispatches on `enqueue`"), which cannot itself explain a multi-second gap — this rules out
the generic scheduler as the timer source. The risk that DOES remain: nothing here prevents an
`"Interactive"`-lane extension completion from queuing behind an unrelated `"UserVisible"`/`"Background"`
turn on the same actor if one is ever in flight, because `serializePerActor` enqueues by lane label
without lane-based reordering within one actor's queue (§4, fix 2).

---

## 3. Why hops 5 and 6 are fast (11 s, then 4 s)

Not a warm-vs-cold extension difference — hop 5 and 6 both call the ALREADY-warm `flow-extension-brep`
actor, same as hops 3 and 4, which were slow. The difference is **how many `dispatchAction{delay_ms:0}`
re-entries the chain still needs**, which `📓️tick-arming-latch-2026-09-12.md` §4 already modelled:
"a converging chain is 5 ticks and 6 extension round trips per preview window" — front-loaded, tapering
as fewer flow nodes remain in `remaining`/`more_work`. The timeline supports this directly: hops 1-4
each sit behind **two** ~11-13 s silent windows (two throttled re-entries per hop — one to notice
pending extension work and emit the invocation, one after the completion settles to notice there is
still more graph to walk and re-arm again). Hop 5 sits behind **one** ~11.4 s window split into two
shorter pieces (7.55 s + 3.87 s — the same floor, but overlapping with node-graph/DAG redraw work that
already had a `[DEBUG]` line, so it reads as "shorter" rather than "absent"). Hop 6 — the terminal
tessellate that finally paints the mesh — needs **no** further re-arm afterward (the chain has nothing
left to walk), so it costs only the ordinary settle time (2.0 s) with no throttled gap at all. Payload
size (116 B → 2265 B) and capability (evaluate vs tessellate) track the SAME shrinking-remaining-work
curve and are not themselves causal — every hop's actual extension answer costs `turns: 1` regardless
of size or capability.

---

## 4. Ranked fixes

1. **(Highest expected win, ~10-20 s/hop) Stop routing zero-delay `dispatchAction` re-arms through a
   real `setTimeout`.** `scheduleDispatchAction` (`🛠️ShellHelpers/🟦️.tsx:766-775`) should schedule a
   `delayMs === 0` re-arm on the SAME unthrottled `MessageChannel` primitive
   `yieldPluginUiContinuation` already proves out (`🔌️PluginRuntime/🟦️.tsx:1418-1429`), falling back to
   real `setTimeout` only for a genuinely positive `delayMs` (none of the current call sites ever pass
   one — every `rearm()`/`flow-eval-resolve` in `🌊️flow`, `🌀️generation2d`, `🌀️procedural` hard-codes
   `delay_ms: 0`, §2). This is a same-file, same-pattern fix; the team already wrote and justified the
   primitive it needs. **Test**: extend `scheduleDispatchAction`'s existing unit coverage (it already
   takes an injectable `schedule` for tests) with a case asserting a `delayMs:0` call does NOT invoke a
   provided `setTimeout` stub; add a live-probe assertion (`console-dump-probe.mjs`, same URL) that the
   full 6-hop chain settles in under 3 s wall clock, matching this ticket's own target and the native
   0.58 s baseline (`📓️tick-arming-latch-2026-09-12.md` §3).
2. **(Defensive, not currently the dominant cost) Make the per-actor command-ingress queue
   lane-aware, not just lane-labelled.** `serializePerActor`
   (`🔌️PluginRuntime/🟦️.tsx:1129-1139`)/`TurnScheduler` should let an `"Interactive"` turn (a real
   command, an extension `respond`/`completed`) jump ahead of a queued `"UserVisible"`/`"Background"`
   turn on the SAME actor, so a future long `drainTypedOperations` continuation run can never sit in
   front of a user-visible extension completion. **Test**: a `TurnScheduler` unit test enqueuing
   `Background` then `Interactive` work for one actor id and asserting execution order is
   Interactive-first (mirrors the "lane priority could never out-rank an already-in-flight FIFO head"
   guarantee the class's own header already documents, `🎭️actor/📦️packages/🟦️typescript/🟦️.ts:8-10` —
   extend it to say "…except across lanes for work not yet started").
3. **(Verification, not a code change) Re-measure the tick/round-trip count after fix 1 lands.**
   `📓️tick-arming-latch-2026-09-12.md` §4 predicted "≤16" `flowEvalTick` invocations for a converging
   example; this probe saw ~7 for a simpler one, consistent. Confirm that count doesn't creep back up
   once the timer floor is removed (a real regression would now be visible as invocation COUNT again,
   not wall clock, so the existing fixture `🔒️tick-latch.json` already guards it).
4. **(Immediate next validation step, no source edit)** Run `console-dump-probe.mjs` once more,
   identical env vars, and diff the resulting per-hop gap sizes against this run's 11-13 s figures — if
   they are noisy/variable across runs that is further evidence for a browser-scheduler-level throttle
   (vs. a fixed constant), which is what fix 1 targets; if they are bit-for-bit stable, look for a
   literal constant instead. Not run here to keep this audit read-only and single-pass; the existing
   console lines already localize the gap to the same two call sites in both hypotheses.

---

## 5. The concurrent `invoke`-door lane — what is on disk right now

The door `📓️extension-request-locale-2026-09-12.md` §5 described as "not attempted here… the next
package" is **landed and working** as of this read:

- `PluginWasmHandle.invoke(capability, request, context?)` — `🔌️PluginRuntime/🟦️.tsx:2630-2699`,
  returned from `loadPluginModule` (`:2699`, `return { ...richHandle, refreshUi,
  captureExtensionCompletion, invoke, bindDocumentPort }`). It submits one `Event::Request` on a
  lazily-activated, memoized per-plugin request actor (`ensureRequestActor`, `:2043-2050`) and drains it
  via `driveInboundRequest` (`🖼️wire-turn.ts:282-296`, budget `INBOUND_REQUEST_TURN_BUDGET = 64`,
  `wire-turn.ts:241`), decoding a `fault` arm into a `SemioFaultError` and enforcing both the
  `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` (request) and `GUEST_HOST_ANSWER_CEILING_BYTES` (answer)
  bounds the realloc-abort lane already declared (`📓️extension-result-realloc-2026-09-10.md` §4.1).
- `wireEffectToFriendly` routes `"respond"` (`🖼️wire-turn.ts:212-219`, `wireRespondAnswer`), so the
  effect is no longer dropped as unmapped.
- This is proven live by THIS probe, not just by a unit test: 6/6 extension round trips answered `ok`
  (`req 1..6`, `status: ok`), `window:procedural-preview` converges to `meshes: 3`, and both surfaces
  report `status: "ok"` in `hosts.json`. `extension.invoke-unavailable` — the fault that produced every
  prior boot's `meshes: 0` — never appears in this console.
- One inconsistency worth flagging to whoever owns ticket bookkeeping: §5 of the locale doc frames this
  door as unbuilt ("Not attempted here"), but it is fully present and exercised in the currently-served
  build. Either that doc is stale relative to the invoke-door lane's landed work, or this probe ran
  against a wasm build newer than that doc's own restage — worth reconciling in `📓️status.md` rather
  than re-describing the door as missing.

Nothing in this door's code path (per-turn cost `turns: 1`, memoized actor, bounded pages) contributes
to the ~24 s/hop latency; §1-§4 above localize that entirely to the `dispatchAction` re-arm timer
sitting OUTSIDE this door, between hops.

---

## Files read (no edits made)

- `🔌️PluginRuntime/🟦️.tsx` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`)
- `🛠️ShellHelpers/🟦️.tsx`, `🏛️ShellHost/🟦️.tsx` (same `🧱️elements` directory)
- `🖼️wire-turn.ts` (`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts`)
- `🟦️.ts` (`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts`, `TurnScheduler`)
- `🟦️.ts` (`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`, `SHARD_LIVENESS_POLICY`)
- `⏱️flow-eval-tick/🦀️.rs`, `🧮️evaluate/🦀️.rs`, `✅️flow-eval-resolve/🦀️.rs` under
  `✏️s/🔌️plugins/🌊️flow/…` and `✏️s/🔌️plugins/🌀️procedural/…/🧊️generation3d/…` and
  `…/🌀️generation2d/…`; `🧵️preview-eval/🦀️.rs` (generation3d)
- `🗑️generated/probe-restage-4-long/console.txt`, `hosts.json` (read only)
- `📓️extension-request-locale-2026-09-12.md`, `📓️tick-arming-latch-2026-09-12.md`,
  `📓️extension-result-realloc-2026-09-10.md` (this ticket)
- `project-wasm-pool-pump-starves-interactive-jobs.md` (memory) — its cooperative-maintenance-cadence
  mechanism was checked and ruled out as the source of this latency (that gap is about wasm
  `WorkerPool::pump` cadence for RETAINED typed operations, a different code path from the
  `dispatchAction`/`scheduleDispatchAction` re-arm this audit localizes the cost to).
