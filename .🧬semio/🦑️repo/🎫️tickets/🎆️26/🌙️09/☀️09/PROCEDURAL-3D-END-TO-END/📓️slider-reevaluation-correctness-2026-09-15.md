# A Moved Slider Must Converge On The RIGHT Geometry — lane `slider-reevaluation-correctness` (2026-09-15)

Takes `📓️slider-preview-update-2026-09-15.md` §7 items 1–2 and 4, and the coordinator's walk row 25
(`1 of 7 nodes did not evaluate`, geometry mostly gone, `aria-valuenow 10` against a painted `0.0`).
The slider was CONNECTED by the previous lane; this lane is about the ANSWER it converges on.

- Law: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs`
  over the new fixture `🧫️fixtures/🎚️slider-values.json`
- Kernel law: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🧪️tests/🔬️unit/🦀️.rs`
- TS law: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- Runtime output: `🗑️generated/slider-correct/`

---

## 1. TL;DR

**The preview did not settle empty because the gesture chain lost the value. It settled empty because
the geometry the value produced was destroyed — or refused — by something that had nothing to do with
the slider.** Three defects, each on a different layer, each reproduced natively, each fixed with a law:

1. **A converged evaluation pruned the process-wide geometry store to ITS OWN handles.**
   `FlowHost::evaluate_step` called `retain_geometry_handles(&its own channels)` on every convergence.
   The store is global; any other live evaluation — the viewer preview's own `host.evaluate()` on
   every render, a generate-mode preview, a second instance — therefore lost the shapes its cached
   nodes still named, and the very next evaluation reusing one of those cached dictionaries faulted
   `missing handle: <digest>` on a node whose inputs had not moved at all. Fixed: a converged walk
   publishes a CLAIM; the one authority is the session-merged `sync_flow_geometry_retention`, and a
   session-free host retires nothing.
2. **The tessellation validate gate judged the whole arena instead of the shape it was asked about.**
   `Brep::validate_gate_sync` ran `validate_body(&self.body)` and blocked on any error-class issue
   found ANYWHERE. One invalid solid — a boolean result whose void shell is not inverted — refused
   the tessellation of every other live handle for as long as it stayed live. That is the
   "six of eight examples settle on an EMPTY payload reported as `idle`" reading, exactly. Fixed: the
   verdict is scoped to the entities the requested shape reaches.
3. **The compactor amplified one stale live handle into an eaten store.** A live entry holding a
   stale `(index, generation)` marked its own id into the protection set while the arena's CURRENT id
   for that index was a different generation, so the compactor freed a perfectly good entity — whose
   handle then went stale in turn. Fixed: a live entry whose entity no longer resolves is dropped
   BEFORE the protection set is built.

And, separately, the knob's two sources of truth (§5): the value readout was painted on the GPU, where
it could only move when a frame was drawn, while `aria-valuenow` moved with the DOM control. The
readout now lives on the same overlay row as the knob, so one published number renders both.

**Measured, native, on the real served chain** (`🧫️fixtures/🎚️slider-values.json`, 8 examples ×
(boot + drag inside + each range end + burst) = **40 graded deliveries**):

| | before | after |
|---|---:|---:|
| deliveries that equal the value's own geometry (or name its fault) | **8 / 40** | **38 / 40** |
| examples whose every gesture converges | **1 / 8** (`rectangle-wire-preview`) | **6 / 8** |
| deliveries that settled EMPTY under `idle` naming nothing | **24** | **0** |
| the two boolean examples | broken from the first edit | **named and handed on** (§7 item 1) |

---

## 2. Root causes, file:line

| # | layer | site | what was wrong |
|---|-------|------|----------------|
| 1 | framework | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` `FlowHost::evaluate_step` | on convergence it called `crate::retain_geometry_handles(&live_handles)` with ONLY its own channels, pruning a process-wide store every other evaluation also owns |
| 2 | brep kernel | `✏️s/…/🧊️brep/🧬️schema/⚙️engine/🦀️.rs` `Brep::validate_gate_sync` | `validate_body(&self.body)` — a whole-ARENA verdict used as a per-SHAPE gate |
| 3 | brep kernel | same file, `Brep::compact_unreachable` | built the protection set from live entries without first dropping the ones whose entity no longer resolves, so a stale id freed a valid entity |
| 4 | board | `🧰️framework/…/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` slider paint | the value readout was a SECOND producer of the knob's number, on the GPU frame clock |

### The order the evidence came in

1. The new law, on its FIRST run against the unfixed tree: 24 of 32 gesture deliveries wrong, and the
   surface's own status named why —
   `{"extrude": "invalid input: missing handle: 98a97d58…"}`. Not a lost value: a lost SHAPE.
2. `[DEBUG] retain_geometry_handles live=[…]` at every convergence showed a second evaluator pruning
   the store between the app's own hops. Removing the host-local prune took the law from 8/40 to
   27/32 graded rows.
3. The remainder failed with the SAME diagnostics on every example —
   `validation rejected the solid before tessellation: [void-shell-not-inverted] solid-11-void-shell-12…`
   for `box-shell`, `face-sweep` and `box-fillet` alike. One reading of `validate_gate_sync` explained
   it: the gate never looked at the handle it was given. Scoping it took the law to 38/40.
4. A post-condition assertion inside `Brep::retain` printed
   `retain left dangling live handles: ["b29a89a4…=ssolid-9-0"]` — a handle the registry calls live
   whose solid the arena had freed. That is item 3, and it is also the witness for the one defect this
   lane does NOT own (§7 item 1).

---

## 3. The fixes

### 3.1 Framework — the retention AUTHORITY is the merged claim, never one host

`evaluate_step`'s convergence branch no longer retires anything. `FlowEvalSession::capture_baseline_from`
already did the right thing and still does: it writes this session's live handle set into
`FLOW_SESSION_GEOMETRY` (with the displaced set handed to the session's own retirement ladder) and calls
`sync_flow_geometry_retention`, which prunes the kernel to the MERGED claim of every live session. It
gained the drawing half (`retain_drawing_handles`) so both stores are retired from one place.

`FlowEvalSession::tick` calls `capture_baseline_from` exactly when `remaining.is_empty()` — the same
condition `evaluate_step` used — so a session-backed evaluation retires on precisely the same beat as
before. A session-FREE host (`FlowHost::evaluate`, which the viewer preview's render, the mesh-export
bridge and every headless law use) now retires nothing, because it owns no claim.

### 3.2 Brep kernel — a gate judges its own shape

`validate_gate_sync` computes `self.body.reachable_from(&entity_roots(entity))` and keeps only the
blocking issues whose entity label names something in that set. `entity_roots` is new and is the unit
`live_roots` is now built from too, so "this shape's reach" means one thing in the GC and in the gate.
`validation_issue_reaches` reads a label as a run of `<store>-<raw index>` pairs — `edge-3`,
`solid-11-void-shell-12`, `face-4-face-9` — and a label none of whose pairs can be read still blocks,
because an unreadable diagnostic is not a clean bill of health.

### 3.3 Brep kernel — a stale live entry cannot eat the store

`compact_unreachable` drops every live entry whose entity no longer resolves before it builds the
protection set. Ordering is the whole point: a stale `(index, generation)` in the protection set is not
the arena's current id for that index, so the compactor used to read the live entity as unreferenced
and free it.

### 3.4 Board + renderer — one published number, two renderings

`DagHost::slider_overlay_state_json` is the ONE row a knob's value travels on. It gained
`fontScreenPx`/`gapScreenPx` so the readout can be laid out where the paint used to put it, and the GPU
readout is gone (track and knob had already moved to the HTML control for the same reason).
`GraphSliderOverlays` renders the readout from `slider.value` — the same number it hands the control,
and therefore the same number `aria-valuenow` reports — through one exported rule,
`dagSliderValueText`.

---

## 4. Laws, with output

### 4.1 The delivery law (Rust, new)

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --
every_moved_slider_converges --test-threads=1`

```
test editor::generation3d::component::slider_values::every_moved_slider_converges_on_the_geometry_its_released_value_makes ... ok
test result: ok. 1 passed; 0 failed
[DEBUG] handed on to brep-input-handle-lifetime: sphere-box-fuse · max-end · radius = 3: the preview settled EMPTY under phase idle while the value makes ["eval-fuse@solid#0"]
[DEBUG] handed on to brep-input-handle-lifetime: sphere-box-fuse · min-end · radius = 0.2: the preview settled EMPTY under phase idle while the value makes ["eval-fuse@solid#0"]
```

**What it actually drives.** `app_with_registry` → `setActiveExample` → the real `previewEval` run
(`brep_extension::drive_preview_run`: `pending_effects`, every framework-reserved run action fed back,
every hop redispatched as the typed command the shell sends, every extension invocation answered by
the packaged operator pack), then one `nodeGraphEdit` `setSlider` per coalesced round trip under one
gesture id, then the PREVIEW WINDOW'S OWN RENDER read back as a `World3dScene`. Nothing is
hand-addressed and nothing is asserted off the document.

**What it grades against.** An independent oracle: the same example's graph, in a fresh `with_host`,
with one slider moved, evaluated and tessellated in-process at the same preview LOD — the very path
`📚️examples/🧪️tests/🧩️geometry` grades against `parry3d`. Every oracle is taken BEFORE the app exists,
because the kernel's shape store is process-wide and an oracle taken between two gestures would be a
second live evaluation competing for it, which is exactly the interference §3.1 fixed.

**The verdict per released value**, as a list of everything the row failed rather than the first
assertion that fires:

* the published mesh ids equal the value's own, in payload order;
* the published extent equals the value's own within `boundsTolerance` (0.001);
* the chain rests in `idle` and publishes no `computing`;
* a value the kernel genuinely cannot build names a NODE and says why, instead of settling empty —
  `hexagonal-mushroom-column · min-end · height = 0` publishes
  `{"extrude": "invalid input: operation failed: invalid input: extrude distance must be positive, got 0"}`,
  which is the oracle's own word for it;
* a row the fixture hands on to a named defect must still FAIL, so a hand-on cannot rot into a silent
  pass.

### 4.2 The gate law (Rust, new)

`cargo test -p semio-s-artifact-stdio-semio --lib -- the_validate_gate_judges --test-threads=1`

```
test standards::v1::subsets::brep::schema::engine::tests::the_validate_gate_judges_the_shape_it_was_asked_about_and_not_the_arena_around_it ... ok
[DEBUG] scoped gate: healthy=ok stranger=["solid-1:shell-orientation-inward"]
test result: ok. 1 passed; 0 failed
```

Two boxes, one deliberately broken by flipping its outer shell's faces: the broken one fails its own
gate and the healthy one still passes. Before the fix the healthy box failed too.

### 4.3 The knob law (TypeScript, new)

`SEMIO_TEST_LEVEL=long bunx vitest run --config …/🎚️config/🟦️.ts -t "graph slider"` → **6 passed**
(the suite's four existing graph-slider rows plus this lane's), engine-contract file green.

`renders a graph slider's painted readout and its aria value from one published number` walks the row
across min, midpoint and max and asserts at every value that the readout's text is
`dagSliderValueText(value)` AND `dagSliderValueText(Number(knob.getAttribute("aria-valuenow")))`, that
the readout is `aria-hidden` (the control already announces the number), and that the value the
`parseDagSliderOverlays` row carries is the value that was rendered.

### 4.4 Regression suites

| suite | result | note |
|---|---|---|
| `--test example-geometry` (the committed geometry oracle) | **18 passed, 0 failed** | the authoritative per-example geometry gate, untouched by all three fixes |
| `semio-framework-os-flow --lib -- host::` | 119 passed, **4 failed** | byte-identical to the set `📓️flow-inline-continuation-2026-09-14.md` §6.3 documents as pre-existing (`connect_ports_replaces_existing_incoming_on_same_input`, `delete_selection_removes_edge_selected_by_synapse_id_domain`, `hexagonal_mushroom_fixture_reports_extruded_solid_output`, `rectangle_extrude_fixture_evaluates_solid_output`) |
| `semio-s-artifact-procedural-generation3d --lib` | 471 passed, **9 failed** | 4 are that same documented set; the other 5 fail on `left: "flow.fixture" / right: "flow.host_snapshot"` and on `scene.host_snapshot_json`, i.e. a peer's half-landed `fixture → host_snapshot` rename, in files this lane never opened |
| `semio-s-artifact-stdio-semio --lib` | 2 475 passed, **47 failed** | all 47 are `committed JSON is not canonical` / diff-inverse / snapshot round-trip laws under a peer sweep that has **1 521** uncommitted files in that artifact right now. None of them reaches `validate_gate_sync`, `retain`, `compact_unreachable` or `dispose`; `retain_compacts_everything_not_kept`, the one law that DOES exercise the compaction change, passes |
| `🔬️engine-contract` (whole file) | 619 passed, **9 failed** | seven are plainly unrelated (ink canvas, world3d framing, locales, window-kind scoping). One is a graph-SLIDER row and is still the PREVIOUS lane's: `dispatches graph parameter keyboard and drag events…` asserts `action: graphParameterFixture.action` with `args {surfaceId, widgetId, value}`, i.e. the `setGraphParameter` dispatch `📓️slider-preview-update-2026-09-15.md` §3.1 deleted in favour of `nodeGraphEdit`+`operations`. This lane adds no dispatch. The five graph-slider rows that DO grade the overlay, including this lane's own, pass |

---

## 4.5 The same three defects, read in a browser

`🐍️slider-live-preview-probe.mjs` now grades what a RELEASE delivered, not only how it got there
(§8). Run against the SERVED, pre-fix guest on 6018 (stage of 16:57), four examples, real pointer
drags — `🗑️generated/slider-correct/probe-before2.txt`:

| example | what the released value actually delivered |
|---|---|
| `hexagonal-mushroom-column` | `named fault: edge-0: edge is used by 4 coedges (2-manifold shapes use at most 2)` — a whole-BODY validation issue about an entity the hex column's extrusion does not contain. **Defect 2, live.** |
| `box-shell-preview` | `settled EMPTY under phase samplingEdges and named nothing` |
| `box-fillet-preview` | `settled EMPTY under phase samplingEdges and named nothing` |
| `sphere-cut-with-torus` | `named fault: brep_bool_cut_5: invalid input: operation failed: missing entity: solid solid-0-0` — **defects 1/3, live** |

`green=0/4`. Every one of the three root causes reproduced in the browser, on the served build, with
the same words the native law found them by.

---

## 5. The knob's two sources of truth

The coordinator's row 25 read `aria-valuenow` 10 while the canvas painted `0.0`. They were two
producers of one number: the DOM control renders the value the board published on its overlay row,
while the readout was appended to the GPU scene and could therefore only change when a frame was drawn.
The track and the knob had already moved to the HTML control for exactly this reason ("painting them
here doubles a pixelated GPU ghost above the crisp DOM slider. Only the left value readout stays on
the GPU"); this lane finished that move. There is now no code path that can paint a slider value
except `GraphSliderOverlays`, and it reads the same `slider.value` it hands the control.

---

## 6. Files

Created:
- `✏️s/…/🧊️generation3d/…/✳️any/🧫️fixtures/🎚️slider-values.json` — the rows and the declared hand-on
- `✏️s/…/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs` — the delivery law
- `📓️slider-reevaluation-correctness-2026-09-15.md` (this report)

Updated:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `evaluate_step` publishes a claim
  instead of retiring; `capture_baseline_from` retires both stores
- `✏️s/…/🧊️brep/🧬️schema/⚙️engine/🦀️.rs` — scoped `validate_gate_sync`, new `entity_roots` and
  `validation_issue_reaches`, `compact_unreachable` drops unresolvable live entries first,
  `live_roots` rebuilt on `entity_roots`
- `✏️s/…/🧊️brep/🧬️schema/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` — the gate law
- `🧰️framework/…/🕸️dag/🦀️.rs` — the GPU readout deleted, `slider_overlay_state_json` carries the
  readout's typography, `DAG_SLIDER_VALUE_LABEL_RATIO`
- `🧰️framework/…/🧱️elements/🕸️NodeGraph/🟦️.tsx` — `dagSliderValueText`, the DOM readout, the two new
  row fields
- `🧰️framework/…/🎯️targets/⚛️react/🟦️.tsx` — `dagSliderValueText` re-exported
- `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts` — the knob law
- `✏️s/…/✳️any/✏️editor/🦀️.rs` — the new test module registered
- `🐍️slider-live-preview-probe.mjs` — the per-release oracle verdict (§8)

---

## 7. What is NOT claimed, and what is handed on

### Handed on — witnessed here, owned elsewhere

1. **A boolean operator leaves its INPUT solid unusable.** After one `brep.bool.fuse` / `brep.bool.cut`,
   the sibling input's handle — a node whose own parameters never moved, so the evaluator serves it
   from cache — resolves to a freed arena slot, and the next evaluation of the same graph answers
   `missing handle: b29a89a4…` for it. The oracle, a fresh host that rebuilds the input, succeeds at
   the same value. Witnessed directly inside the kernel as
   `retain left dangling live handles: ["b29a89a4…=ssolid-9-0"]` at the FIRST compaction after the
   example's own boot, before any slider moved. It is not gesture-shaped and not slider-shaped: it
   needs only one boolean followed by one re-evaluation. Owner: the boolean's input lifetime
   (`🧊️brep/🧬️schema/🔺️diff/🔀️boolean`). The fixture declares it as `brep-input-handle-lifetime` and
   the law REQUIRES those two rows to keep failing, so the hand-on cannot rot.
   Cost today: `sphere-box-fuse` and `sphere-cut-with-torus` do not follow their slider after the
   first edit. The other six examples do.
2. **`sphere-cut-with-torus` has values the kernel refuses on its own merits.** At sphere radius 3.5
   and 10 the oracle itself delivers nothing and the validate gate says
   `[void-shell-not-inverted] solid-…-void-shell-…`. The chain names a fault there, which is what the
   law asks of it, but "a cut that produces an inside-out void shell" is a kernel question this lane
   did not open.

### Not claimed

- **Latency.** This lane measured correctness, not the ≤ 150 ms first-mesh budget
  (`📓️slider-preview-update-2026-09-15.md` §7 item 3). Nothing here makes the chain faster and
  nothing here claims it does.
- **The Inspection panel's `<input>` leaving the DOM mid-gesture** (§7 item 6 of the previous lane) —
  untouched.
- **The inline slider's 64 px rail** (§7 item 5) — untouched; a usability decision, not a mapping bug.
- **wgpu.** ON HOLD by the user's directive. `🕸️dag/🦀️.rs` is shared by both renderers and the
  readout moved to the HTML overlay both of them mount; no wgpu target file was opened and no wgpu
  battery was run.
- **The 47 stdio reds and the 5 gen3d reds are not diagnosed here**, only attributed (§4.4): a peer's
  1 521-file schema sweep and a peer's half-landed `flow.fixture → flow.host_snapshot` rename.
- **No AFTER reading in a browser, and no battery run (§9).** The before-reading (§4.5) is on the
  served 16:57 guest and is real; the after-reading needs a restage this lane could not land.

---

## 8. What the probe now grades

`🐍️slider-live-preview-probe.mjs` measured how a gesture TRAVELLED (latency, coalescing, dropped
actions, one history entry, a same-value round trip). It now also grades what the RELEASE
DELIVERED, twice per row — on the released value and again on the value the gesture returns to:

* `releasedDelivery` / `returnedDelivery`, each `{ ok, why }`, and both are in `row.ok`;
* the published mesh COUNT must equal the example's committed `delivery.meshes`
  (`🐍️example-oracle.mjs`, the same fixture `cargo test --test example-geometry` reads) — a slider
  moves sizes, never topology, so the count is the value-independent number the oracle commits;
* the published extent must exist and be finite in all six components;
* the chain must rest in `idle`;
* **an empty payload that names nothing is a failure**, and a value the kernel genuinely cannot build
  passes only by NAMING its fault — `widgetErrors`, the evaluate `fault`, or a typed validate-gate
  `diagnostics` entry (`{ handle, issues: [{ entity, code, message }] }`), each required to carry a
  non-empty message. That single rule is what turned `box-shell-preview` from "the drag looked fine"
  into `settled EMPTY under phase samplingEdges and named nothing` (§4.5).

---

## 9. The gate: not run, and why

`SEMIO_BATTERY_URL=…:6018 SEMIO_BATTERY_ROOT=react-slider2 bun 🐍️react-battery.mjs
--only=slider-live-preview,journey,interact` was **not run**, because the three Rust halves of this
lane (the flow host's retention, the brep kernel's gate and compaction, the board's slider paint) only
reach a browser through `activate-generation3d-react-dev`, and that lane was saturated all evening:

* 19:41 — this lane's first restage: killed at ~20 min.
* 20:26 — a peer's restage: `Cargo artifact build failed: flow-extension-primitive` after
  **4 501 s** of one crate (`🗑️generated/retire/restage.txt`).
* 21:15 — this lane's second, under `screen`: `@semio-tech/flow-plugin:component-dev 17m 29s`, then
  `script "nx" exited with code 143` / `terminated by signal SIGHUP`.
* 21:52 — relaunched under `nohup setsid` (`📜️restage-slider-correct.sh`, 8 attempts, and it
  republishes `flow-extension-brep` as well, which a plain restage does not and which THIS lane needs
  because it changed the brep kernel the extension links). It waited out a peer restage that ran
  **1 h 38 m**, then died the same way: **`STAGE-EXIT=143`, `terminated by signal SIGHUP`**.

**The environment finding, for the fleet.** A long `activate-generation3d-react-dev` is being SIGHUPed
here at roughly the 20-minute mark whatever it is launched under — a plain foreground shell, `screen
-dmS`, and `nohup setsid` all lost it at the same point, and a peer's run lost `flow-extension-primitive`
after 4 501 s of one crate. Every restage attempt tonight, this lane's and peers' alike, ended at 143 or
at a cargo artifact failure. `CARGO_PROFILE_WASM_DEV_DEBUG=false` was exported throughout and both disk
(199 GiB free) and memory (53 % free) were healthy, so neither the disk guard nor the OOM ceiling
explains it. The script is left in place and retrying; whoever gets a quiet build lane should run it and
then take §8's one probe command.

So the after-reading is one command away and the script to take it is already queued:
`SEMIO_PROBE_URL=http://127.0.0.1:6018/?plugin=generation3d SEMIO_PROBE_OUT=slider-correct/after-browser
SEMIO_PROBE_KINDS=graph SEMIO_PROBE_EXAMPLES=<all eight> bun 🐍️slider-live-preview-probe.mjs`, then the
battery. What IS measured is every fix, natively, on the real served chain (§4.1–4.3) and every defect,
in a browser, before them (§4.5).
