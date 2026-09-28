# Authored Fill Rules

## Reason

Editable SVG import needs to retain both nonzero and even-odd winding. Draw previously hardcoded `evenodd` when producing path scene nodes, while retained pointer picking unconditionally used nonzero winding. Compound paths could therefore be painted as holes but selected as filled regions.

## Changes

- Added the shared `DrawingFillRule` schema and owned Rust/TypeScript `FillRule` vocabulary (`evenodd`, `nonzero`). New Draw artwork uses even-odd by default; SVG import must explicitly retain the source rule, including SVG's nonzero default.
- Stored the rule on `DrawingAttributes`. Scene projection and retained pointer picking now read the same authored value. Existing SVG and PDF exporters consume the projected rule.
- Preserved the value in the retained layer clone skeleton and included it in the incremental semantic digest with a separate bounded step.
- Updated GraphQL/Protobuf declarations and snapshot text grammars, the demo DSL asset, TypeScript creation defaults and 42 authored fixture files.
- Added four neutral compound-path cases covering matching and opposite contour directions, plus invalid enum cases. TypeScript picking is checked against Three.js SVGLoader and triangulated shapes, with xmldom used only by tests.
- Added native pack/scene/SVG checks, retained pointer-query checks, and a digest-field sensitivity case. These tests are authored and remain unverified.

## Evidence

- `tests-fill-rule-red.txt`: test-first failure on the missing implementation.
- `tests-fill-rule-current.txt`: registered TypeScript target passed 141 tests, 18,710 assertions, 24 files.
- `tests-fill-rule-fixtures.txt`: same 141 tests / 18,710 assertions passed after the authored fixture and creation-default updates. Existing 44 field-patch cases and 36-command publication audit passed.
- `tests-native-fill-rule.txt`: native run 29440 terminated before Draw compilation, because shared `semio-framework-ui` has an uncovered `SceneMaterialKind3d::Authored` match arm. This is distinct from the preceding OS-kernel compiler diagnostics. No unrelated framework source was changed.
- Component build 93519 is confirmed live and has not yet produced a new component for browser verification.

## Remaining

Validate the authored semantic mutation and bilingual inspector control through native tests and the updated app. Confirm undo/redo, archive restoration and rendering in the app. Complete native checks once shared compilation permits them. This work fixes the data model and projection prerequisite; it does not complete SVG document import or the broader editing goal.

## Semantic Editing and Inspector

The `set-layer-fill-rule` semantic mutation now has a strict schema, Rust/TypeScript implementations, sparse diff, inverse, operation manifest, binary tag, tagged-union schema and oracle catalog entry. Retained candidate mutation, owner retirement, target discovery and semantic digest include the rule. Existing authored sparse-diff fixtures explicitly include the new nullable lane.

The Properties panel exposes localized Even-Odd/Nonzero choices. Multi-selection edits use the existing `patchLayers` command, preserving one semantic history transaction. Locked selections disable this field. Mixed select fields now display the localized mixed-value placeholder. The field-patch contract includes two accepted enum values and two rejected inputs.

Native tests were added for canonical diff/application/inversion, text/binary codec roundtrips, missing/no-op targets, two-layer undo/redo and inspector selection/lock/localization behavior. These are authored, not yet confirmed. Native semantic descriptor count is now 17 and the existing retained-candidate test includes the new variant.

- `tests-fill-rule-mutation-red.txt`: test-first run failed because the semantic setter module was absent.
- `tests-fill-rule-mutation-current.txt`: registered TypeScript target passed **142 tests / 18,718 assertions**, including independent fast-json-patch comparison. The independent Ajv field-patch audit passed **48 cases**; publication audit retained **36 commands**.
- Native run **79513**, `tests-native-fill-rule-mutation.txt`, remains live. Prior native failures in shared framework compilation do not establish this run's outcome.
- Catalog/fixture follow-up TypeScript run **24026**, `tests-fill-rule-catalog-current.txt`, also passed **142 tests / 18,718 assertions**, 48 field-patch cases and the 36-command publication audit (exit 0).
- Component build **93519** remains live; the new inspector has not been verified in the browser. No duplicate build or unrelated process cancellation was performed.

The goal and ticket remain open. SVG document import and the broader end-user acceptance gaps remain incomplete.
