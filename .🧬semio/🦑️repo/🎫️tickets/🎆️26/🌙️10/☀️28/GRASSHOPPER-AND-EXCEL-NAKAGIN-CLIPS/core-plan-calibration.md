# Core Plan Calibration

Read-only independent calibration,2026-10-29. No production changes. This report corrects the overly broad phrase chamfered core: the primary plan distinguishes square outer envelope, inward-facing triangular corner piers and a clipped-corner central lift enclosure.

## Actionable Findings

1. Retain5.5m outer core side provisionally. The primary5.05 label does not safely establish exterior width;5.5−2×.22=5.06 is compatible with a5.05 clear span. Its extension-line endpoints need proper source interpretation before changing all attachment frames.
2. Add source-visible triangular corner piers/internal diagonal faces rather than simply cutting exterior corners off a square core. Current four thin uniform perimeter walls omit this structural shape.
3. The central clipped enclosure is a COMPOUND: two stacked left service compartments plus a right lift chamber separated by a thick vertical partition. The1.35m lift parameter may approximate the right chamber; it must not be enlarged to the compound width. The thin cabin rectangle is a third, separate outline.
4. Native tambour assets contain no detailed inner piers/lift enclosure. Their dimensions cannot supply nonexistent chamfer lengths. Any primary-image trace must be labelled inferred, not native-measured.

## Primary Plan Reading

Source: [Domus520 original section/typical plan](https://www.domusweb.it/en/from-the-archive/2010/12/21/kurokawa-s-capsules.html), original JPEG domus-kurokawa-capsule-03.jpg,1280×1547. Local inspected file is domus-section-full.jpg. In-memory crops/grid overlays were used for inspection; no extra image files were written.

The four dark corner shapes have orthogonal outward edges and diagonal inward faces. Together with the intervening attachment/door strips, they fit an approximately square exterior envelope. The central compound enclosure separately has four short diagonal corner edges. It is not a single giant lift shaft: a thick vertical partition and a horizontal subdivision define two left compartments alongside the right lift chamber. Their precise service identities are not labelled in this raster. The inspected floor11-original.jpg diagram independently depicts square blue outer shaft bounds, triangular corner piers and a clipped central enclosure; it is corroborating schematic evidence, not a measured drawing.

The5.05 label is clearly legible. Dark extension-line peaks occur around original raster rows1193–1194 and1364–1365 at x150–220; the dimension span is approximately171–172pixels. These lines extend through the west capsule zone and do not unambiguously identify two exterior structural faces. The core-plus-pier apparent exterior spans approximatelyx313–514 andy1184–1380. Treat line thickness and scan distortion as real uncertainty.

### Trace Coordinates, Not Measured Metres

The following manual trace brackets identify the exact observable shape to reproduce. Pixel origin is image top-left. They are approximate by several pixels; they are not an exact construction drawing digitization.

| Shape | Approximate primary raster coordinates / ratio | Safe interpretation |
| --- | --- | --- |
| Outer core/pier envelope | x313–514,y1184–1380; about201×196px | Approximately square outer envelope; no evidence for replacing it with an exterior regular octagon. |
| Northwest pier | Orthogonal corner near317,1185; inner diagonal joins roughly362,1185 and317,1227 | Two inward leg extents about42–46px, roughly.21–.23 of core side. If5.5m is retained, a preliminary inferred leg is about1.2m, with at least.1m uncertainty. Exact endpoints should be traced in source-native vector/detail before declaring measured. |
| Compound enclosure outline | Threshold-assisted bbox x369–455,y1241–1320; top straight edge aboutx378–447; chamfer legs about8–9px | Compound service+lift enclosure, not lift chamber alone. At capsule-based35–36px/m scale roughly2.4×2.2m; chamfer leg roughly.23–.26m. These remain inferred trace values. |
| Left service compartments | Thick vertical partition x394–398; clear compartments roughlyx374–393, with a horizontal divider | Two separate chambers are visibly recoverable. Their exact equipment/duct use is not labelled; do not invent it. |
| Right lift chamber | Clearx399–451,y1247–1314 | Approximately52×67px; at nominal capsule scale roughly1.45×1.9m. Width1.35m is a plausible current proxy, not established measured exactness. |
| Cabin rectangle | Thin linesx411–445,y1260–1300 | About34×40px, approximately.95×1.1m at nominal capsule scale. Distinct from chamber and compound enclosure. |
| Nominal capsule calibration | North capsule bounds aboutx318–406,y1047–1190 (~88×143px); nominal2.5×4m gives35.2/35.75px per metre | A few-percent difference remains; use raw anchors, do not overstate precise metres. Hardware-inclusive source-CAD capsule bounds are larger than the nominal envelope, so pixel matching must distinguish them. |

These ratios are suitable for an honestly inferred reconstruction constrained by the primary topology. They are not a reason to invent precise new quantities or force every axis onto a perfect regular octagon. Preserve attachment ports and re-run actual support/mesh clearance after any internal pier/shaft reconstruction.

## Native CAD and GLB Extraction

Inspected native standard files:

- 🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🥁️tambours/🧱️standard/📐️tambour.3dm
- same folder/🧊️tambour.glb

Rhino reports metre units and46 objects: one Brep on volume layer,8 door PolylineCurves,37 wireframe curves. The Brep has15 faces,38 edges and25 vertices. Its complete plan coordinate set is:

~~~text
(-2.75,-2.75),(-2.75,0),(-2.75,2.75),
(0,0),(0,2.75),
(2.75,-2.75),(2.75,0),(2.75,2.75)
~~~

No inner-corner/chamfer or central shaft vertex exists in that Brep. Normalized by2.75, coordinates are only−1,0,1. Vertical vertices are0,.916667,1.833333,2.75,3.666667,4.583333; they describe the kit's layered envelope, not primary2.90m dimensions.

Door polylines sit at x ory=±2.75, lateral ranges.5–1.3 or−1.3–−.5, and Z.2–2.2,1.116667–3.116667 or2.033333–4.033333. These are useful authored ports but contain no lift/pier detail.

The GLB has46 mesh nodes without node transforms. Y is vertical. Its material geometry mesh45 has15 primitives and the same plan coordinates after mapping nativeXY to GLB XZ. Wireframe expansion gives outer plan extrema±2.75275; material envelope extrema are±2.75. Convex hull is the square, not an octagon. Therefore neither native CAD nor exported GLB can furnish actual inner chamfer lengths; the kit is not an independent detailed as-built core source.

## Vertical Glyph Cautions and Search

The prior audit's confident4.00 ground-height reading is withdrawn pending independent source resolution: focused readers disagree over the middle digit in4.?0. Do not promote it to documented4.00 solely from this raster. The repeated2.90 dimension is materially clearer. The B roof-head glyph remains unresolved from the original1280px source; enlarging/interpolating cannot recover lost strokes.

A fresh web search found no accessible higher-resolution primary scan. The original Domus JPEG is already1280×1547. A UPC thesis search index reproduces a typical-plan figure attributed to Lin2011, but its PDF fetch returned403 and it was not inspected. Other student/redrawn plans and autogenerated permit/BIM pages are not accepted as surveyed evidence. MoMA confirms original drawings exist in its exhibition material, but the public press text does not expose a dimensioned high-resolution section sufficient to settle the B glyph. No new B roof value is asserted.

Search leads, not dimensional evidence: https://upcommons.upc.edu/server/api/core/bitstreams/e6844d5c-4a0a-4e12-8033-e41a404a5b53/content ; [MoMA original-material context](https://press.moma.org/exhibition/nakagin-capsule-tower/).
