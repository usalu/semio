# Shared IFC4/IFC4x3 Details Constraint Audit

Read-only source review of shared Details native/TS and retained custom native edits. Main owns4/4x3 SET fix; High owns2x3 equivalent. No production edits or native execution.

## SET/UNIQUE References

Shared TS referenceMutation and native accepts_reference_append_controlled check duplicate IDs only for appendReference. Replacement of an existing SET/UNIQUE element can introduce a duplicate. Minimal IFC4/4x3 owned graph: IFCUNITASSIGNMENT.Units=[#2,#3], both targets IFCSIUNIT; replace first element through /entities/0/args/0/value/0/reference with decimal3. Declared SET rule requires uniqueness; unchanged self-reference replacement is legal. Same exact own2x3 repro was executed in ifc2x3-details-peer-review.md. Root/High already confirmed shared4gap, so this audit did not duplicate production edits.

Validate every enclosing aggregate that declares SET or UNIQUE, not only immediate virtual field. Catalog nested references include IFCBSPLINESURFACE.ControlPointsList LIST OF LIST OF IfcCartesianPoint (ordinary LIST permits repeated points). Do not impose uniqueness on all reference lists. Nested scalar unique example IFCINDEXEDPOLYGONALFACEWITHVOIDS.InnerCoordIndices is LIST[1:?] OF LIST[3:?] OF UNIQUE IfcPositiveInteger. Its uniqueness applies within each inner list; equal numbers across different rings are not prohibited by this declaration alone.

## Concrete Native Typed Reference Bypass

normalized_reference_edit_controlled recognizes direct reference payload paths such as /entities/0/args/5/value. If incoming value is DslValue::Number rather than decimal String, it currently returns Ok(None) for nonvirtual paths. Generic native_argument_edit then accepts as_u64 and emits Reference without target/existence/domain checks. Numeric0, missing ID or wrong entity type can therefore bypass typed validation at this recognized path. Source evidence also exists in own2x3 normalized_reference_edit followed by generic edited_subtree. UI renders reference payload as decimal text, but retained raw event admission must enforce the same rule. Refuse malformed type at any recognized typed reference payload, or parse exact u64 into the same domain-validation route; do not fall through to raw mutation planning.

## Derived and Enum Admission Gap

TS referenceMutation explicitly rejects found.attribute.derived. Native provider item_capabilities disables derived edits and supplies enum choices, but generic details_edit_is_admitted falls back to shape-based snapshot_edit_value_is_admitted. Custom native_argument_edit parses tagged IfcValue and scalar payload type without consulting derived/catalog enum constraints.

Concrete paths: whole argument /entities/0/args/2 for IFCGEOMETRICREPRESENTATIONSUBCONTEXT.CoordinateSpaceDimension can replace Derived marker with an integer through shape-valid raw event; enum payload /entities/0/args/8/value on IFCWALL can accept arbitrary string outside IfcWallTypeEnum. Nested unique integer payload can change InnerCoordIndices[1,2,3] to[2,2,3]. These are source-derived admission gaps, not executed native observations. Treat provider UI controls as presentation protection until retained event planner enforces them. Do not describe full EXPRESS validator completeness: only require declaration constraints that these exact typed controls promise.

## Scope and Control

Reference target scans use original controlled callbacks and inspect complex constituents; exact decimal parsing retains positive u64 IDs. Native catalog lifetimes remain valid, using static profile declaration storage and original borrowed graph values. Generic whole-graph flattening is not required for any proposed constraint: resolve one declared slot and its enclosing aggregates, validate/clone only owned argument or constituent with existing retained cursor, then emit existing mutations through original Store authority. Existing whole-instance/complex fallback gates must stay explicit rather than being represented as fully resumable arbitrary graphs.

No additional native borrow/type flaw identified in reviewed reference helper signatures. Main/High own fixes and subsequent actual-source regressions.

## Fresh Reference Fix Review and Correction

Main added immediate enclosing SET/UNIQUE replacement scan excluding selected slot, controlled target checks for whole reference/unset wrappers, and refusal of every recognized nonstring reference payload. TS/runtime13group evidence is author-reported; native actual branch regressions remain authored/unexecuted. Fresh native context traversal requires each suffix segment value and does not consume virtual reference. My initial transient claim that the new whole-slot branch also matched virtual paths incorrectly transferred TS/own2 context behavior to native4; that claim is retracted. Exact native4 source does not demonstrate that regression. Main is adding an explicit terminalIndex structural guard as hardening. Broad derived/scalar/enum admission remains the separately planned next gate.
