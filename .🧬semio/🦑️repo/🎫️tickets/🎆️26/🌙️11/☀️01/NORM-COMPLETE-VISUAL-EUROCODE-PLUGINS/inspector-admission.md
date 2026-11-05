# Scalar Inspector Admission

Current validation: actual proper package31479 completed exit0 with11418tests, zero failures,79729assertions in61.52seconds. It includes all17newinspectoradmissioncases and the corrected nested-edit contract. The native rendered UI assertions remain pending with native_full; package success is not substituted for that gate.

2026-11-04. This repairs the source-confirmed VDI product identity edit affordance described in audits/e2e-inspector-admission-audit.md. Product identity remains immutable and its value remains visible. No editing rule, identity mutation, external runtime dependency or admission gate was added or weakened.

## Contract and Implementation

A closed language-neutral JSON fixture and independent strict Ajv schema cover fifteen family providers, both explicit interface locales, text identity, quantities, integers, booleans, localized choices, null values, missing descriptive metadata and admitted fields.

FieldSpec and FieldView now carry read_only. admit_field_meta combines each family's actual EDIT_RULES.can_set_field(path) predicate with intentional read-only metadata. The root-owned predicate handles exact and nearest typed ancestor setters; typed document addressing and semantic payload validation remain commit authority. All fifteen providers use that native vocabulary. EN1997 owns a thin editor metadata adapter over its check-owned descriptions, avoiding a dependency cycle.

The shared control renderer emits an accessible localized read-only value instead of a bound input, slider, select, toggle or stepper. Quantities retain their selected display unit and decimal locale; choices retain localized labels; booleans, empty values and mixed bulk values have localized copy. Unknown descriptive metadata keeps the raw field name and visible value. A bulk write requires admission for every selected row. Field-specific NDP reset, annex reset and automatic remedy/preview affordances are suppressed for inadmissible fields. Independent list insert/remove behavior is preserved, as requested by root.

## TDD Evidence

Native UI assertions were authored before production changes: a shared fixture replay over six scalar kinds plus metadata fallback, a retained admitted control, an all-selected-rows bulk law and an actual VDI product identity control using the VDI metadata provider. Native field metadata tests assert explicit read-only intent and fallback labels. Native execution is owned by native_full and remains pending; these authored assertions are not a rendered UI pass.

The independent Bun schema/wiring suite first reported sixteen production admission failures after the schema witness admitted its fixture (1 pass, 16 fail). An initial schema authoring error used a union rejected by strict Ajv; it was corrected to anyOf without changing the oracle. After production changes, all17tests passed, zero failures,51assertions in393ms. It checks the real fifteen provider functions against each family's actual setter predicate and the real metadata/control pipeline; it does not execute Rust UI rendering.

Actual proper main15442 included this new suite and passed all17cases. The complete package run executed11418tests across45files:11417pass,1fail,79722assertions in91.62seconds. The sole failure is the root-owned nested-edit fixture schema's strictRequired error; all fifteen TypeScript schema compiler closures passed with zero diagnostics in22.7seconds. Labels, canonical/source agreement, strict wires, deletion and browser scenario contracts also passed. No full package pass is claimed.

## Native Fixture Maintenance

Existing shared rendering mocks now declare admitted boolean/count fields and an explicit bulk editing vocabulary. Existing metadata coverage still checks nonempty EN/DE labels; EN1993 container coverage was strengthened to reject fallback-only metadata. Existing ISO/DIN4108 native metadata tests now use the actual FieldSpec label/unit/choices interface instead of removed field names. Assertions retain their original expected values.

## Evidence and Pending Work

Generated logs: inspector-red.log (initial schema witness plus production red), inspector-red-2.log (1 pass,16fail), inspector-green.log (17pass), rt-main-3.log (actual full package result). These remain under the ticket generated folder until ticket completion.

Native UI assertion execution, all-family editor compilation and actual hosted interaction remain required. Root owns the browser harness and nested-edit schema correction. native_full owns shared inspection and VDI native verification. No foreign job or owned main was restarted because of elapsed waiting.
