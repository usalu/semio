# IFC4 Complete Declaration Details Reuse Plan

Read-only2026-10-21; no production edits. Main has now added official IFC4 catalog and common declaration contracts; prior catalog audit is historical. High owns full catalog-backed details execution and received direct interface recommendations.

## Current Ownership Facts

IFC4 🧬️schema/📚️catalog now retains unchanged IFC4.exp,source.json,LICENSE.md,JSON,Rust data and tests. IFC common 📚️catalog/{🦀️.rs,🟦️.ts,🧬️schema} owns declaration source contracts and catalog-parameterized query functions. IFC4 facade exposes ifc4_catalog,ifc4_attribute_declarations returning borrowed static slice,ifc4_is_subtype,resolveType/enum/select queries. IFC4x3 catalog facade now reuses shared declarations but still returns Vec clones from ifc_attribute_declarations.

IFC4x3 editor and declaration provider consume EXACT same semio_s_artifact_stdio_ifc_4_model::...IfcSnapshot/IfcEntity/IfcValue/IfcComplexType, and EXACT same semio_s_artifact_stdio_ifc_4_mutations::...IfcMutation/edit_rules as IFC4 editor. There is no model conversion needed for shared details. Distinct standard schema identity remains in editor/factory/controller/codec/catalog selection. Do not rename/cast a4x3 artifact to4 to reuse controls.

Rust nominal type caveat: shared catalog source is mounted separately as v4catalog::shared and4x3catalog::declaration. These define nominally different IfcCatalog types despite same source file. Passing4catalog to a provider whose signature imports4x3catalog::IfcCatalog will fail. Preferred stable solution is one canonical first-party common declaration crate/export. An infrastructure-compatible short sharedsource mounting approach imports catalog aliases from its wrapper parent and is instantiated separately with the matching version catalog; do not accidentally combine independent mounted types. No external type leak/runtime dependency is required.

## Shared Provider Interface

Move rather than copy IFC4x3 ✏️editor/details.rs implementation into concise IFC common domain folder, with IfcDetailsProvider<'snapshot,'catalog> {snapshot:&IfcSnapshot,catalog:&IfcCatalog,raw:DslSnapshotDetailsProvider<IfcSnapshot>}. Constructor takes snapshot and explicit catalog. Resolve inherited attributes through catalog.entities[name].all_attributes; complex primary/additional constituents use direct declaration attributes. Convert all global ifc4x3_catalog()/ifc_is_subtype/ifc_attribute_declarations references to catalog-parameterized shared operations or provider methods. Helper resolved,accepts_entity,has_entity_type,select_members must receive catalog or self; no hidden4x3global may remain.

Use same public SnapshotDetailsProvider interface and render_snapshot_details_provider_revisioned. Keep raw fullsource via delegated Dsl provider. Existing capabilities already cover complete declaration fields: named inherited attribute presentation, derived readonly, enum/select variants, typed wrappers, optional templates, nested aggregates, bounded candidate references and exact decimal direct entry. REFERENCE_WINDOW256 is a UI paging cap, not a schema limit; direct exact-ID reference entry must still reach every admitted target.

Thin perstandard construction passes correct normative catalog; avoid a second handwritten schema map. Semantic inspector edit policy is separate: catalog declaration presence does not automatically authorize every semantic mutation. Continue existing semantic window and full structural details tab.

## Rendering Alone Is Insufficient

Existing4x3 normalized_reference_edit lives in editor/🦀️.rs lines24–58, outside details.rs. Shared provider emits virtual paths /reference and /appendReference; IFC4 generic edit_rules cannot interpret these by itself. Share a declaration-aware normalizer receiving provider,event,check_target,original tick, returning normalized persisted SnapshotEditEvent:

- scalar reference payload exact decimal text -> existing Reference u64 value;
- optional empty virtual reference -> Unset;
- appendReference -> InsertValue at parent/value/-;
- validate target exists and matches declared entity/select subtype;
- validate upper cardinality and SET/UNIQUE no duplicate target;
- reject malformed present values, unknown virtual paths and lossy Number conversion.

Admission must normalize only bounded shape/decimal data without scanning entire graph; target lookup/type/set validation belongs contextual reduction under originalcx. Current4x3 admission passes check_target=false but still constructs provider/context; preserve budget bounds on that work rather than advertise all scans constant-time.

Other4x3 root special paths are equally necessary for advertised variants: wholeentity type switching, newentity identity/GlobalId preparation, direct complex constituent type switch, omitted emptycomplex initial insertion, and inherited/direct argument templates. Share these planning functions or disable those creation/type-switch controls until implemented; rendering attractive variants whose events fall through generic entity-kind resolver is incomplete.

Output remains existing IfcMutation (SetEntityArg,SetEntityName,SetEntityComplex,InsertEntity,etc); shared special route applies complex_edit first where required, then EDIT_RULES.resolve. Emit goes through unchanged shared retained publication/inverse limits. Do not add new direct state path. Keep semantic inspector specialpath first so /ifc-ID/field stays stable and does not become a persisted JSON pointer. Generic details keep /entities/index/... pointers with existing publication revision conflict handling.

Current4x3 fresh_global_id uses SystemTime+RandomState and collision set scans; reuse requires explicit authoring-seed/control consideration. Avoid copying nondeterministic implicit generator into a new common planner unexamined. Event publication must retain chosen generated id; replay applies chosen mutation, never regenerates identity. Exact remaining u64 id fallback needs cancellation if scanning a full population.

## Package Boundaries

Provider/UI depends on framework UI/plugin/editing contracts, model package and correct catalog; place it in surface/UI package boundary, not pure catalog/model package. Planner emits mutation package types; do not make pure model depend on mutations/UI. Current IFC4 io module inspector already sits in surface crate and can host full details adapter. Viewer must not import editor; if viewer wants readonly declaration fields, common provider carries editable capability flag independently of editor commands. Sharedcatalog contracts must not import version-specific snapshot/mutation types.

## Concrete Short Slice for High

1. Extract current full provider once with explicit catalog parameter; establish canonical declaration type path/source wrapper arrangement.
2. Replace4x3 provider instantiation and IFC4 full Details instantiation with shared provider+respectivecatalog; preserve IFC4 inspector/context hook and standard owners.
3. Extract reference edit normalization and full-details special event planner once, including omittedcomplex/type-switch/newentity paths, using original tick for snapshot scans.
4. Wire admission/contextual reduction to normalized events with existing IfcMutation and publication/inverse checks. Do not claim all EXPRESS validity; catalog-aware creation must still signal unsupported/rule-incomplete cases accurately.
5. Register permanent tests in bothstandardpackages and verify no4x3global leaks through commonprovider or4facade.

## Required Tests

Same generic provider fixture across catalogs; all776IFC4 declarations visible (declared abstract kinds may be readonly/typeconstituentonly), inherited slot labels/order, derived contextargs readonly, optional scalar creation/clear, enum choices, nested aggregate/itemtype/width templates, select primitive+entity alternatives, exactwideIDs, reference missing/wrongtype/optionalclear, SETduplicate and upperbound rejection, retained virtualpath admission, actual mutation+inverse identity, complex firstinsert/typechange, wholeentity typechange preservingid/overlappingattributes, unknown entity rawfallback, and4x3 smoke behavior unchanged. Use existing4x3 details/reference neutral corpus and test-only IfcOpenShell schema extraction as independent attribute/type oracle. UI assembly/native dispatch remains unexecuted until actual repo build works; grammar and TS source controls do not prove native builder/typecheck success.
