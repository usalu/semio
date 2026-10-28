# IFC Facts for Six BIM Teaching Clips on Villa Savoye

Research date: 2026-10-27. Read-only research; no repository file was changed except this report.

## 0. How to read this report (verification status)

The buildingSMART primary pages could not be fetched in this session. WebFetch and curl both received HTTP 403 from `standards.buildingsmart.org` and `ifc43-docs.standards.buildingsmart.org` (the ifc43-docs host 301-redirects to the standards host). The EXPRESS schema on GitHub was also not reachable. Facts therefore come from three tiers, marked on every item:

- **[P]** Stated in a buildingSMART page excerpt returned by search (URL given). Still open the page before narrating an exact name.
- **[D]** Secondary or vendor source (tool docs, vendor help, national guides, blogs). Use for context only.
- **[M]** My own knowledge of IFC, not verified in this session. Confirm against the official schema before a clip uses it.

Any statement not tagged [P] should be checked against the official IFC 4.3.2.0 (IFC4X3_ADD2) documentation before publication.

## 1. Version and naming

- **[P]** IFC 4.3 was approved as an ISO standard and published as **ISO 16739-1:2024** (buildingSMART announcements: https://www.buildingsmart.org/?p=29521 and https://www.buildingsmart.org/ifc-4-3-approved-as-a-final-standard/). The current official release is **IFC 4.3.2.0**, with schema identifier **IFC4X3_ADD2** (https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcWall.htm; forum confirmation of the identifier: https://forums.buildingsmart.org/t/file-schema-ifc4x3-add2-or-ifc4x3/5485).
- **[P]** Many ifc43-docs pages are labelled "under development" with build stamps (20250327, 20251201, 20260310, 20260907). Do not quote those stamps as the released standard. Use the official 4.3.2.0 pages under `standards.buildingsmart.org/IFC/RELEASE/IFC4_3/`.
- **[P] / [M]** Name changes relevant for the clips:
  - IFC 2x3 and IFC4: `IfcBuildingElement` is the abstract-style supertype for walls, slabs, columns and the rest. IFC 4.3 uses **`IfcBuiltElement`** for the same role (https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcBuiltElement.htm). The IFC4 page https://standards.buildingsmart.org/IFC/RELEASE/IFC4/FINAL/HTML/schema/ifcproductextension/lexical/ifcbuildingelement.htm shows the older name.
  - [M] In IFC4 `IfcBuildingElement` is a SUPERTYPE OF ONEOF(...) with a fixed subtype list. In 4.3 `IfcBuiltElement` is a generalisation that can be instantiated when no subtype fits. The 4.3 page excerpt says this, but verify the ABSTRACT/ONEOF detail.
  - [M] `IfcBuildingElementProxy` (IFC4) was renamed/restructured in 4.3. Do not use it in clips.
  - [M] Enum values changed between releases (for example columns and roofs). Use the 4.3.2.0 enum tables only.
- **Teaching rule:** Use IFC 4.3 names (`IfcBuiltElement`, schema `IFC4X3_ADD2`). Mention IFC4/IFC2x3 only when a student's software exports the older name.

## 2. Inheritance chains (IFC 4.3)

Chain shared by all building elements:

`IfcRoot -> IfcObjectDefinition -> IfcObject -> IfcProduct -> IfcElement -> IfcBuiltElement -> <subtype>`

- **[P]** The IFC 4.3 page for IfcBuiltElement lists supertypes IfcRoot, IfcObjectDefinition, IfcObject, IfcProduct, IfcElement (https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcBuiltElement.htm).
- **[P]** IfcWall and IfcSlab use the same chain; the IfcSlab page lists supertypes IfcRoot, IfcObjectDefinition, IfcObject, IfcProduct, IfcElement (https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcSlab.htm).

| Entity (IFC 4.3) | Direct supertype | Status | Notes |
|---|---|---|---|
| IfcColumn | IfcBuiltElement | [M] direct supertype | Adds PredefinedType (IfcColumnTypeEnum) |
| IfcWall | IfcBuiltElement | [M] | Adds PredefinedType (IfcWallTypeEnum) |
| IfcSlab | IfcBuiltElement | [M] | Adds PredefinedType (IfcSlabTypeEnum) |
| IfcWindow | IfcBuiltElement | [M] | Overall size attributes; partitioning is on the type |
| IfcRamp | IfcBuiltElement | [M] | Adds PredefinedType (IfcRampTypeEnum). Flights are separate IfcRampFlight elements |
| IfcRoof | IfcBuiltElement | [M] | Adds PredefinedType (IfcRoofTypeEnum) |
| IfcBeam | IfcBuiltElement | [M] | Adds PredefinedType (IfcBeamTypeEnum) |
| IfcSpace | IfcSpatialStructureElement | [M] | NOT a building element. Spatial branch: IfcSpatialElement -> IfcSpatialStructureElement -> IfcSpace |

- **[M]** IfcSpatialElement is a subtype of IfcProduct in IFC 4.3 (verify). IfcSpatialStructureElement adds CompositionType and LongName (verify). IfcSpace is in the spatial branch, which is why it is not in the element tree. Confirm with https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcSpace.htm.
- **[M]** IfcColumn page: https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcColumn.htm. Its direct supertype should be IfcBuiltElement in 4.3. Verify.

### Attributes added per level

| Level | Attributes it adds | Confidence |
|---|---|---|
| IfcRoot | GlobalId, OwnerHistory, Name, Description | [M] (standard for all IFC4/4.3 roots) |
| IfcObjectDefinition | none explicit (inverses only) | [M] |
| IfcObject | ObjectType | [M] |
| IfcProduct | ObjectPlacement, Representation | [M] |
| IfcElement | Tag | [M] |
| IfcBuiltElement | none explicit in the 4.3 page excerpt (inverses such as openings and fillings) | [P] excerpt lists inherited counts; verify explicit attributes |
| IfcColumn / IfcWall / IfcSlab / IfcRamp / IfcRoof / IfcBeam | PredefinedType (typed enum) | [M] |
| IfcWindow | OverallHeight, OverallWidth (and possibly PredefinedType; verify) | [M] |

Attribute order in STEP follows this inheritance order: GlobalId, OwnerHistory, Name, Description, ObjectType, ObjectPlacement, Representation, Tag, PredefinedType (see section 9).

## 3. Enumerations (PredefinedType)

### IfcColumnTypeEnum

- **[P]** COLUMN, PIERSTEM, PIERSTEM_SEGMENT, PILASTER, STANDCOLUMN, USERDEFINED, NOTDEFINED. Source: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcColumnTypeEnum.htm (via search excerpt).
- **[P]** PILASTER is a column set into a wall (may be load-bearing or decorative). PIERSTEM is an individual vertical part of a pier.
- **[M]** Villa Savoye pilotis map to **COLUMN**.

### IfcSlabTypeEnum

- **[P]** APPROACH_SLAB, BASESLAB, FLOOR, LANDING, PAVING, ROOF, SIDEWALK, TRACKSLAB, WEARING, USERDEFINED, NOTDEFINED. Source: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcSlabTypeEnum.htm. TRACKSLAB was added in ADD2.
- **[P]** BASESLAB is a floor slab against the ground (foundation). FLOOR covers floor slabs and bridge decks. ROOF covers flat or sloped roof slabs.
- **[M]** Villa Savoye intermediate floors map to FLOOR; the roof-garden slab maps to ROOF (or IfcRoof, see below).

### IfcRampTypeEnum

- **[P]** STRAIGHT_RUN_RAMP (one straight flight, no turns), TWO_STRAIGHT_RUN_RAMP (two straight flights with a landing, no turn), HALF_TURN_RAMP (180 degree turn via two flights and a halfspace landing). Source: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcRampTypeEnum.htm.
- **[M]** Other values (QUARTER_TURN_RAMP, SPIRAL_RAMP, USERDEFINED, NOTDEFINED) exist; verify.
- **Teaching caution:** Pick the value from the actual geometry of the modelled ramp, not from a photo. Check the Villa Savoye ramp run against plans before naming its type.

### IfcRoofTypeEnum

- **[D]** FLAT_ROOF, SHED_ROOF, GABLE_ROOF appear in IFC4-era documentation (https://docs.xbim.net/XbimDocs/documentation/Xbim.Ifc4/Xbim.Ifc4.Interfaces.IfcRoofTypeEnum.html) and a Japanese MLIT codelist. The enumerator is SHED_ROOF, not SHED.
- **[M]** Other values (HIP_ROOF, HIPPED_GABLE_ROOF, GAMBREL_ROOF, MANSARD_ROOF, BARREL_ROOF, RAINBOW_ROOF, BUTTERFLY_ROOF, PAVILION_ROOF, DOME_ROOF, FREEFORM, USERDEFINED, NOTDEFINED) exist in 4.3; verify exact list on the 4.3.2.0 page.
- **Villa Savoye:** roof garden is a flat roof, so FLAT_ROOF is the expected value [M].

### IfcWindowTypeEnum

- **[M]** WINDOW, SKYLIGHT, LIGHTDOME, USERDEFINED, NOTDEFINED (IFC4 list). Verify 4.3 additions.
- **[M]** Ribbon windows: each module is an IfcWindow (or a single IfcWindow with a ribbon-like size, depending on the model). Decide one convention and keep it.

## 4. Type vs occurrence; property sets; quantity sets

### Type and occurrence

- **[M]** An occurrence (IfcColumn) is placed in the building. A type (IfcColumnType) holds shared information. The link is IfcRelDefinesByType (relating type to occurrences). In 4.3 the type class is IfcColumnType; the type-level PredefinedType is separate from the occurrence.
- **[P]** Pset_ColumnCommon and related psets are the standard property sets for these entities.
- **Teaching point:** One type (for example "Pilotis round 0.50 m") can have many occurrences (pilotis P01 to P06). Changing the type changes all of them.

### Pset_ColumnCommon

- **[P]** LoadBearing: boolean; TRUE means the object is meant to carry loads (https://standards.buildingsmart.org/IFC/DEV/IFC4_4/HTML/lexical/Pset_ColumnCommon.html is IFC 4.4 dev; the IFC4 ADD2 version is https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2/HTML/schema/ifcsharedbldgelements/pset/pset_columncommon.htm).
- **[P]** FireRating: IfcLabel, national fire classification.
- **[P]** IsExternal: boolean; the element faces the outside of the building.
- **[P]** Slope: angle against horizontal, default 0 degrees. Geometry takes priority if they conflict (German text in the IFC4 ADD2 page).
- **[M]** Also Reference, Status, and fire/combustibility properties. Verify the full list in 4.3.2.0.

### Pset_WallCommon [M]

- Expected: Reference, Status, IsExternal, LoadBearing, FireRating, AcousticRating, ThermalTransmittance, ExtendToStructure, Compartmentation, Combustible, SurfaceSpreadOfFlame (verify all).
- Not confirmed by search in this session.

### Pset_SlabCommon

- **[P]** ThermalTransmittance (IfcThermalTransmittanceMeasure, U-value through the full path including all materials), LoadBearing, IsExternal (https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/Pset_SlabCommon.htm; the IFC4.4 dev version is https://standards.buildingsmart.org/IFC/DEV/IFC4_4/HTML/lexical/Pset_SlabCommon.html).
- **[M]** Also Reference, Status, FireRating, AcousticRating, Compartmentation.

### Pset_WindowCommon [M]

- Expected: Reference, Status, IsExternal, FireRating, AcousticRating, SecurityRating, ThermalTransmittance, GlazingAreaFraction, Infiltration (verify).
- Not confirmed by search.

### Pset_RampCommon

- **[D]** A tunnel-project IFC 4.3 page lists IsExternal, ThermalTransmittance, LoadBearing, FireRating (https://technical.buildingsmart.org/projects/2024tunnel/schema/ifcsharedbldgelements/pset/pset_rampcommon.htm).
- **[D]** No Slope property was found for RampCommon. Do not teach a Slope property on ramps until verified. The slope of a ramp is a geometric matter, so describe it as "derived from the modelled flights" unless 4.3.2.0 shows a property.

### Quantity sets

- **[P]** Qto_SlabBaseQuantities (IFC 4.3.2.0): Width, Length, Depth, Perimeter, GrossArea, NetArea, GrossVolume, NetVolume, GrossWeight, NetWeight (https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/Qto_SlabBaseQuantities.htm). The search excerpt's text about Width/Depth was contradictory. Verify which name carries thickness before narrating it.
- **[D]** Qto_WallBaseQuantities (IFC4 export example): Length, Width, Height, GrossFootprintArea, NetFootprintArea, GrossSideArea, NetSideArea, GrossVolume, NetVolume.
- **[M]** Qto_ColumnBaseQuantities: Length, CrossSectionArea, OuterSurfaceArea, GrossVolume, NetVolume, GrossWeight, NetWeight (verify).
- **[M]** Qto_WindowBaseQuantities: Height, Width, Area, PerimeterLength, plus gross/net variants (verify).
- **[M]** Quantity sets are computed by the authoring software and are optional. Teach them as "quantities that a tool derives", not as authored facts.

## 5. Materials

- **[M]** IfcMaterial: a single named material (for example reinforced concrete).
- **[M]** IfcMaterialLayerSet: ordered layers with thickness (for example a roof build-up or wall build-up). IfcMaterialLayerSetUsage adds LayerSetDirection (AXIS1/AXIS2/AXIS3), DirectionSense (POSITIVE/NEGATIVE), and OffsetFromReferenceLine.
- **[P]** Guidance: IfcWall and IfcSlab use IfcMaterialLayerSetUsage when thickness is uniform along the path and a layer set describes it fully. Walls with varying thickness or non-vertical extrusion axes use IfcWall without that usage (https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcWall.htm).
- **[P/M]** IfcMaterialProfileSet / IfcMaterialProfileSetUsage: for elements whose profile is swept along a directrix, such as columns with a profile that may taper uniformly (https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcColumn.htm, development build). Use it for pilotis only if the column is a constant profile.
- **[M]** IfcRelAssociatesMaterial links the material definition to one or more elements.
- **Teaching point:** The Villa Savoye ground slab and roof slab can use a layer set (concrete plus insulation plus finishes). The reinforced concrete columns use a single material with a profile.

## 6. Relationships

- **[M] IfcRelAggregates**: decomposition. Project -> Site -> Building -> BuildingStorey. Also spaces are aggregated under storeys (not contained).
- **[M] IfcRelContainedInSpatialStructure**: building elements (walls, columns, slabs, windows) are contained in a storey (or space). Spaces are NOT contained; they are aggregated.
- **[M] IfcRelReferencedInSpatialStructure**: referencing an element in a storey without containment.
- **[M] IfcRelVoidsElement**: a wall (or slab) is voided by an IfcOpeningElement. The opening is a real element with its own geometry.
- **[M] IfcRelFillsElement**: the window (IfcWindow) fills the opening element. Correct chain for a window in a wall: wall -[IfcRelVoidsElement]-> opening -[IfcRelFillsElement]-> window.
- **[M] IfcRelConnectsElements**: physical connections between elements (for example a column joined to a slab). Use it only where a real connection is modelled.
- **[M] IfcRelSpaceBoundary**: space boundaries (IFC 4.3, used for energy and space analysis). Usually created by tools, not hand-authored in clips.
- **[M] IfcRelDefinesByType**: occurrence -> type.
- **[M] IfcRelDefinesByProperties**: property sets and quantity sets on occurrences.
- **[M] IfcRelAssociatesMaterial**: material to elements.

## 7. Geometry representations

### Representation identifier and type

- **[P]** IfcShapeRepresentation: RepresentationIdentifier (kind of representation) and RepresentationType (geometric model). The identifier says what the geometry is for; the type says how it is described (https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcShapeRepresentation.htm).
- **[P]** Identifiers named in the 4.3 table: **Axis** (single line, 2D or 3D), **Body** (3D solid, wireframe or surface), **Body-FallBack** (tessellated or boundary fallback next to a solid), **Box** (simplified 3D box), **FootPrint** (2D projection to the ground), **Annotation** (2D or 3D annotation; IFC4 limited this to 2D).
- **[P]** RepresentationType values in the 4.3 table: **Curve2D**, **SweptSolid**, **Brep**, **CSG**, **Clipping**, **AdvancedBrep**, **BoundingBox**, **SectionedSpine**, and tessellated types. IFC4 used **Tessellation** and **SolidModel**. The IFC4 ADD1 table used "Mesh" where 4.3 uses "Body-FallBack".
- **[M]** Plans and sections are NOT stored as separate IFC geometry. `FootPrint` and `Axis` are per-element 2D representations. Plan and section drawings are generated from the model by view tools, and annotations are separate.

### Solids and profiles

- **[M] IfcExtrudedAreaSolid**: a profile extruded along a direction (the usual representation for a prismatic column, wall, or slab).
- **[M] IfcCircleProfileDef**: circular profile with Radius (round pilotis; ProfileType AREA).
- **[M] IfcRectangleProfileDef**: rectangular profile (square or rectangular columns).
- **[M] IfcTriangulatedFaceSet / IfcPolygonalFaceSet**: mesh geometry (tessellation). Use for free forms or imports; avoid for parametric teaching examples.
- **[M] IfcFacetedBrep**: boundary representation.

### Teaching point

For a round Villa Savoye piloti: IfcColumn (COLUMN), body representation RepresentationIdentifier 'Body' with RepresentationType 'SweptSolid', geometry IfcExtrudedAreaSolid over an IfcCircleProfileDef. Confirm the section and the identifier spelling in 4.3.2.0.

## 8. GlobalId (IfcGloballyUniqueId)

- **[P]** IfcGloballyUniqueId is a 128-bit GUID stored as a 22-character string (https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcGloballyUniqueId.htm; IFC4 FINAL: https://standards.buildingsmart.org/IFC/RELEASE/IFC4/FINAL/HTML/schema/ifcutilityresource/lexical/ifcgloballyuniqueid.htm; background: https://technical.buildingsmart.org/?p=933).
- **[P]** Alphabet (64 symbols): `0-9`, `A-Z`, `a-z`, `_`, `$`. In order: `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$`.
- **[P]** The encoding is not plain base64 and not a 36-character UUID. The first byte fills the first two characters (so the first character is 0 to 3). The remaining 15 bytes go in five groups of three bytes, each giving four characters. Total: 2 + 5 x 4 = 22.
- **[P]** Base-85 was used in the earliest versions; R2x and later use base 64 so the strings do not clash with STEP and XML special characters.
- **[M]** Implementations must convert between the standard GUID and this encoding. Use a library or the official algorithm; do not hand-type GUIDs in clips.
- **Example string:** `0YvctVUKr0kugbFTf53O9L` (22 characters; first character 0). This string is widely used in buildingSMART and ifcOpenShell sample files [M, not re-verified in this session].
- **Teaching point:** The GlobalId is the stable identity of an element across exchanges; the STEP `#n` number is only a local file reference and changes from file to file.

## 9. STEP physical file (ISO 10303-21) and an IFC4.3 excerpt

Syntax facts [M]:
- A file starts with `ISO-10303-21;` and ends with `END-ISO-10303-21;`.
- Sections: `HEADER;` ... `ENDSEC;` then `DATA;` ... `ENDSEC;`.
- Entity instances: `#<n>=<ENTITY NAME>(<attribute list>);`
- Strings in single quotes; `$` means unset (OPTIONAL attribute left empty); `*` means derived; enumerations use dots, for example `.COLUMN.`; references are `#n`.
- Attribute order follows the schema inheritance: IfcRoot (GlobalId, OwnerHistory, Name, Description), IfcObject (ObjectType), IfcProduct (ObjectPlacement, Representation), IfcElement (Tag), IfcColumn (PredefinedType).

Excerpt (placeholders #5, #120, #121 must exist elsewhere in the file; this is a syntactically correct fragment, not a full model):

```
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('ViewDefinition [DesignTransferView]'),'2;1');
FILE_NAME('villa-savoye-teaching.ifc','2026-10-27T10:00:00',('Teaching'),('Example'),'','','');
FILE_SCHEMA(('IFC4X3_ADD2'));
ENDSEC;
DATA;
#101=IFCCOLUMN('0YvctVUKr0kugbFTf53O9L',#5,'Pilotis P01',$,$,#120,#121,'P01',.COLUMN.);
ENDSEC;
END-ISO-10303-21;
```

Field mapping for #101:
- GlobalId `'0YvctVUKr0kugbFTf53O9L'` (IfcRoot)
- OwnerHistory `#5` (IfcRoot; IFCOWNERHISTORY instance)
- Name `'Pilotis P01'` (IfcRoot)
- Description `$` (IfcRoot, optional)
- ObjectType `$` (IfcObject, optional)
- ObjectPlacement `#120` (IfcProduct; IFCLOCALPLACEMENT)
- Representation `#121` (IfcProduct; IFCPRODUCTDEFINITIONSHAPE)
- Tag `'P01'` (IfcElement)
- PredefinedType `.COLUMN.` (IfcColumn; IfcColumnTypeEnum)

[M] The FILE_DESCRIPTION view string is a convention used by exporters; the 4.3 schema does not require a specific value. Verify the header against an export from a 4.3 tool before publishing.

## 10. Structural analysis domain

- **[D]** IfcStructuralCurveMember: line-type analytical member for beams, columns, rods. Straight or curved (https://iaiweb.lbl.gov/Resources/IFC_Releases/R2x3_final/ifcstructuralanalysisdomain/lexical/ifcstructuralcurvemember.htm; IFC4.2 Bridge: https://nibs.org/files/docs/IFC4x2_Bridge/schema/ifcstructuralanalysisdomain/lexical/ifcstructuralcurvemember.htm). It needs a topology representation.
- **[P]** IfcStructuralSurfaceMember: face-type analytical member for slabs, walls and shells. Depth is carried by the Thickness attribute (https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralSurfaceMember.htm).
- **[M]** IfcStructuralPointConnection: node (point) connection; IfcBoundaryNodeCondition: support stiffness/restraint per degree of freedom (translation and rotation, X/Y/Z). Verify attribute names in 4.3.2.0.
- **[M]** IfcStructuralAnalysisModel: groups analytical items (members, connections, loads) into a model with a predefined type (for example in-plane or 3D frame). Verify.
- **[D]** IfcStructuralMember is abstract; its inverse relationship to IfcRelAssignsToProduct was present in IFC2x3 but was not confirmed for 4.3 (https://iaiweb.lbl.gov/Resources/IFC_Releases/R2x3_final/ifcstructuralanalysisdomain/lexical/ifcstructuralmember.htm). The link from the analytical member to the physical element is the critical point for the clip: verify the exact relationship (IfcRelAssignsToProduct or IfcRelConnectsStructuralElement) in 4.3.2.0 before naming it.
- **Villa Savoye mapping [M]:** pilotis -> IfcStructuralCurveMember (vertical axis line); concrete slabs -> IfcStructuralSurfaceMember with Thickness; walls that are infill (not structural) would not be analytical members.

### What is typically lost in architecture to analysis translation

- Elements become idealised lines or faces on the axis or mid-plane; eccentricities (offset from axis) must be modelled explicitly.
- Openings, ribbon windows and their stiffness effect are simplified or removed.
- Connection stiffness (pinned vs rigid) is an assumption, not a geometry property.
- Support conditions and boundary restraints are missing from the architecture model; they are added in the analysis model.
- Loads (dead, live, wind) are not architectural information; they come from the structural engineer.
- Material nonlinearity and section properties (area, second moment of area) are derived or added; the architect's model does not carry them.
- Load paths between elements (slab to column to foundation) are implied in the analysis, not stored as architectural relations.

## 11. Model View Definitions, IDS, BCF, ISO 19650

### Model View Definitions (MVD)

- **[P/D]** IFC4 Reference View (v1.2, 2019): a standardised subset for reference models, mostly one-way exchange; geometry changes go back to the author (https://standards.buildingsmart.org/MVD/RELEASE/IFC4/ADD2_TC1/RV1_2/HTML/schema/views/reference-view).
- **[D]** Design Transfer View: for editable hand-over between BIM tools; supports parametric editing of connected elements within a limited scope. The official Design Transfer View text was not retrieved; vendor help pages only (https://help.graphisoft.com/AC/20/INT/AC20Help/07_Interoperability/07_Interoperability-71.htm).
- **[D]** buildingSMART states that the 4.3 MVD policy has three implementation levels: Reference View, Alignment-Based View, Design Transfer View (https://standards.buildingsmart.org/IFC/DEV/IFC4_4/HTML/content/introduction.html; verify the 4.3 policy page).
- **[M]** The "Coordination View" (IFC2x3 CV2.0) is what many exporters still write. Do not present it as an IFC 4.3 view.

### IDS (Information Delivery Specification)

- **[D/P]** IDS is an official buildingSMART standard; version 1.0 was approved on June 1, 2024 (https://www.buildingsmart.org/standards/bsi-standards/information-delivery-specifications-ids/). It is machine-readable XML that states which information a model must contain. It is a requirement specification, not a model.
- **[M] Structure:** an `ids` document holds `info` and one or more `specification` blocks. Each specification has `applicability` (which objects, via facets such as entity, property, attribute, classification, material, partOf) and `requirements` (facets with a cardinality of required, optional, or prohibited, and optional value restrictions). Verify exact element names against the IDS 1.0 XSD.
- **Example requirement [M]:** for all IFCCOLUMN instances, Pset_ColumnCommon must exist with property LoadBearing (required), with value TRUE; FireRating must also exist (required, any value).
- **Caution:** `LoadBearing = TRUE` is not universal. Architectural (non-load-bearing) columns and pilasters exist (IfcColumnTypeEnum PILASTER). A rule that forces TRUE on all columns will reject correct models. A better teaching rule: LoadBearing must exist; its value must match the structural model.
- **[D]** Example IDS requirement found in the wild: FireRating required in Pset_WallCommon for IFCWALL (buildingSMART example; see https://github.com/buildingSMART/IDS/issues/237). Use it as a model for the clip, not as the authority.

### BCF (BIM Collaboration Format)

- **[D]** BCF is a communication format for issues (topics), not a geometry format. Current version: BCF-XML 3.0 (https://github.com/BuildingSMART/BCF/blob/1d3ad49bced7f27135c5ce4ca6ac3591ad1057b8/Documentation/README.md; repository buildingsmart/bcf-xml).
- **[D]** Package: ZIP; one folder per topic named by GUID (lowercase). The folder holds `markup.bcf` (topic XML: header, topic properties such as title, type, status, labels, assigned-to, due date; comments; viewpoint references) and one or more viewpoint files (`.bcfv`, visinfo schema: camera, selected components, clipping, markups) with optional snapshots.
- **[D]** BCF 3.0 adds supplemental documents (documents folder and documents.xml) and extensions for topic types, statuses, and labels.
- **[D]** Known issue in BCF 2.0 discussions: viewpoint-to-comment links are optional, so orphan viewpoints and comments can exist (https://github.com/buildingsmart/bcf-xml/issues/23).
- **Teaching point:** BCF refers to IFC elements by GlobalId, so an issue on the pilotis can point to the same element across exchanges.

### ISO 19650 basics

- **[D]** ISO 19650-1 covers concepts and principles; ISO 19650-2 covers the delivery phase (information management during design and construction). Federated models: teams author their own models, which are combined for coordination (https://ukbimframework.org/wp-content/uploads/2020/02/ISO_19650_Guidance_Part_2_Processes_for_Project_Delivery_ThirdEdition.pdf).
- **[D]** Common data environment (CDE): shared environment in which information containers are stored and managed. An information container is a file or set of structured or unstructured information with a unique identifier (UK BIM Framework Part C: https://www.ukbimframework.org/wp-content/uploads/2021/02/Guidance-Part-C_Facilitating-the-common-data-environment-workflow-and-technical-solutions_Edition-1.pdf).
- **[D]** Status codes in the UK national annex used in workflows: S0 initial work in progress, S1 coordination, S2 information, S3 review and comment, S4 stage approval (vendor documentation: https://help.autodesk.com/cloudhelp/ENU/Docs-Files/files/files-naming-standard/Status_Codes.html). The authoritative table is ISO 19650-2 National Annex NA (https://www.cdbb.cam.ac.uk/files/national_annex_guidance.pdf). Treat these as UK convention; do not present them as global ISO codes.
- **[D]** Revision letters in the UK convention: P-prefix for preliminary (shared) revisions, C-prefix for contractual, A/B for published (vendor sources; verify in Part C).
- **[M]** The exact status table and whether S5 is withdrawn: check the national annex.

## 12. Data, information, knowledge

- **[M]** ISO 19650-1 talks about information (information model, information container, information requirements). It does not define a data-information-knowledge hierarchy. The DIKW pyramid is a general concept (Ackoff), not an ISO term.
- **Safe teaching statement:** "In ISO 19650 terms we manage information containers in a CDE; the IFC file is one kind of container." Do not say the standard defines data, information and knowledge as stages.

## 13. Villa Savoye mapping for the clips (planning aid)

| Element | IFC 4.3 class | PredefinedType / key attributes | Notes and checks |
|---|---|---|---|
| Reinforced concrete pilotis | IfcColumn | COLUMN; Pset_ColumnCommon.LoadBearing TRUE | Round section: IfcCircleProfileDef. Confirm section size from documentation. |
| Floor slabs | IfcSlab | FLOOR | Layer set usage only if thickness is uniform and layers are known |
| Roof garden slab | IfcSlab (ROOF) or IfcRoof (FLAT_ROOF) | Choose one and keep consistent | Roof build-up (waterproofing, soil) is a teaching example of IfcMaterialLayerSet |
| Ribbon windows | IfcWindow in IfcOpeningElement voids of walls | WINDOW (verify) | Wall -> void -> window fill chain |
| Exterior walls | IfcWall | Pset_WallCommon.IsExternal TRUE; LoadBearing per design | Villa Savoye's concrete frame carries loads; verify which walls are infill (LoadBearing FALSE) |
| Ramp | IfcRamp (with IfcRampFlight) | STRAIGHT_RUN_RAMP, TWO_STRAIGHT_RUN_RAMP or HALF_TURN_RAMP per actual geometry | Verify the flight arrangement before naming the type |
| Spaces (rooms) | IfcSpace (aggregated under IfcBuildingStorey) | Not a building element | Spaces are aggregated, not contained |
| Site, building, storeys | IfcSite, IfcBuilding, IfcBuildingStorey | Aggregation tree | IfcRelAggregates |

[M] The roof garden and ramp decisions are teaching choices; state the modelling choice on screen.

## 14. Safe teaching statements

These are short sentences that are correct as far as this research can verify. Items marked [M] should be confirmed before narration.

1. IFC is an open, ISO-standardised format for exchanging building information; IFC 4.3 is published as ISO 16739-1:2024.
2. In IFC 4.3, walls, slabs, columns, windows, ramps, roofs and beams are all subtypes of IfcBuiltElement, which descends from IfcElement and IfcProduct.
3. IfcSpace is a spatial element, not a building element; spaces are grouped under a storey through aggregation.
4. Every IFC element has a GlobalId, a 22-character identifier that is meant to stay stable across exchanges.
5. The STEP number (#101) is only a local reference inside one file.
6. A column's type is stored in its PredefinedType (for example COLUMN or PILASTER); the entity itself is IfcColumn.
7. Pset_ColumnCommon.LoadBearing states whether the column is meant to carry loads.
8. A window sits in a wall through an opening: the wall is voided by an IfcOpeningElement, and the window fills that opening.
9. Plans and sections are views generated from the model; the model is the primary information.
10. An analytical model is a separate representation: pilotis become line members and slabs become surface members.
11. IDS is a machine-readable requirement specification; BCF carries issues, viewpoints and comments and is not a model format.
12. ISO 19650 organises information in information containers that live in a common data environment.

## 15. Common mistakes to avoid

1. Calling IFC "the drawing format". IFC carries the model; drawings are derived.
2. Saying a window is "in" a wall without an opening. The chain requires IfcOpeningElement with IfcRelVoidsElement and IfcRelFillsElement.
3. Putting spaces under IfcRelContainedInSpatialStructure. Spaces are aggregated.
4. Using IFC4 names (IfcBuildingElement) in 4.3 teaching material, or IFC2x3 names in IFC4 files.
5. Teaching LoadBearing = TRUE as a universal rule for all columns (PILASTER and architectural columns can be non-load-bearing).
6. Claiming a Slope property exists for ramps. Ramp slope is geometric; the property was not confirmed.
7. Writing the GlobalId as a UUID or as plain base64. It is 22 characters with a custom alphabet.
8. Using the Coordination View name as if it were IFC 4.3 MVD.
9. Presenting an ifc43-docs "under development" build stamp as the released standard.
10. Using "SHED" for a roof type; the enumerator is SHED_ROOF [D].
11. Stating that ISO 19650 defines data, information and knowledge as a hierarchy.
12. Stating that the UK status codes (S0 to S4) are global ISO codes.
13. Stating that IDS is a model format; it is a specification of requirements.
14. Mixing analytical and physical geometry: a column line member is not the column solid.
15. Claiming all columns are IfcColumn; some pilotis may be better modelled as other products depending on the authoring tool. State the modelling decision.
16. Using `[DEBUG]`-style or tool-only fields in a teaching file as if they were IFC data.

## 16. Open items to verify before publishing a clip

1. Open https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcColumn.htm and confirm the supertype line and PredefinedType.
2. Confirm the Pset_WallCommon and Pset_WindowCommon property lists in 4.3.2.0.
3. Confirm the Qto_ColumnBaseQuantities, Qto_WallBaseQuantities and Qto_WindowBaseQuantities property names.
4. Confirm the full IfcRoofTypeEnum and IfcRampTypeEnum values, including QUARTER_TURN_RAMP and SPIRAL_RAMP.
5. Confirm the IfcStructuralAnalysisModel and IfcRelAssignsToProduct / IfcRelConnectsStructuralElement relationships in 4.3.2.0.
6. Confirm the IDS 1.0 XML element names and the BCF 3.0 file layout against the official XSDs.
7. Confirm the ISO 19650-2 national annex status table.
8. Confirm the Villa Savoye facts (pilotis section, ramp configuration, which walls are infill) against architectural sources before narration.

## 17. Sources used

Primary (search excerpts, pages not fetched directly):
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcWall.htm
- https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcSlab.htm
- https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcBuiltElement.htm
- https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcColumn.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcColumnTypeEnum.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcSlabTypeEnum.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcRampTypeEnum.htm
- https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/Pset_SlabCommon.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2/HTML/schema/ifcsharedbldgelements/pset/pset_columncommon.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/Qto_SlabBaseQuantities.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcShapeRepresentation.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcGloballyUniqueId.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4/FINAL/HTML/schema/ifcutilityresource/lexical/ifcgloballyuniqueid.htm
- https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralSurfaceMember.htm
- https://standards.buildingsmart.org/MVD/RELEASE/IFC4/ADD2_TC1/RV1_2/HTML/schema/views/reference-view
- https://www.buildingsmart.org/?p=29521
- https://www.buildingsmart.org/ifc-4-3-approved-as-a-final-standard/
- https://www.buildingsmart.org/standards/bsi-standards/information-delivery-specifications-ids/
- https://forums.buildingsmart.org/t/file-schema-ifc4x3-add2-or-ifc4x3/5485
- https://github.com/buildingSMART/IDS/issues/237
- https://technical.buildingsmart.org/?p=933
- https://technical.buildingsmart.org/projects/2024tunnel/schema/ifcsharedbldgelements/pset/pset_rampcommon.htm
- https://docs.xbim.net/XbimDocs/documentation/Xbim.Ifc4/Xbim.Ifc4.Interfaces.IfcRoofTypeEnum.html (secondary)
- https://iaiweb.lbl.gov/Resources/IFC_Releases/R2x3_final/ifcstructuralanalysisdomain/lexical/ifcstructuralmember.htm (IFC2x3)
- https://iaiweb.lbl.gov/Resources/IFC_Releases/R2x3_final/ifcstructuralanalysisdomain/lexical/ifcstructuralcurvemember.htm (IFC2x3)
- https://nibs.org/files/docs/IFC4x2_Bridge/schema/ifcstructuralanalysisdomain/lexical/ifcstructuralcurvemember.htm (IFC4.2 Bridge)

Secondary and vendor (context only):
- https://github.com/BuildingSMART/BCF/blob/1d3ad49bced7f27135c5ce4ca6ac3591ad1057b8/Documentation/README.md
- https://github.com/buildingsmart/bcf-xml/issues/23
- https://help.graphisoft.com/AC/20/INT/AC20Help/07_Interoperability/07_Interoperability-71.htm
- https://help.autodesk.com/cloudhelp/ENU/Docs-Files/files/files-naming-standard/Status_Codes.html
- https://www.ukbimframework.org/wp-content/uploads/2021/02/Guidance-Part-C_Facilitating-the-common-data-environment-workflow-and-technical-solutions_Edition-1.pdf
- https://ukbimframework.org/wp-content/uploads/2020/02/ISO_19650_Guidance_Part_2_Processes_for_Project_Delivery_ThirdEdition.pdf
- https://www.cdbb.cam.ac.uk/files/national_annex_guidance.pdf
- https://www.nordicbim.com/knowledge/en/ifc-4.3
- https://info.cype.com/en/new-feature/exporting-quantities-related-to-structural-elements-to-the-bim-model-in-order-to-facilitate-their-measurement/

Blocked in this session (not fetched): all standards.buildingsmart.org and ifc43-docs pages (HTTP 403); GitHub IFC4X3_ADD2.exp (HTTP 404 at the tried path).
