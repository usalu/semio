# Architectural Primitives

The shared drawing vocabulary keeps the existing section coordinates so clips can be revised concurrently. Standing and seated people become one opaque closed silhouette each. Section material becomes closed filled geometry with visible structural thickness; windows get reveals and an oblique projected open sash. The house uses one floor-to-floor scale: 1.2 drawing units for 3 metres; standing people are 0.66 units tall and a normal radiator is 0.26 units tall.

The portable JSON contract and cases are written before the implementation. Geometry tests use Manim as the independent drawing oracle, and raster connectivity is compared with SciPy's image labelling during visual verification. Generated media stays in the ticket's generated folder.

## Verification

The first test run failed all six geometry requirements against the original drawing helpers. After implementation, all nine architectural geometry and raster tests pass. Manim supplied independent shape dimensions and rendered masks; SciPy component labelling matched the local flood fill and found exactly one visible component for each human body. The rendered 1600 × 1000 primitive contact sheet was visually inspected: section materials meet at corners, roof thickness remains legible, standing and seated silhouettes are solid, the radiator has a filled casing, and the casement and bottom-hinged window projections are distinct. Generated preview: `🗑️generated/architecture-primitives.png` (temporary; remove with other ticket outputs when complete).

### Shared API

`room_section(..., window_side="left"|"right"|"both"|"none")` provides inner bounds, wall thickness, split apertures and separate right glazing while retaining a Rectangle for left glazing. `window_glyph(..., wall=..., inward=...)` tracks a closed hidden reference so scaled/moved windows can open repeatedly. `window_opening(..., fraction=..., tilt=...)` returns a target for continuous controls; `open_window` animates that target and supports closing. `radiator(..., height=..., width=...)` accepts explicit dimensions for closeups. House metadata now includes inner bounds, shared storey height and wall thickness.

### Adjacent Glyph Audit

The pendant lamp now uses a closed filled shade and an opaque bulb that overlaps the shade, with a test for material fill and physical connection. The energy tank is a teaching gauge: its separate live fill is intentionally controlled by energy level, and the outline/cap remain legible while empty; no conversion to an opaque solid tank was needed.

Final shared primitive verification: all **10** tests pass, including the lamp regression and the two independent raster component counts. Seated head reference is invisible but transforms with the complete figure, preserving heat/breath attachment metadata.

## Roof Junction Correction

The initial contact sheet exposed a triangular air gap between rectangular wall tops and the pitched roof underside. Both upper wall ends now have slope-cut polygons that meet the underside across their full thickness. Roof thickness remains unchanged. New metadata provides `roof_wall_top` (inner maximum), `roof_wall_outer_top`, and exact per-side `roof_wall_tops` coordinate pairs for chapter-specific material replacements.

A 1200 × 1200 no-stroke material raster test failed before the correction at (-1.765, 1.20465). It now verifies one connected material component with SciPy and samples every roof-wall joint through the entire wall thickness for air gaps. A second test checks exact sloping top dimensions. All **12** shared architecture tests pass, and the contact sheet was re-rendered and visually inspected after correction.
