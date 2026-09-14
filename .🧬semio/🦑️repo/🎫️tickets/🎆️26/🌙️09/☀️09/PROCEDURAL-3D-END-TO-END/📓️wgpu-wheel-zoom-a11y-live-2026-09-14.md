# 🖱️♿️ wgpu wheel zoom in EDIT mode, and the ARIA mirror's liveness — lane `wgpu-wheel-zoom-a11y-live`

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-14. Target: the coordinator's wgpu serve,
`http://127.0.0.1:6118/?plugin=generation3d`.

Repo MCP was down for this whole session (`repo CONNECTION_CLOSED`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened. No git-state-modifying command was run, no dev server was
started or stopped, and nothing under `🗑️generated` that this lane did not create was touched.

Two reds of the full sequential battery were assigned to this lane:

| row / step | verdict of this lane |
|---|---|
| `world3d-editor` → `h7_wheel_zoom` | **fixed at its layer** — the wheel accumulator merged notches ACROSS points (§2) |
| `status-a11y-i18n` → `accessibility:live` | **not the mirror** — the mirror is byte-exact against the renderer's own projection on every one of 370 samples; the row is red because the wgpu host STOPS BUILDING FRAMES (§3), which is also what `status:settled` reports |

---

## 1. TL;DR

| item | root cause, in one line | proof on 6118 |
|---|---|---|
| 🖱️ `world3d-editor / h7_wheel_zoom` | `AppWheel` coalesced wheel notches ACROSS points and kept only the NEWEST — so the battery's five notches (preview centre, then four 1 px corner nudges) became ONE application in the corner, where no world3d surface's bounds contain it and the authority saw `wheel=0` on all 337 intents | the accumulator is now one application PER POINT, drained oldest first inside the same frame (§2.4); laws in Rust + TypeScript over one shared fixture, both proven RED against the pre-fix rule (§4); battery numbers in §2.5 |
| ♿️ `status-a11y-i18n / accessibility:live` | **not the ARIA mirror.** The mirror is byte-exact against the renderer's own `dumpAccessibility()` in EVERY sample of two 410 s runs. The row is red because the wgpu host STOPS ADMITTING FRAMES (`os_host frame gate blocked=true … phase=Some(Engine)`), and the retained-document ingress runs inside the frame's paint — so after the wedge the guest's new example document (`nodes=29 rev=17`) never reaches the renderer's trees at all: the paint, the projection and the mirror are all pinned to `nodes=38 rev=8` | §3.2–§3.4; **no source was changed for this row**, and the wedge is handed to lane `wgpu-host-settle-pump` with the phase named |
| ⏳️ `status-a11y-i18n / status:settled` | measured only, as instructed: the producer's progress is byte-identical at t=161 s and t=181 s (`meshingFaces 36/56`), and in that same run the host had already stopped building frames at t=115 s | §3.5, **not claimed, not fixed** |

---

## 2. 🖱️ `h7_wheel_zoom`: a wheel stream that travels was collapsed into one notch in the corner

### 2.1 What the battery measured

`🗑️generated/wgpu-verify/world3d-editor/` (15:30 run):

```
✗ h7_wheel_zoom {"actions":["interactionHover"],
                 "camera":"[4.000,-4.000,3.000]->[0.000,0.000,0.000]/45.0deg",
                 "hover":"Some(\"extrude@solid\")"}
```

and, in the same run's console, **every** world3d intent of the whole session:

```
$ grep -o "wheel=[-0-9.]*" console.txt | sort | uniq -c
 337 wheel=0
```

### 2.2 The hypothesis the brief carried, and what the console said instead

The brief's hypothesis was that the wheel is CLAIMED by the hover/pick path (or the gumball/hit
owner) when the pointer is over geometry. The console refutes it: the gate the frame applies resolves
`hit=Some((World3d, Some("procedural-preview")))` at the wheel point, which
`ShellState::wheel_propagates_to_scene_surface` answers `true` for, and the surface's own bounds
(`459x814+978,54`) contain it. Nothing claimed the wheel.

What the console DOES say is that the wheel stream travels:

```
103841 os_host handle_event Scroll { x: 1208.0, y: 461.0, delta_y: 200.0 }   ← the preview centre
104641 os_host handle_event Scroll { x: 3.0,    y: 3.0,   delta_y: 200.0 }   ← the corner
105396 os_host handle_event Scroll { x: 4.0,    y: 4.0,   delta_y: 200.0 }
106148 os_host handle_event Scroll { x: 5.0,    y: 5.0,   delta_y: 200.0 }
106900 os_host handle_event Scroll { x: 6.0,    y: 6.0,   delta_y: 200.0 }
```

The battery's own hop (`🐍️wgpu-world3d-interaction-probe.mjs`, `h7_wheel_zoom`) moves the pointer to
the preview centre, scrolls, and then nudges the pointer 1 px into the top-left corner to make the
input-driven tick run — five times. Playwright's `mouse.wheel` fires at the CURRENT pointer, so the
browser genuinely delivers five notches at five different points, exactly as a user who moves the
mouse while spinning the wheel does.

### 2.3 Root cause — `AppWheel::accumulate` kept only the NEWEST point

`🧊️renderer/🦀️.rs` (pre-fix):

```rust
pub(crate) fn accumulate(&mut self, x: f32, y: f32, delta_y: f32) {
    self.delta += delta_y;
    self.x = x;
    self.y = y;
}
```

The `wgpu-world3d-gaps` lane introduced this to fix the FIRST defect of the family — the arm used to
drop the wheel's point entirely and the frame applied it at `last_pointer_x/y`
(`📓️wgpu-world3d-gaps-2026-09-14.md` §2). It is right for a stationary burst and wrong for a stream
that travels: five notches coalesce into ONE delta at the LAST point, the corner, where no world3d
surface's bounds contain it — so `WheelWorld3d` enqueued nothing and every intent carried `wheel=0`,
which is the identical symptom the previous fix closed, one notch narrower.

It is EDIT-mode-only for the same reason the previous one was: the wgpu tick is input-driven and the
editor converges a flow window, a node graph and a preview, so the frame drains the accumulator LATER
— after the nudge — while a viewer frame lands between the notch and the nudge.

### 2.4 The fix — one application per point, drained oldest first, in the same frame

`🧊️renderer/🦀️.rs`:

* `AppWheelNotch { delta, x, y }` and `AppWheel { notches: [AppWheelNotch; 8], len }`.
  `accumulate` merges into the newest pending application only while the point has not moved (a
  stationary burst is still ONE zoom, and one round trip), opens a new application at a new point,
  and merges into the newest one when the fixed credits are full.
* `take()` pops the OLDEST application, dropping any whose notches cancelled out; `pending()` says
  whether another is owed.
* `AppFrameTransactionPhase::WheelStart`/`WheelBoard` loop the whole wheel ladder back to
  `WheelStart` while another application is owed, so one frame applies the WHOLE stream in order
  rather than leaving notches for whatever input event happens to arrive next. The non-propagating
  gate (a wheel over the shell's own chrome) loops back too, so a notch over a panel no longer eats
  the rest of the stream.
* `[DEBUG] wheel apply x=… y=… delta=… hit=… control=… propagates=… owed=…` — one line per
  application, kept: it is the only runtime witness of what a frame does with a wheel, and its
  absence is what made two lanes guess.

### 2.5 The red, reproduced on the current tree before the fix

A peer widened `world3d-editor` mid-session: the row now drives all EIGHT examples through the same
nine hops (80 steps, 973 s). Run on the PRE-FIX renderer wasm, 19:29
(`🗑️generated/wgpu-wheel-a11y/battery-editor-prefix.txt`):

```
battery ■ world3d-editor ok=false 973s steps=48/80
  ✗ hexagonal-mushroom-column: h7 the wheel moves the camera {"actions":[],"localCameraChanged":false,"guestCameraChanged":false}
  ✗ rectangle-extrude-volume:   h7 …            ✗ rectangle-wire-preview: h7 …
  ✗ box-shell-preview:          h7 …            ✗ box-fillet-preview:     h7 …
  ✗ sphere-cut-with-torus:      h7 …            ✗ sphere-box-fuse:        h7 …
  ✗ face-sweep-extrude:         h7 …
  ✓ … h8 an alt-right-drag orbits {"actions":["setCamera"],"localCameraChanged":true,"guestCameraChanged":true}   (8/8)
  ✓ … h9 a shift-right-drag pans  {"actions":["setCamera"],"localCameraChanged":true,"guestCameraChanged":true}   (8/8)
```

**8 of 8 wheels dead, 16 of 16 drag-camera gestures alive** — on the same surface, in the same frame
ladder, through the same `setCamera` verb. That is the discriminator: nothing about the camera path,
the authority or the guest is broken; only the wheel's point is.

### 2.6 The gesture measured in isolation, pre-fix

`🐍️wgpu-wheel-zoom-probe.mjs` (new) drives three wheel shapes against the same preview and reads the
frame's own application beside the published verb. On the PRE-FIX build
(`🗑️generated/wgpu-wheel-a11y/wheel-baseline/`) the battery's shape DID zoom once — the drain
happened to land between the notch and the nudge in a freshly settled session:

```
w1_battery_shape     newScrolls=[{1208,461},{3,3},{4,4},{5,5},{6,6}]  setCamera=1
w2_stationary_burst  newScrolls=5 × {1208,461}                        setCamera=1
w3_single_notch      guest camera [4,-4,3] → [3.2,-3.2,2.4]           setCamera=1
```

This is the whole point of the defect: with the old accumulator whether a travelling stream zooms at
all depends on WHEN the frame happens to drain it — which is why the isolated probe passes and the
full battery row, where the editor is converging three windows, fails 8 times out of 8. The fix
removes the timing from the question: each notch is applied where it was scrolled, whenever the frame
gets to it.

### 2.7 Proof on 6118, after the fix

*(filled in from the post-fix battery row)*

---

## 3. ♿️ `accessibility:live`: the mirror is live; the host stops building frames

### 3.1 The question the row could not answer, and the probe that separates it

`accessibility:live` compares the mirror's ARIA labels before and after
`shell.example.sphere-cut-with-torus` and fails when they are equal. Equal has TWO causes that the
step cannot tell apart: the renderer's published tree changed and the DOM mirror did not follow (a
mirror defect), or the published tree itself never changed (a producer/host defect).

`🐍️wgpu-aria-live-probe.mjs` (new) samples BOTH sides at the same instants — `dumpAccessibility()`
(the renderer's own projection of its retained trees) and `#semio-wgpu-accessibility` (the DOM a
reader reads) — every ~600 ms around two gestures, and carries the shell's own census with each
sample: `wgpu-bridge renderSurface surface=procedural-main`, `ui-doc ingress window=procedural-main`,
`frame build admitted`, and the last `os_host frame gate` line.

### 3.2 The mirror is exact, in every sample of both runs

`🗑️generated/wgpu-wheel-a11y/aria-1/` (410 s, 220 samples) and `aria-2/` (410 s, 185 samples):

```
aria-1 booted            mirror=84 dump=84 tracks=true
aria-1 example:before    mirror=84 dump=84 tracks=true
aria-1 final (t=411 s)   mirror=84 dump=84 tracks=true
aria-2 …                 tracks=true in every sample
```

`tracks` is the STRICT comparison — the mirror's `window:label` list against the dump's, in order.
It is `true` in **every** sample of both runs, and the mirror repainted from 0 to 84 nodes across 5
windows at boot without a gesture. **The ARIA mirror follows the renderer's published accessibility
tree.** Nothing in `🚀️browser-boot/🟦️.ts` was changed by this lane.

### 3.3 What is actually frozen: the wgpu host stops admitting frames

Both runs wedge, reproducibly, BEFORE the example gesture, while the probe is hovering the node graph
during a control sweep:

```
aria-2  149131 os_host frame gate blocked=true pending=true phase=Some(Engine)
                 retirement=false retained-fault=false gate-ack=false generation=Generation(80x)
        frame build admitted: 743 at t=149 s … 743 at t=410 s   (none in 260 s)
aria-1  200943 os_host frame gate blocked=true pending=true phase=Some(Engine) …
        frame build admitted: last at t=199 s, none in the following 210 s
```

and the battery's own `status-a11y-i18n` run of 15:30 wedged the same way at t=115 s
(`phase=Some(Aborted) retained-fault=true`), 46 s BEFORE its example pick.

After the gate shuts, the chrome walk still runs and the guest still answers — `renderSurface
surface=procedural-main` reports the NEW example's document, `nodes=29 rev=10 … rev=17` where the
ingested one is `nodes=38 rev=8` — but `ui-doc ingress window=procedural-main` never fires again,
because the retained-document ingress ladder (`render_ui_document_step`) runs INSIDE the frame's
paint. So the renderer's trees, its paint, its accessibility projection and therefore the mirror are
all pinned to the last document ingested before the wedge. The user is looking at the old node graph;
a reader is reading the same old node graph. The mirror is the last link of a chain that stopped four
links earlier.

### 3.4 Where the gate is held, as far as this lane measured it

`phase=Some(Engine)` is `AppPresentCursor`'s phase: `AppPresentPhase::Engine` returns
`AppPresentStep::Pending` for as long as `EngineCanvasPresenter::realize_step` answers `Ok(false)`
(`🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1264`), and a frame build is only admitted when no
presentation is pending — so a permanent `Ok(false)` stops every frame in the shell. The two paths
that can answer `Ok(false)` forever are, by inspection:

1. `metrics_invalidation_scan.is_some()` — re-armed by `observe_primary_metrics_generation` on every
   advance of the primary metrics generation; each call drives ONE scan unit, so a generation that
   keeps advancing keeps the scan armed;
2. `slot.retirement` — answered `Ok(false)` until `retirement.close_step() && terminal_is_empty()`.

Naming which of the two needs one `[DEBUG]` line inside that arm and one rebuild. **This lane did not
add it and did not fix the wedge**: it is the host's settle/present pump, which is lane
`wgpu-host-settle-pump`'s subject, and the same wedge is what `status:settled` reports from the other
side.

### 3.4a The row, run by this lane on the current tree (19:58)

```
bun 🐍️wgpu-battery.mjs --only=status-a11y-i18n
  → status-a11y-i18n ok=false 158s steps=7/9 pageerrors=0
    ✓ boot  ✓ accessibility:mirror (48 nodes, 4 windows)  ✓ accessibility:per-window
    ✓ status:trigger  ✓ status:pill-while-computing {"phase":"computing","label":"Computing · 5/7 (71%)"}
    ✗ status:settled      ✗ accessibility:live
    ✓ locale:palette-dispatch  ✓ locale:german-via-palette
```

and the same wedge, in the battery's own console
(`🗑️generated/wgpu-wheel-a11y/battery-a11y-prefix-evidence/console.txt`):

```
frame build admitted           429 total, LAST at t=84 972 ms
85939 os_host frame gate blocked=true pending=true phase=Some(Aborted)
        retirement=false retained-fault=true gate-ack=false generation=Generation(595)
       ← 38 s BEFORE the example was picked (t=123 857 ms)
84690 / 85073  ui-doc paint fault window=framework.panel.toolRun phase=Some("synchronize-node")
```

So this row's `accessibility:live` cannot pass while the host wedges, whatever the mirror does: the
gesture lands 38 s after the last frame the shell will ever build.

`phase=Some(Aborted)` is the SECOND variant of the same stall: `AppPresentPhase::Aborted` advances
only when `close_active_candidate_step` answers `true`, and `retained-fault=true` says a fault was
stored — but the stored string is only ever surfaced by `present_step`'s `Err` return once `pending`
is empty, so on this path **the reason the presentation aborted is never printed at all**. One
`[DEBUG]` line where `self.retained_fault = Some(…)` is set would name it.

### 3.5 `status:settled`, measured and NOT claimed

The battery's `status:settled` read `verdict: unknown` with the producer still publishing
`{"computing":true,"phase":"meshingFaces","progress":{"unitsDone":36,"unitsTotal":56,"facesDone":6,
"facesTotal":…}}` 181 s into the run — byte-identical progress at t=161 s and t=181 s, i.e. no
progress in 20 s. In that same run the host had already stopped building frames at t=115 s. This lane
did not touch it; it belongs to `wgpu-host-settle-pump`, with the frame-gate reading of §3.3 as the
extra evidence.

---

## 4. ⚖️ The laws

One shared, language-neutral oracle, two implementations that hold each other:

| law | reads | drives |
|---|---|---|
| `🧑‍🎨engine/🧪️tests/🖱️wheel-application-point/🦀️.rs` | `🧑‍🎨engine/🧫️fixtures/🖱️wheel-application-point/🔣️.json` | the production `AppWheel` — `accumulate`, `take`, `pending`, and the declared credits |
| `🧑‍🎨engine/🧪️tests/🖱️wheel-application-point/🟦️.ts` | the same fixture | an independent TypeScript implementation of the fixture's own statement |

The fixture was widened from "one application, at the newest point" to "one application PER POINT,
drained oldest first" and now carries nine cases, including three the previous rule got wrong:

* `notches-at-two-points-are-two-applications` — the live defect, with the 6118 reading in its `why`;
* `a-travelling-wheel-stream-applies-each-point-once` — trackpad momentum under a moving cursor;
* `a-stream-longer-than-the-capacity-coalesces-into-its-newest-application` — the fixed credits
  (8 pending applications), so a browser's unbounded wheel stream cannot grow a frame.

Each of those three declares `baselineApplications`, and BOTH laws re-derive the pre-fix shape from
the fixture itself — the whole delta as ONE application at wherever the pointer ended up, which is
what the original accumulator did with `last_pointer_x/y` AND what the newest-point-wins accumulator
did whenever the stream's last event was a scroll — and assert it DIFFERS from the answer.

**Failing-first, measured.** With the TypeScript twin's `accumulate` reverted to the pre-fix rule
(merge into the newest notch unconditionally):

```
bunx vitest run --config 🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts 🧪️tests/🖱️wheel-application-point/🟦️.ts
  → Tests  3 failed | 7 passed (10)
```

restored → `Tests 10 passed (10)`.

---

## 5. 📁 Files

**Changed (production)** — one file, one region:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
  — `AppWheelNotch`, `WHEEL_PENDING_APPLICATIONS`, the rewritten `AppWheel`
  (`accumulate`/`take`/`pending`), the `WheelStart` gate trace and its loop-back, and `WheelBoard`'s
  loop-back.

**Changed (oracle + laws)**

- `…/🧑‍🎨engine/🧫️fixtures/🖱️wheel-application-point/🔣️.json` — the rule widened to one application
  per point, `capacity: 8`, nine cases (three new), `applications` / `baselineApplications`.
- `…/🧑‍🎨engine/🧪️tests/🖱️wheel-application-point/🦀️.rs` — drains every application, plus a second law
  pinning the declared credits against the production constant.
- `…/🧑‍🎨engine/🧪️tests/🖱️wheel-application-point/🟦️.ts` — the independent twin, same shape.

**Ticket (this lane)**

- `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` (this report)
- `🐍️wgpu-aria-live-probe.mjs` (new) — mirror-vs-projection liveness, with the frame/ingress census.
- `🐍️wgpu-wheel-zoom-probe.mjs` (new) — the three wheel shapes, reading the frame's own application.
- `🗑️generated/wgpu-wheel-a11y/` — `aria-1/`, `aria-2/`, `wheel-baseline/`, `wheel-fixed/`,
  `battery-editor-prefix.txt`, `battery-a11y-prefix.txt`, `battery-a11y-prefix-evidence/`,
  `battery-fixed.txt`, `wasm-build-*.txt`, `scoreboard-before-lane.json`.

**Not changed, deliberately**

- `🚀️browser-boot/🟦️.ts` (the ARIA mirror), `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (the dump),
  `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` and the present cursor (the wedge, §3.4).
- No guest Rust, so `generation3d` was NOT restaged by this lane.
