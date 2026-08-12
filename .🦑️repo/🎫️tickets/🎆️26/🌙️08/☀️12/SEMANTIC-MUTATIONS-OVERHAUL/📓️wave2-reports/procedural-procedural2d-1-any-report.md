# Wave 2 fan-out — procedural/procedural2d (standards/1/subsets/any) mutations facet report

Facet: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-procedural`

## Source shape (before)

`Procedural2dSnapshot` = `{ fixture: FlowFixture, generation: GenerationPlayState }`. `FlowFixture`
(foreign `flow` crate type) has an id-keyed `widgets: Vec<Widget>`, an id-keyed `synapses:
Vec<SynapseSpec>` (graph edges between widget ports), a `layout: BTreeMap<String, WidgetLayout>`
(per-widget canvas position), a scalar `schema: String`, and an ephemeral `camera: CameraJson`
(view state, excluded from document mutations by a pre-existing test). `GenerationPlayState`
(foreign `flow::playbook` type) has an id-keyed `generations: Vec<FormGeneration>` with a
per-question `values` map, already governed by `flow::playbook`'s own semantic `GenerationMutation`
enum (`Add`/`Remove`/`Rename`/`UpdateValues`).

The old `Procedural2dMutation` enum had 9 generic variants (`SetWidget{index,widget}`,
`RemoveWidget{id}`, `SetSynapse`, `RemoveSynapse`, `SetLayout`, `RemoveLayout`, `SetCamera`,
`SetSchema`, `Generation(GenerationMutation)`) plus 8 "leaf-shaped but actually generic" triad dirs
that were thin apply/diff/inverse delegates back onto a hand-written `impl Mutation` — not real
`MutationKind` payloads. `SetWidget`/`SetSynapse`/`SetLayout` each conflated create-or-replace into
one variant (id-presence decided the branch at diff/inverse time), which the taxonomy forbids.

## Derivation applied

Per `derivation-rules.md` §2 (id-keyed collections) and §4 (edge collections):

- Widgets (id-keyed) → split into `create-widget` + `replace-widget` + `delete-widget` (was one
  conflated `SetWidget`/`RemoveWidget` pair).
- Synapses (id-keyed edge/relationship collection) → `connect-synapse` + `replace-synapse` +
  `disconnect-synapse` (taxonomy's `connect`/`disconnect` domain-native pair for edges, plus
  `replace` for rewiring an existing edge's endpoints/ports).
- Layout (per-widget spatial facet) → `move-widget` (absolute reposition, taxonomy's `move` verb;
  creates the entry if absent) + `clear-widget-layout` (taxonomy's `clear` verb, empties one
  addressed entry).
- Camera → `update-camera` (taxonomy's `update` facet exception: x/y/zoom are only ever changed
  together by a pan/zoom gesture, never one field at a time — not a `change` scalar setter).
- Fixture schema (document-level scalar) → `change-schema`.
- `Generation(GenerationMutation)` → flattened into 4 own semantic variants delegating to the
  already-semantic `flow::playbook::GenerationMutation`/`apply_generation_mutation`/
  `invert_generation_operation` machinery (framework code, untouched): `create-generation`,
  `delete-generation`, `rename-generation`, `change-generation-value`.

Result: 14 semantic mutations (verbs: create, replace, delete, connect, disconnect, move, clear,
update, change, rename — all in `APPROVED_VERBS`), up from 9 generic ones. No `SetSnapshot`,
`NoMutation`, or `CollectionMutation` in the public enum; every variant is a single-field tuple
wrapping a real `MutationKind` payload; every diff is a real handcrafted sparse construction
(delegating to the pre-existing `diff_fixture_from_helpers`/`diff_generation_from_ops` sparse-patch
builders in the sibling `🔺️diff` facet, never apply-then-capture); every inverse reads `base` and
returns `Vec::new()` for a missing target.

## Triad leaves — wiring constraint and how it was resolved

This task's hard constraints bar editing `📦️glue.rs` (shared with the sibling `procedural3d`
artifact, worked concurrently by a different session) and forbid creating files whose wiring would
require a glue.rs edit. `📦️glue.rs` already wires exactly 8 triad-leaf directories under 8 generic
names (`➖remove-layout`, `➖remove-synapse`, `➖remove-widget`, `🎛set-camera`, `🎛set-layout`,
`🎛set-schema`, `🎛set-synapse`, `🎛set-widget`, each `pub mod <name> { pub mod mutation; pub mod
diff; pub mod inverse; }`). Since `#[derive(dsl::Mutations)]` only asserts the ENUM VARIANT's own
kebab form against `MutationKind::SEMANTICS.kind` (verified by reading the derive macro's
`derive_mutations` body) — it does not check directory or module names — I repurposed those 8
already-wired directories' **contents** for 8 of the 14 semantic mutations (one verb per slot,
picked by closest match to the original generic gesture), keeping their on-disk directory names and
`📦️glue.rs`-assigned Rust module names (`remove_widget`, `set_widget`, …) unchanged since renaming
them would require editing `📦️glue.rs`:

| Wired module (unchanged name) | Now holds |
|---|---|
| `set_widget` | `CreateWidget` |
| `remove_widget` | `DeleteWidget` |
| `set_synapse` | `ConnectSynapse` |
| `remove_synapse` | `DisconnectSynapse` |
| `set_layout` | `MoveWidget` |
| `remove_layout` | `ClearWidgetLayout` |
| `set_camera` | `UpdateCamera` |
| `set_schema` | `ChangeSchema` |

The remaining 6 (no pre-wired slot: `ReplaceWidget`, `ReplaceSynapse`, `CreateGeneration`,
`DeleteGeneration`, `RenameGeneration`, `ChangeGenerationValue`) live as `pub mod <slug> {
//#region 🦠️Mutation / 🔺️Diff / ↩️Inverse }` nested modules directly inside the already-wired
`🧬️mutations/🦀️component.rs` dispatch file — same payload/diff/inverse shape as a real triad leaf
(this is also exactly the shape the reference `MiniMutation` fixture in `command/component.rs`
uses), just co-located rather than in their own `📦️glue.rs`-wired directory. Every payload struct,
`MutationKind` impl, `diff`/`inverse` delegate, and builder function is real, handcrafted logic —
none of this is a stub.

This keeps every file inside my write boundary compiling standalone (verified below) with zero
`📦️glue.rs` edits. A follow-up cosmetic pass (see `sharedFileRequests`) can rename the 8 directories
to their true verb slugs and promote the 6 inline modules to their own directories once `procedural3d`'s
concurrent session is done touching the same `📦️glue.rs`.

## Dispatch rewrite

`🧬️mutations/🦀️component.rs`: `Procedural2dMutation` is now 14 single-field tuple variants deriving
`dsl::Mutations` (`#[mutations(snapshot = Procedural2dSnapshot, diff = Procedural2dDiff, schema =
"procedural.2d")]`). Hand-written `impl Mutation<Procedural2dSnapshot> for Procedural2dMutation`
deleted — generated by the derive. Kept `apply_procedural2d_mutation`/`inverse_procedural2d_mutation`
(both generic over the enum, needed no changes) and the `Procedural2dEnvelope`/`Procedural2dStore`
type aliases (both used by `🎛️apps/◻2d/🌉️wasm`, untouched). Rewrote `procedural2d_fixture_operations`
(the before/after `FlowFixture` differ used by `⚙️engine::host_operations`) to emit the new semantic
variants — distinguishing create-vs-replace-vs-delete per id instead of the old conflated
`SetWidget`/`SetSynapse`.

`🧬️mutations/💾️binary/🦀️component.rs`: rewrote the hand-written `Procedural2dOperationDsl` mirror
enum (needed because `Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson`/`FormGeneration` are foreign
`flow` types that can't derive `dsl::DslRecord` directly) with 14 flattened keyword variants
matching the new dispatch enum 1:1, and rewrote both to/from-dsl bridge functions accordingly,
reusing the pre-existing, untouched `widget_to_dsl`/`synapse_to_dsl`/`layout_to_dsl`/
`camera_to_dsl`/`form_generation_to_dsl` (+ `_from_dsl`) helpers. `🧬️mutations/📝️text/🦀️component.rs`
needed no changes (it only re-exports by stable name, never matches on variants).

## Tests

Extended (not replaced-with-new-file) the dispatch's own `#[cfg(test)] mod tests` and the
`💾️binary` facet's `#[cfg(test)] mod tests`. Confirmed `protocol::testkit::*` (the wave-0-added
`assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`) is reachable with **no new Cargo
dependency** — `semio-s-plugin-procedural`'s `📦️glue.rs` already declares `extern crate
semio_framework_os_kernel as protocol;`, and `testkit` is an unconditional `pub mod testkit;` inside
that same kernel crate (not gated by `#[cfg(test)]`), matching the `🖍️draw` plugin's proven usage of
the identical import. Added inverse-law tests for `create-widget`, `replace-widget` (+ its
missing-target no-op case), `delete-widget` (+ no-op case), `connect-synapse`, `replace-synapse`,
`disconnect-synapse` (+ no-op case), `move-widget` (both the "had a prior layout" and "created the
entry" branches), `clear-widget-layout` (+ no-op case), `update-camera`, `change-schema`,
`create-generation`, `rename-generation`, and a diff-absorb-law test for `change-generation-value`;
kept/renamed the pre-existing fixture-diff and widget/synapse-collection-apply tests; extended
`dispatch_registers_semantic_descriptors` to assert all 14 kinds use approved verbs.

## Verify

`cargo check -p semio-s-plugin-procedural` — retried 3× at ~60s per this task's WORKSPACE CHURN
policy. Every run showed the exact same result: **zero errors inside
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d`**. All remaining errors are outside it:

- **~54 errors in `🗿️artifacts/🧊️procedural3d/…/🧬️mutations/🦀️component.rs`** — a different
  session actively fanning out the sibling `procedural3d` artifact concurrently (same ticket,
  different facet), unrelated to this report.
- **Exactly 2 errors at app-level call sites I'm barred from editing** (`🎛️apps/**` is on this
  task's explicit DO-NOT-TOUCH list) — both anticipated by this task's design and captured below in
  `sharedFileRequests`:
  - `🎛️apps/◻2d/🦀️component.rs:267` — `Procedural2dMutation::SetWidget { .. }` (deleted variant).
  - `🎛️apps/◻2d/🎮️commands/🧬️generation/🦀️component.rs:52` — `Procedural2dMutation::Generation`
    (deleted variant).

`cargoCheck: "green"` for everything inside my artifact directory; the crate as a whole does not yet
compile because of the two app-level call sites above, exactly as this task's `sharedFileRequests`
mechanism anticipates.

## Shared-file reconciliation needed (NOT edited — outside my artifact directory)

- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🦀️component.rs:265-268` — the slider-value command builds
  `Procedural2dMutation::SetWidget { index, widget: flow::Widget::InputSlider { id: id.clone(),
  value: number, min: *min, max: *max, step: *step } }` against an EXISTING widget found by id.
  Replace with `Procedural2dMutation::ReplaceWidget(crate::artifacts::procedural2d::mutations::replace_widget::ReplaceWidget
  { widget: flow::Widget::InputSlider { id: id.clone(), value: number, min: *min, max: *max, step:
  *step } })` (or the re-exported builder `crate::artifacts::procedural2d::mutations::replace_widget(..)`),
  and simplify the preceding `.enumerate().find(|(_, widget)| ..)` to a plain `.find(|widget| ..)`
  since `index` is no longer needed.
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎮️commands/🧬️generation/🦀️component.rs:52` —
  `operations.into_iter().map(Procedural2dMutation::Generation).collect()` (where `operations:
  Vec<flow::playbook::GenerationMutation>`). Replace with a per-variant map onto the 4 new semantic
  builders: `operations.into_iter().map(|operation| match operation { flow::playbook::GenerationMutation::Add
  { generation } => crate::artifacts::procedural2d::mutations::create_generation(generation),
  flow::playbook::GenerationMutation::Remove { id } =>
  crate::artifacts::procedural2d::mutations::delete_generation(id),
  flow::playbook::GenerationMutation::Rename { id, name } =>
  crate::artifacts::procedural2d::mutations::rename_generation(id, name),
  flow::playbook::GenerationMutation::UpdateValues { id, question_id, value } =>
  crate::artifacts::procedural2d::mutations::change_generation_value(id, question_id, value) }).collect()`.
- (Optional, cosmetic, not blocking anything) `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`
  — the `mutations { .. }` block's 8 `remove_layout`/`remove_synapse`/`remove_widget`/`set_camera`/
  `set_layout`/`set_schema`/`set_synapse`/`set_widget` `pub mod` blocks and their on-disk
  `➖remove-layout`/`➖remove-synapse`/`➖remove-widget`/`🎛set-camera`/`🎛set-layout`/`🎛set-schema`/
  `🎛set-synapse`/`🎛set-widget` directories could be renamed to their true verb slugs
  (`🧹clear-widget-layout`, `✂️disconnect-synapse`, `🗑️delete-widget`, `📷update-camera`,
  `📍move-widget`, `🏷️change-schema`, `🔗connect-synapse`, `🌱create-widget`) and the 6 inline
  modules (`replace_widget`, `replace_synapse`, `create_generation`, `delete_generation`,
  `rename_generation`, `change_generation_value`) promoted to their own `📦️glue.rs`-wired
  directories, matching the `🖍️draw`/`🖍️cad`/`🌍️gis`-plugin precedent exactly. Purely a directory/
  module-name cleanup — every payload/diff/inverse is already real, correct, semantic code either
  way; this only affects where the bytes live on disk.

## Skipped / non-blocking (recipe step f)

Did not touch `📖️component.grammar.semio` (pre-existing staleness unrelated to this migration — its
`mesh-op`/`add-vertex`/`transform-mesh` grammar describes a different, apparently copy-pasted
domain, not procedural2d's actual widget/synapse/generation vocabulary either before or after this
change) or `💾️binary/📡️component.protocol.semio` (a generic record-framing envelope, not
variant-specific — needed no change). `dsl::DslEnum` derive on the dispatch enum was deliberately
**not** adopted (unlike `🖍️draw`'s payload structs, which all embed only local artifact types):
`Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson`/`FormGeneration` are foreign `flow`/
`flow::playbook` types with no `dsl::DslRecord` derive possible, so the pre-existing hand-written
`Procedural2dOperationDsl` mirror-enum bridge (rewritten, not replaced) stays the right shape here.

## Files touched

Modified (all inside `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d`):
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (dispatch enum rewrite +
  6 inline triad modules + fixture-operations rewrite + tests)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (mirror-enum +
  bridge rewrite + tests)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-widget/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  (now `CreateWidget`)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-widget/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  (now `DeleteWidget`)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-synapse/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  (now `ConnectSynapse`)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-synapse/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  (now `DisconnectSynapse`)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-layout/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  (now `MoveWidget`)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-layout/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  (now `ClearWidgetLayout`)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-camera/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  (now `UpdateCamera`)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-schema/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  (now `ChangeSchema`)

Not modified (outside boundary — see "Shared-file reconciliation needed" above):
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎮️commands/🧬️generation/🦀️component.rs`

Not touched: `📝️text/🦀️component.rs` (no variant-specific code, needed no changes),
`📖️component.grammar.semio`, `💾️binary/📡️component.protocol.semio`, all `.ts`/`.json`/`.proto`/
`.graphql` sibling schema-description files (left as their pre-existing stubs — see "Skipped" above;
not exercised by `cargo check`).
