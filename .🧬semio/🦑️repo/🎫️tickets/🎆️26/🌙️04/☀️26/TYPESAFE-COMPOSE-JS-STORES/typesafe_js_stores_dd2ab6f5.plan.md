---
name: typesafe js stores
overview: Make `compose/js` expose only clean, fully typed store facades with typed methods and per-event subscription methods, backed by Rust/GraphQL event and command definitions. The implementation will reuse the existing Rust `KitEvent` and GraphQL schema generation flow, then migrate `compose/react` and `compose/sketchpad` away from raw command and wire APIs.
todos:
 - id: event-contract
   content: Type the Rust/GraphQL event contract and add schema/subscription coverage.
   status: completed
 - id: js-store-api
   content: Refactor compose/js to typed store methods and per-event subscriptions with no public command leaks.
   status: completed
 - id: react-boundary
   content: Migrate compose/react to consume and expose only clean typed store APIs.
   status: completed
 - id: sketchpad-migration
   content: Remove sketchpad kitWire/string-command usage and route mutations through typed stores.
   status: completed
 - id: validation
   content: Run Rust, JS, React, and Sketchpad targeted tests and close the repo ticket.
   status: completed
isProject: false
---

# Typesafe compose/js Stores

## Scope

- Reopen the existing matching ticket `Single Async Kitstore Export In compose/js` and attach this work to the `Running Sketchpad` or `Kit App` goal depending on the repo tool's current accepted goal slug.
- Primary files:
  - [compose/rs/lib.rs](compose/rs/lib.rs)
  - [compose/graphql/schema.graphql](compose/graphql/schema.graphql)
  - [compose/js/index.ts](compose/js/index.ts)
  - [compose/react/index.tsx](compose/react/index.tsx)
  - [compose/sketchpad/index.tsx](compose/sketchpad/index.tsx)

## Implementation Plan

1. Extend the Rust/GraphQL event contract.
   - Keep `compose/rs` as the source of truth for event kinds.
   - Replace or augment the current opaque GraphQL `scalar KitEvent` with explicit generated event payload types, or add a generated JSON-schema/TypeScript export from the Rust `KitEvent` enum if GraphQL unions are too bulky.
   - Add subscription tests in `compose/rs/lib.rs` near `kit_graphql_smoke` so emitted event payloads are stable and schema export updates `compose/graphql/schema.graphql`.

2. Create a typed `compose/js` event layer.
   - Replace `export type KitEvent = Readonly<Record<string, unknown>>` with a discriminated union matching Rust event variants.
   - Add typed narrowers and subscription helpers for every top-level event variant, including command lifecycle events, graph invalidation events, entity events, design/piece/type nested events, and set rejection events.
   - Keep RxJS private and expose only callback subscription methods returning `Unsubscribe`.

3. Replace public command leaks with clean store methods.
   - Remove or internalize public string/unknown command surfaces such as `executeComposeKitCommand`, `createKitCommandEngine`, `KitTypedShellCommand`, `patchField(field: string, value: unknown)`, and low-level command batch entry points where app code can use typed methods instead.
   - Add typed methods on `KitStore`, `DesignStore`, `TypeStore`, `PieceStore`, `ConnectionStore`, `FamilyStore`, `FileStore`, and `FolderStore` for each supported mutation and read path.
   - Keep command construction and GraphQL/Rust command mapping inside `compose/js` only.

4. Add per-event subscription methods to stores.
   - `KitStore` gets one typed subscription method per top-level `KitEvent` variant, plus generic `subscribeEvent` only if it remains fully typed.
   - Entity stores get scoped subscriptions such as design/piece/type/connection-specific events, returning typed narrowed payloads rather than `Record<string, unknown>`.
   - Derived read stores keep invalidation-only subscriptions where appropriate, but their internal filters must consume typed events.

5. Migrate `compose/react` to be a pure typed-store consumer.
   - Stop re-exporting raw command/wire APIs upward.
   - Move command wire construction and event-affects filtering into `compose/js`.
   - Preserve React hook signatures where possible, but update internals and embedded tests to use typed store/client mocks.

6. Migrate `compose/sketchpad` away from `kitWire`.
   - Replace `kitWire: { command: string; args: unknown[] }` command results with direct typed store method calls exposed through `compose/react` hooks.
   - Keep app state commands and UI transactions, but route kit mutations through typed store methods only.
   - Resolve the name collision between sketchpad app `DesignStore` and exported entity `DesignStore` by using clearer local names if needed.

## Verification

- Run or add focused checks in existing files only:
  - Rust GraphQL/schema tests for generated schema and subscription event payloads.
  - `compose/js` embedded Vitest coverage for event parsing, every subscription method, and store methods without raw command leaks.
  - `compose/react` embedded Vitest for typed store consumption.
  - Relevant `compose/sketchpad` Playwright slices for kit/design/type flows after `kitWire` removal.
- Finish by closing the reopened ticket with the complete touched file list and verification results.
