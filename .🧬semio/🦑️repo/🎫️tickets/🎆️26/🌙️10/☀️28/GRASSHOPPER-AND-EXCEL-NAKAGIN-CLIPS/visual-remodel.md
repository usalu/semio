# Detailed Nakagin Visual Assembly

> **Final evidence note — supersedes earlier progress wording:** The measured count evidence is 140 capsules, split A 76 / B 64, with documented per-floor totals. Capsule poses and layout families are reconstructed from native CAD fixtures and authored assumptions, including inferred omissions; core and roof heights are assumed/derived as tagged in the shared model. Earlier phrases such as “surveyed-layout crown clearance” or “measured handed geometry” describe intermediate implementation work and do not claim survey accuracy. Reference interiors and proposed A–D program studies remain separate.

The renderer will consume named solids from the shared model. Capsule exterior uses the physical aperture, rim, glazing, rails, rear attachment and shell. Plans retain wall hatches and dimensions and project the shared interior furniture. Tower views use exterior level of detail and globally order individual visible faces by full camera depth; semantic layers no longer override physical depth. Named component groups remain references to their rendered faces for existing animated highlights.

Validation targets: independent tier/depth ordering, z contribution to axon depth, shared part visibility and dynamic capsule dimensions, 140 capsule groups and semantic detail in core/capital/podium.

## Gallery Audit

Initial gallery generated at `🗑️generated/nakagin-detail-gallery.png`. Audit revealed tessellation seams should have zero stroke and enclosed furniture must be omitted from opaque axon renders (plans retain full furniture). Core full-height wall faces require storey bands for physically correct occlusion with plates; requested shared model refinement. Building shell LOD now coalesces corner sectors and omits subpixel aperture tunnel surfaces. Bridges will be included from structure_assembly.

## Verification

Renderer AST parsed through the repository Python ingress. Native dependency environment smoke constructs all assemblies, confirms 140 named capsule groups, checks monotonically increasing camera depth, and verifies clones retain capsule mappings after position/scale changes. Full-detail tower currently has 5534 visible drawing objects after doorway subdivision. Cached template clone plus reposition/scale takes approximately 0.8 seconds (raw clone approximately 0.34 seconds); cold construction approximately 4.8 seconds. An eight-entry immutable tower cache avoids rebuilding geometry during common live redraws.

The core override cache key includes tower index, storeys, side, storey height and height, so every live core parameter regenerates the model assembly. Capsule axon supports an explicit cutaway that removes the top and nearest long wall while retaining the same detailed furniture solids used in plans.

## Measured Layout and Teaching Corrections

The measured count correction supersedes the initial 70/70 assumption: A has 76 capsules and B has 64. Grasshopper tower/tree scenes and bilingual transcripts now use cumulative measured floor sums, one-or-two-item occupied branches, source-derived probe addresses, dynamic example levels and actual nullable Flip Matrix transposition. Null addresses draw as empty circles. The tower cache will consume world-placed slot assemblies through cached coordinate conversion, preserving the model layout family and handedness.

The partial-tower capital is translated from the shared detailed assembly to the requested partial core top; the full building retains its surveyed-layout crown clearance. Capital cache keys include all live height/overhang/taper/core-side controls.

Latest detailed per-slot model produced 9076 visible objects and a 36-second cold construction when individually rebuilding all world solids. The renderer now shares canonical local meshes across identical parameter/layout families (equivalent to the model slot_assembly composition) and performs each placement once during face emission. This retains measured handed/side-window geometry while avoiding 140 repeated tessellations.

The Grasshopper tower presentation was rebuilt around native solved canvas views. Its ghost candidates now come from the unmasked final Orient body planes, rather than the intermediate door-port Move frames. Bilingual transcripts are regenerated from all five current scene NARRATION tables, eliminating stale historical totals and window-size claims.

The original uncached tower lifecycle run was stopped after 411 seconds at the parent request because it imported an obsolete assembly path. Its replacement input prints a [DEBUG] progress line per narrated step. Final lifecycle and gallery verification wait for the reference/study interior model API to be saved, avoiding testing a deliberately incomplete dependent contract.

Final narrative audit uses “documented counts” for the measured evidence and “authored/reconstructed” for frame placement and masks. The capsule chapter distinguishes four proposed room-program studies sharing a 1.30 m centred porthole from independent dimensional slider studies. Original reference interiors and proposed program interiors are separate rendering arguments.

Partial towers now select capsule levels by the actual slot level index, and derive their core/head height from the selected capsule crowns plus the documented service overrun. They no longer infer datum Z from nominal podium-storey count times pitch. Full cores render from ground so the open pilotis exposes the proper core walls. Full default head height remains authored.

A stale lifecycle snapshot reached the tower-colour step and revealed an undefined ARCH palette alias; this was fixed. Native authored labels are now localized in a copied display definition for German, including capsule datum, crown offset and service overrun; length fields explicitly show metres. Engine definitions and IDs remain unchanged.

Caller audit found program-colour views already pass explicit types/tints, while omitted types are reference views or selection studies. Tower axon/elevation now default to off-white physical shell materials; proposed program colours require an explicit option. Missing hardcoded historical slots in Anatomy/Capsule were replaced by authored occupied slots, variable branch rows and real per-floor representative capsules.

Tower-only core LOD now retains exterior concrete walls with full opening reveals/sills, roof and ventilation while omitting enclosed lift walls, doors, internal landings/floor strips and risers. It culls inward-facing opaque core surfaces. Standalone core drawings retain all previously available components, and model assemblies remain complete.

Final core LOD comparison gallery preserves the exterior silhouette, door recesses, bridge connections and roof services while reducing the current building from 6263 to 4921 objects. Cold drawing construction: 7.81 seconds; warmed clone and placement: 0.391 seconds. All 140 capsule groups and monotonically increasing full camera depth passed. Reference material defaults are off-white, with proposed program colours explicitly enabled only in study views. The pre-window Boolean teaching state now closes the aperture with a matching shell face instead of leaving an unglazed hole.

Program-study cache audit found and fixed dropped type identity: canonical numeric dimensions are shared, but full study mode must also cache and forward the program key to select its fittings. The smoke input now requires A to contain a bed and C a sofa without a bed. Reference and exterior caches continue sharing identical physical geometries across program labels.

The new portable `study_variants` cases and `test_program_study_cache_keeps_type_identity` passed against actual Manim cutaway faces (3.052 seconds). Final native scene validation is partitioned into fresh language processes to bound retained Manim cache memory; the earlier combined run completed DataTrees English before the repository runner terminated externally during German without a Python traceback.

Fresh final Tower English native lifecycle passed all authored beats and final Cairo with zero errors: 643.828 seconds. The German lifecycle starts in a separate new process, using current labels, program cache identity and datum/crown graph.

## Final Native Tower Evidence

Fresh language-isolated validation completed every authored Tower clause, native animation setup/interpolation/cleanup, finite geometry checks and final Cairo capture. English: 643.828 s, zero failures/errors. German: 668.944 s, zero failures/errors. Durable generated results: `scenes-grasshopper-Beat3_Tower-en/result.json` and `scenes-grasshopper-Beat3_Tower-de/result.json`, both success true. These results are preserved here before generated-output cleanup.

The final complete visual suite passed all six tests in 20.992 seconds: full camera-depth including height, shared named capsule geometry, live core dimensions, distinct A/C study interiors, cached whole-tower transforms/depth ordering and an actual pixel difference for the open/closed window toggle.

## Four-Direction and Section Audit

The final projection-only audit completed successfully and was visually inspected at 1920×1080. `nakagin-four-exterior-directions.png` shows N/E, N/W, S/W and S/E shared-model views, each with 140 named capsule groups and physical off-white materials. `nakagin-representative-sections.png` shows the original-reference capsule cutaway, a full-detail core slice at Z 7.5–15.0 m exposing stairs/landings/lift/services, and a longitudinal podium cutaway. The section script clips existing shared meshes only; it does not change the production model or renderer API. Both transient images reside in the ticket generated folder for cleanup after audit.

The initial Casting English handoff produced no final result and its process was no longer available. It was rerun as a fresh isolated English process; the completed result is recorded below.

## Final Native Casting English Evidence

Fresh English-only Casting validation completed successfully in 192.976 seconds (session 59355, exit 0). All authored beats, native animation lifecycle and final Cairo capture passed. The durable `scenes-grasshopper-Beat4_Casting-en/result.json` records one test, zero failures, zero errors and `success: true`; `checks.jsonl` records both the toolkit Probe and Beat4_Casting English as successful. No production source changes were made for this validation.

## Final Bridge Visibility And Lower Construction Views

Display audit found that the bridge adapter discarded the newly modeled bases, V-frames, posts, crossbars and L-returns. A portable role/actualvertex regression ran red; the adapter now consumes every part from each dedicated bridge assembly, avoiding a needless fullstructure rebuild. Lower-only counts0/1/2 also ran red: the former global-crown fallback drew an11m core and fullhead. They now produce no core/head at0, cores ending at actualoffice4.00 or flatroof7.72 at1/2, no roof vent/head beforecapsules, with independentpodiumtoggle. Emptydrawings/ground toggles are render-safe. NumPy actualcorewallbounds and exactbridgevertices provide independent geometry checks.
