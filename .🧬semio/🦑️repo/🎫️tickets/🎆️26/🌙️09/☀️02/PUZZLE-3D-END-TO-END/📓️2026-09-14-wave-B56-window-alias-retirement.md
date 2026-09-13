# Wave B56 — retiring the `1:window` alias surface

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave B56, 2026-09-13/14. Written incrementally.

Predecessors: `📓️2026-09-13-wave-B54-mutation-latency-2.md` §6.4/§8.2 (the blocking defect),
`📓️2026-09-13-wave-B48-nakagin-selection-lane.md` §6.4 + residuals 1–2 (the alias holding a job),
`📓️2026-09-13-wave-B50-camera-lane-regression.md`, `📓️2026-09-13-wave-B52-reconcile-ladder-livelock.md`,
`📓️2026-09-12-wave-B25-surface-contexts-view-state.md` (the alias made a `Window(last)` binding),
`📓️2026-09-10-fill-build-host-tick.md` §8.29 (where the alias came from).

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, shared live tree. No state-modifying git command,
  no worktree, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, every command foreground. This wave does NOT
  close or reopen the ticket and deletes nothing under `🗑️generated`.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- `RUST_MIN_STACK=134217728` on every law run.
- B53 (export lane) and B55 (probe residuals) live in parallel; peer hunks in the plugin `🦀️.rs` files
  were re-read before every edit and none was reverted.

## 1 What the census actually says — and it is not what the brief supposed

### 1.1 WHICH registry refuses: the aggregate resident CREDIT ledger, priced exactly

`reserve_mounted_owned` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:475`) answered one
opaque word, `registry-reservation-unavailable`, for THREE different refusals inside
`SurfaceReconcileReservation::try_new` (`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:2254`): a zero
generation, the aggregate resident credit ledger, and the handback registry. That is why B48 residual 1 and
B54 §8.2 could name the symptom and not the table. This wave split it into four named reasons and printed
BOTH registries' live occupancy beside every refusal (`registry=` in `PatchTracker::debug_state`).

The answer, measured natively:

```
[DEBUG] credit ceiling=3 surfaces; refusal=slots=[51:surface-0#g1:---:ack0/rev0:outSome(0),
  51:surface-1#g2:---:ack0/rev0:outSome(1), 51:surface-2#g3:---:ack0/rev0:outSome(2)]
  ready=[g1:--r-,g2:--r-,g3:--r-] … unadmitted=3 closing=0 output_fault=none
  reserve_refusal=unmounted:registry-resident-credit-exhausted
  registry=resident=3s/12291i/25875744B of 33554432B handback=378/384
```

**Three.** One render reservation asks for `SurfaceReconcileLimits::default().max_bytes` —
`UI_RESIDENT_SURFACE_BYTES`, 8 MiB (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs:11`) — of a
`UI_RESIDENT_AGGREGATE_BYTES` budget of 4 × that, 32 MiB (`:12`). With the contract's own fixed backing
already resident, **exactly three surfaces can hold a reconcile reservation at one time, process-wide**,
while one puzzle3d instance mounts THIRTEEN. The handback registry is not the constraint and never was:
`UI_RESIDENT_SLOTS * 6` = 384 slots, 378 still free at the moment of refusal.

So `registry-reservation-unavailable` was never a property of the surface it named. **It is whichever
surface asks fourth.** B54 caught `1:window`; this wave's live run caught `1:framework.section.catalogue`
(§1.3). That reframes the blocking defect and is the single most important finding here.

### 1.2 What the `1:window` surface held, and who produced it

| role | site | what it did |
| --- | --- | --- |
| **host mint** | `windowHostContextBindings`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1611` | appended ONE extra binding per refresh: surface `<instance>:window`, carrying the **last** bound window's `bodyKey` and `windowKey` |
| host consumer | `uiRefreshSurfaceEvents`, same file `:1730` | turned every binding into a `surface-visible` event, so the alias was mounted like a real pane |
| host default | `wirePatchSurfaceId`, same file `:1413` | `patch.surface.surface ?? "window"` — routed a body-less patch to the alias instead of refusing it |
| host default | `applyRetainedWindowPatches`, same file `:2803` | `wirePatchSurfaceId(patch) ?? "window"` — same, on the retained-tree path |
| **guest producer** | `Event::JobProgress`, `…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:641` | `UiText::try_format("{}:window", binding.instance)` → dirty |
| **guest producer** | `Event::JobCompleted`, same file `:659` | same |
| **guest producer** | `Event::Message { Backbone }`, same file `:704` | same |
| guest consumer | `SurfaceContexts::insert`, `…/⚛️reactor/🪟️surfaces/🦀️.rs:72` | admitted it as an ordinary `SurfaceRole::Window(last)` binding (wave B25) |
| guest consumer | `PatchTracker` | gave it a slot, a generation, an operation id, a reconcile reservation and an output slot like any real surface |

The three GUEST sites are why the alias existed at all: a spawned job's progress, its completion and a
document-backbone message each owned no surface, so they invented one. The host then had to mount a context
for the invented name (W-G3 §8.29) — and, because that mount carried the last pane's body, it republished
the **180-object world body a fourth time per refresh** under a surface no pane reads, while the real panes
never received the update the dirty existed to deliver. Both halves were wrong.

### 1.3 Live on #61 with the host mint retired — `T/🗑️generated/b56-2026-09-13T00-07-48-w61-host-retired.md`

Diagnostics armed before boot, fresh page, self-verified Nakagin switch (`pickerLabel="Nakagin Capsule
Tower" verified=true`, `instances=180`), one proven pick, one `setCamera`, one `Delete`:

```
[13.17s] example-switch switched=true instances=180 bytes=54254 waitedMs=4524
[186.23s] pick 0.55,0.40 ids=["7dc5b737-…"] waitedMs=61006 ingress=5 settled=1 refresh=2 lane=2
[267.70s] setCamera moved=false waitedMs=64193 ingress=2 settled=0 refresh=1
[403.48s] deleteSelection landed=false before=180 after=180 removed=[] waitedMs=135319
[403.57s] deleteSelection census ingress=4 settled=0 … moreWork=1426 lines=12945
[433.96s] alias census reserveRefusals=668 moreWorkLines=3269
```

and the census itself:

```
[actor] [DEBUG] reactor more-work streak=551 seen=675 sources=["reconcile"] contended=false effects=0
 patches=[slots=[1:puzzle3d-main#g14:--R:ack2/rev2:outNone, 1:puzzle3d-main-top#g15:--R:ack2/rev2:outNone,
   1:puzzle3d-main-perspective#g16:--R:ack2/rev2:outNone, 1:framework.panel.artifact#g17:--R:ack2/rev2:outNone,
   1:framework.panel.catalogue#g18:--R:ack2/rev2:outNone, 1:framework.panel.inspection#g19:--R:ack2/rev2:outNone,
   1:puzzle3d.panel.settings#g20:--R:ack1/rev1:outNone, 1:framework.panel.history#g21:--R:ack2/rev2:outNone,
   1:framework.section.engagements#g22:--R:ack2/rev2:outNone, 1:framework.section.measures#g23:--R:ack2/rev2:outNone,
   1:framework.section.tools#g24:--R:ack2/rev2:outNone, 1:framework.section.catalogue#g12:--R:ack1/rev1:outNone,
   1:window#g13:--R:ack0/rev0:outNone]
  ready=[] terminals=[] producer_terminals=[] deferred=[1:framework.section.catalogue]
  rejected=0 unadmitted=0 closing=0 output_fault=none
  reserve_refusal=1:framework.section.catalogue:registry-reservation-unavailable
  generation_exhausted=false close_cursor=1]
```

Three readings, all of them load-bearing.

1. **The host mint is gone, proven by the alias's own revision.** `1:window#g13:` is `ack0/rev0` — mounted
   as a slot by the guest's dirty and **never rendered, never published, never acknowledged**. Against
   B54's `1:window#g31:--R:ack2/rev2` and B48's `1:window#g14:-J-:ack1/rev0:outSome(0)`, that is the exact
   signature of "the host no longer gives this surface a context": the reserve succeeds, the render faults
   (`surface 1:window has no host context`), the grant is cancelled, the slot sits at revision 0. **The
   alias no longer republishes the world body and no longer holds an output slot** — 25 % of every world
   publication is gone, as B48 residual 2 predicted. The three guest sites that still dirty it ride #62.
2. **The refusing surface is NOT the alias.** `reserve_refusal=1:framework.section.catalogue:…`, 668
   refusals across the run. The refusal is surface-agnostic, exactly as §1.1's arithmetic says it must be.
3. **`deleteSelection` still does not land** (135 s, 180 → 180 objects, `settled=0`, `moreWork=1426`).
   **Retiring the alias is necessary and it is not sufficient**, and the reason is now named rather than
   suspected: the reconcile credit ceiling is THREE and the session mounts thirteen surfaces.

`hostContextFaults=0` in that tape is an instrumentation gap, not a contradiction: the render fault is
raised as a `shell_fault_effect` (`…/🔄️turn/🦀️.rs`, the `Err(fault)` arm of the dirty-render match) and
reaches the shell's fault surface, not the console. The `ack0/rev0` slot is the proof instead.

## 2 The retirement — every producer and consumer, and what replaces it

### 2.1 Host: no synthetic surface is ever minted or defaulted to

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`

- `DEFAULT_LEFTOVER_WINDOW_SURFACE` — **deleted**, including from the `testState` export.
- `windowHostContextBindings` — binds one entry per authored window instance the host view carries, and
  nothing else. The `alias` local and its trailing `push` are gone.
- `wirePatchSurfaceId` — `patch.surface.surface` must be present and non-empty; otherwise the patch names
  no surface and its caller refuses it (`plugin-ui.projection-surface-required`, `acceptUiPatches`). The
  `?? "window"` default is gone, so a leftover Viewport patch without an instance is **refused with a named
  fault** rather than routed to an alias.
- `applyRetainedWindowPatches` — the second `?? "window"` is gone; a body-less patch is refused with a
  named `[DEBUG]`-prefixed warning and the previously retained body is kept.

### 2.2 Guest: background work names the real instance

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs`

- `SurfaceContexts::insert` **refuses** the synthetic name: a surface whose body part is `window`, for a
  view that carries window instances, none of which is itself called `window`, is rejected with
  `a surface named \`window\` is synthetic: an app with window instances is addressed by instance`. An app
  that genuinely authors a window instance named `window` still mounts it — the refusal is about a name no
  instance claims, so it can never false-positive.
- `SurfaceContexts::background_surfaces()` — every mounted window instance, or (for an instance that owns no
  window) its app-level PANELS. That is the document/panel scope the brief asks for when a leftover
  genuinely has no window, and it is never a window.
- `SurfaceRole` needed no change: wave B25 had already made the alias an ordinary `Window(last)` binding, so
  there was no alias ROLE to remove — only the synthetic NAME, which is now refused at the door.

`…/🔌️plugin/🦀️.rs` — `plugin_instance_background_surfaces` exposes that roster to the reactor (empty for an
instance with nothing mounted, so a job completing before the first refresh dirties nothing at all instead of
inventing a surface).

`…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — the three `"{}:window"` mints are replaced by one
`dirty_background_surfaces(runtime, instance, &mut dirty)` helper at all three sites (`Event::JobProgress`,
`Event::JobCompleted`, `Event::Message { source: Backbone }`). This also **fixes a second defect in passing**:
job progress and backbone messages now reach the real panes, which they never did while they dirtied a
surface no pane reads.

### 2.3 The leftover encode carries the instance

`…/🔌️plugin/🦀️.rs` `leftover_interaction_view_from` takes the addressed window instance
(`meta.view_state.window_id`) and emits it as `interactionView.windowId`; `None` for a windowless action is
the document scope, never a synthetic window. Host side,
`🛠️ShellHelpers/🟦️.tsx` `interactionViewFromLeftoverOutput` decodes `windowId` into
`LeftoverInteractionViewV1`, and `🏛️ShellHost/🟦️.tsx` `applyLeftoverInteractionView` prefers the addressed
window and falls back to the leftover's OWN instance before falling back to document scope — so the pane a
leftover belongs to is never inferred.

### 2.4 `default_window_surface_has_host_context` — adjusted, not deleted

Renamed `default_window_surface_has_host_context_and_the_synthetic_name_is_refused`. The DEFAULT-window
semantics are kept verbatim (a real default window surface resolves its instance, projects its own host view
and retains it); the second half now requires the synthetic name to be refused and the surface to have no
context to render from. Its fixture keeps the same shape with `id` pointing at a real instance (`7:right`)
plus `syntheticId`/`syntheticRefusal`.

## 3 Laws

| law | file | what it pins |
| --- | --- | --- |
| `binds only real window instances and refreshes Inspection when leftover selects` | `…/🧑‍🎨engine/🧪️tests/🔬️window-host-context/🟦️.ts` | for an app whose host view carries window instances, the binding table has one entry per carried instance, **no** entry named `window`, and no duplicate surface — oracle and implementation agree on the same fixture |
| `every_mounted_surface_renders_against_its_own_view_state_while_one_pick_reaches_every_body` | `…/🔌️plugin/🧪️tests/🔬️surface-view-state-routing/🦀️.rs` | **no surface named `window` is ever mounted for an app with window instances**: the three real surfaces mount and render against their own view, and `plugin_mount_surface("1:window", …)` is a named fault whose message quotes the rule; the refused surface then has no host context to render from |
| `default_window_surface_has_host_context_and_the_synthetic_name_is_refused` | `…/⚛️reactor/🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle/🦀️.rs` | a REAL default window surface keeps its host context and projection; the synthetic name is refused and leaves nothing behind |
| `background_surfaces_name_every_window_then_fall_back_to_panels` | same file | background work addresses every mounted window instance, falls back to app panels when there is no window, and never names a `:window` surface |
| `the_refused_reconcile_reservation_names_the_resident_credit_ledger_not_the_handback_registry` | `…/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` | the refusal names the TABLE, the credit ceiling is a handful of surfaces rather than the slot count, and the handback registry still has headroom when credit runs out — the instrument B48 residual 1 asked for |
| `two_hundred_publications_leave_the_reconcile_registries_holding_nothing` | same file | **after 200 leftover-producing publications the registry holds zero reservations** — no credit, no slot, no handback; and no single round is ever allowed to hold more than one reservation's worth |
| `interaction_select_leftover_window_instance_rides_the_encode` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | a window-addressed `interactionSelect` leftover encode names the instance it was dispatched for, for BOTH puzzle3d panes |
| `an_alias_window_surface_the_host_never_acknowledges_does_not_block_the_retirement_ladder` | `…/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` | wave B52's law, kept unchanged — it is about an unacknowledged surface in general, which survives the alias |

## 4 Verification — every command foreground, tails quoted

### 4.1 Framework plugin laws, against a pre-edit baseline of the same filter

BEFORE (`🗑️generated/b56-baseline-plugin.txt`, taken before any Rust edit):

```
running 22 tests
… test component::reactor::surface_context::tests::default_window_surface_has_host_context ... ok
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 670 filtered out; finished in 0.20s
```

AFTER (`🗑️generated/b56-after-plugin.txt`):

```
running 23 tests
test component::reactor::surface_context::tests::background_surfaces_name_every_window_then_fall_back_to_panels ... ok
test component::reactor::surface_context::tests::default_window_surface_has_host_context_and_the_synthetic_name_is_refused ... ok
…
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 671 filtered out; finished in 0.21s
```

Name diff: `default_window_surface_has_host_context` → `…_and_the_synthetic_name_is_refused` (adjusted, not
deleted), plus one new law. Nothing dropped, nothing newly failing.

Final combined run of every law this wave touches or adds (`🗑️generated/b56-final-plugin.txt`):

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- surface_context reserved_undo leftover two_hundred_publications the_refused_reconcile every_mounted_surface --test-threads=1
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 671 filtered out; finished in 0.36s
```

### 4.2 The two registry laws

```
running 1 test
test component::reactor::patches::tests::the_refused_reconcile_reservation_names_the_resident_credit_ledger_not_the_handback_registry ...
  [DEBUG] credit ceiling=3 surfaces; refusal=… reserve_refusal=unmounted:registry-resident-credit-exhausted
  registry=resident=3s/12291i/25875744B of 33554432B handback=378/384 …
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 0.00s
```

```
test component::reactor::patches::tests::two_hundred_publications_leave_the_reconcile_registries_holding_nothing ...
  [DEBUG] 200 publications returned every reservation: SurfaceReconcileRegistryCensus {
    resident_items: 0, resident_bytes: 709920, resident_slots: 0, resident_aggregate_bytes: 33554432,
    handback_free: 384, handback_slots: 384 }
ok
```

`resident_slots: 0` and `handback_free == handback_slots` after two hundred publications: **the publication
path itself leaks nothing.** The `resident_bytes: 709920` residue is the contract's own fixed backing, which
is the baseline the law compares against, not a leak.

### 4.3 puzzle3d

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
warning: `semio-s-artifact-puzzle-3d` (lib) generated 95 warnings …
    Finished `dev` profile [unoptimized] target(s) in 57.29s          → 0 errors, 122 warning lines
```

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- leftover window --test-threads=1
test editor::puzzle3d::component::unit_tests::interaction_select_leftover_window_instance_rides_the_encode ... ok
…
test result: FAILED. 54 passed; 1 failed; 0 ignored; 0 measured; 700 filtered out; finished in 5.80s
```

The one failure is **pre-existing and not this wave's**:
`the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind` fails on its LAST assertion
(`with nothing focused the panel falls back to the roster` expects `puzzle3d-main`, gets
`puzzle3d-main-top`). Established three ways: it fails when run **alone** under its own filter; both the
assertion (`git show HEAD:…` still contains it verbatim) and the implementation it tests
(`puzzle3d_addressed_window_id`, `…/✏️editor/🦀️.rs:966`, whose roster clause deliberately SKIPS the base
window kind — `!puzzle3d_window_id_is_kind(id)`) are unmodified from HEAD; and it exercises no path this
wave touched (`ViewModel::for_panel` + `focused_window_id` + roster, no surface contexts, no leftover
encode, no patch routing).

### 4.4 Wasm target and the host half

```
cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
warning: `semio-framework-plugin` (lib) generated 21 warnings …
    Finished `dev` profile [unoptimized] target(s) in 5.58s           → 0 errors
```
(`libsemio_framework_plugin-*.rmeta` under `wasm32-wasip2` is newer than `⚛️reactor/🔄️turn/🦀️.rs`, so the
guest half really was type-checked for the wasm target.)

Host TypeScript: `bun x tsc --noEmit -p tsconfig.json` in the renderer-react package reports **763
pre-existing errors repo-wide** (that tsconfig sweeps every plugin and the demonstrator), and **zero of them
mention `windowId`, `LeftoverInteractionView`, `wirePatchSurfaceId`, `windowHostContextBindings` or
`applyUiPatchToRetained`** — this wave's host edits introduce no new type error. A LATER re-run of the same
command aborts after two errors, both in the repo-root `📜️script.ts`, where a peer's in-flight refactor
(134 insertions / 1 138 deletions against HEAD) has moved an `import` above the shebang
(`TS18026: '#!' can only be used at the start of a file`). That blocks the full sweep, so the final
`applyUiPatchToRetained` signature tightening (its `surface` is a required `string` now, and a patch naming
none is refused as desynced instead of inventing `window`) is covered by call-site inspection rather than by
tsc: the one production caller narrows the surface to non-null with an early `continue` before the call, and
every `🔬️engine-contract` caller passes a string literal.

The react vitest lane could not be run as a lane: a peer is mid-relocation of its config
(`📋️project.json`'s `test` target runs `📜️script.ts test`, which now points at
`…/🧑‍🎨engine/🧪️tests/🎚️config/🟦️.ts`, and both that file and the old `vitest.config.ts` are absent as of
01:23 — `nx run …:test` consequently collects ONE file and five tests). **Reported, not worked around.** The
`🔬️window-host-context` law was executed instead through `T/🔍️b56-window-alias.ts`, which drives the same
fixture and the same assertions:

```
[DEBUG] b56 window-host-context bindings=2 surfaces=["puzzle3d-main-top","puzzle3d-main-perspective"] synthetic=0
```

## 5 Live evidence and what rides wasm #62

**Live now (vite-live host half, :6013, wasm #61 guest)** — §1.3: the alias is no longer minted, no longer
rendered, no longer published and no longer holds an output slot (`1:window#g13:--R:ack0/rev0:outNone`
against B54's `ack2/rev2` and B48's `-J-:ack1/rev0:outSome(0)`). The world body is published three times per
refresh instead of four.

**Rides #62** (all Rust): the four named refusal reasons and the `registry=` occupancy field in the reactor
census; `SurfaceContexts::insert`'s refusal of the synthetic name; `background_surfaces`; the three reactor
background-dirty sites; `plugin_instance_background_surfaces`; `leftover_interaction_view_from`'s `windowId`.
Recipe unchanged from B54 §8.3: `bun nx run @semio-tech/puzzle-plugin:component-dev` then
`:materialize-dev` with `CARGO_PROFILE_WASM_DEV_DEBUG=false`, then
`bun 🔍️b56-window-alias-census.ts --port=6013 --label=w62-after`.

**What #62 must show**, and the honest prediction: `hostContextFaults` 0 with **no `1:window` slot in the
census at all**, and `registry=resident=…` printed beside every refusal. It will **not** by itself show
`reserve_refusal=none` on the large document, because §1.1 and §1.3 prove the refusal is not the alias's:
with the alias gone the session still mounts twelve surfaces against a credit ceiling of three.

## 6 Residual — the real blocker, now named and instrumented

**`UI_RESIDENT_AGGREGATE_BYTES` (32 MiB) admits only THREE concurrent reconcile reservations of
`UI_RESIDENT_SURFACE_BYTES` (8 MiB) each, while one puzzle3d session mounts thirteen surfaces.** Every
surface that asks fourth is refused `registry-resident-credit-exhausted`, `reserve_mounted` hands it back,
the turn defers it, `redirty_acknowledged_deferred_surfaces` re-dirties it, and the reactor answers
`more-work` with `effects=0` for ever. That is B54 §8.2's blocking defect, restated as arithmetic rather than
as a property of one surface — and it is why §1.3's `deleteSelection` still did not land with the alias
retired.

Three candidate fixes, for the next wave to choose between with the census in hand:

1. **Price the reservation by the surface, not by the ceiling.** `SurfaceReconcileLimits::default()` asks for
   the maximum a surface may EVER hold; a reservation sized to the tree actually being reconciled would let
   thirteen small panels and one large world body coexist. The `try_shrink` path already exists — it runs
   AFTER the reservation, which is exactly too late.
2. **Admit reconciles behind an explicit concurrency gate** rather than letting them race the credit ledger,
   so a refusal becomes a queue position instead of a defer/re-dirty spin.
3. **Raise the aggregate** — the cheapest and the least principled, and it only moves the ceiling.

Smaller, recorded:

4. The render fault for a dirty surface with no host context reaches the shell fault surface but **not the
   console** (§1.3), so `hostContextFaults` counts 0 where the slot's `rev0` is the only evidence. A
   `[DEBUG]`-prefixed console tap on that arm would make the guest producers self-reporting.
5. The react vitest lane is broken by a peer's in-flight config relocation, and the repo-root `📜️script.ts`
   currently does not parse (§4.4). Both are peer work in flight, reported rather than worked around.
6. `applyUiPatchToRetained`'s last-resort `?? "window"` was the fourth synthetic-name site and is also gone;
   a patch that names no surface is now refused as desynced. No `"window"` surface name remains anywhere in
   the host (`rg '?? "window"'` → 0) or the guest (`rg '{}:window'` → 0).
