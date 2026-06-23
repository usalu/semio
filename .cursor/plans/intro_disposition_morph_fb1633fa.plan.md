---
name: Intro disposition morph
overview: Stop reveal auto-animate from FLIPping individual intro text lines. Move the morph anchor onto the disposition wrapper so each one-to-one intro block animates smoothly as one unit. Preserve the ghost/opacity mechanism entirely (it is the only way to do one-to-many and many-to-one). Enforce "no extra animation" surgically via per-pair styles, not by stripping ghost CSS.
todos:
  - id: wrapper-anchor
    content: Set data-id (revealMorphId) and MorphAnchorOnWrapperContext=true on intro flow disposition wrappers in InteractiveDisposition/buildInteractiveSlideLayout
    status: completed
  - id: drop-leaf-ids
    content: Pass anchorOnWrapper into flow TextMorphView/AuthorsMorphView/AffiliationsMorphView and omit leaf data-id when the wrapper owns the anchor
    status: completed
  - id: pair-source
    content: Extend isRevealAutoAnimatePairSource so intro flow wrappers (data-intro-slot) qualify; leaf nodes drop out
    status: completed
  - id: pair-options
    content: In presentationAutoAnimateMatcher use {scale:false, styles:[]} for intro wrapper pairs with a simple wrapper-box measure
    status: completed
  - id: preserve-ghosts
    content: Keep all catalogue ghost/opacity machinery (target-ghost, source-ghost, morph-one, fade keyframes, presentationMorphGhostAutoAnimateCss injection) globally intact; intro changes must not disable or exclude it
    status: completed
  - id: revert-blunt-strips
    content: Revert intro-wide enforcement (stripRevealAutoAnimateSheetToFlipOnly + introFlowMorph skip, deck autoAnimateStyles:[], section:not(--intro) ghost exclusions); enforce no-extra-animation only via per-pair styles:[] on one-to-one pairs
    status: completed
  - id: remove-leaf-machinery
    content: Delete leaf-text intro measure code (revealIntroFlowTextMeasureForAutoAnimate, row-lock, intro branch of text pair options)
    status: completed
  - id: transition-none
    content: Add data-transition="none" to intro arrangement sections in ArrangementSectionSurface
    status: completed
  - id: css-guard
    content: Scope any intro animation:none guard to non-ghost one-to-one targets so ghost fade keyframes still run on intro ghost morphs
    status: completed
  - id: tests
    content: Update intro tests to assert wrapper-anchored one-to-one morph and FLIP-only pair options; assert ghost/opacity morphs (catalogue + any intro ghost) still work; run renderer + core test targets
    status: completed
isProject: false
---

# Intro Disposition Morph (FLIP-on-text forbidden)

## Goal
The intro uses **only one-to-one morphs**: each disposition morphs into itself. The only motion is reveal auto-animate moving the **disposition wrapper** smoothly between its grid positions (no per-text-line FLIP, no incidental opacity/size tweens). The **ghost + opacity mechanism is for the catalogue** (one-to-many / many-to-one) and must stay globally intact and untouched by intro changes. Intro slide transition is `none`.

## Non-negotiable: keep the ghost/opacity mechanism (catalogue)
The ghost machinery is the only way to do one-to-many / many-to-one, so it is preserved globally and must NOT be disabled or excluded by anything intro-related:
- Classes/anchors: `presentation-target-ghost`, `presentation-source-ghost`, `presentation-morph-one`, and their detectors (`elementIsTargetGhostAnchor`/`SourceGhostAnchor`/`MorphOneAnchor`, `elementIsLabelMorphSource`).
- Keyframes/CSS: `presentation-target-ghost-fade-out`, `presentation-morph-one-fade-out`, `presentation-target-ghost-frame`, figure crop morph keyframes, and the injected `presentationMorphGhostAutoAnimateCss`.
- Core data: `morphFrom` (many-to-one), `morphTo` (one-to-many), `revealMorphCompanion`, `affiliationEmbodimentMorphLabels`/`morphLineLabels`.

Intro itself contains no ghost morphs, but the intro-wide strips I added earlier (sheet strip, deck `autoAnimateStyles: []`, `section:not(--intro)` exclusions) still leak into and weaken the catalogue ghosts, so they are reverted. "No extra animation" on intro is enforced only by setting per-pair `styles: []` on the one-to-one pairs (which controls reveal's per-element style tweens and does not touch ghost CSS).

## Core change: anchor the morph on the wrapper, not the leaf text

Scope: all intro flow dispositions are one-to-one, so the wrapper-anchor conversion applies to every intro flow disposition. Catalogue ghost companions (`morphFrom`/`morphTo`/`revealMorphCompanion`) are outside intro and keep their current rendering untouched.

### 1. Give the intro flow wrapper a `data-id`
In [framework/product/presentation/renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx), the wrapper only gets `data-id` for canvas-framed dispositions (`revealMorphId` at line 2484, applied at line 4204). For intro flow (where `introFlowGridRow !== undefined`), also emit `data-id = disposition.morphId` (base id) and set `MorphAnchorOnWrapperContext` to true for that subtree (line 4233).

### 2. Drop leaf `data-id` on flow text/authors/affiliations
Flow leaf views always emit `data-id`:
- `TextMorphView` (line ~1197)
- `AuthorsMorphView`
- `AffiliationsMorphView` (line ~1394)

Pass the `anchorOnWrapper` flag (already consumed by `PositionedTextMorphView`/`FigureMorphView`, line 2258) into these flow views and **omit leaf `data-id`** when the wrapper owns the anchor. Leaf nodes keep their classes/markup, just no `data-id`.

Consequence (intended, matches "intro only uses one-to-one morphs"): authors/institutions morph as one block; individual lines/marks no longer morph independently. Label/suffix/mark changes (e.g. "Leibniz Universitaet Hannover" -> "LUH" + new suffix) swap instantly inside the moving block.

### 3. Make the wrapper a valid auto-animate pair source
In `isRevealAutoAnimatePairSource` (line 657): currently a `.presentation-interactive-disposition` only qualifies when `--canvas-framed`. Extend it so an intro flow disposition (has `data-intro-slot`) also qualifies, and ensure leaf nodes inside it no longer qualify (they will have no `data-id`, so they are skipped automatically).

### 4. Pair options = FLIP translate only
In `presentationAutoAnimateMatcher` (line 850), for the intro wrapper pair use `{ scale: false, styles: [] }` (no opacity/font/size tweens). Provide a simple slide-local wrapper measure (reuse `revealInkMeasureForAutoAnimate` against the wrapper box; no `tightElementBoundsRect` text-ink logic).

## Revert the blunt intro-wide enforcement (it kills ghosts)
These were added in a prior pass and are too broad; they prevent intro from ever using ghost/opacity morphs. Replace them with surgical per-pair `styles: []` on one-to-one pairs only:
- Remove `stripRevealAutoAnimateSheetToFlipOnly` and the `patchPresentationAutoAnimateStyleSheet(introFlowMorph)` strip branch (line ~951) so the injected ghost CSS is preserved on intro.
- Revert the deck `autoAnimateStyles: []` (line ~4659) back to reveal's default, so ghost/figure pairs without explicit options keep their normal style tweens ("morphs as is").
- Revert the `section:not(.presentation-arrangement--intro)` exclusions in [globals.css](framework/product/presentation/renderer/react/globals.css) (ghost fade-out / morph-one rules, ~lines 947-988) so intro ghost morphs fade like everywhere else.

## Remove the dead leaf-text intro machinery
Delete the leaf-text-specific intro code that caused the fly-ins:
- `revealIntroFlowTextMeasureForAutoAnimate`, `introSlotMorphHeadingLeaves`, `INTRO_FLOW_ROW_CENTER_Y_SLOTS`, the row-locked Y logic
- the intro branch of `revealTextAutoAnimatePairOptions` (leaf-text path)

Keep `REVEAL_FLIP_ONLY_AUTO_ANIMATE_STYLES` as the `styles: []` constant applied per one-to-one pair.

## Slide transition = none on intro
In `ArrangementSectionSurface` (around line 4540), add `data-transition="none"` to intro arrangement sections so reveal does no slide-level crossfade there. Leave the deck-level `transition` and catalogue slides as they are (catalogue morph is matched auto-animate, unaffected by `none`).

## CSS
In [framework/product/presentation/renderer/react/globals.css](framework/product/presentation/renderer/react/globals.css):
- Scope any intro `animation: none !important` guard to **non-ghost one-to-one** targets (e.g. `:not(.presentation-target-ghost):not(.presentation-source-ghost):not(.presentation-morph-one)`) so ghost fade keyframes still run on intro ghost morphs. Prefer removing it entirely if per-pair `styles: []` already prevents the unwanted tween.
- Keep the intro grid rows holding each one-to-one block in place.

## Tests
Update intro tests in [framework/product/presentation/renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx) and [framework/product/presentation/core/index.ts](framework/product/presentation/core/index.ts):
- One-to-one intro morph anchor is the wrapper: assert `data-id` on the `[data-intro-slot]` wrapper and **absence** of `data-id` on leaf `h2`/`h4` for one-to-one intro flow.
- Adjust/remove tests referencing leaf ids like `h4[data-id="institutions--a"]`, title/description leaf measure, row-lock Y.
- Add: matcher pairs one-to-one intro wrappers (not leaf text); pair options `{ scale: false, styles: [] }`.
- Ghost mechanism still intact: keep/strengthen tests that the injected ghost CSS (`presentation-target-ghost-fade-out`, `presentation-morph-one-fade-out`) is present, and that it is NOT excluded on intro. Update the existing `section:not(.presentation-arrangement--intro)` assertions accordingly.
- Keep catalogue figure many-to-one / one-to-many morph tests green.
- Verify with the project test target (`@semio-tech/framework-presentation-renderer-react:test`) and the core package tests.

## Manual verification
Hard-refresh http://localhost:6050 and step the Einleitung: Titel -> Beschreibung -> Ziel -> Autoren -> Fakultaet -> Universitaeten -> Lehrstuehle. Each block should sit in its row and only slide smoothly as a unit when layout shifts; no text flying from top/bottom, no fades.