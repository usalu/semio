## Layer order

**What the order means in the solver.** `ThermalLayer` is documented "innermost first". `regionSetup` takes the emissivity and the solar absorptivity of the **last** layer's material (`region.layers[length - 1]`, else the region's own material): the last layer is the exposed outer surface. The thermal resistance is the sum over all layers (`layerResistance`, `layerVapourResistance`) and does **not** depend on order, nor do the film, the conduction `(T_interior - T_s)/(ΣR + R_film)` or the Gagge skin-to-surface resistance. Further users of the order: the nested-shell thermal view (`🧊️volume/🌡️thermal`: the outermost shell, the last layer, shows the surface value, the inner shells the skin value beneath), the validator rule that compares the area with the solids of the outermost layer (C1), and the 1:50 `volumes-layers` rule. For a body the first layer is next to the skin (air gap, underwear), the last the outer garment; for a conditioned envelope element with two air sides the **first layer faces the interior** (the side of `conditioned.interiorTempC` and its interior film) and the **last layer faces the environment**. Consequence: only where the first and the last material differ in emissivity or absorptivity does a reversed order change a result (the surface temperature through radiation and solar gain, and the surface-driven emitted radiation).

**Audit.** All 195 regions with two or more layers were compared with `git show HEAD:<path>` (`areareview-layers.py`; data `🗑️generated/areareview/layers.json`) and every order was judged physically.

| asset | region | committed order | order now | verdict | effect |
|---|---|---|---|---|---|
| bar-stool | seat-pad, backrest | leatherette, foam | foam, leatherette | correct (cover outside) | outer material hr-foam -> pu-leatherette: emissivity and absorptivity change |
| height-adjustable-desk | top-board | laminate, board | board, laminate | correct | same material both layers: none |
| office-swivel-chair | chair-fabric | fabric, foam | foam, fabric | correct | same material: none |
| operating-table | table-pad | vinyl, foam | foam, vinyl | correct | same material pad-foam: none |
| single-bed | mattress | quilt, foam-core | foam-core, quilt | correct | cold-foam -> quilted-cover |
| three-seat-sofa | seat-cushions, back-cushions, arm-pads, base-rail | fabric first | core first, fabric last | correct | hr-foam / polyester-batting / birch-plywood -> upholstery-fabric |
| curtain-wall-bay | glass-centre, glass-edge | pane-out, cavity, pane-in | pane-in, cavity, pane-out | correct (pane-in faces the interior) | glass both sides: none; the nested-shell display and the outermost-layer area follow the exterior pane |
| curtain-wall-bay | mullion, transom | cap, break | break, cap | correct (cap exterior) | aluminium both: none |
| chalkboard | board-face, leaf | enamel, core | core, enamel | correct (enamel faces the room) | mdf-core -> enamel-board (the committed order gave the enamelled board the emissivity of its MDF core) |
| city-bus, tram | body-panels, roof, doors | skin (paint), insulation (interior-trim) | **insulation, skin** | **wrong in the committed catalog and the working tree, fixed** (exterior paint last) | outer material interior-trim -> paint: emissivity and absorptivity of all vehicle shell regions, solar gain of roofs |
| tower-crane | cab | steel-s355, lining (machinery) | **lining, steel-s355** | **wrong, fixed** (the 3 mm steel skin is the exterior, the 30 mm lining the interior) | outer material machinery -> steel-s355 |
| seated-office-worker | chair-fabric | upholstery, foam | **foam, upholstery** | **wrong, fixed** | same material: none |
| etics-brick-wall-12/-30, solid-brick-wall-1960, masonry-wall-with-lintel, cantilever-balcony, external-venetian-blind facade, tied-canopy wall-face/wall-plate, tilt-window, sliding-door, sectional-garage-door, spandrel, diesel-generator enclosure-walls, vehicles (paint over metal), all clothing and plumage regions | | | | correct as they are (interior first: plaster, brick, insulation, render; clothing from the skin outwards; metal under paint) | none |

The two-layer glass/air/glass, steel/cavity/steel and sandwich units are symmetric: order has no effect there.

Fixed with `areareview-layerfix.py` (text surgery reordering the layer objects, JSON-compared; also in the volumes sources it needs none, thermal solids name their layer by id).

**Pins that move because of the confirmed changes** (emissivity or absorptivity of the exposed surface changed; nothing else did): thermal results of bar-stool, single-bed, three-seat-sofa, chalkboard, city-bus, tram and tower-crane cab: the catalog-wide tests `🌡️catalog-physics` and `🌡️thermal-span` (inequalities, not expected to fail), `🌐️thermal-multiview` and `🧊️chain-state` (tower-crane), `🏷️cast-provenance` and its fixture (chalkboard), `🎨️thermal-legend` (scale ranges of any scene containing these assets); no fixture pins a number of these assets as far as the search of `S/🧪️tests` and `S/🧫️fixtures` shows (the curtain-wall fixture `🧫️fixtures/🏗️curtain-wall-bay` defines its own regions, and its pins do not move because of the unchanged materials).
