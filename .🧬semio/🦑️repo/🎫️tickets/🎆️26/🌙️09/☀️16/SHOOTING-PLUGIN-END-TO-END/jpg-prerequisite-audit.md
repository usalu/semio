# JPG Prerequisite Audit

Read-only inspection of the terminal native-cargo.log and current JPG source/mount/manifests. No source edits, builds, tests, graph work or duplicate commands. Native owns correction and rerun.

## Actual Failure

Original native session26714 ended with repository command status101. The raw compiler result reports two unresolved semio_framework_ui_contract imports in document/baseline main windows and one missing jpg_baseline::paint_region, then could not compile semio-s-artifact-stdio-jpg-schema-2. This is an actual compiler failure, not a capacity snapshot or failed shooting assertion.

## True Compilation And Module Ownership

The failing normal library owner is baseline/schema/packages/rust/Cargo.toml named semio-s-artifact-stdio-jpg-schema-2. Its lib mounts mirror and reexports mirror publicly. That mirror directly path-mounts both document and baseline edit windows; therefore their ui-contract imports require a normal dependency in this manifest. The actual codebase owner exists at framework/modules/ui/contract/packages/rust/Cargo.toml, named semio-framework-ui-contract. The locale crate does not replace ui::text and Label ownership. Adding the dependency only to the higher JPG root or baseline editor crate would not provide it in schema-2 external prelude.

The same schema-2 mirror mounts the entire document editor under editor::jpg_any, whose component defines pub(crate) paint_region. Its baseline namespace mounts only modes/windows, not the baseline editor component. The full baseline editor is mounted by the higher semio-s-artifact-stdio-jpg-io-registry crate, which already depends on schema-2 and reexports its windows. Mounting that editor into schema-2 would invert or duplicate this ownership and risk a cycle; the missing baseline symbol is real in this lower compilation context.

## Preserve Required Baseline Behavior

Baseline paint-region source exists and is typed to JpgBaselineEditor/JpgBaselineEditCommand with payload schema s.stdio.jpg.baseline.command.paint-region.v1. Document paint-region is typed to JpgAnyEditor and uses its distinct any payload schema. The compiler suggestion to use jpg_any is only name resolution advice; it does not establish the correct baseline command implementation. Do not substitute the Any work owner or remove the paint action.

The window needs only ActionDefinition metadata. A direct source comparison establishes that both integer_arg/action declaration blocks are byte-identical. Those declarations depend only on the common action id paint-region, maximum interactive row bound65536, owned plugin action contracts and localized labels; they do not need editor types or payload schemas. The clean lower seam is a standalone JPG-owned pure action declaration module available to schema-2, reused by both typed command implementations and windows. Keep baseline typed execution, admission/cancellation, payload schema, mutation route and localized re-encode notice intact. Native was sent this concrete owner recommendation.

Existing baseline window tests require paint-region as a Mutation action and render notices for English/German. Their presence helps identify preserved scope, but this audit did not execute them. Actual native rerun remains required after owner correction; no native acceptance pass is claimed.
