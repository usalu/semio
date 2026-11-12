# 1112 reactor
- W-reactor-1: surfaces 5x E0433 stale ident `semio_framework_plugin_runtime_ops` -> `semio_plugin_runtime_ops` (reactor/📸️checkpoint, reactor/📥️cold-pair 🦀️.rs). Fixed.
- runtime-exchange stale ident fixed by `commands` (not touched by me).
- W-reactor-4: upstream `semio-plugin-reactor-turn` 10 errs in turn/📥️receiving: OriginalResponseActionSource::admit (4 args) and OriginalIntentActionSource::admit (5 args) lost the `failure` arg; close steps (`step_granted`, `begin`, `ControlledRetirement::step`) now return OriginalRetirementPoll.
  Fix: removed `&mut self.birth.failure` at both admit calls; added `retired_poll` (close_original_effect_step) in receiving/🎟️source and re-exported it; the 8 poll-shaped close sites in receiving/🦀️.rs, 📦️completed/🦀️.rs, 📦️completed/⚠️fault/🦀️.rs use `retired_poll`; Result-shaped sites keep `retired`.
- Verified: W-reactor-5 native rc 0 (surfaces+context); W-reactor-6 wasm (wasm32-wasip2) rc 0 (surfaces+context).
- Open: BirthFrame.failure field now unwritten by admit (no error/warning checked); not tested (check only).
- Rerun: chk.sh W-reactor-7 Cargo.toml <native|wasm> -p semio-plugin-reactor-surfaces -p semio-plugin-reactor-context

## Cleanup (coordinator ruling: one protocol per type)
- `retired` deleted from receiving/source; only `retired_poll` (close_original_effect_step) and re-exported `begun` (job-receiving, begin_granted -> poll) remain. Ported: receiving/🦀️.rs metadata release and close_intent_source to `retired_poll` (Complete(RetainedWorkOutcome{Ok(step),work:1})); auxiliary.rs and forms.rs begin_granted sites to `begun`. All sites were close steps; none left Result-shaped. Note: `begun` maps a None begin step to Complete (old code raised `invalid`).
- BirthFrame.failure: nothing writes or reads it after admit lost its failure out-param (callees `OriginalResponseActionSource::admit` and `OriginalIntentActionSource::admit` take only admission+output). Removed `failure` and `retirement` (only retired the failure) from BirthFrame, plus `begin` re-export and unused imports; BirthFrame is now {admission}.
- Verified: W-reactor-10 native rc 0, W-reactor-11 wasm rc 0 (surfaces+context). Check only; no tests run.
STATUS: FIXED-verified W-reactor-11
