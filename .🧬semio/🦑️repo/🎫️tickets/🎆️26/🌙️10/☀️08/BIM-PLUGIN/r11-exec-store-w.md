# r11-store-w: flow wasm extension evaluation path on grants

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs` (crate `semio-framework-os-flow`), tests in `🧪️tests/🔬️unit/🦀️.rs` of the same directory.

## What changed

- Currency: every close/retire path is now `RetainedCloneGrant` -> `Result<RetainedCloneStep, ValueError>` with an honest `RetirementDemand` quote, mirroring the finished siblings (`ContributedExtensionStub::retire_step`, `retire_flow_extension_registries_step`). `ValueRetirementStep`, `(maximum_items, maximum_bytes)` and `PluginCloseStep` are gone from this file.
- Shared helpers (private, top of `EvaluationJobs`):
  - quotes: `value_quote`, `binding_quote`, `binding_close_quote`, `job_quote`, `handoff_demand` (+ `domain_demand`, `owned_demand<T>`);
  - `funded(demand, body)` builds the one-item grant that pays exactly a quote (copy axis = `body.max(demand.copy)`, same rule as `registry_grant`), `admitted(grant, demand)` applies the law (no item or an under-funded copy/capacity/release axis yields, depth below the quote errors), `settled` reports `Complete` only with the terminal witness;
  - `hand_off` moves one still-owned allocation into a neural `ValueRetirement` under the caller grant: the slot keeps the allocation until a frame is admitted, `OwnershipLimit` (missing backing slot) turns into a `reserve_step` turn; `hand_off_cold` / `retire_text` / `retire_dictionary` / `retire_owned` are the self-funded form for the evaluation machinery's own transitions (pay the exact quote, reserve first); `retire_turn` / `drain_cold` are the self-funded drivers; `close_slot` closes an optional custody slot and drops it on the terminal witness.
  - Queue depth: the neural queue length is private, so hand-overs are admitted against `EVALUATION_QUEUE_DEPTH = 256` instead of a caller depth; draining uses the owner's own `next_depth_demand`.
- `EvaluationInputPreparation`: `step(maximum_units, maximum_bytes)` keeps its signature (units = transitions, bytes = body page, other axes are the owner's quote); the typed binding steps under a quote-funded grant (`RetainedDictionaryInput::step(grant)`). New public `close_demands(body)` and `close_step(grant)` (target ladder retirement -> binding -> parser -> projection -> writer -> typed dictionary -> text, one item per turn). `impl RetireOwned` / `EvaluationInputRetirement` removed: a displaced preparation now sits in `RetainedEvaluation.displaced` (a stack closed by its own quote) instead of being pushed as an erased retirement frame.
- `EvaluationOutput`, `RetainedEvaluation`: same target ladder (`close_target` -> `close_demands` + `close_step(grant)` share one selection so quote and turn cannot drift). The candidate's dictionary and identity are two items now (identity moves into `dependency_identity`, then is handed off next turn). A job closes through `OperatorJob::close_step(grant)` with `next_close_*_demand` quotes.
- Registry level (public API changed, greenfield, no alias): `retire_cancelled_evaluations_step(owner, grant) -> Result<RetainedCloneStep, ValueError>` (replaces `retire_cancelled_evaluations_close_step(owner, units, bytes)` and the old unit-only `retire_cancelled_evaluations_step`), `evaluation_retirement_demands(owner, body) -> Result<RetirementDemand, ValueError>` (replaces `evaluation_retirement_next_close_byte_demand`), `retire_cancelled_evaluations_funded(owner, maximum_units)` (self-funded drive for cancel_* and tests/peers). `evaluation_retirement_pending`, `cancel_*`, `evaluation_progress` unchanged in signature.
- `ExtensionEvaluationResources` implements the new `ExtensionResourceOwner`: `retirement_demands(body)`, `close_step(grant) -> PluginLifecycleStep` (below-quote yields `Progress(default)`, registry hand-off is `copied_items: 1`, readers still alive on the original `RegistryRetirement` report `Blocked`, `Complete` only via `PluginLifecycleStep::retained(step, terminal)`), `next_close_byte_demand` removed.
- `evaluation_dependency_identity` drains its cursors through `retire_owned` + `drain_cold`.
- Tests migrated (intent kept): fixture registry drop, preparation cancel/close cutoffs (zero-item and one-axis-starved grants yield without mutation, every turn `fits(grant)`), retained evaluation retirement loops (`retire_cancelled_evaluations_funded`), output close (zero-item grant yields, quote-funded drain), extension close receipt (zero-item / starved-axis grants yield, per-turn `fits`, summed `released_bytes >= source bytes`). New test helper `starved`.

## Gate

- `cargo check -p semio-framework-os-flow --lib` (final run): 0 errors, none in this file (only the pre-existing dead-code warning for `parsed_operator_input`). `--lib --tests` check: 0 errors.
- `cargo test -p semio-framework-os-flow --lib -- extensions::wasm`: 8 passed, 0 failed (12 s). A first run livelocked (a reserve turn leaves the queue non-terminal with an empty page, and the retirement-first ladder then released that page again, forever); fixed by making every ladder hand off all custody first and drain the retirement last.

## Remaining / for other owners

- Callers of the removed API outside this file (not touched): `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs` (`impl ExtensionResourceOwner for BrepExtensionResources` still has the old `close_step(items, bytes)` / `next_close_byte_demand`), brep tests `🥽️mesh/🧪️tests/🔬️unit`, `🧪️tests/🔬️extension-guest-standalone`, `🧪️tests/🔬️evaluate-budget` (`retire_cancelled_evaluations_close_step`, `PluginCloseStep`), and the `brep-extension` test under `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d`. Replacements: `retire_cancelled_evaluations_funded(owner, 1)` for a self-funded drain turn, `retire_cancelled_evaluations_step(owner, grant)` + `evaluation_retirement_demands(owner, body)` for a granted turn.
- `parsed_operator_input` is dead code from before this change (left alone).

## Notes / risks

- Peer `ed.ts` in the shared scratchpad was overwritten by me by mistake (I wrote a JSON-pairs tool under that name); it was recreated with the documented interface `bun ed.ts file old1 new1 [old2 new2 ...]` (exact match, errors on 0 matches, replaces all matches when more than one). My own tool is `mine-ed-json.ts`. Peers that relied on other behaviour of the original should re-check.
- Turn counts changed (a reserve turn plus a hand-over turn per stacked owner); `maximumTurns` fixture laws may need a look when the tests run.
