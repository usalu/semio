# 📓️ terra M5-puzzle-procedural-gis report

Packet: **M5-puzzle-procedural-gis** — migrate `🧩️puzzle`, `🌀️procedural`, `🌍️gis`, `💠️lowpoly`,
`📸️remodel` to the new SDK, with this batch's distinctive requirement: move genuinely long-running
compute off the turn loop via `Effect::SpawnJob` + `start-job`/`step-job`/`cancel-job`.
Read: `📌️important.md`, `📓️design-abi.md` §6, `📓️terra-M0-stdio-report.md`,
`📓️terra-M1-small-plugins-report.md`.

## Status: IN PROGRESS — declarations landed for all five crates; acceptance builds running.
This file is being written incrementally while builds are in flight; do not treat an unfilled
section as green.

## 0. Headline finding — the audit's "WFC in puzzle" is not where the WFC is

The packet brief says puzzle's WFC precompute has "21 effect sites measured." That 21 is
`📓️design-abi.md` §0's measured `HostEffect` USAGE COUNT for puzzle (a totally different
measurement — old effect call sites, now all already `Effect::` per A3's mechanical rename,
confirmed 0 `HostEffect::` hits in all five crates). Puzzle's own precompute (brush/fill
placement, `🗿️artifacts/🧊️3d/…/✏️editor/⏳️precompute/`) is real background compute, but it is a
weighted-sampling/collision-search placement system (`weighted_sample_without_replacement`,
`brush_candidate_suggestion_weight`), not WFC, and — more importantly for this packet's job-move
requirement — it is **already incrementally bounded**: `precompute_step_lane` caps each call to an
explicit `budget: u32` (8-12 units), and `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS = 12.0` caps wall
time too (`⏳️precompute/🦀️component.rs:47`). Its docstring even names the bug this already fixed:
"a large per-call budget here is exactly what froze the UI... hundreds of Monte-Carlo collision
task units, blocking, every tick" (`✏️editor/🦀️component.rs:1013-1014`). What IS genuinely
unbounded — driving those bounded steps — is the calling convention: the HOST redrives
`fillBuildTick`/`suggestionsTick` via a hardcoded 120ms external interval (same docstring), the
old self-tick pattern `📓️design-abi.md` §2 names for deletion.

The REAL wave-function-collapse engine — 10,930 LOC, literal `wfc_engine` module (bitset, AC-3/
AC-4 propagation, entropy heuristics, beam search, graph/grid-2d/grid-3d solvers) — lives in
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`,
inside **procedural**, not puzzle. Its own docstring states the design ruling directly: "THE SOLVE
ITSELF IS AN INFERENCE... `compile_and_solve`... calls `solver.solve(seed)`" — a single synchronous
call, exposed as a `store::InferredField<AssemblySnapshot>` (computed/cached whenever the field is
read, not incrementally). This is EXACTLY the class `📓️design-abi.md` §2 names when it says
`artifact-infer` becomes the well-known cold job kind `semio.infer` "so a WFC/SfM-class inference
can no longer trap on a whole-store fuel budget" — literally naming WFC as the motivating case for
`semio.infer`. However: `assembly` is **not currently mounted** on procedural's `Plugin` (no
`.artifact(crate::artifacts::assembly::declaration()...)`, no `.editor()`/`.viewer()` — confirmed
by reading `🌀️procedural/🦀️component.rs`'s own comment: "assembly's editor/viewer are authored...
but not yet mounted... Wire once that lands", a pre-existing gap from a different ticket, not
something this packet's brief asked me to close). So the WFC solve is real, present in the tree,
and precisely matches design-abi's own `semio.infer` motivating example — but is not reachable at
runtime today (dormant code), which is why it does not show up as a live turn-budget incident yet.

The genuine, provably reachable, single-call unbounded compute is **remodel's structure-from-motion
pipeline** — see §3.

## 1. Wiring check (M0's two findings) — already correct in all five, nothing to fix

Same table shape as M0/M1. Confirmed by grep before any edit, all five:

| crate | `Cargo.toml` requests `component-guest`? | `📦️glue.rs` calls `plugin_exports!(plugin::plugin)`? |
|---|---|---|
| 🧩️puzzle | yes | yes, `📦️glue.rs:2438` |
| 🌀️procedural | yes | yes, `📦️glue.rs:1447` |
| 🌍️gis | yes | yes, `📦️glue.rs:994` |
| 💠️lowpoly | yes | yes, `📦️glue.rs:678` |
| 📸️remodel | yes (`workspace = true` form) | yes, `📦️glue.rs:963` |

## 2. Declarations added (item 1) — one edit per crate, `<crate>/🦀️component.rs`

Same shape M0 (stdio)/M1 (draw/forms/mathematical/layout/raster) already landed:
`.activation(ActivationEvent::OnArtifactKind{kind: crate::artifacts::<x>::artifact_kind().id})` per
REAL owned top-level artifact kind (live read, never hardcoded), `.execution(ExecutionMode::Isolated)`
(the SDK default — none of these five have `.handler(...)` or cross-plugin extension attachment: 0
hits for `.handler(` in all five, 0 `🧩️extensions/` dirs), `.requests(CapabilityRequest{...})` per
genuinely-used effect that maps to a documented `📓️design-abi.md` §5 `CapabilityId`.

| crate | artifact kinds activated | requests added | reason |
|---|---|---|---|
| 🧩️puzzle | `2d.puzzle`, `3d.puzzle`, `5d.puzzle` (3 real `artifact_kind()` fns, confirmed) | `documents.write`, `ui.dialog`, `shell.clipboard` | editor mutation (all 3); `Effect::OpenDialog` (puzzle3d add-object, `🎮️commands/🖌️add-brush-object` chain); `Effect::ClipboardWrite` (puzzle5d copy/cut, `✏️editor/🦀️component.rs:2298/2336`) |
| 🌀️procedural | `2d.procedural`, `3d.procedural` (assembly not mounted — §0) | `documents.write` | editor mutation |
| 🌍️gis | `2d.map` (gismap only — `gisterrain` is a composed child, no own `ArtifactKindSpec`, confirmed: no `artifact_kind()` fn in `🏔️gisterrain/🦀️component.rs`) | `documents.write`, `shell.navigate` | editor mutation; `Effect::OpenExternalUrl` (`🎮️commands/🌐️shell/🦀️component.rs:25`) |
| 💠️lowpoly | `3d.lowpoly` | `documents.write` | editor mutation |
| 📸️remodel | `3d.remodel` | `documents.write`, `ui.dialog` | editor mutation; `Effect::RequestFileOpen` (`import-frames`), `Effect::RequestMediaFrames` (`import-video`) |

**Capability ids I deliberately did NOT request** (checked every `Effect::` variant each crate
emits against `📓️design-abi.md` §5's documented `CapabilityId` list, same discipline M1 used):

- Puzzle's `SetActiveTool`/`SetActiveUtility`/`DispatchAction` — host-owned UI-chrome/RPC effects
  with no broker gate today, same category M1 already established for draw/layout's identical
  effects.
- Lowpoly's `LoadDocument` — same category, M1's own draw precedent for this exact effect.
- Remodel's `Notify`/`DownloadMediaExport` — same category.
- **Procedural's `Effect::InvokeExtension`** — deliberately NOT requested. See §4: `extension_id`
  is a genuinely dynamic runtime value (any plugin that contributes a flow operator via
  `.flow_extension(...)`, not just procedural's own 7), so a static per-id `extension.invoke:<id>`
  request cannot honestly enumerate the real target set from this crate alone. Fabricating either
  an incomplete list of procedural's own 7 flow-extension ids or an unverified wildcard would be
  guessing, not a measured ask — flagging this as a real open design question (does the broker
  support a wildcard `extension.invoke:*` grant, or must every target be named ahead of time?) for
  whoever owns the capability-broker packet, rather than inventing an answer.

No quotas declared for any of the five — no `QuotaSchema` field has a number I can defend from a
real measurement (see §3 for why I'm treating remodel's tick ceiling as evidence for the
**job move**, not as license to invent a `fuel_per_turn`/`turn_deadline_ms` guess).

## 3. Genuinely long-running compute found (measured, not predicted) — and why the job move is
## blocked upstream, not by anything in these five crates

### 📸️remodel — real, reachable, single-call unbounded (this packet's actual "SfM" finding)

`run_whole_pipeline` (`🎮️commands/🚀️run-reconstruction/🦀️component.rs`, identical body duplicated
in `🚀️run-stage/🦀️component.rs`) is called synchronously from `RunReconstruction`/`RunStage`/
`RetryStage`'s `handle()`. It contains:

```rust
loop {
    ticks += 1;
    if ticks > REMODEL_MAX_RECONSTRUCTION_TICKS { /* REMODEL_MAX_RECONSTRUCTION_TICKS = 200_000 */ }
    match engine.advance(RECONSTRUCTION_STEP_BUDGET) { /* RECONSTRUCTION_STEP_BUDGET = 8 */
        Working { .. } => continue,
        Done => return Ok(Emit::mutations(...)),   // one Emit, terminal
        Failed(message) => return Ok(Emit::mutations(...)),
    }
}
```

up to 200,000 × 8 = 1,600,000 engine work units, entirely inside ONE `handle()` call, with **zero
host-observable yielding** — confirmed by the test's own docstring: "The staged execution model is
synchronous, end-to-end — RunReconstruction... runs the WHOLE pipeline to a terminal Done/Failed
stage inside the ONE dispatch (no `advanceReconstruction` re-dispatch loop)." This is precisely the
"trap on a whole-store fuel budget" scenario `📓️design-abi.md` §2 describes `semio.infer`-class
jobs existing to prevent, just for reconstruction instead of inference.

### 🧩️puzzle — real background compute, already correctly step-bounded; the antipattern is the
### HOST-side external tick, not an in-guest unbounded loop

See §0. `precompute_step_lane`/`precompute_step` are already budget-capped per call; nothing in
puzzle's own code blows a turn. What's real and needs fixing is that `fillBuildTick`/
`suggestionsTick` are driven by a HOST-hardcoded 120ms interval dispatching `AppCommand`s, not by
the plugin's own `Effect::SetTimer` — the literal `📓️design-abi.md` §2 "self-tick loops... →
set-timer + next-wake" case. See §4 for why converting this is blocked, not skipped.

### 🌀️procedural — the real WFC (assembly) is dormant/unmounted (§0); `flow-eval-tick` is a
### self-redispatch loop around `Effect::InvokeExtension`

`flow-eval-tick` (`🎮️commands/🧮️flow-eval-tick/🦀️component.rs`, both procedural2d and
procedural3d) calls `session.tick(&mut host)` (the `flow` crate's OWN `FlowEvalSession`/`FlowHost`
evaluation loop — unrelated to `semio_framework_plugin`'s host/executor), and while `more` is true,
emits `Effect::DispatchAction{action: "flowEvalTick", req: RequestId(100), ...}` to re-arm itself
next turn — a guest-side self-tick, matching the antipattern `📓️design-abi.md` §2 names. Separately,
there are THREE `Effect::InvokeExtension` call sites (procedural2d's own `flow-eval-tick`;
procedural3d's `flow-eval-tick`; procedural3d's editor preview-tessellation path,
`✏️editor/🦀️component.rs:789`, statically targeting its own `"brep"` flow extension for preview
mesh tessellation) — all three build the effect with a HARDCODED `RequestId` literal (100/101/105),
not one allocated by the SDK's `RequestRegistry`. The kernel-level `Effect::InvokeExtension` shape
already matches `📓️design-abi.md` §2's post-`response_action` field list (confirmed: `req`/
`extension_id`/`capability`/`request_json`, no `response_action` — A2/A3 already did that rename),
so what's left "old-style" is the CALLING CONVENTION: a fire-and-forget effect with a self-picked
id and no registered waker, relying on the `flow` crate's own separate polling
(`take_pending_extension_eval`/re-dispatch) rather than the intended
`host::extensions::invoke(...).await` surface the packet brief names — which needs (d) below to
exist at all.

### 🌍️gis / 💠️lowpoly — no long-running compute found

Checked both for generation/decimation/tessellation/triangulation-scale loops (`grep -i
"generat|decimat|tessellat|remesh|simplif|quadric"` under lowpoly, `"generat|mesh.*build|
tessellat|contour|triangulat|decimat"` under gis) — every hit was either a doc-comment false
positive or ordinary per-selection interactive mesh editing (`💠️lowpoly/…/⚙️engine/🦀️component.rs`'s
`tessellate_all_json`/`add_primitive`/selection methods — bounded by the active object's own
geometry, not a global generation pass). Same category as M1's five small plugins: no job-move
work applies here, declaration-only.

## 4. Why `Effect::SpawnJob` / `Effect::SetTimer`-driven ticks / async `InvokeExtension` are
## BLOCKED UPSTREAM — precise evidence, not a hand-wave

I traced all three of this packet's named conversions to their host/SDK plumbing before writing
any guest-side code, per the packet's own "verify before assuming" instruction. All three hit real,
specific, currently-missing shared-framework mechanics — none in `✏️s/🔌️plugins/**`:

**(a) `Effect::SpawnJob` has no generic host-side executor for a job kind a plugin declares.**
`🔌️plugin/🖥️host/🦀️component.rs` converts `Effect::SpawnJob`/`Effect::CancelJob` wire↔kernel types
(line 1024-1025) but that is the ONLY place either variant is referenced in the host — there is no
code anywhere that reads a `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ...}` and
spawns/drives a job for it. The only real execution path,
`PluginInstanceHandle::run_job_to_completion` (same file, `//#region 🔀️PostTurnRelay`), is called
directly by `IoRouter`/`ArtifactInferenceRouter` for exactly THREE hardcoded well-known kinds
(`semio.io-run`, `semio.io-sniff`, `semio.infer`) — never generically. Emitting
`Effect::SpawnJob{kind: "remodel.reconstruct", ...}` today would compile and produce a wire effect
the host silently drops — a regression (silent data loss), not a fix, so I did not add it.

**(b) The guest-side job dispatcher (`⚛️reactor/💼️jobs/🦀️component.rs`) only implements 2 of the 3
already-well-known kinds, and is SHARED, non-generic code — a plugin cannot extend it.** Its
`step_job` match has exactly two arms (`semio.io-run`, `semio.io-sniff`); `semio.infer` is not
implemented (`other => Failed("job.unknown-kind")` would fire). This file lives in
`semio_framework_plugin` (the SDK crate every plugin links, not any plugin's own crate) — the
`ComponentGuest`/`JobsGuest` impl that wires WIT `start-job`/`step-job` to it
(`🔌️plugin/🦀️component.rs:33-56`) is defined ONCE, in the framework, calling `crate::reactor::
jobs::{start_job,step_job,cancel_job}` where `crate::` resolves inside `semio_framework_plugin`
itself — a downstream plugin crate has no override point. Confirmed no per-plugin job-kind
registration hook exists anywhere in `Plugin`'s builder (`🏗️builder/🦀️component.rs`) — I grepped for
`job_handler`/`JobHandler`/`.job(` and found nothing.

**(c) `Event::Timer` only wakes the async executor's parked futures — it never reaches
`ArtifactApp`/command dispatch.** `⚛️reactor/🦀️component.rs:152-155`: `Event::Timer{id} =>
{ ARMED_TIMERS...; EXECUTOR.wake(id); }` — that's the entire handler. There is no path from a fired
timer to a re-invocation of a plugin's own `handle()`/tick-style command. Converting puzzle's
120ms-interval `fillBuildTick`/`suggestionsTick` to `Effect::SetTimer` therefore requires EITHER
this reactor-level Timer→dispatch wiring, OR (b) below.

**(d) The async command surface `📓️design-abi.md` §4 promises (`Emit.tasks: Vec<AsyncTask>`, so a
handler can `.await` `host::invoke_extension`/etc. and then emit mutations) does not exist yet.**
`Emit<Mutation, ConfigMutation, DraftMutation>` (`🔌️plugin/🦀️component.rs:8542`) has exactly the
fields it had before this packet: `artifact_mutations`, `config_mutations`, `draft_mutations`,
`description`, `coalesce_key`, `effects: Vec<Effect>`, `events`, `ui_scope`, `child_emits` — no
`tasks` field, no `AsyncTask` type anywhere in the crate (`grep -c "struct AsyncTask"` → 0). The
async `Host::invoke_extension`/`Host::spawn_job`/etc. methods DO exist and DO correctly resolve via
`Event::Completed{req,...} → REGISTRY.resolve(req,...)` when reached through `host::request(...)`'s
proper allocation path — but nothing in the SDK lets a synchronous command `handle()` reach that
path today. `grep -rn "invoke_extension"` across the WHOLE repo (not just my five crates) returns
**zero call sites anywhere** — this confirms it's unused, unreachable infrastructure right now, not
something I broke or need to "finish wiring" within my own crate.

**Net effect**: procedural's InvokeExtension conversion (needs (d)), puzzle's tick→timer conversion
(needs (c), or (d)), remodel's SfM job move and the dormant assembly WFC's `semio.infer` completion
(both need (a), and the WFC additionally needs (b)) are ALL blocked by shared-framework gaps outside
`path_scope: ✏️s/🔌️plugins/{🧩️puzzle,🌀️procedural,🌍️gis,💠️lowpoly,📸️remodel}/**`. Per `📌️important.md`
rule 3 ("Never edit outside your packet's path_scope... emit a lease-request block and stop"), I am
not unilaterally redesigning `🔌️plugin/🖥️host`, `⚛️reactor/💼️jobs`, or `⚛️reactor`'s Timer routing —
these are real, multi-file SDK architecture decisions (how does a plugin register a job-kind
handler or a Timer-driven command hook? a trait? a registry? threaded through `Plugin::builder`?),
not a two-line fix like M0's `component-guest` feature flag. I did NOT emit any code that pretends
these paths work (no fabricated `Effect::SpawnJob` call sites, no `Effect::SetTimer` call sites that
would silently do nothing) — every effect I left untouched in these five crates' existing tick/
reconstruction/invoke-extension code is because touching it without (a)-(d) would either not
compile against the real SDK surface or would compile and then silently regress at runtime.

### 🔒️ LEASE-REQUEST — shared-file work needed to actually close this packet's headline ask

Files: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` (possibly),
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (`Emit`/`AsyncTask`, possibly).

Needed, in priority order for THIS packet's plugins: (1) generic host-side `Effect::SpawnJob`
execution (spawn/drive a pooled instance for an arbitrary declared job kind, not just the 3
hardcoded routers) + `Event::JobCompleted` delivery back to the ORIGINATING instance (today
`Event::JobCompleted` is received and discarded: `⚛️reactor/🦀️component.rs` "`let _ = result;`" with
an explicit comment "no `req`-per-job correlation table yet"); (2) `semio.infer` implemented in the
guest job dispatcher, routed to whichever artifact kind's registered inference (mirrors the OLD
`.inferences([...])`/`artifact_assemblies()` registry M0 already found for stdio); (3) EITHER
`Event::Timer` → app-command dispatch wiring, OR `Emit.tasks`/`AsyncTask` (`📓️design-abi.md` §4) —
either unblocks puzzle's tick conversion and procedural's InvokeExtension conversion. I did not
attempt any of these myself — real, cross-cutting SDK design work, not a surgical fix, and squarely
`🧰️framework/**`, not `✏️s/🔌️plugins/**`.

## 5. Acceptance

Environment: heavy system-wide contention observed throughout (145 concurrent cargo/rustc
processes at one point, other tickets' target dirs — `target-m2` etc. — visible in `ps aux`), so
every command here ran far past a normal check's wall time; each was still run to completion in
one continuous session (auto-backgrounded by the tool past its 10-minute foreground window, waited
out with a monitor rather than abandoned), never treated as hung.

```
$ export CARGO_TARGET_DIR=.../🎯️target-m5
$ cargo check -p semio-s-plugin-puzzle --lib
    Finished `dev` profile [unoptimized] target(s) in 30m 12s
$ echo $?
0
```
GREEN. 4 warnings, all pre-existing unused-import/dead-code, none in my edited region (`🦀️component.rs`'s
`plugin()` builder chain itself has zero warnings). Full log: `🧪️m5-puzzle-lib1.txt`.

```
$ cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 41m 24s
$ echo $?
0
```
GREEN. Full log: `🧪️m5-puzzle-wasm1.txt`. **🧩️puzzle: both acceptance commands pass.**

### 🌀️procedural
```
$ cargo check -p semio-s-plugin-procedural --lib
```
RUNNING — result pasted below once it completes.
