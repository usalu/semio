
# Final Physics Preview Render Issue

## Diagnosis

The optional final preview failed in Physics Beat1 at the first house FadeIn (scene line 739 in the logged attempt). The root cause is the concurrently introduced architectural section assembly updater, not the primary accent changes.

Current house_section returns a group containing section_assembly. section_assembly attaches _update_section_assembly as a live updater. That updater selects only pieces whose fill opacity is exactly 1, rebuilds a merged VMobject from them, and replaces the composite points. Manim FadeIn creates a starting copy faded to opacity 0. Updating that copy therefore removes the composite's point geometry. Manim Transform subsequently strict-zips family_members_with_points() from the source, starting copy and target; those family lengths now differ.

Relevant source: shared visuals _update_section_assembly lines 971–990, especially opacity filter line 984 and composite.set_points line 988; section_assembly lines 993–1002; house_section material assembly line 1111. The line references describe the source inspected during diagnosis and may move under concurrent work.

## Minimal Reproduction

Ran a fresh no-render reproduction through repo with the existing teaching Python environment. It creates only house_section(ORIGIN), scales it, creates FadeIn, calls begin(), updates the animation copies by 1/6 second and interpolates at 1/6. This reproduces without meter cards, text labels or any authored P_PRIMARY arguments.

Observed family point counts:

- Immediately after animation.begin(): [18, 18, 18, 18].
- After animation.update_mobjects(1/6): [18, 17, 18, 18].
- animation.interpolate(1/6): ValueError: zip() argument 2 is shorter than argument 1.

This confirms a live updater changes point-bearing family topology on the faded animation copy.

## Recommendation

The architectural section implementation should keep the composite's point-bearing topology stable during FadeIn/FadeOut and other copy-based transforms. Its material-membership decision should not depend on transient animation opacity. Preserve material identity separately from animated appearance, and update appearance without deleting composite geometry from a faded copy. Add a focused FadeIn regression that updates copies and interpolates across several frames before relying on a complete clip render.

No production source was modified for this diagnosis. Existing accent verification and all-series visual evidence remain applicable; this optional new render is blocked by concurrent section-assembly behavior.
