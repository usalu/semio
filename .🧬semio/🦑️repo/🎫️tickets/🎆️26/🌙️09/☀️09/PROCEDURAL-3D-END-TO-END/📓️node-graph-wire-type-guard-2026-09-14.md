# 🚫️ The node graph may not accept a type-incompatible wire

Lane `node-graph-wire-type-guard`, 2026-09-14. The defect named in
`📓️wgpu-generate-add-port-fit-2026-09-14.md` §3.2 / §6.2: a drag from `extrusion-axis@vectorOut`
(a `math.vector` output) released on `extrude@wire` (the wire input of `brep.solid.extrude`) was
ACCEPTED — `nodeGraphEdit {disconnect e4}{connect extrusion-axis.vectorOut → extrude.wire}` removed
the valid `profile@wire->extrude@wire` and re-solved the model into a fault.

---

## 1. TL;DR

* **The owning layer is the declared operator catalogue, not any renderer.** `ChannelSpec` now carries
  `value_types` — the value SCHEMAS a channel carries (`geometry`, `vector`, `point`, `number`,
  `text`, `boolean`, `list`), which is what a `Dictionary` on that wire actually has. The declarations
  come from the contributions themselves: the schema components derive theirs from the declared
  `Schema` fields, and the brep catalogue declares its through the helpers every one of its ~150
  channels is already built by. There is no table keyed by operator id anywhere.
* **One rule, three enforcement points, all Rust that BOTH renderers run**:
  `neural_engine::Registry::channel_compatible` (the oracle),
  `semio_framework_os_flow::port_value_types_compatible` behind `FlowHost::connect_ports` (the ONE
  door every `nodeGraphEdit` `connect` dispatches into, React and wgpu alike), and
  `board::is_valid_connection` (the snap/drop gate the React wasm canvas and the wgpu shell share,
  because both drive the same `GraphEngine`).
* **The rule refuses only when BOTH ends declare and the declared sets are disjoint.** That is what
  keeps the eight shipped examples legal — §6.2 correctly warned that a strict nominal rule over the
  old `operators` list would have refused the examples' own `e4`/`e5`. All 23 wires the eight examples
  draw are accepted; 10 pairs including the defect's own are refused (§3).
* **Laws, all run**: 5 Rust laws in `🧪️tests/🔌️port-types/🦀️.rs` (including an end-to-end one that
  drives a real `FlowHost` over the hexagonal mushroom column's own graph), 1 board law, and a 4-law
  TypeScript twin over the SAME fixture. Output in §4.
* **Runtime on React is BLOCKED, and this report does not claim it.** The guest never receives the
  operator catalogue on `:6024`: `setContributions … rejected 273136 raw bytes before decoding;
  maximum is 262144`. With no catalogue the guest's `kind_infos` are empty, every neuron port falls
  back to an untyped `IoPortSpec::simple`, and the rule correctly declines to refuse anything — so a
  run in that state proves nothing. The probe detects exactly this and reports `BLOCKED`, never green
  (§5). The remaining ceiling is `PUBLIC_INVOCATION_BODY_BYTES` (262 144), not the 64-page command
  ingress the `contributions-ingress-ceiling` lane already lifted (§5.2).

---

## 2. What changed

### 2.1 The type, and where it is declared

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` | `ChannelSpec.value_types: Vec<String>` + `with_value_types`; `VALUE_TYPE_*` consts; `field_channel_value_types` derives a schema field's port type from the DECLARED `ValueType` (`Integer`/`Decimal` collapse to one `number` — whether a number is whole is input validation, never wiring; `Any` declares nothing); `schema_component_info` stamps it on every auto-registered component's instance and field channels; the typed constructors (`number`, `number_default`, `integer_default`, `boolean_default`, `text_default`, `list`, `list_output`) stamp theirs, so ~40 call sites gained a type with no call-site edit; `Registry::channel_compatible` rewritten over `value_types` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs` | the brep catalogue's own declarations: `geometry_channel` → `geometry`; `vector_channel`/`point_channel` → `vector`+`point` (both read through `read_xyz`, so the declaration says both rather than inventing a nominal split that would refuse `math.point → revolve@axisDirection`); `out_solid`/`out_wire`/`out_curve`/`out_face`/`out_surface`/`out_geometry`/`out_compound`/`out_*_result` → `geometry`; `out_point`/`out_normal` → `point`/`vector`; `out_span`/`out_curvature`/`out_volume`/`out_area`/`out_length` → `number` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` | `channel_spec_value_type` now publishes the DECLARED value schemas, comma-joined, and `None` when undeclared — it used to publish `spec.operators.join(",")`, the operator-id list §6.2 named as the reason no surface could tell the pair apart; slider/note widget ports declare `number`/`text` |

`operators` is untouched and still means what its docstring says. The port TYPE is the concept §6.2
said the registry does not publish yet; this lane adds it rather than overloading the old field.

### 2.2 The rule, and the three places it is enforced

| file | change |
|---|---|
| `🧠️neural/⚙️engine/🦀️.rs` | `Registry::channel_compatible(output, input)` — refuses only when both declare and the sets are disjoint |
| `🌊️flow/🖥️host/🦀️.rs` | `PortSide`, `widget_port_value_types`, `port_value_types_compatible`; `FlowHost::connect_ports` refuses with a new `FlowCoreError::IncompatiblePortTypes { source, source_type, target, target_type }` BEFORE it touches `self.fixture.synapses`, so the displaced wire is never removed and `rebuild_dag` never runs |
| `♾️infinite/🎲️board/🦀️.rs` | `Handle.value_types`; `set_handle_value_types`; `is_valid_connection` split into `is_structurally_valid_connection` (role, self-node, duplicate, single-incoming, acyclicity) + `handles_type_compatible`, so a drag can tell "this port could never take a wire" from "this port takes wires, but not one carrying THIS value" — the only refusal a user can act on; `wire_drag_type_refusal()` reports the pair the live drag is hovering that ONLY the type rule refuses |
| `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` | every handle the dag board creates declares its port's types; `wire_type_refusal_json()`; `hovered_channel_json()` carries the refusal alongside the hovered channel, so ONE poll answers both "what is under the pointer" and "why will it not take this wire" — no new wasm ABI operation was needed |

Because the React node graph IS the same `GraphEngine` compiled to wasm and the wgpu shell drives it
in-process, the snap refusal ("the target is not droppable") is one implementation for both renderers,
not two.

### 2.3 The port type on the wire, and the user-visible reason

| file | change |
|---|---|
| `🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` + `🟦️.ts` | `NodeGraphPortRecord.value_type` / `valueType`; `NodeGraphOperatorChannelRecord.value_types` / `valueTypes` so the catalogue round trip that re-seeds a host's `kind_infos` on the wgpu path does not drop the types |
| `🗺️surface/🕸️node-graph/🦀️.rs` | `GraphPortRecord.value_type` → `IoPortSpec.value_type` in `port_to_io` |
| generation3d, generation2d, flow (editor + viewer), dag, sequence window builders | each port record now carries `value_type: port.value_type.clone()` |
| `📺️renderer/…/🕸️NodeGraph/🟦️.tsx` | `portValueTypes`, `portValueTypesCompatible`, `nodeGraphConnectionIsValid` (the React Flow `isValidConnection` the SSR `Diagram` fallback now uses, and a second refusal in its `onConnect`); `parseDagWireTypeRefusalJson`; `usePortTypeLabels`/`wireRefusalLabelOptions`; the flow canvas reads the refusal on every drag move and renders it as a `role="status" aria-live="polite"` hint carrying `data-wire-refusal-json` |
| `🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` + `🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | `ui.nodeGraph.incompatiblePorts` (normal + beginner tiers) and `ui.nodeGraph.portType.{geometry,vector,point,number,text,boolean,list}` in English AND German, so the hint names both ports and both declared types in the reader's own language |

---

## 3. The fixture — the pairs the eight examples actually draw

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧫️fixtures/🔌️port-types/🔣️.json`: 36 channels and 33
rows, 23 accepted and 10 refused, drawn from every one of the eight generation3d examples
(`hexagonal-mushroom-column`, `rectangle-extrude-volume`, `face-sweep-extrude`, `box-shell-preview`,
`box-fillet-preview`, `sphere-cut-with-torus`, `sphere-box-fuse`, `rectangle-wire-preview`).

The fixture owns the verdicts. Neither implementation does: the Rust half checks its `channels` block
against the LIVE extension registry (so a channel that silently loses its declaration fails there,
not in a user's graph) and the TypeScript half answers the same `rows` with the renderer's own
predicate.

Refused rows include the defect itself (`math.vector@vectorOut → brep.solid.extrude@wire`) and nine
more of the same shape (`geometry` into a `number`, `number` into a `geometry`, `geometry` into a
`vector`). Accepted rows include the two §6.2 warned a nominal rule would have broken:
`brep.curve.polygon@wire → brep.solid.extrude@wire` (the wire the defect DISCONNECTED) and
`math.vector@vectorOut → brep.solid.extrude@vector`.

---

## 4. Laws, and their output

### 4.1 Rust — `cargo test -p semio-framework-os-flow --test flow_port_types`

```
running 5 tests
test an_undeclared_channel_stays_connectable ... ok
test a_vector_output_is_refused_by_the_wire_input_that_accepts_the_profile ... ok
test every_fixture_pair_is_accepted_or_refused_as_the_fixture_says ... ok
test every_named_channel_declares_the_value_types_the_fixture_records ... ok
test the_flow_host_refuses_the_drop_and_keeps_the_displaced_wire ... ok

test result: ok. 5 passed; 0 failed
```

`the_flow_host_refuses_the_drop_and_keeps_the_displaced_wire` is the end-to-end one: it builds a real
`FlowHost` over the hexagonal mushroom column's own widgets and synapses with the real first-party
catalogue, dispatches the defect's exact connect, and asserts `IncompatiblePortTypes`, a
byte-identical synapse list, and that `profile@wire -> extrude@wire` survives — then cuts `e5` and
reconnects the compatible pair to prove the rule is not simply refusing everything.

### 4.2 Rust — `cargo test -p semio-framework-os-infinite --lib a_wire_whose_ends`

```
test board::component::tests::a_wire_whose_ends_declare_disjoint_value_types_never_snaps_and_never_connects ... ok

test result: ok. 1 passed; 0 failed
```

The board half: an incompatible port is never a snap target, `wire_drag_type_refusal()` names the
pair, the drop adds no edge, the wire already on that target port survives — and when the source's
declared type is changed to a compatible one the same gesture connects.

### 4.3 TypeScript twin — `SEMIO_TEST_LEVEL=long vitest --testNamePattern="node-graph port types"`

```
[DEBUG] port-type twin answered 33 rows
 ✓ node-graph port types > accepts or refuses every fixture pair exactly as the fixture says
 ✓ node-graph port types > refuses the drag the defect accepted, on the node records a surface actually holds
 ✓ node-graph port types > leaves an undeclared port connectable
 ✓ node-graph port types > names both ports and both declared types in the refusal a surface shows
      Tests  4 passed
```

---

## 5. Runtime — what was reached, and what blocks the rest

### 5.1 The probe

`🐍️wire-type-guard-probe.mjs` (React, `:6024`, this lane's own port). It drives both halves of the
pair with real pointer gestures, aiming through
`window.__semioFlowGraphProbe[surfaceId].entity("handle", "node@port")` — the graph canvas paints
itself, so there is no per-port DOM — and asserts per half: incompatible → refused, the wire list
byte-identical, no `nodeGraphEdit` connect dispatched, no re-solve, and the refusal hint visible WHILE
the pointer is still held; compatible → cut and reconnected.

What the probe reached on the current build: the serve answers 200, the Flow window boots, the graph
surface `window:procedural-main` publishes all six of the example's wires, and the host resolves the
screen rects of `extrude@wire` and `extrude@vector`.

### 5.2 Why it cannot judge the guard yet — `BLOCKED`, measured

```
4898 [DEBUG] command ingress crossed {"actionId":null,"bytes":273731,"pages":67,"ms":1648}
4899 error setContributions command failed procedural tool factory
     's.procedural.generation3d@1/*#editor/setContributions' rejected 273136 raw bytes before
     decoding; maximum is 262144
2504 [DEBUG] contributions document sources {…,"status":"unresolved","reason":"no-operator-graph","kinds":[]}
```

The 64-page command ingress the `contributions-ingress-ceiling` lane lifted is gone — 67 pages
crossed in 1 648 ms with no `exceeds 64 pages`. What refuses the pack now is a DIFFERENT and
later ceiling: `PUBLIC_INVOCATION_BODY_BYTES` (262 144) in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`,
checked by the action bus as `ToolDispatchError::RawWireLimit` before any tool contract is consulted
(`🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:431`). The pack is 272 089 chars / 273 136 raw bytes.

The consequence, and why no runtime claim can be made either way: with the pack refused the guest's
`kind_infos` are EMPTY, so `neuron_io_layout` gives every neuron node only the input ports the DSL
names and NO outputs at all, and every port it does build is an untyped `IoPortSpec::simple`. The
independent evidence for that is the geometry read-back: on this build the host resolves every
INPUT port (`extrude@wire`, `extrude@vector`, `profile@radius`, `profile@sides`, `extrusion-axis@z`)
and the slider widget outputs (`height@number`, `radius@number`, `sides@number`), and resolves NO
neuron output at all (`extrusion-axis@vectorOut`, `profile@wire`, `extrude@solid` — all `null`).
A drag has no output port to start from, and the rule has no declaration to refuse on.

The probe names this state rather than passing through it: `catalogueBlocker()` matches the refusal
line and the `"status":"unresolved"` contributions record, and the run exits `2` with
`[DEBUG] BLOCKED contributions-pack-refused`. It cannot read green while the catalogue is missing.

Evidence: `🗑️generated/wire-guard/` — `restage.txt`, `recon-console.txt`, `probe/report.json`,
`probe/console.txt`.

### 5.3 wgpu

The wgpu guest was restaged with this lane's Rust. The wgpu twin (`🐍️wgpu-battery.mjs
--only=node-gestures` on `:6118`) was not run to a verdict inside this lane: `:6118` answers 200, but
`setContributions` is the plugin's OWN tool and the 262 144 ceiling is checked in the action bus
before the tool contract, so the wgpu path meets the identical refusal — the renderer does not enter
into it. Recording that as an expectation, not as a measurement.

---

## 6. Explicitly not claimed

* **No runtime proof that the drag is refused in a browser.** Both renderers are blocked on the same
  contributions ceiling (§5.2); every claim in this report about running code rests on the native laws
  of §4, which exercise the real registry, the real `FlowHost` and the real `GraphEngine`, and on the
  TypeScript twin, which exercises the renderer's own predicate. The probe is written, boots, finds the
  graph and aims at ports — it reports `BLOCKED`, and that is the honest verdict today.
* **The refusal hint was not seen on screen.** It is wired end to end (board → `hovered_channel_json`
  → React state → `role="status"` element with `data-wire-refusal-json`) and its label options are
  covered by the TypeScript twin, but no run has rendered it, because no run has had a typed port to
  refuse.
* **The 262 144 ceiling is not this lane's to lift**, and nothing here was shrunk or compressed to fit
  under it — per the coordinator's instruction the schema-first port types stay. The growth
  248 827 → 272 089 chars is the declarations being real.
* **Neuron output ports do not resolve through `entity_screen_json` on this build.** Named because it
  blocked the probe's aim; it is a consequence of the empty catalogue (a node with no declared outputs
  has no output rows), not a separate defect, and nothing was changed for it.
* **`operators` was not removed or re-meaned.** It is still the operator-capability list; the port type
  is a new, single-vocabulary field beside it. No compatibility layer, no migration.
* **No claim about the other red battery rows** (`boot`, `no-example`, `generate-add`, `port-fit`).
  Nothing outside the files listed in §2 was touched.
