# Independent Roof Plan Pixel Audit R48

Read-only product audit, 2026-10-29. Parent owns ticket lifecycle. Inspected actual 737×869 author roof bitmap, not preview dimensions. No model geometry or source edited.

## Registration

Independently checked the visible grid against full raster and candidate enlargements. Five controls per direction: u=[8,189,369,550,730], v=[51,232,413,592,773]. Grid spacing 4.75m; world X decreases rightwards; Y increases downwards. The world ordinate of the first horizontal grid is1.25m, based on existing column-grid correspondence, not inferred from the hollow square. Least squares gives u=8.2+180.5i, v=51.4+180.4j. Residuals u=[−.2,.3,−.2,.3,−.2]px, v=[−.4,.2,.8,−.6,0]px. Consequently X=(730.2−u)/38; Y=(v−3.9263158)/37.9789474. This is materially the same as the proposed38px/m registration. These residuals quantify raster grid consistency only; they do not establish drawing accuracy against the building.

## Hollow Square Boundaries

Inspected nearest-neighbour crops and thresholded grayscale rows (threshold100/255). Salon outer ink u228..242,v43..57; inner dark boundary u232..238,v47..53. Kitchen outer ink u689..703,v199..213; inner dark boundary u693..699,v203..209. Both show the SAME concentric-square construction and a white central void. Outer stroke is approximately1–2px thick, inner approximately1px. Thus total ink bounding box is not the masonry surface boundary. The centerline of the outer boundary yields about13px or0.342m; inner boundary about6px or0.158m; central clear white about5px or0.132m. A defensible rounded reconstruction is0.35×0.35m masonry outside and0.15×0.15m hole, each direction uncertain roughly0.03–0.04m. The raster cannot justify a different width and depth from15 versus14px antialias extents.

Salon center u235.5–236,v50–50.5 maps X13.006–13.018,Y1.213–1.226m. Kitchen u696–696.5,v206 maps X0.887–0.900,Y5.321m. Recommend nominal axes salon(13.01,1.22), kitchen(0.89,5.32), with practical registration uncertainty±0.05m, and state reconstruction basis. Parent candidate(13.0,1.237)/(0.882,5.329) is within this uncertainty.

## Identity and Construction Limits

The nearby salon column-grid symbol at u189,v51 is grey, circular, with cross/axis lines. The candidate at236,50 is a separate hollow-square symbol about46.5px=1.224m horizontally from that column axis. This independently rejects the interpretation that the candidate merely IS the adjacent column axis. Other roof structural remnants near bottom raster use thin small rectangles surrounding grey circular points, visibly different from the thick double-square symbols.

Morphology alone does not label function. Pair the salon square with the official author-book p117–119 discussion of the salon chimney adjacent to column2; pair kitchen square with p119 executed kitchen flue against the other curved solarium wall, absent from planned FLC19443 but present in FLC19572 and original executed photos. R39/R44 original-publication photographs support presence of terminal silhouettes but their7–15px widths cannot independently distinguish these footprints or assign height. This author2024 drawing bears Savoye1930/FZaparaín imprint and is reconstructed plan evidence, not an original FLC drawing or surveyed as-built terminal measurement.

The double-square plan supports a hollow prismatic representation. Rendered hollow-brick construction with reinforcement is supported by the author-book account summarized in prismatic-chimneys-r44.md; this bitmap does not expose separate brick leaves, reinforcement, liner thickness, cap, internal partition, base penetration shape or height. Do not invent a cap or turn line weight into exact masonry thickness. A0.35m outside/0.15m hole reconstruction implies0.10m nominal wall, with no claim that this raster measured it precisely.

## Recommendation

Accept candidate XY within±0.05m as author-drawing reconstruction after parent review; normalize both outer footprints to0.35m squares and0.15m square holes if schema supports that authored fidelity. Keep source classification as reconstructed author drawing corroborated by functional text and executed photograph presence. Do not promote the unverified three-tall/two-low lead. No global roof translation follows from this audit.
