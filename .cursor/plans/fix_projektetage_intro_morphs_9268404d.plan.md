---
name: Fix Projektetage Intro Morphs
overview: Restore the working commit-34 intro morph behavior (slides 0-6) by removing the later "intro flow morph" grid-lock layer that suppresses all auto-animate, while keeping the catalogue/media morphs and content additions intact.
todos:
  - id: ticket
    content: Read repo://goals and reopen/create the intro-morph ticket via repo MCP
    status: completed
  - id: renderer-config
    content: "Restore stock auto-animate for intro: enable per-slide unmatched, drop data-transition none, drop intro pair options in matcher"
    status: completed
  - id: renderer-layout
    content: Remove intro-flow grid surface + introFlowLocked/gridRow branches; restore generic centered flow placement
    status: completed
  - id: renderer-helpers
    content: Delete now-unused intro-flow helper functions and REVEAL_FLIP_ONLY_AUTO_ANIMATE_STYLES + imports
    status: completed
  - id: css
    content: Remove presentation-arrangement-surface--intro-flow grid CSS in globals.css
    status: completed
  - id: core
    content: Remove unused INTRO_FLOW_MORPH_* helpers in core/index.ts
    status: completed
  - id: tests
    content: Update existing renderer test region to assert stock intro auto-animate behavior
    status: completed
  - id: verify
    content: Verify slides 0-6 morph and slides 7-9 still work on the live deck (port 6050)
    status: completed
isProject: false
---

# Fix Projektetage Intro Morphs (Slides 0-6)

## Root cause (verified on the live deck at `:6050`)

The intro slides render correctly at rest, but transitions cut with no animation. Live instrumentation of slide 0->1 showed only `title` is paired as an auto-animate target, its `transform` stays `none` / `opacity: 1` for the whole run, and `description`/`goal`/`authors`/`institutions` are never animated.

This is a regression from the "intro flow morph" rework added after commit `65cc16208` (34). Commit 34 had ZERO intro-specific renderer code: intro used the generic centered flow layout plus stock reveal auto-animate (`autoAnimateUnmatched: true`, no custom pair options). The later rework in [framework/product/presentation/renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx) kills the morph three ways:

- `autoAnimateUnmatched: false` (line 4840) + per-slide `data-auto-animate-unmatched="false"` (lines 4721-4723) -> new lines never fade in.
- Intro pairs use `revealIntroFlowWrapperAutoAnimatePairOptions()` (line 805): `{ scale: false, styles: [], measure: revealIntroFlowDispositionMeasureForAutoAnimate }`. The custom `measure` (line 776) snaps every slot to its grid-row center, so paired `title` has identical from/to rects (zero translate); empty `styles` blocks size/opacity tweening.
- `data-transition="none"` on intro flow slides (line 4720).
- The grid surface `presentation-arrangement-surface--intro-flow` (5-row grid, weights `[26,16,14,14,30]`) pins `title` to row 1 on every slide, so even with real measurement it would not move.

## Goal

Slides 0-6 morph like commit 34: matched `title` FLIP-moves/scales, and newly added lines (`description`, `goal`, `authors`, `institutions`) fade in. Keep catalogue/media morphs (slides 7-9) and content embodiments (affiliation abbreviations/deltas) unchanged.

## Approach (scoped restore, not a full file revert)

Restore generic centered-flow layout + stock auto-animate for intro slides only, leaving the catalogue ghost-morph machinery (which depends on global `autoAnimateUnmatched: false`) untouched.

### Renderer: [framework/product/presentation/renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx)
- Section element (around lines 4708-4734): drop `data-transition="none"` and the `data-auto-animate-unmatched="false"` for intro flow; instead set `data-auto-animate-unmatched="true"` on intro flow slides (per-slide override so the global `false` still protects catalogue morphs). Stop applying the `presentation-arrangement-surface--intro-flow` grid surface so intro dispositions stack/center generically.
- Matcher (around lines 906-911): for intro wrapper pairs pass no options (`undefined`) instead of `revealIntroFlowWrapperAutoAnimatePairOptions()`, so reveal computes real FLIP from/to rects.
- Disposition component (around lines 3720, 3869, 4174-4202): remove the `introFlowLocked` / `introFlowGridRow` branches that lock position and disable drag, restoring generic flow placement.
- Delete now-unused intro-flow helpers: `revealIntroFlowWrapperAutoAnimatePairOptions`, `revealIntroFlowDispositionMeasureForAutoAnimate`, `revealIntroFlowInkForAutoAnimate`, `measureIntroFlowDispositionInkInSection`, `revealFlowMorphDisablesUnmatched`, `isIntroFlowMorphSlide`/`isIntroFlowMorphTransition`, `REVEAL_FLIP_ONLY_AUTO_ANIMATE_STYLES`, and their imports.

### Renderer CSS: [framework/product/presentation/renderer/react/globals.css](framework/product/presentation/renderer/react/globals.css)
- Remove the `presentation-arrangement-surface--intro-flow` grid rules and `presentation-interactive-disposition--intro-flow` row placement so intro returns to centered flow.

### Core: [framework/product/presentation/core/index.ts](framework/product/presentation/core/index.ts)
- Remove the unused `INTRO_FLOW_MORPH_SLOT_ROW`, `INTRO_FLOW_MORPH_ROW_WEIGHTS`, `introFlowMorphRowCenterFraction`, `introFlowMorphGridRow` (lines ~1505-1535). Leave `introSlideFiles`/`introEmbodiments` (arrangement spec) as-is.

### Tests
- Update the existing renderer test region (assertions around lines 6630-6743, 7985, 8030) that encode the broken intro-flow options/unmatched behavior to instead assert stock auto-animate (matched title pairs without intro options, intro slides enable unmatched). Per repo rules, edit the existing test file/region; do not add new test files.

## Verification (mandatory, runtime)

Use the running dev server (port 6050) and step slides 0->6, confirming via DOM instrumentation that during each transition: matched `title` gets a non-`none` transform when it should move, and new lines appear with a fade (unmatched). Confirm catalogue/media morphs (slides 7-9) still animate correctly.

## Process
- Read `repo://goals`, then reopen/create the matching ticket via repo MCP before editing; keep any temp logs/screenshots inside the ticket folder; close with a summary of touched files when done.