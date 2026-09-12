# Interaction Coverage — World3dHost, generation3d selection commands, `send-message` (2026-09-12)

Closes gaps **#3, #4, #6** of `📓️audit-hover-selection-2026-09-12.md` §8 (and its §5.3 items 1–2, §7).
Every result below was **run**, in the foreground; the raw logs are in
`🗑️generated/interaction-coverage/`. Nothing in this lane started a wasm build or a dev server.

---

## 0. Headline

| Gap | State | Proof |
|---|---|---|
| #3 — no interactive vitest on `World3dHost.tsx` | **closed** | 8 mounted jsdom laws + a 28-check Node oracle over one shared fixture |
| #4 — rotate/scale/delete-selection have no dispatch-and-assert tests | **closed** | 3 new `🧪️tests/🔬️unit/🦀️.rs` dirs, 8 new Rust laws |
| #6 — `send-message` dropped unmapped in `wireEffectToFriendly` | **closed, not dead** | root-caused (double-handling, not lost work), mapped explicitly, 10 fixture-driven vitest laws |

**Two live defects found and fixed while doing it** (both made existing tests pass *vacuously* or fail):

1. **Four Rust laws about `interactionSelect` were red or vacuous** because `interactionSelect` is a
   framework-reserved **tool job** — `handle_action` only ADMITS its `Effect::SpawnJob`, and
   `testkit::settle` (a *typed-operation* settle) cannot drive it. This is the same
   admission-vs-publication trap `testkit::dispatch`'s own doc comment already warns about for typed
   commands. Fixed with a new `testkit::select_graph` that drives the reserved job. **It was NOT the
   `instance-owner-poisoned` FlowHost ownership defect the brief warned about — `FlowHost`
   (`history_store_from_baseline` / `evaluate_step` / `set_neuron_params`) was not touched and needed
   no change.**
2. **The repo's own DOM-test boundary silently dropped pointer coordinates and click modifiers**
   (`🖌️render.ts`), so no pointer-driven law in the repo could have proved anything about a drag or a
   modifier-click. Fixed at the boundary.

---

## 1. Gap #3 — `World3dHost` interactive vitest

### 1.1 What was added

| File | Role |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` | **new** — the language-agnostic gesture→action law |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖱️world3d-interaction/🟦️.tsx` | **new** — mounts the real host in jsdom and plays the fixture |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts` | `world3dPointerGestureOracle` (Node twin) + `world3d-interaction-check` |
| `…/🎯️targets/⚛️react/vitest.config.ts` | registers the suite in `engineTestSuites` |
| `…/🎯️targets/⚛️react/📋️project.json` | `world3d-interaction-check` target |
| `.vscode/launch.json` | `⚖️gate🖱️world3d-interaction🌐️renderer`, order `411.099849` |

### 1.2 How the host is actually mounted

The audit's §5.3 item 1 was right that only `renderToStaticMarkup` snapshots existed. A live jsdom
mount is possible, but only if the WebGL seam — and **only** that seam — is replaced:

- `@react-three/fiber` is **partially** mocked (`importOriginal` + override): `useFrame`/`useLoader`
  become inert and `useThree` hands out a **real `three.PerspectiveCamera`** built from the fixture's
  own pose. `CameraRefBridge` then populates `cameraRef`, so `resolveClickInstanceId`,
  `resolveMarqueeInstanceIds` and `projectWorldPoint` run the **host's real projection math**, not a stub.
- `@semio-tech/infinite-world-r3f`'s `WorldCanvas` becomes a `<div>` that captures `onPointerMissed`,
  and `WorldOrbitGated` captures `onCamera` — the two callbacks a real canvas would invoke.
  `WorldOrbitViewControls`/`WorldProjectionRig`/`WorldLodBridge`/`WorldVolumeLayer`/`WorldReferenceLayer`
  become pass-throughs. Every pure export of that package stays real.
- Everything under test — `handleInstancePointerDown`, `dispatchInstanceHover`, `handleEmptyClick`,
  `handleCameraChange`, `handlePointerDown/Move/Up`, `finalizeMarqueeSelection` — is the host's own code.
- Instance meshes render as real DOM nodes (r3f intrinsics degrade to custom elements that still
  carry React's `onClick`/`onPointerMove`), so a pick is a real DOM event on a real per-instance node.

**Why `<Canvas>` itself cannot mount**: jsdom has no `ResizeObserver`, and `react-use-measure` (inside
r3f's `CanvasImpl`) throws outright. Polyfilling it would only get us to a real WebGL request.

### 1.3 The 8 laws (all green)

1. one pickable mesh per scene instance
2. **surface identity** — consumes `🪪️world-surface-identity.json`: both panes of one document publish
   their own `data-surface-id` / `data-window-instance-id` (peer wave B20's law)
3. instance pick → `interactionSelect`, `merge: "replace"`, `method: "pick"`, target
   `[{granularity:"object", id:"extrude@solid"}]` — the **topology** id (`interactionId`), never the render id
4. shift-pick → same, `merge: "add"`
5. instance pointer-move → `interactionHover`, `channel:"pointer"`, granularity `"handle"` (the
   scene's own `domainGranularityId`, **not** the pick's `"object"`)
6. background click (`onPointerMissed`) → `interactionSelect` with an **empty** target list
7. completed orbit → **exactly one** `setCamera` after `CAMERA_SYNC_DEBOUNCE_MS`, carrying the **last**
   pose nested under `camera` (two `onCamera` steps, one dispatch)
8. marquee release (pointerdown → 2 moves → pointerup over the viewport) → `interactionSelect`,
   `merge:"replace"`, with the **deduplicated** topology targets (two rendered instances of one channel
   collapse onto one target, proving `interactionTargetsForInstances`)

### 1.4 The third-party twin

`world3dPointerGestureOracle` (`📜️script.ts`, the same convention as the existing
`directoryHomeBootstrapOracle` / `scopedPresenceOracle`) reads the **same JSON** with `node:assert`,
no React / no three / no DOM, and:

- re-implements `marqueeModeFromModifiers`+`instanceMergeArg` and `interactionTargetsForInstances`
  **from the documented law** and asserts every gesture's declared `merge` and `targets` match;
- asserts a pick reports `object` granularity while a hover reports the scene granularity, an empty
  click carries `[]`, a camera gesture keeps only the LAST pose in exactly one dispatch, and a marquee
  path exceeds the drag threshold;
- pins the **source call sites** (`dispatch("interactionSelect", world3dSelectionActionArgs(…))` etc.
  verbatim, plus the two wire-shape return literals) so a refactor cannot re-point a gesture at a
  different verb while the mounted suite still passes on a stale mock;
- asserts `graphPointerDown` is dispatched nowhere in the host.

```
world3d-pointer-gesture-oracle: checks=28 clean
 Test Files  1 passed (1)
      Tests  8 passed (8)
```

### 1.5 Live defect fixed: the DOM-test boundary dropped pointer state

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🖌️render.ts` — the repo-owned test
boundary — had **two** holes that made pointer-driven laws unwritable:

- `click(target: Element)` took **no** `eventInit`, unlike every sibling method, so a modifier-click
  could only ever exercise the unmodified `replace` branch. *(Measured: the additive-pick law failed
  with `expected 'replace' to be 'add'` until this was fixed.)*
- jsdom ships **no `PointerEvent`**, and `@testing-library`'s `createEvent` picks its constructor by
  event-type name — so `pointerDown`/`pointerMove`/`pointerUp`/`pointerCancel` all degraded to a bare
  `Event` carrying no `button`, `clientX`/`clientY` or `pointerId`. `World3dHost.handlePointerDown`
  opens on `event.button !== 0`, read `undefined`, and returned: **the marquee never even started**,
  and the law would have read green while proving nothing. Fixed with a guarded `PointerEvent`
  polyfill (registered once, only when the environment genuinely lacks the class).

Both fixes are additive; the full `SEMIO_TEST_LEVEL=long` renderer corpus shows **no new failures**
(see §4).

---

## 2. Gap #4 — rotate / scale / delete-selection unit tests

New dedicated test directories mirroring `translate-selection`'s, each declared with a
`//#region 🧪️Tests` `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;`:

| Command | Laws |
|---|---|
| `🔄️rotate-selection` | accumulates ONE `brep.xform.rotate` neuron (second grab adds `π/2` to `π/2`, count stays 1); an **ids-less** rotate transforms the framework-owned `graph` selection; a rotate naming nothing changes nothing |
| `📏️scale-selection` | ONE uniform `brep.xform.scale` neuron, factors **multiply** (`2.0` then `×3.0` → `6.0`), and the three axis factors average into one `factor`; ids-less scale reads the `graph` selection; no-target scale is a no-op |
| `❌️delete-selection` | removes the `graph`-selected widget **and prunes its synapses**; an EMPTY graph selection removes **nothing** (the destructive-row law) |

`rotate_and_scale_selection_persist_into_flow_graph` was **moved out** of translate-selection's test
file into the two commands it belongs to (CLAUDE.md: repeated code lives next to what it tests) and
strengthened from "a neuron exists" into the accumulate/multiply laws above.

### 2.1 Live defect fixed: `interactionSelect` is a reserved tool job, and four laws never drove it

The brief flagged a possible `instance-owner-poisoned` `FlowHost` ownership defect. **That is not what
this was.** `FlowHost` needed no change. The real cause:

`interactionSelect` is dispatched as `ArtifactReservedToolJob::new(FrameworkInteractionSelectJob…)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:14678`). `handle_action` therefore returns an
**admission** carrying `Effect::SpawnJob{kind: framework.reserved.tool}` — exactly what the browser
console shows (`spawn-job routed kind=framework.reserved.tool job=65`). The selection only reaches
`protocol::InteractionState` once the host **drives** that job
(`settle_framework_reserved_admission`, which `start_job`/`step_job`s it). `testkit::settle`
(= `settle_registered_typed_operation`) settles a *typed operation* and returns a clean,
fault-free receipt without ever touching the reserved spawn.

Result: four laws read `selection.get("graph") == None`:

- `component::tests::generation3d_interaction_selection_owns_its_persisted_history` — **red**
- `component::tests::context_menu_reads_the_framework_owned_graph_selection` — **red**
- `component::fold_contract::interaction_select_publishes_through_the_retained_typed_path` — **red**
- `component::work_capacity::interaction_select_passes_the_reserved_preflight` — **red**

All four were red at HEAD **before** any of my edits (verified by running
`generation3d_interaction_selection_owns_its_persisted_history` alone, on its own, before touching
anything — `left: None, right: Some(["height"])`). §5 of the hover audit cites the first two as live
passing evidence; they were not.

**Fix** — one new testkit helper, used by all six call sites (four above + my three new ids-less laws):

```rust
// ✏️editor/🧪️tests/🔬️testkit/🦀️.rs
pub async fn select_graph(app: &mut Generation3dApp, granularity: &str, ids: &[&str]) -> InvocationResult
```
It dispatches the real `interactionSelect` on `GENERATION_3D_INTERACTION_DOMAIN` and then
`settle_framework_reserved_admission`s the job, with a doc comment naming the admission-vs-publication
trap so the next author does not re-introduce it.

### 2.2 Result

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- interaction_select selection --test-threads=1
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out
```
(8 new command laws + the 4 repaired laws + the pre-existing selection corpus.)

---

## 3. Gap #6 — `send-message` effects on the `interactionSelect` route

### 3.1 Root cause: double-handling, not lost work

Traced from the ticket's own console captures
(`🗑️generated/console-dump/console.txt:54-59`, `probe-restage-1/console.txt:53-57`):

```
spawn-job routed kind=framework.reserved.tool job=65 inline=1
job done kind=framework.reserved.tool job=65 status=done steps=2
job-completed leftover job=65 effects=send-message,send-message
job-completed leftover frame instance=1 kind=Invocation history=5      ← CONSUMED here
wireEffectToFriendly: unmapped effect "send-message" dropped  ×2        ← re-offered, then warned
performInvocation settled {"actionId":"interactionSelect","effects":1}
history patch applied {...,"labels":["Select", …]}                      ← the payload DID land
```

The chain:

1. `route_app_frame` (`⚛️reactor/🔄️turn/🦀️.rs:1417-1452`) wraps **every** non-`UiPatch` `AppFrame`
   reply in `Effect::SendMessage{Shell{instance}}`.
2. `deliverJobCompletionTurn` (`🔌️PluginRuntime/🟦️.tsx:2032-2035`) parks the reserved job's turn
   effects in `pendingTurnEffects` **without** the `shellFrameBytes` split its two sibling consumers
   (`runQueuedTurn`, `drainTypedOperations`) perform — deliberately, because that reply belongs to the
   ORIGINAL invocation, not to a later `turnOutcomes` push.
3. `performInvocation` drains it into `invocationFromFrames`, which **applies** those frames through
   `leftoverShellInvocationFrames` (that is where `history=5` comes from, and the `history patch
   applied` line one entry later proves it) — and then hands the **same list** to
   `wireEffectToFriendly`, whose `default:` arm warned and dropped them.

So the two effects were **already consumed**; the warning was a false alarm caused by mapping
transport frames a second time. The emitter is **not dead** and must not be deleted.

### 3.2 Fix

`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` gains a declared contract:

```ts
export const WIRE_SEND_MESSAGE_ROUTED_TARGETS: readonly string[] = ["shell", "backbone"];
export function wireSendMessageTargetTag(effect: WireVariant): string | null
export function isRoutedWireSendMessage(effect: WireVariant): boolean
```
and an explicit `case "send-message":` in `wireEffectToFriendly` that returns `null` **silently** for a
`shell` or `backbone` endpoint (transport, already owned — the friendly `Effect` union in
`🎠️kernel/🟦️.ts` deliberately declares no `sendMessage` member) and **warns loudly, naming the
endpoint**, for anything else. `🔌️PluginRuntime/🟦️.tsx`'s own copy of `wireEffectToFriendly` imports
those three helpers from `🖼️wire-turn.ts` (it already imported `wireExtensionInvocation` from there) and
carries the same case — no fourth copy of the rule.

**No silent drop remains**: a `shell`/`backbone` send-message is a declared, documented no-projection
(with a named consumer); a `topic`/absent endpoint has no host route at all and now says exactly that
instead of hiding behind "unverified wasm-boundary conversion".

### 3.3 Test

`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧫️fixtures/📨️effect-wire-routes.json` +
`🧰️framework/🔨️modules/🎭️actor/🧪️tests/📨️effect-wire-routes/🟦️.ts`, wired as an in-source
(`import.meta.vitest`) suite on `🖼️wire-turn.ts` (added to `includeSource` **and** `coverage.include`;
the config's own doc comment warns a file absent from `includeSource` never runs while still reporting
green). Seven fixture rows — shell-for-this-instance, shell-for-another-instance, backbone, topic,
endpoint-less, a genuinely unknown tag, and a `notify` control — each pinning the friendly projection,
the `shellFrameBytes` payload, **and whether `console.warn` fires**, plus a law that a routed
send-message is never reported as `unmapped`.

```
🖼️wire-turn.ts > 📨️ send-message effects at the wire→friendly boundary
 Test Files  1 passed (1)      Tests  10 passed (10)
```

### 3.4 Left alone (flagged, not mine)

`leftoverClipboardWriteEffects` (`🔌️PluginRuntime/🟦️.tsx:2652-2657`) still logs
`[DEBUG] leftover clipboard-write missing tags=send-message,…` whenever a leftover list contains any
send-message and no clipboard-write — a peer's debugging aid that fires on the same false premise.
It drops nothing; not touched.

---

## 4. Everything that was run

| Command | Result |
|---|---|
| `bun ./📜️script.ts world3d-interaction-check` (react target) | **oracle 28 checks clean; 8/8 vitest** → `🗑️generated/interaction-coverage/world3d-interaction-check.txt` |
| `bun x vitest run --config vitest.config.ts 🖼️wire-turn` (actor pkg) | **10/10** → `…/actor-wire-turn.txt` |
| `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- interaction_select selection --test-threads=1` | **25 passed, 0 failed** → `…/generation3d-selection-tests.txt` |
| `SEMIO_TEST_LEVEL=long bun x vitest run` (whole react renderer corpus) | **906 passed, 10 failed** — the same 10 as before my first edit (see below) |
| `cargo test … --lib` (whole crate, both parallel and `--test-threads=1`) | **355 passed, 9 failed** — same 9 both ways, all pre-existing |
| `bun ./📜️script.ts typecheck` (react target) | the two errors my files introduced were fixed; the rest is pre-existing repo-wide noise |

### 4.1 Pre-existing failures I did **not** touch (peer churn, verified unrelated)

React renderer corpus (identical set before and after my edits):
`🔬️engine-contract` — `world3dContextMenuSurfaceV1 is not a function` (a peer changed its signature)
and `buildNoteShellCommandAction` (a peer added `inverseArgs`/`inverseCommandId`);
`🧩️package-integration` — 6 worker-byte-identity/Bun-pin rows;
`🔌️PluginRuntime` in-source — `binds two instances of one body…` and `readAppDocumentPack()` (a peer
added an `ops` field).

generation3d crate (identical set under `--test-threads=1`, so not contention from my new tests):
`add_generation_records_an_undoable_generation_operation`,
`an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain`,
`generation_preview_is_one_app_transient_shared_by_two_generation_windows`,
`two_instances_converge_disjoint_widget_moves`, `undo_redo_round_trips_flow_graph_edits`,
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`
(`generation3d-publication.contended`), `refresh_pending_effects_arms_flow_eval_tick_chain` /
`switching_active_example_changes_preview_meshes` (the two swap between runs — flaky, and the second
is the `meshes=0` contributions-starvation gap #1, another lane's).

---

## 5. Gaps this lane did NOT close

- **#1** contributions push starved (`meshes=0`) — another lane's; still blocks every browser proof.
- **#2** wgpu `World3d` tests overflow the default 2 MiB native stack — untouched; still masked by the
  128 MiB `RUST_MIN_STACK` floor.
- **#5** per-example browser punchlist — blocked behind #1.
- **#7** viewer (`👁️viewer/**`) hover/selection runtime evidence — untouched.
- The nine pre-existing generation3d crate failures in §4.1 are logged, not fixed.

---

## 6. Files touched (absolute)

```
/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json          (new)
/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖱️world3d-interaction/🟦️.tsx                                  (new)
/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts
/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts
/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📋️project.json
/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🖌️render.ts
/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts
/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧫️fixtures/📨️effect-wire-routes.json                                       (new)
/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts
/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🧪️tests/📨️effect-wire-routes/🟦️.ts                                                                (new)
/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🧪️tests/🔬️unit/🦀️.rs   (new)
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️scale-selection/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️scale-selection/🧪️tests/🔬️unit/🦀️.rs    (new)
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❌️delete-selection/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❌️delete-selection/🧪️tests/🔬️unit/🦀️.rs   (new)
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️translate-selection/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/.vscode/launch.json
```
