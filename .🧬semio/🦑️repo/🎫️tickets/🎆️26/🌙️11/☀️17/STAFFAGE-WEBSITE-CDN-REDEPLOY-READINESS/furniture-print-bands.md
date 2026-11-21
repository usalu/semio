# Park Bench and Picnic Table print bands

Both 1:50 drawings now start at 1:20. Park Bench continues through 1:55 and Picnic Table through 1:75. The bench has three open 17.1 mm plan joints between the four seat slats. The picnic table has four open 24 mm plan joints between the five top planks. The drawings use 0.13 mm pens in the views that show the joints. The exterior edges in plan, construction dimensions and 3D solids were preserved.

Before the change, the print sweep reported Park Bench gaps below the office laser profile’s 0.15 mm minimum at 1:20 and 1:25 and Picnic Table joints below the minimum at 1:20. Both authored assets also had a 1:20–1:30 coverage gap. After the change, each asset passed `validateAsset`. The exhaustive sweep passed all 36 integer scales for Park Bench and all 56 for Picnic Table. The minimum measured clear gaps were 0.168 mm at 1:55 for Park Bench and 0.190 mm at 1:75 for Picnic Table. [Reproducible print check](./check-furniture-print.ts)

The print check evaluates all named poses and all three views. At the endpoint shared by two bands, it selects the 1:50 drawing explicitly for the measurement and validates the original asset separately. The original construction retains its 10 mm drainage joints. The narrower board outlines are a drafting choice for this print representation.
