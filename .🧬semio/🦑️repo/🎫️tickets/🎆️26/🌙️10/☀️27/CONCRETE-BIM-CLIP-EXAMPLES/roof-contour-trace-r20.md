# Roof Contour Trace R20

> Superseded in part: see [roof-aperture-topology-correction-r20.md](roof-aperture-topology-correction-r20.md). The circulation-gap and ramp-axis interpretations below are withdrawn; raw native curve controls remain valid.

## Evidence and Scope

Read-only audit of CMN 2021 visitor PDF page 2, inspected beside the extracted native vector image and current SC-1 source. The primary source [CMN visitor document](https://www.villa-savoye.fr/content/download/9824393/file/document-visite-villa-savoye-FR-2021.pdf?inLanguage=fre-FR&version=3), §18, confirms solarium access by both ramp and stair and the landscape opening in the ramp axis. The visitor diagram is an illustration, not an as-built dimensioned survey. Its vector decimal precision must not become claimed historical metric precision.

## Native Contour (Authoritative Trace Coordinates)

PDF coordinates have origin bottom-left, units PDF points, page 1122.52 × 595.276. The roof screen is constructPath operation 6707 in roof-vector-r19-page2.json, with translation (1009.2938,434.6015). It contains TWO disconnected subpaths; connecting them into a stadium invents a wall through a circulation break. These native cubic controls preserve the illustrated contour without fitting circular replacements.

First/right subpath (local coordinates relative to that translation):

- M (0,0)
- C (7.023,0) (12.717,-5.694) (12.717,-12.717)
- C (12.717,-19.741) (7.023,-25.434) (0,-25.434)
- L (-7.738,-25.434)
- L (-7.738,-16.900)
- L (-35.134,-16.900)

Second/left subpath:

- M (-46.949,-16.391)
- C (-51.071,-19.767) (-57.344,-21.792) (-63.637,-21.792)
- C (-76.214,-21.792) (-86.410,-13.450) (-86.410,-3.160)
- C (-86.410,0.633) (-85.025,4.161) (-82.646,7.104)
- L (-73.822,7.104)

The right curve is semicircular in this drawing; the left limb is an asymmetric partial ellipse with a short straight return. They do not form two equal half circles. Both are thin strokes in the visitor drawing: do not treat lineweight as surveyed wall thickness.

## Plan Registration

The visible roof perimeter is approximately x=915.3..1035.6 and y=403.7..538.5 PDF points. The source building uses x=0..19m, y=0..21.5m and a 4.75m grid. These imply roughly 6.3 PDF points/m, consistent with the visible bottom column dots being spaced about 29.8 PDF points. A provisional diagnostic transform is x=(PDFx-915.3)/6.3 and y=(PDFy-403.7)/6.3. This is an inferred registration only; fit actual grid dot centers across all floors before publishing coordinates. Do not scale the visitor page as a measured survey.

Under that provisional transform, the right contour extends approximately x=13.7..16.9m, y=0.9..4.9m; its upper straight limb reaches x≈9.34m at y≈2.19m. The left contour spans approximately x=1.20..7.48m, y=1.45..6.03m. These differ materially from current SC-1. The raw native coordinates above are the reproducible deliverable; rounded metric extents are only a registration sanity check.

## Aperture and Arrival

The ramp guide is path6685 translated by (979.6782,454.0899): its two straight sides are local x=0 and x=-7.690, its rounded upper turn reaches local y=58.308; its arrow approaches the lower end. The approximate ramp axis is therefore PDFx=975.8332. The right screen straight limb lies at PDFy=417.7015 from PDFx=974.1598 to1001.5558. Their intersection locates the primary-source-attested aperture axis near the left end of that straight limb. The plan does not resolve aperture jambs/sill/head: width, height and exact station remain unmeasured. A rectangular opening can be known to exist without a defensible numeric size.

The stair drawing is a separate subpath of6685: it begins PDF(946.4372,464.1259), runs to(955.6342,464.1259), rounds around to(955.6342,457.9749), then returns left. Its landing/circulation continues down toward the left screen return; it must remain distinct from the ramp and the right screen. The two disconnected screen ends near PDF(962.3448,418.2105) and(974.1598,417.7015) leave a real diagrammatic circulation interval. Preserve this gap; do not close the contour or bridge it with a guessed straight segment.

## Remaining Source Gaps

Aperture width/sill/head and wind-wall height changes require registered elevation or detail evidence. The current overall9.40m must not be globally replaced by the structural +9.335 leader without identifying its wall and finished datum. Rooflight/chimney rectangles are visible in the visitor illustration but are not named, dimensioned or sectioned sufficiently to identify each object or create solids. The red guide disks and deliberately disabled PDF clipping hide linework; use the original PDF page for final interpretation. A clipping-correct render should precede source publication. No source edits, geometry tests or historical-fidelity claim were made by this audit.

## Tool Limit

repo --help still fails with malformed runtime registry field restart. Filesystem reads used the established Node API workflow. The native path extraction already available in the ticket was inspected directly; no shell bypass, registry modification or source change was used.
