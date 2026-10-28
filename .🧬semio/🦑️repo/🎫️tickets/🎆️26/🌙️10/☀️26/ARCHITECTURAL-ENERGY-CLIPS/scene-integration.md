# Architectural Scene Integration

## TDD Baseline

Before implementation the three scene integration tests produced 12 failed assertions and one missing-dimension error in 170 seconds. Failures established independent physics glyphs, incorrectly normalized overview radiator height, and missing physical room-scale metadata. Tests compare language-neutral dimensions against Manim-computed bounds.

## Implementation

Consolidated six duplicated Physics architecture helpers onto shared definitions. Heating plant people and emitters now use one room-derived metric scale, realistic wall thickness, a closed filled roof and wall-recessed windows. Overview emitters and people now derive final size from storeys. Heating fundamentals uses a 35 cm wall instead of a 1.17 m wall, and physically dimensioned occupant and radiator. Updated envelope door, north-facing window, and recap surface consumers to use the new solid material grouping.

## Verification

Targeted scene integration tests and representative render review are in progress.

## Additional Render-Driven Corrections

The reviewed plant preview exposed an oversized boiler, pale section material, and a tank fill detached by group transforms. Boiler, tank, closed flue and hydraulic endpoints now derive from the same room scale. Shared live-fill geometry follows the transformed frame. The final unique-path 1600×900 overview was independently accepted.

Actual source-frame review then found the remaining Physics pendant-lamp duplicate and ventilation occupants inside the sash swing. Four additional Physics helpers now use canonical shared implementations, including the connected filled lamp and transform-safe tank. Ventilation occupancy is laid out from actual open-pane bounds with 0.12 m clearance, preserving adult scale and count. Both feature cases failed before their implementation. The complete eight-method scene suite passed in 5.782 seconds after these changes.

The aggregate authoring suite passed 82 tests before these final render-driven additions (one existing Windows symlink skip). A final aggregate run will include the new occupancy, visibility and caption regressions. Complete five-video composition is in progress; the redundant partial dry sweep was superseded by this stronger real render path.

## Occupancy Emission Integration

The full renderer exposed a remaining use of the old occupant-coordinate list in moisture droplets. A test executing the actual source scene-setup assignments reproduced the NameError before the fix. The coordinate list now derives from placed person geometry, so emission clusters follow the same layout. All nine scene integration methods passed in 5.935 seconds; the repaired actual source then rendered successfully and its head/pane clearance was independently confirmed in six refreshed frames.

## Solar Annotation Clearance

Actual source 86 footage showed the gross-area label crossing the ceiling and the final wattage crossing the right wall. A production scene-assignment test with independent Manim bounds reproduced three clearance failures before implementation. The area, live factors and result now use one right-aligned interior column derived from room bounds. All ten scene integration methods passed in 6.076 seconds afterward. The refreshed actual source is awaiting its normal cache-resume render and independent footage check.
