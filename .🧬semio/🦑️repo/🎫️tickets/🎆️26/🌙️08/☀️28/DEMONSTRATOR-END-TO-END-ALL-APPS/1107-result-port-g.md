STATUS: PARTIAL. Wasm GREEN (checked, no error in W-port-g-12.txt; earlier W-port-g-3/4): job-owned-text, job-worker-authority, actor-retained-turn, mounted-owner, framework-actor, actor-job-state, actor-receipt, testing-caller. UNVERIFIED: retained-command, reactor runtime-ops, instance-app callers (not reached: upstream os-history 73, os-dsl 5, manifest-actions 1, all peer-claimed); native and tests not run.

## Edits
- job-owned-text: FundedText advance_original/take_original/close_step take &mut RetainedAllocationEffect (performed(work) once, reported work = effect.performed_work()); advance/take_into enter via cx.with_original_effect inside the work loan and charge once with receive_original_retained_work (overrun guard); construction admission removed. Tests ported (close_original_effect_step, denied loans via with_original_work_loan), UNVERIFIED (not compiled).
- job-worker-authority: u128 work; close_original_worker_failure(cx, grant) runs the retirement child and error birth inside cx.with_original_effect, single charge via receive_original_effect_poll.
- actor-retained-turn (job source close_step): owner.step in with_original_effect, consume_work once, OriginalRetirementFailure::into_value_error.
- mounted-owner actor capture: SharedUtf8::admit with local actual marker, u128 work.
- testing-caller Cargo.toml: +6 deps moved with OriginalWorkFixture from testing-laws (LOCK NEEDED).
- framework-actor wire-turn: page present/close_step, backing close_step, receiver close_step, diagnostic advance now take the effect; helper poll_step. Callers to port (callers lane): retained-command full-operation:423, pending:147, diagnostics sites, reactor 📮️publication.
## Rerun
chk.sh W-port-g-8 🌎️hub/Cargo.toml wasm -p semio-framework-job-owned-text -p semio-framework-job-worker-authority -p semio-framework-actor-retained-turn -p semio-plugin-mounted-owner -p semio-framework-actor -p semio-plugin-testing-caller, then native and tests (--tests for job-owned-text).


## Later edits
- actor-receipt bootstrap validate_next_budget rebuilds the remaining RetainedTurnInput from the receipt. actor-job-state close helpers take the effect, charge via cx.consume_work once.
- framework-actor delivery/receiving advance+close_step take the effect. Callers ported (UNVERIFIED): retained-command take_result_page/pending close_step, reactor publication, PluginApp::take_typed_operation_result_page(receiver, effect) + instance impl, artifact_app_laws::take_fixture_result_turn (wallet, tests only) and test callers. Not ported: actor crate tests, reactor publication tests close_pages.