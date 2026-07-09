---
name: Persist Slide Modifications
overview: Make per-slide modifications (drag/resize transforms, enlarge, and selection) persist across slide navigation so they survive going back and forth, and are cleared only by the existing reset buttons.
todos: []
isProject: false
---

## Background

All slide modifications already live in ephemeral React state inside `usePresentationInteraction` in [framework/product/presentation/renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx). Disposition ids are already slide-unique (`renderSlideId--morphId--embodimentId`, via `dispositionInteractionId`), so state keyed by these ids cannot collide across slides.

The ONLY thing wiping modifications on navigation is this effect:

```2812:2816:framework/product/presentation/renderer/react/index.tsx
useEffect(() => {
	setSelectedIds(new Set());
	setTransforms(new Map());
	setEnlargedIds(new Set());
}, [slideEpoch]);
```

The two reset buttons stay as-is and remain the way to clear: per-disposition reset (`clearTransform`) and whole-slide reset (`resetSlide`).

## Changes (all in `renderer/react/index.tsx`)

- Remove the `slideEpoch` reset effect (lines 2812-2816). Because ids are slide-unique, transforms/enlarge/selection automatically persist per slide and survive navigation.
- Drop the now-unused `slideEpoch` parameter from `usePresentationInteraction` (line 2807) and update its docstring (line 2806) which currently says "resets when slideEpoch changes".
- Simplify `PresentationInteractionProvider` (lines 4022-4030) to no longer accept/pass `slideEpoch`; update its render site (line 4253). `slideEpoch` itself stays in `PresentationDeck` since it is still needed for re-measurement (lines 1715, 3171, 3187) and PDF re-render.
- Make empty-space deselect stop clearing enlarge: `clearSelection` (lines 2859-2862) currently also clears `enlargedIds`. Since enlarge is a modification, change it to clear only `selectedIds` so modifications reset only via the reset buttons (and the enlarge toggle). This honors "they just reset when the reset buttons are pressed".

## Tests (extend existing test block in same file; do NOT add new files)

- Add a test that drags/resizes a disposition, navigates to another slide and back (via `deck.slide(...)` / reveal navigation as done in existing auto-animate tests around line 6339), and asserts the modification is still applied.
- Add a test that enlarging a disposition then clicking empty space keeps it enlarged (updating expectations consistent with the new `clearSelection` behavior).
- Keep existing reset-button tests ("resets the whole slide..." line 7060, "resets a dragged canvas-framed disposition..." line 7101) passing — they should already pass unchanged.
- Run the presentation react tests via `launch.json` / `nx` (not ad-hoc scripts) and confirm green.

## Notes / out of scope

- [framework/product/presentation/AGENTS.md](framework/product/presentation/AGENTS.md) currently states "all ephemeral modification are lost after a slide change", which now contradicts the new spec. Workspace rules forbid editing `AGENTS.md`, so this doc is left unchanged and should be updated by the repo owner.
- Per workspace rules, this work runs inside a ticket: reopen the relevant ticket or open a new one via the repo MCP before editing, and close it with a summary when done.
  </plan>
  <todos>[{"id": "ticket", "content": "Reopen/open the appropriate ticket via repo MCP and associate with a goal"}, {"id": "remove-reset", "content": "Remove slideEpoch reset effect in usePresentationInteraction and drop unused slideEpoch param + update docstring"}, {"id": "provider", "content": "Simplify PresentationInteractionProvider to not pass slideEpoch; update render site"}, {"id": "clear-selection", "content": "Make clearSelection clear only selection, not enlarge"}, {"id": "tests", "content": "Extend renderer test block: navigation-persistence test + empty-click-keeps-enlarge test; run via nx/launch.json and confirm green"}]
