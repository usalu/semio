# Cooling Render Repairs

The fresh complete render exposed three authored-scene blockers, all in the cooling video. Source 69 (Office Room) and source 78 (Solar Irradiance) attempted to morph house geometry containing shared drawable aliases. Native Manim failed at Transform.begin because the aligned family cardinalities differed. Clearing source/target updaters alone reproduced the same failure.

Independent tests proved that an in-place snapshot of each unique drawable’s own points, held in independently owned updater-free clones, preserves root identity and exact Cairo pixels while giving Manim a tree suitable for interpolation. The new neutral helper is authoring/clip/animation/freeze_drawables; only the two affected source owners import it. The office boundary freezes facade and room shell; the solar boundary freezes only the retiring building so cube irradiance callbacks remain live.

Source 86 (Sorption Cooling) passed two-component positions to a native three-component placement API. All three authored microbe positions were corrected with explicit z = 0. No compatibility fallback was added.

The four focused permanent cooling tests passed in 4.982 seconds after the authored edits. Their independent controls reproduce both alias failures, require exact source/target Cairo endpoint pixels, test translucent alias ordering without duplicate alpha, retain live cube face updates, and verify every actual microbe call against native 3D placement. The helper and fixtures are owned by the independent verifier; shared fonts and visuals remain unchanged.

Normal production recovery selects cooling and energy_balance under one 16 GiB/eight-CPU grant, eight jobs, 480 × 270 at 4 fps. It refreshes stale source owners through production guards and rebuilds chapters. Every generated source artifact is retained until that recovery succeeds; no success certificate is edited or copied.
