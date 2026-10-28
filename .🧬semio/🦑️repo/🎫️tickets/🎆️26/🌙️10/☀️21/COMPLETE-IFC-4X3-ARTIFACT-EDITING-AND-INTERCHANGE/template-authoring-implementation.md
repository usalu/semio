# Selected Template Authoring

The TypeScript IFC4X3_ADD2 graph now accepts an actual `applyTemplate` domain command. This adds selected published entries and the correct carrier relationships through the existing owned mutation capture, sparse diff planner, central transaction/inverse and revision-checked event history. The metadata catalog remains unchanged. Native command reduction and UI integration remain subsequent work; no native execution or complete-specification flag was changed.

## Schema and Command Contract

The closed schema is `🧬️schema/🏷️templates/✍️apply/🔣️.json`, identified by `https://semio.tech/s/stdio/ifc/4x3/template-application.schema.json`. Native TypeScript entity IDs, selected integer values and Binary64 bits are bigint; decimal JSON transport follows the existing snapshot schema. `types.ts` explicitly owns all public types.

`{ mutation: "applyTemplate", application: { template, target, entries, identities, ownerHistory?, description?, methodOfMeasurement?, performance? } }` is one branch of the existing finite mutation union. Each selected entry supplies its exact published name and form. Leaf values are existing tagged `IfcValue` data, and all optional units are canonical `{kind:"reference",value:bigint}` references. Tables supply separate defining/defined columns and units, optional expression and interpolation. Quantities support explicit formula and description. Performance history requires retained `lifeCyclePhase` and `predefinedType`, with ObjectType when USERDEFINED; lifecycle is never generated or defaulted.

Public `ifcTemplateIdentityKeys` and `ifcTemplateIdentityKeysAsync` use the same controlled source/selection/enumeration planner as application. Exact key order is selected `entry:<index>` followed by any newly needed `enum:<index>` at that entry, then `set`, `relation` for occurrence attachments, and `performance`/`control` for performance attachments. Material/profile and type attachments omit `relation`. New enum requirements are determined against the original owned graph. Every identity supplies `{key,id,guid?}`; only actual IfcRoot instances require/accept GlobalId. The command rejects missing, extra, duplicated or colliding IDs and GlobalIds. Nothing random is generated during planning, inversion or replay.

`{mutation:"applyDiff",diff}` is a closed, nonrecursive generic inverse command. It captures its own sparse diff, validates all ordinary graph/coordinate/overlap invariants, and passes through the same central applier. This restores newly created sets, properties, enumeration references, relationships and type HasPropertySets together. History events retain the complete owned application and exact diff/inverse.

Template publication is asynchronous: `applyIfcMutationAsync` and `IfcEditHistory.executeAsync` perform full graph EXPRESS admission after the owned central transaction is prepared, then check cancellation and the expected revision immediately before publication. Synchronous template publication returns `asyncRequired`; it cannot bypass the async semantic evaluator. Identity/diff planning and generic inverse computation remain available synchronously. A result proven schema-invalid returns `invalidDomain`; incomplete semantic execution returns `validationIncomplete`; cancellation retains its `cancelled` classification. Thus existing schema-invalid graphs require repair before template application.

## Actual Carriers and Leaves

| Released category | Authored graph |
| --- | --- |
| PSET_OCCURRENCEDRIVEN | IfcPropertySet → IfcRelDefinesByProperties → applicable IfcObject |
| PSET_TYPEDRIVENOVERRIDE | IfcPropertySet in type HasPropertySets, or an occurrence relationship override |
| PSET_PERFORMANCEDRIVEN | IfcPropertySet → performance history through IfcRelDefinesByProperties; history → applicable target through IfcRelAssignsToControl |
| PSET_MATERIALDRIVEN | IfcMaterialProperties.Material |
| PSET_PROFILEDRIVEN | IfcProfileProperties.ProfileDefinition |
| QTO_OCCURRENCEDRIVEN | IfcElementQuantity → occurrence relationship |
| QTO_TYPEDRIVENOVERRIDE | IfcElementQuantity in type HasPropertySets, or an occurrence override |

Six property forms create actual IfcPropertySingleValue, IfcPropertyReferenceValue, IfcPropertyEnumeratedValue, IfcPropertyListValue, IfcPropertyBoundedValue and IfcPropertyTableValue. Six quantity forms create IfcQuantityLength/Area/Volume/Weight/Count/Time. Selection is explicit; unselected/deprecated source properties are not automatically materialized. Existing same-template attachments are refused as conflicts rather than silently duplicated or edited.

Selected scalar values are explicitly tagged with the released metadata domain; quantity attributes retain their actual declared simple domain. Full retained EXPRESS execution checks positive and constrained measures, exact integer counts, widths, table cardinality/uniqueness, reference domains and all applicable model rules. Additional authoring validation preserves ordered numeric bounds and a set point within them without rounding bigint values. Physical unit checks resolve measure aliases to released UnitType vocabulary; quantity units must be IfcNamedUnit. Ratio/count/numeric measures require explicit dimensionless named units. Generic simple domains admit the actual declared IfcUnit select. Complete named enumeration literals and their source ordering are authored; reuse requires the complete typed values and exact unit reference. A conflicting named definition is refused.

The carrier distinctions follow the pinned [ADD2 template category definition](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/IfcPropertySetTemplateTypeEnum.htm). Enumeration identity and unit override behavior follow the pinned [IfcPropertyEnumeration definition](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/IfcPropertyEnumeration.htm). Source archive provenance and all 760/3988 metadata comparisons remain in `property-set-implementation.md`.

## Controls and Admission

Capture owns only plain object/array containers and own data properties, refusing prototypes, accessors, cycles, unknown fields and bounded-depth/count violations. The finite command schema discriminates each entry's allowed fields. Actual nested graph leaves also pass normal graph value admission. Generic diff admission retains all overlapping-target, ID, reference and coordinate checks.

The generator yields while capturing, indexing source entities, scanning existing attachments, collecting reference targets, comparing enumeration definitions, processing selected values, constructing real entities and preparing the inverse. This shares the same caller signal and limits as central application. `validateTemplate` progress describes actual schema compilation/admission/evaluation work. There is no unbounded identity prepass, generated default identity, separate DTO state store, intermediate publication or artificial progress loop.

## Test-First and Independent Evidence

The application schema, neutral `cases.json`, test file and initial failing import preceded implementation. Neutral behavior is retained in `✍️apply/🧪️.feature`. `template-apply.test.ts` and the existing central `🧬️editing/shape.test.ts` pass **34 tests / 4,990 assertions**. This includes all seven carriers, all twelve forms, type quantity attachment, subtype/predefined selection, exact integers above 2^53, explicit units, measurement method/formula/description, domains, table constraints, enumeration reuse/conflicts, unknown/prototype/accessor refusal, inverse, undo/redo, cancellation at actual capture/planning/semantic frontiers and a real revision race during semantic work. Targeted strict TypeScript checks pass for both suites.

Retained ticket inputs `template-authoring.ts` and `template-authoring-oracle.py` generated and independently authored **15 models** covering all seven carriers and all twelve forms. IfcOpenShell **0.8.3** native `pset.add_pset`, `pset.add_qto` and `control.assign_control` APIs choose the carrier/relationship route; native entity constructors supply forms not supported by its higher-level editing API. Complete normalized entity populations agree **15/15**. Both the semio and independent native models pass `express_rules=True` with **zero diagnostics in every case (30 validated models)**. The generated STEP sources and native results are under `🗑️generated/template-authoring`; the source snapshots use actual valid STEP headers. Scripts remain reproducible input assets after generated output cleanup.

The cached repository executable exposes no general Bun/Python/compiler execution route, so direct Bun, TypeScript and bundled Python execution were the necessary fallback, as previously documented in the ticket. A broader facade-only strict compiler invocation reaches an unrelated unresolved workspace import `@semio-tech/framework-interaction` in framework replication; no bypass or build-system edit was introduced. Native Rust execution remains blocked by the existing authentic compiler ingress, and no Rust execution is claimed.

## Remaining Work and Explicit Limits

- Native template mutation reduction, job controls, details UI and native oracle/execution remain open. This report is a clean TypeScript implementation point for independent review, not completion of the full IFC goal.
- Existing complex **target** instances are explicitly refused; simple occurrence/type/material/profile targets and referenced legal complex resources are admitted through normal full-model semantics. Complex target mutation needs direct constituent coordinate support before it can be claimed.
- Explicit physical units currently require the canonical released UnitType matching their measure; arbitrary user-defined derived-unit dimension equivalence is not independently implemented. Dimensionless explicit units currently require named units with explicit zero exponents. No claim is made that every semantically equivalent custom unit encoding is accepted.
- Model-level EXPRESS evaluator limitations documented in `express-fixes.md` remain limitations here and refuse publication when evaluation is incomplete. No metadata/support flag claims their absence.

## Changed Files

New: `🏷️templates/✍️apply/{types.ts,🔣️.json,🟦️.ts,cases.json,🧪️.feature}`, `🧪️tests/{template-apply.test.ts,template-fixtures.ts}`, and the two retained ticket oracle inputs plus this report.

Updated: new IFC4x3 facade `🟦️.ts`, central `🔺️diff/🟦️.ts` capture/progress, `native-shape.ts` schema documents, `🧬️mutations/{🟦️.ts,🔣️.json}` command/inverse/history paths, and adjacent `🧪️tests/🧬️editing/shape.test.ts` schema registration. The existing IFC TypeScript package script registers template-apply.test.ts alongside the earlier suites. No old IFC4 code, native provider, AGENTS file, Git state, build system, worktree or external runtime dependency was changed. New authoring file paths remain below the 256-character limit.
