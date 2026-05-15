# Board play + fixture file drop

Ticket work: elements board play page, UI shell, fixture DnD, `setSelectionIds`, dev command.

## Files
- `elements/client/lib/board/js/index.ts` — fixture types/parser, `fixtureFileDrop` event, `setSelectionIds`
- `elements/client/lib/board/react/index.tsx` — `BoardCanvas` file drop + ring affordance
- `elements/client/lib/board/play/*` — Vite play app, triptych layout, Nakagin fixture
- `elements/client/lib/board/{package.json,project.json,dev.script.ts,vitest.config.ts}`
- `dev.script.ts`, `package.json` — `dev board` / `dev:board`