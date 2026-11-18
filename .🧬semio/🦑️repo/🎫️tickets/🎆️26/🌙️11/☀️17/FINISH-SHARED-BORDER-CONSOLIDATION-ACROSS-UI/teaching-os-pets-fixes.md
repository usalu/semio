# Teaching, OS, pets, hub, print: shared-border fixes

Source: `audit-products.md` (presentation and `✏️s/🔌️plugins` excluded: owned by another agent; course excluded).

## Fixed
- **T2 Segmented** (`🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx`): REAL (hand-made `border-l` divider). Radiogroup is now `data-shared-border-group`, options are `data-shared-border-cell` with `border-0`; the manual `index > 0 && border-l` is removed.
- **T1 Button rows**: REAL where buttons form one control. New `ButtonGroup` (`role=group`, `data-shared-border-group`, one outer frame) and `Button grouped` (`data-shared-border-cell`, `border-0`) in `chrome`. Applied to: transport play/step/start (`⏯️transport`), viewer zoom out/zoom value/zoom in and fit/print size/centre (`🎛️viewer`), playground undo/redo/clear (`🧪️playground`), collection compare/place/save as (`⭐️collections`). Not converted (deliberately independent actions or different components): `PageFrame` actions of the detail page (favourite, compare, place, share), Notice actions, collection rename/delete row, export panel buttons.
- **O1 OS block palette** (`💻️os/.../🧩️BlockListHost/🟦️.tsx`): the list column has no right border, so no double seam with the palette `border-l`; the real problem was a column of fully bordered chips with gaps. Entries are now cells of one vertical `data-shared-border-group` frame (`border-0`, one divider per seam, none after the last). Palette `border-l` kept as the panel edge. Block and step cards left as deliberate cards.
- **O2 AgentChatPanel header** (`💬️AgentChatPanel/🟦️.tsx`): raw `border-b border-border` replaced by `borderNormalBottomClass` (same geometry, now follows the shared seam colour).

## Skipped (verified, no change)
- T3 teaching stage groups: vanilla web component with its own CSS, no ui stylesheet; already one hairline per seam.
- T4-T11, O3-O9: clean or deliberate cards per the audit; rulers/ticks (`border-l` spans) are not seams.
- H2 hub stat tiles: card grid with gaps, intentional.
- Pets (PT1-PT4), print, repo, server, semio-tech: no findings, no seams.
- C1/C2 (CAD REPL): plugin, other agent.

## Tests
- New `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🧷️shared-borders/🟦️.tsx` + `🥒️.feature` (3 tests: ButtonGroup/grouped cells, ungrouped Button keeps frame, Segmented group/cell without `border-l`). Passes (3/3) with ticket config `vitest-teaching.config.ts`.
- Full teaching suite run: 100 passed; 12 files fail on pre-existing missing fixture `🎓️teaching/🧫️fixtures/🚶️reference-walker/🔣️.json` and one catalog-loading count (119 vs 152 ready), unrelated.
- OS changes have no jsdom run and no visual render check.
