# Official Cant Counterpart Corpus Oracle

Executed authored cross-platform alignment-cant-oracle.py through existing IfcOpenShell0.9.0.post1 dependency interface. Four existing manifest entries under alignment paths were parsed; exact original hashes checked against pinned corpus commit ef22440ec501551ccc9689309560bbc7e26aa9d7. Two cant layouts found, one with explicit direct terminal geometry. Generated/alignment-geometry/cant-oracle.json records original headers/schema identifiers and all observed IDs/measure branches/order. No source edits/native execution/fullview/schema validation claim.

## Requested segmented-reference-curve example

Original SHA256d4320f2fcc7e9c385f89c56c3002a54127f8d9fb4c5a5a447bd7238806b1eadf; exact header/schema_identifier IFC4X3_ADD2. Cant61→RelNests63.RelatedObjects [62,149]; terminal149.DesignParameters148 is IfcAlignmentCantSegment with HorizontalLength0. Terminal149.Representation is absent, so no direct ProductDefinitionShape/shape/item correspondence can be followed. Parent IfcSegmentedReferenceCurve112 has Segments[113,157], final157 actual IfcLengthMeasure zero and BaseCurve89. This proves both terminal zeros exist in model, but does not authenticate business149↔curve157 correspondence solely from their last positions. Keep indirect matching unresolved.

## Explicit cant path in another existing official example

[linear-placement-of-signal](https://raw.githubusercontent.com/buildingSMART/IFC4.x-specification-models/ef22440ec501551ccc9689309560bbc7e26aa9d7/models/alignment-geometries-and-linear-positioning/linear-placement-of-signal/linear-placement-of-signal.ifc), SHA2563c6b97f7a82f1a1d114233702e876f387a03701be0d8a621685e5773ce664c97, original header/schema_identifier IFC4X3_ADD2. Cant layout2388→RelNests2447, ten ordered members ending6001→DesignParameters6011 actual IfcAlignmentCantSegment.HorizontalLength0. Same terminal Representation6101→ShapeRepresentation6102 (Axis/Segment)→CurveSegment6201, actual IfcLengthMeasure wrapper with wrappedValue0. Exact curve6201 is member index9 of10 and final element of IfcSegmentedReferenceCurve2594.Segments; BaseCurve2543.

This is direct object-identity correspondence, not guessed curve matching. It provides the missing cant positive fixture alongside previous explicit horizontal/vertical example. The other two inspected alignment files had no cant layout; absence is local to this four-file pinned alignment subset, not all buildingSMART examples.

## Executable use

An explicit selected-direct-counterpart predicate can use original H/V/C terminal→Representation→Representations→Items paths and validate zero under actual SELECT branch. Preserve segmented-reference-curve as an important indirect-only case requiring unresolved correspondence policy; do not convert it into either direct-match pass or automatic failure without explicit applicability rule. Negative fixture: change only6201 length, keep unrelated zero curve elsewhere, ensure predicate follows actual6001 representation rather than selecting any zero. Full parent geometry consistency, conversion and fullview certification remain separate.
