# Energy Model Direct-Leaf Cutover

## Scope

- Mutation root: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
- Semantic mutation: `♻️replace-model` / `ReplaceModel` / `replace-model`
- Existing behavior remains authoritative: typed payload ownership, sparse diff construction, explicit inverse, committed language-neutral vector, text/binary round trips, and the public report bridge.

## Red Evidence

The scoped 17-rule structural policy reported four breaches before the cutover:

1. Missing direct Rust owner.
2. Mutation-specific match dispatch in the root text codec.
3. Hidden implementation wiring in the aggregate root.
4. Aggregate-root impurity from a hand-maintained kind list and hidden wiring.

## Cutover Contract

- Move the payload and `MutationKind` implementation directly to `♻️replace-model/🦀️component.rs`.
- Keep `🔺️diff`, `↩️inverse`, `📝️text`, `💾️binary`, and `🧪️tests` as optional leaf facets.
- Keep the aggregate root transparent: public leaf re-export, one-field enum wrapper, and structural correspondence only.
- Add the language-neutral descriptor, Draft-07 payload schema, TypeScript, GraphQL, protobuf, text, and binary leaf identities.
- Remove the nested `🦠️mutation` implementation facet and all `::mutation` routes.
- Preserve the public mutation report, test catalog, committed fixture, and wire round trips.

## Verification Ledger

- Scoped 17-rule structural policy: `0` breaches.
- Descriptor: accepted by the dependency-free repository validator and Ajv Draft-07.
- Payload schema: compiled by Ajv Draft-07.
- Direct Rust owner: parsed by `rustc +nightly-2026-07-07 -Zunpretty=ast-tree`.
- Scoped `rustfmt --check`: passed.
- Scoped `git diff --check`: passed.
- Stale direct-route, forbidden fallback-vocabulary, and `[DEBUG]` scan: clean.
- Registered runtime command: `bun nx run @semio-tech/energy-plugin:test-quick` reached the shared STDIO dependency and exited `1` before energy tests. The dependency is down from the prior 116 errors to 31: 29 stale glTF/TXT/JSON/XML/SVG consumers owned by the textual-base lane and two PDF semantic verbs rejected by `protocol::APPROVED_VERBS`. No diagnostic named an energy path or symbol. Exact remaining classes were routed to their owners before another shared build.
