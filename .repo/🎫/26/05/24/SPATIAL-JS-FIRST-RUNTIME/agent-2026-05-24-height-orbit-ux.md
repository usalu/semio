# Session 2026-05-24 — height preview + orbit UX

**Repo MCP:** not available; manual ticket note.

## Changes

- `spatial/js/renderer-r3f/play/main.tsx` — Help text: teal wall at second corner, no “orbit paused”; right-drag to orbit if clicks conflict.
- `spatial/js/renderer-r3f/index.tsx` — `OrbitControls.mouseButtons`: **LEFT** no-op (`-1` cast), **RIGHT** rotate, **MIDDLE** dolly so factory **left** picks are not eaten by rotate. Fixed `heightMoveOn` to a real **boolean** (was `Vec3 | false` from `&& corner`).

## Verification

- `bun nx run @spatial/js-renderer-r3f:test` — pass (2026-05-24).

---

## 2026-05-24 — First-corner cursor preview

- `spatial/fixtures/factory.json` — `start` seeds `cursor`; `pickingFirstCorner` handles transient `pointer.move`; first `pointer.down` clears `cursor`; display `point` role `cursor`.
- `spatial/js/renderer-r3f/play/main.tsx` — `pointerMoveActive` includes `pickingFirstCorner`; help text mentions cyan dot.
- `spatial/js/renderer-r3f/index.tsx` — ground `pointerMoveEnabled` for first + second corner; `PointItem` styles `role === "cursor"`.
- `spatial/js/core/index.ts` — Vitest: cursor track + clear after first pick.

Verified: `bun nx run @spatial/js-core:test`, `@spatial/js-renderer-r3f:test`.
