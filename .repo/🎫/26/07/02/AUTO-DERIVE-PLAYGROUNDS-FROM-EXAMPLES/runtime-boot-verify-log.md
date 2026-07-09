# Runtime Boot Verify Log

Date: 2026-07-02

## createRuntime smoke test (Bun, via app-registry)

Command: load each playground app, `createPlayground().createRuntime()`.

| Result                   | Count |
| ------------------------ | ----- |
| OK                       | 22    |
| FAIL (WASM in Node only) | 2     |

### Failures (expected in Node)

- `trinity-jack` — `wasm.trinitysession_new` (WASM not loaded outside browser)
- `trinity-rewrite` — same

### Runtime fixes applied

- **note** — `this.id` → `NOTE_PLAY_APP_ID`; added `eagerPlayExampleGlob` import; static fallback for `semio.note.json`
- **writer** — added `isPlaygroundExampleLocked`, `playgroundResolvedExampleId` imports; static fallback for example JSON
- **gis-2d** — fixed `MapPlayController` constructor typo (`readonly exampleHost`)
- **puzzle-3d** — restored `PUZZLE_3D_PLAY_KINDS` constant
- **procedural-3d** — restored `createProceduralPlayFixtureStore` + type
- **s** — added `sPlayAppDefinition` via `createPlaygroundApp`
- **package.json** — removed stale `./playground` exports from 23 core packages
