# Tool Factory Duplicate `setActiveUtility`

## Cause
`ingress` (`🧵️instance/📥️staging/🚪️ingress`) registers two factory sets on one controller: first `register_framework_reserved_factories` (includes `FrameworkSetActiveUtilityJobFactory`, key `<controller>/setActiveUtility`), then the app's `register_tool_job_factories`. The tests-05 fixture `TestAppCommandFactory` (`🔬️plugin-runtime-plugin-builder-contract-x`) listed `"setActiveUtility"` in `TEST_APP_COMMAND_TOOL_IDS` and `TEST_APP_COMMAND_PUBLICATION_CONTRACTS`, so the same key was claimed twice and the action bus refused it with `ToolRegistrationError::DuplicateKey`.

## Owner
The framework. `setActiveUtility` is injected by the builder whenever an app declares utilities and `handle_action_invocation` serves it directly (`SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID` branch in `🧷️command-routes/📥️staging/🚦️actions`); the app never owns a tool factory for it. Real plugins (CAD `CAD_RETAINED_TOOL_IDS`) already omit it, and no other `TOOL_IDS` in the repo lists a reserved verb. It is not a `TOOL_JOB_IDS` proof row of `TestCommand`, so the proof catalog is unaffected.

## Fix
Removed `"setActiveUtility"` from `TEST_APP_COMMAND_TOOL_IDS` and its row from `TEST_APP_COMMAND_PUBLICATION_CONTRACTS`; the docstring now states reserved verbs are never listed. `TestCommand::SetActiveUtility` and `command_id` stay.

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-x/🦀️.rs`

## Not verified (static work only), watch for
- `set_active_utility_carries_its_value_directly_and_emits_no_operations` (part-5) asserts an `active-utility` event from `TestCommand::SetActiveUtility`; the framework path now answers with an empty emission, so this law may need to assert the framework-served shape instead.

## Command
`repo cargo --lang=en --store=shared --configuration=prototype --parallelism=2 -- test --manifest-path <root>/Cargo.toml -p semio-plugin-tests-08 --lib reserved_section_surfaces_render_accessor_maps_instead_of_app_bodies`
(also run `-p semio-plugin-tests-05 --lib` and the part-5 `set_active_utility` law)
