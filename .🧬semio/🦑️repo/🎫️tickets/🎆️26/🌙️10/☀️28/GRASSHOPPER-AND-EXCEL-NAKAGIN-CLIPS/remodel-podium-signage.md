# Podium and Historic Signage Remodel

## Primary Evidence

The 1973 Domus elevation and original photograph (retained domus-section-full.jpg and domus-4.png) show opaque horizontal upper-podium cladding, a narrow high window strip, rounded top/bottom fascia returns, recessed ground entrance glazing and open pilotis. The former full-height upper glazing is unsupported by these sources. [Domus original article](https://www.domusweb.it/en/from-the-archive/2010/12/21/kurokawa-s-capsules.html) identifies an entrance-floor self-service café and offices on the floor above. [MoMA extended labels](https://press.moma.org/wp-content/uploads/2025/05/The-Many-Lives-of-the-Nakagin-Capsule-Tower-ExtendedLabels.pdf) identify ochre-yellow office floors. Native repository CAD supplies the irregular podium roof footprint, recessed ground footprint and 0/4/7.5 levels; parent is independently auditing the global dimension chain.

The original photograph shows vertical white 中銀 lettering near the high corner of the tall machinery head, on two adjacent faces. The current anonymous dark enclosure omits this major identifying detail. Glyph outlines, stroke widths, placement and mounting-panel dimensions will be explicitly inferred from the photograph rather than presented as measured lettering.

## Implementation Plan

Retain parent-owned levels and footprint. Author podium.architecture with normalized facade band heights, cladding/seam/fascia dimensions, entrance bay and room-fitout records; capital.signage stores sourced characters and inferred vector geometry/placement. New domain portable cases precede implementation, with independent NumPy/Rhino geometry checks for band ratios, closed solids, true glazing bounds, rounded return profile, hall/café/office furniture and entrance clearance, and sign character/face placement. Runtime geometry stays library-free.

Use a recessed glazed hall and café at ground level, open pilotis around them, and upper office partitions/fitout with ochre floor finishes. Exact room plans and fitting placement are unavailable in the inspected source and will remain reconstruction assumptions. Keep both south core-entry approach bands unoccupied. Root owns any added upper-office core doorway. Historic lettering will occupy the tall north/west high-corner faces with a small reconstructed mounting panel while retaining the open rear service/bracing zone elsewhere.

## Coordination

Execution owns podium_assembly and signage helpers plus podium.architecture/capital.signage keys and dedicated domain cases/tests. GH worker owns capsule detail functions/keys. All edits reread and compare exact ownership blocks before saving; no global datum, capsule or circulation changes. Functional support/headroom gates remain required after the additions.

## Independent Dimension-Crop Reading

The primary-podium-chain.png crop clearly shows the upper-storey label 3.72. The adjacent ground-storey label is too degraded to distinguish 4.40 from 4.00 confidently: the middle glyph looks angular like a 4, but its raster merges with the surrounding strokes. I would transcribe it as 4.?0 pending another original scan, not promote either reading to a measured parameter. Native CAD 4.0/3.5 remains a separately labelled reference, not a resolution of the printed drawing. No global datums were modified by this agent.

## Implemented and Verified

The upper podium now has separate opaque lower/upper cladding bands, real recessed strip glazing, window rails/mullions, panel joints and closed curved fascia-return solids joined to the facade/roof profile. Window limits are normalized to current parent-owned podium levels, so later datum corrections propagate automatically. Ground pilotis remain open around recessed glazed hall/café rooms. The source-grounded office/café programs include explicitly assumed partitions, door openings, finish floors, supported ceilings/lighting, reception/benches, cafe tables/chairs/service counter and office desks/chairs/storage. Office finish is ochre; exact partitions/fittings are not claimed as recovered original drawings. Every fitting carries room/basis metadata.

The tall A roof head carries vertical white 中銀 lettering on north/west high-corner faces. Manually authored normalized vector strokes have closed physical geometry, small inferred mounting panels, live-height positioning and size limits derived from head width. Both the mount and every stroke remain within the actual oblique roof boundaries under shorter/taller/narrower parameter overrides. The west mounting panel occupies only the upper sign zone, preserving exposed rear bracing elsewhere.

Dedicated portable cases and tests: **9 Python/NumPy tests PASS (2.008s)** and **AJV accepts authored data plus rejects all 8 new invalid variants**. Checks include every closed positive-volume podium part, exact window-strip ratios, real curved profile vertices and radial attachment, independent polygon containment of each room, all room-door/core approach clearances, each fitting’s support path, exact historical characters/faces, and live-dimension roof-face containment. A regression first failed for the east office desk crossing its door approach; only that inferred desk row/storage placement changed to clear it. A separate radial-profile regression first failed on an unattached outward fascia and now passes with the return joined inward to facade/slab.

New domain paths: 🧪️cases/🏬️podium/🔣️.json, 🧪️tests/🏬️podium/🐍️.py and 🧪️tests/🏬️podium/🟨️.cjs. No runtime dependencies were added; geometry remains library-free. Integrated model/circulation/visual gates remain necessary after parent-owned core/datum changes settle.

A final facade connection pass aligns horizontal window rails with the actual recessed glass/mullions and gives every rolled fascia return a small assumed metal anchor flange bearing on the floor/roof slab. These are physical closed solids, with two anchors per authored footprint edge. The nine domain tests remain green (2.468s) after these attachment corrections.

## Separate Roof and Parapet Datums

The parent’s final working podium datum is flat roof7.72m with levels0/4.00/7.72; this agent did not set those global levels. A separate0.40m derived parapet/coping rise is now modeled physically above the slab. The high opaque cladding and curved fascia meet a closed metal coping and inner return; the return bears through an anchor flange on the actual flat slab. Rooms/ceilings/slabs continue to derive from the flat level. A new portable NumPy bounds oracle proves slab top and finished edge are different, and ten podium/sign tests passed (1.712s). The recovered crop still does not justify claiming an exact ground-storey digit; the final level choice remains linked to the parent’s independently calibrated scan evidence.
