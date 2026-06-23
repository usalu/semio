# Typed Semantic Kit Events

## Summary

Implemented typed semantic kit subscription events: Rust owns `SemanticKitEvent` (camelCase JSON via payload structs + externally tagged enum), each semantic row carries `KitChange` (`forward` / `inverse` / `kind`). Emitted from transaction `ChangeKitCommands`, GraphQL actor `ChangeKitCommands` / `ChangeKitWithInverse`, and kit-graph RPC paths that apply kit changes. JS exports `KitSemanticChangeEvent`, `SemanticKitEventWire`, `KitChangeWire`, `normalizeKitEventFromSubscription`, and strict entity filters that read semantic targets and command atoms. GraphQL `KitEvent` scalar documented for wire shape.

## Files touched

- `compose/rs/lib.rs` — `SemanticKitEvent`, `semantic_kit_event_from_kit_change`, `KitEvent::SemanticChange`, emission sites, tests (`gql_kit_event_semantic_change_serializes_renamed_design`, event stream assertions without `PartialEq` on `KitEvent`)
- `compose/graphql/schema.graphql` — `KitEvent` scalar documentation
- `compose/js/index.ts` — wire types, `KitEvent` union, normalization, filters, embedded Vitest

## Verification

- `cargo test` in `compose/rs` (exit 0)
- `npm --workspace @compose/js run build` (pass)
- `npm --workspace @compose/js run test` (21 tests)
- `npm run depcruise:layers` (no violations)

## Log

- Ticket opened for implementation.
- Implementation completed; ticket closed in `ticket.json`.
