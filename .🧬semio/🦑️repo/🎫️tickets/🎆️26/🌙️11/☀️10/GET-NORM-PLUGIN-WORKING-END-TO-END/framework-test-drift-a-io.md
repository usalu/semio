# drift-a-io report

Status: sources edited, NOT compile-verified by me (coordinator throttled cargo; my checks were stopped while queued). Fix any residue from the coordinator's combined drift-a-all.log.

- io-control-testing lib: removed local CloseWork/with_effect/CLOSE_WORK. Scope::close now quotes work (retirement_work_demand) under a zero-limit finite_work loan, then closes under finite_work(units) and drives the effect via clone-source-testing drive::drive. Deps added: testing-admission, clone-source-testing; control-contract removed. Public with_decoding signature unchanged. Tests: retire via drive; Watched delegates close_into/recover_close_into/close_step/next_computational_work.
- io-binary-source test: drain uses drive (work quoted by owner). Dev-dep clone-source-testing added.
- diagnostic tests: shared controlled_step helper (cursor tests, pub(crate)) used by message tests and domain test; accept closures split for E0499; u128 work fix in cursor selected(). Dev-dep added.
- replication-error test: retirement_seed boxed as dyn RetirementCursor, local close_step via drive_reply with work from next_computational_work. Dev-dep added.
Risks to check: tally of released bytes in Watched (step_progress after close_into); empty-recipient quote path in Scope::close.
