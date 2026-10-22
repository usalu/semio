---
name: Restore Einführung Intro Behaviour
overview: Restore the projektetage Einführung/description slides to behave exactly as in commit 1c41ebc by reverting the post-commit intro layout (absolute surface + flex/grid) back to reveal-centered flow and stopping the FLIP-transform stripping that breaks intro auto-animate. DOM elements and data-ids are already identical, so no rendering/id changes are needed.
todos:
 - id: ticket
   content: Read repo://goals and open/reopen a presentation ticket for restoring Einführung intro behaviour
   status: cancelled
 - id: css-flow
   content: "globals.css: remove absolute positioning for non-positioned interactive sections and scope .presentation-arrangement-surface absolute to --positioned; add display:contents for non-positioned surface"
   status: cancelled
 - id: css-intro
   content: "globals.css: remove the display:flex centering on .presentation-arrangement--intro:not(positioned); keep heading-margin and intro typography/gap rules"
   status: cancelled
 - id: flip-clear
   content: "index.tsx: drop the .presentation-arrangement--intro :is(h1..p) selector from clearRevealAutoAnimateInlineLayout so intro FLIP transforms are not stripped"
   status: cancelled
 - id: tests
   content: Run renderer/react and projektetage test + typecheck targets; update only CSS-source assertions tied to removed rules; keep DOM/id tests green
   status: cancelled
isProject: false
---

# Restore Einführung Intro Behaviour

## Root cause (verified by code diff vs commit 1c41ebc)

The intro `data-id` elements, section `data-auto-animate-id`, and `autoAnimateId` generation are all unchanged from the commit. Two post-commit changes broke the visuals:

1. Layout: non-positioned interactive sections became absolute + got an absolute `.presentation-arrangement-surface` wrapper and `.presentation-arrangement--intro { display:flex }`, removing reveal's native vertical centering. The commit rendered intro dispositions in normal flow, centered by reveal.
2. Auto-animate: `clearRevealAutoAnimateInlineLayout` strips `transform`/`transition` from intro `h1..p`, killing reveal's FLIP morph. The commit only cleared `[data-auto-animate-target]`.

## Changes

### `framework/product/presentation/renderer/react/globals.css`

- Delete the absolute rule for non-positioned interactive sections (currently lines 98-103: `position:absolute; inset:0; width/height:100%`) so reveal centers the slide like the commit.
- Scope the absolute `.presentation-arrangement-surface` rule (currently lines 105-111) to `.presentation-arrangement--positioned` only, and add `.presentation-arrangement--interactive:not(.presentation-arrangement--positioned) > .presentation-arrangement-surface { display: contents; }` so flow dispositions are laid out and centered by the section/reveal (matching the commit which had no surface wrapper).
- Remove the `display:flex; align-items:center; justify-content:center` from `.presentation-arrangement--intro:not(.presentation-arrangement--positioned)` (currently lines 1052-1059). Keep `--r-heading-margin` (1049) and the typography rules (intro-rows row-gap, intro-line column-gap, authors+affiliations margin, `:is(h1..p){margin:0;line-height:1.1}`) — these mirror the commit's intro typography and the existing CSS-source test assertions.
- Keep `.presentation-arrangement--interactive { overflow: visible }` and the non-positioned pinned-content centering rule intact (asserted by tests).

### `framework/product/presentation/renderer/react/index.tsx`

- In `clearRevealAutoAnimateInlineLayout` (line 533), remove the `".presentation-arrangement--intro :is(h1, h2, h3, h4, p)"` entry from `selectors` (line 537), leaving only `[data-auto-animate-target]` exactly as the commit. This restores native FLIP for intro text so title -> short and description full -> short morph again.
- Leave the intro branch of `presentationAutoAnimateMatcher` (lines 791-797) and the `introFlowMorph` patch flag as-is: they already reproduce the commit's `data-id` leaf pairing and do not strip animation.

## Tests / verification

- The intro DOM/id tests (around index.tsx 5240-5320) already assert the commit's structure and must stay green; do not change them.
- Adjust only CSS-source string assertions that reference a rule we delete/move (verify the regexes at index.tsx 5256-5264 and 5409-5415 still pass; update them only if a removed declaration was asserted).
- Run `bun nx run @semio-tech/framework-presentation-renderer-react:test` and `bun nx run @semio-tech/mit-bestand-praesentation-projektetage:test`, then `:typecheck`, and confirm all green before closing.

## Repo workflow

- Read `repo://goals`, then open (or reopen) a ticket via the repo MCP under `🎯️presentation` for this fix; keep any temp/log files inside the ticket folder.
