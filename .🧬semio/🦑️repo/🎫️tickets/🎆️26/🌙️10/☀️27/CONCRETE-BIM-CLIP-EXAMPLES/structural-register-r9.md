# Structural Register R9

Read-only research input, 2026-10-28. Companion portable table: structural-register-r9.json. No source files edited. Do not convert unknown profile orientation, oval major axis or transfer elevation to invented numeric data.

## Evidence and Registration

Primary research: [Zaparaín Hernández et al.,2022, DOI10.3989/ic.88625](https://doi.org/10.3989/ic.88625), [author-hosted PDF including annexes](https://www.fernandozaparain.com/_files/ugd/89d2e0_ff4aadc4c8a3464d97e8c5f36e0b93d8.pdf). Local generated/author-r8.pdf. Article printed8–10 covers columns/ramp; annexes are PDF18–24 because of the author file's prefatory pages. The authors use measured building evidence and FLC contractor plans; this register is an independent image reading of their reconstruction, not a new measured survey.

High-resolution generated images: register-r9-ground.png(PDF19), register-r9-upper.png(PDF20), register-r9-section22.png and register-r9-section23.png. Readable upper detail crops: register-r9-upper-crop.png and register-r9-upper-mid.png. Reference coordinates in JSON refer to earlier archive-r8-page19.png at1.5x, NOT4x high-resolution pixels.

At1.5x the main grid axes are x92,275,459,643,826 and y225,409,593,777,960. Four4.75m modules span19m. Mapping to CMN-local Cartesian coordinates:

- x=19-(pixel_x-92)/(734/19)
- y=1.25+(pixel_y-225)/(735/19)

The180degree diagram rotation reverses x in page pixels, while local Cartesian y increases down the original archival page. Column5 is(0,1.25);1 is(19,1.25);29 is(19,20.25);33 is(0,20.25). Exact main axes were snapped to their known4.75m grid rather than retaining pixel rounding noise. Offset posts retain image-read coordinates with approximately±0.04m uncertainty. Stair centre(585,535) maps approximately(6.24,9.26); the old(6,8.8) teaching position must not anchor the archival registration.

**Envelope constraint:**19m is grid width. Exterior walls lie approximately0.15m outside each transverse end axis. The image suggests x[-.15,19.15] and y[-.05,21.55], but these are image-read bounds, not a new exact19×21.5m survey. Preserve measured grid and overhangs instead of silently compressing them into the old teaching box.

## Numbered Support Register

Coordinates are CMN-local metres. Ground/first profile labels are plan-symbol interpretations combined with article§6. A first-ceiling plan does not automatically establish that every drawn support starts at first-floor slab; transfer exceptions are explicit below. The full JSON preserves profile dimensions/basis and unresolved fields.

| Archive ID | x,y | Ground profile | First profile | Extent/exception |
|---|---|---|---|---|
| 1 | 19, 1.25 | circle | square |  |
| 2 | 14.25, 1.25 | circle | circle |  |
| 3 | 9.5, 1.25 | circle | circle |  |
| 4 | 4.75, 1.25 | circle | circle |  |
| 5 | 0, 1.25 | circle | square |  |
| 6 | 19, 6 | circle | square |  |
| 7 | 14.25, 6 | circle | circle |  |
| 8 | 10.898, 6.006 | circle | absent | Ends below upper ceiling |
| 9 | 8.128, 6.006 | circle | absent | Ends below upper ceiling |
| 10 | 4.75, 6 | square | square | Continues stair head |
| 11 | 0, 6 | circle | square |  |
| 12 | 14.263, 6.756 | circle | absent | Ends below upper ceiling |
| 13 | 10.898, 8.23 | square | absent | Ends below upper ceiling |
| 14 | 8.128, 8.23 | square | circle |  |
| 15 | 4.737, 8.23 | square | circle | Continues stair head |
| 16 | 19, 10.75 | circle | aerodynamic_oval |  |
| 17 | 0, 10.75 | circle | square |  |
| 18 | 10.898, 12.572 | square | square |  |
| 19 | 8.128, 12.572 | circle | circle |  |
| 20 | 4.737, 12.753 | square | circle |  |
| 21 | 14.263, 13.012 | circle | absent | Ends below upper ceiling |
| 22 | 19, 15.5 | circle | aerodynamic_oval |  |
| 23 | 0, 15.5 | circle | square |  |
| 24 | 14.263, 16.786 | circle | square |  |
| 25 | 10.898, 16.889 | square | square |  |
| 26 | 8.128, 16.889 | rectangle | rectangle |  |
| 27 | 4.737, 17.277 | circle | circle |  |
| 28 | 9.5, 18.079 | rectangle | rectangle |  |
| 29 | 19, 20.25 | circle | square |  |
| 30 | 14.25, 20.25 | square | square |  |
| 31 | 9.5, 20.25 | circle | circle |  |
| 32 | 4.75, 20.25 | square | square |  |
| 33 | 0, 20.25 | circle | square |  |
| 34 | 9.5, 6.006 | rectangle | rectangle | Transfer birth, not ground pier |
| 35 | 9.5, 8.23 | rectangle | rectangle | Transfer birth, not ground pier |
| 36 | 10.898, 15.468 | square | square | Transfer birth, not ground pier |
| PA | 9.5, 12.572 | chamfered_rectangle | chamfered_rectangle | Ramp support, beam6 |
| PB | 9.5, 16.889 | chamfered_rectangle | chamfered_rectangle | Ramp landing beamF |

## Profile Dimensions and Confidence

Article§6.1–6.2: ground circular pilotis enlarged to28cm finished diameter except19; drain-bearing2/4 are33cm finished. Upper ordinary circular profiles retain22cm, while2/4 retain enlarged33cm drain profiles.19 is excluded from enlargement:22cm is the supported nominal reading, not a newly measured finished diameter.

Square columns are nominal22×22cm structural dimensions; article notes approximately1.5cm plaster each face. Do not present25cm finished square and22cm structural square as inconsistent alternatives.26/28 are16×22cm;34/35 are18×22cm. Their local rectangle orientation remains unresolved in this register and is null in JSON rather than guessed.

First-storey16/22 are special aerodynamic profiles. Article§6.2 describes three distinct sections along their height, with measured15cm finished narrow width in their freestanding zone, and a10cm facade setback. The main-axis table therefore is not the oval centroid: JSON records an approximately10cm inward first-storey shift for16/22 separately from their ground grid axes. The oval major length, analytic ellipse ratio and orientation were not numerically established here. A single15cm circle is wrong; an invented ellipse is also unsupported. Trace the actual varying contour from the contractor detail/complete book before claiming exact profile geometry.

PA is12×15cm with3cm chamfers, later plastered ovoid, born on basement-ceiling beam6. PB is a similar ramp stiffener born on landing beamF.34/35/36 and PB use transfer support rather than uninterrupted foundation-to-roof piers. Article§8 distinguishes their ramp support paths. Their exact birth elevations should come from the sections, not the default storey base.

Annex3 has no corresponding upper support at8/9/12/13/21. First-level16/22 support the terrace screen, not a conventional occupied-room roof column. At roof,10/15 are the explicitly established stair-head supports; article§5.1 discusses uncertainty in the other stair-head corners. Article§5.2 says proposed upward continuation of2/3/4 into the solarium screen was discarded: the actual screen is self-supporting with22×8cm ribs and8cm hollow-brick infill plus render.

## Structural and Finished Datums

The high-resolution section leaders distinguish these surfaces; one generic floor height loses actual differences:

| Elevation m | Physical meaning |
|---|---|
|+2.91|Common underside of first structural slab|
|+3.08|Top of17cm terrace structural slab|
|+3.14|Top of23cm inhabited structural slab|
|+3.11 / +3.17|Local finishes above those respective structural tops; not universal levels|
|+6.34|Typical upper slab underside|
|+6.51|Top of17cm roof structural slab|
|+6.60|Local roof weathering/finish surface|
|+6.77|Perimeter upstand/top-edge reference, not general structural slab top|
|-2.42|Basement slab/floor top|
|-2.72|Bottom of basement strip footing|

Article§6.1 confirms clear ground/first heights289/315cm. Do not derive every local finish by subtracting those rounded clear heights from structural soffits. Annex6 also labels3.11,3.14,3.17 and small bath/landing variations, so the model needs surfaces/parts, not three rigid elevation constants. A7cm basement-ceiling web and downstand ribs are distinct from the ground finish slab.

## Slab Regions Before Authoring

The JSON contains approximate pixel bounds and status for traced region candidates. They are not fabrication polygons; subtract real openings and reconcile interfaces before authoring.

**Annex2 ground ceiling:** the entrance-side full-width band(pixels86,175–831,409) is occupied23cm; the northeast wing right of ramp(pixels512,409–831,1010) is principally23cm; southwest hanging-garden wing left of ramp(pixels86,409–405,960) is17cm with real garage rooflight voids. The rear central strip beside ramp termination is annotated h20: do not erase that exception by forcing only17/23 categories. Ramp and landing transitions require separate polygons.

**Annex3 first ceiling:** most room roof is h17; stair/bath block around pixels459,409–700,593 carries h23. The hanging garden(pixels86,409–459,777) remains open to sky, with narrow canopy/frame portions distinct from a filled slab. The kitchen patio is a real northeast-side void, approximately pixels700,409–826,523, not part of the main hanging garden. Read the beam/rooflight outlines as boundaries; beam-depth annotations such as22×47 are not slab thickness labels.

For a model that adopts these datums, rebuild walls, apertures, columns, ramp endpoints, stairs and all fixture elevations coherently, then independently test physical intersections. Existing passing tests against the old0/3.5/6.7 abstraction do not justify distorting the archival geometry.

## Basement and Remaining Limits

Annex1 is a narrow two-bay central enclosure; register it through the grid above, then align its actual stair and internal column14. The cellar's outside face is approximately original pixels388–660 in x and390–832 in y, excluding projections; these correspond to local x4.30–11.34 and y5.52–16.94. Trace masonry inner/outer faces, central partial divider and annex projections separately. The small side projections are not labelled hoist apertures and must not be named as such without architectural evidence.

Full book university URL remains identified but unreachable from this audit's network. No alternate complete book was found in the author's public page links; the author PDF supplies structural rather than full furnishing plans. Exact facade build-ups, all oval contours, rectangular profile rotations, every support birth and the hoist location remain open. Explicit unknowns in the portable table are deliberate research constraints, not permission for arbitrary defaults.
