# Actual Details Contextual Control Audit

## Scope And Result

Read-only review of mounted IFC4x3 editor/details/action source after reading details-implementation.md, host-template-form-integration.md, native-template-host-independent-audit.md and staged-template-form-plan.md. No UI/native interaction, compiler execution or source edit occurred. The semantic worker confirms staged-form source is untouched while aggregate fixes take priority; the details worker owns reference reduction. This audit therefore identifies a concrete remaining control affordance and integration interfaces, not a new form implementation or blanket completion approval.

## P2 Completeness Gap — Resolve Actual Property Enumeration Choices

The shared mounted provider in IFC details/🦀️.rs enum_values (approximately lines 1036-1050) immediately returns no choices unless the current payload is IfcValue::Enum. It then consults only static EXPRESS enum declarations or BOOLEAN/LOGICAL. alternatives (929-971) provides generic SELECT leaf templates and reference suggestions. IFC4x3's profile detail_variants override handles only MVD header text, not contextual property enumerations.

IfcPropertyEnumeratedValue.EnumerationValues is a LIST OF IfcValue, typically containing typed IfcLabel/String or numeric values. Its optional EnumerationReference points to an actual IfcPropertyEnumeration whose EnumerationValues supplies the admissible graph values. Pinned IFC4X3_ADD2.exp's IfcPropertyEnumeratedValue WR21 uses that reference's actual values; static PSD suggestions are not a substitute for a model-authored enumeration. No resolver for this relation exists in the reviewed mounted provider or TS detailsControl/declarationControl.

Concrete authored helper fixture: property #1 selects IfcLabel('Red'), refers to enumeration #2 containing IfcLabel('Red') and IfcLabel('Blue'). At /entities/0/args/2/value/0 the TS helper returns zero enumValues and 109 generic IfcValue SELECT choices; the nested label payload also has zero enumValues. Executed ticket input details-context-probe.ts records this helper observation in 🗑️generated/details-context-probe.json with nativeExecution=false/uiInteraction=false. It is not a valid full IFC document fixture or a rendered interaction proof. Native source predicts the same missing choices.

Implement a shared source-revision-bound contextual choices query: resolve the exact selected property's reference, preserve every actual typed value/tag, exact bigint/binary64 payload and ordering, then expose choices at the complete IfcValue item path. Missing/unset/stale enumeration references need explicit unavailable/repair behavior. Query work must use original progress/cancellation and complete addressing; do not synchronously scan an arbitrary graph during render or truncate actual enumeration values to a suggestion window. Optional values and current out-of-domain values must remain distinguishable from nonexistent options.

This is a missing requested affordance, not proof of a publication bypass. Existing details edits intentionally allow progressive repair and do not claim full WHERE conformance after each small edit. This audit did not turn that scope into a hidden data-loss finding.

## Actual Renderer And Event Route

IFC4x3AnyEditor render mounts render_snapshot_details_provider_revisioned with the shared provider. The details tree calls provider.enum_values and item controls; scalar enum_control emits the existing SET_SNAPSHOT_VALUE_ACTION_ID path binding, while full typed variants use existing template/variant controls. Editor action decoding routes snapshot edits into its retained job/provider domain path and central mutation publication. Existing declared enums, optional unset/create, nested aggregates, SELECT wrappers and exact reference input are present source branches; no new regression was established in them.

The shared neutral ComponentTree is the appropriate native/web control boundary. A contextual IfcValue choice can require replacing a typed value, not merely displaying its string payload: two distinct allowed values may have different named domains or exact numeric words. Avoid implementing the choice solely in an IFC React component or flattening all values into strings. The TS helper's current enumValues:string[] model is insufficient as the sole full typed contextual-choice contract; a shared typed value/label/path description is needed. Native enum_control already has a nonstring variant path, but source-owned contextual query generation and same-schema renderer/event tests remain open.

## Actual Template Action Staging Gap

apply/factory.rs action() exposes the real released-template select and text target, but entries/identities/ownerHistory/performance remain ActionArgDef::any fields. The React buildActionCategoryTree renders only expandedAction.args through renderStagedArgControl and executes one ActionDescriptor with effective staged args. There is no selected-declaration-dependent nested field producer or reset hook there today. This confirms the staged-template plan's open interface; it does not invent a second document draft or bypass the existing ApplyTemplate receiver/planner/refusal gate.

The future neutral producer must distinguish static released PSD literals from actual source-referenced enumeration definitions, preserve readonly kind/name discriminants, unit/reference targets, independent table-axis domains and exact identities, reset incompatible staged entries when the template changes, and reject stale generation/revision query responses. Both native and React action panes must consume the same schema/description/events. Current per-window shell staging and final Execute authority should remain the owner; no form event should directly mutate the graph or manufacture semantic admission.

## Required Evidence Before Closing This Gap

Add neutral and TS/native provider tests for actual referenced label/numeric/mixed-domain enumeration choices, absent/wrong-kind/missing references, stale source revision, deep aggregate paths and exact value replacement events. Add actual renderer staging/Execute tests in both targets, including labels/keyboard/error association and cancellation of expensive graph queries. Existing static enum/control metadata or PSD oracle matches do not prove contextual graph choices, and authored native tests remain unexecuted until authentic ingress runs. Retained TYPEOF normative uncertainty and template.semantic-admission-unavailable remain explicit separate gates.
