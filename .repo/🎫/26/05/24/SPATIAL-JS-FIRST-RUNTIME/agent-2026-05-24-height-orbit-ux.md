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

Verified: `bun nx run @spatial/js-renderer-r3f:test`.

---

## 2026-05-24 — Ground pick XY / grid alignment

- **Cause:** Factory/brep use **footprint XY** and **height Z**; `GroundPickPlane` used a mesh in **world XZ** (Y-up floor) but still emitted `[p.x, p.y, planeZ]`, so **world `p.z` was dropped** → cursor moved on **one axis** only.
- **Fix:** Pick mesh is **XY at world `z = planeZ`** (no `rotation.x = -π/2`). `GridHelper` gets `rotation.x = π/2` and a tiny **+Z** offset so lines sit on the same working plane as the box preview.

