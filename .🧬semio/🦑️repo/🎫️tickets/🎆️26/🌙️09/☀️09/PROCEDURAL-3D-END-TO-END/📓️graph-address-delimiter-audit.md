# Graph Address Delimiter Audit

Read-only source audit, 2026-11-21. No Cargo or production changes. The codec and its supported directional-channel behavior are described in `📓️directional-graph-port-hover.md`.

## Admission

Flow `Widget` and `SynapseSpec` in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🧱️document-model/🦀️.rs` store identities and channel names as `String`. The explicit descriptor path `…/🧩️widget-descriptors/🦀️.rs` trims the ID and rejects only empty text; `…/🖥️host/✏️host-authoring/🧭️domain-plans/🦀️.rs` checks duplicate widget ID, not delimiter characters, when adding it. Thus a short ID such as `a@b` or `a#b` is admitted by this authoring path. Generated UI IDs happen to be simpler; that is not the admitted document contract.

Plugin `ChannelSpec.name` is also a `String`; `ChannelSpec::requires/provides/named` and `Registry::register_operator` do not reject `@`, `#`, `!i` or `!o` in the inspected code: `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/⚙️operator-record/🦀️.rs`, `…/⚙️engine/📔️registry/🦀️.rs`. Plugin-provided channels containing those characters are therefore supported by these APIs, though this audit did not locate a built-in operator using them. No global identifier character or byte bound was found in these admission paths. UI `UiText` has a 512-byte UTF-8 budget (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📏️budget/🧬️schema/🔣️.json`); that distinct rendering budget does not ban delimiters and the examples below are well within it. This is not a claim that arbitrarily long identifiers successfully render everywhere.

## Actual supported reproducer

Use the normal explicit widget descriptor with a short ID and the built-in Box operator:

`{ "kind": "neuron", "id": "a@b", "neuronKind": "brep.prim3d.box" }`

The descriptor's concrete schema fields are defined by `WidgetDescriptor` and the authoring plan above. Box publishes the standard output `solid` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🔶️primitives-curves-surfaces-and-sweeps/🦀️.rs`). Its output instance address is `a@b@solid#0`. `widget_id_from_instance_id` in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs` returns `a`, not `a@b`. `✏️editor/🎮️commands/🧭️transforms/🦀️.rs::ensure_gumball_node` calls that decoder and looks for widget `a`, so translating this selected built-in box refuses as `NoShapeSource` (or targets the wrong node if `a` exists). `✏️editor/🎯️selection/🦀️.rs::ComponentTarget::parse` splits at the first `@` then refuses the remaining `b@solid` channel, so a face/edge/vertex selection on that box is rejected. An explicit `a#b` ID fails similarly because widget extraction splits at the first `#`. This is source-derived, not a mounted test execution.

## Other address paths

- `🧵️preview-eval/🦀️.rs::split_endpoint` splits the first `@`; an endpoint for widget `a@b` and channel `solid` becomes node `a`, port `b@solid`. The projected graph edge therefore references the wrong node. `✏️editor/🎮️commands/🧭️navigate-graph/🦀️.rs` and `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` likewise split at the first delimiter.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs::operator_channel_code` uses the *last* `@`, so a channel `x@y` is shortened to `y` when deriving a catalogue row from projected port IDs. A channel `x#y` is rejected by `ComponentTarget::parse`; transform selection truncates it at `#`.
- The directional codec in `🧬️schema/🔌️graph-ports/🦀️.rs` does escape leading `!` on outputs and prefixes inputs, so `graph_port_channel(graph_port_code(name, side))` recovers names like `!iX` and `!oX` when applied once to an encoded code. `nodeGraphEdit` does that to separate source/target port IDs before `GraphEditor::connect_ports`; no delimiter split is required there. However `ensure_gumball_node` compares its selected encoded port directly to the authored output name. An output channel `!iX` becomes `!o!iX` and fails that comparison. Do not infer nodeGraphEdit is itself broken solely by prefix escaping.

## Fix and test scope

Use one unambiguous, length-delimited or escaped shared address codec for widget, direction, channel and instance index, rather than relying on mixed first/last delimiter splitting. Preserve structured node and port fields in graph edge projection where possible, rather than re-parsing concatenated endpoints. Update producer and all consumers together: `🧬️schema/🔌️graph-ports/🦀️.rs`, `🧵️preview-eval/🦀️.rs`, `🧵️preview-eval/🧊️preview-geometry/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/{🕸️flow,👁️preview}/🦀️.rs`, `✏️editor/🎯️selection/🦀️.rs`, `✏️editor/🎮️commands/🧭️{transforms,navigate-graph}/🦀️.rs`, `🚪️io/📝️text/📸️snapshot/🦀️.rs`, plus viewer and browser/WGPU scene adapters that publish or interpret these IDs. Avoid changing authored IDs or plugin channels as a workaround.

Add language-neutral fixtures for widget IDs `a@b`, `a#b`; channel names `x@y`, `x#y`, `!iX`, `!oX`, empty and non-ASCII; both directions and instance indices. Assert injective round trips, graph edge endpoint identity, hover/selection, component parsing, gumball output choice, export and navigation, including a collision with an actual widget `a`. Existing relevant tests are in `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/🎯️selection` and the directional hover ticket fixtures. Check both mounted renderers after the owned app build is available.

Unqualified relative paths in the last sections refer to `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`.
