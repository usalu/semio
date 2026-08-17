# W4-TESTS — `semio-framework-plugin --lib` fallout fix

## Correction to the record

Several earlier lane reports called the 59 `cargo test -p semio-framework-plugin --lib` failures
"pre-existing unrelated failures" or "canonical surface id fixture debt" in a way that implied they
were inherited, not caused by this ticket. **That characterization was wrong for the large majority
of them.** 53 of the 59 failures panicked directly inside `AppBuilder::build_definition`'s new
assertion (`🦀️component.rs:5166`, contract §1 — `App::builder`'s `id` must parse as a canonical
`surface_app_id`), because the SDK's own test fixtures (`plugin_builder_contract_tests`,
`app_builder_tests`, `example_source_tests`, and one `testkit_tests` construction path) still built
apps with hand-written pre-migration ids such as `"synthetic-play"`, `"flat-menu-test"`,
`"good-app"`, `"history-app"`. That is direct fallout of this ticket's own work, not debt someone
else left behind, and shipping it under an "unrelated" label would have been a false claim.

The other 6 of the 59 are genuinely unrelated, pre-existing bugs with no connection to surface ids —
see "Left failing" below. Those are correctly out of this lease's scope; the record should say so
explicitly rather than lumping them in with the assertion fallout.

## What was fixed

All 53 assertion-caused failures are fixed by giving each fixture a **canonical** surface id built
the same way real surfaces do it — via `surface_app_id(&dialect, role)` from a fixture `Dialect`/
`ArtifactDialect`, never a hand-written string literal. Per module, one shared fixture id
helper/dialect was added and reused, instead of 53 ad-hoc literals:

- **`component::app::app_builder_tests`** (27 tests) — added `canonical_test_app_id(slug) -> String`
  (dialect `s.test.app-builder.<slug>@1/*#editor`), used it in `minimal_app(...)` and the three
  direct `App::builder(...)` calls (`good-app`, `icon-app`, `good-terminology-app`). The two tests
  that asserted a literal `controller_id` (`"history-app"`, `"clipboard-app"` — `AppBuilder`
  defaults `controller_id` to `id.clone()`) now assert against `canonical_test_app_id(...)` instead
  of the old literal.
- **`component::app::example_source_tests`** (2 tests) — same pattern, `canonical_test_app_id(slug)`
  with dialect `s.test.example-source.<slug>`.
- **`component::plugin_runtime::plugin_builder_contract_tests`** (24 tests) — added
  `TEST_APP_DIALECT: Dialect` (`s.test.synthetic`) and `test_app_surface_id() -> String`. `TestApp`'s
  runtime `ArtifactApp::APP_ID` const (still a hand-typed `&'static str` — the runtime trait's
  `APP_ID` cannot call a heap-allocating fn in a `const`, unlike the authoring traits' derived ids)
  now holds the same canonical string, guarded against drift by a new
  `test_app_id_matches_its_own_dialect` unit test that asserts `TestApp::APP_ID ==
  test_app_surface_id()`. Every `App::builder("synthetic-play", ...)` call, plus every downstream
  literal comparison that has to agree with it at runtime (`bundle.create_app(...)`,
  `app.app_id()`, `ClipboardFragment.source_app`, `CommandAddress`/`ActionAddress.app_id` —
  `dispatch_command` literally compares `app_id != A::APP_ID`), now reference `TestApp::APP_ID` or
  call `test_app_surface_id()` instead of the literal `"synthetic-play"`. One more fixture,
  `flat_menu_registry()`, used a second hand-written id (`"flat-menu-test"`, unrelated to the
  document label of the same text used inside `TestApp::context_menu`'s test branch) — switched to
  `test_app_surface_id()` too since nothing compares the exact app-id value there.

No production code changed. No assertion was weakened, deferred, or `#[ignore]`d.

## Result

| | before | after |
|---|---:|---:|
| `cargo test -p semio-framework-plugin --lib` | 160 passed, **59 failed** | **213 passed, 7 failed** |
| `cargo check -p semio-framework-plugin --all-targets --keep-going` | — | **0 errors** |

Full output in `🧪️w4-tests.txt` (this ticket folder).

## Left failing (7) — none are surface-id fallout, all out of this lease's scope

Confirmed by panic location: none of the 7 remaining failures hit `🦀️component.rs:5166` (the
canonical-id assertion). Each was re-run in isolation (`--test-threads=1`, single test at a time) to
rule out interference from the id-assertion panics that used to abort the whole suite early.

1. `component::app::artifact_definition_contract_tests::identities_and_locales_are_explicit_and_conflicts_do_not_overwrite`
2. `component::app::artifact_definition_contract_tests::plural_definition_carries_every_artifact_capability_without_a_dispatch_edit`
3. `component::app::artifact_definition_contract_tests::registry_rejects_duplicate_schema_dialect_codec_mime_and_extension_claims_atomically`

   These three fail inside `ArtifactDefinition::validate()`/`ArtifactDefinitionRegistry` with
   `"s.stdio.ifc.localized does not use the canonical **localization** identity grammar"` /
   `"...first-0 does not use the canonical **resource** identity grammar"`. That is a completely
   different identity system (`ArtifactIdentity`/`ArtifactDefinition`/`ArtifactCapability`, resource
   and localization grammar) from `AppRole`/`surface_app_id` (contract §1 here is about app/surface
   ids, not artifact resource identities). A dialect fixture has no bearing on this; fixing it would
   mean fixing a different, apparently mid-flight contract's grammar enforcement, well outside this
   lease's file/module scope.

4. `component::app::testkit::testkit_tests::assert_two_instances_converge_on_disjoint_edits` — fails
   with `Fault { code: "module.vcs", message: "validation failed: change ... has an invalid edit
   reference edit-..." }` on `pump b` (backbone convergence via `MemoryBackbone`). A VCS/store-layer
   bug, not an id problem; `DummyApp` here never goes through `AppBuilder` at all
   (`new_app::<DummyApp>()` is the registry-less constructor).

5. `component::plugin_runtime::plugin_builder_contract_tests::a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames`
   — fails intermittently with `register child factory: Conflict { kind: "s.test.child" }`; passes
   when run alone. This is cross-test global-registry pollution under parallel execution (a
   process-wide child-factory registry keyed by artifact kind), not a surface-id issue — it doesn't
   even always fail (confirmed flaky across repeated `cargo test` runs, see `🧪️w4-tests.txt`).

6. `component::plugin_runtime::plugin_builder_contract_tests::merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads`
   — fails seeding its fixture with `ValidationFailed("history repeats or omits conflict identity
   test-degraded-conflict")`, a history/merge-conflict validation bug unrelated to app ids.

7. `component::plugin_runtime::plugin_builder_contract_tests::view_action_emitting_ops_is_rejected` —
   **newly exposed** by this fix (previously masked: it used to panic at construction time on the
   `"synthetic-play"` id before ever reaching its own assertion). Now that `contract_registry()`
   builds successfully, the test runs its real body and finds that `TestCommand::BadView` — a
   `View`-kind action that intentionally emits an operation to prove the SDK rejects it — is *not*
   rejected. Root cause: `VcsArtifactApp::dispatch_typed_command_inner`'s kind-discipline guard
   (`🦀️component.rs` around line 11624) resolves the dispatched verb's `ActionKind` via
   `self.registry.get_command(&verb)`, which only looks in `AppActionRegistry`'s `app_commands`/
   `mode_commands` maps — `"badView"` was declared with `.view_action(...)` (a plain action, indexed
   in the registry's separate `actions` map, reachable via `AppActionRegistry::get`, not
   `get_command`), so the lookup returns `None` and the enforcement is silently skipped. This is a
   pre-existing SDK dispatch bug with nothing to do with dialects or canonical ids — fixing it means
   changing `dispatch_typed_command_inner` (production code, outside this lease's test-module-only
   lease) or possibly is intentional-but-under-tested behavior worth its own ticket. Left failing and
   flagged here rather than contorted into a dialect fix.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
  - `component::app::app_builder_tests` — added `canonical_test_app_id`; fixed `minimal_app` and 3
    direct `App::builder(...)` calls and 2 `controller_id` assertions.
  - `component::app::example_source_tests` — added `canonical_test_app_id`; fixed 2 `App::builder(...)` calls.
  - `component::plugin_runtime::plugin_builder_contract_tests` — added `TEST_APP_DIALECT`,
    `test_app_surface_id()`, and `test_app_id_matches_its_own_dialect` test; updated `TestApp::APP_ID`
    and every `"synthetic-play"`/`"flat-menu-test"` literal (13 + 1 sites) to the canonical id.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w4-tests.txt` — verification output (created).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/📓️w4-tests-report.md` — this report (created).

No other files were modified. No modifying git commands were run.
