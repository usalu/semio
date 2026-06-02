---
name: Fix Projektetage 8to9 Morph
overview: Fix the runtime auto-animate morph on slides 8 to 9 of the 33. Projektetage deck so each focus tile (Rippenplatte 1-6, Unterzug 1-3, Stütze) morphs into its own per-tile ghost positioned at the column label, fading out while the real label text appears, matching commit 223's visual output. Keep the current morphFrom/ghost architecture and the "Rippenplatte" label.
todos:
  - id: ticket-repro
    content: Open presentation ticket; start projektetage dev server; add [DEBUG] logs in presentationAutoAnimateMatcher to dump focus->labels pairs and from/to rects; reproduce 8->9 in browser.
    status: completed
  - id: diagnose
    content: "Confirm root cause: ghost not animatable at FLIP time (visibility:hidden), data-id mismatch/duplication, or settle/dormant timing."
    status: completed
  - id: fix
    content: "Apply targeted fix (primary: rest .presentation-morph-source ghost via opacity:0 so reveal can FLIP it; keep label .presentation-morph-into fade-in) and any data-id/matcher fix the logs require."
    status: in_progress
  - id: verify
    content: Verify in browser that 10 tiles morph into the 3 label slots and fade while labels appear (screenshot); extend existing tests; run framework + deck vitest; remove [DEBUG] logs; close ticket.
    status: pending
isProject: false
---

# Fix Projektetage Slides 8 to 9 Morph

## Decisions (confirmed)
- Keep current `morphFrom`/ghost architecture; commit `223` is only the visual reference, not a revert target (its `split`/`columnMorphTiles`/`morphTargets` API no longer exists in core).
- Keep col1 label text as "Rippenplatte".

## Background
Slides 8 to 9 are `catalogue-focus` (Bauteilarten) to `catalogue-labels` (Bauteilbeschriftungen). The intended effect (per 223): the 10 focus tiles fly into the three column label positions and fade out 1->0 while the three label texts fade in 0->1.

reveal.js auto-animate cannot match many sources to one target, so the deck creates one ghost per source tile on slide 9, each carrying the same `data-id` as its source tile, placed at the column label position, hidden at rest. This is already wired:
- `columnLabelMorphFrom` in [mit-bestand/präsentation/33.projektetage/spec.ts](mit-bestand/präsentation/33.projektetage/spec.ts) builds one `morphFrom` slot per tile.
- `expandArrangementMorphFrom` in [framework/product/presentation/core/index.ts](framework/product/presentation/core/index.ts) turns each slot into a `morphSource` ghost disposition.
- The matcher `presentationAutoAnimateMatcher` and ghost CSS live in [framework/product/presentation/renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx) and [framework/product/presentation/renderer/react/globals.css](framework/product/presentation/renderer/react/globals.css).

The unit tests (jsdom) only assert matcher pairing and that an `autoanimate` event fires; they cannot verify the visual FLIP. So tests pass while the morph is visually broken at runtime. The fix therefore requires browser confirmation, not just green tests.

## Step 1: Open a ticket and reproduce in the browser
- Open a ticket (repo MCP `ticket_open`, goal `presentation`), e.g. `26/06/02/PROJEKTETAGE-FOCUS-LABEL-MORPH`. Put all temp logs/scripts inside the ticket folder.
- Start the deck dev server (`bun nx run @mit-bestand/praesentation/projektetage:dev`, port 6050) and drive reveal to slide 8 then 9 in the browser.
- Add temporary `[DEBUG]` logs inside `presentationAutoAnimateMatcher` ([renderer index.tsx](framework/product/presentation/renderer/react/index.tsx) ~L315-343) to print, for the focus->labels transition: each kept pair's `data-id`, `from`/`to` `nodeName`, and `from`/`to` `getBoundingClientRect()`. This confirms whether reveal pairs tile->ghost and whether the ghost `to` rect is the label position (non-zero, measurable).

## Step 2: Diagnose why the ghost FLIP does not play
Confirm at runtime which of these is the actual cause (most-likely first):
- Ghost not measurable/animatable at FLIP time: `.reveal .presentation-morph-source { opacity:0 !important; visibility:hidden !important; }` ([globals.css](framework/product/presentation/renderer/react/globals.css) ~L625-647). `visibility:hidden` (and any zero-box state) can make reveal treat the ghost as having no animatable box, so tiles fade as unmatched instead of morphing. The user's prescription ("same id and 0 opacity") points at resting the ghost via opacity only.
- `data-id` mismatch: verify slide-8 tile wrappers and slide-9 ghost wrappers carry identical `data-id` values (tile participant ids), with no duplicate `data-id` on inner nodes (same bug class fixed in the Description Morph ticket via `revealMorphId` only when `declaredRect !== undefined`, [renderer index.tsx](framework/product/presentation/renderer/react/index.tsx) ~L1237-1244).
- Settle/dormant timing: `catalogue-focus` sets `settleBeforeMorphTo: ["catalogue-labels"]` ([Bauteilarten.ts](mit-bestand/präsentation/33.projektetage/slide/Hauptteil/Einführung/Medien/Bauteilarten.ts)); confirm the pending/running classes actually reach the ghost during 8->9 and are not overridden.

## Step 3: Apply the targeted fix
Based on Step 2 evidence, the primary expected change is to let the ghost be reveal-animatable while still invisible at rest, in [globals.css](framework/product/presentation/renderer/react/globals.css):
- Rest the `.presentation-morph-source` ghost with `opacity:0` (drop `visibility:hidden`, or otherwise keep a measurable, animatable box) so reveal can FLIP it from the tile position to the label position, then fade 1->0 during `running` (existing `presentation-morph-source-fade-out`). Keep it non-interactive (`pointer-events:none`) at rest.
- Ensure the three label texts still fade in via `.presentation-morph-into` (rest visible, hidden during pending, fade-in during running) so the real "Rippenplatte"/"Unterzug"/"Stütze" text lands at the same position the ghosts converge to.
- If Step 2 reveals a `data-id` or matcher-filter problem instead, fix it in [renderer index.tsx](framework/product/presentation/renderer/react/index.tsx) so every tile pairs 1:1 with its same-id ghost and no ghost double-matches.

Adjust only the minimum needed; keep regions/subregions structure.

## Step 4: Verify runtime behavior, then clean up
- In the browser, step 7->8->9 and confirm: 8->9 shows all 10 tiles travel to the three label slots and fade out while the three labels fade in (matching 223). Capture a screenshot for the ticket.
- Extend the existing tests (do not add new test files): in [framework/product/presentation/renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx) tests assert the ghost rests animatable (e.g. not `visibility:hidden`) and pairs 1:1; in [mit-bestand/präsentation/33.projektetage/index.ts](mit-bestand/präsentation/33.projektetage/index.ts) keep the morph-source/label assertions green.
- Run the framework presentation + deck vitest suites and confirm all pass (state actual run results, no assumptions).
- Remove all `[DEBUG]` logs. Close the ticket via `ticket_close` with a summary and the files touched.