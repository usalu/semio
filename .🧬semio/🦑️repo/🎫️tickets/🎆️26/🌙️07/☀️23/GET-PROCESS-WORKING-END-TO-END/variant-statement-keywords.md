# Variant Statement Keywords

Record-open of an original drawing fails inside `FillStyleNativeReceiver` while selecting `LinearGradient`.

`encode_pack` writes the DSL record (`__dsl_to_record`). A `#[dsl(statements)]` field is `FieldValue::Statements`, and `variant_tag()` is the statement keyword. The DSL derive emits `#[dsl(key)]` or `to_kebab(ident)`, so `LinearGradient` is `linear-gradient`. The native variant selector matched that tag against `spec.all`, whose leaves are the value wires (`rename_all = "camelCase"`, `linearGradient`). Phase 4 then called `invalid()`.

`NativeVariantSpec` now has `statements`. Phase 1 (`NativeSchemaKeySource::VariantTag`) matches that schema. `spec.all` stays the value wires for `RecordText` (`kind: "linearGradient"`). The receive derive emits one statement leaf per variant, in variant order, so `Field(index)` is still the variant index. Unit enums, `Result`, the refusal kind, and the variant work law set `statements` explicitly.

Hot-path `[DEBUG]` prints were removed from `invalid()`, `marker_mismatch`, partial-work settlement, and paged-list leaf admission. They allocated inside `observe_retirement_allocations` and broke the exact heap receipt on `genuine_original_drawing_member_receives_closes_and_funds_exact_handoff`.

`seed-rec5` compiled far enough to fail `semio-s-artifact-draw-drawing-model` before the test ran: `FillStyle` variant fields were `pub` (E0449; the enum already publishes them), and `testkit` compiled `test_step` without linking `semio-framework-value-clone-source-testing`. `test_step` is now `cfg(test)` only, and `testkit` enables serde alone. The variant fields no longer carry `pub`. The rerun waits while other cargos hold `native/target`.

A later read of the schema walker agrees: `combined()` is only the union pop, and `DrawingGroupBody` has no flatten/own name collision. Do not change `combined()` or the `"dup"` oracle. `invalid()` no longer prints a backtrace. `semio-plugin` already finished a lib check with no compile error. The editor catalogue failure is a fixture turn bound (`WorkLimit` at the contribution command), separate from this selector. Eighteen stale test packages still fail to compile; that work starts after record-open is green.
