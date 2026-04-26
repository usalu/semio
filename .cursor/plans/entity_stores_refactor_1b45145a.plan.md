---
name: entity stores refactor
overview: Refactor the semio JS/react store boundary so each semio entity has one authoritative live store facade, with direct and computed properties exposed only on the entity store that owns them. React and Sketchpad will become thin consumers of those stores instead of mixing snapshot, DTO, and ad hoc live-read paths.
todos:
 - id: audit-store-surface
   content: Audit and classify every current semio/js store/read helper by entity and property ownership.
   status: in_progress
 - id: define-store-contract
   content: Define the internal entity-store base and the public per-entity store contracts in semio/js.
   status: pending
 - id: complete-piece-design-type
   content: Move computed piece/design/type reads into their owning stores and extend GraphQL read mapping where needed.
   status: pending
 - id: migrate-react-hooks
   content: Refactor semio/react hooks to consume entity stores through useSyncExternalStore only.
   status: pending
 - id: migrate-sketchpad-consumers
   content: Update sketchpad consumers to the new React hooks and remove direct duplicate field reads.
   status: pending
 - id: validate-and-close
   content: Extend existing tests and run focused JS/react/sketchpad validation before closing the ticket.
   status: pending
isProject: false
---

# Entity Store Refactor Plan

## Target Shape

- Use `semio/js/index.ts` as the only public store implementation layer: `KitStore` owns the WASM/GraphQL transport, lifecycle events, scopes, and entity-store factories.
- Introduce or complete exactly one `*Store` per entity exposed to UI code: `KitStore`, `DesignStore`, `TypeStore`, `PieceStore`, `ConnectionStore`, `FamilyStore`, `FileStore`, `FolderStore`, plus missing first-class stores such as `AuthorStore`, `TagStore`, `ConceptStore`, `QualityStore`, `PortStore`, `ConnectorStore`, `RepresentationStore`, `LayerStore`, `GroupStore`, `PropStore`, and `StatStore` where the schema exposes them.
- Define field ownership once. For example, `PieceStore` owns `plane`, `center`, `flatPlane`, `flatCenter`, `flatPose`, parent/child/path/alternative reads; `DesignStore` owns design collections and design-level computed catalogs; `TypeStore` owns type fields and best representation; `KitStore` owns only kit-root fields and entity collections.

## Implementation Steps

- Start from the existing open `Typesafe Semio Js Stores` track, then audit current overlap in `semio/js/index.ts`: `LiveKitRoot`, `KitStoreClient`, `SemioKit*ReadStore`, `WasmKitStoreClient.getDto()`, and existing entity stores. Collapse duplicated read helpers into entity stores, keeping shared internals private.
- Build a common `EntityStore` base/internal helper in `semio/js/index.ts` for `scope`, `id`, `subscribe`, `version`, typed `read`, field snapshots, and event filtering. This should keep RxJS internal and avoid exposing raw `read(...)`, command wires, or DTO cache mechanics.
- Extend the GraphQL read mapping in `semio/js/index.ts` for missing Rust read commands needed by the entity stores, especially `PieceStore` computed reads (`flatPlane`, `flatCenter`, `flatPose`, `path`, `parentPieceId`, `parentConnectionId`, replacement alternatives) and missing design/type read surfaces.
- Replace `KitStoreClient` direct methods like `readPieceFlatPlane`, `readDesignClusterableGroups`, and `readTypeBestRepresentation` with calls through the owning entity stores. Keep compatibility only inside the migration layer while updating consumers; do not expose duplicate public fields after the refactor.
- Refactor `semio/react/index.tsx` so hooks delegate to `@semio/js` entity stores via `useSyncExternalStore`. Remove parallel exposed field sources from `IndexedSchemaState`, `SemioKitViewStore`, `SemioKitDesignReadStore`, `SemioKitShallowListReadStore`, and per-field ad hoc live-read hooks where they duplicate entity-store fields.
- Update `semio/sketchpad/index.tsx` imports and wrappers to consume only the React hooks backed by the new stores. Keep the sketchpad UI store distinct from semio entity stores to avoid the current `useDesignStore` naming collision.
- Extend existing embedded tests only: add `semio/js/index.ts` tests asserting single ownership of computed properties and entity-store invalidation; update `semio/react/index.tsx` hook tests where present; adjust existing `semio/sketchpad/index.tsx` Playwright coverage for representative direct and computed fields.

## Validation

- Run focused JS checks for `semio/js`, then `semio/react`, then affected `semio/sketchpad` Playwright slices.
- Run the layer/dependency check to confirm `sketchpad -> react -> js -> GraphQL -> rs` remains strict.
- Verify no public duplicate access remains for computed fields such as `flatPlane`: it should be exposed by `PieceStore` and consumed through React hook wrappers only.
