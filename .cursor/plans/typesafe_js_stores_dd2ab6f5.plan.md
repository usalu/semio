---
name: typesafe js stores
overview: Make `semio/js` expose only clean, fully typed store facades with typed methods and per-event subscription methods, backed by Rust/GraphQL event and command definitions. The implementation will reuse the existing Rust `KitEvent` and GraphQL schema generation flow, then migrate `semio/react` and `semio/sketchpad` away from raw command and wire APIs.
todos:
 - id: event-contract
   content: Type the Rust/GraphQL event contract and add schema/subscription coverage.
   status: in_progress
 - id: js-store-api
   content: Refactor semio/js to typed store methods and per-event subscriptions with no public command leaks.
   status: pending
 - id: react-boundary
   content: Migrate semio/react to consume and expose only clean typed store APIs.
   status: pending
 - id: sketchpad-migration
   content: Remove sketchpad kitWire/string-command usage and route mutations through typed stores.
   status: pending
 - id: validation
   content: Run Rust, JS, React, and Sketchpad targeted tests and close the repo ticket.
   status: pending
isProject: false
---

# Typesafe semio/js Stores

## Scope

- Reopen the existing matching ticket `Single Async Kitstore Export In semio/js` and attach this work to the `Running Sketchpad` or `Kit App` goal depending on the repo tool's current accepted goal slug.
- Primary files:
  - [semio/rs/lib.rs](semio/rs/lib.rs)
  - [semio/graphql/schema.graphql](semio/graphql/schema.graphql)
  - [semio/js/index.ts](semio/js/index.ts)
  - [semio/react/index.tsx](semio/react/index.tsx)
  - [semio/sketchpad/index.tsx](semio/sketchpad/index.tsx)

## Implementation Plan

1. Extend the Rust/GraphQL event contract.
   - Keep `semio/rs` as the source of truth for event kinds.
   - Replace or augment the current opaque GraphQL `scalar KitEvent` with explicit generated event payload types, or add a generated JSON-schema/TypeScript export from the Rust `KitEvent` enum if GraphQL unions are too bulky.
   - Add subscription tests in `semio/rs/lib.rs` near `kit_graphql_smoke` so emitted event payloads are stable and schema export updates `semio/graphql/schema.graphql`.

2. Create a typed `semio/js` event layer.
   - Replace `export type KitEvent = Readonly<Record<string, unknown>>` with a discriminated union matching Rust event variants.
   - Add typed narrowers and subscription helpers for every top-level event variant, including command lifecycle events, graph invalidation events, entity events, design/piece/type nested events, and set rejection events.
   - Keep RxJS private and expose only callback subscription methods returning `Unsubscribe`.

3. Replace public command leaks with clean store methods.
   - Remove or internalize public string/unknown command surfaces such as `executeSemioKitCommand`, `createKitCommandEngine`, `KitTypedShellCommand`, `patchField(field: string, value: unknown)`, and low-level command batch entry points where app code can use typed methods instead.
   - Add typed methods on `KitStore`, `DesignStore`, `TypeStore`, `PieceStore`, `ConnectionStore`, `FamilyStore`, `FileStore`, and `FolderStore` for each supported mutation and read path.
   - Keep command construction and GraphQL/Rust command mapping inside `semio/js` only.

4. Add per-event subscription methods to stores.
   - `KitStore` gets one typed subscription method per top-level `KitEvent` variant, plus generic `subscribeEvent` only if it remains fully typed.
   - Entity stores get scoped subscriptions such as design/piece/type/connection-specific events, returning typed narrowed payloads rather than `Record<string, unknown>`.
   - Derived read stores keep invalidation-only subscriptions where appropriate, but their internal filters must consume typed events.

5. Migrate `semio/react` to be a pure typed-store consumer.
   - Stop re-exporting raw command/wire APIs upward.
   - Move command wire construction and event-affects filtering into `semio/js`.
   - Preserve React hook signatures where possible, but update internals and embedded tests to use typed store/client mocks.

6. Migrate `semio/sketchpad` away from `kitWire`.
   - Replace `kitWire: { command: string; args: unknown[] }` command results with direct typed store method calls exposed through `semio/react` hooks.
   - Keep app state commands and UI transactions, but route kit mutations through typed store methods only.
   - Resolve the name collision between sketchpad app `DesignStore` and exported entity `DesignStore` by using clearer local names if needed.

## Verification

- Run or add focused checks in existing files only:
  - Rust GraphQL/schema tests for generated schema and subscription event payloads.
  - `semio/js` embedded Vitest coverage for event parsing, every subscription method, and store methods without raw command leaks.
  - `semio/react` embedded Vitest for typed store consumption.
  - Relevant `semio/sketchpad` Playwright slices for kit/design/type flows after `kitWire` removal.
- Finish by closing the reopened ticket with the complete touched file list and verification results.
