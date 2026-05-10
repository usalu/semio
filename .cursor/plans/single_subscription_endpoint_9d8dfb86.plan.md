---
name: single subscription endpoint
overview: "Replace the flat per-event Subscription block with a single `operation: Operation!` endpoint, and add a `FailedOperation` concrete type so async failures travel through the same feed."
todos:
  - id: replace_subscription
    content: "Replace type Subscription block (L5979-6097) with `Subscription { operation: Operation! }`."
    status: pending
  - id: add_failed_operation
    content: Add FailedOperation type + FailedOperationEdge + FailedOperationConnection implementing Operation/EntityEdge/EntityConnection.
    status: pending
  - id: drop_orphan_payloads
    content: Delete unused `type Command` and `type Error`; rename region to OperationPayloads.
    status: pending
  - id: validate
    content: Run graphql parse + buildSchema; rg sweeps for removed names.
    status: pending
  - id: ticket_update
    content: Append problem/change/verification to .repo/🎫/26/05/10/GRAPHQL-TARGET-MUTATION-CLEANUP/ticket.md.
    status: pending
isProject: false
---

# Single Subscription Endpoint

## Why
With the async command/operation architecture in place, dynamic per-event subscription fields add no expressive power: every concrete operation already implements the `Operation` interface ([semio/graphql/target.schema.graphql L956-964](semio/graphql/target.schema.graphql)), so a single feed plus inline fragments is strictly more flexible than dozens of named fields. The flat block at L5979-6097 is replaced wholesale.

## Schema Changes — `semio/graphql/target.schema.graphql`

### 1. Replace the entire `type Subscription` block (L5979-6097)

```graphql
type Subscription {
  # 📡 Single async feed of every operation produced by any mutation.
  # Clients discriminate concrete operations (RenamedTag, MovedPiece, FailedOperation, ...) via inline fragments.
  operation: Operation!
}
```

That is the whole block. No `commandSucceeded`, no `operationSucceeded`, no `operationFailed`, no `error`, no per-entity event fields.

### 2. Add `FailedOperation` concrete type in the `#region SubscriptionPayloads` (currently L5764-5774, next to `Command` / `Error`)

`Operation` is the only payload of the new feed, so failures must implement it to be deliverable:

```graphql
type FailedOperation implements Operation & Entity {
  id: ID! # data // uuidv7
  hash: String! # cached
  owner: Entity # reference // Change
  owns: EntityConnection # reference
  modification: Modification! # computed
  scope: Entity! # reference
  message: String! # data // failure reason
}
```

Plus the matching edge/connection pair to follow the schema's `*Edge` / `*Connection` convention used by every other concrete operation:

```graphql
type FailedOperationEdge implements EntityEdge {
  cursor: String! # computed
  node: FailedOperation! # reference
}

type FailedOperationConnection implements EntityConnection {
  edges: [FailedOperationEdge!]! # computed
  pageInfo: PageInfo! # computed
  hash: String! # cached
}
```

### 3. Retire payload types that the new endpoint no longer references

`Command` (L5766-5768) and `Error` (L5770-5772) only existed to feed the now-deleted `commandSucceeded` / `operationFailed` / `error` fields. They are no longer reachable from the schema root. Delete both, and rename `#region SubscriptionPayloads` to `#region OperationPayloads` to reflect that only `FailedOperation` and friends live there.

If `grep` finds any remaining references to `Command` or `Error` outside that region, drop those references too — there should be none after the Subscription replacement.

## Out of Scope
- The four missing concrete operation types (`StartedSession`, `EndedSession`, `RenamedDesign`, `UpdatedDesignDescription`) noted in earlier work. They remain a separate cleanup; the single endpoint does not depend on them existing.
- Resolver / backend wiring. Schema-only change.

## Verification
- `node -e "require('graphql').parse(require('fs').readFileSync('semio/graphql/target.schema.graphql','utf8'))"` → parse OK
- `node -e "require('graphql').buildSchema(require('fs').readFileSync('semio/graphql/target.schema.graphql','utf8'))"` → build OK
- `rg -n "commandSucceeded|operationSucceeded|operationFailed|^\s*error: Error" semio/graphql/target.schema.graphql` → no matches
- `rg -n "renamedKit|createdTag|addedConnector|movedPiece|deletedDesign" semio/graphql/target.schema.graphql` → only the concrete `Renamed*` / `Created*` / `Added*` / `Moved*` / `Deleted*` operation type definitions, no Subscription field references
- `rg -n "^type (Command|Error) " semio/graphql/target.schema.graphql` → no matches
- `rg -n "FailedOperation" semio/graphql/target.schema.graphql` → type, edge, connection definitions only

## Ticket Update
Append to `.repo/🎫/26/05/10/GRAPHQL-TARGET-MUTATION-CLEANUP/ticket.md`:
- Problem: flat per-event Subscription duplicates the Operation interface and conflicts with the async architecture.
- Change: collapse to `Subscription { operation: Operation! }`; add `FailedOperation` (+ edge/connection) implementing `Operation` so failures share the feed; remove now-orphaned `Command` / `Error` payload types.
- Verification: parse + build commands above, plus the rg sweeps.