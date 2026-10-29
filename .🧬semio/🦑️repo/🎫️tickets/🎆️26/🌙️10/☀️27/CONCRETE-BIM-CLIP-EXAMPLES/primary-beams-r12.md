# Primary Beam Structure R12

Source: [Zaparaín Hernández et al.,2022](https://doi.org/10.3989/ic.88625), author PDF annex2/3 (PDF19/20), checked against annex5/6 (PDF22/23) and article§§3.3,7. The source is CC BY4.0. Registration follows structural-register-r9.md and its4.75m axes. This is a source-informed reconstruction with explicit inferred fields.

## Published Structure

The final coordinated source publication contains59 primary member objects: original31 exposed members plus28 flush/end/tie members. Three slab products SL-1/SL-R/SL-KIOSK are partitioned into92/79/7 non-overlapping material regions. Embedded beam strips use MAT-RC and name their owning beam with region.member. Every beam explicitly identifies its embedded_slabs. The member geometry contains only71 exclusive solid parts outside the slabs; the API combines those with shared slab-region parts for full member representation and quantity. Slab reference/walking surfaces remain complete: no artificial beam openings were made.

Important correction from plan-plus-section verification: first-level facade16/18×50cm members are upward beams with full extent2.91–3.41m, not downstands. Article§3.3/7 confirms33cm upward depth above the terrace slab. Full-section fixture corrects the24 rear-beam bottom to2.77m and13–18–25 beam bottom to2.71m against annex6.

The source-specific primary network includes interrupted inner frames, first-floor transfer cross-members8–9/13–14, roof frontal room frames, rear room frames, open garden boundary members, cantilever end ties, ramp edges and bath/stair reinforcement. No regular grid through the hanging garden was generated. Coordinates at displaced supports are averaged to a constant centerline when the register's slight offsets do not determine a visible skewed section.

## Inputs and Verification

The language-neutral primary-beams-r12.fixture.json preceded the initial exposed-zone implementation. primary-beams-r12.full.fixture.json records59 full sections and width ranges; primary-beams-r12.full.test.py preceded the ownership partition implementation. primary-beams-r12.full.mjs exports prepare/publish, using orthogonal cell subdivision, inherited source-region priority,3D solid subtraction and box consolidation. It adds no product runtime dependency.

Independent NumPy/Shapely oracle passed the initial31-member45-part partition:10.9696038m³ exposed volume and no pair overlaps. Independent full-section oracle passed59 references and three regional slab coverage/no-overlap checks; its original full-minus-slab volume was14.0857291071m³. The final71-part revision adds two section-datum corrections and awaits rerun against its refreshed generated snapshot. The test independently derives source member rectangles and physical slab masks, compares union-minus-slab sections against exclusive parts over everyz interval, and checks zero slab intersection.

Publication used fresh coherent source312 and resulted in340 elements. The model author received the source window back to regenerate analysis load links after the material change. Parent owns schema/API/full-quantity/analysis propagation and final representation/export validation.

## Remaining Source Limits

Section annotations25–30/22–30/18–24cm establish width ranges, but the inspected plan/text do not establish their direction or intermediate profile. These remain explicit unknowns; the physical register currently uses a constant minimum width with inferred provenance. Inventing a taper direction would falsely increase precision. Three unlabelled flush/stair ties use explicitly inferred standard widths/depths. Local rooflight trimmers are outside this primary-member register.

The open garden facade beam is drawn slightly diagonal; its constant-axis approximation remains inferred. Its16×32cm nominal section and the nearby6.77m cornice reference need the original contractor detail to distinguish beam body from shaped water cap. Current nominal beam datum6.19–6.51m is inferred and must not be presented as an exact measured cornice. The author PDF figure10 (PDF14) did not decode in the first PDFJS render: the figure area is blank due to a JPEG2000 rendering limitation. The complete structural annex remains readable. Publisher/DOI web opens also failed during this follow-up.

The earlier erroneous roof garden void was reconciled by the model author before full publication to the archival main garden bounds x9.5–19,y6.006–15.5m, with a separate kiosk cover cutout. This report does not claim an exact or complete as-built architectural model.

## Identifier and Analysis Follow-Up

The model author corrected59 beam and3 slab/member relation identifiers to the provider's canonical UUIDv5 URL namespace and IFC compression. Both retained beam author helpers now use that same deterministic contract, so future fresh merges preserve canonical identifiers. The model author regenerated all analysis links after ownership material partitioning and reported consistency=[] on the shared340-element model.
