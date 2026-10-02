# Full Macro Owner Fixture Consistency

Read-only JSON/source inspection; no schema validator, test or compiler executed. Current snapshot17macroScopes/42macroRejections/25macroOwners. Root concurrently updates schema, so recheck the exact reviewed fields before next producer.

## Schema Blockers Found

Rejection array cardinality42 is correct but its closed enum has41 IDs: missing raw-input-inside-print-token-tree. Owner cardinality25 is correct but enum has19 IDs, missing:

- inner-leaf-rewriting-attribute
- inner-root-rewriting-attribute
- raw-module-default-filename
- raw-module-identity-ambiguous-target
- function-local-in-inline-module
- inner-inline-rewriting-attribute

The17positive scope IDs match their enum by direct comparison. Preserve exact closure, not a relaxed arbitrary ID pattern.

## Incorrect Local Scope Golden

inner-root-rewriting-attribute source begins with an inner attribute before pub fn run. Its recorded start41 is a space before the opening brace; actual brace42. Recorded end192 is the closing brace, but half-open body scope ends193. Correct to[42,193). All other inspected local bounds correspond to authored brace/end boundaries: [13,164), [13,189), [51,202), [26,177), [55,206). Sources are ASCII for these boundary calculations, so these are also UTF16 positions.

## Mount And Problem Expectations

All25owner rows native=true, so unconditional binary execution currently has no native=false contradiction. Unsupported-expression rows are attributed-function-cannot-seal and function-local-with-code-emission; the test intentionally does not require two refs for them. Others require two distinct templateOffset facts, retaining constant and dynamic same-input sites. Problem arrays are derived from the singular closed row.problem; complete gate assertions remain violations[] plus exact from/code arrays.

Raw-module-identity-ambiguous-target uses active all() one.rs sealed and inactive any() two.rs r#sealed, so default native success is coherent. Its expected mounts['module'] retains physical origin despite canonical target ambiguity; this is consistent only if graph preserves observable nonadmitted contexts separately from successful authority. If the new scope provider removes poisoned contexts entirely, update this expected observable shape coherently rather than treating test golden as authority. The gate must refuse that source's finite proof either way.

Inline local positive and inner-inline rewrite rows record mounts['root'] because harness filters contexts to sourceScope.length===0 before reading mount.kind. This does not independently assert the actual inside inline context. Add canonical sourceScope['inside'] context/proof expectation and require the positive finite refs' modulePath['inside']; otherwise the row may pass its mount assertion while inline membership is missing or wrongly granted.

Unknown outer rewrite/path alternatives and unknown inner leaf/root expect no eligible contexts; private normal/raw-default rows expect module. These need the same explicit distinction between observed provenance and admitted scope authority across all rows. Missing manifest deletion and replacement parent-mount rows still materialize/delete or replace actual files, with runtime output checks disabled only for removed parent mount.

Temporary refs preparation catches unsupported scanner errors into[] only to build graph; whole inspectRustSourceDirection later must report the exact unsupported error. Do not reuse that fallback as production admission. No observed row requires weakening complete-report assertions.

## Remaining Verification Boundaries

Default native-only cfg rows prove syntax/default behavior; they do not independently prove the alternate enabled branch. Opposite-branch native cases remain necessary for same-key alternative semantic coverage. Fixture file maps should require root/source existing and removals actual keys in schema/tests; current schema files accepts arbitrary string keys without portable path constraints. Retain actual emitted dep-info and runtime witnesses once the registered producer reaches native execution. No passing claim follows this audit.
