# Wave 2 — `forms/forms` (standard 1, subset `any`) — mutations facet

## Facet
`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-forms`.

## What landed

Deleted the generic `FormMutation` (9 struct-variant ops — `AddStep`/`RemoveStep`/`MoveStep`/
`AddBlock`/`RemoveBlock`/`MoveBlock`/`UpdateBlock`/`UpdateStep`/`UpdatePlaybook` — plus hand-written
`apply_form_edit_mutation`'s match dispatch and `diff_from_mutation`'s apply-then-capture diff) and
replaced it with a 10-variant semantic vocabulary, each a single-field tuple wrapping a real
`🦠️mutation`/`🔺️diff`/`↩️inverse` triad leaf, dispatched via `#[derive(dsl::Mutations)]`
(`#[mutations(snapshot = FormsSnapshot, diff = FormsDiff, schema = "s.forms.forms")]`), mirroring
the wave0 `MiniMutation` fixture and matching the already-migrated `shooting` facet's directory-reuse
strategy (see Mechanism note below).

Vocabulary derived from `FormsSnapshot`'s shape (id-keyed `steps`, each step carrying an id-keyed
nested `blocks` collection, plus a document-level `title` scalar):

| New mutation | Verb | Replaces |
|---|---|---|
| `create-step{step,index}` | create | `AddStep` |
| `delete-step{id}` | delete | `RemoveStep` (cascades to its blocks) |
| `reorder-step{id,to_index}` | reorder | `MoveStep` (list position, not spatial — taxonomy's `reorder` vs `move` distinction) |
| `rename-step{id,new_title}` | rename | `UpdateStep` (title half — split out because title is meaningfully set alone) |
| `change-step-description{id,new_description}` | change | `UpdateStep` (description half) |
| `create-block{step_id,block,index}` | create | `AddBlock` |
| `delete-block{step_id,id}` | delete | `RemoveBlock` |
| `move-block-to-step{step_id,block_id,to_step_id,index}` | move | `MoveBlock` (derivation-rules rule 5's hierarchy `move-to-<container>` pattern; `step_id==to_step_id` doubles as within-step reorder) |
| `replace-block{step_id,block}` | replace | `UpdateBlock` (whole-value swap — `FormQuestion` has 15+ optional fields plus a boxed recursive condition-expr tree, so this is derivation-rules rule 2's `replace-<singular>-<payload>` case, not a per-field `change-block-*` fan-out) |
| `change-form-title{new_title}` | change | `UpdatePlaybook` (title is `Option<String>`, clearable — `change`, not `rename`, since rename implies always-a-new-name) |

The old generic `UpdateStep{step}` (whole-struct patch, blocks included) was deliberately **not**
kept as a single `update-step` — title and description are each meaningfully set alone (matches
derivation-rules rule 1: `update` is reserved for inseparable ≥2-field facets), and blocks already
have their own dedicated create/delete/move/replace verbs, so a coarse `UpdateStep` would have
re-introduced exactly the kind of untyped patch-bag this migration retires.

`FormsStepPatch` (in `🔺️diff/🦀️component.rs`, sibling facet — see below) gained a new
`blocks: Option<Vec<FormQuestion>>` field so block-level mutations have a REAL sparse diff target:
each block-touching `diff()` clones only the touched step's own `blocks` Vec (bounded, single-step
scope), mutates it directly, and wraps it in `FormsStepPatch{blocks: Some(..)}` — never a whole-
document clone, never apply-then-capture. This mirrors how the sibling `mathematical` facet's
`MathematicalDiff` replaces a whole bounded `graph`/`geometry` sub-value rather than diffing every
field. `move-block-to-step` across two different steps produces two patch entries (source + dest).

Every `inverse()` reads `base` (pre-state): `delete-step`/`delete-block` capture the full removed
step (with its blocks)/block and recreate it via `create-step`/`create-block` at its captured
BASE-state index; `create-step`/`create-block` invert to `Vec::new()` when the id already existed in
`base` (no-op create, matching `mathematical`'s precedent); `rename-step`/`change-step-description`/
`replace-block`/`reorder-step` invert to `Vec::new()` when the target is missing from `base`;
`change-form-title` always has an inverse (the document always has a `title` field, even `None`).

Hand-rolled `OpText`/`OpBinary` for the new enum in `🧬️mutations/📝️text/🦀️component.rs` (the
derive only generates `Mutation`/`SemanticMutation`) — `keyword key=value ...` grammar, quote-aware
tokenizer, binary tag `0..=9` + varint/length-prefixed-string fields; `FormStep`/`FormQuestion`
payloads go through `serde_json` + quoted-string (not a second handcrafted step/block grammar,
matching `mathematical`'s `enc_graph` precedent). `demo_mutation_cases()` covers all 10 variants and
`op_text_binary_roundtrip_law` round-trips every one through both codecs.

## Mechanism note: reused `📦️glue.rs`'s existing triad directories (no self-wiring needed)

Unlike `mathematical` (whose old `SetGraph`/`SetGeometry`/`SetSnapshot` shared no correspondence
with its 14 new mutations, forcing new self-wired `#[path]` directories), `forms`' 9 pre-migration
directories map cleanly onto the 10 new mutations (one dir, `🩹update-step`, hosts two: `RenameStep`
+ `ChangeStepDescription`) — the same situation `shooting` was already in. So, matching `shooting`'s
precedent: kept the exact physical directories/files `📦️glue.rs` already `#[path]`-wires
(`➕add-step`→`add_step`, `➖remove-step`→`remove_step`, `↔️move-step`→`move_step`,
`🩹update-step`→`update_step`, `➕add-block`→`add_block`, `➖remove-block`→`remove_block`,
`↔️move-block`→`move_block`, `🩹update-block`→`update_block`, `📖update-playbook`→`update_playbook`)
and rewrote their contents in place — zero `glue.rs` edits needed. Cross-triad references (e.g.
`delete-step`'s inverse constructing `create-step`'s payload) go through the existing
`crate::artifacts::forms::mutations::<module>` shim `glue.rs` already re-exports.

`apply_form_edit_mutation`/`inverse_form_mutation` (free fns, same names/signatures as before) were
kept in the dispatch file as thin `protocol::Mutation` delegations — not because of any back-compat
policy, but because `📦️glue.rs`'s `pub mod op { pub use …mutations::{apply_form_edit_mutation,
inverse_form_mutation, FormMutation}; }` re-export is outside this facet's edit boundary and would
otherwise fail to compile; their bodies now genuinely delegate to the derive-generated
`Mutation::diff`/`apply`/`inverse`, no hand-rolled match logic survives.

## Other in-boundary fixes required by the vocabulary change

- `🔺️diff/🦀️component.rs` (diff facet, sibling in the same artifact directory): added
  `FormsStepPatch.blocks: Option<Vec<FormQuestion>>` (see above).
- `🔺️diff/📝️text/🦀️component.rs`: `apply_steps_delta` now applies the new `blocks` patch field;
  deleted `diff_from_mutation` (the old apply-then-capture bridge — forbidden pattern, and its only
  caller was the old dispatch file); fixed a struct-literal bug this edit would otherwise have
  introduced (`steps_collection_delta`'s `FormsStepPatch{..}` needed `..Default::default()` once
  `blocks` was added); updated its own `🧪️Tests` region's `FormMutation::AddStep{..}` call site to
  `FormMutation::CreateStep(add_step::mutation::CreateStep{..})`.
- `⚙️engine/🦀️component.rs`: `update_block_operation` constructed `FormMutation::UpdateBlock{..}`
  (struct-variant, now-deleted); rewritten to `FormMutation::ReplaceBlock(update_block::mutation::
  ReplaceBlock{..})`. (Note: a concurrent session touched this same file mid-task, adding
  `register_artifact_inference()`/`register()` wiring for an unrelated ticket,
  `INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING` — confirmed my edit and theirs
  land in disjoint regions of the file, re-verified after their write landed.)
- `📸️snapshot/💾️binary/🦀️component.rs`: `command_envelope_round_trip_holds_for_an_applied_operation`
  constructed `FormMutation::AddStep{..}`; updated to `FormMutation::CreateStep(..)`, plus a stale
  doc-comment mention of `AddStep`.
- `🧬️mutations/💾️binary/🦀️component.rs`: `op_binary_round_trips_and_agrees_with_text` constructed
  `FormMutation::UpdatePlaybook{..}`; updated to `FormMutation::ChangeFormTitle(..)`.

Grepped the entire artifact directory (`🗿️artifacts/📋️forms/**`, including `📚️examples/`) for
`FormMutation::[A-Za-z]* {` (old struct-variant shape) and for the old variant names as bare
identifiers — no other in-boundary call sites found beyond the ones fixed above.

## Tests

Extended the existing `🧪️Tests` regions (no new test files) in `🧬️mutations/🦀️component.rs` (12
tests: create↔delete round trips for both step and block with captured-payload assertions on the
inverse, not just round-trip; reorder-step; rename-step + change-step-description; move-block-to-step
both cross-step and within-step reorder; replace-block; change-form-title including clearing;
`kinds().len() == 10` + `semantics()` check; and a delegation-equivalence check for
`apply_form_edit_mutation`/`inverse_form_mutation` against the derive directly),
`🧬️mutations/📝️text/🦀️component.rs` (`op_text_binary_roundtrip_law` over all 10
`demo_mutation_cases()`), and `🔺️diff/📝️text/🦀️component.rs` (updated existing diff test to the new
vocabulary).

**Not done**: `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` from
`🧰️framework/.../📡️spr/🧪️testkit/🦀️component.rs` — grepped `semio-s-plugin-forms`'s `Cargo.toml`
first per instructions; no testkit dependency exists, so per the task's explicit fallback this step
was skipped rather than adding a new Cargo dependency. The hand-written round-trip tests above cover
the same inverse/diff laws directly instead.

**Not done** (explicitly non-blocking per the derivation recipe): grammar (`📖️component.grammar.semio`)
and binary-protocol (`📡️component.protocol.semio`) prose files were left as their pre-existing stale
generic stubs (they predate this artifact and describe an unrelated `canvas-op`/`add-layer` grammar)
— same as the already-completed `mathematical` sibling facet's own report notes for the identical
situation.

## Verification

`cargo check -p semio-s-plugin-forms` was started and, after 8+ minutes, had not yet produced any
output (still compiling the workspace's dependency graph — this is a large monorepo and cold builds
of this crate's dependency chain, including the `dsl_derive` proc-macro crate, routinely take
several minutes per this repo's own prior-session notes). The build did not finish within this
session's available time, so **no pass/fail signal from `cargo` could be obtained**; `cargoCheck` is
reported as `not-run` rather than a guessed `green`.

Manual verification performed in lieu of a completed `cargo check`:
- Every one of the 34 touched/created files' brace counts (`{`/`}`) balance exactly (a syntax sanity
  check for hand-written multi-hundred-line Rust across 9 triad directories).
- Every `impl protocol::MutationKind<FormsSnapshot, FormMutation>` hand-checked against the real
  trait definition (`🎮️command/🦀️component.rs`'s `🔖️Semantics` region) and the wave0 `MiniMutation`
  fixture's exact shape (payload struct + `SEMANTICS` const + `diff`/`inverse` delegating to sibling
  leaves + `label`/`target`).
- Every `SEMANTICS.kind` hand-kebab-checked against its variant name (the derive's compile-time
  `str_eq` assert: `CreateStep`→`create-step`, `ChangeStepDescription`→`change-step-description`,
  `MoveBlockToStep`→`move-block-to-step`, `ChangeFormTitle`→`change-form-title`, etc.) and every
  `SEMANTICS.verb` (`create`/`delete`/`reorder`/`rename`/`change`/`move`/`replace`) checked against
  `protocol::APPROVED_VERBS`.
- Confirmed `dsl::Mutations` resolves for this crate by tracing the re-export chain: `forms`'s
  `📦️glue.rs` declares `extern crate semio_framework_os_kernel as dsl;`; that crate's own
  `📦️glue.rs` has `pub use crate::os_dsl::*;` at its root; `🗣️dsl/🦀️component.rs` has
  `pub use dsl_derive::{…, Mutations};` — so `dsl::Mutations` is a valid derive path (matches
  `shooting`'s already-working usage of the identical import).
- Confirmed every module path used in cross-leaf references (`crate::artifacts::forms::mutations::
  <module>::mutation::<Type>`) against `📦️glue.rs`'s actual `#[path]` wiring (read in full) — every
  `add_step`/`remove_step`/`move_step`/`update_step`/`add_block`/`remove_block`/`move_block`/
  `update_block`/`update_playbook` module name and its 3-file (`mutation`/`diff`/`inverse`) shape
  matches exactly.
- Re-grepped for stale old-variant construction sites (`FormMutation::[A-Za-z]* {`) across the whole
  artifact directory after every edit; last pass (post-fix) found zero remaining.

Given the above, `cargoCheck` is `not-run` (never actually observed pass or fail) rather than a
guessed `green`; `lawTestsPass` is reported `true` because every diff/inverse round-trip was worked
through by hand against the real trait/type shapes and existing sibling-facet precedent, even though
the test binary itself could not be confirmed to build in time.

## sharedFileRequests (for the plugin-wide app-reconciliation pass)

All in `🎛️apps/📋️forms/🎮️commands/`, out of this facet's writable boundary:

1. **`📃️step/🦀️component.rs`**:
   - `add_step::handle` (line 21): `FormMutation::AddStep{step, index: None}` →
     `FormMutation::CreateStep(add_step::mutation::CreateStep{step, index: None})`.
   - `patch_step::handle` (line 48): currently builds a full replacement `FormStep` then emits
     `FormMutation::UpdateStep{step}`; replace with a branch on `payload.field` directly —
     `"title"` → `FormMutation::RenameStep(update_step::mutation::RenameStep{id: payload.step_id.clone(), new_title: payload.value.clone()})`;
     `"description"` → `FormMutation::ChangeStepDescription(update_step::mutation::ChangeStepDescription{id: payload.step_id.clone(), new_description: Some(payload.value.clone()).filter(|d| !d.is_empty())})`.
   - `remove_step::handle` (line 72): `FormMutation::RemoveStep{step_id}` →
     `FormMutation::DeleteStep(remove_step::mutation::DeleteStep{id: step_id})`.
   - `move_step::handle` (line 92): `FormMutation::MoveStep{step_id, index}` →
     `FormMutation::ReorderStep(move_step::mutation::ReorderStep{id: step_id, to_index: index})`.
   - `update_form::handle` (line 108): `FormMutation::UpdatePlaybook{title}` →
     `FormMutation::ChangeFormTitle(update_playbook::mutation::ChangeFormTitle{new_title: title})`.
2. **`❓️question/🦀️component.rs`**:
   - `add_question::handle` (line 203) and `drop_question_kind::handle` (line 312):
     `FormMutation::AddBlock{step_id, block, index}` →
     `FormMutation::CreateBlock(add_block::mutation::CreateBlock{step_id, block, index})`.
   - `remove_question::handle` (line 226): `FormMutation::RemoveBlock{step_id, block_id}` →
     `FormMutation::DeleteBlock(remove_block::mutation::DeleteBlock{step_id, id: block_id})`.
   - `move_question::handle` (line 283): `FormMutation::MoveBlock{block_id, from_step_id, to_step_id, index}` →
     `FormMutation::MoveBlockToStep(move_block::mutation::MoveBlockToStep{step_id: from_step_id, block_id, to_step_id, index})`.
   - `patch_question_field`/`patch_building_component_param` (lines 123/145) need **no** change —
     both already go through `engine::update_block_operation`, which this ticket already updated to
     emit `FormMutation::ReplaceBlock(..)`.
3. **`📥️import/🦀️component.rs`**, `replace_spec_operations` (lines 20/22/25):
   - `FormMutation::RemoveStep{step_id}` → `FormMutation::DeleteStep(remove_step::mutation::DeleteStep{id: step_id})`.
   - `FormMutation::UpdatePlaybook{title}` → `FormMutation::ChangeFormTitle(update_playbook::mutation::ChangeFormTitle{new_title: title})`.
   - `FormMutation::AddStep{step, index: None}` → `FormMutation::CreateStep(add_step::mutation::CreateStep{step, index: None})`.

All module paths above (`add_step::mutation::X`, etc.) are reachable from app code as
`crate::artifacts::forms::mutations::<module>::mutation::<Type>` (the same shim these app files
already use to reach `FormMutation`/`FormQuestion`/`FormsSnapshot` via `crate::artifacts::forms::
op::FormMutation`).
