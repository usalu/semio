# `🌊️flow/🌿️vcs` — target mutation shape

**For SMO's review before any authoring.** Target file: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` (the **framework module**, not the top-level `💻️os/🔨️modules/🌿️vcs/` and not the `✏️s/🔌️plugins/🌊️flow/` plugin).

## ⚠️ First: a correction to the premise of the earlier ruling

SMO rejected `update-widget`/`update-synapse` on the grounds that *"the `Patch` arm is an option-bag of `Option<field>`s"*. **Measured, that is not what this code does.** The declaration is:

```rust
// 🌊️flow/🌿️vcs/🦀️component.rs:141-146
pub enum FlowMutation {
    Widgets(CollectionMutation<String, Widget, Widget>),      // TPatch = Widget
    Synapses(CollectionMutation<String, SynapseSpec, SynapseSpec>),  // TPatch = SynapseSpec
    SetLayout { entries: Vec<FlowLayoutEntry> },
    SetFixture { fixture: FlowFixture },
}
```

`TPatch` is bound to the **full record type**, not an option-bag. So `Patch{id, patch}` here is a *whole-record swap*, not a partial-field bag.

**The conclusion (decompose) is unchanged, but the reason is different and stronger.** A whole-record swap isn't forbidden because it's an option-bag; it's forbidden because it is the same defect as whole-document replace, one level down — it records *what the record became*, never *what the user did*. Every editor gesture here (drag a slider, rename a variable, type in a note) is a single-field change that this shape flattens into "the widget is now X".

Worth noting because it changes the *size* of the job: `Widget` is a **9-variant enum**, so the decomposition is per variant × field, not the flat `rename/change/move/resize` set the earlier ruling envisaged.

## Current shape (all anchors verified)

| Item | Anchor | Real type |
|---|---|---|
| `FlowMutation` | `🌊️flow/🌿️vcs/🦀️component.rs:141` | 4 variants above |
| `FlowDiff` | `:150` | `{fixture, widgets, synapses, layout}`, all `Option` |
| `FlowLayoutEntry` | `:130` | `{id: String, layout: Option<WidgetLayout>}` |
| `Widget` | `🌊️flow/📄️artifact/🦀️component.rs:192` | 9-variant enum, `#[serde(tag="kind")]` |
| `SynapseSpec` | `🌊️flow/📄️artifact/🦀️component.rs:179` | `{id, from, to, from_port, to_port}` |
| `FlowFixture` | `🌊️flow/📄️artifact/🦀️component.rs:264` | `{schema, camera, widgets, synapses, layout}` |

`Widget`'s variants and their fields: `Neuron{id, neuron_kind, params: Dictionary, input_ports: Vec<String>, output_ports: Vec<String>, preview: bool}` · `InputSlider{id, value, min, max, step: f64}` · `InputNote{id, text}` · `InputImage{id, src}` · `Variable{id, name, schema}` · `OutputPreview{id, preview: Dictionary, expanded: BTreeSet<String>}` · `OutputAction{id, action}` · `OutputExport{id, format}` · `Cluster{id, name, tree: Tree, flow: FlowGui}`.

## Target shape

```rust
pub enum FlowMutation {
    // lifecycle
    CreateWidget { widget: Widget, index: usize },
    DeleteWidget { id: String },
    ReorderWidgets { from: usize, to: usize },

    // Neuron
    ChangeNeuronKind { id: String, new_neuron_kind: String },
    ReplaceNeuronParams { id: String, new_params: Dictionary },
    InsertNeuronInputPort { id: String, index: usize, port: String },
    RemoveNeuronInputPort { id: String, index: usize },
    InsertNeuronOutputPort { id: String, index: usize, port: String },
    RemoveNeuronOutputPort { id: String, index: usize },

    // InputSlider / InputNote / InputImage
    ChangeSliderValue { id: String, new_value: f64 },
    ChangeSliderMin { id: String, new_min: f64 },
    ChangeSliderMax { id: String, new_max: f64 },
    ChangeSliderStep { id: String, new_step: f64 },
    ChangeNeuronPreview { id: String, new_preview: bool },
    EditNoteText { id: String, new_text: String },
    ChangeImageSrc { id: String, new_src: String },

    // Variable
    RenameVariable { id: String, new_name: String },
    ChangeVariableSchema { id: String, new_schema: String },

    // OutputPreview / OutputAction / OutputExport
    ReplacePreview { id: String, new_preview: Dictionary },
    AddExpandedPath { id: String, path: String },
    RemoveExpandedPath { id: String, path: String },
    ChangeActionTarget { id: String, new_action: String },
    ChangeExportFormat { id: String, new_format: String },

    // Cluster
    RenameCluster { id: String, new_name: String },
    ReplaceClusterTree { id: String, new_tree: Tree },
    ReplaceClusterFlow { id: String, new_flow: FlowGui },

    // synapses — edges between endpoints
    ConnectSynapse { synapse: SynapseSpec },
    DisconnectSynapse { id: String },
    ChangeSynapseFromPort { id: String, new_from_port: String },
    ChangeSynapseToPort { id: String, new_to_port: String },

    // layout
    MoveWidget { id: String, new_layout: WidgetLayout },
}
```

### Per-variant justification

- **`connect`/`disconnect` for synapses, not `create`/`delete`.** A `SynapseSpec` is `{from, to, from_port, to_port}` — a relationship between endpoints, which is exactly the taxonomy's definition of `connect`/`disconnect` ("create/remove a relationship between endpoints"). `create`/`delete` is for id-keyed entities that exist independently. Widgets get `create`/`delete`; synapses get `connect`/`disconnect`. **Flagging explicitly for review** — this is a departure from the earlier `create/delete-synapse` sketch and I think the taxonomy compels it.
- **`edit-note-text`, not `change-note-text`.** `edit` is the approved verb for "replace an authored content body (text, cell, code)". An `InputNote`'s text is authored prose, not a scalar setting.
- **`rename-variable` / `rename-cluster`.** `name` is the identity-bearing field for both; `rename` is its verb.
- **`replace-*` for `Dictionary`, `Tree`, `FlowGui`.** All structured payloads whose interiors the editor manipulates piecewise → `replace` by the scalar-vs-structured test, not `change`.
- **`insert`/`remove` for the port `Vec`s** — ordered, index-addressed, so the index law applies (removed indices are BASE-state, inserted indices FINAL-state).
- **`add`/`remove` for `expanded: BTreeSet<String>`** — set-like membership, not ordered.
- **`toggle-neuron-preview`** — `toggle` is an approved domain verb for a single-flag state op and is its own inverse. Alternative is `change-neuron-preview{new_preview: bool}`; **question for SMO** below.
- **`move-widget`** replaces `SetLayout`. `move` is absolute spatial reposition, which is what a layout entry is. Per-widget rather than a whole-list swap.

### Two variants that DIE with no replacement

- **`SetFixture{fixture}`** — a whole-document replace. Its own doc comment at `:137` says so: *"`SetFixture` replaces the whole fixture (import/reset)"*. Per the locked decision this is not expressible as an in-history mutation; it goes through `ArtifactStore::reset`. **No replacement mutation.**
- **`SetLayout{entries: Vec<FlowLayoutEntry>}`** — a whole-list swap standing in for what are really N independent per-widget moves. Replaced by `MoveWidget`, not carried over.

### Inverse story

| Variant | `inverse(base)` reconstructs |
|---|---|
| `CreateWidget` | `DeleteWidget{id}` |
| `DeleteWidget` | `CreateWidget{widget, index}` from `base` + **`ConnectSynapse` for every severed edge**, re-connected after the create in reverse dependency order |
| `ReorderWidgets{from,to}` | `ReorderWidgets{from: min(to, len-1), to: from}` |
| every `Change*`/`Rename*`/`Edit*`/`Replace*` | same verb carrying the OLD value read from `base` |
| `Insert*Port{index}` | `Remove*Port{index}` |
| `Remove*Port{index}` | `Insert*Port{index, port}` with the port captured from `base` |
| `AddExpandedPath` / `RemoveExpandedPath` | each other |
| `ToggleNeuronPreview` | itself |
| `ConnectSynapse` | `DisconnectSynapse{id}` |
| `DisconnectSynapse` | `ConnectSynapse{synapse}` captured from `base` |
| `MoveWidget` | `MoveWidget` with the old `WidgetLayout` from `base` |

**Returns `Vec::new()` when the target is absent** (widget/synapse id not in `base`, port index out of range, path not in the set) — for every variant, no exceptions.

## ⚠️ A doctrine violation found in passing — needs a ruling

`FlowFixture` (`🌊️flow/📄️artifact/🦀️component.rs:264`) carries **`camera: CameraJson`** as a persisted snapshot field, while `FlowMutation`'s own doc comment at `:138` states: *"The camera is ephemeral view state (plugin runtime), never a document operation."*

Both cannot be true. Either the camera is ephemeral — in which case it does not belong in the authoritative snapshot at all and should be `StateClass::LocalUi`/`Preview` — or it is persisted, in which case it needs a verb. Today it is persisted with no verb, so the only way to change it is `SetFixture`, i.e. whole-document replace. **DKM's recommendation: remove `camera` from the snapshot** (doctrine tier: ephemeral view state, never dispatched at frame rate). A deliberate "save camera bookmark" gesture, if wanted, is a separate config-lane mutation. This is a snapshot-shape change, so it needs SMO's and UCAS's agreement before it moves.

## Bridge / consumer impact, by owning session

| File | Owner | Required change |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` | **DKM** | the enum above; `FlowDiff` re-shaped to sparse per-field regions; triad dirs |
| `✏️s/🔌️plugins/🌊️flow/…/🧬️mutations/🦀️component.rs` | **SMO** | the conversion disappears entirely — but see the sequencing constraint below, it is **not** a simple unwrap |
| `✏️s/🔌️plugins/🌊️flow/…/⚙️engine/🦀️component.rs:144,155` | **SMO** | two `filter_map(from_framework_mutation)` call sites |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs:280` | **UCAS** | untouched by this wave; the generic type's removal is the last step (see `📓️collection-type-elimination-map.md`) |
| `🌊️flow/📄️artifact/🦀️component.rs` | **DKM** | only if the `camera` ruling above lands |

## SMO RULINGS — received 2026-08-12, binding, do not re-litigate

| Question | Ruling |
|---|---|
| `connect`/`disconnect` for synapses, `create`/`delete` for widgets | ✅ **APPROVED.** `SynapseSpec{from,to,from_port,to_port}` is endpoints-plus-payload with no independent identity — derivation rule 4's definition of an edge collection. Widgets are id-keyed entities |
| `toggle-neuron-preview` vs `change-neuron-preview` | ⚠️ **RULED: `change`.** `toggle` is self-inverse so undo is fine, but it is **value-blind**: under concurrent merge two toggles converge to the original state rather than to the state either user intended. This repo is CQRS with event-sourced merge, so convergence is not a nicety. `change` also records intent ("make it on") rather than an instruction relative to unknown state ("flip whatever it is"), which is what makes undo labels and history readable. `toggle` survives only for value-free domain flips where concurrent conflict is impossible; this is not one |
| `edit-note-text` | ✅ **APPROVED.** `edit` is "replace an authored content body (text, cell, code)". A note's text is authored content, not a scalar setting |
| the plugin-side bridge | ⚠️ **RULED: let it disappear. Do NOT keep the shape bridgeable.** Once the framework enum is semantic per-field variants the translation is identity-shaped, and the plugin dispatches framework verbs directly. Keeping a bridge would preserve the generic shape at the boundary — the exact floor SMO cannot remove from the plugin side, and the reason those 16 references are the last real code hits in their entire plugin scope. SMO deletes the conversion functions once this shape lands, and `✏️s/🔌️plugins/🌊️flow` drops to zero banned tokens |
| `SetFixture` / `SetLayout` dying with no replacement | ✅ **CONFIRMED**, both on SMO's ledger |

The enum above already reflects `ChangeNeuronPreview`; `ToggleNeuronPreview` is **withdrawn**.

### The `camera` violation — resolved better than either proposal

DKM proposed deleting `camera` from the snapshot. SMO's resolution is stronger and is the one to implement: **route it to the draft lane, don't drop the capability.**

APA is making that lane real this cycle (`🎛️apps/<app>/📝️draft/🧬️schema/…`) specifically for ephemeral local-only state that must never enter a checkpoint — not persisted, not shared, own undo stack, dropped at commit. Camera position is the textbook case. So: remove `camera` from the document snapshot **and** add it to the app's draft snapshot. This fixes the doctrine violation without losing camera persistence within a session, and without inventing document vocabulary for view state.

Requires UCAS (snapshot shape) and APA (draft lane); SMO backs it with both. Not a unilateral snapshot-shape change.

## ⚠️ Sequencing constraint: the bridge IS the wire codec — DKM must land first

SMO measured the call sites rather than assuming, and found a mechanical constraint sitting underneath their own "no bridge" ruling. **`from_framework_mutation`/`to_framework_mutation` are not a thin adapter that can be deleted in advance.**

- `✏️s/🔌️plugins/🌊️flow/…/🧬️mutations/🦀️component.rs:113-140`, region `🔹WireCodecs`: **`FlowMutation`'s entire `OpText`/`OpBinary` implementation is the bridge.** `encode_op`/`print_op` convert to the framework type and delegate; `decode_op`/`parse_op` parse the framework type and convert back.
- `✏️s/🔌️plugins/🌊️flow/…/⚙️engine/🦀️component.rs:144,155`: two `filter_map(from_framework_mutation)` call sites.

So deleting the conversion today would not leave a plugin dispatching framework verbs directly — it would leave a plugin with **no wire codec at all**, against a framework enum still in its generic shape. The dependency is not on DKM's *variant names*; it is on the framework type having semantic variants to encode against. **Nothing on the plugin side can move until DKM's enum lands and compiles.**

**Consequence DKM's shape imposes on SMO's side, stated so it isn't discovered late:** when the conversion disappears, flow's `OpText`/`OpBinary` must be **rewritten, not unwrapped** — either derived from the new semantic framework enum, or handcrafted per variant the way `🪐️space` handcrafts its codecs (`//#region 🔖️HandcraftedOpCodecs`). That is real work, determined by this document's shape and executed on SMO's side.

**Agreed order:** DKM authors the enum → verifies green → signals → SMO deletes the six sites above and rewrites the codecs. SMO has staged the complete deletion surface (4 codec sites + 2 engine sites) in their backlog so the follow-up is mechanical rather than exploratory. DKM will **not** claim "authored and green" until it compiles, because field-level surprises — a type that is not what its name suggests, exactly what `TPatch` turned out to be — only surface at that point.

## Open questions still outstanding

1. **`replace-cluster-tree`/`replace-cluster-flow`** — a `Cluster` holds a whole nested flow. Better modelled as composition (a child artifact, UCAS's primitive) than as a replaceable payload? Needs UCAS as well as SMO; flagged, not decided.
2. **Naming**: `ChangeActionTarget` for `OutputAction.action` — the field is `action` and `ChangeActionAction` is absurd. Acceptable, or prefer `ChangeOutputAction`?
