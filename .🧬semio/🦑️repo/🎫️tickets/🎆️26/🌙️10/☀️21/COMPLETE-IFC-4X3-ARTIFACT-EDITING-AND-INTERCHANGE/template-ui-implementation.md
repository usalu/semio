# Released Template Selector Checkpoint

The actual native `applyIfcTemplate` action now uses `ActionArgDef::select` populated directly from the immutable complete release catalog. All 760 official names remain in release order, each exact name is the stored option value and locale-invariant data label, the argument is required, and there is no default selection. English and German field labels remain explicit. The existing retained command, typed application schema, planner and full semantic admission gate are unchanged.

The neutral contract is `templates/✍️apply/chooser.json`, with its closed JSON Schema and language-agnostic feature file. `chooser-tests.rs` inspects the actual action argument, actual derived select control, all labels and values, uniqueness, no default, and every carrier population. The already registered `templates.test.ts` consumes the same contract and exact released catalog.

Executed evidence:

- Template TS suite: 12 passing tests, 37 assertions, zero failures; focused strict TS checking succeeds.
- `template-chooser-oracle.py` freshly compared all 760 name/kind pairs with the independent IfcOpenShell 0.9.0.post1 distribution: zero mismatches and exact agreement on all seven carrier populations.
- Both changed Rust files parse through rustfmt with child traversal disabled. This is syntax evidence only.
- Authentic `cargo test -p semio-s-artifact-stdio-ifc-4x3 released_template_action_choices_are_complete_explicit_and_localized --no-default-features` refuses before `rustc -vV`: the repository launcher and authentic enforcing store are required. The test has not been type checked or executed. Full output is `🗑️generated/template-chooser-native-test.txt`.

Source inspection confirms the React staged-argument renderer derives a select and retains an empty value until explicit selection. The native time-travel control path pages option populations larger than its fixed list threshold. No actual native/React interaction test for the new complete selector has executed, and the native action-pane path still needs runtime acceptance.

This is a selector improvement, not complete guided template authoring. Entry-specific adjustable controls, staged identity allocation and declaration-dependent form production remain open. `host-template-form-integration.md` and `template-ui-plan.md` record the actual existing shell staging interfaces and the missing domain-neutral producer. Do not describe the remaining `Any` payload fields as guided forms.
