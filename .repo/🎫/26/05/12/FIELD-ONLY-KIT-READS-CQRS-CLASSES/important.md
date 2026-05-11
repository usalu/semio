# Status (2026-05-12)

- **Done (this pass):** `semio/js/index.ts` rewritten to plan regions: `🌐Transport` (GqlTransport + EventBus + worker wiring), `🧬Entity` (Entity + defineField/defineOperation/defineFields/defineOperations), `🧱Classes` (Kit + all entity classes + weak-entity interfaces), `🚀PublicAPI` (`openKit`), `🧪Tests` (negative grep + Piece.drag stub + EventBus).
- **Done:** `semio/js/kit-store.worker.ts` — still `KitStoreHandle` JSON GraphQL execute/subscribe only (header clarified).
- **Verified:** `npx tsc -p semio/js/tsconfig.json`; `SEMIO_JS_RUN_EMBEDDED_TESTS=1 npx vitest run --config semio/js/vite.config.ts` (3 tests).
- **Open:** Monorepo packages importing removed `@semio/js` symbols (`semio/react`, `semio/sketchpad`, `semio/algorithms`, …) — **ticket stays open** per coordinator instruction (no `ticket_close` until full repo completion).
- **Artifacts:** `phase-0.json`, `phase-0-1-js-done.json`, `weak-entity-ts-note.md`.
