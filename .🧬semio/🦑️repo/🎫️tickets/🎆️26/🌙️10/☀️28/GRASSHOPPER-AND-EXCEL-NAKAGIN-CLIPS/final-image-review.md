# Final Image Review

Partial read-only actual image review of the first two fresh galleries from running root capture17338. Start ledger: 2026-10-30T03:53:35.948787+00:00. Both PNG modification times are later than this start. Capture has not reached terminal/current endpoint acceptance; six remaining galleries are unreviewed.

- nakagin-detail-gallery.png; modifiedUTC 2026-10-30T04:37:39.015080+00:00; size (1920, 1080); SHA256 e3d19c4d2d48f5b6045f5554619d0cd3b24f2bae748639acc409c10593d52085
- nakagin-orthographic-gallery.png; modifiedUTC 2026-10-30T04:38:44.788606+00:00; size (1920, 1080); SHA256 c3f9bd29dea4fd870a1db79e16826f91a919bad4b884103427ba9261a162c0b9

## Detail Gallery

Actual1920x1080 image inspected. Five separated compositions remain inside the frame: capsule cutaway, core, capital, podium and full tower. Circular windows, stacked capsule offsets, podium openings and roof signs are visible. No obvious detached full-face fragments or missing whole tower portions appear at this scale. The open capsule cutaway shows a conspicuous repeated triangular comb along its front upper interior rim (approximately x370-438,y362-397); this is a concrete visible edge-treatment concern for owner review, without attributing its cause from the image alone.

## Orthographic Gallery

Actual1920x1080 image inspected. Plan fixtures and rounded envelope are visible; width2.60m,length4.20m,height2.50m and window diameter1.30m annotations render with strong yellow contrast. Concrete layout defect: the front-view vertical height annotation between the two lower views is partly occluded by the side elevation: the unit at the right end of2.50m is clipped where the side view begins around x955,y752-781. Its right-side counterpart is fully legible. No other text clipping was observed.

These findings prevent an unqualified flawless visual-layout claim for the inspected pair. They do not invalidate separately passing geometry contracts. Root owns interpretation/fix decisions and capture endpoint acceptance.

## Source Diagnosis: Orthographic Annotation Collision

Shared V capsule_views1368-1373 spaces geometric bodies: side left sx=front centre fx+width*scale/2+gap. Default gap1.0 therefore reserves only1.0 scene units beyond the front body. Front height annotation1420 is placed on that same right edge. _away1187-1189 places its text to the right by dimension offset0.3 + label half-width +0.07, so its rightmost bound is frontBodyRight+0.37+label.width. Any label width above0.63 overlaps the side body at default gap. The actual English gallery at scale0.65 exhibits exactly this collision. German quantity formatting and custom fonts/scales can change label width; a larger fixed constant cannot guarantee separation.

Minimal clean layout correction proposal: after annotations attach at1432-1433, position the complete side group to the right of the complete front group using measured bounding boxes and the existing gap. Keep plan/front aligned as today. This defines gap between actual view contents (including annotation text) and automatically handles language/font width. If preserving body spacing is required, shift side only by max(0, front annotated right + explicit clearance - side left); do not hide or truncate the label. No production edit was made.

Language-agnostic case proposal in existing visuals cases: orthographic_annotation_clearance with layout families Slash/Dash/Cross, scales including0.58/0.65, language inputs en/de, requested gap and minimum content clearance. Assert every front annotation text bbox has no positive-area intersection with side geometry bbox, and full front/right bounds + gap <= full side/left bounds after layout. Include labels/geometry inside output frame for the declared gallery configuration. Compute reference rectangle intersection/separation with an independent library (existing Pillow/NumPy or Rhino bbox) rather than merely repeating the placement formula. Existing dimension test148-168 checks only numerical measurements, not text bounds, and therefore cannot catch this defect.

## Source Diagnosis: Cutaway Comb

The actual gallery uses capsule_axon(capsule_types()[0],cutaway=True,bolts=True), default view is changed to55deg azimuth/55deg elevation by1245; visual-smoke then applies uniform0.9 scaling. This high downward view exposes the front panel inner upper edge where the comb appears.

The cutaway code1270-1277 removes entire shell/lining mesh faces according to mean vertex z/x, using z < bounds.top-cut_depth and signed x < edge-cut_depth. It never clips a crossing polygon to either cut plane. Thus adjacent planar triangulation faces can survive/remove differently and a surviving triangle may extend past the cut plane. This makes a tessellation-dependent sawtooth material boundary. Both front shell (_panel1725-1730 -> _perforated_profile) and lining-front2034 use the planar perforated fan: model1708-1712 emits triangles from each sector midpoint to its material boundary. Rounded casing pieces are separately named shell-edge/shell-spherical at1999-2011; the perforated-front fan is planar and is not a physical rounded skin surface.

This precedes _assembly_items and BSP: ordered fragments receive the already jagged retained mesh. _fragment_item220 paints only emitted polygons and230 reconstructs only original retained polygon boundary strokes; it cannot restore material discarded by centroid selection. Assembly499 suppresses shell stroke but gives lining0.45 width, which can emphasize surviving cut triangles. Therefore merely suppressing artificial BSP triangle edges is not a sound fix for the cutaway selector defect: the underlying retained material boundary is wrong before BSP.

Clean fix proposal: replace centroid face removal with exact per-polygon half-space clipping against the two declared cutaway planes, retaining original normal and nonzero polygons, then rebuild the view-only mesh. This is a presentation cut; no physical model change is needed. It must preserve the real aperture and retained rounded facets. Language-agnostic regression: a triangle spanning a cut plane must become the exact clipped polygon; differently triangulated equivalent planar material must give the same clipped area/boundary; all retained vertices obey both bounds; compare material union/area with existing independent geometry library. Test actual front/lining parts in the55/55 view as a regression, including no repeated free boundary teeth. Existing test178-180 checks only overall lining extents and existing cutaway-role checks do not constrain the cut boundary.

Source proves this deterministic centroid-cut defect. Exact pixel-to-semantic-part attribution of each visible tooth was not rerendered in this read-only audit; front shell versus lining contribution requires owner diagnostic if needed before selecting a narrower fix.
