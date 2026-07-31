# Status (2026-05-12)

- **Done (this pass):** `compose/js/index.ts` rewritten to plan regions: `🌐️Transport` (GqlTransport + EventBus + worker wiring), `🧬️Entity` (Entity + defineField/defineOperation/defineFields/defineOperations), `🧱️Classes` (Kit + all entity classes + weak-entity interfaces), `🚀️PublicAPI` (`openKit`), `🧪️Tests` (negative grep + Piece.drag stub + EventBus).
- **Done:** `compose/js/kit-store.worker.ts` — still `KitStoreHandle` JSON GraphQL execute/subscribe only (header clarified).
- **Verified:** `npx tsc -p compose/js/tsconfig.json`; `COMPOSE_JS_RUN_EMBEDDED_TESTS=1 npx vitest run --config compose/js/vite.config.ts` (3 tests).
- **Open:** Monorepo packages importing removed `@semio-tech/compose-js` symbols (`compose/react`, `compose/sketchpad`, `compose/algorithms`, …) — **ticket stays open** per coordinator instruction (no `ticket_close` until full repo completion).
- **Artifacts:** `phase-0.json`, `phase-0-1-js-done.json`, `weak-entity-ts-note.md`, `phase-1-meta-rep.json`.
- **2026-05-12 (Phase 1 meta/rep slice):** Author, Quality, Tag, Concept, Representation, bulky extras, WeakEntities wire updates in `compose/js/index.ts`; React regions + `RepresentationEntityContext` in `compose/react/index.tsx`. Read-only Author/Representation: field hooks only. `ticket_close` blocked: coordinator line above keeps ticket open until full repo completion.
