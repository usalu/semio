# Architecture Wave R8

## Implemented Source Features

Portable architecture cases were authored before implementation in model/🧪️tests/🏛️architecture.json. A separate service entrance now sits in a real left-side faceted bay beside the helical stair. Its0.9m single doorway interrupts curved glazing rather than placing a door behind continuous glass. The exact position is source-informed and inferred, not claimed as a measured as-built coordinate. The documented hall basin and tap were relocated together within the vestibule so the service portal has a true0.8m body-width approach and the main double-door route remains clear. The curtain wall now exports3 distinct glass paths.

Guest parquet and tile surface regions follow the integrated wardrobe partition: the tiled dressing/washing zone below its north edge and the parquet sleeping zone above it together exactly partition the guest floor. Four new materials have bilingual names and assumed densities; all finish products retain stable identities and contain/associate relations.

The hanging garden has concrete paving with grass joints on an explicitly assumed1.2m module and0.04m joint width. Parts are clipped to the real terrace floor and actual garage rooflight holes. Independent polygon-clipping union/intersection checks establish90.138m² of paving and6.412m² of grass over96.55m² of actual floor, with zero overlap, missing area or spill outside the floor. No opaque paving fills a rooflight.

All finish solids are declared nominal15mm surface representations capped at the finished-floor datum (first-floor3.485..3.5), excluded from structural material and load membership. They do not claim exact slab material layering or add a door step. New generic detail kinds are parquet, tile, paving and grass_joint.

Four independent Shapely tests in model/🧪️tests/🏛️architecture.py cover actual service-portal free movement against walls/glass/columns/fittings, guest finish partition and wardrobe relationship, exact paving/grass floor coverage, and nominal datum/structural-exclusion semantics. Coordinator execution is pending.

## Primary Evidence Under Review

The CSIC2022/FLC measured archival plans now provide stronger structural evidence than the CMN schematic. The19×21.5m envelope and4.75m reference grid with1.25m long-axis overhang agree with normalized annex3 proportions. Positions of actual remaining parents alcove posts and the kitchen patio will follow these plans; new structural posts will not be invented from a feature count. The existing inferred private circulation shape may require source topology refinement once these locations are resolved. The definitive narrow cellar plans and their240cm headroom are being inspected separately; no whole-footprint basement is being invented.
