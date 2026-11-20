# Boot failure: "source owner capture lacks original Source"

## Cause
`OriginalSourceOwner::capture_into` (`🌱️value/🧬️retained-clone/⚙️engine/🔗️source-owner/🦀️.rs`) refuses an effect whose Work carries no Source turn. `StepContext::with_original_effect` always opens its Work with `source = None`; only `with_original_source_effect` binds the turn's own Source. Source-requiring producers (the resident `SourceHold` behind `FlowOperatorCatalogueWork`, original reads, post custody) therefore fail when they are advanced under the Source-less variant.

The live cleanup pump (`🔁️cleanup-turns`) drives the mounted maintenance ladder (`mounted_owner_maintenance_step`); the flow operator catalogue owner (`🌊️flow/📔️registry/📇️catalogue-owner/🦀️.rs`) runs there and called `cx.with_original_effect(...)` in its derivation (`phase 0` registry capture, `phase 2` `build.advance` -> `SourceHold::step_into` -> `capture_into`), its close helper and `retain_infos`. The capture refused, the job body ended as a Continuation carrying the `InvariantViolated` value, and the pump reported `checked-out-job` / `Framework OS boot failed`. Every pane whose app owns the flow catalogue (generator and the 6 others) fails at boot.

This is a symptom introduced by the Source-bound Work migration (commit 8fdb3e10b97 added `with_original_source_effect` and the Source check in capture but only migrated `native-turn` birth). Other Source-less `with_original_effect` callers that advance (not retire) a Source owner remain candidates: `tool-run/🎞️tick/🎮️operations` `built_effect` (OriginalToolLedgerRead/Walk, read birth), not touched here.

## Fix
`catalogue-owner/🦀️.rs`: all four `cx.with_original_effect` -> `cx.with_original_source_effect`.

## Test
`🌊️flow/📔️registry/📇️catalogue-owner/🧪️laws/🦀️.rs`, included from `🖥️host/⚖️laws/🦀️.rs` as `operator_catalogue_owner_tests`:
`catalogue_derivation_captures_the_registry_under_the_turns_own_source` drives the owner on an `OriginalJobFixtureCaller` until ready, then closes it.
Run: `repo tools exec cargo -- test -j 2 -p semio-os-flow-host-laws catalogue_derivation_captures`.

## Rebuild (not done)
`procedural` hub (`🌀️procedural`) and every guest embedding semio-os-flow-operators (flow catalogue owner): `playbook-module-procedural`, `puzzle`, `flow` and the other pane hubs that carry the flow extension registry. No browser bundle change needed.

## Follow-up: "source owner changed original Source or Work" on later turns
After the Source-bound capture the next failure is `OriginalSourceOwner::authenticate` (`🔗️source-owner/🦀️.rs`). It required the exact Work binding (`matches_effect`: Work scope, Work issuer and Source turn) of the capturing effect. Production hosts open a fresh context per turn (`borrow_context` issues its own Source turn; `with_original_source_effect` opens its own Work scope and issuer), so a resident owner that captured on turn N and continues its admission/projection on turn N+1 (the flow catalogue `SourceHold`, tool-run ledger reads) can never match. The `wadmit` debug lines are the worker admission of the cleanup job stepping normally, not the failure.

Fix (source-owner protocol): between actions (`route == None`, no recorded reply awaiting recovery) and when the effect carries a Source, `authenticate` re-binds the owner's Work binding to the effect that now drives it; while an action is pending (`route` set) the exact binding is still required. `authenticate` now takes `&mut self` (all callers already had `&mut`).

Not changed: other holders of an own `work` binding compared with `matches_effect` across turns (`RetainedOriginalRead::authenticate` in `📦️original-slot`, read leases/cursors in `🔗️read`, projection frames) may need the same rule if the boot probe shows the same message from them.

Test: the existing law `catalogue_derivation_captures_the_registry_under_the_turns_own_source` already crosses turns (the fixture caller opens a fresh Source per `turn`), so it covers this; no separate test added. Run: `repo cargo -- test -j 2 -p semio-os-flow-host-laws --lib catalogue_derivation_captures` (non-isolated store).

Rebuild: `procedural` hub and every guest embedding semio-framework-value (source-owner is in the value crate): all pane hubs/components that embed it, not only the flow ones.

## Systematic pass: one shared follow rule for every resident holder
Next failure was "admission birth changed its original Work or Source" (`RetainedCloneSourceAdmissionBirth::authenticate`). Same family, so the rule now lives in one place: `OriginalWorkBindingIdentity::follow_between_actions(&mut self, effect, pending)` (`🎟️control/🧬️contract/🎟️work-action/🫴️borrow/🦀️.rs`): when no recorded reply awaits recovery (`pending == false`) and the effect carries a Source, the holder re-binds to that effect; then it answers `matches_effect`. Recovery paths pass `pending = true` and still need the exact binding.

Holders switched (all under `🧰️framework/🔨️modules/🌱️value/`):
- `🧬️retained-clone/⚙️engine/🔗️source-owner`: owner `authenticate` (pending = route set) and admission birth `authenticate`
- `🧬️retained-clone/📦️original-slot` (`RetainedOriginalRead::authenticate`) and `🌱️birth`
- `🧬️retained-clone/🗺️ordered-map/✏️edit/🎟️lease`
- `🔗️read`: lease (`🦀️.rs`), `🪪️issue`, `🪪️authority`, `🪪️lease`, `🪪️maintenance`, `🏗️birth` (two registries), `🧬️clone` (+ `♻️cursor`); the wrapper checks in `🧬️clone/🦀️.rs` that compared a copied binding now only require a Source, the nested holder owns the binding
- `📽️projection`: `🧮️array` (+ `📍️placement`, `🪢️bounded`), `📋️field`, `🪡️captured`
- `🌳️tree/🟰️equal/🎚️record` field validation
Left as is: stateless loan-vs-effect comparisons in close paths (`close_original_into`, `loan_key`, `original_key`) and the `🛬️decode` family (binds its own construction proofs; not on this boot path).

Implementation note: holders reached through `&self` use `OriginalWorkBindingIdentity::admits_between_actions(&self, effect, pending)` (same rule, no store); only the source owner and its admission birth (`&mut self`) use `follow_between_actions`. The record-equality field check was left on `matches_effect`. `repo cargo --store=shared --configuration=prototype -- check -j 2 -p semio-framework-value` passes.
