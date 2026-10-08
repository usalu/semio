# 🗺️ BIM Waves M / I / U / X — Assignments

Golden leaves from Wave F (f1): create-site, delete-site, create-building, delete-building, create-storey, rename-storey,
set-storey-height, set-storey-level, delete-storey, create-wall, delete-wall, set-wall-top; inferences storey-levels,
wall-layout (heights). Recipe: `r3-golden-leaf.md`.

## Wave M — mutation leaves (each agent: full triads + fixtures + tests + sum law + payload schema + descriptor +
enum variant + KINDS + mount + oracle catalog rows + `🥒️.feature` rows)
| Label | Kinds |
|---|---|
| m-materials-layers | create-material, delete-material, set-material, create-wall-type, delete-wall-type, set-wall-type, create-slab-type, delete-slab-type, set-slab-type, create-roof-type, delete-roof-type, set-roof-type |
| m-profiles-openings-types | create-column-type, delete-column-type, set-column-type, create-beam-type, delete-beam-type, set-beam-type, create-window-type, delete-window-type, set-window-type, create-door-type, delete-door-type, set-door-type |
| m-context | set-project-info, set-site, set-building, create-grid-line, delete-grid-line, set-grid-line |
| m-walls | set-wall-axis, set-wall-base-offset, set-wall-type-of, set-wall-location, flip-wall, split-wall, create-curtain-wall, delete-curtain-wall, set-curtain-wall |
| m-frame | create-column, delete-column, set-column, create-beam, delete-beam, set-beam |
| m-horizontal | create-slab, delete-slab, set-slab-boundary, set-slab, create-roof, delete-roof, set-roof-footprint, set-roof-shape |
| m-openings-stairs | create-opening, delete-opening, move-opening, set-opening, create-stair, delete-stair, set-stair |
| m-railings-spaces | create-railing, delete-railing, set-railing, create-space, delete-space, set-space |
| m-multi-data | move-elements, rotate-elements, delete-elements, rename-element, set-element-property, remove-element-property, set-element-classification, remove-element-classification |

## Wave I — inferences (`🧬️schema/💡️inferences/<emoji><slug>/`, fields of `ModelInference`)
| Label | Fields |
|---|---|
| i-walls | `🧱️wall-layout` complete: location-line offsets, left/right face curves, L/T/X joins (miter/butt), join-trimmed footprint loops, curtain-wall layout |
| i-openings | `🪟️opening-frames` |
| i-solids-walls | `🧊️element-solids` for walls (with opening voids and reveals), curtain walls (grid panels + mullions), opening fillers (window/door frames, glass, leaves) |
| i-solids-rest | `🧊️element-solids` for columns, beams, slabs (holes, slope), roofs (flat/shed/gable/hip/mansard + overhang), stairs, railings |
| i-spaces-quantities | `🏠️spaces`, `🧮️quantities`, `🪜️stair-runs` |
| i-plan-diagnostics | `🗺️plan-linework`, `⚠️diagnostics` |

## Wave U — UI
| Label | Scope |
|---|---|
| u-editor | editor app: modes, plan/world/section windows, presence, configs, transient, terminology, keybindings, panels outliner/properties/library/schedule, selection domains |
| u-tools | utilities + gesture interactive jobs (wall line/arc, curtain wall, column, beam, slab, roof, window, door, opening, stair, railing, space, grid, measure, select/move/rotate/curve-handle) |
| u-viewer | viewer app (view mode, world + plan windows, camera) |

## Wave X — IO / examples
| Label | Scope |
|---|---|
| x-ifc | IFC 2x3 export (geometry, spatial structure, openings, Psets, quantities, materials) + import; ifcopenshell oracle |
| x-gltf-svg | glTF 2.0 export of element solids, SVG plan export; three/gltf oracle |
| x-examples | `🏡️house`, `🏢️office` examples (+ demo upgrade), example tests via inferences |
