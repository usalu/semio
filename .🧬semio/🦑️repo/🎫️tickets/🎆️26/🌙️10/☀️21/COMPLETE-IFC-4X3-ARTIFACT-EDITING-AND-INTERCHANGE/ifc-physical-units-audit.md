# IFC Physical Units Audit

Read-only evidence from unchanged repository fixtures and pinned official EXPRESS releases.

## Ready official fixture tests

### C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏛️official/column-straight-rectangle-tessellation.ifc

SHA-256 58bb9b2cae96edf2c368de95f526650f3f282e4e7fc37c0065c501bdbeed00a1

    #12= IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);
    #13= IFCMEASUREWITHUNIT(IFCLENGTHMEASURE(0.0254),#12);
    #15= IFCCONVERSIONBASEDUNIT(#14,.LENGTHUNIT.,'inch',#13);
    #36= IFCUNITASSIGNMENT((#15));
    #37= IFCPROJECT('0CxDbxzA1B4eLeOw9eIjQx',$,'Project',$,$,$,$,(#40),#36);

### C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏛️official/polygonal-face-tessellation.ifc

SHA-256 a76afccde57b9c5e21d797352beb309106037fcaed261ce61fbee413c9460bea

    #12= IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);
    #3061= IFCPROJECT('37E0t9DhPDuPXM8sgxexaw',#3054,'Grasshopper Project',$,$,$,$,(#1),#3062);
    #3062= IFCUNITASSIGNMENT((#3063,#13,#14,#3064,#3065));
    #3063= IFCSIUNIT(*,.LENGTHUNIT.,.MILLI.,.METRE.);

### C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏛️official/tessellation-with-individual-colors.ifc

SHA-256 3ab1290207d8538da9d8ebc14826b769eeb79fce7d75aaa85c35ed7a5a8dbd92

    #100= IFCPROJECT('2yXUajt9D3DwMqV1WYGofM',#6,'IfcProject',$,$,'IfcProject','',(#7),#101);
    #101= IFCUNITASSIGNMENT((#102,#103,#104));
    #102= IFCSIUNIT(*,.LENGTHUNIT.,.MILLI.,.METRE.);

The first two samples are retained unchanged from buildingSMART Certification-datasets revision 80d976a9b193a26a8e928c3e79bff67af1de68a8; source folder IFC 4.0.2.1 (IFC 4 ADD2 TC1)/ISO Spec - ReferenceView_V1.2. Their adjacent 🔣️.json records exact raw pinned URLs and bytes: column4594, individual-colors2593. Adjacent LICENSE.txt: copyright buildingSMART International Ltd., CC-BY-4.0. Column scale is exactly0.0254 metres per source inch; #14 dimensions=(1,0,0,0,0,0,0). Individual-colors scale0.001 metres per source millimetre. Polygonal source uses mm too, scale0.001; its own manifest entry pins Standards-builds a8664407b63d42651e48a78a252c76d09dd6274c,4203bytes, CC-BY-ND-4.0 and separate notice. Do not confuse IFC4 samples with similarly named IFC4X3 files in4x3 corpus: their bytes/schema/hashes differ.

## Formal positional contract

Pinned ticket official-IFC4-ADD2-TC1.exp and official-IFC2X3-TC1.exp independently agree on these zero-based serialized positions:

- IfcProject UnitsInContext index8. IFC4 inherits IfcContext (ObjectType4,LongName5,Phase6,RepresentationContexts7,UnitsInContext8). IFC2x3 inherits IfcObject ObjectType4, then directly LongName5,Phase6,RepresentationContexts7,UnitsInContext8. IFC4 assignment optional;2x3 required. Missing assignment must remain unresolved, not metres.
- IfcUnitAssignment Units index0.
- IfcNamedUnit Dimensions0,UnitType1.
- IfcSIUnit Prefix2,Name3; Dimensions is derived and serialized '*' at0. Validate the derived placeholder rather than expecting a reference to dimensions.
- IfcConversionBasedUnit Name2,ConversionFactor3. Its inherited explicit Dimensions0 must be length exponents(1,0,0,0,0,0,0); UnitType1 LENGTHUNIT.
- IfcMeasureWithUnit ValueComponent0,UnitComponent1. Preserve typed IFCLENGTHMEASURE wrapper check and finitepositive factor.
- IfcConversionBasedUnitWithOffset IFC4 adds ConversionOffset4; do not silently treat offset subtype as ordinary multiplicative length conversion. IFC2x3 has no such entity.

Both official EXPRESS enums contain exactly EXA,PETA,TERA,GIGA,MEGA,KILO,HECTO,DECA,DECI,CENTI,MILLI,MICRO,NANO,PICO,FEMTO,ATTO. Powers respectively18,15,12,9,6,3,2,1,-1,-2,-3,-6,-9,-12,-15,-18; absent prefix0. Neither includes modern RONNA/QUETTA/RONTO/QUECTO or ZETTA/YOTTA/ZEPTO/YOCTO. Unknown prefixes must refuse.

Primary prefix descriptions: https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/FINAL/HTML/ifcmeasureresource/lexical/ifcsiprefix.htm correctly states MILLI10^-3. Caution: legacy official IFC4 ADD2 lexical page https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2/HTML/schema/ifcmeasureresource/lexical/ifcsiprefix.htm displays MILLI10^-6, an inconsistent prose table typo (MICRO also10^-6); pinned EXPRESS defines names but no numeric powers. Do not encode that typo. Current official table independently has MILLI10^-3: https://standards.buildingsmart.org/IFC/DEV/IFC4_3/HTML/lexical/IfcSIPrefix.html . This latter is corroboration, not IFC4 schema substitution.

## Version2 candidates and scope

Existing own2 official-architecture.ifc has #13 Project→#14 assignment→#15 SI MILLI METRE, SHA256 c4db65ba847f6b369a95d6c54fa11f4750cbe6d59f021934e8923d8d578e5885. Its official provenance/license must be read from adjacent existing geometry fixture metadata before a new unchanged-official claim. Existing wellness original base/CV20/COBie copies share hash727fd6bdef74a7d0a059bc21c24f3c4666b125e80bda64753f19a26b7c6df51e, #120→#107→#42 MILLI METRE; structural seed hash differs and is a derived seed, not unchanged official input. No independently verified official imperial2x3 fixture was established in this pass; use authored neutral2x3 conversion chain cases labelled authored plus already verified unchanged IFC4 inch fixture.

## Bounded resolver tests

Follow only Project UnitsInContext assignment, not any arbitrary LENGTHUNIT found globally: polygonal file has unused #12 SI metre but actual project#3063 millimetre. Refuse duplicate assigned length units, wrong dimensions/type, cycles, malformed typed factor, negative/zero/nonfinite factor, multiplication overflow/underflow tozero, missing/foreign reference and exhausted originalcontrol. Tick every scanned project/unit/dimension/conversion link; preserve explicit unresolved/error status without world metre guess. Geometry vertices, placements, profile measures and camera must scale coherently once at defined boundary, never scale each conversion chain link into geometry repeatedly. Volume scales by factor cubed, area by squared.
