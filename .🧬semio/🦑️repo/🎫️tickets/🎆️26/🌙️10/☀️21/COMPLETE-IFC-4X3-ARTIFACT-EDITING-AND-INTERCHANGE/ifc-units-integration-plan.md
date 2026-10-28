# IFC Units Integration Map

Read-only current source exploration; no production edits.

## Existing boundaries

IFC4 geometry surface C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📐️geometry/🦀️.rs render_scene uses GeometryScene camera, meshes_json and instances_json in source coordinates. Standard2 🚪️io/📐️geometry mirrors it. Neither currently has physical measurement metadata or unit conversion UI. Existing shared GeometryGraph + LengthUnitWork now supply own-version scale without graph flattening.

Remote4x3 has distinct exchange infrastructure: C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry.ts IfcTriangleMesh explicitly promises SI metres/Z-up; IfcGeometryGraph.lengthScale requires exactlyoneproject and delegates dimensional unitMeasure. native_gltf.rs length_scale currently supports only SI metre with prefixes and refuses conversion-based units. ifc_to_gltf_controlled multiplies points by scale and changes Z-up toY-up as [x,z,-y]. TS gltf.ts ifc4x3ToGltf has the same explicit output contract. Reverse gltfToIfc4x3/createIfc4x3ExchangeGraph creates a new metre IFC project. Do not feed an already scaled SI mesh through the source-coordinate scaler again.

native_graph.rs Reader owns scale for BIM domain interchange; explicit property unit overrides are resolved separately from projectscale. native_fem_owned.rs analytical validation currently requires metre project coordinates (error around515). A simple shared lengthresolver replacement must not relax analytical dimensions/load/material constraints or silently rescale its authored FEM model.

No general physical distance/area/volume UI API was established by inspected framework Rust/plugin sources. World3d renders scene payload and selection; window_measures are window layout measures, not geometry quantities. Avoid reusing the latter name as evidence of physical measurement support. A focused IFC measurement readout therefore needs an explicit first-party typed quantity presentation contract, rather than assuming worldscene already interprets units.

## Minimal end-to-end slice

Resolve explicitly selected project's scale using the captured immutable source and retained preparation owner. Keep GeometryScene positions, placements and camera in sourcecoordinates. Add readonly localized IFC readout/metadata carrying projectID, declaredunitID, resolutionstatus, metresPerUnit and coordinateSpace=source. A distance computed in that same space converts d_m=d_source*s; area A_m2=A_source*s*s; volume V_m3=V_source*s*s*s, checking all intermediate/final finite values. Unresolved units show explicit unavailable physicalmeasurement; never label sourcevalues asmetres. For robustness with extreme scales, bounded Decimal or carefully ordered arithmetic policy belongs the authored numeric contract, not externalruntime dependency.

Use existing selectedproduct stableID and retained Details readout; do not invent persisted UI unitstate. Display preference for metre/mm/imperial can be ephemeral windowconfiguration only after explicit quantitydimension contract. Language comes from ViewModel locale (English/German labels), with no implicit defaultlanguage.

## Editing declared project units

Use actual typed Details referencefield for IfcProject.UnitsInContext index8 to choose an existing IfcUnitAssignment; navigate its Units collection and selected SIUnit Prefix2/Name3 or ConversionBasedUnit fields. Shared4/43 IfcDetailsProvider and controlled planner in /🔎️details/🦀️.rs emit SetEntityArg plus actual diff/inverse via retained snapshotfactory; own2 Details emits own UpsertInstance through same retained authority. Do not directly write inspectorstate or projectscale.

Changing unitdeclaration changes physicalinterpretation of existing coordinates; it does not numerically convert the model. Make that behavior explicit in label/readout. A future wholemodel unitconversion is a separate domaincommand touching all dimensional values, not an incidental prefixedit. Existing generic Details route may edit enum/name independently; resolver then reports invalid mismatcheddeclaredunit rather than pretending every intermediate state isvalid.

## Export integration

SPF/sqlite/nativepack export preserves authored snapshot and unitentities unchanged. Geometric export that promisesmetres (glTF) consumes s exactlyonce at its current boundary and records sourceproject/unit metadata as diagnostic/extras only where targetcontract permits. GeometryScene sourcepayload should declare its coordinateSpace without requiring target to infer it. If exporting sourcecoordinates to a target supporting explicit units, emit targetunitmetadata; if targetrequiresmetres, bake point/translations consistently and recomputebounds/camera in that targetspace. No snapshot/camera partialrescale.

Shared resolver currently supports IFC4+2x3 graphstandard only; remote4x3 needs its own borrowedgraphadapter/profile before reuse. Do not cast its snapshot toIFC4. Its existing rich dimensional unitMeasure handles derivedengineeringunits beyond length; factor out sharedlength logic without replacing those domains or losing overrides. A first integration can keep4x3 independent while adding paritytests for same physicalfacts.

## Mounted accounting and tests

LengthUnitWork currently limits logicalwork and scalarclose; native BTree backing allocation/release is not represented as retained physicalreceipts. Before mountedpreparation, supply firstparty funded index/sets or explicit originalowner storage producer, quote next capacity/copy/release demand, debit originalcx including errorpaths and hold sourcelease until cursor and payload close. Use existing MountedSnapshotReadPreparation grant/record before leaseissue; no fresh unlimitedcontext. Cache scale with full sourceidentity/version/project and exact geometryoptions; invalidate after retainedunitmutation.

Test unchanged official inchcolumn andmmfixtures: source geometry/camera invariant, readout SI distances/areas/volumes scalecorrectly, geometricexport metrecoordinates correct once, SPF roundtrip unitrows unchanged, unitdeclarationedit changes metadata interpretation and undo restores it. Tinyoriginalgrants/cancel/staleversion/close tests must exercise actual mountedpath rather than helperalone. Native build/GUI execution not established.
