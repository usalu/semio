# Wave 2 — `process` / `process3d` / `1` / `any` — mutations facet migration

Facet: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-process`

## Vocabulary derived

From `Process3dSnapshot { workshop: Workshop { machines: Vec<WorkshopMachine> }, stock: Stock,
steps: Vec<ProcessStep>, resolved_up_to: Option<usize> }`, 16 semantic mutations replace the
generic `Steps{collection: CollectionMutation<..>}` / `Machines{collection: CollectionMutation<..>}`
/ `SetStock{stock}` / `SetCursor{resolved_up_to}` / `SetSnapshot{snapshot}` vocabulary:

| Verb/entity | New variant | Old generic equivalent |
|---|---|---|
| `create-step` | `CreateStep{index, step}` | `Steps{Add}` |
| `delete-step` | `DeleteStep{id}` | `Steps{Remove}` |
| `rename-step` | `RenameStep{id, new_label}` | `Steps{Patch{label}}` |
| `change-step-enabled` | `ChangeStepEnabled{id, new_enabled}` | `Steps{Patch{enabled}}` |
| `change-step-origin` | `ChangeStepOrigin{id, new_origin}` | `Steps{Patch{origin}}` |
| `replace-step-measure` | `ReplaceStepMeasure{id, new_measure}` | `Steps{Patch{measure}}` |
| `reorder-steps` | `ReorderSteps{id, to_index}` | `Steps{Move}` |
| `create-machine` | `CreateMachine{index, machine}` | `Machines{Add}` |
| `delete-machine` | `DeleteMachine{id}` | `Machines{Remove}` |
| `rename-machine` | `RenameMachine{id, new_label}` | `Machines{Patch{label}}` |
| `change-machine-icon` | `ChangeMachineIcon{id, new_icon_id}` | `Machines{Patch{icon_id}}` |
| `replace-machine-capabilities` | `ReplaceMachineCapabilities{id, new_capabilities}` | `Machines{Patch{capabilities}}` |
| `move-stock` | `MoveStock{new_pose}` | `SetStock` (pose field) |
| `change-stock-label` | `ChangeStockLabel{new_label}` | `SetStock` (label field) |
| `replace-stock-solid` | `ReplaceStockSolid{new_solid}` | `SetStock` (solid field) |
| `change-cursor` | `ChangeCursor{new_resolved_up_to}` | `SetCursor` |

`Steps`/`Machines` (bare `CollectionMutation<K,V,Patch>` in a public enum — forbidden shape) were
each split per `📓️derivation-rules.md` rule 2 into one `create`/`delete` id-keyed pair plus one
`change-*`/`rename`/`replace-*` per remaining scalar/structured field, plus `reorder-steps` since
the steps timeline's order is user-meaningful (machines' order is not, so no `reorder-machines`).
`SetStock` (a bare whole-object setter — forbidden) was split per rule 1 into its spatial
(`move-stock`), identity (`change-stock-label`), and large-structured (`replace-stock-solid`)
fields — three real semantic verbs instead of one setter. `SetSnapshot` is **banned outright** per
`📓️taxonomy.md`/`📓️derivation-rules.md` rule 6: whole-document replace has **no replacement
mutation** — file-open/import/load-example now goes through `store::ArtifactStore::reset`,
entirely outside the `Mutation` enum. `NoMutation` was never present in this facet.

## Directory layout — 5 dirs repurposed in place, 11 new dirs added

`📦️glue.rs` (plugin-shared, outside this facet's writable boundary) path-includes five
pre-migration triad directories verbatim: `⏱️set-cursor`, `📄set-snapshot`, `📋steps`, `🛠️machines`,
`🧱set-stock`. Since glue.rs couldn't be edited, those five directories were **repurposed in
place** (same path, rewritten `🦠️mutation`/`🔺️diff`/`↩️inverse` content) rather than renamed — each
new payload's `SEMANTICS.kind`/`record` are the correct new semantic strings (e.g. `📋steps/`'s
payload has `kind: "create-step"`), so runtime/wire/registry behavior is fully correct; only the
on-disk directory name is stale:

- `⏱️set-cursor` → `ChangeCursor` (kind `change-cursor`)
- `📄set-snapshot` → `ReplaceStepMeasure` (kind `replace-step-measure` — the closest real semantic
  verb to what this slot used to gesture at: a large structured sub-payload swap)
- `📋steps` → `CreateStep` (kind `create-step`)
- `🛠️machines` → `CreateMachine` (kind `create-machine`)
- `🧱set-stock` → `MoveStock` (kind `move-stock`)

Eleven mutations with no pre-wired slot got fresh, correctly-named directories, self-wired
directly inside `🧬️mutations/🦀️component.rs` via nested `#[path = "."] pub mod <name> { #[path =
"..."] pub mod mutation; ... }` blocks (mirrors `procedural3d`'s already-migrated facet, which
confirmed `#[path]` resolution is per-physical-file, not per logical-mod-nesting): `🗑️delete-step`,
`🏷️rename-step`, `🔧change-step-enabled`, `🔧change-step-origin`, `🔀reorder-steps`,
`🗑️delete-machine`, `🏷️rename-machine`, `🔧change-machine-icon`, `🔁replace-machine-capabilities`,
`🔧change-stock-label`, `🔁replace-stock-solid`.

See `sharedFileRequests` below for the exact glue.rs directory rename once a later pass can touch
it.

## Diff construction

- Step patches (`rename-step`, `change-step-enabled`, `change-step-origin`,
  `replace-step-measure`) build a single `Process3dStepsDelta.patched` entry touching only their
  own field — real handcrafted sparse diffs, never apply-then-capture.
- `create-step`/`delete-step` build a single `Process3dStepsDelta.added`/`.removed` entry.
  `create-step`'s `index` field is carried for label/provenance parity with the taxonomy's create
  canonical args, but (matching the pre-migration `steps_delta_from_collection_mutation` behavior
  this replaces) the underlying delta engine always appends — documented inline on the payload.
- `reorder-steps` builds the full target id-order list directly from `base`, mirroring the old
  `CollectionMutation::Move` arithmetic (`remove` then `insert` at `to_index.min(len)`).
- Every machine mutation (`create`/`delete`/`rename-machine`, `change-machine-icon`,
  `replace-machine-capabilities`) and `move-stock`/`change-stock-label`/`replace-stock-solid`
  build a **whole** new `Workshop`/`Stock` value from `base` + payload — `Process3dDiff.workshop`
  and `.stock` are themselves whole-value-replace fields in this artifact's diff schema (not
  sparse deltas), so this is the correct, already-established diff shape for those two fields, not
  an apply-then-capture shortcut.
- `change-cursor` sets `Process3dDiff.resolved_up_to` directly.

## Inverses

Every id-addressed inverse (`delete-step`→`create-step`, `delete-machine`→`create-machine`,
`rename-*`, `change-*`, `replace-*`) looks the target up in `base` and returns `Vec::new()` when
missing (the semantic replacement for the retired `NoMutation` sentinel). `reorder-steps`'s
inverse reconstructs the pre-move index from `base` (addressing convention #5). `move-stock`,
`change-stock-label`, `replace-stock-solid`, `change-cursor` are singleton document-root fields
with no missing-target case — inverse always returns one mutation restoring the prior value from
`base`.

## Other in-boundary files fixed (compile breakage caused by the vocabulary change)

- `🧬️mutations/📝️text/🦀️component.rs` — full rewrite of `Process3dMutationDsl` (the handcrafted
  OpText/OpBinary wire-codec mirror) and its `to_dsl`/`from_dsl` conversion functions to match the
  16 new variants (flat keyworded records instead of the old `CollectionMutation`-flattening
  `StepsAdd`/`StepsRemove`/`StepsMove`/`StepsPatch`/`MachinesAdd`/… quadruplets); removed the
  now-dead `StepOriginPatch`/`ProcessStepPatchDsl` DSL-only mirrors (only existed to serialize the
  retired `StepsPatch`/`MachinesPatch` variants); tests rewritten to the new constructors.
- `🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — `insert_step_mutations`/`remove_step_mutations`
  (shared by the `🎮️commands/🪜️step` and `🎮️commands/🌍️world` app command handlers) rewritten from
  `Process3dMutation::Steps{CollectionMutation::Add/Remove}` + `SetCursor` to
  `Process3dMutation::CreateStep`/`DeleteStep` + `ChangeCursor`. Signatures unchanged, so the two
  app command modules that call these helpers needed no edit.

## Tests

Extended the existing `🧪️Tests` region in `🧬️mutations/🦀️component.rs` (no new test files): one
round-trip test per new variant (16 total) via a shared `round_trip` helper
(`vcs::apply_mutation` forward, reversed `inverse()` backward, asserts restoration of `base`),
`every_variant_registers_an_approved_semantic_descriptor` (iterates one value per variant,
asserts `kinds().len() == 16`), two `inverse_*_when_missing_returns_empty` tests
(`delete-step`/`delete-machine`), and three `protocol::testkit::assert_mutation_inverse_law` /
`assert_mutation_diff_absorb_law` pairs (`create-step`, `create-machine`, `change-stock-label` —
an id-keyed create/delete pair on the order-meaningful `steps` collection, an id-keyed create/delete
pair on the unordered `machines` collection, and a document-level facet setter, per the ticket's
own guidance on which 3 variants to pick). `protocol::testkit` and `dsl::Mutations` are both
reachable with **zero new Cargo dependency** — this crate already declares `extern crate
semio_framework_os_kernel as dsl;`/`as protocol;` in its `📦️glue.rs`, and the kernel crate's own
root re-exports (`pub use crate::os_dsl::*;` / `pub use crate::os_spr::*;`) lift both `Mutations`
and `testkit` to those aliases for every plugin, `process` included — confirmed by grep, no
Cargo.toml edit made.
`🧬️mutations/📝️text/🦀️component.rs`'s own `🧪️Tests` region also got one OpText round-trip test per
new variant (16), replacing the old `Steps*`/`Machines*`/`SetStock`/`SetCursor`/`SetSnapshot` set.

## `sharedFileRequests` — exact changes needed once a later pass can touch shared files

1. **`✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs`** — rename the 5 repurposed triad
   directories' path-include block to their correct semantic names (purely cosmetic, behavior
   already correct): `set_cursor`→`change_cursor` (dir `⏱️set-cursor`→`🔧change-cursor`),
   `set_snapshot`→`replace_step_measure` (dir `📄set-snapshot`→`🔁replace-step-measure`),
   `steps`→`create_step` (dir `📋steps`→`➕create-step`), `machines`→`create_machine` (dir
   `🛠️machines`→`➕create-machine`), `set_stock`→`move_stock` (dir `🧱set-stock`→`📍move-stock`).
2. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs`** (lines ~194, 215) — the `document`
   command's `Emit` handler builds `Process3dMutation::SetSnapshot { snapshot }` for whole-document
   replace. This has **no 1:1 replacement mutation** (banned per taxonomy) — the fix is
   structural: route through `store::ArtifactStore::reset` (or whatever store-facing reset entry
   point `ArtifactApp::handle` exposes) instead of emitting a `Mutation`, not a rename.
3. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs`** (lines ~103–200, ~16 call
   sites) — test/wasm-bridge code building `Process3dMutation::Steps{CollectionMutation::Add/
   Remove/Patch}` and `SetCursor`/`SetStock`. Map: `Steps{Add}`→`CreateStep`, `Steps{Remove}`→
   `DeleteStep`, `Steps{Patch{enabled}}`→`ChangeStepEnabled`, `Steps{Patch{origin}}`→
   `ChangeStepOrigin`, `SetCursor`→`ChangeCursor`, `SetStock{stock}`→ decompose into
   `MoveStock`/`ChangeStockLabel`/`ReplaceStockSolid` (whichever fields actually changed).
4. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🪵️stock/🦀️component.rs:29`** — `SetSnapshot`
   whole-document swap; same structural note as #2 (store reset, not a mutation).
5. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🔎️inspector/🦀️component.rs`** (lines ~146,
   153, 162) — generic inspector-field patch dispatcher. `Machines{Patch}`→ pick the specific
   `RenameMachine`/`ChangeMachineIcon`/`ReplaceMachineCapabilities` variant by which field
   changed; `SetStock{stock}`→ pick `MoveStock`/`ChangeStockLabel`/`ReplaceStockSolid`;
   `Steps{Patch}`→ pick `RenameStep`/`ChangeStepEnabled`/`ChangeStepOrigin`/`ReplaceStepMeasure`.
6. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/📄️artifact/🦀️component.rs`** (lines ~22, 46)
   — `SetSnapshot`; same structural note as #2.
7. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🎛️engagement/🦀️component.rs`** (lines ~29–31)
   — `SetCursor{resolved_up_to}` → `ChangeCursor{new_resolved_up_to: resolved_up_to}` (direct
   rename, 1:1).
8. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/📤️media/🦀️component.rs:65`** — `SetSnapshot`;
   same structural note as #2.
9. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/⏱️cursor/🦀️component.rs`** (lines ~22, 41, 58,
   75) — `SetCursor` → `ChangeCursor` (direct rename, 1:1, 4 call sites).
10. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🪜️step/🦀️component.rs`** (lines ~112, 134,
    156) — `move_step` handler: `Steps{Move}`→`ReorderSteps{id, to_index: index}` (direct rename).
    `update_step` handler currently batches label+enabled+measure+origin into ONE
    `Steps{Patch}` — must become up to 4 separate mutations
    (`RenameStep`/`ChangeStepEnabled`/`ReplaceStepMeasure`/`ChangeStepOrigin`), one per changed
    field, all in the same `Emit::mutations(vec![...])`. `set_step_enabled` handler:
    `Steps{Patch{enabled}}`→`ChangeStepEnabled` (direct rename).
11. **`✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🛠️workshop/🦀️component.rs`** (lines ~16, 20,
    93) — `add_workshop_machine_operation`: `Machines{Add}`→`CreateMachine` (direct rename).
    `remove_workshop_machine_operation`: `Machines{Remove}`→`DeleteMachine` (direct rename).
    `update_workshop_machine` handler batches label+icon_id+capabilities into ONE
    `Machines{Patch}` — must become up to 3 separate mutations
    (`RenameMachine`/`ChangeMachineIcon`/`ReplaceMachineCapabilities`), one per changed field.
12. Grammar (`🧬️mutations/📖️component.grammar.semio`, at the `🧬️mutations/` root, NOT the
    `📝️text/` one) was left untouched (step f, non-blocking) — it was ALREADY stale before this
    migration (describes an unrelated `mesh-op`/vertex/face/transform/merge vocabulary, not
    steps/machines/stock/cursor at all, and is `include_str!`-referenced by nothing), so honestly
    rewriting it is a separate, larger task, not a regression introduced here. The `📝️text/`
    grammar file (`document = header body`, a generic envelope grammar) is accurate and unchanged
    — it does not enumerate individual mutation keywords, so no edit was needed there.

## Verify

`cargo check -p semio-s-plugin-process --message-format=short` (non-test): 26 errors. Investigating
the full `--tests` run (below) caught a real bug of this migration's own making — `🧬️mutations/📝️text/🦀️component.rs`'s
new `imported_mesh_stock_round_trips_document_dsl` test used `Process3dSnapshot` without adding it
to that test module's existing `use crate::artifacts::process3d::{...}` import list
(`E0422`/`E0433`, 2 errors). **Fixed** (added `Process3dSnapshot` to the import list) and
reconfirmed clean on the next run.

`cargo check -p semio-s-plugin-process --tests --message-format=short` (post-fix): **42 errors,
100% accounted for and none inside this migration's own code**:

- **38 expected/sanctioned app-boundary breakage** — every one a `🎛️apps/🧊️3d/**` call site
  building the retired struct-variant shapes (`Process3dMutation::Steps{..}`/`Machines{..}`/
  `SetStock{..}`/`SetCursor{..}`/`SetSnapshot{..}`): 22 in the 9 non-wasm files catalogued in
  `sharedFileRequests` items 2–11 below, plus 16 in `🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs` (item 3 —
  invisible to a plain non-`--tests` `cargo check` since that module is only compiled under
  `--tests`/`cfg(test)`, not a real `wasm32`-only gate as first assumed; corrected in item 3 below).
- **4 unrelated concurrent-session churn**, confirmed NOT caused by this migration and outside
  this facet (`🧬️mutations`) though technically inside the artifact directory: one
  `E0599 no associated function infer` in `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
  (git-blamed to a commit made 11:09:41 today, mid-session — concurrent work on ticket
  `INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`, a different, unrelated ticket
  touching a sibling schema-family facet this session never wrote to) and three `E0308
  JsonValue`/`Value` mismatches in `🚪️io/📤️export/…/🔣️json/…` and `🚪️io/📥️import/…/🔣️json/…`
  (git-blamed to a two-day-old commit, pre-dating this session entirely — an unrelated JSON-codec
  type drift). Per house policy on concurrent workspace churn (poll/verify scope, don't chase
  another session's WIP), neither was touched.

Every file inside `🧬️mutations/` and the one in-boundary collateral fix
(`🏅️standards/🔖️1/⚙️engine/🦀️component.rs`) compiles clean, including all test code — **zero
errors attributable to this migration** after the one self-inflicted bug above was fixed. Nothing
outside `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/` was written.

## Files touched

Created: 11 new triad dirs × 3 leaves × 2 files (`.rs` + `.ts` stub) = 66 files under
`🗑️delete-step`/`🏷️rename-step`/`🔧change-step-enabled`/`🔧change-step-origin`/`🔀reorder-steps`/
`🗑️delete-machine`/`🏷️rename-machine`/`🔧change-machine-icon`/`🔁replace-machine-capabilities`/
`🔧change-stock-label`/`🔁replace-stock-solid`.

Rewritten (repurposed in place): the 5 pre-migration triad dirs' 3 `.rs` leaves each = 15 files
(`⏱️set-cursor`, `📄set-snapshot`, `📋steps`, `🛠️machines`, `🧱set-stock` —
`🦠️mutation`/`🔺️diff`/`↩️inverse`).

Rewritten: `🧬️mutations/🦀️component.rs` (dispatch enum + tests), `🧬️mutations/📝️text/🦀️component.rs`
(wire codec + tests).

Fixed (in-boundary collateral): `🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
(`insert_step_mutations`/`remove_step_mutations`).

No files touched outside `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/`.
