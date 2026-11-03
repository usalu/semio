# Report C2: Components

**Assets:** steel-column, sliding-door, tilt-window, tied-canopy, wheelchair-ramp, stair-flight, spiral-stair.

## State
- Front and plan exist at every authored scale; tilt-window 1:500 stays unavailable. `presentation.viewpoints` is removed, and the en/de silhouette decisions are extended. Writes are atomic.
- Builders: `T/🧰️C2/*.py`, built from the pristine originals in `orig/`. Driver: `all.ts`.
- **Final round:** the consistency sweep (every pose plus six clip samples) is clean for all seven. The check is suitable under `office-laser` at every authored scale.
- **Fixed:** an inverted `>=` scale gate (stair-flight stringer tone, plus the hatch leaking to 1:200 in tilt-window and wheelchair-ramp). The tilt-window front is redrawn at 0.90 m with no outer handle.
- **Solids:** the tilt-window sash members are `extruded`, so the tilt reads in front and plan.
- **Conventions:**
  - Stairs and ramps use the stair convention in plan from 1:200 (walking line, start dot, arrow).
  - The sliding-door plan is a section at 1.0 m. In front its leaf is hidden behind the jamb and drawn as thin hidden outlines at 1:100 and 1:50.

## Open (handed to QA14)
- Not re-read: the stair-flight front, the spiral 1:100/1:200 sheets and the final stair-flight plan.
- Previews were inspected only at selected scales.
- The full suite was not run.
