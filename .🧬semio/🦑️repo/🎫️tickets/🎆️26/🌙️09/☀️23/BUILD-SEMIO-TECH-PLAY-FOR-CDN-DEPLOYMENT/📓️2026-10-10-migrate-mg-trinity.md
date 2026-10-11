# 2026-10-10 mg-trinity: source-level migration done, NOT compiler-verified

Executor `mg-trinity`, scope `✏️s/🔌️plugins/🔱️trinity/**`. Every file below was migrated to the grant-based retained-clone protocol at source level. No cargo check or test has run, because `os.status` never reached `GREEN os` (last value `PARTIAL os 11:50`: `semio-framework-plugin` has 455 errors, and every trinity crate depends on it). Verification (native + wasm32-wasip2 check per crate, then `--lib` tests) is pending.

## Check / test results (release path GREEN)
Slot-gated `cargo check --lib`; logs in `.🧬semio/🦑️repo/⚡️cache/play-fleet/mg-trinity/`.
- jack native: `Finished dev profile`, 0 errors.
- jack native `--features component-app-assembly`: `Finished dev profile`, 0 errors.
- jack `--target wasm32-wasip2 --features component-app-assembly`: `Finished dev profile` in 2m 26s, 0 errors.
- rewriting native: `Finished dev profile` in 3m 53s, 0 errors.
- rewriting native `--features component-app-assembly`: `Finished dev profile` in 3m 28s, 0 errors.
- rewriting `--target wasm32-wasip2 --features component-app-assembly`: `Finished dev profile` in 1m 45s, 0 errors.
- `--tests` and unit tests: NOT RUN yet (corrections #45: tests after the cold release passes). Test sources are migrated at source level; rewriting still needs `test_identity!` in its lib root and the identity argument at its `dispatch` test call sites (operations tests, child-frame test).

Rewriting editor fixes found by the component build: window-config text/binary mutation codecs no longer import private `set_camera`/`set_lod_mode` modules; set-lhs/rhs-json commands import `RewriteRuleMutation` from `schema::mutations`; `compiled_jack_query` calls `io::text::snapshot::build_rule_query`.

## Out-of-scope errors
None open.

## Files changed (all under `🗿️artifacts/`)
### jack
- `🦀️.rs`: `RetireOwned` on Port/Node/Edge/EntityRef; Camera derives `RetainedClone` + `CanonicalJsonTree`; removed the plugin-local camera edit module.
- `🪆️content/🦀️.rs`: `JackContentOwner` is a `RetireOwned` (Drop and `take_snapshot` removed); new `trinity_mounted_owner_policy`, `jack_self_funded_grant`, `drive_retirement_to_terminal`, `retire_owned_to_terminal`, `JACK_SELF_FUNDED_BODY_BYTES`.
- `🛂️manifest/🦀️.rs`: Manifest/NodeKindDef/EdgeKindDef/PortKindDef derive `RetainedClone`.
- `🔨️modules/🏠️host/🦀️.rs` (rewritten): bespoke `JackOwnedRetirement` and three retirement factories deleted; snapshot/mutation decode authorities, rejected-conflict authority and clone authority on the grant protocol; `JackStoreInitializationAuthority` ported (BuildOwners phase, outcome lending through `JobOutcomeBorrow`, fault publication, grant-based close); new `jack_document_preparation_factory` (see below).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/`:
  - `🧬️schema/♻️retirement/🦀️.rs`: manual `RetireOwned for JackSnapshot` (local owner moved out of the child handle and retired as its own deferred field).
  - `🧬️schema/📸️snapshot/🦀️.rs`: `JackSnapshot` derives `RetainedClone`.
  - `🧬️schema/🌳️ast/🦀️.rs`, `🧮️executor/🦀️.rs`: `RetireOwned` on Query/Clause/Pattern/.../QueryResult/GraphEffect.
  - `🧬️schema/🧮️executor/🪜️execution/🦀️.rs`: `QueryExecutionPreparation`/`QueryExecution` carry staged closing owners with `close_demands`/`close_step(grant)`; displaced values retired synchronously.
  - `🧬️schema/🧬️mutations/🦀️.rs` and `🔎️set-query/🦀️.rs`: `RetireOwned` + `RetainedClone` + `CanonicalJsonTree`; `TrinityGraphMutation` implements `WindowConfigApplyMutation<JackSnapshot>` (set-query exchange, bounded by `JACK_QUERY_MAXIMUM_BYTES`).
  - `🧬️schema/⚙️operations/🦀️.rs`, `🚪️io/💾️binary/🧬️mutations/🦀️.rs`: stores installed with `install_unscheduled_catalog` + `funded_bounded_artifact_store_owners`, closed with `close_owned_unscheduled`.
  - `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` and its test: `retire_owned_to_terminal`.
  - `✏️editor/🦀️.rs`: `TrinityJackCommand` derives `RetireOwned`; owner/disposer overrides deleted; document preparation now `jack_document_preparation_factory()`.
  - `✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs`: `JackQueryWork` on `checkpoint_byte`, `terminal_frame_release_bytes`, grant-based `close_step` and the four `next_close_*_demand` methods.
  - `👁️viewer/🦀️.rs`: `TrinityJackViewCommand` derives `RetireOwned`; owners override deleted.
  - `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/**`: bespoke retirement and factory deleted (framework factories); mutation types derive `RetireOwned`.
  - `✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/**`: manual `RetireOwned` replaced by the derive.
  - `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/**`: state derives `RetireOwned`/`RetainedClone`; mutations derive `RetireOwned`/`RetainedClone`/`CanonicalJsonTree`; owner uses `WindowConfigApplyEdit` with `WindowConfigApplyMutation` (exchange of camera / lod mode).
- Tests migrated: host unit tests, executor unit tests, editor unit tests, results-transient bounded test (five-currency grants, demand-funded close loops), reorganize-tool test, sqlite snapshot test.

### rewriting
- `🦀️.rs` (`LayoutPoint` canonical tree), `🧬️schema/♻️retirement/🦀️.rs` (manual `RewriteRuleMutation` retirement removed, `document_store_owners` removed), `🧬️schema/🧬️mutations/**` (enum + 9 payloads: `RetireOwned` + `CanonicalJsonTree`), `🧬️schema/🌳️typed/**` (rule types canonical tree, `RuleLayout` full `RetireOwned`), `🧬️schema/⚙️operations/🦀️.rs`, `✏️editor/🦀️.rs`, `👁️viewer/🦀️.rs` (owners overrides deleted, `RetireOwned` on commands), `✏️editor/🪟️window/🎚️config/**` (window config on `WindowConfigApplyEdit`).
- Tests migrated: document-retirement test (rewritten to demand-funded grants; asserts the last-alias semantic and equal totals for shared vs owned retirement of the same value; the old exact byte totals from the fixture are no longer asserted because the new `released_bytes` is not the fixture's heap-byte model, the TS oracle still validates the fixture independently), child-frame test, editor unit test, window-config ownership test, sqlite snapshot test.

## Camera edit versus `WindowConfigReplaceEdit`
My plugin-local `CameraWindowEdit` was deleted per corrections #23. `WindowConfigReplaceEdit` could not be used because it requires `S: Copy` and `lod_mode` is a `String`; both window configs now use `WindowConfigApplyEdit` (field-set exchange, `S: RetainedClone`).

## Open items / blockers
1. `semio-framework-plugin` is RED (455 errors, rn-os / pl-a..pl-d). All verification waits for `GREEN os`.
2. Rewriting document one-item preparation is still `bounded_config_store_one_item_preparation_factory`, whose `advance` always refuses (`UnsupportedOwner`), so rewriting document mutations cannot be applied until a real edit exists. It cannot use `WindowConfigApplyEdit`: its nine mutations insert/remove `PropertyBag` / `RuleLayout` map entries and produce zero-or-more inverse rows (`drag-rule-nodes`, `set-rule-layout-points`), which allocates inside `exchange` (the contract forbids that), and `BTreeMap` has no `RetainedClone` yet (corrections: only ordered/paged maps). Needs a decision: either a dedicated `RetainedCloneEdit` over ordered maps, or accepting a bounded-allocation exchange. `RewritingSnapshot` also has no `RetainedClone` for the same reason.
3. `OptionCursor::advance` bug (value-core, corrections "Costs / findings" 1) will hit `JackSnapshot` (`manifest_id`, `root_node_id` are usually `None`) until fixed.
4. Jack document preparation clones the whole `JackSnapshot` (content child is aliased, not copied), capped by the store at 1 MiB; queries are already bounded at `JACK_QUERY_MAXIMUM_BYTES` (3584).
5. Demand/copy numbers in the Jack initializer, executor and query job ladders are unverified until the first compile and test run.
6. `retire_envelope` (ungranted bundle form) is still used by Jack's envelope helper; rn-os plans to remove it.

## Document lanes (updated)
Both Jack and Rewriting now use `store::mutation_apply_preparation_factory::<Snapshot, Mutation>()`. Jack's whole-snapshot exchange (`jack_document_preparation_factory`, `WindowConfigApplyMutation<JackSnapshot>` on `TrinityGraphMutation`) was removed. Rewriting gained `RetainedClone` on `RewritingSnapshot`, `Lhs`, `Rhs`, `Pattern`, `Assignment`, `ParameterSpec`, `ParameterKind`, `RuleLayout`, `LayoutPoint`; the unused `REWRITING_ARTIFACT_MUTATION_MAXIMUM_BYTES` was deleted. Requirements to verify at compile: `OpBinary` for both mutation enums and `x-semio-inverse-rows` on the drag-rule / set-rule-layout-points leaf schemas.
