# Architectural Details R7

The authoritative CMN visitor leaflet (2021, second PDF page, sections 3, 6–7, 11–17), inspected as cmn-plans.png, supports the built-in fixture programme. The CMN teaching dossier pages 8–10 supports the solid helical stair guard topped by a metal tube, bedroom low sliding storage and window shelves, and parents dressing cupboards. See architectural-reference-r3.md and detail-completeness-r7.md for exact reference links and orientation.

## Schema-First Contract and Test Evidence

Portable cases were written first in clip/🧱️model/🧪️tests/🛁️details.json using the existing generic horizontal-polygon solid schema. The first execution of test_details_have_actual_hollow_and_occupied_volumes failed on FIX-HALL-BASIN because the original object occupied its entire bowl volume. This is a real red result against the existing source.

Four independent Shapely/NumPy tests in 🛁️details.py verify occupied/free basin samples, room containment, every actual column footprint, NumPy shoelace and Shapely quantity parity, 0.8 m person routes, real door approaches, consecutive guard solid intersections in plan and elevation, and a free stair-arrival exit. The prepared source trial passes all four tests. Shared-source integration is pending the coordinated geometry freeze and requires revalidation against the final source.

## Authored Built-In Forms

The hall basin and twin laundry troughs have separate bottom/rim solids and open bowls; trough legs and hall pedestal supply actual support. Two circular tabletops have hollow central rings wrapped around the two source-informed hall columns, while keeping the x8.5 entrance path open. Taps are separate metal fittings with inferred functional geometry.

Kitchen worktops include actual sink recess and base storage, rather than a solid block at sink height. The parents bath is hollow; its separate tiled chaise has 48 variable-width horizontal strips that form rounded ends and an undulating reclining surface, avoiding the actual UP-D2 column. Individual tile subdivisions are deliberately not asserted as historic measurements. The guest wardrobe separates washing and sleeping sides, while preserving a side passage; the washing bowl sits in the rooflight zone. Son concrete desk, low bedroom window storage, parents dressing cupboards, pantry cupboards and the boudoir window shelf follow the cited programme. Both servant rooms receive individual hollow basins and radiators.

Two terrace planter rings surround the actual coordinated garage-lightwell rectangles at x14.65–15.35/y8.85–9.55 and x14.65–15.35/y12.75–13.45. They leave the central light path open. Their exact sizes and positions are inferred from the plan topology, not surveyed.

Each stair receives a solid curved outer guard and separate metal tube composed from overlapping horizontal sector prisms. The final 25 degrees are left open for the real arrival exit; the geometry follows the source helical rise and reserves the central tread route. This is a finite horizontal-solid approximation to a smooth helix, not an exact swept surface.

## Limits and Provenance

Every metric coordinate, bowl size, tap shape, cupboard subdivision, chaise profile and guard thickness is inferred. Each new object records its source section and this dimensional uncertainty in bilingual source metadata. The source evidence establishes architectural built-ins rather than generic loose furniture. No unsupported beds, sofas, modern appliance ensemble or plumbing network was added. Schema, identity, containment/material/connection relations and final plan/axon inspection remain final integration gates.

## Final Integrated Result

The coordinated fresh-source merge completed atomically after the remodel agent froze wall/space/slab ownership. The final source has 210 elements and 21 spaces; 36 fittings/guards were authored or replaced (31 net new). It includes the chauffeur basin in the new actual bathroom and independent tap; the shared bathroom bath/basin; two concrete planter rings around actual slab lightwell openings; and the existing two-part terrace table remains. A separate MAT-METAL provides finish semantics for taps and stair tubes. All new elements have stable GUIDs, English/German names, source provenance, storey containment, material relations and architecture model membership. Sixteen explicit element connections relate integrated tables to columns, planters to their skylights, stair guards/tubes and taps to fixtures.

All four independent detail regressions pass against the integrated on-disk source, including the final chauffeur subdivisions, shared bathroom, changed support set and real serving hatch. Guard tests additionally prove coverage at 0.65 m above actual stair tread tops. The combined model regression and native consumer visual QA are delegated to the coordinator/model-remodel owner. No product runtime library or new geometry engine is required. The unsupported cellar plan and unsurveyed hoist position remain explicitly outside this detail wave; no invented hoist or underground room was added.

Changed files: clip/🧱️model/🔣️.json, clip/🧱️model/🧪️tests/🛁️details.json, clip/🧱️model/🧪️tests/🛁️details.py, and this audit. No generated test output was retained.

Final targeted QA moved the guest bowl/tap completely under the actual ROOFLIGHT-1 polygon (x1.1–1.8, y9.85–10.45). A fifth independent Shapely test now proves that vertical alignment. All five detail tests pass after the correction.
