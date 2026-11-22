# Plugin Units Preservation Test

Test: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts`.
Result: 4 pass / 7 fail before, 9 pass / 2 fail after (`repo test-file` and `bun test` agree).

Cause overview: the records were last recorded Oct 20 to Nov 5 and the sources moved under them (Nov 5 to Nov 22) through other agents' work, not through the new `retained_window_input_...` test alone. Re-record scripts live in this folder (`plugin-units-*.ts`, run with the repo's bun, `--write` applies).

## Fixed

1. Builder laws (`📋️preserved.json`, `plugin-units-preserved-check.ts`)
   - 188 laws had changed text with identical assertion counts (re-hashed). Seven grew assertions (re-recorded, growth only).
   - Five new laws live in already recorded sources and were recorded as rows: `typed_cancellation_projection_obeys_the_neutral_contract`, `select_all_takes_an_optional_granularity_scope_and_ignores_undeclared_ids`, `a_queued_task_resume_is_delivered_across_funded_turns_and_answers_once`, `a_refused_task_resume_answers_with_a_fault_and_the_next_one_still_delivers`, `a_cancelled_task_resume_queue_returns_every_owner_after_any_number_of_turns`. `addedAssertions` is now total minus original (1169 - 1096), schema row count 216 to 221.
   - One genuine assertion removal is kept visible, not hidden: `ui_tree_stamping_caches_interaction_topology_from_a_domain_bound_tree` lost `assert!(previous.is_empty())` because `stamp_and_cache_interaction_ui` no longer returns the previous topology. Recorded as `removedAssertions: 1` plus `removalReason` (new optional schema fields; the test now requires every such row to be named and to carry a reason).
   - Command-page target now links `semio-framework-value-testing-allocator` (it owns the one `#[global_allocator]`); the test checks the dev-dependency, the `use ... as _;` link and the allocator owner instead of a local `#[global_allocator]`.
2. Feature verdict owner (`📋️feature-owner.json`, `📋️feature-locks.json`)
   - Facade mount list: `🎟️caller/👷️work` replaced by `📨️host-event-observation` (`🧪️testing/🦀️.rs`).
   - tree-sitter-rust (wasm) cannot parse `type Context<'t,'s> = () where 's: 't;` in `🎟️fixture-dispatch-steps-and-document-loading`; the test normalizes that one GAT form before parsing (source left valid Rust).
   - `remainingFunctions` re-recorded (167 to 215); removed names are helpers only: `typing_run`, `context`, `initial_snapshot` (moved into the caller crate).
   - Defining records re-hashed: `DeclaredVerbOutcome` (+ its two impls) gained `failure: Option<PluginLifecycleFailure>`, `declared_bridge_arguments` gained schema validation; `testsSha256` updated, laws unchanged in assertions.
   - Third-party pin hashes re-recorded for 146 of 174 locks (ordinary crates.io pin churn).
   - Expected `semio-plugin-ownership` dependency list and declared-verdict lock order updated to the sorted Cargo order.
3. Surface carriers (`📋️surface-carriers.json`, `📋️surface-carrier-lock.json` pins): 9 of 13 bodies re-hashed (assertions unchanged); `remaining` re-recorded (476 to 489; added helpers and laws from other agents; helper `build_tool_job` lost one assertion, a non-law helper, recorded as is); `dependencies` sorted; the global allocator check moved to the allocator owner as above.
4. Surface fixture (`📋️surface-fixture.json`, `🧬️surface-fixture.json`, `📋️surface-fixture-lock.json`, test constants)
   - 34 new function rows from `📄️natural-file-reserved-job-laws` and `🧵️retained-work-and-child-emission-laws` and others (10 new laws: 22 to 32; total 183 to 212; test title and constant updated). 67 bodies re-hashed, no assertion drop.
   - Six zero-assertion bodies dropped (`parse_op`/`print_op`/`encode_op`/`decode_op` in `set-surface-count`, now derived; two `initial_snapshot`).
   - Defining proof macro `bounded_first_step_tool_proofs` moved to `📜️declaration/👷️job/🧾️bounded-proof-and-factory-registry`; body identical (same sha, 6 scope replacements).
   - Dependency edges: +4 (`schema-state`, `value-testing-admission`, `decl-tool-contract`, `instance-jobs`), `semio-os-kernel` gained `protocol-laws`; workspace-inherited edges resolve through the root `[workspace.dependencies]`. Lock entry records updated from the manifests (manifest equals lock).
5. `📋️feature-consumers.json` dependency list updated to the root lock (+`decl-media`, `decl-tool-contract`).

## Still failing (not preservation drift): stale `Cargo.lock` files

Both remaining failures (`registration dependencies and every live plugin lock...` and `feature verdict owner...`) stop in the loop over the 174 roots in `📋️locks.json`:

- 44 locks still carry the old `semio-plugin-ownership` dependency set (all `🏭️bridge` locks, ~25 `🗄️stdio/🗿️artifacts/*`, `🪐️space`, three `🧑‍💻dev` roots).
- `🔋️energy/Cargo.lock` and `🗄️stdio/🗿️artifacts/☁️las/Cargo.lock` lack `semio-plugin-registration` and `semio-plugin` although their manifests depend on it (las still names the removed `semio-framework-plugin-registration`).
- 17 locks changed whether `semio-plugin-testing-laws` is present (record `presence` not updated, it is lock state, not source); `semio-plugin-testing-caller` dependency lists differ per lock.

They need a cargo lock refresh, which was out of scope (no cargo builds). A scratch copy of the test with those locks filtered out passed registration fully and failed only on the consumer/caller lock lists, so no further source drift is hidden behind them. Another agent's lock regeneration (the `Cargo.lock` files modified Nov 21 and Nov 22) is in flight; after it lands re-run the test and, if `presence` or caller lists still differ, re-record via `plugin-units-consumers.ts`.
