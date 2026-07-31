# Delete Play-Host Layer — Verify Log

## Framework

- [x] `rendererPackage` / `rendererExport` on `PlaygroundAppManifest`
- [x] `playgroundRendererImports` virtual module with async factory support
- [x] `loadPlaygroundRendererContribution` + `resolvePlaygroundRendererExport`
- [x] `bootPlaygroundApp` loads renderer from manifest (no `loadRenderer`)
- [x] OS `app-contribution-registry` uses manifest renderer imports
- [x] `derivePlaygroundCreateRuntime` / `runtimeBootstrap` on `createPlaygroundApp`
- [x] `usePlayController`, `createFixtureFileBridge`, `createOsInstanceHost`, `finalizeRendererContribution`
- [x] `buildControllerTreeSidePanelBody` helper in playground core

## Apps

- [x] All 23 `play-host.tsx` deleted; content merged into `react/index.tsx` `//#region 🔖PlayHost`
- [x] All `./play` package exports removed
- [x] All core manifests updated with `rendererPackage` / `rendererExport`
- [x] `loadRenderer` removed from all app cores
- [x] Declarative panel tabs migrated (core `sidePanelBodies` + runtime `panelTabs`)

## Tests

| Package                                                | Result                                                                                                                                                                                                           |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@semio-tech/ui-styling:test`                          | 12/12 pass                                                                                                                                                                                                       |
| `@semio-tech/framework-playground-renderer-react:test` | 25/25 pass                                                                                                                                                                                                       |
| `@semio-tech/draw-core:test`                           | 6/6 pass                                                                                                                                                                                                         |
| `@semio-tech/note-core:test`                           | 11/11 pass                                                                                                                                                                                                       |
| `@semio-tech/forms-core:test`                          | FAIL — transitive import chain `forms → flow-react → procedural-2d-react` hits circular init (`FlowExtensionHost` undefined during vitest collection); pre-existing core↔react coupling, not play-host boot path |
| `@semio-tech/framework-playground-core:test`           | FAIL — pre-existing missing `@semio-tech/framework-core` vitest alias                                                                                                                                            |
| dependency-cruiser (draw, note, flow, playground)      | clean                                                                                                                                                                                                            |

## Remaining play-host cleanup (follow-up)

Merged the last 4 apps that still used `./play-host` subpath exports (cycle workaround):

- flow, dag, procedural-3d, puzzle-3d

All now export `*AppRenderer` from main `index.tsx` `//#region 🔖PlayHost` with async factory + lazy core import where core↔react cycles exist.

Zero `play-host.tsx` files remain in the repo.

Manual boot via launch.json not run in this session. Renderer resolution path:

```
semio.app manifest → virtual:semio-playground-apps playgroundRendererImports → bootPlaygroundApp → finalizeRendererContribution → applyAppRendererContribution
```

Draw example: `[DEBUG] draw play exported document` / `[DEBUG] draw play imported document from file` via `createFixtureFileBridge` pattern.
