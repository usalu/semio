# Independent Physical Geometry Audit R6

Read-only reinspection of the frozen164-element JSON source and current Python derivation, 2026-10-28. All coordinates are authored teaching geometry, not survey measurements. Read model-remodel-r3.md and architectural-reference-r3.md. No source changes and no rendering were performed.

## R5 Fix Verification

All six identified R5 contradictions have been addressed at their cause: the private circulation now runs x6.8–8.1, giving1.225m clear after the0.075m wall half-thickness; son door now occupies y17.85–18.7 next to the new public hall; new W-PARENT-BATH-S/N close the bathroom; CW-1 has two explicit glazing_paths ending at the entrance jambs and Python building_geometry consumes them; SC-1 now ends x7.5 before the aperture begins x7.85; RAD-1 now occupies x6.3–8.3 in SP-SALON. These fixes do not establish global clash freedom.

## Remaining Concrete Contradictions

1. **Upper piloti penetrates the integrated bath chaise.** UP-D2 has axis(4.75,15.5), radius0.15 and extends from first to roof soffit. FIX-BATH second solid part occupies x4.7–5.7,y14.5–16.5,z3.5–4.1. Their volumes therefore intersect: the column axis itself is within the chaise footprint, not merely a tolerance touch. Evidence: JSON tags UP-D2 around line9571 and FIX-BATH around line6942. The grid is declared inferred; resolve fixture/column placement together, rather than portraying the source arrangement as a measured historical structure.

2. **New bathroom south wall clips the guest doorway corner.** OP-D-GUEST occupies x4.3,y9.5–10.35 on W-PRIVATE. New W-PARENT-BATH-S is centred y10.3, thickness0.15, spanning x4.3–6.8; its solid reaches y10.225–10.375. Thus the final0.125m of the nominal0.85m guest portal is blocked on its hall side, leaving only0.725m before this return wall. Evidence: OP-D-GUEST around line5403; W-PARENT-BATH-S around line9968. Keep the intended room topology but move the portal wholly below y10.225 or revise the wall junction.

3. **Garage room polygon does not match its enclosure at the angled lower corner.** SP-GARAGE uses vertex(14.5,3.5) while W-G-GARAGE-END terminates(15.15,3.5), and W-G-GARAGE-CORNER follows x15.15 to y7. The space's diagonal(12,6)→(14.5,3.5) differs from the actual wall(12,6)→(15.15,3.5). The derived40.1m² space area therefore is not the enclosed usable garage area. Evidence: SP-GARAGE around line10292, W-G-GARAGE-END/CORNER. Align room geometry with the actual interior wall faces before using that area as a capacity certificate.

## Additional Requested Checks and Evidence Limits

The solarium screen remains on the low-y roof side, consistent with the CMN plan topology in architectural-reference-r3.md. Its frame aperture is x7.85–10.45 at y5.8. Both ramp axes x8.65 and10.05 pass through that horizontal aperture span. It is therefore unsupported to claim that the current opening points away from the ramp. The exact screen curves and dimensions remain assumed, and the reference report contains no surveyed coordinates with which to validate them.

Both helical stairs use positive one-turn winding, start_angle0, centre(6,8.8), radius1.1. Their final wedge reaches the east side and explicit landing patches extend x6.9–7.55,y8.35–9.25. This is a physical east-side arrival connected to slab across the stair void. The CMN audit does not establish a precise historic winding direction, so no direction error is asserted. Ramp upper arrivals remain at the low-y end y8 with0.7m-deep terminal landings; the northward half-turns are y17.3–18. The roof frame lies beyond the upper arrival toward low y. This authored sequence is internally plausible; it is not a verified measured promenade.

The three garage door openings span y7.2–9.8,10–12.6,12.8–15.4 in the east enclosure. The main rectangular garage strip is only4.1m wide in its room polygon, while the model car is4.6m long by1.75m wide. Therefore three ordinary perpendicular parked cars cannot be inferred from three doors. Angled parking could change that result; no assertion that all possible three-car packings are impossible is made here. A credible three-car claim requires three explicit non-overlapping oriented rectangles inside the actual enclosure, avoiding ground columns C4(14.25,10.75) and D4(14.25,15.5), plus verified ingress. The existing arrival route ending outside the vestibule establishes none of those parking conditions.

Most intended room portals now lie on their actual shared boundaries: son/hall, parents/hall, parents bath/hall, guest/shared bath, son/shared bath, pantry/kitchen and salon/hall. Boudoir has a terrace entry and is now physically separated from parents by W-PARENTS-BOUDOIR. Support-under-slab checks and independent Shapely execution remain with the coordinator. No global certification or arbitrary historical precision is claimed.
