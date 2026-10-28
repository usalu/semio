# Template Application Through the Artifact Infrastructure

## Normative Attachment Distinctions

The pinned [ADD2 IfcPropertySetTemplateTypeEnum definition](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/IfcPropertySetTemplateTypeEnum.htm) distinguishes the following carriers. The convenient current documentation hostname redirects to a development build, so this plan uses the immutable release source.

| Released template kind | Required attachment model |
| --- | --- |
| Material driven | IfcMaterialProperties attached to IfcMaterialDefinition |
| Profile driven | IfcProfileProperties attached to IfcProfileDef |
| Performance driven | Property set attached to IfcPerformanceHistory, which controls the applicable object through IfcRelAssignsToControl |
| Occurrence driven property/quantity | Subtype of IfcObject |
| Type driven override property/quantity | IfcTypeObject or an overriding set at IfcObject |

The published archive uses seven template kinds; the general EXPRESS enum additionally supports type-only and unspecified custom templates. The released-catalog API deliberately describes the published archive while the complete schema editor still admits the general IFC template entities.

## Source-Owned Command Contract

Application must be a domain command yielding an admitted sparse diff/transaction, with exact inverse and revision/event publication. The catalog alone must not insert data. Use one real IFC graph; reuse schema attribute lookup and existing value/identity rules. Native UI controls must use the retained request's funded cursor and cancellation, including all large value copying and inverse preparation. Plain successful snapshot return is insufficient evidence of event publication.

Choose selected entries explicitly rather than instantiate every optional/source property, including deprecated source notes. Validate each selected entry against its released form: single/reference/enumerated/list/bounded/table or one of six quantity types. Preserve exact integer/binary64 payloads, named enum identity/literals, explicit units and table dimensions. Quantity domains come from actual IfcQuantity* value declarations. Required identities and GlobalIds must be retained in a deterministic command/event or diff; repeated planning/inversion must not generate different identities.

Attachment must distinguish real type HasPropertySets, occurrence relationships, material/profile property carriers and performance history. Applicability selectors and required predefined types must be satisfied explicitly; do not attach performance properties directly to the controlled element merely because it appears in the template's applicability list. Reuse canonical named enumeration definitions only when their complete values/units agree. Conflicting or unsupported attachments must produce actionable diagnostics before publication.

## Test-First Acceptance

Use neutral scenarios covering every property and quantity form, all seven published attachment kinds, subtype/predefined applicability, explicit units, partial selection, duplicate/conflicting definitions, exact IDs, undo/redo, stale revisions and cancellation at actual work frontiers. Independently author equivalent models with IfcOpenShell's native property APIs and validate full emitted STEP with schema and EXPRESS rules. Confirm values and relationships after reimport. Native tests must exercise actual retained job capacity/extent, command reduction, central diff/inverse and publication; direct resolver calls alone do not prove host end-to-end behavior.

Metadata infrastructure is complete as described in `property-set-implementation.md`. This command/UI stage remains open goal work and must not be advertised as implemented by the catalog.
