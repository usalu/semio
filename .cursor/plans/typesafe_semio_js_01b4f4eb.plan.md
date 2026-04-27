---
name: Typesafe Semio JS
overview: Refactor `semio/js` so its public and internal method surfaces use explicit DTOs instead of loose generic, unknown, Record, Json, or any-style shapes, then update direct consumers and embedded tests to match.
todos:
 - id: ticket-reopen
   content: Reopen the existing `TYPESAFE-SEMIO-JS-STORES` ticket for this work.
   status: in_progress
 - id: dto-inventory
   content: Inventory all loose method signatures and group them by DTO family in `semio/js/index.ts`.
   status: pending
 - id: dto-definitions
   content: Introduce concrete DTO definitions and schemas for GraphQL, commands, reads, events, and entity patch inputs.
   status: pending
 - id: js-refactor
   content: Replace loose semio/js method inputs and outputs with DTOs and update internal helpers.
   status: pending
 - id: react-consumer
   content: Update `semio/react/index.tsx` consumers and stubs to the new DTO contracts.
   status: pending
 - id: tests-validation
   content: Extend embedded tests and run targeted build/test/lint validation.
   status: pending
isProject: false
---

# Typesafe Semio JS DTO Refactor

## Scope

- Attach implementation to the existing open ticket `TYPESAFE-SEMIO-JS-STORES` rather than opening a duplicate.
- Primary file: [semio/js/index.ts](semio/js/index.ts).
- Direct consumer to keep compiling: [semio/react/index.tsx](semio/react/index.tsx).
- Validation files/configs: [semio/js/package.json](semio/js/package.json), [semio/js/tsconfig.json](semio/js/tsconfig.json).

## Current Findings

- `semio/js/index.ts` is a single-file package entry with embedded Vitest coverage and about 456 loose type hits (`unknown`, `Record`, `SemioJson`, generic helpers, `z.any`).
- The largest loose surfaces are GraphQL parsing, shell/batch command DTOs, live read snapshots, `KitStoreClient`, `WasmKitStoreClient.getDto/getSnapshot`, event filters, and patch helpers such as `piecePatchToWireCommands` / `connectionPatchToWireCommands`.
- `semio/react/index.tsx` imports these APIs and has test stubs typed against current loose return values, so it must be migrated together with `semio/js`.

## Implementation Plan

1. Reopen `TYPESAFE-SEMIO-JS-STORES` and keep all tracking tied to that ticket.
2. Add a DTO section in `semio/js/index.ts` using concrete names only: input DTOs, id DTOs, metadata DTOs, full DTOs, result DTOs, event DTOs, GraphQL request/response DTOs, and command DTOs.
3. Replace loose read/write outputs with named DTOs:
   - `KitStoreReadSnap.data` becomes a discriminated read DTO instead of loose data.
   - `KitStoreClient` read methods return concrete DTO arrays or entity DTOs.
   - `getDto()` / `getSnapshot()` return `KitFullDto` or scoped full DTOs.
4. Replace patch and command inputs with explicit DTO unions:
   - Piece, connection, design, kind, family, file, folder, author, concept, tag, quality, and port patch DTOs.
   - Shell variables and batch command inputs become concrete DTOs instead of map-shaped payloads.
5. Refactor GraphQL and worker boundaries so parsing is immediately validated into concrete DTOs. Keep unavoidable dynamic parsing isolated behind schema parse helpers whose exported and method signatures remain concrete.
6. Update `semio/react/index.tsx` imports, stubs, and hook adapters to consume the new DTO return shapes without casts to loose payloads.
7. Extend embedded tests in `semio/js/index.ts` and existing React tests/stubs in `semio/react/index.tsx` to compile-check the no-loose-method surface and cover DTO conversions.
8. Run targeted checks: `npm run build` and `npm run test` in `semio/js`, plus the relevant `semio/react` type/test check if its package scripts expose one. Finish by checking lints for edited files.

## Risk Controls

- Keep the refactor confined to existing files; no new test files.
- Do not preserve compatibility aliases for old loose names.
- Prefer existing zod schemas and existing entity schemas over new ad hoc parsers.
- Work from method signatures inward so compile failures identify the remaining DTO gaps.
