---
name: Live Subscription Field Tree
overview: "Replace the opaque `Subscription { event: Json! }` with a typed live-query tree that mirrors `Query`, so any selection (down to a single scalar like `u` of a coordinate) becomes a fine-grained subscription. Add the missing id-based accessors on `Kit`, `Design`, `Type`, `Folder`, `Piece`, `Group`, `Connection`, `Connector`, `Port`, `Session`, etc. so the example path resolves end-to-end."
todos:
 - id: subscription_tree
   content: "Replace `type Subscription { event: Json! }` with a live-query mirror of Query (`session`, `wip`, `authoritative`, `conflicts`, `node(id)`, `entity(hash)`) and a region comment documenting live-query semantics."
   status: completed
 - id: kit_id_accessors
   content: "On `Kit`, replace bare `design`/`type` singulars with `design(id: ID!)`/`type(id: ID!)`; add id-based accessors for family/file/folder/author/concept/tag/quality/prop/attribute/stat."
   status: completed
 - id: design_id_accessors
   content: On `Design`, replace bare `piece`/`connection` with id-based variants; add `layer(id)`, `group(id)`, `author(id)`, `quality(id)`, `prop(id)`, `attribute(id)`, `stat(id)`.
   status: completed
 - id: type_id_accessors
   content: On `Type`, replace bare `connector`/`representation` with id-based; add `port(id)`, `concept(id)`, `tag(id)`, `quality(id)`, `prop(id)`, `attribute(id)`, `stat(id)`, `author(id)`. Keep `bestRepresentation`.
   status: completed
 - id: child_id_accessors
   content: Add id-based accessors on Folder (file/subFolder/family/type/design), Piece (prop/attribute/childPiece/childConnection), Connection/Connector/Port/File/Tag/Concept/Quality/Prop/Stat (attribute, quality, tag where applicable), Group (piece).
   status: completed
 - id: session_navigation
   content: "Add `Session.alternative(id: ID!): Alternative` and `Session.theKit: Version` so subscription paths mirror SessionCommandInput. Promote Checkpoint.change/edit id args from optional to required."
   status: completed
 - id: validate_schema
   content: "Re-run `compose/graphql/scripts/export-schema.ts`; ripgrep checks for remaining bare singular accessors and remaining `event: Json!`; validate a sample subscription doc with the user's full example path against the new SDL."
   status: completed
 - id: ticket_lifecycle
   content: Open ticket via repo MCP under goal r2602/runningsketchpad with this plan id, update ticket.md with problem/change/verification, close ticket on completion.
   status: completed
isProject: false
---

# Live Subscription Field Tree

## Goal

Make `subscription { wip { alternative(id: $alt) { kit { design(id: $des) { piece(id: $piece) { flatPosition { center { u } } } } } } } }` valid SDL and emit-on-change. Generally: any leaf reachable via `Query` must be subscribable. Subscription becomes a live-query mirror of Query — same root fields, same return types — so the _selection set_ alone determines what is tracked. Drop `event: Json!`.

## Design

### Semantics

- Each Subscription field returns the same type as the equivalent Query field (`Session`, `Graph`, `Node`, `Entity`, `ConflictConnection`).
- The server emits the resolved selection once on subscribe (initial value), then again whenever any reachable field changes (live-query). Granularity is fully driven by the client's selection set: select one scalar → emit only when that scalar changes.
- Errors propagate per emission; the stream stays open until the client unsubscribes.

### Schema replacement (`compose/graphql/target.schema.graphql` lines 8256-8259)

```graphql
# 🔴 Live subscription. Each field mirrors `Query`. The server emits the
# resolved selection on subscribe and re-emits whenever any field within
# the selection changes (live-query). Subscribe to a single leaf scalar to
# receive only that scalar's updates.
type Subscription {
 session: Session!
 wip: Graph!
 authoritative: Graph
 conflicts: ConflictConnection!
 node(id: ID!): Node
 entity(hash: ID!): Entity
}
```

`event: Json!` is removed (the new typed tree subsumes it; clients select what they want). The two prior subscription plans (`subscription_tree_mirrors_mutation_270e731b` and `single_subscription_endpoint_9d8dfb86`, both event-stream shaped) are superseded by this design.

### Required id-accessor additions on existing types

The example path requires `design(id: ID!)` on `Kit` and `piece(id: ID!)` on `Design`, but the convention should be uniform across the navigation. Add `<entity>(id: ID!): <Entity>` everywhere a `<entities>: <Entity>Connection` exists, replacing the current bare singular accessors (which today silently return "the first" or similar).

Concrete edits to `compose/graphql/target.schema.graphql`:

- `Kit` (~L7511): replace `design: Design` / `type: Type` with `design(id: ID!): Design` / `type(id: ID!): Type`; add `family(id: ID!)`, `file(id: ID!)`, `folder(id: ID!)`, `author(id: ID!)`, `concept(id: ID!)`, `tag(id: ID!)`, `quality(id: ID!)`, `prop(id: ID!)`, `attribute(id: ID!)`, `stat(id: ID!)`.
- `Design` (~L7013): replace `piece: Piece` / `connection: Connection` with id-based variants; add `layer(id: ID!)`, `group(id: ID!)`, `author(id: ID!)`, `quality(id: ID!)`, `prop(id: ID!)`, `attribute(id: ID!)`, `stat(id: ID!)`.
- `Type` (~L4694): replace `connector: Connector` / `representation: Representation` with id-based; add `port(id: ID!)`, `concept(id: ID!)`, `tag(id: ID!)`, `quality(id: ID!)`, `prop(id: ID!)`, `attribute(id: ID!)`, `stat(id: ID!)`, `author(id: ID!)`. (Keep `bestRepresentation: Representation` — it's not a lookup.)
- `Folder` (~L1507): add `file(id: ID!)`, `subFolder(id: ID!)`, `family(id: ID!)`, `type(id: ID!)`, `design(id: ID!)`.
- `Piece` (~L5801): add `prop(id: ID!)`, `attribute(id: ID!)`, `childConnection(id: ID!)`, `childPiece(id: ID!)`.
- `Connection` (~L5941): add `attribute(id: ID!)`.
- `Connector` (~L4439): add `attribute(id: ID!)`, `quality(id: ID!)`.
- `Port` (~L3909): add `attribute(id: ID!)`, `quality(id: ID!)`.
- `File` (~L1642): add `tag(id: ID!)`, `quality(id: ID!)`, `attribute(id: ID!)`.
- `Tag` / `Concept` / `Quality` / `Prop` / `Stat` artifact regions: add `attribute(id: ID!)`.
- `Group` (~L5653): add `piece(id: ID!)`.
- `Session` (~L7980): add `alternative(id: ID!): Alternative` and `theKit: Version` (mirrors `SessionCommandInput.alternative(id)` and `SessionCommandInput.theKit`).
- `Checkpoint` already has `change(id: ID): Change` / `edit(id: ID): Edit` — make these `id: ID!` for consistency with the rest of the navigation (drop the optional id semantics; there is no current call site that relies on omitting the id).

After this, the user's example resolves through:

```mermaid
flowchart LR
  Sub[Subscription] --> wip[wip: Graph]
  wip --> alt["alternative(id): Alternative"]
  alt --> kit[kit: Kit]
  kit --> design["design(id): Design"]
  design --> piece["piece(id): Piece"]
  piece --> flat[flatPosition: Position]
  flat --> center[center: Coordinate]
  center --> u["u: Float"]
```

### What this plan does NOT change

- Operation/Modification/Diff trees and `Event` enum stay as they are. Subscriptions track _state_, not events.
- Mutation tree (`session → theKit | alternative(id) → unsavedChange(id) → kit → …`) is unchanged.
- Rust resolvers (`compose/rs/lib.rs::gql::Subscription`) are not rewritten by this ticket — schema-only. A follow-up ticket wires `EventBus` updates into the new live-query tree (the existing bus already broadcasts the typed events needed to invalidate selections). The schema change is the contract; the resolver is the implementation.

## Verification

- Re-run the schema export script (`compose/graphql/scripts/export-schema.ts`) — must still parse.
- `rg "^  (design|type|piece|connection|port|connector|representation|attribute|file|folder|family|author|concept|tag|quality|prop|stat|childPiece|childConnection|layer|group|alternative|change|edit): \w+( |$)" compose/graphql/target.schema.graphql` — every match should be either a Connection-returning plural OR an `(id: ID!)`-taking singular; no bare singular accessors left.
- `rg "^\s+event: Json!" compose/graphql/target.schema.graphql` → 0 matches.
- Author a smoke `subscription` document with the full example path and run it through `graphql-js` `validate` (script in ticket folder).

## Ticket

Open ticket `🎫 Live Subscription Field Tree` under goal `🎯r2602🎯runningsketchpad` (sketchpad apps need fine-grained reactive selections). Files touched: `compose/graphql/target.schema.graphql`. Plan id: this plan's id.
