---
name: piece hierarchy reads
overview: Move piece hierarchy metadata from the `DesignStore.piecePlacement` aggregate to first-class `PieceStore` GraphQL fields, then update the JS and React read adapters so downstream consumers keep using the existing placement metadata hook shape without querying the removed design field.
todos:
 - id: rust-piece-fields
   content: Add Rust PieceStore hierarchy fields and remove DesignStore piecePlacement aggregate.
   status: completed
 - id: graphql-regenerate
   content: Regenerate and inspect the GraphQL schema for the new PieceStore shape.
   status: completed
 - id: js-read-adapter
   content: Update semio/js getPiecesMetadata to query PieceStore fields and rebuild the existing metadata map.
   status: completed
 - id: react-downstream
   content: Adjust React hook comments/types and update downstream consumers that referenced the removed aggregate directly.
   status: completed
 - id: verify
   content: Run focused Rust, GraphQL, JS/React, and downstream validation commands.
   status: completed
isProject: false
---

# Piece Hierarchy Read Refactor

## Scope

- Primary files: [semio/rs/lib.rs](semio/rs/lib.rs), [semio/graphql/schema.graphql](semio/graphql/schema.graphql), [semio/js/index.ts](semio/js/index.ts), [semio/react/index.tsx](semio/react/index.tsx).
- Downstream validation focus: [semio/sketchpad/index.tsx](semio/sketchpad/index.tsx), [semio/algorithms](semio/algorithms), [semio/ui/index.tsx](semio/ui/index.tsx), plus package checks that cover those consumers.
- Implementation should attach to the existing Rust/GraphQL store cleanup workstream if its open ticket is still active; otherwise open a narrow ticket for this schema/read-path change before editing.

## Current Shape

`DesignStore` currently exposes an aggregate field:

```300:312:semio/graphql/schema.graphql
	type DesignStore {
		pieces: [Piece!]!
		pieceByDtoId(id: String!): Piece
		connections: [Connection!]!
		piecesFull: [PieceFullDto!]!
		connectionsFull: [ConnectionFullDto!]!
		flattenMap: [DesignFlattenMapEntryDto!]!
		clusterableGroups(selection: [String!]!): [[String!]!]!
		qualitySum(qualityId: String!): Float!
		replaceableCatalog(selection: [String!]!): ReplaceableCatalog!
		includedDesigns: [IncludedDesignInfoDto!]!
		includedDesignIds: [String!]!
		piecePlacement: [PiecePlacementMetadataDto!]!
	}
```

`semio/js` reads that field only to populate placement metadata for React hooks:

```2602:2610:semio/js/index.ts
async getPiecesMetadata(scope: KitReadScope, designId: string): Promise<ReadonlyMap<string, PiecePlacementRowDto>> {
  const d = kitGraphqlData(
    await this.gqlRunWithReadScope(scope, {
      query: `query($scope: KitReadScopeInput!, $id: String!) { kit(scope: $scope) { designByDtoId(id: $id) { piecePlacement { pieceId fixedPieceId parentPieceId depth path plane { origin { x y z } xAxis { x y z } yAxis { x y z } } center { x y z } } } } } }`,
      variables: { id: designId },
    }),
  ) as { kit?: { designByDtoId?: { piecePlacement?: readonly JsonValue[] } | null } | null };
  const rows = gqlDataKitRoot(d)?.designByDtoId?.piecePlacement;
```

## Implementation Plan

- In `semio/rs/lib.rs`, remove `PiecePlacementMetadataDto`, `KitGraph::piece_placement_metadata`, `get_pieces_metadata_json`, and the `DesignNode::piece_placement` resolver.
- Add `PieceStore` hierarchy helpers/resolvers:
  - `parentPiece: Piece` as nullable GraphQL field, backed by `PieceStore.parent_piece`.
  - `depth: Int!`, derived from the parent chain length.
  - `path: [Piece!]!`, returned as piece nodes from root/fixed piece through the current piece.
- Keep `flatPlane`, `flatCenter`, and `flatPose` on `PieceStore`; these replace the removed aggregate’s `plane` and `center` data for callers that still need placement rows.
- Regenerate `semio/graphql/schema.graphql` with the existing `npx nx build semio/graphql` path so SDL is produced from Rust, not hand-maintained.
- In `semio/js/index.ts`, replace the `designByDtoId { piecePlacement ... }` query with `designByDtoId { pieces { id parentPiece { id } depth path { id } flatPlane ... flatCenter ... } }`, then keep returning `ReadonlyMap<string, PiecePlacementRowDto>` for now by mapping:
  - `pieceId = piece.id`
  - `parentPieceId = piece.parentPiece?.id ?? null`
  - `depth = piece.depth`
  - `path = piece.path.map(p => p.id)`
  - `plane = piece.flatPlane`
  - `center = piece.flatCenter`
  - `fixedPieceId = path[0] ?? piece.id` only as a compatibility field inside JS/React until downstream hooks are renamed.
- Update the JS zod parser/query result typing to validate the new piece-store response, not `PiecePlacementMetadataDto`.
- In `semio/react/index.tsx`, keep `usePiecesMetadataMap`, `usePieceMetadata`, `useIsConnectedPiece`, `usePieceDepth`, `useFixedPieceId`, and `useParentPieceId` behavior stable initially, then rename internal comments from placement aggregate wording to piece hierarchy metadata. No domain computation should move into React.
- Search downstream bundles for `piecePlacement`, `PiecePlacementMetadataDto`, `fixedPieceId`, `parentPieceId`, and `piecesMetadataFor`; remove direct schema references and ensure all consumers flow through `semio/react` hooks or `semio/js` store reads.

## Validation

- Run `npx nx build semio/graphql` and confirm `schema.graphql` no longer contains `piecePlacement` or `PiecePlacementMetadataDto`, and `PieceStore` contains `parentPiece`, `depth`, and `path`.
- Run Rust checks focused on schema and piece hierarchy tests, including the existing GraphQL schema parity test in `semio/rs/lib.rs`.
- Run JS/React checks that cover the GraphQL query string and parser tests.
- Run targeted downstream checks for sketchpad/algorithms/ui consumers that rely on piece metadata, then broaden only if failures indicate shared API fallout.
