---
name: typesafe semio boundary
overview: Make `semio/rs`, `semio/js`, and `semio/react` expose specific, typed APIs instead of generic JSON/string/unknown surfaces. The refactor will continue the existing `Typesafe Semio Js Stores` ticket and treat Rust/GraphQL as the source of truth, with JS and React as strictly typed consumers.
todos:
 - id: boundary-audit
   content: Audit public generic and untyped surfaces across rs/js/react and record exact replacements.
   status: completed
 - id: rust-graphql-types
   content: Refactor Rust and GraphQL command, read, and event boundaries to explicit typed shapes.
   status: completed
 - id: js-store-types
   content: Refactor semio/js stores and transport parsing to expose specific typed methods and outputs.
   status: completed
 - id: react-typed-hooks
   content: Refactor semio/react hooks and host command routing to consume only typed semio/js APIs.
   status: completed
 - id: verification
   content: Extend existing tests in place and run focused Rust, JS, and React validation.
   status: completed
isProject: false
---

# Typesafe Semio Boundary

## Scope

Continue the existing open ticket `Typesafe Semio Js Stores` and associate the work with the `Running Sketchpad` goal, because this is the active store boundary that feeds sketchpad/play through `semio/react`.

Primary files:

- [semio/rs/lib.rs](semio/rs/lib.rs)
- [semio/graphql/schema.graphql](semio/graphql/schema.graphql)
- [semio/js/index.ts](semio/js/index.ts)
- [semio/react/index.tsx](semio/react/index.tsx)

Do not widen into other technologies unless needed to repair direct consumers. Existing embedded test sections will be extended in place; no new test files.

## Implementation Plan

1. Establish the typed boundary contract.

- Audit public exports in `semio/rs`, `semio/js`, and `semio/react` for `serde_json::Value`, `Record<string, unknown>`, `unknown`, `any`, generic read rows, field-string patches, and string command dispatch.
- Classify each occurrence as either an internal serialization detail or a public boundary leak. Public leaks are removed; internal serialization stays behind typed parse/format functions.

2. Make Rust and GraphQL specific.

- Replace stringly event fields with typed enums/objects where still present, especially command lifecycle results and status/kind fields.
- Replace JSON-based field patch/add-child paths with explicit command input variants and typed DTO payloads.
- Replace undo snapshots and read helper outputs with typed snapshot/read structures instead of loose JSON values where they cross module boundaries.
- Regenerate/update `semio/graphql/schema.graphql` from the Rust schema so reads, writes, lifecycle events, and command outputs are explicit GraphQL shapes instead of JSON scalar tunnels.

3. Make `semio/js` a typed transport/store facade.

- Replace `ReadKitCommandOutput = Record<string, unknown>` and generic batch result rows with a discriminated mapping from each `ReadKitCommand` to its exact output type.
- Replace public `unknown` command results and patch helpers with named command methods and specific patch DTOs per entity.
- Keep GraphQL/wasm parsing internal, with Zod or direct typed guards at the boundary, but expose only typed `KitStore`, entity stores, events, command receipts, and lifecycle subscriptions.
- Remove public command-wire leaks where React can call a specific store method instead.

4. Make `semio/react` a strict typed consumer.

- Remove `// @ts-nocheck` only after the store surface is strong enough for compilation.
- Replace `KitHostGraphOp` payloads from `unknown`/`readonly unknown[]` with exact DTO and id types.
- Replace `executeSemioKitCommand(command: string, ...args: unknown[])` and child-kind string dispatch with typed React hooks or typed store calls.
- Replace `HookTriad<any>`, `readonly unknown[]`, and schema-hook string dispatch with concrete hooks and typed return values.

5. Verify and tighten.

- Extend Rust tests in `semio/rs/lib.rs` around GraphQL schema parity, command input/output typing, lifecycle events, and read results.
- Extend embedded JS Vitest tests in `semio/js/index.ts` for typed read outputs, typed events, and typed store methods.
- Extend embedded React tests in `semio/react/index.tsx` for typed hook consumption and command routing.
- Run targeted Rust, JS, and React checks first, then broader affected build/test commands if the targeted checks pass.

## Acceptance Criteria

- `semio/react/index.tsx` no longer uses `// @ts-nocheck`.
- Public exports from `semio/js` and `semio/react` do not expose `any`, `unknown`, `Record<string, unknown>`, generic command strings, or generic field-patch APIs for kit domain operations.
- Public Rust/GraphQL mutation/read/event surfaces use explicit types; JSON is only an internal serialization boundary.
- Existing sketchpad/play consumers compile against the typed API without compatibility aliases for removed generic APIs.
- Relevant Rust, JS, and React tests pass after the refactor.
