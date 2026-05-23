# Board play + fixture file drop

Ticket work: elements board play page, UI shell, fixture DnD, `setSelectionIds`, dev command.

## Files
- `elements/client/lib/board/js/index.ts` — fixture types/parser, `fixtureDrop` event, drag MIME + codec, `setSelectionIds`
- `elements/client/lib/board/react/index.tsx` — `BoardCanvas` file drop + ring affordance
- `elements/client/lib/board/play/*` — Vite play app, triptych layout, Nakagin fixture
- `elements/client/lib/board/{package.json,project.json,dev.script.ts,vitest.config.ts}`
- `dev.script.ts`, `package.json` — `dev board` / `dev:board`
- `elements/client/lib/react/index.tsx` — `UIAppConfig.onActiveWindowChange`, `UIProps.initialPanelVisibility`, `UICanvas` wiring
- Triptych: per-pane cameras (`triptychCamerasFromFixture`), per-pane selection (`selectionSeedForFixture` + `BoardSelectionReporter`), left panel fixture drop + stats, right panel inspector for active pane + selection
- `BoardCanvas` defers DOM-portal `{children}` until `contextRenderer` is set so `useBoard` / `useBoardEvent` never see a null provider on first paint.
- Play graph markers must be direct `Fragment`/`Node`/`Edge` under `BoardCanvas` (`nakaginBoardMarkers`); `buildBoardSceneDescriptor` does not traverse custom component types.