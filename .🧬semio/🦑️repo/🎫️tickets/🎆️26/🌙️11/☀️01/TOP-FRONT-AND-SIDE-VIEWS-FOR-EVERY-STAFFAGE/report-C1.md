# Report C1: Components

**Assets:** glass-balustrade, glulam-beam, cantilever-balcony, curtain-wall-bay, masonry-wall-with-lintel, panel-radiator, external-venetian-blind, sectional-garage-door.

## State
- Front and plan exist at every authored scale. Balustrade, blind and radiator 1:500 stay unavailable. `presentation.viewpoints` is removed.
- **Builders:** `T/🧰️C1/<asset>.py` and `lib.py`, plus the driver `all.ts`.
- **Check:** ok for all eight and suitable under `office-laser`.
- **Sweep and consistency:** ok at every scale. Two fixes from it: the glulam 1:200 plan axis now uses role `detail`, and the garage-door 1:500 front got its drums.
- **Vitest:** the 17 tests for these assets pass (three-views and front-projection), including the radiator front gradient.
- **Solids:** `extruded` for the blind slats and rail and the garage-door panels.
- **Conventions:**
  - Curtain wall and masonry wall: front and plan are sections through the bay or opening, because their face is the elevation. Section planes and exterior side are stated in the decisions.
  - Balcony: front is seen from the free edge.

## Open (handed to QA13)
- Not inspected: balustrade and glulam at 1:100–1:500, the radiator, and the clip instants (`tilt-sweep`, `open-close`).
- Blind slats are hidden under the box in plan (physically correct).
- Radiator gradient: the threshold is 5 K in the fixture and 8 K in the feature text. FW must align them.
