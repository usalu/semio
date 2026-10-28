# Final Source Audit: Architecture Optimization Videos

Read-only source audit on 2026-10-27. Source files were not changed. This audit report is the sole write. Actual rendering and output inspection belong to the parent agent.

## Scope

Read root AGENTS.md, curriculum-audit.md, six canonical scene files, both visual modules, stdlib model and JSON data, six standalone wrappers, root playlist, shared clip manifest, and relevant portable test assertions. All successful reads used repo tools exec --light node.

## Findings Sent for Resolution

1. Decision visual chromosome name is literal child in both languages. Localize to Kind in German.
2. Index stacked contributions depend on segment colors without criterion labels. Add redundant visible identification or numeric contribution labels.
3. Solver history x label Evaluations / generation mixes quantities while x=0 is the four-candidate initial population. Label Generation (0 initial, 1 first child, 2 repeated child) or plot actual cumulative evaluations.
4. Pareto guide is a solid connecting line between unevaluated intermediate designs. Narration already explicitly states the line is only a visual guide. A dashed guide and visible sampled-only label would reinforce this in the caption-free audience.

These were sent to author_decisions while authors were active; final resolution must be checked after that agent finishes.

## Correctness Checks

- Quartic derivative is x(x-2)(x+1), so local minima -1 and 2, intervening local maximum 0, and global minimum x=2 over [-1.5,2.5] agree with narration. f(2)=-8/3; constrained x<=0.5 selects -1. Negation preserves design location and reverses preference correctly.
- Full grid has 3 squared = 9 points. LHS permutation uses each of nine marginal strata exactly once on each axis. No assertion claims one sample per 2D cell or a certified optimum.
- DOE responses 9,5,7,19 match 10+3A+2B+4AB. A main effect is 6; conditional B effects are -4 and 12; their half difference is 8. Curvature and independent-noise caveats are accurate. Intermediate low-B confirmation value is 8.
- Fixed-reference normalization gives A approximately 0.467, B 0.333, C 0.400 under weights (0.5,0.3,0.2). Changing to (0.7,0.2,0.1) gives C 0.250 as claimed. Daylight direction is reversed consistently; bounds caveat and compensation limits are narrated.
- Pareto A=(100,50), B=(120,35), C=(80,65), D=(140,60), E=A support the narrated comparisons. Equality does not dominate. The model computes nondominance over all three criteria whereas this scene projects two; the current data give the same set, but future data edits should preserve explicit criterion scope.
- Genetic parents P2=(0,0.5), P3=(0.5,1) each score 0.125. Their midpoint (0.25,0.75) scores zero; mutation produces (0.30,0.70) with score 0.005. Bounds, reevaluation, deterministic elitist record, plateau, seed/restart and model-validation caveats are accurate.

## Language, Accessibility, Routing

All six scene titles and narration clauses have complete en/de pairs. ClipScene requires explicit SEMIO_TEACHING_LANGUAGE en or de; standalone compose functions require language. German decimal formatting is centralized. Scene labels, candidate IDs, point shapes and narration provide redundant meaning in extrema, sampling, DOE and Pareto; index segment identification needs the above fix. Caption timing uses shared measured clause holds. Shared registration names six independent video IDs, one chapter each, separate output names and the intended canonical source scenes. Wrappers route to their own single video; root playlist intentionally selects all six. Existing README explicitly discloses silent captioned output when speech credentials are unavailable.

## Validation Limit

This is source review, not a claim that rendered frames, captions, screen boundaries, spoken audio or delivered file encoding have passed inspection. Parent must inspect actual render outputs and rerun checks after author fixes. No source tests were executed by this audit agent.
