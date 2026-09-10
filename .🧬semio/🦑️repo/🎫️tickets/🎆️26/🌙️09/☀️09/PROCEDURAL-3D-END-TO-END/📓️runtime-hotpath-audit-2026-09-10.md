# Runtime hot-path audit — procedural 3d, 2026-09-10

Read-only audit against the current working tree (HEAD 9b605a4550 + uncommitted edits from the
extension-round-trip / eval-continuation / node-graph-attach lanes; see `git status` for the file
list). Method: `rg`/`grep` over the named taxonomy paths, `sed -n` reads of the matched sites, and
`git log --date=iso -S'<marker>'` for attribution. No cargo build, no dev server, no browser was run.

---

## 1. `[DEBUG]` emission inventory

### 1.1 Rust hot path (`eprintln!`)

| # | file:line | marker (abridged) | frequency | added by | verdict |
|---|---|---|---|---|---|
| R1 | `🔌️plugin/🦀️.rs:29464` | `maintenance stage=<n> elapsed_us=<n> outcome=<..>` | every `maintenance_step` call whose ONE stage takes ≥2ms (`RuntimeLiveCleanupJob::step`, called once per reactor turn's cooperative-maintenance pump) | committed 9b605a4550 (2026-09-09 12:10, this ticket, boot-path/rebuild wave) | **gate behind a debug flag** — real signal (stage cost) but floods once any stage crosses 2ms, which stage 22 does routinely here |
| R2 | `🔌️plugin/🦀️.rs:29546` | `cooperative maintenance callback overran the interactive ceiling … (elapsed <n>us) — recorded, not fatal` | every maintenance turn whose WALL time (incl. OS descheduling) ≥ `INTERACTIVE_STEP_CEILING_US` (8 000us) | committed ebbace9b32 (2026-09-09 17:57, this ticket) | **keep, but gate** — it is the perf signal the goal cares about, but at flood volume it drowns the fault-path lines around it |
| R3 | `⚛️reactor/🔄️turn/🦀️.rs:287` | `Event::Completed req=<n> ok=<bool>` | **every** `Event::Completed` delivered to the reactor — once per extension round-trip hop 5, i.e. once per evaluate/tessellate result per node per tick | uncommitted (`git status`: `M`), doc-labelled "extension round-trip hop 5", this ticket | **remove** — doc-labelled temporary (`🐞️ … — temporary, ticket 26/09/09/PROCEDURAL-3D-END-TO-END`), unconditional, now stale (round trip is verified, `📓️eval-continuation-runtime-2026-09-10.md` §3) |
| R4 | `⚛️reactor/🔄️turn/🦀️.rs:290` | `continuation resolve hit req=<n> instance=<i> action=<a>` | same cadence as R3 (hop 6a) | uncommitted, this ticket | **remove**, same reason |
| R5 | `⚛️reactor/🔄️turn/🦀️.rs:306` | `continuation resolve dropped req=<n> — owning instance has no acknowledged live lifetime` | fires only when the owning instance died mid-flight | uncommitted, this ticket | **keep** — genuine fault-adjacent diagnostic, low frequency |
| R6 | `⚛️reactor/🔄️turn/🦀️.rs:308` | `continuation resolve miss req=<n> — routed to the parked-future registry` | hop 6c, same cadence as R3 when a hand-minted req is used | uncommitted, this ticket | **remove/downgrade** — doc calls this the regression signature; worth one counter-based log, not per-hit |
| R7 | `⚛️reactor/🔄️turn/🦀️.rs:856` | `reactor more-work streak=<n> seen=<n> executor_deadline=… command_ingress=…` | guarded: only once `more_work` has been true for ≥2048 consecutive turns AND (`streak` a power of two OR `streak % 4096 == 0`) | committed 599a5d8450 (2026-09-09 07:58) | **keep** — already throttled to ~1 line per doubling; this is exactly the "the reactor never idles" signal boot #5 needs |
| R8 | `⚛️reactor/🔄️turn/🦀️.rs:864` | `reactor more-work streak ended after <n> turns (seen=<n>)` | once, when a long streak ends | committed 599a5d8450 | **keep** |
| R9 | `⚛️reactor/🦀️.rs:716` | `extension invocation minted instance=<i> extension=<id> capability=<c> responseAction=<a> req=<n>` | hop 2 — once per `queue_extension_invocation`, i.e. once per evaluate/tessellate dispatch per node per tick | uncommitted, doc-labelled "extension round-trip hop 2", this ticket | **remove** — same stale-instrumentation class as R3/R4/R6 |
| R10 | `🔌️plugin/🦀️.rs:30225` | `cooperative-maintenance instance=<i> turn=<n> generation=<g> status=<a>-><b> entries=<n> phase=<..> clock=<bool> pool=<..>` | guarded: `turn <= 4096 && turn.is_power_of_two()` (~13 lines per instance lifetime, then silent) | pre-existing (not this ticket; not touched by `git log -S` inside this session's diff) | **keep** — self-limiting, cheap |
| R11 | `🔌️plugin/🦀️.rs:30060` | `native close deadline instance=<i> …` | `#[cfg(test)]`-gated — **does not compile into the served wasm or native binary** | n/a | **no action** — not actually on the runtime hot path, despite living in the same file |
| R12 | `🔌️plugin/🦀️.rs:19761` | `envelope decode drive <count> step …` | guarded `count % 2048 == 0`; inside a `#[cfg(test)] mod` fixture-loop (confirmed: enclosing `#[cfg(test)]` at line 19041, `for _ in 0..100_000 { … case[...] }` pattern) | test-only | **no action** |
| R13 | `🔌️plugin/🦀️.rs:17025`, `🧵️retained-command/🦀️.rs:697` | `registered keyed dispatch progress …`, `retained-command raw allocation …` | inside `#[cfg(test)]` fixture-loop test bodies | test-only | **no action** |
| R14 | `🏪️store/🦀️.rs:8010` | `history-expected-record path <p> kind <k> terminal <t> start <n>` | fault path only — fires when a decode token is malformed | pre-existing | **keep** — genuine diagnostic, rare |
| R15 | `🏪️store/🦀️.rs:8394` | `envelope field decode diagnostic <code> at offset <n> path <p>` | fault path only — fires on a decode diagnostic error | pre-existing | **keep** |
| R16 | `…/generation3d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:31` | `flowEvalTick invoke extension=<id> operatorId=<op> nodeHash=<h>` | hop 1 — once per node per eval tick that reaches an extension operator; the tick self-redispatches until `session.tick()` settles, so this fires repeatedly per node until resolved | uncommitted, doc-labelled hop 1, this ticket | **remove** (or gate) — plugin-level instrumentation added for the same round-trip debugging pass |
| R17 | `…/generation3d/…/✏️editor/🦀️.rs:2207,2234` | `flowTessellate invoke …` / `flowTessellate skipped: no plugin contributes 'brep'` | hop 1b — once per preview handle per tessellate dispatch | uncommitted, doc-labelled hop 1b, this ticket | **remove** (2207 fault-shaped, could stay; 2234 is per-tick chatter) |
| R18 | `…/generation3d/…/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs:18` | `flowEvalResolve nodeHash=<h> outputBytes=<n> seeded=<bool>` | hop 7 — once per node per resolved evaluate | uncommitted, doc-labelled hop 7, this ticket | **remove** |

### 1.2 TypeScript hot path (`console.*`)

The task brief describes `[DEBUG] thunk start/done command-ingress` and `[DEBUG] command ingress
settled` lines. **These could not be located verbatim** in `🔌️PluginRuntime/🟦️.tsx`,
`🏛️ShellHost/🟦️.tsx`, `🛠️ShellHelpers/🟦️.tsx`, or `🧵️backbone-worker.ts` as of this read —
`🔌️PluginRuntime/🟦️.tsx` currently has **zero** `console.*` calls at all (grep confirmed, 2997
lines). `📓️eval-continuation-runtime-2026-09-10.md` §3/§6 documents a hop 4 line it added at
`🔌️PluginRuntime/🟦️.tsx:2067` (`[DEBUG] extension completion submitted {instanceId, req, status}`)
and a hop 3 line at `🏛️ShellHost/🟦️.tsx:1616` — hop 3 is still present (see below), hop 4 is gone: the
file's mtime (04:10) is after that doc's landing (04:07), so a later concurrent edit (most likely the
node-graph-attach lane, landed 04:14) removed it while touching the same file. **Conclusion: the "TS
thunk/command-ingress" flood named in the brief is either already fixed, or was paraphrased browser
text that does not map 1:1 to a literal source string — treat the TS side as much lighter than the
Rust side right now**, modulo the lines below.

| # | file:line | marker (abridged) | frequency | verdict |
|---|---|---|---|---|
| T1 | `🏛️ShellHost/🟦️.tsx:1586` | `extension invocation completed {extensionId, capability, instanceId, req, status}` | hop 3b — once per extension call | **keep, low volume** (one per node per tick, not per turn) |
| T2 | `🏛️ShellHost/🟦️.tsx:1616` | `invokeExtension resolve {…, resolved, loaded:[…]}` (`console.debug`) | hop 3 — once per extension call; `loaded` serializes the WHOLE plugin id list every time | **gate/trim** — the `loaded` array is dead weight once the round trip is healthy |
| T3 | `🕸️NodeGraph/🟦️.tsx:2062,2131,2138,2170,2177,2202` | `node-graph first draw …` / `host mount …` / `session ready …` / `attach called …` / `surface ready …` / `attach failed …` | once per mount/attach, not per turn (matches boot #5's console excerpt) | **keep** — one-shot lifecycle trace, not a flood source |
| T4 | `🏛️ShellHost/🟦️.tsx` (~70 more sites) | assorted `console.log/warn/error("[DEBUG] …")` | one-shot per user action / catch clause | **keep** — these are the "real diagnostics" the flood buries, not the flood itself |

**Net finding for §1**: the console flood described in the brief is overwhelmingly the **Rust
`eprintln!` side** (R1–R3, R9, R16–R18 above), all of it either (a) this ticket's own temporary
extension-round-trip instrumentation (R3, R4, R6, R9, R16–R18 — seven call sites, all doc-labelled
"temporary" and all uncommitted right now) or (b) the maintenance/cooperative-maintenance clock (R1,
R2) which is genuine perf signal but unconditional/under-throttled at the 2ms and 8ms thresholds this
app hits routinely. The TS side contributes one-shot lifecycle noise, not per-turn floods.

---

## 2. The cooperative-maintenance clock

`MAINTENANCE_STAGES: u8 = 24` (`🔌️plugin/🦀️.rs:23799`). `maintenance_step` (line 24324) advances
`self.maintenance_stage` by exactly one MOD 24 per call, then `match`es on the OLD stage — i.e. **one
call = one stage = one 1/24th slice of the rotation**, driven once per reactor turn via
`RuntimeLiveCleanupJob::step` (`🔌️plugin/🦀️.rs:29449` calling `instance.app.maintenance_step(1,
RUNTIME_CLOSE_BYTES_PER_STEP=4096)`), itself pumped by `run_runtime_live_cleanup_turn` /
`runtime_live_cleanup_pump_one`.

There is a fast exit before the rotation even starts: if 24 named collections/cursors
(`tool_operations`, `latest_wins_commands`, `envelope_ingress`, `envelope_decode_jobs`, …, all the way
to `presence_store.local_read_maintenance_is_idle()`) are simultaneously empty, `maintenance_step`
returns via `advance_snapshot_read_returns_one` WITHOUT touching the stage rotation at all
(`🔌️plugin/🦀️.rs:24349-24354`). So a truly idle instance with a settled document does not pay for
the 24 stages — the cost only appears while there is retained state to walk, which is exactly the
generation3d case: the flow window's transient store always holds at least one `SetPreviewEval`
snapshot (see §3), so that fast exit never triggers here.

### What each stage does (idle-cost read)

| stage | what it scans | cost for an idle-but-non-empty generation3d instance |
|---|---|---|
| 0 | `tool_operations` ring buffer (`ARTIFACT_LIVE_OUTPUT_SLOTS` slots) for a `Worker`/`Retiring` entry | O(slots), no-op scan |
| 1 | `media_closures` cursor | O(1) no-op (generation3d has no media exports) |
| 2 | `segmented_closures` cursor | O(1) no-op |
| 3-7 | further fixed-slot closure/retirement cursors (snapshot/child-content/child-admission-abort/peer-roster/peer-presence) | O(1) no-op each |
| 8 | `document_snapshot_read_returns.drive(store.take_returned_snapshot_read_retirement, …)` | real work only when a snapshot read was returned this generation |
| 9 | `store.maintenance_retirements_step` | real work only when the store has a displaced/retiring root |
| 10-14 | envelope ingress/decode/field-decoder/completed-record/store-replacement pumps | O(1) no-op — generation3d does not use the artifact-envelope import path in this scenario |
| 15 | `A::mounted_job_maintenance_step` (app-owned mounted jobs) | depends on the app; generation3d's tessellation job (`📓️tessellation-jobs-2026-09-09.md`) is mounted here when a tessellate is in flight |
| 16 | `instance_operation_owner.maintenance_step` | O(1) cursor advance |
| 17 | `latest_wins_keys.advance` | O(1) |
| 18 | `tool_cancellations.cleanup_finished_slot` cursor over `TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS` | O(1) per call, full cycle is O(slots) |
| 19 | `presence_store.maintenance_local_reads_step` | real work only when a presence read was returned (camera/show-mode updates every eval tick DO return one — see §3) |
| 20 | `child_admission_abort_step` | O(1) no-op (no spawned children) |
| 21 | `retire_document_windows_step` | O(1) no-op unless a window just closed |
| **22** | `window_transient_store.maintenance_step` → `TypedWindowTransientStoreOwner::maintenance_step` (`🪟️window/🫧️transient/🦀️.rs:216-231`): walks a `BTreeMap<window_id, partition>` cursor, one partition per call, and calls `partition.store.maintenance_returned_reads_step(…)` on it | **this is the one that actually retires bytes**: it is where a previously-published `SetPreviewEval{eval_text}` snapshot's OLD generation gets its returned read freed. `eval_text` is the WHOLE flow-eval-session JSON (§3) — for the 7-node hexagonal-mushroom-column example this is easily tens of KB of JSON, and dropping/retiring that string is exactly the kind of allocator work that turns into a multi-ms stage when it lands on this one call |
| 23 | `pump_worker_job_retirements` (process-wide worker-job slot array) | O(1) unless a session was torn down mid-flight |

So: **stages 0-7, 10-14, 16-18, 20-21, 23 are genuinely O(1) no-ops for this app in this scenario** —
"wasted work" only in the sense that the reactor still burns one full turn's maintenance slot walking
them in rotation even when nothing is there (24 turns to get back to stage 22 again). **Stage 22 is
the real cost center**: it is a byte-proportional retirement of the previous tick's serialized eval
JSON, and it recurs every 24 turns purely because the rotation is round-robin, not
priority/work-aware — there is no fast-path that revisits stage 22 sooner just because it is the one
with pending bytes.

`interactive_step_contract_violated(elapsed_us) = elapsed_us >= INTERACTIVE_STEP_CEILING_US (8_000)`
(`⏱️trace/🦀️.rs:99-103`). R2's 12 200us callback overrun is the WHOLE `run_runtime_live_cleanup_turn`
wall clock (host scheduling + the one pumped stage), not pure CPU inside stage 22 — the code's own
comment at `🔌️plugin/🦀️.rs:29546` says as much ("measures wall time including descheduling"). R1's
6 900us is the narrower `maintenance_step` call itself, i.e. the actual stage-22 retirement of the
large `eval_text` string. Both numbers point at the same root cause: **a large String is being
allocated, serialized, and later freed once per eval tick, and the free lands inside the 8ms
interactive turn budget**.

---

## 3. Reactor turn budget

`REACTOR_EXECUTOR.with(|executor| executor.run_until_deadline(64, 256*1024,
Instant::now() + Duration::from_millis(8)))` (`⚛️reactor/🔄️turn/🦀️.rs:833`) — confirms the 8ms/turn
ceiling quoted in the brief; `INTERACTIVE_STEP_CEILING_US = 8_000` (`⏱️trace/🦀️.rs:99`) is the same
number reused as the per-callback fault threshold for maintenance (§2).

### The hop chain (from `📓️eval-continuation-runtime-2026-09-10.md` §3, cross-checked against source)

Per geometry node, one `flowEvalTick` round trip to an extension is **7 named hops**, each a
guest<->host boundary crossing:

1. `flowEvalTick` declares the evaluate invocation (`⏱️flow-eval-tick/🦀️.rs:31`, R16)
1b. (tessellate path only) `flowTessellate` declares the invocation (`✏️editor/🦀️.rs:2234`, R17)
2. SDK mints `req`, parks the continuation (`⚛️reactor/🦀️.rs:716`, R9)
3. TS host resolves the extension actor (`🏛️ShellHost/🟦️.tsx:1616`, T2)
3b/3c. TS host runs the capability / reports a dispatch fault (`🏛️ShellHost/🟦️.tsx:1586` / `:4845`)
4. TS submits `Event::Completed` to the guest turn (was `🔌️PluginRuntime/🟦️.tsx:2067` — now removed, §1.2)
5. Reactor receives `Event::Completed` (`⚛️reactor/🔄️turn/🦀️.rs:287`, R3)
6. Registry resolves hit/dropped/miss (`⚛️reactor/🔄️turn/🦀️.rs:290/306/308`, R4-R6)
7. App consumes the result (`✅️flow-eval-resolve/🦀️.rs:18`, R18)

Hops 1-2 and 5-7 are Rust-side and settle within the SAME reactor turn as the `Event::Completed`
delivery (the code comment at `⚛️reactor/🔄️turn/🦀️.rs:274-278` is explicit: "a plugin that wanted an
extension result had to hand-mint a `RequestId`... redispatched into the owning app instance with the
outcome merged onto the original request object... resumed... AFTER `run_until_idle` so a task that
resolved just now is redispatched the SAME turn, not the next one"). Hops 3/3b/4 are a full
JS-microtask round trip (the extension lives in a separately-loaded plugin module in the SAME tab, no
network/worker hop for the react target) — cheap in wall time but it forces the guest turn that issued
hop 2 to end (the reactor cannot block on a JS promise inside one 8ms window) and the RESPONSE arrives
on a LATER `poll()` call, i.e. a later turn.

**Turns-to-first-mesh for the hexagonal-mushroom-column example** (`scene.nodes=7, edges=6`, per boot
#3): every node whose evaluation depends on an extension (`profile`, `extrusion-axis`, `extrude` per
the boot-#3 statusJson) needs at minimum turn N (dispatch, hops 1-2) then turn N+1 or later (hops
5-7, gated on the JS round trip actually completing before the next `poll`). With `flowEvalTick`
self-redispatching every turn while `session.tick()` still returns `true` (`👁️viewer/🦀️.rs:184-186`),
and the extension chain being strictly sequential per the flow graph's dependency edges (profile →
extrusion-axis → extrude → column-preview, from the boot-#3 evaluation snapshot), a lower bound is
**~2 turns per extension-dependent node × the number of such nodes on the critical path** — call it
6-8 turns minimum for this 7-node graph, before counting redispatch turns spent while
`session.tick()` is still stepping the LOCAL (non-extension) nodes, or turns lost to work-capacity /
fold-contract faults that currently abort the operation before eval starts at all (`📓️status.md`
04:12/04:14 — `work-capacity` lane still open as of this audit).

### Serialization overhead per tick

- **`SetPreviewEval{ eval_text }`** (`👁️viewer/🦀️.rs:154,200`): on every `flowEvalTick`/view-command
  step that completes a tick, `session.eval_json().to_string()` re-serializes the WHOLE eval session
  to a fresh `String`, published as an ephemeral window-transient mutation. This is the string §2
  identifies as stage 22's retirement cost.
- **`preview_payload` re-run per render** (`👁️preview/🦀️.rs:300-360`): `render()` (`:395-406`) always
  re-parses `eval_text` with `dsl::json::parse`, then `preview_payload` rebuilds its `meshes`/`mesh_id_by_handle`
  dedup maps FROM SCRATCH every call and calls `semio_framework_os_flow::tessellate_geometry(&handle,
  tolerance)` for every preview channel item not already in that per-call-local dedup map — i.e.
  **tessellation is not cached across renders**, only de-duplicated within one render pass. Any
  render triggered by an unrelated UI change (camera drag, hover) re-tessellates every visible mesh.
- **Fixture re-projection**: `render()`'s fallback path (`:399-401`, `owned_eval = … Some(evaluate_fixture(&document.fixture))`
  when `eval_json` is empty) calls `host.evaluate()` — a full flow evaluation — synchronously inside
  the PURE render function whenever the ephemeral `preview_eval_text` has not yet been populated
  (e.g. right after a document open/replace, or after the eval session was reset). Combined with
  `Generation3dViewCommandWork::step`'s own `input.snapshot.fixture.clone()` on every command start
  (`👁️viewer/🦀️.rs:174`), the fixture (the whole flow graph document) is cloned at least once per
  view-command dispatch, independent of whether anything in it changed.

---

## 4. Ranked optimization list (impact × effort)

| rank | target | file:line | change | metric moved |
|---|---|---|---|---|
| 1 | Remove the 7 stale round-trip probes | `⚛️reactor/🔄️turn/🦀️.rs:287,290,308` (R3,R4,R6); `⚛️reactor/🦀️.rs:716` (R9); `⏱️flow-eval-tick/🦀️.rs:31` (R16); `✏️editor/🦀️.rs:2234` (R17); `✅️flow-eval-resolve/🦀️.rs:18` (R18) | delete now that `📓️eval-continuation-runtime-2026-09-10.md` §4 shows the round trip unit-tested end to end (`hex_column_evaluates_end_to_end` passing); all are doc-labelled "temporary" already, all uncommitted | **console lines/boot**: removes the highest-frequency unconditional sites — one line per hop per node per tick, i.e. the majority of "thousands of lines per boot" |
| 2 | Gate stage-cost/callback-overrun logs behind a flag | `🔌️plugin/🦀️.rs:29464` (R1), `:29546` (R2) | wrap in a `SEMIO_MAINTENANCE_TRACE` (or similar) check read once at process start, default off in served/dev builds, on in a dedicated perf-diagnosis run | **console lines/boot** without losing the signal — flip it on only when chasing a stage-cost regression |
| 3 | Stop re-serializing the whole eval session every tick | `👁️viewer/🦀️.rs:200` (`session.eval_json().to_string()`) + `SetPreviewEval` (`…/set-preview-eval/🦀️.rs`) | publish only the DELTA the preview window needs (mesh/instance ids that changed) instead of the full eval JSON string, or keep the `FlowEvalSession`'s own value and hand the preview a cheap reference/version stamp instead of a freshly-allocated `String` | **ms/turn** (removes the stage-22 6.9ms retirement + the allocation cost) and **bytes/turn** |
| 4 | Cache tessellation across renders | `👁️preview/🦀️.rs:311-338` (`preview_payload`'s per-call-local `mesh_id_by_handle`) | key the mesh cache by `(handle, tolerance)` in a struct that survives across `render()` calls (e.g. on `Generation3dViewTransient` or a small LRU keyed by brep handle), so an unrelated re-render (camera/hover) does not re-tessellate unchanged geometry | **ms/turn**, **turns-to-first-mesh** for every render after the first — this is likely the single biggest win once eval reaches a stable state, since `tessellate_geometry` is the most expensive call in the render path |
| 5 | Avoid the synchronous fixture clone + full evaluate fallback in the pure `render()` path | `👁️preview/🦀️.rs:399-401` (`evaluate_fixture` fallback), `👁️viewer/🦀️.rs:174` (`input.snapshot.fixture.clone()`) | make `render()` fail closed (empty payload) instead of silently re-evaluating the WHOLE fixture when `preview_eval_text` is absent, and stop cloning the fixture per command-work start when the tick can borrow it instead | **ms/turn** on the cold/reset path, **turns-to-first-mesh** (avoids a redundant full evaluate racing the real `flowEvalTick` chain) |
| 6 | Make stage 22 self-prioritizing instead of round-robin | `🔌️plugin/🦀️.rs:24358` (`self.maintenance_stage = (self.maintenance_stage + 1) % MAINTENANCE_STAGES`) | let a stage that reports non-zero `released_bytes` be revisited on the NEXT call instead of rotating away for 23 turns; today a large pending retirement (stage 22) sits queued for up to 24 turns behind cheap no-op stages 0-21 before being serviced again | **ms/turn variance** — smooths the tail latency the 6.9ms/12.2ms outliers show, without changing total work |
| 7 | Trim `T2`'s `loaded` array from the hot debug line | `🏛️ShellHost/🟦️.tsx:1616` | drop `loaded: plugins.map(...)` from the `console.debug` payload (or gate the whole line) — it serializes every installed plugin id on every extension call | **bytes/turn** on the TS side, minor but free |

**Not actionable from this audit alone** (flagged for the owning lane, not re-litigated here): the
`retained command exceeds semantic work capacity` fault blocking every typed operation as of boot #5
(`work-capacity` lane, `📓️status.md` 04:14) sits upstream of all turn-budget math in §3 — until it is
fixed, "turns-to-first-mesh" cannot be measured end to end on a live boot, only reasoned about from
source.
