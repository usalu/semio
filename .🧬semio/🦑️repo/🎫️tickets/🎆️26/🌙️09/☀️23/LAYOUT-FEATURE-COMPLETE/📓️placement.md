# Native 2D placement

[Native 2D artifact placement](4cb27671-de17-4243-9f4d-1b69437b2181) stopped with a full agent database and did not finish the drop path.

`ImageLink` now stores `artifactKind` and `artifactRef`. Empty values stay off the wire. The catalogue roster adds `png`, `pdf`, `drawing`, `raster`, and `svg`. Dropping one of those creates an image frame plus a link:

| Drop kind | `artifactKind` |
|-----------|----------------|
| png | `s.stdio.png` |
| pdf | `s.stdio.pdf` |
| svg | `s.stdio.svg` |
| drawing | `s.stdio.semio` |
| raster | `s.raster.raster` |

A ready `proxyDataUrl` paints as a host image. Without a proxy, the canvas shows the artifact kind as text so the link is visible on a large sheet.

`cargo test -p semio-s-artifact-layout-layout --lib -- dropping_a_pdf canvas_layers_labels_a_linked_pdf canvas_layers_paints command_from_action_round_trips` — 4 passed.


## Rest of the 2D set

The catalogue and drop path now share `NATIVE_PLACEMENTS`: png, jpg, gif, bmp, tiff, pdf, svg, drawing (`s.draw.drawing`), dwg, dxf, raster, bitmap (`s.wfc.bitmap`), cad, map (`s.gis.gismap`), and fem2d. Each drop writes `artifactKind` and `artifactRef` on a new image-frame link.

Interactive paint skips frames outside a view span of 2000/zoom around the camera. The selected frame stays. Zoom below 0.35 still omits story text.

`cargo test -p semio-s-artifact-layout-layout --lib -- every_placeable interactive_display_skips dropping_a_pdf canvas_layers_labels` — 4 passed.

Zoomed-in frames without a proxy draw a typed mark (page, stroke, map, curve, or grid) and the artifact kind. Zoomed-out frames keep the box and drop the mark. The document tree shows `artifactKind` and `artifactRef` on each link. `canvas_layers_labels_a_linked_pdf` passed.

The interactive canvas omits the baseline lattice. Accurate display-list builds still include it. `interactive_display_omits_the_baseline_lattice` passed.

A rotated placed image is drawn as a rotated frame instead of an upright bitmap. An unrotated proxy still paints as an image. `a_rotated_proxy_is_not_painted_as_an_upright_image` passed.
