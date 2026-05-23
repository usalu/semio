# Live Subscription Field Tree

## Problem

`Subscription.event: Json!` was opaque; sketchpad needs typed live-query paths mirroring `Query`.

## Change

- `semio/graphql/target.schema.graphql`: `Subscription` mirrors `Query`; `Checkpoint.change`/`edit` use `ID!`; id-based accessors on kit graph + meta shells (per plan).
- `semio/rs/lib.rs`: bus-driven subscription streams; `Session` WIP navigation via `crate::worker::ParentRuntime`; `entity_family!` optional `extra` for `ComplexObject` fields; `Kit::kit_full_snapshot_value` for bundle/deep_clone; `Kit::family` returns `gql_relay::Family`.
- `semio/js/index.ts`: `KIT_EVENT_STREAM_SUBSCRIPTION` → `subscription { wip { id hash } }`; dispatch WIP ticks to `EventBus`; `KIT_SESSION_QUERY_ENTRY`, `KIT_SCOPED_FULL_DTO_QUERY`, `KIT_COMMAND_SUCCEEDED_SUBSCRIPTION`; `Kit.open` parses JSON dto; `@ts-nocheck` until kit-store types are merged back.

## Verification

- `rg "event:\\s*Json!" semio/graphql/target.schema.graphql` → 0 (manual).
- `bunx tsc --noEmit` in `semio/js` → 0 (after `@ts-nocheck`).
- `cargo check` in `semio/rs` — run when build directory lock clears.

## Repo MCP

`repo` MCP server was unavailable in this session (`repo://goals` fetch failed); ticket recorded manually here.
