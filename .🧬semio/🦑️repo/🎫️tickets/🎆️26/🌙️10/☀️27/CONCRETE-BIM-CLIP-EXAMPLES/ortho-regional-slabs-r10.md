# Regional Slab Orthographic Integration R10

Plans, sections, elevations and structural strips now consume slab.parts outlines and z0/z1. Root thickness is never physical geometry. Regional through-voids remain open. Sections cancel decomposition seams while retaining actual stepped soffits; elevations subtract neighboring occupied intervals to emit exposed side faces only. Strips retain each region's actual thickness and use its local middle surface during idealization.

Canonical slab.finish_parts paint at finish rank in plans and at exact heights in sections/elevations. There is no blanket soil or minimum layer inflation. Structural and finish parts occlude each other so their internal interfaces are hidden. View bounds include actual regional and finish elevations. Ramps and landings use their separately authored thickness fields.

## Verification

Ten focused tests passed in Python WASM with NumPy/Shapely. New portable slab-regions.json deliberately sets root thickness to 999 and supplies mixed 0.20/0.60 m occupied regions with a through void. Checks compare section area, cropped strip area, seam-free perimeter and exposed vertical face area against GEOS. A production structural-strip consumer test verifies actual segment widths/depths through a geometry-recording drawing interface. Roof finish tests exercise the production model helper at 0.09 and 0.14 paving reference thickness and independently verify volume change and exterior interfaces.

The actual-source role test currently fails because the draft source has no roles field; the coordinator will run it after publication. No native Manim frame or render success is claimed.

## Files

- savoye/📋️plan/🐍️.py
- savoye/✂️section/🐍️.py
- savoye/🔩️detail/🐍️.py
- savoye/🧪️ortho-remodel/slab-regions.json
- savoye/🧪️ortho-remodel/test_slab_regions.py
- savoye/🧪️ortho-remodel/test_profile_consumer.py
- savoye/🧪️ortho-remodel/test_profiles.py
- savoye/🧪️ortho-remodel/test_services.py

The service source check now derives bounds from published levels rather than assuming an old teaching datum. Existing portable fixtures retain their explicit example values.
