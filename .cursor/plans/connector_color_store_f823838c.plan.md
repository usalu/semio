---
name: Connector Color Store
overview: Move connector color ownership from kit-wide colored connector rows to object-oriented cached connector reads in Rust, then remove the obsolete GraphQL/JS/React/sketchpad callers.
todos:
 - id: rust-color-store
   content: Implement cached ConnectorStore color in Rust and remove kit-wide colored connector rows.
   status: completed
 - id: schema-js-react
   content: Update GraphQL, JS, and React APIs to expose connector.color and remove coloredConnectors reads.
   status: in_progress
 - id: ui-consumers
   content: Update sketchpad, algorithms, and UI consumers to read connector-local colors.
   status: pending
 - id: verification
   content: Extend existing tests and run focused Rust/TypeScript validation.
   status: pending
isProject: false
---

# Connector Color Store Plan

## Scope

- Work under the existing open ticket `Refactor Kit API To Object-Oriented With Centralized Change Management`; if implementation mode requires a ticket action, reopen or continue that ticket via repo MCP before editing.
- Primary files: [semio/rs/lib.rs](semio/rs/lib.rs), [semio/graphql/schema.graphql](semio/graphql/schema.graphql), [semio/graphql/local.schema.graphql](semio/graphql/local.schema.graphql), [semio/js/index.ts](semio/js/index.ts), [semio/react/index.tsx](semio/react/index.tsx), [semio/sketchpad/index.tsx](semio/sketchpad/index.tsx), [semio/algorithms/.storybook/stories/kit-store/commandSchema.ts](semio/algorithms/.storybook/stories/kit-store/commandSchema.ts), and any direct `semio/ui` consumers found during implementation.

## Rust and GraphQL

- Add a first-class `Color` DTO/scalar wrapper in `semio/rs/lib.rs` so GraphQL exposes `ConnectorStore.color: Color!` rather than a bare kit-wide string row. Keep serialization as the current CSS color string unless the codebase already has a richer color shape during final inspection.
- Add a cached `color` read on `ConnectorStore` using existing cache patterns. The cache dependency key will be derived from the connector port and the normalized compatible-port identity set for that port, so connectors sharing the same compatibility group resolve to the same color and only recompute when connector port or port compatibility changes.
- Invalidate connector color caches from the mutation paths that can affect it: connector port changes, port compatible-port changes, port id changes, connector id fallback changes, and graph rewire/import paths.
- Remove `KitColoredConnectorDto`, `kit_colored_connector_rows`, `ReadKitColoredConnectorsCommand`, and `KitStoreGraphql::colored_connectors` completely.
- Add `color` to `ConnectorMetadataDto`, `ConnectorShallowDto`, `ConnectorFullDto`, read command output/resolvers, and the generated GraphQL schema/local schema.

## Downstream Consumers

- In `semio/js/index.ts`, remove `KitColoredConnectorRowDto`, `readKitColoredConnectorsCommand`, `readColoredConnectors()`, `semioParseColoredConnectorRowsJson`, and `kitEventAffectsKitColoredConnectorsRead`; add `Color` and connector `color` typing/parsing/query selections wherever connector DTOs are read.
- In `semio/react/index.tsx`, remove `useKitColoredConnectors` and its import/dependency; rely on connector reads/hooks carrying `connector.color`.
- In `semio/sketchpad/index.tsx`, remove the local `colorPortsForTypes`/kit-wide coloring path and change connector rendering/port-colored type logic to use `connector.color` from the store. Keep selection compatibility overlays (selected, compatible, incompatible) as UI state layered on top of the connector base color.
- In `semio/algorithms/.storybook/stories/kit-store/commandSchema.ts`, remove the obsolete read command key. Audit `semio/ui` for any remaining `coloredConnectors` assumptions and switch to connector-local color when present.

## Verification

- Extend existing Rust tests in `semio/rs/lib.rs` to cover connector color derivation, compatibility grouping, cache stability, and invalidation after connector-port and port-compatibility changes. Do not create new test files.
- Run focused Rust checks/tests for `semio/rs`, regenerate/verify GraphQL schema output if the repo provides a build command, then run targeted TypeScript checks for `semio/js`, `semio/react`, `semio/sketchpad`, `semio/algorithms`, and `semio/ui` as available.
- Finish by closing the ticket via repo MCP with all edited files listed after implementation verification.
