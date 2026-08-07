# Family notation guide v2

| Family | Artifacts | Key terminals |
|--------|-----------|---------------|
| F1 graph | dag, wires, jack, rewrite, flow, sequence | node, port, ARROW/EDGEARROW, chain |
| F2 mesh | lowpoly, procedural*, block*, puzzle*, cad, process3d, remodel | VEC3, halfedge, face, transform |
| F3 sheet | 15 norms, architect, energy | QUANTITY=FLOAT UNIT, clause, verdict |
| F4 canvas | draw, raster, note, layout, present, shooting, forms | stroke, layer, COLOR, box |
| F5 catalog | curate, home, playground | stock, typology, compat |
| F6 text | writer, imperative, playbook, mathematical | statement, fence, expr |
| F7 geo | gismap, gisterrain | POINT, CRS, tile |
| F8 eng | fem2d, fem3d, vcs | node, element, load, support, commit |

Per-artifact 4-byte domain magic + segment kinds + one spr record tag per Operation variant.
