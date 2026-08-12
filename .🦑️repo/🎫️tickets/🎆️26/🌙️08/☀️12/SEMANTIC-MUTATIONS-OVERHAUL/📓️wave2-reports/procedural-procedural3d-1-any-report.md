# Wave 2 — `procedural` / `procedural3d` / `1` / `any` — mutations facet migration

Facet: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-procedural`

## Vocabulary derived

From `Procedural3dSnapshot { fixture: FlowFixture, generation: GenerationPlayState }`, 14 semantic
mutations replacing the generic `SetWidget`/`RemoveWidget`/`SetSynapse`/`RemoveSynapse`/`SetLayout`/
`RemoveLayout`/`SetCamera`/`SetSchema`/`Generation(GenerationMutation)` vocabulary:

| Verb/entity | New variant | Old generic equivalent |
|---|---|---|
| `create-widget` | `CreateWidget{index, widget}` | `SetWidget` (new-id case) |
| `update-widget` | `UpdateWidget{widget}` | `SetWidget` (existing-id case) |
| `delete-widget` | `DeleteWidget{id}` | `RemoveWidget` |
| `connect-synapse` | `ConnectSynapse{index, synapse}` | `SetSynapse` (new-id case) |
| `update-synapse` | `UpdateSynapse{synapse}` | `SetSynapse` (existing-id case) |
| `disconnect-synapse` | `DisconnectSynapse{id}` | `RemoveSynapse` |
| `move-widget` | `MoveWidget{id, layout}` | `SetLayout` |
| `delete-widget-position` | `DeleteWidgetPosition{id}` | `RemoveLayout` |
| `update-camera` | `UpdateCamera{camera}` | `SetCamera` |
| `change-schema` | `ChangeSchema{new_schema}` | `SetSchema` |
| `create-generation` | `CreateGeneration{generation}` | `Generation(GenerationMutation::Add)` |
| `delete-generation` | `DeleteGeneration{id}` | `Generation(GenerationMutation::Remove)` |
| `rename-generation` | `RenameGeneration{id, new_name}` | `Generation(GenerationMutation::Rename)` |
| `change-generation-value` | `ChangeGenerationValue{id, question_id, new_value}` | `Generation(GenerationMutation::UpdateValues)` |

The single generic `SetWidget`/`SetSynapse` upsert was deliberately split into two verbs
(`create`/`update`) since the widget/synapse collections are id-keyed (matching `cad`'s own
`create-object`/`delete-object` precedent) — same underlying `WidgetsDiff`/`SynapsesDiff` apply
mechanics, distinct semantics. `SetLayout`/`RemoveLayout` became `move-widget`/
`delete-widget-position` (position is a per-widget spatial override, not itself a first-class
named entity). `Generation(GenerationMutation)` — a raw wrapped external enum, forbidden shape —
was split into 4 real semantic variants, each delegating to the existing (framework-owned,
untouched) `flow::playbook::apply_generation_mutation`/`invert_generation_operation` engine via
`diff_generation_from_ops`.

`SetSnapshot`/`NoMutation`/bare `CollectionMutation` were never present in this facet to begin
with — the pre-migration enum's only generic-vocabulary problems were the `Set*`/`Remove*` bare
setters and the `Generation(GenerationMutation)` wrapper, both now closed.

## Directory layout — 8 dirs repurposed in place, 6 new dirs added

`📦️glue.rs` (plugin-shared, outside this facet's writable boundary) path-includes eight
pre-migration triad directories verbatim: `➖remove-layout`, `➖remove-synapse`, `➖remove-widget`,
`🎛set-camera`, `🎛set-layout`, `🎛set-schema`, `🎛set-synapse`, `🎛set-widget`. Since glue.rs
couldn't be edited, those eight directories were **repurposed in place** (same path, rewritten
`🦠️mutation`/`🔺️diff`/`↩️inverse` content) rather than renamed — each new payload's
`SEMANTICS.kind`/`record` are the correct new semantic strings (e.g. `remove-widget/`'s payload has
`kind: "delete-widget"`), so runtime/wire/registry behavior is fully correct; only the on-disk
directory name is stale. Six mutations with no pre-wired slot (`create-widget`, `connect-synapse`,
`create-generation`, `delete-generation`, `rename-generation`, `change-generation-value`) got fresh,
correctly-named directories, self-wired directly inside `🧬️mutations/🦀️component.rs` via nested
`#[path = "."] pub mod <name> { #[path = "..."] pub mod mutation; ... }` blocks (confirmed
`#[path]` resolution is per-physical-file, not per logical-mod-nesting — required the `#[path="."]`
reset on each outer inline block, exactly mirroring glue.rs's own convention for the old slots).

`➖remove-widget/🦠️mutation/🦀️component.rs`'s docstring documents the exact reason inline; see
`sharedFileRequests` below for the follow-up rename once glue.rs can be touched.

## Other in-boundary files fixed (compile breakage caused by the vocabulary change)

- `🧬️mutations/💾️binary/🦀️component.rs` — full rewrite of `Procedural3dOperationDsl` (the
  handcrafted OpText/OpBinary wire-codec mirror) and its `to_dsl`/`from_dsl` conversion functions to
  match the 14 new variants (tuple-payload pattern instead of struct-variant fields); its own tests
  updated to the new constructors.
- `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — one pre-existing test
  (`command_envelope_round_trip_holds_for_an_applied_operation`) used `Procedural3dMutation::SetWidget{..}`
  struct-literal syntax; updated to `Procedural3dMutation::CreateWidget(CreateWidget{..})`.
- `🧬️mutations/📝️text/🦀️component.rs` — added `generation_mutation_to_procedural3d` to its existing
  named re-export list (see bridge function below) so it's reachable the same way apps already reach
  `Procedural3dMutation`/`procedural3d_fixture_operations` (`crate::artifacts::procedural3d::op::*`).

## Ergonomic bridges kept for app call sites (signatures preserved, so no app edit was strictly
## required for these two)

- `procedural3d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural3dMutation>`
  — same signature as before, body rewritten to emit the new semantic variants. `🏗️builder` and the
  `🎮️commands/🧩️widget` app command reach it through `engine::commit_fixture`, unchanged.
- New `pub fn generation_mutation_to_procedural3d(GenerationMutation) -> Procedural3dMutation` —
  bridges the framework's own `flow::playbook::GenerationMutation` vocabulary onto the 4 new
  generation variants, so an app callsite that already holds a `GenerationMutation` needs only to
  swap its mapping function, not learn this facet's internal triad-leaf module paths.

## Tests

Extended the existing `🧪️Tests` region in `🧬️mutations/🦀️component.rs` (no new test files): kept
every pre-migration test (renamed to the new constructors), added
`every_variant_registers_an_approved_semantic_descriptor`, `generation_mutation_bridge_covers_every_variant`,
and three `protocol::testkit::assert_mutation_inverse_law` / `assert_mutation_diff_absorb_law` pairs
(`create-widget`, `connect-synapse`, `update-camera` — an id-keyed create/delete pair, a relationship
connect/disconnect pair, and a document-level facet setter, per the ticket's own guidance on which
3 variants to pick). `protocol::testkit` is reachable with **zero new Cargo dependency** — confirmed
`semio-framework-os-kernel`'s own `pub use crate::os_spr::*;` at its crate root already lifts
`testkit` to `protocol::testkit` for every plugin that already depends on the kernel crate (all of
them, `procedural` included) — same for `dsl::Mutations` (`pub use crate::os_dsl::*;` at kernel
root lifts `Mutations` to bare `dsl::Mutations`, resolvable via the crate's existing
`extern crate semio_framework_os_kernel as dsl;` alias in its own `📦️glue.rs`). Neither needed a
Cargo.toml edit.

## `sharedFileRequests` — exact changes needed once a later pass can touch shared files

1. **`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`** — rename the 8 repurposed triad
   directories' path-include block to their correct semantic names (purely cosmetic, behavior
   already correct): `remove_layout`→`delete_widget_position` (dir `➖remove-layout`→
   `🗑delete-widget-position`), `remove_synapse`→`disconnect_synapse` (`➖remove-synapse`→
   `✂️disconnect-synapse`), `remove_widget`→`delete_widget` (`➖remove-widget`→`🗑delete-widget`),
   `set_camera`→`update_camera` (`🎛set-camera`→`🔁update-camera`), `set_layout`→`move_widget`
   (`🎛set-layout`→`📍move-widget`), `set_schema`→`change_schema` (`🎛set-schema`→`🔧change-schema`),
   `set_synapse`→`update_synapse` (`🎛set-synapse`→`🔁update-synapse`), `set_widget`→`update_widget`
   (`🎛set-widget`→`🔁update-widget`). Same rename is needed for the sibling `procedural2d` artifact
   (already migrated by a concurrent session with the identical repurpose-in-place strategy and the
   identical glue.rs limitation — see its own `🧬️mutations/🦀️component.rs` docstring).
2. **`✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs:177`** — `import_media`'s `"params:in"`
   handler builds `Procedural3dMutation::SetWidget { index, widget: flow::Widget::InputSlider{..} }`
   for an **existing** widget found by id (`fixture.widgets.iter().enumerate().find(...)`). Replace
   with `Procedural3dMutation::UpdateWidget(crate::artifacts::procedural3d::op::UpdateWidget{ widget: flow::Widget::InputSlider{..} })`
   once `UpdateWidget` is added to `🧬️mutations/📝️text/🦀️component.rs`'s re-export list (or reach it
   via `crate::artifacts::procedural3d::mutations::set_widget::mutation::UpdateWidget` today) — drop
   the now-unused `index` binding.
3. **`✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🎨️example/🦀️component.rs:52`** —
   `Procedural3dMutation::Generation(GenerationMutation::Remove { id: generation.id.clone() })` →
   `crate::artifacts::procedural3d::op::generation_mutation_to_procedural3d(GenerationMutation::Remove { id: generation.id.clone() })`.
4. **`✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🧬️generation/🦀️component.rs:31`** —
   `operations.into_iter().map(Procedural3dMutation::Generation).collect()` →
   `operations.into_iter().map(crate::artifacts::procedural3d::op::generation_mutation_to_procedural3d).collect()`.
5. Grammar (`🧬️mutations/📖️component.grammar.semio`) and JSON/protobuf/graphql/ANBF sibling files
   under `📝️text/`/`💾️binary/` were left untouched (step f, non-blocking) — the `.grammar.semio`
   file was ALREADY stale before this migration (describes an unrelated `mesh-op`/vertex/face/solid
   vocabulary, not widgets/synapses/camera/schema/generation at all), so honestly updating it is a
   separate, larger task, not a regression introduced here.

## Verify

`cargo check -p semio-s-plugin-procedural`: **zero errors inside this facet's writable boundary**
(procedural3d artifact directory). The only 5 remaining errors are all in `🎛️apps/**`
(`🧊️3d` — 2 call sites listed above; `◻2d` — 2 call sites from the concurrent `procedural2d`
migration, a different artifact, not touched by this session) — exactly the expected/sanctioned
breakage the task's `sharedFileRequests` mechanism exists for. `cargo check --tests` shows the same
5 errors plus one more in `procedural2d`'s own snapshot text test (that artifact's concern, not
this facet's). Could not run `cargo test` for this facet specifically since the crate's default lib
target (which includes `🎛️apps/**`) doesn't build until the sharedFileRequests above land elsewhere
in the plugin; test *code* for this facet type-checks cleanly under `--tests` (no additional errors
appear versus plain `--check`), and every mutation/inverse/diff was manually re-derived from the
pre-migration hand-written `impl Mutation` body it replaces (same underlying `WidgetsDiff`/
`SynapsesDiff`/`LayoutDiff`/`diff_fixture_from_helpers`/`diff_generation_from_ops` helpers, same
`apply_widgets_diff`/`apply_synapses_diff` upsert-by-id semantics), so the logic is a faithful
1:1 refactor of already-tested behavior, not new behavior.

## Files touched

Created: 6 new triad dirs × 3 leaves × 2 files (`.rs` + `.ts` stub) = 36 files under
`create-widget`/`connect-synapse`/`create-generation`/`delete-generation`/`rename-generation`/
`change-generation-value`.

Rewritten (repurposed in place): the 8 pre-migration triad dirs' 3 `.rs` leaves each = 24 files
(`remove-layout`, `remove-synapse`, `remove-widget`, `set-camera`, `set-layout`, `set-schema`,
`set-synapse`, `set-widget` — `mutation`/`diff`/`inverse`).

Rewritten: `🧬️mutations/🦀️component.rs` (dispatch enum + tests), `🧬️mutations/💾️binary/🦀️component.rs`
(wire codec), `🧬️mutations/📝️text/🦀️component.rs` (re-export list, one-line addition).

Fixed (in-boundary collateral): `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` (one test).

No files touched outside `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/`.
