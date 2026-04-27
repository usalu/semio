---
name: scoped graphql reads
overview: Refactor the Semio Rust GraphQL/store surface so reads are always scoped and undo/redo exists only at draft and transaction scope. The work should attach to the existing open `Scoped Kit Read Refactor` ticket once execution is approved.
todos:
 - id: ticket
   content: Reopen the existing `Scoped Kit Read Refactor` ticket before implementation and close it after verification.
   status: completed
 - id: rust-graphql
   content: Refactor `semio/rs/lib.rs` GraphQL query/mutation surface so all reads require `KitReadScopeInput` and live undo/redo disappears.
   status: in_progress
 - id: schema
   content: Regenerate or update `semio/graphql/schema.graphql` to match the Rust schema.
   status: pending
 - id: js-client
   content: Update `semio/js/index.ts` client queries to always use scoped reads.
   status: pending
 - id: tests
   content: Extend existing Rust and affected JS tests to cover scoped reads and scoped-only undo/redo.
   status: pending
 - id: verify
   content: Run focused Rust/JS/schema verification and fix regressions.
   status: pending
isProject: false
---

# Scoped GraphQL Reads Refactor

## Context

- Primary implementation file: [semio/rs/lib.rs](semio/rs/lib.rs).
- Generated/static schema file: [semio/graphql/schema.graphql](semio/graphql/schema.graphql).
- Downstream clients to adjust: [semio/js/index.ts](semio/js/index.ts) and, if compile feedback requires it, [semio/react/index.tsx](semio/react/index.tsx).

Current mismatch:

- `KitReadScope` and `ReadKitCommand` already exist and support scoped materialization:

```6575:6671:semio/rs/lib.rs
pub mod kit_read_scope {
    // ...
    pub enum KitReadScope {
        TheKit,
        Checkpoint { checkpoint_id: Id },
        Alternative { alternative_id: Id },
        Draft { session_id: Id, draft_id: Id },
        Transaction { session_id: Id, draft_id: Id, transaction_id: Id },
    }
    // ...
    pub fn resolve_read_graph(kit: &KitGraphRef, scope: &KitReadScope) -> Result<KitGraphRef> {
        // materializes the requested view
    }
}
```

- GraphQL still has an unscoped live read root and live undo/redo:

```27322:27337:semio/rs/lib.rs
impl RootQuery {
    async fn kit_store(&self, ctx: &Context<'_>) -> Result<KitStoreNode> {
        Ok(KitStoreNode(ctx.data::<KitGraphRef>()?.clone()))
    }

    async fn kit_read_scope(&self, ctx: &Context<'_>, scope: KitReadScopeInput) -> Result<KitStoreNode> {
        let g: KitGraphRef = ctx.data::<KitGraphRef>()?.clone();
        let rs = kit_read_scope_from_gql(scope)?;
        let view = crate::kit_read_scope::resolve_read_graph(&g, &rs).map_err(|e| Error::new(e.to_string()))?;
        Ok(KitStoreNode(view))
    }
}
```

## Implementation Plan

1. Reopen the existing open ticket `Scoped Kit Read Refactor` before editing, then close it after verification with the touched files.

2. Make GraphQL reads explicitly scoped.

- Remove `Query.kitStore` from `RootQuery` and the schema.
- Rename `kitReadScope(scope: ...)` to a single clean query, likely `kit(scope: KitReadScopeInput!): KitStore!`, so there is no Live/Store/Gql split in the read API.
- Keep `KitStoreNode` as an internal resolver wrapper only; it should represent a resolved scoped graph, not a public “live store”.
- Remove or rename fields that imply unscoped live reads, especially `liveFullDto`. Use scoped names such as `fullDto`, `metadata`, `typesShallow`, etc.
- Route every resolver through the scoped materialized graph. Avoid adding alternate live shortcuts.

3. Collapse live write/undo semantics.

- Remove `LiveBatchInput`, `LiveBatchCommandInput::Undo`, `Redo`, and live design/change mutation paths from GraphQL.
- Keep write operations under the already-scoped session/draft/transaction path.
- Preserve `UndoDraft`, `RedoDraft`, `UndoTransaction`, and `RedoTransaction` only.
- If any “the kit” write remains needed, model it as a draft/transaction lifecycle operation rather than a live mutation shortcut.

4. Align `semio/js` with the scoped GraphQL API.

- Remove the compatibility branch in `gqlRunWithReadScope` that rewrites `kitReadScope` to `kitStore` for `theKit`.
- Make all reads pass a `KitReadScopeInput`, including the main committed kit scope.
- Update snapshot, materialization, VCS state, canUndo/canRedo, and read command mapping to use the unified scoped query.
- Remove public wording that distinguishes live store reads from scoped reads.

5. Regenerate or update [semio/graphql/schema.graphql](semio/graphql/schema.graphql).

- Prefer the existing schema-build command if it is available and reliable.
- If schema generation is not usable, update the checked-in schema by matching the Rust async-graphql output shape.

6. Extend existing tests only.

- Add Rust tests inside the existing `#[cfg(test)] mod tests` in [semio/rs/lib.rs](semio/rs/lib.rs).
- Cover that GraphQL rejects/does not expose unscoped `kitStore` reads.
- Cover that scoped reads work for `theKit`, draft, and open transaction scopes.
- Cover that live undo/redo is absent from GraphQL, while draft and transaction undo/redo remain available.
- Add or adjust existing JS tests in [semio/js/index.ts](semio/js/index.ts) if client query rewiring changes expectations.

## Verification

- Run focused Rust checks for `semio/rs` (`cargo test` from `semio/rs`, and `cargo check` if compile feedback is faster).
- Run relevant JS checks for `semio/js` after client query updates.
- Run schema-related check/build if the GraphQL schema build command is present.
- Use lints/diagnostics on touched files after edits.
