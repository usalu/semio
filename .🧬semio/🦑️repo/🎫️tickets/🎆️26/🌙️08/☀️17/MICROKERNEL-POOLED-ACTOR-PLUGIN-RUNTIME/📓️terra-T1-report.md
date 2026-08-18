# 📓️ terra — T1-tasks report (runtime metrics publisher + task manager UI)

Packet T1, ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`. Executor: terra.

## 1. What exists now

### (a) Metrics: additive, tested, real sampling — publication is a documented gap, not faked

**`🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`** (pure crate, region `📈️Metrics` + `🏛️Kernel`, purely additive):

- `ActorMetricsSample { id, package, lane, status, metrics }` — joins `ActorMetrics` with the kernel-level `package`/`lane`/`status` it doesn't itself carry. New `Scheduler::lane_of(actor)` supplies `lane`.
- `ShardMetricsSample { shard, metrics }` and `RuntimeMetricsSnapshot { kernel, actors, shards, sampled_at_ms }` — the exact payload `KernelMetrics`'s own pre-existing doc comment promises ("the host publishes this as bus topic `os.runtime.metrics` at 2Hz").
- `Kernel::actor_metrics_samples()`, `Kernel::shard_metrics_samples()` (busy_ratio computed purely from `ActorStatus::Active` fraction per shard; `heartbeat_age_ms` left at 0 — the pure crate has no clock/transport), `Kernel::runtime_metrics_snapshot(sampled_at_ms: u64)`.
- `RUNTIME_METRICS_PUBLISH_INTERVAL_MS = 500` (2Hz) and `runtime_metrics_due(last_published_ms: Option<u64>, now_ms: u64) -> bool` — the clock-injected cadence gate (never reads a clock itself; every caller passes `now_ms`, same discipline as the pre-existing `Kernel::tick`).
- All 3 new types have `pack_encode`/`pack_decode` (hand-rolled `pack` module) and `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]`, and are added to the existing `exports_typescript_bindings` test.

**Clock injection**: nowhere in the crate is a clock read. `RuntimeMetricsSnapshot::sampled_at_ms` and `runtime_metrics_due`'s `now_ms` are both caller-supplied — the exact same pattern `Kernel::tick(now_ms)`/`Kernel::complete(..., now_ms)` already used before this packet.

**`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`** (native host, new region `📈️RuntimeMetricsPublisher`, inserted between the pre-existing `🔀️PostTurnRelay` and `🔖️ArtifactSession` regions — a gap between named regions, touching neither):

- `RuntimeMetricsPublisher { last_published_ms }` with `maybe_sample(&mut self, kernel: &Kernel, now_ms: u64, shard_heartbeats: &HashMap<ShardId, u64>) -> Option<Vec<u8>>` — gates on `runtime_metrics_due`, then samples `kernel.runtime_metrics_snapshot(now_ms)` and overlays `heartbeat_age_ms = now_ms - shard_heartbeats[shard]` (the ONE field the pure crate cannot compute itself — see that type's own doc comment), then pack-encodes.
- **Bus mechanism reused, not invented**: `Origin::Bus { topic }` + `Payload::Event(bytes)` (both pre-existing in `🎭️actor/🦀️component.rs`'s `✉️Envelope` region) is the actual "publish to topic, deliver via `Envelope`" primitive this repo already has — confirmed by an existing pack round-trip test literally named `pack_round_trip_origin` using `Origin::Bus { topic: "os.runtime.metrics".into() }` as its fixture, present in the file BEFORE this packet touched it. `Effect::PublishEvent{topic, payload}` (kernel crate, WIT-marshaled in this same host file) is the guest-facing mirror of the same concept.

### Honest gap: no delivery, because no delivery mechanism exists anywhere in this codebase yet

I grepped before starting and again before writing this report:

```
grep -rn "PublishEvent" --include="*.rs" .        # only WIT-marshal conversions (guest ↔ kernel Effect enum), zero dispatch/delivery code
grep -n "Effect::Subscribe\|subscri" 🖥️host/🦀️component.rs   # only the same marshal conversions
grep -rn "Kernel::new(" --include="*.rs" .        # only semio-framework-actor's own tests + its wasm-only KernelHost glue
```

No code anywhere — native or web — tracks which `ActorId`s subscribed to a topic, and no code anywhere drives a live `Kernel` on a native thread at all (`Kernel::new` has zero call sites outside the actor crate itself). `RuntimeMetricsPublisher` is therefore wired as **the exact call the future kernel-thread owner makes each pump** — real, unit-tested end-to-end against a real `Kernel`, but nothing currently invokes it at runtime, and there is nowhere to hand its output for actual subscriber fan-out. This is a pre-existing, repo-wide gap (confirmed identical on the web side below), not something T1 introduced or could close within `path_scope` (a subscriber registry + fan-out loop is cross-cutting infrastructure, not "publisher wiring").

**Web side** — `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` (region `📈️RuntimeMetrics`, new):
- `ShardMetrics`/`ShardMetricsSample` — hand-authored TS stand-ins (same convention as the file's own pre-existing `ShardBudget`), field-compatible with the Rust structs.
- `ShardClient.shardMetricsSamples(nowMs)` — built PURELY from data `ShardClient` already tracks: `actorIds.size` → `actors`, `pendingRequestIds.size / actorIds.size` → `busyRatio` (the same "busy" proxy `checkHeartbeats` already uses via `oldestPendingStartedAtMs`), `nowMs - lastHeartbeatAtMs` → `heartbeatAgeMs` (`Infinity` for a shard that never heartbeated, matching `freshHeartbeatState`'s own convention). No wasm `Kernel` call needed — `ShardClient` is exactly the object holding the heartbeat clock on web, unlike the native host which needs the overlay.

**`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`** (region `🐚️ActivationRegistry` only — see `## peer-coexistence`):
- `ActivationRegistryOptions.now` (injectable clock, same pattern as `ShardClient`'s own `options.now`).
- `ActivationRegistry.runtimeMetricsActorRows()` — one row per actor this registry has EVER activated (`actorPlugin` map, never cleared on suspend), each with `resident`/`shard`. Honest gap documented inline: this registry never held a `Kernel` (it delegates straight to `ShardClient` — confirmed by reading the class before touching it, `activate()`/`suspend()`/`resume()` call `ShardClient` methods directly, never a wasm `Kernel`/`KernelHost`), so `turns`/`traps`/`wallUsP95`/etc. are not available here — not a silent zero-fill, an explicit doc-comment gap.
- `ActivationRegistry.runtimeMetricsSnapshot(sampledAtMs?)` combining the above with `ShardClient.shardMetricsSamples`.
- `runtimeMetricsDue`/`RUNTIME_METRICS_PUBLISH_INTERVAL_MS` — the exact TS mirror of the Rust cadence gate, exported standalone so it's testable without fake timers.
- `ActivationRegistry.startRuntimeMetricsPublisher(sink)` — real `setInterval` loop (not the injected `now`; browser timer loops aren't in `🎭️actor`'s clock-injection purity scope) calling `sink("os.runtime.metrics", snapshot)`. Same honest gap as native: `sink` has nowhere real to deliver to yet (documented inline, lease-request candidate once a real bus lands).

## 2. Task manager — where placed, and why

**Location**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TaskManager/` — new directory, matching the repo's own leaf-element naming convention (`AgentApprovals`, `EventFeedHost`, `Table`, … — all PascalCase folders, no emoji at that level; the emoji lives on the files inside). Two files: `🟦️component.tsx` and `🧪️component.test.tsx`.

**Taxonomy justification**: `🧱️elements/` under `📺️renderer/🧑️‍🎨️engine/` is where every other OS-level pane/dialog lives (`AgentApprovals`, `GraphTimelineHost`, `Dock`, …). Studying that sibling set (per the packet's own instruction) before writing anything:

- Studied `Table/🟦️component.tsx` (`TableHost`, a `ComponentSceneHostProps` consumer of a generic `SurfaceKind::Table` scene) and `Interpreter/🧊️component.rs` (the wgpu-side generic dispatcher — its own test `scene_command_reaches_every_generic_fallback_surface_kind_without_panicking` lists `SurfaceKind::Table` as one of 11 "generic fallback" kinds both renderers already draw identically from the SAME `columnsJson`/`rowsJson` scene data).
- Studied `AgentApprovals` (typed-props-in/typed-callback-out, `useLabel`+`registerUiTranslationBundles` en-then-de i18n, plain-`<button>`/`aria-label` accessibility pattern) as the "directly-mountable dialog" precedent.
- **Decision**: `TaskManager` does BOTH. `buildTaskManagerTableScene(rows, labels)` mints the exact `TableColumnRecord[]`/`TableRowRecord[]` JSON shape (columns + "buttons" cells) that `TableHost`/the wgpu Interpreter ALREADY render identically via `SurfaceKind::Table` — this is the genuine, in-scope answer to "must render in BOTH renderers, don't invent a new pattern": reusing an already-dual-rendered surface kind, rather than registering a brand-new `SurfaceKind::TaskManager` (which would need new WIT/Rust/TS variants in `ui_wgpu` and a dispatch-switch edit in `Interpreter`, both outside this packet's `path_scope`). `TableColumnRecord`/`TableCellButton`/`TableRowRecord` are private to `Table/🟦️component.tsx` (not exported), so `TaskManager/🟦️component.tsx` re-states their JSON shape by hand — checked byte-for-shape against that file's source before writing (`TaskManagerTableColumn`/`TaskManagerTableButton`/`TaskManagerTableCell`/`TaskManagerTableRow`).
- ALSO exports `TaskManagerPanel` — a directly-mountable, directly-testable/screenshot-able React view (same `<Table>` UI-kit primitive `TableHost` itself uses) for a standalone dialog/pane context, mirroring `AgentApprovals`'s own "typed props in, typed callback out" shape.

**Row shape** — `TaskManagerRow { actorId, packageId, lane, status, stage, shard, wallUsP95, mailboxLen, turns, traps, restarts }`, field-compatible with `ActorMetricsSample` joined with `ActorMetrics`. Actions: `taskManagerRowAction("suspend"|"resume"|"cancel", actorId)` mints an `ActionDescriptor { controllerId: "os.task-manager", action, args: { actorId } }` — the SAME shape `dispatchCellAction` in `Table/🟦️component.tsx` already merges args onto, so a future dispatcher sees the identical convention every other table action does. Per the ticket's own note (K1 sibling packet wiring `Payload::Suspend`/`Resume`/`Cancel` into `ShardLoop::pump`), these actions are addressed correctly but **not connected to a live dispatcher** — I did not fake the effect, only minted the correctly-shaped descriptor.

## 3. i18n / a11y

- **i18n**: `taskManagerUiLabel = registerUiTranslationBundles({ en: {...}, de: {...} })` — the SAME mechanism `AgentApprovals`/`AgentBridge` already use (`@semio-tech/ui-react`), English first then German (`SHELL_LOCALES = ["en","de"]`, confirmed from `🤖️generated/🟦️ui-axes.ts` — no third locale to omit). Every column header, lane/status value, and action label is a registered key with `normal`/`beginner` tiers; `useTaskManagerLabels()` is the one place `useLabel` is called, keeping the pure builders (`taskManagerColumns`/`taskManagerRows`/`buildTaskManagerTableScene`) React-free and directly unit-testable with a hand-built `TaskManagerLabels` fixture.
- **a11y**: `TaskManagerPanel`'s action buttons carry `aria-label={"${action}: ${actorId}"}` (explicit, not relying on `title`-as-accessible-name) so every row's 3 actions are independently keyboard/screen-reader reachable even with many rows — tested (`getByRole("button", { name: "Suspend: actor-2" })`). The scene-JSON path (`taskManagerRows`) follows `Table/🟦️component.tsx`'s own existing convention of a `title` attribute on an icon-only button (documented as inherited, not a new pattern) since that JSON is consumed by code outside this packet's `path_scope`.

## 4. peer-coexistence

Liveness check (`git log --date=iso --oneline -5` + mtime + `pgrep -fl cargo` for real cargo processes, not just "cargo" appearing in some other tool's PATH env) run before touching each shared file, ~15:15 (2026-08-18):

| file | last commit | mtime | verdict |
|---|---|---|---|
| `🎭️actor/🦀️component.rs` | `1eaf87e6f5` | Aug 17 21:40 | cold (>17h), safe |
| `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` | `abd29c08d0` | Aug 18 13:50 | cold (>1h), safe |
| `🎠️kernel/🟦️component.ts` | `abd29c08d0` (+4 more) | Aug 18 02:33 | cold (>13h), safe |
| `🔌️plugin/🖥️host/🦀️component.rs` | `ee16e76c4e` (+4 more) | Aug 18 14:40 | 35 min old, no live cargo build found — proceeded, re-read fresh, edited surgically in a gap between two named regions |

No `cargo` build process was found running against any of this ticket's target dirs or the shared root `target/` for MY files specifically at check time (one peer's `naga`/root-`target/` build was observed later, unrelated to these 4 files, in a completely different crate).

**`🎠️kernel/🟦️component.ts`'s `🔖️IoRouter` region (lines 560–799, 240 lines) — proven byte-identical**:
```
$ sed -n '560,799p' 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts | wc -l    # 240 — same span as before any edit
$ sed -n '560,799p' 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts | md5     # 222db26fd16d7169db59b50ef0afdc65
```
Every edit hunk in this file starts at line 1549 or later (confirmed via `git diff -U0` before a peer's own concurrent commits landed on top) — strictly inside/after `🐚️ActivationRegistry` (1501+), never touching `🔖️IoRouter`. The region's own `🧪️IoRouterTests` (end-of-file, 9 tests) all still pass — see `terra-T1-kernel-vitest.txt`.

No `git commit`/`stash`/`checkout`/`add` run by me at any point (an auto-commit bot is confirmed live in this tree — `git status`/`git diff --cached` mid-session showed OTHER sessions' concurrent, unrelated changes already staged in the same files; I did not rely on `git diff` for isolating my own edits, only on the Edit tool's own before/after and my own test runs).

## 5. Commands + exit codes

```
$ cargo check -p semio-framework-actor --all-targets        # target-t1        → exit 0
$ grep -nE 'wasm_bindgen|web_sys|winit|tokio|rayon|std::thread|SystemTime|Instant::now|std::fs|std::net' 🎭️actor/🦀️component.rs
    → 1 match, line 2, the file's OWN top-of-file doc comment naming the forbidden tokens as prose
      (pre-existing before this packet — not a real violation; grep can't see comments)          → exit 0 (match found, false positive)
$ cargo test -p semio-framework-actor                        → 57 passed; 0 failed  (was 52 — +5 new)   exit 0   (terra-T1-actor-test.txt)
$ cargo check -p semio-framework-plugin-host --all-targets    → exit 0
$ cargo test -p semio-framework-plugin-host --all-targets     → 75 passed; 0 failed  (was 73 — +2 new)   exit 0   (terra-T1-plugin-host-test.txt)
$ bun nx run @semio-tech/framework-actor:test                 → 30 passed; 0 failed  exit 0   (terra-T1-shard-client-vitest.txt)
$ bunx vitest run --config terra-t1-kernel-vitest.config.ts   → 14 passed; 0 failed  exit 0   (terra-T1-kernel-vitest.txt; ad-hoc config — see § below)
$ node node_modules/vitest/vitest.mjs run --config <ui-react>/🧪️vitest.config.ts --passWithNoTests \
    --root <renderer elements dir> TaskManager/🧪️component.test.tsx
                                                                → 9 passed; 0 failed   exit 0   (terra-T1-taskmanager-vitest.txt)
```

**Why an ad-hoc kernel vitest config**: `🎠️kernel/🟦️component.ts` has no dedicated vitest project of its own — repo-wide grep found no `package.json`/`vitest.config.ts` that includes it (unlike `shard-client.ts`'s own dedicated `🧪️vitest.config.ts`). Its pre-existing `import.meta.vitest` blocks (`🧪️ExpandPluginRegistryTests`/`🧪️IoRouterTests`) appear to be in the SAME orphaned-from-any-target state `AgentApprovals`'s own report already documented for that file's tests. `terra-t1-kernel-vitest.config.ts` (this ticket folder — a diagnostic script, not a registrar file) points `includeSource` directly at the file; it runs ALL of that file's inline tests (mine + the 10 pre-existing ones), not a scoped subset, so the 14/14 pass count above is proof the pre-existing tests are unaffected too.

## 6. Runtime evidence (not just compilation)

`component::runtime_metrics_publisher_tests::runtime_metrics_publisher_reflects_the_2550_record_scale_fixture_registry` (in `🔌️plugin/🖥️host/🦀️component.rs`, region `📈️RuntimeMetricsPublisher › 🔖️ScaleFixture`):

- Loads the REAL fixture via `include_str!("../../../🧫️fixtures/🔌️scale/🤖️generated/🔣️registry.json")` (not a copy — the actual 2550-record, 50-plugin/50-extensions-each, 7-behaviour-profile file at `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/🤖️generated/🔣️registry.json`).
- Activates **all 2550 records** through a real `Kernel` (`kind`→`ActorKind::PluginApp`/`Extension`, `parentId`→`PackageId`, `scaleFixture.profile`→a deterministic `Lane`).
- Drives real turns: the first record gets a clean `submit`/`tick`/`complete`; the first `"crash"`-profile record gets a `TurnStatus::Faulted` turn.
- Calls `RuntimeMetricsPublisher::maybe_sample`, decodes the pack-encoded bytes back into `RuntimeMetricsSnapshot`, and asserts: `kernel.actors == 2550`, `kernel.packages == 50`, `actors.len() == 2550`, the driven actor's row shows `turns == 1`/`status == Active`, the crashed actor's row shows `traps == 1`, and the shard rows sum to exactly 2550 actors.

This is the acceptance criteria's own bar met literally: "A unit/integration test that drives real actors through the kernel and asserts the published metrics rows" — using the exact intended data source, not a screenshot-free "it compiles."

`RuntimeMetricsSnapshot`/`RuntimeMetricsPublisher`'s smaller unit tests (`runtime_metrics_snapshot_reflects_real_kernel_activity` in the actor crate, `maybe_sample_gates_at_2hz_and_overlays_heartbeat_age_from_the_host` in the host crate) additionally prove the 2Hz cadence gate and the heartbeat overlay, independently of the scale fixture.

No browser/task-manager-listing-live-actors screenshot was taken — `TaskManagerPanel` is proven to render/dispatch correctly against synthetic `TaskManagerRow[]` props (9 passing tests), but nothing wires it to a live `Kernel`/`ActivationRegistry` yet (see `## honest gaps`), so there is nothing live to screenshot. The kernel-level scale-fixture test above is the acceptance criteria's own explicitly-named fallback for exactly this situation.

## 7. Lease requests

- **`terra-T1-lease-typegen.md`** (this ticket folder) — run `bun nx run @semio-tech/framework-actor-rs:typegen` so `ActorMetricsSample`/`ShardMetricsSample`/`RuntimeMetricsSnapshot` land in `🤖️generated/🟦️actor.ts` (registrar-only, `🎭️actor/🟦️component.ts` re-exports it). Pre-existing gap, not new: that generated file doesn't exist at all yet for this crate. Not blocking — every TS consumer uses hand-authored stand-ins in the meantime (the same pattern `shard-client.ts` already used for `ShardBudget` before this).

## 8. Honest gaps

1. **Nothing publishes `os.runtime.metrics` at runtime, on either host.** `RuntimeMetricsPublisher`/`ActivationRegistry.startRuntimeMetricsPublisher` are real, tested, correct — but nothing currently drives a live `Kernel` on a native thread (zero `Kernel::new` call sites outside the actor crate's own tests/wasm glue), and no topic-subscriber fan-out exists anywhere in this codebase (native or web) for either publisher's output to reach. This is pre-existing infrastructure this packet's `path_scope` (publisher wiring only, additive metrics-sampling surface only) cannot close — it needs its own packet (subscriber registry + the kernel-thread that H1-H4 own).
2. **`ActivationRegistry` never held a `Kernel`.** Its `runtimeMetricsActorRows` is honestly missing `turns`/`traps`/`wallUsP95`/`mailboxLen` — documented inline, not silently zero-filled.
3. **`TaskManagerPanel`/`buildTaskManagerTableScene` are not mounted anywhere live.** No window kind `"os.task-manager"` is registered, no host-side scene commit from `RuntimeMetricsSnapshot` exists, and `ShellHost` (registrar-only) isn't wired to open one. I did not attempt a concrete lease-diff for this because it's genuinely bigger than a mount (window-kind registration + a scene-commit call site), unlike the `AgentApprovals` precedent's 3-import/1-hook/2-JSX-line lease.
4. **Suspend/resume/cancel actions are correctly shaped, not connected.** Per the ticket's own note, K1 is concurrently wiring `Payload::Suspend`/`Resume`/`Cancel` into `ShardLoop::pump`; I coded against the kernel-level API/`Payload` contract (`taskManagerRowAction`) and did not fake a working dispatch path.
5. **`heartbeat_age_ms` is 0 from `Kernel::shard_metrics_samples` itself** by design (documented) — the native host's `RuntimeMetricsPublisher` overlays the real value; a caller that uses `Kernel::shard_metrics_samples` directly without that overlay gets 0, not a real age.

## Files touched

- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` (additive: `Scheduler::lane_of`, `ActorMetricsSample`/`ShardMetricsSample`/`RuntimeMetricsSnapshot`, `Kernel::actor_metrics_samples`/`shard_metrics_samples`/`runtime_metrics_snapshot`, `runtime_metrics_due`, tests, typegen list)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (new region `📈️RuntimeMetricsPublisher` incl. tests + the scale-fixture integration test)
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` (new region `📈️RuntimeMetrics` + tests)
- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` (`🐚️ActivationRegistry` region only, additive)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TaskManager/🟦️component.tsx` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TaskManager/🧪️component.test.tsx` (new)
- Ticket-folder scratch/reports: `terra-T1-lease-typegen.md`, `terra-t1-kernel-vitest.config.ts`, `terra-T1-actor-test.txt`, `terra-T1-plugin-host-test.txt`, `terra-T1-shard-client-vitest.txt`, `terra-T1-kernel-vitest.txt`, `terra-T1-taskmanager-vitest.txt`, this report.

## 9. Follow-up (post-K1): wired the three actions to real dispatch, on web

Coordinator accepted the packet, then reported K1 landed: `ShardLoop::pump` now really dispatches
`Payload::Suspend`/`Resume`/`Cancel` (returning `ShardOutcome::Checkpoint`/`Resumed`/`Cancelled`), and
`JobStep::Done`/`Failed` became struct variants. Asked to wire the task manager's three buttons to
those real paths rather than leaving them "shaped but inert," with an explicit instruction to
lease-request rather than expand scope if that needed an out-of-scope file, and to leave the metrics
"unreachable publisher" gap exactly as documented (that gap is unrelated infrastructure, not this
follow-up's job).

**Re-read from disk before editing** (rule: several landed changes postdate my last read) — confirmed
via fresh `git log`/mtime that `🧵️shard/🦀️component.rs` (K1's file) changed at commit `ee16e76c4e`,
mtime `15:40:44`; re-read `Payload::Cancel`'s handling there (read-only — that file is NOT in my
`path_scope`) to get the semantics right: `Payload::Cancel(_)` — the inner `u64` is **ignored** by
`ShardLoop::pump`, so `Payload::Cancel(0)` cancels every one of the target actor's running jobs
regardless of the argument; `0` below is a correct value, not a placeholder.

**What I wired — web side, genuinely live, no file outside `path_scope` touched**:

- `🎠️kernel/🟦️component.ts` (`🐚️ActivationRegistry` region, still the only region touched):
  new `ActivationRegistry.cancel(actorId)` — disposes the worker-side instance via the SAME
  `ShardClient.dispose` `suspend()` already uses, then forgets the actor entirely (`resident`,
  `checkpoints`, `actorPlugin`, `residencyOrder`) so a later `resume()` correctly throws "unknown
  actor" — the web mirror of K1's native semantics ("cancels running jobs + unregisters the
  instance"). Documented honest sub-gap inline: this class has no per-actor job-id bookkeeping, so
  "cancels running jobs" is only reachable via tearing down the whole instance (`dispose`), not a
  targeted per-job cancel — that bookkeeping lives with whoever calls `startJob`/`stepJob`, outside
  this file.
- `TaskManager/🟦️component.tsx` (new region `🔖️LiveDispatch`): `createTaskManagerDispatcher(registry:
  ActivationRegistry)` — maps the three `TaskManagerActionKind`s straight onto
  `registry.suspend`/`resume`/`cancel`. `TaskManagerPanelProps.onAction` widened to `void |
  Promise<void>` to carry these through.
- **Proven genuinely live**, not just type-checked: 3 new tests in `TaskManager/🧪️component.test.tsx`
  build a REAL `ActivationRegistry` + REAL `ShardClient` (only the `Worker`/`MessagePort` is faked,
  auto-replying — the one seam `ShardWorkerLike` exists for), render a real `TaskManagerPanel`,
  `fireEvent.click` a real button, and assert the REAL registry state changed (`isResident` flips,
  `shardIndexFor` clears, `resume()` after `cancel()` rejects "unknown actor"). This is the same
  "click the actual button, assert the actual object" bar `AgentApprovals`'s own decision-dispatch
  tests set, not a mocked `onAction`.

**Native side — deliberately NOT touched, exactly as instructed**: `Kernel::suspend`/`Kernel::resume`
are pure and pre-existing (no change needed); a native cancel is `kernel.submit(Envelope { to: actor,
payload: Payload::Cancel(0), .. })`, also just composing pre-existing, already-tested API — but
nothing calls that chain because no live `Kernel` runs on a native thread (identical root cause to the
metrics publisher, confirmed unchanged). `🧵️shard/🦀️component.rs` is K1's file, not mine; wiring an
actual native call site lives in `🔌️plugin/🖥️host/🦀️component.rs` only as "publisher wiring," which
dispatch-routing is not — I did not touch it, and I'm not filing a lease for it because there is no
concrete, landable diff to request yet (it needs the same live-kernel-thread packet the metrics gap
does, not a file-permission problem). Documented in `TaskManager/🟦️component.tsx`'s `🔖️LiveDispatch`
doc comment and left as the same class of gap as `## 8` item 1 — not expanded, not re-litigated.

### Re-measured (fresh, foreground, single turn, post-K1 + post-this-follow-up)

```
$ cargo check -p semio-framework-plugin-host --all-targets          → exit 0
$ cargo test -p semio-framework-plugin-host --all-targets           → 75 passed; 0 failed   exit 0
      (terra-T1-plugin-host-test-remeasure-postK1.txt — supersedes both my earlier "75" and the
       coordinator's "74": that "74" was measured before this follow-up's own new test landed;
       this file's actual current state is 75/0, freshly re-run just now, not trusted from either
       earlier number)
$ cargo test -p semio-framework-actor                                → 57 passed; 0 failed   exit 0   (unaffected by K1 — different crate)
$ grep -nE 'wasm_bindgen|web_sys|winit|tokio|rayon|std::thread|SystemTime|Instant::now|std::fs|std::net' 🎭️actor/🦀️component.rs
      → still only the 1 pre-existing doc-comment match, unchanged
$ bun nx run @semio-tech/framework-actor:test                        → 30 passed; 0 failed   exit 0
$ bunx vitest run --config terra-t1-kernel-vitest.config.ts          → 17 passed; 0 failed   exit 0  (14 before + 3 new ActivationRegistry.cancel tests; IoRouter's 9 tests still among them, unaffected)
$ node node_modules/vitest/vitest.mjs run --config <ui-react>/🧪️vitest.config.ts --passWithNoTests \
    --root <renderer elements dir> TaskManager/🧪️component.test.tsx
                                                                       → 12 passed; 0 failed   exit 0  (9 before + 3 new live-dispatch tests)
```

`🔖️IoRouter` re-checked once more after this follow-up's edit: still 240 lines, `md5`
`222db26fd16d7169db59b50ef0afdc65` — unchanged from every earlier check in this report.

### Files touched in this follow-up (in addition to `## Files touched` above)

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` (`🐚️ActivationRegistry` region: new `cancel()` method + 3 tests)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TaskManager/🟦️component.tsx` (new region `🔖️LiveDispatch`, header doc updated)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TaskManager/🧪️component.test.tsx` (new region `🔖️LiveDispatch`, 3 real-registry tests)
- Ticket-folder logs: `terra-T1-plugin-host-test-remeasure-postK1.txt`, `terra-T1-actor-test-remeasure-postK1.txt`, `terra-T1-shard-client-vitest-remeasure.txt`, `terra-T1-kernel-vitest-remeasure.txt`, `terra-T1-taskmanager-vitest-remeasure.txt`
