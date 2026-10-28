# Next Released MVD Clause Research

Choose **explicit Body SweptSolid Geometry representation validation** next. It directly checks supported rectangle/circle/polyline extrusion output and original official geometry, while requiring no new mesh generator or whole-view architecture. Implement its three representation clauses as one selected concept, reusing the existing controlled Body Tessellation source/query/membership pattern. A successful concept result must remain separate from schema, geometry evaluation and complete view conformance.

## Authenticated release and applicability

Pinned primary [released concept](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SweptSolid_Geometry/content.html), IFC4.3.2.0/IFC4X3_ADD2, commit `610ee3f95db9d0380b1eb8de4dc6e17833531518`, SHA256 `c571f3ac400b669781c09cc666b5cd55031b6a480333df6ac47d8ebb1f8dd7c0`,59143bytes. The captured file is `generated/mvd-next/swept-solid.html`; source/oracle provenance is `generated/mvd-next/source-oracle.json`.

Released lines117–122 constrain the selected representation to Body identifier, SweptSolid type, and extruded/revolved area-solid item classes. Its diagram uses IfcElement and general usage table lists IfcElement; the accompanying usage explanation is recommended-pattern context. It does not require all elements or bodies to choose this representation. Explicit caller applicability is therefore the bounded contract. The generic diagram's broader IfcSweptAreaSolid node must not silently broaden the named prose classes to every swept-solid family.

The pinned local occurrence catalog contains eight resource-entity/view rows for this concept (four entity names across both displayed views), while the concept record itself is non-universal with empty view-use arrays. `generated/mvd-next/occurrences.json` records this census. These are documentation occurrences, not executable requirements or automatic occurrence selection. Historical DEV/sample mvdXML remains discovery-only per the earlier machine-readable-source audit.

## Independent original examples

Authored input `mvd-next-clause-oracle.py` accesses only the immutable buildingSMART source and existing IfcOpenShell0.9.0.post1 interface. It reads original corpus files in place; no source copies. Actual exact ADD2 headers and item inheritance observations:

- `extruded-solid.ifc`, SHA256 `c488f0367106a17289468409531b1b687f72c25cea8e1b46924f59972ad7a8e9`: representation1020, Body/SweptSolid, item1021 IfcExtrudedAreaSolid.
- `slab-openings.ifc`, SHA256 `958350475139ad5ee7bfdf1181c259c9f188c977302233c10dbe5644b5843a90`: representations314/323/334, each Body/SweptSolid with extruded items313/322/333.

All observed items satisfy the named class membership independently. This oracle observes representation values/classes, not a fresh full EXPRESS run. It does not certify relational opening subtraction; that exchange limitation remains diagnosed. Neither meshing support nor a generated GLB determines this concept verdict. A revolved solid can satisfy the representation clause even while its meshing route is unsupported.

## Concrete schema-first implementation slice

Request selected representation IDs plus explicit applicability (true/false/unresolved), using the established exact-u64 and closed request/report conventions. Resolve the actual IfcShapeRepresentation, then owner-qualified IFCREPRESENTATION RepresentationIdentifier/RepresentationType/Items with full shared query status. Preserve missing/ambiguous/late-unknown paths and actual source pointers; do not scan an entire source once per target. Check named item-role membership through released catalog ancestry, retaining unknown after positive matches. Numeric/profile/depth/extrusion validity belongs to separate schema/geometry checks.

Use one neutral matrix: original1020; extruded, revolved and mixed allowed items; wrong identifier; wrong representation type; unrelated tessellated item; mixed allowed/foreign item; missing/dangling/duplicate target; missing/ambiguous item reference; unknown item or complex constituent; foreign declaring-owner conflict; duplicate actual owner; explicit nonapplicability; unresolved applicability; original signal/callback/limit capture; initial/interior/completion cancellation. Preserve source aliases privately and never publish a partial successful report after terminal cancellation. Confirm accepted descendants, including tapered named subclasses, against the exact released IfcShapeRepresentationTypes rule before freezing their expected classification rather than inferring an exact-class exclusion from the word basic.

Schema-defined nonempty Items and complete physical-instance validity remain separately reported; do not invent a fourth concept predicate. The scope is one complete explicit three-clause representation check, not complete Body geometry, ReferenceView, Alignment-basedView or all-MVD conformance. No production files were edited, no runtime dependency installed, and no native execution or capability claim is made.

## Exact released tapered exclusion and independent physical classification

Pinned local `catalog/IFC4X3_ADD2.exp` SHA256 is `f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860`. In IfcShapeRepresentationTypes, lines13582–13592 require the named extruded/revolved ancestry and **exclude both tapered ancestry branches** for SweptSolid. Lines13594 onward accept SweptAreaSolid, SweptDiskSolid or SectionedSolidHorizontal ancestry for AdvancedSweptSolid. Tapered entity declarations occur at6598 and10108. A generic positive superclass check alone is insufficient for the proposed SweptSolid predicate.

The frozen concept should therefore require both an allowed extruded/revolved role and absence of either tapered role; unknown membership cannot establish the exclusion. Other AdvancedSweptSolid families are not swept into the selected basic concept. A representation labelled AdvancedSweptSolid is a type-clause failure for this explicitly selected Body SweptSolid concept, even when it is independently valid IFC under that other classification.

`mvd-swept-tapered-oracle.py` creates eight physical variants through a test-tool-independent interface, retaining the original official extruded-solid graph as base. It changes the selected representation to actual basic/tapered extruded/revolved item objects, with actual required inherited fields/axis/end profile, and runs IfcOpenShell0.9.0.post1 full EXPRESS validation. `generated/mvd-next/tapered-oracle.json` retains all diagnostics and inheritance observations; original source SHA is unchanged.

Observed matrix: basic extruded and basic revolved are full-EXPRESS-valid under both SweptSolid and AdvancedSweptSolid. Both tapered classes are full-EXPRESS-valid under AdvancedSweptSolid. Under SweptSolid each tapered variant has exactly one diagnostic: IfcShapeRepresentation.CorrectItemsForType. This independently corroborates the explicit released exclusion, rather than relying on class hierarchy or diagram interpretation. All eight physical variants remain generated test output; no original fixture or production source was edited. No meshing or native execution is claimed. The Python process exited normally.

## Tessellation extraction baseline

Before root's shared Body executor extraction, four current Tessellation source hashes (TS endpoint/types/definition/schema) are frozen in `generated/tess-shared-body-before-hashes.json`. Authored `tess-shared-body-before.ts` invokes the actual public endpoint on all nine original independent physical representation IDs from the existing oracle. Complete reports, requests and prior oracle verdicts are preserved in exclusive `generated/tess-shared-body-before-reports.json`; all nine match. Source pointers/diagnostic paths are retained for exact after comparison, not merely verdict totals. Original graph and before outputs remain unchanged. Root was notified that all probe processes exited and files were free for production edits.

The next definition must separately identify released HTML three-clause provenance and the actual pinned EXPRESS exclusion policy, keeping explicit applicability. This proposed common executor refactor has no after proof until root supplies a frozen checkpoint and the exact preserved reports are compared.
