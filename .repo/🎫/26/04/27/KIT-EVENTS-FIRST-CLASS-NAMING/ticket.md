# Kit Events First Class Naming

## Summary

Flattened kit mutation events onto a single `KitEvent` tree: removed `SemanticChange` / `SemanticKitEvent`; Rust payloads are `*KitEvent` structs (`RenamedDesignKitEvent`, `ChangedKitEvent`, …) with `#[serde(rename = "camelCase")]` variant tags on `KitEvent`. Classifier is `kit_event_from_kit_change`. GraphQL scalar wrapper renamed `GqlKitEvent` → `KitEventScalar`. JS: `KitClassifiedMutationEvent`, per-payload `*KitEventWire` types, `isKitClassifiedMutationEvent`, filters use `kitClassifiedMutationTouches*`; subscription normalization no longer unwraps `SemanticChange`.

## Files

- `compose/rs/lib.rs`
- `compose/js/index.ts`
- `compose/graphql/schema.graphql`

## Verification

- `cargo test tests::events::backbone` (and `gql_kit_event` tests)
- `npm --workspace @compose/js run build` + `test`
- `npm run depcruise:layers`
