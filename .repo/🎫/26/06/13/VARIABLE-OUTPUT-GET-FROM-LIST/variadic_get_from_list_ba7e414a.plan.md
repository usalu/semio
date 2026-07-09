---
name: Variadic Get From List
overview: 'Turn the flow "Get" (list.get) component into a variable-output component: the first output is labelled "i", and at high zoom a "+" control on the output side adds "i+1", "i+2", ... each reading consecutive list elements from the base index, respecting wrap.'
todos:
 - id: operator
   content: "flow/module/list/lib.rs: register list.get with variadic_output spec; rewrite Get::evaluate to emit keys 0..count = list[index+n] (wrap-aware); update Get tests"
   status: completed
 - id: host-storage
   content: "flow/core/lib.rs: add output_ports to Widget::Neuron, default_neuron_output_ports, dynamic output labels (i, i+1, ...) + variadic_outputs flag in neuron_io_layout, thread output_ports through widget_io_ports/add_widget"
   status: completed
 - id: host-count
   content: "flow/core/lib.rs: inject output count into neuron params in tree_from_fixture so Get sees count"
   status: completed
 - id: host-api
   content: "flow/core/lib.rs: add add_output_port/remove_output_port + wasm bindings; handle output pending insert in pointer_down_screen"
   status: completed
 - id: dag
   content: "dag/lib.rs: side-aware pending insert, variadic_output_insert_positions, output-side hit-test and + painting at zoom>=1.5"
   status: completed
 - id: react
   content: "flow/react/index.tsx: add outputPorts to neuron widget type + serialization; optional catalogue summary update"
   status: completed
 - id: verify
   content: Run Rust + vitest suites via nx and confirm runtime add-output behaviour; close ticket
   status: completed
isProject: false
---

# Variable-Output "Get From List"

Make `list.get` a variadic-output component. Keep inputs `list`, `index` (the base `i`), `wrap`. Outputs are dynamic: port 0 labelled `i` = element at `index`, port `n` labelled `i+n` = element at `index+n` (wrap-aware). A `+` control on the output (right) column at zoom >= 1.5 adds outputs, mirroring the existing input-side variadic `+`.

This is done inside a repo ticket (mcp `ticket_open`, associate with the most fitting goal from `repo://goals`).

## 1. Operator: variadic output spec + multi-output eval

File: [flow/module/list/lib.rs](flow/module/list/lib.rs)

- Register `list.get` (currently `register_simple`, lines 265-277) via `registry.register_operator` so it carries `variadic_output: Some(VariadicSpec { slot_key: "value", min: 1, max: None })`, like `math.addVariadic` in [flow/module/math/lib.rs](flow/module/math/lib.rs) (lines 576-580). Keep a single declared output ChannelSpec (used for `produces`/fallback).
- Rewrite `Get::evaluate` (lines 41-60): read `list`, `index`, `wrap` as today, plus `count` (default 1, derived from params - see step 3). For `n in 0..count`, resolve `index+n` (wrapping via `indices[(index+n) % len]` when `wrap`, else bounds-checked), and build an output `Dictionary` with key `n.to_string()` -> value payload (same Dictionary-vs-atom wrapping as today). Return that dictionary directly (keys `0..count-1`) instead of `channel_output("value", ...)`.
- Update the `Get` tests in the `#region Tests` block to assert multi-output keys (`0`, `1`, ...) and wrap behaviour.

## 2. Neural engine

File: [neural/engine/lib.rs](neural/engine/lib.rs)

- No core change needed: outputs are routed by `from_port` via `synapse_source_value` (lines 1095-1118) which already reads arbitrary output dict keys; `variadic_output` already exists on `OperatorInfo` (line 553). Verify only.

## 3. Flow host: widget storage, dynamic layout, count injection, add/remove API

File: [flow/core/lib.rs](flow/core/lib.rs)

- Add `output_ports: Vec<String>` (serde `default`, alias-free) to `Widget::Neuron` (struct at lines 21-36).
- Add `default_neuron_output_ports(kind, output_ports, kind_infos)` mirroring `default_neuron_input_ports` (lines 434-447): when the kind has `variadic_output`, default to `(0..min)` ids.
- `neuron_io_layout` (lines 449-504): when the kind has `variadic_output`, generate dynamic output `IoPortSpec`s from `default_neuron_output_ports` - `id = "0".."n-1"`, and all textual reps + `label` = `i` for index 0 else `i+{n}` (uniqueness rule satisfied). Set the returned `variadic_outputs` flag to `true` in every branch where the kind is variadic-output (currently only the variadic-input branch returns it, line 475; fixed-input branch at 478-486 returns `false`).
- `add_widget` descriptor path (around lines 867-871) and `widget_io_ports` (line 508): thread `output_ports` like `input_ports`.
- `tree_from_fixture` (lines 258-268): for `Widget::Neuron` with non-empty `output_ports`, inject the count into the neuron `params` copy under reserved key `count` (e.g. `params.insert("count", Decimal(output_ports.len()))`). Widget params stays clean; `Get::evaluate` reads it via the `input.merge(&neuron.params)` done in `evaluate_channels_cached` (line 960).
- Add `add_output_port(widget_id, index)` / `remove_output_port(widget_id, port_id)` mirroring `add_input_port`/`remove_input_port` (lines 1609-1697): mutate `output_ports`, enforce `variadic_output.min/max`, and reindex synapses where `from == widget_id` with numeric `from_port` (the output side), calling `begin_change` + `rebuild_dag` + `touch_channel_eval`.
- `pointer_down_screen` (lines 1835-1838): after the input `take_pending_port_insert`, also drain an output pending insert and call `add_output_port` (see step 4 for the DAG-side hook).
- Add `addOutputPort`/`removeOutputPort` wasm bindings next to `addInputPort`/`removeInputPort` (lines 2838-2854) for parity.
- Update flow-core tests/snapshots that serialize neuron widgets to include `output_ports`.

## 4. DAG: output-side "+" insert (layout, hit, paint)

File: [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs)

- Generalize pending insert to be side-aware: introduce `enum DagPortSide { Input, Output }`, change `pending_port_insert` (line 1715) and `take_pending_port_insert` (line 2434) / `port_insert_hit` (line 2439) to carry/return the side. Update the existing variadic test (lines 5899-5905) and the flow-core handler accordingly.
- Add `variadic_output_insert_positions(node)` mirroring `variadic_input_insert_positions` (lines 987-996) but using `computation_output_label_x(node, "+", px)` and the output row index; gate on the node being variadic-output.
- Extend `port_insert_hit` to also test the output column `+` row (use `computation_output_column_x_bounds`, lines 854+, for the hit x-range) and return `Output` side.
- Extend `+` painting: in the paint path at lines 4140-4143 also call a plus-control paint for `variadic_outputs: true` at `zoom >= DAG_VARIADIC_PLUS_ZOOM_THRESHOLD`; either generalize `paint_variadic_plus_controls` (lines 3463-3478) to take positions for both sides or add an output companion.
- Node height already accounts for the extra output row via `computation_io_row_count(..., variadic_outputs)` (lines 70-80) once the flag is set - verify.

## 5. React + catalogue

File: [flow/react/index.tsx](flow/react/index.tsx)

- Add `outputPorts?: readonly string[]` to the neuron widget TS type and its serialization (mirror `inputPorts` at lines 463 and 525). Interaction needs no JS logic: the `+` hit-test and `add_output_port` run entirely in Rust via `pointer_down_screen` (same as input-side today).
  File: [flow/play/index.ts](flow/play/index.ts)
- Optional: update the `list.get` catalogue summary (line 549) to reflect consecutive reads.

## 6. Verify

- Run flow-core Rust tests, list-module Rust tests, dag Rust tests, and flow react vitest via `nx` (per repo task-runner rules). Confirm runtime: build the flow play harness, drop a "Get", zoom in past 1.5, click the output `+`, confirm `i+1` appears and a downstream connection reads `list[index+1]` (with a `[DEBUG]` log in `Get::evaluate` to confirm count + emitted keys, removed after).
- Close the ticket with `ticket_close` summarizing all touched files.
