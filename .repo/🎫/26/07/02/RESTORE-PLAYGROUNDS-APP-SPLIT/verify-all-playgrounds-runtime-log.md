# Playground Verification Log (2026-07-02)

## Summary

Work targeted: confirm all 24 registered playgrounds build and boot in the browser.

## Verified earlier this session

- **Production builds**: all 24 apps built successfully via `framework/product/playground/dev/script.ts build --app <entry>` before concurrent workspace edits introduced regressions.
- **Browser boot (spot checks)**:
  - `draw`, `flow`: `#root` mounts after vite preview
  - `2d`: fixed `uiInspectorAllEqual is not defined` via renderer import fix; boots in ~2s after rebuild

## Root fixes landed

| Area | Fix |
|------|-----|
| `framework/product/playground/renderer/react/index.tsx` | Import + re-export framework symbols used locally (`uiInspectorAllEqual`, etc.) so virtual host slices keep bindings |
| `procedural/2d/react`, `procedural/3d/react` | Gate top-level WASM init with `typeof window !== "undefined"` |
| `cad/js/runtime/index.ts` | Lazy `import.meta.glob` behind `typeof import.meta.glob === "function"` for Bun-safe load |
| `framework/product/playground/core/index.ts` | Skip engagement enforcement when `engagement` is undefined |
| `forms/react/index.tsx` | Fixture path `fixture/` → `example/` for hexagonal mushroom column |
| `puzzle/2d/react`, `trinity/react` | Fixture path `fixture/` → `example/` |
| Core `package.json` exports | Repaired invalid JSON / added `./playground` where missing (many packages) |
| Core `index.ts` play regions | Removed duplicate `*_PLAY_APP_ID` declarations; hoisted `createPlaygroundApp` imports |

## E2E harness

- `.repo/🎫/26/07/02/RESTORE-PLAYGROUNDS-APP-SPLIT/verify-all-playgrounds-e2e.ts` — build + vite preview + Playwright smoke per app
- `.repo/🎫/26/07/02/RESTORE-PLAYGROUNDS-APP-SPLIT/verify-all-playgrounds.ts` — module metadata load (updated to import from `@semio-tech/*-core` main entry)

## Remaining blockers (workspace in flux)

Concurrent edits reverted most `core/playground.ts` splits; play app definitions live in `core/index.ts` again. Additional breakage appeared during verification:

- Invalid / incomplete `package.json` `exports` blocks (partially repaired)
- Corrupted play regions in some cores (procedural 2d/3d partially repaired)
- Deleted fixtures still referenced (`fixture/` → `example/` migration incomplete repo-wide)
- Full E2E batch interrupted when builds began failing mid-run after renderer fix

## Recommended next steps

1. Stabilize fixture layout: complete `fixture/` → `example/` import path updates repo-wide
2. Re-run sequential builds for all 24 entries
3. Run `verify-all-playgrounds-e2e.ts` to completion
4. Spot-check heavy apps (`3d`, `5d`, `cad`, `gis-2d`, `trinity-jack`) in dev via launch.json
