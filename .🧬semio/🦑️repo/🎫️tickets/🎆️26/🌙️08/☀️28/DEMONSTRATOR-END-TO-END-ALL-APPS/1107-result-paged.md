STATUS: semio-os-store-paged-one-item lib VERIFIED native (W-paged-105 via testing-laws gateway, no paged error) and wasm32-wasip2 (W-paged-107: Checking reached, not among failed crates); gateway now red only upstream (store-read 13, replay, one-item-batch 32, member-open 36, codec-registry 4). Tests UNVERIFIED.

## Changes
- OneItemOwners moved from the batch crate into paged-one-item/📬️owners (batch re-exports it); close_step(effect)->Poll via RetirementPollExit::settle. Cargo: feature durable-group-testing (LOCK NEEDED logged, features only).
- cold-apply: SemanticProjection/SemanticEdit ported (effect, no last_turn_work); preparation close ladder on effect; Messages phase quotes advance_work_demand; test seam is now cold_projection_metadata_owner (law must drive it with its own effect; old json law file unwired).
- root: advance = with_effect turn per phase (quote probe 2^20 down to 8, /4 per refusal); Edit-without-post-read, nonterminal CloseEdit (receive_effect frame) and Sealing stay on the direct receiving path; Seal phase now quoted (1) instead of a typed refusal; close ladder fully effect-shaped, pending keeps state, cursor close start is paid (begin_close_paid).
- mutation-apply: all stages on receive_effect (Birth, Decide, Inverse frames) or with_effect one-shot; FactoryIssuerIdentity capture removed (gone upstream).
- Cross-lane: PagedOneItemEdit close_* now take &Arc factory, close_step(&Arc,effect)->Poll; store3 SemanticEdit/drivers adapted. Plugin editors (mesh, brep, zip, docx), one-item fixture and paged tests still use the old shapes (not on lib path).

## Notes for others
- store3: with_effect(.., u64::MAX) is refused by the Job work recipient (observed+additional<=fuel); MutationApplyEdit opens its own actions, so it cannot run nested in the outer Stage::Edit effect. Also begin() paying one(effect) in both driver and native caller double-charges.
- Failure receipts no longer add the parent copy bytes in mutation-apply preflight (OriginalRetirementFailure carries no setter).

## plugin-job split (coordinator request)
child_vcs (3 files, 28 KB) moved to new crate semio-plugin-job-child-vcs (plugin/👷️job/🏛️reserved/🪆️child-vcs/📦️packages/🦀️rust, files stay in place); built/built_with/retired moved to semio-plugin-job-receiving::original_steps. plugin-job closure 262,957 -> 233,099 B (12 files, identical with testkit). Admission of plugin-job, child-vcs and receiving completed with no refusal in W-paged-107 (wasm); their compile was not reached (blocked upstream). The child-vcs unit tests are still not wired as [[test]] (they never were); imports updated.
