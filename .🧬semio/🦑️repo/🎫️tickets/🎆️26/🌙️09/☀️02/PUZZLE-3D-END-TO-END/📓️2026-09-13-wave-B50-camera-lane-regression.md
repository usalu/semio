# Wave B50 — the camera lane regression on wasm #59

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave B50, 2026-09-12 (report file dated 2026-09-13 per the
coordinator's naming). Written incrementally while the wave ran.

## 0 The regression as handed over

Full battery #59 (`🗑️generated/battery-2026-09-13-59-6013.txt`) reports, three times over:

```
camera settle puzzle3d-main-perspective moved=false waitedMs=30184
```

for orbit, pan AND zoom, plus focus/zoom via the context menu, while #57 and #58
(`battery-2026-09-13-57-6013.txt`) moved in about a second (`moved=true waitedMs=1213/951/670`).
Picks still land, the veil cleared at 48 s, `FAULTS=0`.

## 1 First fact: the host half is NOT frozen between #58 and #59

The coordinator's framing is "wasm #59 = #58 + B46's guest changes". That is true of the GUEST half
only. The host half is served by a **Vite dev server** off the live repo, so every host edit between
the two battery runs is in the #59 measurement as well:

| artefact | mtime |
| --- | --- |
| `🗑️generated/battery-2026-09-13-57-6013.txt` | Sep 12 16:54:13 |
| `🗑️generated/battery-2026-09-13-58-6013.txt` | Sep 12 17:21:22 |
| `…/🌐️World3dHost/🟦️.tsx` | **Sep 12 18:34:21** |
| `🗑️generated/battery-2026-09-13-59-6013.txt` | Sep 12 18:48:31 |

So `🌐️World3dHost/🟦️.tsx` was rewritten 14 minutes before the #59 battery and after both green
batteries. B46's own host hunks (pick bounds, overlay `activeObjectId`) landed around 15:50–16:20 and
are therefore already inside the GREEN #57/#58 runs — they cannot be the regression. The host edit at
18:34 is a live suspect for the whole of it.

## 2 The hop table, measured on #59 (host live off the repo, guest = the served wasm)

Probe `🔍️b50-camera-lane.ts` (this wave's input file; runs on a fresh page with
`SEMIO_RUNTIME_DIAGNOSTICS` armed in `addInitScript`, so the permanent refresh-lane taps print).
Outputs `🗑️generated/b50-2026-09-12T17-01-04-repro1.md` and `…-slots.md` / `.ndjson`.

The served guest IS #59: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/dist/component-release/semio_s_plugin_puzzle.wasm`
is Sep 12 18:23:18 and the materialized core module
`…/🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules/🧩️puzzle/semio_s_plugin_puzzle_component.core.wasm`
is Sep 12 18:24:22.

| # | hop | evidence on #59 | verdict |
| --- | --- | --- | --- |
| 1 | gesture → `OrbitControls` rig | `pan-shift-right rigMoved=true rigBefore={"position":[9.6468,-1.9008,4.6119]…} rigAfter={"position":[7.9605,2.9068,7.7309]…}`; `zoom-wheel rigMoved=true` | **works.** `data-viewport-camera-json` moves on every pan/zoom. (Alt+right orbit moved the rig in run 1 and not in run 2 — secondary, §6) |
| 2 | rig → debounced `setCamera` dispatch | `[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"setCamera"}` + `[DEBUG] command ingress lane {"actionId":"setCamera","seq":25,"lane":"Interactive"}` | **works** |
| 3 | host → guest exchange | `[DEBUG] plugin_exchange actionId=setCamera branch=catalog` and the guest's own `[DEBUG] puzzle3d.utility.publish action=setCamera window=Some("puzzle3d-main-perspective") utility= map_hit=false` | **works — the guest runs the arm** |
| 4 | settle | `[DEBUG] performInvocation settled {"actionId":"setCamera","frames":2,"frameKinds":["Invocation","Ephemeral"],"historyCursor":null,"historyUpserts":0,"historyCanUndo":null,"effects":0}` | **settles, no fault, no history row** (which is correct — `setCamera` is `ActionKind::View` on the WindowConfig lane only) |
| 5 | host asks for the window bodies | `[DEBUG] refreshUi lane {"decision":"owed"…}` / `"pass"` with `windowBodies:["puzzle3d.play.composite"]`, and once with `{"kind":"full"}` | **works** |
| 6 | **guest answers the window bodies** | `[DEBUG] refreshUi sections {"scope":{"kind":"full"},"utilities":{},"asked":["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"],"changed":[],"hashes":{"puzzle3d-main":"94ebfe0d:1","puzzle3d-main-top":"5877dc1d:1","puzzle3d-main-perspective":"aba169d7:1"}}` | **THE BREAK. `changed:[]`, and every hash still ends `:1` — the retained surfaces are at revision 1, their BOOT revision, on a `full` scope included** |
| 7 | `data-camera-json` | `publishedBefore` == `publishedAfter` == `{"position":[9.17,-5.67,4.2575],"target":[3.5,0,0.005],"up":[0,0,1],"zoom":1,"fov":50,"projection":"perspective"}` for all three gestures | frozen, correctly, at what the host already holds |

Hop 7's frozen value is exactly `main::framed_camera("puzzle3d-main-perspective", …)`
(`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:186`) and the Top pane's is
`framed_camera(WINDOW_INSTANCE_TOP, …)` — `up:[0,1,0]`, `fov:45`, `projection:"orthographic"`. Both
panes are therefore still on the OPENING pose the guest derives when `camera_unset()` is true, i.e.
**no `setCamera` write has ever been observed by the render, for either pane, in the whole run**.

## 3 The culprit: the guest reactor wedges one surface slot and every later reconcile starves

The `[actor]` reactor trace — the GUEST's own — is where it is visible. `🔍️b50-camera-lane.ts`'s
`slotStates()` prints every DISTINCT publication-slot state per step with its count, plus the
`more-work streak` high-water mark.

**At boot the surfaces really do advance** (7 distinct states, streak 33):

```
boot slots maxStreak=33 distinct=7
  1:puzzle3d-main-perspective#g3:--R:ack1/rev1:outNone x30 | 1:window#g4:--R:ack1/rev1:outNone x28
| 1:puzzle3d-main-perspective#g3:--R:ack0/rev1:outNone x6 | 1:puzzle3d-main-perspective#g3:-J-:ack0/rev0:outSome(2) x5
| 1:window#g4:--R:ack0/rev1:outNone x4 | 1:window#g4:-J-:ack0/rev0:outSome(0) x3
| 1:puzzle3d-main-perspective#g3:P--:ack0/rev0:outSome(2) x1
```

`P--` → `-J-` → `--R:rev1` → `ack1/rev1`: a full reconcile, per surface.

**From the first gesture onward there are exactly TWO states for the rest of the run**, and the
reactor spins without bound:

```
orbit-alt-right slots maxStreak=2323 distinct=2  1:puzzle3d-main-perspective#g3:--R:ack1/rev1:outNone x2323 | 1:window#g14:-J-:ack1/rev0:outSome(0) x2323
pan-shift-right slots maxStreak=5219 distinct=2  1:puzzle3d-main-perspective#g3:--R:ack1/rev1:outNone x2548 | 1:window#g14:-J-:ack1/rev0:outSome(0) x2548
zoom-wheel      slots maxStreak=7843 distinct=2  1:puzzle3d-main-perspective#g3:--R:ack1/rev1:outNone x2372 | 1:window#g14:-J-:ack1/rev0:outSome(0) x2372
```

with, on every one of those thousands of lines:

```
[actor] [DEBUG] reactor more-work streak=40 seen=91 sources=["reconcile"] contended=false effects=0
  patches=[slots=[1:puzzle3d-main#g1:--R:ack1/rev1:outNone,1:puzzle3d-main-top#g2:--R:ack1/rev1:outNone,
  1:puzzle3d-main-perspective#g3:--R:ack1/rev1:outNone,1:window#g14:-J-:ack1/rev0:outSome(0),
  1:framework.panel.artifact#g5:--R:ack1/rev1:outNone, … ] ready=[g14:--r-] terminals=[]
  producer_terminals=[] deferred=[] rejected=0 unadmitted=0 closing=0 output_fault=none
```

Read literally: the `window` surface was RE-ADMITTED after boot as generation **g14** (boot's was g4),
and it is wedged in `-J-` holding a **zero-byte** pending output (`outSome(0)`) that never acks, never
produces a revision (`rev0`) and never retires (`terminals=[]`). It is the only `ready=[…]` entry, so
the reconcile source is permanently owed, `more-work` is answered forever (`streak` climbing 2 323 → 5 219 →
7 843 across three 12-second windows, monotone), and **every other surface is frozen at the revision
it reached at boot** — `puzzle3d-main-perspective#g3:--R:ack1/rev1`, which is the `:1` in hop 6's
hashes.

That is B48's hop 7 realized: `⚛️reactor/🔄️turn/🦀️.rs:1093`–`1104` refuses a surface whose previous
reconcile is still in flight and only `defer`s it, `🩹️patches/🦀️.rs:389`/`:348` drop a defer with no
free slot, and `🔄️turn/🦀️.rs:484` re-dirties only ONE deferred surface per turn. With g14 permanently
in flight there is never a free slot, so nothing is ever re-dirtied again.

### 3.1 So the "camera lane regression" is not a camera defect

Nothing on the camera path is broken. `setCamera` is dispatched, reaches the guest, runs and settles;
the WindowConfig lane is alive (§4); the camera simply rides a retained world surface that stopped
advancing at boot. The same wedge explains, in the same #59 battery, every other verdict that needs a
world or panel body to advance:

| verdict | #57 | #58 | #59 |
| --- | --- | --- | --- |
| `camera-orbit` / `camera-pan` / `camera-zoom` | PASS | PASS | **FAIL** |
| `context-menu-zoom-moves-camera` | PASS | PASS | **FAIL** |
| `inspection-object-fields` | — | PASS | **FAIL** |
| `gumball-scene-delta` | FAIL | FAIL `poseLen=57618` | FAIL **`poseLen=266`** |
| battery total | — | `PASS=73 FAIL=25` | `PASS=67 FAIL=32` |

`gumball-scene-delta`'s `poseLen` is the tell: 57 618 bytes of world payload in #58, **266** in #59 —
266 is exactly the boot-revision instances payload this wave's probe reads at `[6.83s]`
(`instancesBytes=266`, the 1-object Concrete Forest). In #59 the world lane never leaves its boot
revision, so every world-lane reader — camera, guest selection, instances, Inspection — is reading
boot values 24 minutes into the run.

This is the SAME defect wave B48 is working, measured independently here and reaching the identical
hashes: B48's `📓️2026-09-13-wave-B48-nakagin-selection-lane.md` §3 quotes
`hashes:{"puzzle3d-main":"94ebfe0d:1","puzzle3d-main-top":"5877dc1d:1","puzzle3d-main-perspective":"aba169d7:1"}`
— byte-identical to hop 6 above, taken hours apart, and B48 records it as "not Nakagin-specific,
measured on the 1-object document". B48 §3.1 left two readings open ((a) the guest re-rendered a
byte-identical tree, (b) the guest never re-rendered). **The slot census above settles it as (b)**, and
names the wedged slot: the `window` surface at generation g14, `-J-:ack1/rev0:outSome(0)`.

## 4 What is NOT the cause — each excluded by measurement, not by reading

| candidate from the brief | verdict |
| --- | --- |
| B46's `interaction_topology_memo` keyed on `(store.generation(), config_store.generation())` | **excluded.** It is the ONLY memo in `🔌️plugin/🦀️.rs` (`rg '_memo'` → 6 hits, all this one) and its single reader is `resolve_domain_topology`'s `HierarchyProvider::Topology` arm (`:23152`). No camera, window-config or render path consults it. Its hunk is a pure refactor of the code that was inline at `:22335` before |
| B46's memo making `plugin_refresh_ui` answer the window bodies unchanged | **excluded.** `plugin_refresh_ui` (`:33786`–`:33794`) renders each window as `instance.app.render(body_key, None, view.for_window_instance(key))` and hashes the node with `ui_refresh_section` — a pure fnv1a of `serde_json::to_string(&node.root)`. Neither consults the memo. The new law in §5 exercises exactly that render call and the camera DOES move |
| B46's host hunks (GLB pick bounds, overlay `activeObjectId`) intercepting the orbit drag | **excluded by timing AND by measurement.** They landed ~15:50–16:20, i.e. inside the GREEN #57 (16:54) and #58 (17:21) batteries; and hop 1 shows the rig moving while the published pose does not |
| B44's per-object residency / `instancesDelta` / structural fingerprint (host) | **excluded.** `data-camera-json` is `world3dCameraDomJson(sceneCamera)` (`🌐️World3dHost/🟦️.tsx:6550`) and `sceneCamera` short-circuits to `parsedCamera` whenever `sceneCameraJson.includes('"position"')` (`:4939`–`:4942`) — it never passes through `advanceWorldInstanceResidency`. The host froze because the guest answered `unchanged`, which is `uiRefreshSectionUnchanged` behaving as specified |
| the host's `setCamera` payload shape | **excluded.** `buildWorldCameraDispatchArgs` (`:811`) sends `{position, target, zoom, up}` under `camera`, deliberately without `projection` — byte-for-byte the shape every green native camera law dispatches |
| the WindowConfig publication lane being dead in #59 | **excluded.** All eight `window-option-*` verdicts and `window-options-lane-responsive` / `window-options-emit-no-history` PASS in #59, and `setCamera` settles with no `window-config.window-context` fault (the fault `:22612` raises when the authority capture fails) |
| `projection-repaints-camera` / `projection-control-flips` | **not this regression.** Both FAIL in #58 as well (`waitedMs=30236` / `30371`); they are pre-existing |
| the new untracked `semio-framework-ui-viewport` crate (`🖱️ui/🪟️viewport/`, created 17:23, i.e. INSIDE the #58→#59 window) | **excluded.** `Viewport3dOrbit`'s strict `FromValue` admits only `position/target/zoom/up`, but puzzle3d's camera is `Puzzle3dCamera` (`✏️editor/🎚️config/🦀️.rs:77`, mtime Sep 12 03:39, untouched) and nothing on the puzzle3d camera path references the new crate |

### 4.1 Why #59 and not #58, given the wedge is in code both share

Every file on the reconcile path is byte-identical between the two wasms: `⚛️reactor/🔄️turn/🦀️.rs`
and `⚛️reactor/🩹️patches/🦀️.rs` were last written **Sep 12 11:46** as of both the 16:42:58 (#58) and
18:23:18 (#59) release builds — they do not appear in the full `*.rs` mtime census for the
16:43 → 18:24 window (287 files, `🗑️generated/` census). They have since moved to 19:08–19:14, which is
B48 fixing them live.

So #59 did not introduce the mechanism; it changed the surface traffic the mechanism starves under. The
only guest delta in that window that touches a retained surface is B46's outliner work
(`📌️panels/🗿️artifact/🦀️.rs` 16:55 — the arena clamp and row-action degradation, which change the
`puzzle.3d.play.document` body's paging) plus `📡️wire/🦀️.rs`'s membership set (17:15). Separating the
tipping hunk further needs a bisect wasm build, which this wave is not permitted to run — and it is
moot, because the defect is the wedge, not the traffic.

## 5 The law

`set_camera_moves_the_pose_the_window_refresh_route_publishes`
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:3747`).

Every pre-existing camera law measured [`render_window`] — the `<body>:<windowInstanceId>` surface
route. The refresh route `plugin_refresh_ui` actually uses is `render(body_key, None,
view.for_window_instance(key))` with NO instance-suffixed body key, and it is the route whose section
hash `uiRefreshSectionUnchanged` compares. That route had **no camera law at all**, so a refresh route
publishing the opening pose forever would have read green. The law closes that gap: it asserts the
refresh route's published pose leaves the opening pose, equals what `setCamera` wrote, and does not
leak into the sibling instance.

**It passes on #59's tree.** Stated plainly, because it is the honest result: the law proves the guest
render half of the camera lane is correct in the source the #59 wasm was built from, which is what
makes §3's reading — the surface was never re-rendered — the only one left. No artifact-level law can
fail on this regression, because the regression is not in the artifact; the law that fails on it is a
reactor reconcile-drain law, and that is B48's lane (`⚛️reactor/🔄️turn/🧪️tests/🕹️deferred-render-drain/`,
`⚛️reactor/🧪️tests/🔬️reconcile-budget/`, both being written as this wave closes).

## 6 Verification — all foreground

| command | result |
| --- | --- |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 camera window_config set_camera` | **9 passed / 0 failed** |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | **0 errors** (`Finished dev profile … in 27.94s`) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | **0 errors** (`Finished dev profile … in 2.64s`) |
| renderer-react vitest / `bun x tsc` | **not run, and not owed** — this wave changed NO host file (only the new law and the new probe) |

```
running 9 tests
test editor::puzzle3d::component::example_switch::a_document_swap_republishes_the_camera_fit_lane_and_an_object_edit_does_not ... ok
test editor::puzzle3d::component::unit_tests::an_opening_camera_is_stable_and_never_overrules_a_pose_the_user_set ... ok
test editor::puzzle3d::component::unit_tests::camera_actions_are_view_actions_that_emit_no_artifact_mutations ... ok
test editor::puzzle3d::component::unit_tests::every_pane_opens_on_its_own_framed_camera_before_any_gesture ... ok
test editor::puzzle3d::component::unit_tests::flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone ... ok
test editor::puzzle3d::component::unit_tests::one_window_config_mutation_publishes_exactly_one_generation_and_quiesces ... ok
test editor::puzzle3d::component::unit_tests::set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched ... ok
test editor::puzzle3d::component::unit_tests::set_camera_moves_the_pose_the_window_refresh_route_publishes ... ok
test editor::puzzle3d::component::unit_tests::window_config_publish_hostile_static_law_rejects_silent_ok_into_iter_drop ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 740 filtered out; finished in 0.94s
```

Logs: `🗑️generated/wave-B50-tests-final.txt`, `wave-B50-tests-baseline.txt`, `wave-B50-law-1.txt`,
`wave-B50-check-artifact.txt`, `wave-B50-check-plugin-wasm.txt`. No temporary `[DEBUG] ` tap was added
by this wave (the `puzzle3d.utility.publish`, `refreshUi sections`, `refreshUi lane`,
`applyHostEffects refresh` and `reactor more-work` lines quoted above are peers' permanent taps, armed
through `SEMIO_RUNTIME_DIAGNOSTICS`).

## 7 Verdicts

| verdict | where it stands |
| --- | --- |
| `camera-orbit`, `camera-pan`, `camera-zoom`, `context-menu-zoom-moves-camera` | **wait for a wasm after B48's reactor fix (#60 or later).** They cannot be fixed host-live: the wedged slot is in the GUEST reactor (`[actor]` trace), and the served guest is frozen at 18:24:22 |
| `projection-repaints-camera`, `projection-control-flips` | **not this regression** — red in #58 too; they belong to whoever owns the measure-select publication |
| `camera-json-attribute`, `window-distinct-camera`, `camera-emits-no-artifact-history`, `camera-per-window`, `camera-lane-responsive` | already PASS in #59 and unaffected (they read the boot revision, which is correct) |
| `set_camera_moves_the_pose_the_window_refresh_route_publishes` | **green now**, native |

Nothing in this wave is fixable host-live, because nothing in this wave is host-side.

## 8 Residual / handed over

1. **The wedged slot, for B48**: the `window` surface at generation **g14** sits at
   `-J-:ack1/rev0:outSome(0)` — a re-admitted surface whose pending output is **zero bytes**. A 0-byte
   reconcile output that is neither completed nor retired is the specific shape to look at; boot's g4
   passed through `-J-:ack0/rev0:outSome(0)` and completed, so the same state is survivable and it is
   the post-boot RE-admission that wedges. `ready=[g14:--r-]`, `terminals=[]`, `deferred=[]`,
   `rejected=0`, `output_fault=none` on every one of ~7 800 spin lines.
2. **The reconcile more-work spin is unbounded** — `streak` reached 7 843 and was still climbing when
   the probe closed, `sources=["reconcile"]`, `contended=false`, `effects=0`. Whatever the fix, a law
   should cap the streak: an owed source that can never be satisfied is a livelock, not a budget.
3. **Alt+right-drag orbit did not move the rig in run 2** (`orbit-alt-right rigMoved=false`) while it
   did in run 1, and pan/zoom moved it in both. Secondary to this wave's subject (the published pose is
   frozen either way) but it is a real flake on the orbit gesture alone and worth one measurement once
   the camera lane is unfrozen — the suspect is the right-button context-menu race, not the camera.
4. **Re-run `--only=camera-gestures,projection-options --port=6013` after the next wasm** to convert §7's
   four waiting verdicts. The probe `🔍️b50-camera-lane.ts` is the cheaper gate: it reports `rigMoved`
   and `publishedMoved` separately plus the slot census, so it distinguishes all three readings
   (gesture lost / round trip lost / surface wedged) in about 60 seconds without a full battery.
