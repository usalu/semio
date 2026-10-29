# Orthographic Remodel

Plan, section, elevation and wall-detail drawings now consume explicit authored walls from every storey. Space outlines provide annotations and terrace hatch only. The old room-derived wall synthesis has no remaining orthographic consumer.

## Geometry

A shared pure wall-cell subtraction handles zero, one or multiple openings, including doors, windows and unfilled landscape openings. Wall body cells leave physical voids; cancelled shared edges prevent false masonry joints. Elevations include actual end caps. Opening identities and filling identities remain selectable.

Plans render centred wall thickness, glazed fillings and diagrammatic door swings. Door hinge and swing are model-authored teaching conventions rather than surveyed as-built hardware. Plan cuts use a height relative to the selected storey and no longer depend on whichever opening happens to be first in the model.

Authored stair treads and landings, fixture parts, parapets, rooflights and ramp guards are rendered as model part extrusions in all orthographic views. Sections intersect each part footprint with the cut plane. Roof screens and curtain glazing retain their explicit path geometry. North consumes the shared geographic vector and keeps its label upright.

Wall details choose an authored opening station when available and a solid midpoint otherwise; centred transverse thickness works for arbitrary source wall orientation. Equipment, drain and pipe envelopes now come from the canonical model services[*].parts interface, consumed unchanged by plans and sections. Equipment point elevations define the base; RAD-1 spans 3.60–4.20 m. The model owns footprint, height and pipe-radius normalization.

## Verification

Portable cases in savoye/🧪️ortho-remodel/cases.json cover a solid wall, mixed window/door/unfilled openings and clipped overlapping openings. test_cells.py extracts the production pure wall-cell and boundary helpers and compares occupied area to an independent NumPy raster oracle.

Independent web-tree-sitter 0.20.8 with its installed Python WASM grammar parsed the plan, section, detail and both ortho-remodel test modules with zero error or missing nodes. This validates syntax only.

Portable service cases additionally cover radiator base height, full-height boiler points, roof-drain box and diagonal pipe radius. test_services.py extracts the production model service-envelope function and section adapter, compares bounds to independent NumPy results, and asserts actual RAD-1 bounds 3.60–4.20 m.

Python execution and Cairo visual QA are pending the repository launcher: repo tools exec currently returns unknown command tools, and repo --help rejects the runtime registry field restart. This report records an environmental blocker, not a passing test result. Parent integration will run the focused fixtures and both-language drawing QA when the ingress is restored.

## Files

- savoye/📋️plan/🐍️.py
- savoye/✂️section/🐍️.py
- savoye/🔩️detail/🐍️.py
- savoye/🧪️ortho-remodel/cases.json
- savoye/🧪️ortho-remodel/test_cells.py

## R4 Audit Resolution

Read source-audit-r4.md and coordinated the authoritative service interface with the model agent. Removed local service prism reconstruction. Stair landings are concatenated with steps in plan, section and elevation. New service fixture file: savoye/🧪️ortho-remodel/test_services.py.

## R5 Consumer Resolution

Read consumer-audit-r5.md. Default piloti detail now selects existing C2. Default slab strip row is C because the old row B between grid columns 2 and 4 contains no supports. The strip excludes upper-storey supports, retains actual ground piloti tops in the physical view and interpolates to the slab middle only during idealization. Portable defaults fixture and test_defaults.py independently select actual supports with NumPy.

Plan, section and elevation physical glazing now iterate disconnected curtain_wall.paths, preserving the entrance gap. The nominal path is no longer drawn as glass. SC-FRAME is a STANDARD source wall and uses the generalized opening-subtracted wall code.

Current-model audit found the old named salon cut at x=4.75 outside SP-SALON (x>=4.80); parent owns the shared sheet cut resolver and is resolving this using authored space geometry. All drawing crops otherwise derive their centres and extents from the current 19.0 by 21.5 m box, and ramp cut resolves current flight x=8.65. Independent Python tree-sitter syntax parsing again found zero errors in all three source files and all three focused test files.
