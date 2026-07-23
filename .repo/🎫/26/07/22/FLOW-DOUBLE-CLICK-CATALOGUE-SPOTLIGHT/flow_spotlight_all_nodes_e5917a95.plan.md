---
name: Flow Spotlight All Nodes
overview: "Restore premigration double-click flow spotlight suggestion behavior: return every catalogue match (no 20-cap) and show the full scrollable list via the expand chevron / ArrowDown, matching `premigration:flow/react/index.tsx`."
todos:
  - id: reopen-ticket
    content: Reopen FLOW-DOUBLE-CLICK-CATALOGUE-SPOTLIGHT and bind plan id
    status: completed
  - id: remove-limit
    content: Remove limit=20 from flowRankCatalogueSuggestions; return all ranked matches
    status: completed
  - id: restore-expand-ui
    content: Restore FlowSpotlight chevron expand/collapse, visible slice, scroll helper, ArrowDown auto-expand
    status: completed
  - id: tests
    content: Extend vitest for uncapped ranking + scroll class; run and log under ticket folder
    status: completed
isProject: false
---

# Restore Flow Spotlight Full Node List

## Root cause

Compared to tag [`premigration`](premigration) (`flow/react/index.tsx`), the migrated spotlight in [`framework/renderer/react/index.tsx`](framework/renderer/react/index.tsx) regressed in two ways that hide nodes:

1. **Hard cap** — `flowRankCatalogueSuggestions(..., limit = 20)` slices to 20. Premigration returned **all** ranked matches (catalogue has 100+ operators).
2. **Missing expand UX** — Premigration collapsed to the top match and revealed **every** match when the ▾ chevron (or ArrowDown) expanded the list with a capped scroll container (`flowSpotlightSuggestionListScrollClass`). Current `FlowSpotlight` always renders the truncated list with no expand control.

```13666:13677:framework/renderer/react/index.tsx
export function flowRankCatalogueSuggestions(sections: readonly FlowCatalogueSection[], query: string, limit = 20): FlowCatalogueItem[] {
  // ...
  return scored.slice(0, limit).map((row) => row.item);
}
```

Premigration reference (no limit; expand gates visibility):

- `flowRankCatalogueSuggestions` → full ranked array
- `visible = expanded ? suggestions : suggestions.slice(0, 1)`
- Chevron when `suggestions.length > 1`; ArrowDown auto-expands

Out of scope for this fix: spotlight slider/note-from-query parsing (separate premigration features). Scope is catalogue node listing parity.

## Ticket / goal

- Goal: `🎯r2602/🎯runningsketchpad` (same as prior spotlight ticket)
- Reopen [`.repo/🎫/26/07/22/FLOW-DOUBLE-CLICK-CATALOGUE-SPOTLIGHT`](.repo/🎫/26/07/22/FLOW-DOUBLE-CLICK-CATALOGUE-SPOTLIGHT) and bind this plan; put logs/notes in that ticket folder

## Implementation

All edits in [`framework/renderer/react/index.tsx`](framework/renderer/react/index.tsx) region `FlowCatalogueSpotlight`, plus tests in [`framework/renderer/react/index.test.ts`](framework/renderer/react/index.test.ts).

### 1. Ranking: no artificial limit

- Change `flowRankCatalogueSuggestions(sections, query)` to return the full scored list (drop `limit` / `slice(0, limit)`), matching premigration.
- Keep existing score/neuron-first ordering (already close enough).

### 2. Restore expand + scroll list UX in `FlowSpotlight`

Mirror premigration interaction:

- State: `expanded` (default `false`)
- `visible = expanded ? suggestions : suggestions.slice(0, 1)`
- `hasMore = suggestions.length > 1`
- Header row: search input + ▾/▴ button (`aria-label` Show all / Collapse) when `hasMore`
- ArrowDown: advance `activeIndex` and `setExpanded(true)` when `hasMore`
- When expanded: `scrollIntoView({ block: "nearest" })` for the active row; `onWheel` stopPropagation so canvas zoom does not steal scroll
- Export `flowSpotlightSuggestionListScrollClass(expanded)` using the premigration **normal** caps (`overflow-y-auto` + `max-h-[min(24rem,70vh)]` when expanded; `overflow-hidden` when collapsed). Skip full DAG LOD chrome wiring unless `drawLodLabel` is already plumbed on the host session type in the same change set.

### 3. Tests

Extend existing vitest coverage in [`framework/renderer/react/index.test.ts`](framework/renderer/react/index.test.ts):

- Empty query returns **all** section items (length equals fixture count, not capped at 20) — build a fixture with >20 items
- `flowSpotlightSuggestionListScrollClass(false)` → `overflow-hidden`; `(true)` → `overflow-y-auto` + max-height class

Run the relevant vitest target via nx/`script.ts` and keep output under the ticket folder.

## Verification

- Double-click empty flow canvas → top suggestion only
- Click ▾ (or ArrowDown) → full catalogue list, scrollable, every node reachable
- Broad/empty query no longer stops at 20
- Escape / outside click still clears ghost and closes (existing behavior)