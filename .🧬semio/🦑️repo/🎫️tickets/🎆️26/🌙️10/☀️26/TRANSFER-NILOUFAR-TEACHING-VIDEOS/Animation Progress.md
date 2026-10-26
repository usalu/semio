# Long Segment Animation Progress

Read-only AST inspection of `AnlagenVerluste.construct` finds 19 direct `self.play` calls, eight `hold_for` calls, and seven `swap_caption` calls. The actual shared helper implementation produces one top-level animation for each hold and two for each caption swap. This gives 41 top-level animations, indexed 0–40; nested animation groups do not add top-level indices.

The observed raw-68 animation 23 is the storage ValueTracker transition, consistent with the expanded source order. Animation 29 is the generation-loss narration hold. After animation 29 completes, 11 top-level animations remain:

| Index | Authored Action |
| ---: | --- |
| 30–31 | Swap to the final-energy sum caption |
| 32 | Animate the final-energy readout |
| 33 | Transform the loss terms into the formula |
| 34 | Reveal the remaining formula and its panel |
| 35 | Hold the sum narration with running heat/flow |
| 36–37 | Swap to the insulation-tip caption |
| 38 | Add pipe insulation while heat/flow continues |
| 39 | Hold the tip narration |
| 40 | Fade out the final caption |

This inspection neither executes nor changes the scene. Existing frame progress and generated SVG writes confirm that the long-running segment is advancing. Raw 80 completed normally during its observation window.
