# Catalog validation during the staffage overview

Loading the 152 authored assets ran semantic validation on the browser thread in groups of eight. The validator evaluates authored drawings at multiple scales. The catalog test took 79 seconds on the shared machine before the build change. This was elapsed test time, including contention, rather than a browser CPU measurement.

The asset module now carries the validator's findings alongside the compact asset JSON and source revision. Vite computes those findings when serving or building the asset. The catalog reads the findings for that asset identity. Injected catalog sources still run validation, and an invalid asset still produces its own unavailable row. A native ESM import attribute lets the build-time validator load the same print profiles as the browser. Compact JSON preserves authored negative zero.

The authored catalog had 51 height or width declarations that differed from the asset documents by more than the validator's one-millimetre tolerance. The manifest now records the asset dimensions. The catalog-loading suite passed all eight tests, including validation of every manifest entry.

The preview grid regression passed all six tests. It checks that two intersecting cells mount two cards in the inert 24-asset fixture and that opening the archive mounts all 24.

## Full React catalog verification

The full React suite with the ticket's `vitest-teaching.config.ts` passed 285 of 286 tests across 30 files. All five payload tests passed. The archive integration check reported 146 ready assets and six invalid assets, with the following findings:

- Hedgehog repeats authored thermal geometry at 1:200 and 1:500.
- Mallard duck repeats its 1:200 structure list at 1:500.
- Red fox repeats its 1:50 thermal emitter geometry at 1:500.
- Hosta repeats thermal and structure geometry at 1:500, and the hidden petiole region references surface geometry.
- Electric kick scooter and electric kettle retain subsets of the 1:200 structural solids at 1:500.

Direct validation of the current sources showed that the scooter and kettle had already been corrected by concurrent edits. The catalog follow-up authored a wider snout patch and coarser dorsal emitter for the hedgehog at 1:200 and 1:500, combined the red fox nostrils into one 1:500 emitter, and simplified the mallard's 1:500 skull loft. For the hosta clump, the 1:200 blades now meet the scale's 30 mm feature size and its drawn root has a corresponding crown solid. Each coarse thermal model uses independent petiole cores inside the body. The 1:500 structure uses a single rounded root crown.

All six assets that failed in the full React run passed direct validation through the Vite asset plugin after those edits. A diagnostic comparison of the hosta's 1:200 solid projection against the drawing found slightly higher overlap in all three views following the feature-size changes. The overlap remains below the separate catalog silhouette suite's 0.92 threshold.

The next archive-wide direct scan passed 139 of 152 assets while additional catalog edits were arriving. The latest findings covered person with rollator, horse, roe deer, Labrador dog, goat, dairy cow, sheep, pig, cargo bike, ivy on trellis, miscanthus grass, multi-stem birch and ceiling diffuser. The subsequent full React run timed out in the archive integration check at its 120-second limit under shared-machine load. It had loaded the pre-fix editor code while that separate regression was still being implemented.

A later full React run found 135 ready entries and 17 invalid entries during continuing work on the asset sources. Its other failure was duplicate imports in the shared-border test file, since removed. The final direct recheck found the four repaired source documents had been replaced by later replica writes. Hedgehog, red fox, mallard and hosta again report previously corrected geometry findings, with additional small features on the mallard body. Scooter and kettle remain valid.

The matching open authoring ticket is [Perfect Handcrafted 3D Models per Scale and Mode for Every Staffage](../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🎫️ticket.json), Claude Code session `811f4228-4005-4343-93ee-0589db33a675`. Its `living-scale-models.py` contains coarse volume replacements for all four models. The script and the new hedgehog and hosta documents have the same 03:59:49 modification time. Its `living-finer-limbs.py` edits mallard and red fox at 1:200 and shares their later 04:00:32–33 write times. The surviving sources are the newer authoring variants. The checked records show no decision rejecting the temporary corrections in this ticket.

| Asset | Validation after the replacement |
| --- | --- |
| Hedgehog | 1:200 thermal matches a subset of 1:50. 1:500 thermal matches a subset of 1:50 and all of 1:200. |
| Red fox | 1:500 thermal matches 1:50. |
| Mallard duck | 1:500 structure matches 1:200. Two body solids at 1:200 are below the scale feature size. |
| Hosta clump | The 1:200 body lacks the drawn root solid and has nine thin blades. Two 1:200 emitter solids are below the feature size. Thermal reuses surface solids for the hidden petioles at both coarse scales. The coarse thermal lists repeat the finer emitters and each other, and the 1:500 structure repeats 1:200. |
| Electric kick scooter and electric kettle | No validation findings. |
