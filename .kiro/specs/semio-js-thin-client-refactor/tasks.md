# Implementation Plan: semio-js-thin-client-refactor

## Overview

Transform `semio/js/index.ts` (~31k lines) into a pure OO thin client by: (1) deleting all domain logic classes, caching, free functions, and legacy stores, (2) retaining Zod schemas, type aliases, entity classes (with OO methods only), WASM bridge, wire types, read command types, GraphQL wire layer, live read facades, constants, and Generator, (3) adding the `Semio` utility class and thin `Kit` class, (4) updating embedded tests to use the WASM bridge, and (5) adding `fast-check` for property-based tests. All code stays in `semio/js/index.ts` using regions.

## Tasks

- [ ] 1. Add fast-check dev dependency and prepare test infrastructure
  - [x] 1.1 Add `fast-check` as a dev dependency in `semio/js/package.json`
    - Add `"fast-check": "^3.x"` to `devDependencies`
    - Run `npm install` or equivalent to update lockfile
    - _Requirements: 8.1_

  - [-] 1.2 Verify existing test runner configuration works with embedded tests
    - Confirm `cross-env SEMIO_JS_RUN_EMBEDDED_TESTS=1 vitest run` executes successfully before any deletions
    - _Requirements: 8.1_

- [~] 2. Checkpoint - Ensure baseline tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 3. Delete all domain logic classes
  - [~] 3.1 Delete `KitImpl` class and all its methods, private fields (`#historyDone`, `#historyUndone`, `#flattenMerkleByDesign`, `ops`, `transactions`, `_applyDiff`, `replayChangeUnchecked`, etc.)
    - Remove the entire `KitImpl` class definition and any helper code only used by it
    - _Requirements: 1.15, 2.1, 2.2, 2.5, 9.3_

  - [~] 3.2 Delete `KitEntity`, `KitEntityIndexes`, `KitEntityCaches`, `KitInteractionsApi`, `KitInteractionEntity`, `KitEntityType`, `KitEntityPiece`, `KitEntityDesign`, `KitDocument`, `KitBackboneBridge` classes
    - Remove all class definitions and any helper code only used by them
    - _Requirements: 9.5_

  - [~] 3.3 Delete `KitTypesOps`, `KitDesignsOps`, `KitFamiliesOps`, `KitFilesOps`, `KitTagsOps`, `KitConceptsOps`, `KitAttributesOps`, `KitOps` classes
    - Remove all class definitions and any helper code only used by them
    - _Requirements: 9.6_

  - [~] 3.4 Delete `KitTransactionsCoordinator`, `KitActiveTransactionSurface`, `Transaction`, `DiffComposer`, `recomputeTxNet` and all transaction/history management code
    - _Requirements: 1.15_

  - [~] 3.5 Delete `InMemoryKitStore` and legacy `KitStore` (JS-side) classes
    - _Requirements: 9.4_

- [ ] 4. Delete all caching code
  - [~] 4.1 Delete `FlatMerkleCacheEntry`, `ensureFlattenGeometryCache`, `getFlattenMerkleCache`, `invalidateFlattenMerkleCaches`, `piecesMetadataCached`, `cachedSqlJs` and all cache-related code
    - Remove all flatten merkle cache maps, SQL.js cache, and any cache management methods
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [ ] 5. Delete all free functions
  - [~] 5.1 Delete all free serialization functions (`serializeKit`, `deserializeKit`, `serializeType`, `deserializeType`, and all per-entity `serialize*`/`deserialize*` variants)
    - _Requirements: 3.1_

  - [~] 5.2 Delete all free ID factory functions (`createTypeId`, `areSameTypeId`, `getTypeId`, and all per-entity `create*Id`/`areSame*Id`/`get*Id` variants)
    - _Requirements: 3.2_

  - [~] 5.3 Delete all free utility functions (`normalize`, `round`, `deepEqual`, `arraysEqual`, `generateUniqueName`, `cn`, `id`, `jaccard`)
    - _Requirements: 3.5, 1.3_

  - [~] 5.4 Delete all free geometry helper functions (`roundPlane`, `serializePlane`, `deserializePlane`, and all per-geometry diff/serialize/deserialize free functions)
    - _Requirements: 3.6_

  - [~] 5.5 Delete all free design-diff builder functions (`addPieceToDesignDiff`, `removePieceFromDesignDiff`, `addConnectionToDesignDiff`, `removeConnectionFromDesignDiff`, `setPieceInDesignDiff`, etc.)
    - _Requirements: 3.7_

  - [~] 5.6 Delete all free collection-diff functions (`getCollectionDiff`, `inverseCollectionDiff`, `applyCollectionDiff`, `mergeCollectionDiff`)
    - _Requirements: 3.8_

  - [~] 5.7 Delete all free backbone factory functions (`createLocalBackbone`, `createDevBackbone`, `createRemoteBackbone`)
    - _Requirements: 3.9_

  - [~] 5.8 Delete all free kit conversion functions (`asKitInstance`, `requireKit`, `duplicateKitForIsolation`, `stripNullsJsonClone`, `detachPieceForLocalMutation`, `detachConnectionForLocalMutation`, `detachDesignForLocalMutation`, `designWithDiff`)
    - _Requirements: 3.10_

- [ ] 6. Delete all domain logic functions
  - [~] 6.1 Delete all flatten geometry functions (`computeChildPlane`, `connectionPlacementTranslationBasis`, `flattenPlacementWalkDesignOrderRoots`, `buildFlattenPieceAdjacency`, `collectUndirectedComponentIds`, `moveTranslationWorldFromPiecePlane`, `childConnectorOriginWorld`, `solveConnectionOriginMinNorm`, `connectionDiffFromStructuralMoveVector`)
    - _Requirements: 1.1, 1.2_

  - [~] 6.2 Delete all representation selection functions (`selectBestRepresentation`, `filterRepresentationsByTagIds`, `getAvailableTagIdsForRepresentations`, `getAllTagIdsFromRepresentations`)
    - _Requirements: 1.3_

  - [~] 6.3 Delete all type/design/piece resolution functions (`resolvePieceTypeForFlatten`, `findTypeInKit`, `findDesignInKit`, `findPieceInDesign`, `findConnectionInDesign`)
    - _Requirements: 1.4_

  - [~] 6.4 Delete all connector compatibility functions (`arePortsCompatible`, `areConnectorsCompatible`, `unifyConnectorPortsAndCompatiblePortsForTypes`)
    - _Requirements: 1.5_

  - [~] 6.5 Delete all diff computation, application, inversion, and merge functions (`getTypeDiff`, `applyTypeDiff`, `inverseTypeDiff`, `mergeTypeDiff`, `getDesignDiff`, `applyDesignDiff`, `inverseDesignDiff`, `mergeDesignDiff`, `getKitDiff`, `applyKitDiff`, `inverseKitDiff`, `mergeKitDiff`, and all variants)
    - _Requirements: 1.6, 1.7, 1.8, 1.9_

  - [~] 6.6 Delete all hashing functions (`hashType`, `hashDesign`, `hashKit`, `hashPiece`, `hashConnection`, `HashWriter`, `sha256bytes`, `formatNumberForHash`, `hashPlaneRoot`, `hashPlaneChain`, `hashCenterRoot`, `hashCenterChain`)
    - _Requirements: 1.10, 1.11_

  - [~] 6.7 Delete all copy/paste logic (`copyDesign`, `pasteDesign`, `mergeDesigns`, `orientDesign`)
    - _Requirements: 1.12_

  - [~] 6.8 Delete all design mutation helpers (`deletePiecesAndConnectionsInDesign`, `removePiecesAndConnectionsFromDesign`, `fixPieceInDesign`, `buildDragMoveStructuralContext`)
    - _Requirements: 1.13_

  - [~] 6.9 Delete all geometry 3D math functions (`planeToMatrix`, `matrixToPlane`, `averagePlane`, `toThreeRotation`, `toSemioRotation`, `toThreeQuaternion`, `toSemioQuaternion`, `vectorToThree`)
    - _Requirements: 1.14_

  - [~] 6.10 Delete all validation functions (`validateKitEntityDiff`, `kitEntityDiffIsBlocking`, `validationReportFromGraph`, `graphValidationFromLedgerReport`)
    - _Requirements: 1.16_

  - [~] 6.11 Delete all semantic command expansion functions (`expandSemanticCommandToDiff`, `FlattenDesignCommand`, `DeletePieceCommand`, `ChangePieceTypeCommand`)
    - _Requirements: 1.17_

  - [~] 6.12 Delete all wire projection/conversion functions (`kitWireProjectionFromImpl`, `kitDataFromWireDto`, `emptyKitWireDto`, `kitGraphToPlainData`)
    - _Requirements: 1.18_

  - [~] 6.13 Delete all ledger diff functions (`emptyLedgerDiff`, `normalizeLedgerDiff`, `squashLedgerChangesForward`, `squashLedgerChangesBackward`, `invertLedgerDiff`, `ledgerKitChangeFromGraph`, `graphKitChangeFromLedger`)
    - _Requirements: 1.19_

  - [~] 6.14 Delete all clusterable group functions (`getClusterableGroups`, `getIncludedDesigns`)
    - _Requirements: 1.20_

- [~] 7. Checkpoint - Ensure file compiles after deletions
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. Clean up entity classes (remove domain logic methods, keep OO data methods)
  - [~] 8.1 Strip domain logic methods from entity classes (e.g., `Piece.flatPlane()`, `Piece.flatCenter()`, `Piece.delete()`, `Piece.changeType()`, `Design.deletePieces()`, etc.) while retaining `serialize`, `deserialize`, `toPlain`, `fromPlain`, `createId`, `areSameId`
    - Ensure each entity class follows the pattern defined in the design: constructor, serialize/deserialize, toPlain/fromPlain, createId/areSameId
    - Geometry classes (`Coordinate`, `Vec`, `Point`, `Vector`, `Plane`, `Camera`) retain their `rounded()` method
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

  - [~] 8.2 Create the thin `Kit` class replacing `KitImpl`
    - Implement `Kit` with constructor, `serialize`, `deserialize`, `toPlain`, `fromPlain`, `createId`, `areSameId` — zero domain logic
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 9.3_

- [ ] 9. Create the `Semio` utility class
  - [~] 9.1 Implement the `Semio` class with static methods delegating to WASM
    - `Semio.normalizeName(s)` → `wasmModule.semioNormalizeName(s)`
    - `Semio.round(value, decimals)` → `wasmModule.semioRound(value, decimals)`
    - `Semio.generateId()` → `wasmModule.generateId()`
    - `Semio.kitFromJson(s)` → `wasmModule.kitFromJson(s)`
    - `Semio.kitToJson(value)` → `wasmModule.kitToJson(value)`
    - `Semio.kitValidate(value)` → `wasmModule.kitValidate(value)`
    - `Semio.kitsAreEqual(a, b)` → `wasmModule.kitsAreEqual(a, b)`
    - `Semio.flattenDesign(kit, designId)` → `wasmModule.flattenDesign(kit, designId)`
    - _Requirements: 3.5, 4.8, 4.9, 4.10, 4.11, 4.12, 4.13_

- [ ] 10. Refactor `KitWorkerApi` from plain object to class
  - [~] 10.1 Convert `kitWorkerApi` object to `KitWorkerApi` class
    - Implement as a class with private `handle`, `eventListeners`, `nextEventListenerId`, `eventGqlStarted` fields
    - All methods delegate to `this.handle` (the `KitStoreHandle` WASM instance)
    - _Requirements: 5.5_

- [ ] 11. Simplify `FallbackKitStoreClient` and `WorkerKitStoreClient`
  - [~] 11.1 Simplify `FallbackKitStoreClient` to pure WASM delegation
    - Remove any local validation logic (e.g., `validateRequiredName`, `validateOptionalDisplayName`)
    - Every method is a pure delegation to `this.handle.*` with `settleSetPromise` wrapping
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 5.6_

  - [~] 11.2 Simplify `WorkerKitStoreClient` to pure Comlink delegation
    - Remove local DTO refresh logic that called `KitImpl` methods
    - Every method delegates to `this.api.*` (Comlink proxy) with timeout wrapping
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 5.6_

  - [~] 11.3 Convert `createKitStoreClient` free function to a static method or companion function
    - Replace `asKitInstance(opts.initialKit)` with `JSON.parse(JSON.stringify(opts.initialKit))`
    - _Requirements: 5.4, 3.10_

- [ ] 12. Remove unused imports and dependencies
  - [~] 12.1 Remove imports for deleted dependencies (`@gltf-transform/core`, `three`, `sql.js`, `jszip`, `uuid`, etc.) if no longer used after deletions
    - Clean up the imports region at the top of `index.ts`
    - Remove unused dependencies from `package.json` if applicable
    - _Requirements: 1.1 through 1.20, 9.1, 9.2_

- [~] 13. Checkpoint - Ensure compilation and existing tests pass after refactoring
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 14. Update embedded tests to use WASM bridge
  - [~] 14.1 Rewrite embedded tests that exercised deleted domain logic to call through `FallbackKitStoreClient` or `Semio` utility class
    - Tests for flatten, diff, hash, placement, etc. → rewrite to use WASM bridge or remove with comment
    - _Requirements: 8.2, 8.3_

  - [~] 14.2 Add export surface smoke tests
    - Verify all expected schemas, type aliases, entity classes, wire types, and bridge classes are exported
    - Verify all deleted domain logic functions/classes are NOT exported (denylist check)
    - _Requirements: 1.1 through 1.20, 6.1 through 6.7, 9.1 through 9.6_

  - [~] 14.3 Add entity class method existence tests
    - Verify each entity class has `serialize()`, `deserialize()`, `toPlain()`, `fromPlain()`, `createId()`, `areSameId()` methods (where applicable)
    - Verify geometry classes retain `rounded()` method
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

  - [~] 14.4 Add WASM bridge integration tests
    - Create a client with a minimal kit via `FallbackKitStoreClient`, call `setField`, verify `SetResult.ok === true`
    - Call `getTypes()`, `getDesigns()`, verify they return data from WASM
    - Call `undo()` / `redo()`, verify state changes
    - Call `vcsState()`, verify it returns VCS tree data
    - Call `executeRead()` with a `ReadCommandBatch`, verify typed results
    - _Requirements: 4.1 through 4.7, 5.1, 5.2, 5.3_

- [~] 15. Checkpoint - Ensure all unit tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 16. Add property-based tests
  - [~] 16.1 Write property test for entity toPlain/fromPlain round-trip
    - **Property 1: Entity toPlain/fromPlain round-trip**
    - Generate random valid plain objects for each entity kind using fast-check arbitraries matching the Zod schema
    - Verify `Entity.fromPlain(data).toPlain()` deep-equals the input for all entity kinds
    - Minimum 100 iterations
    - **Validates: Requirements 3.1, 7.2**

  - [~] 16.2 Write property test for entity serialize/deserialize round-trip
    - **Property 2: Entity serialize/deserialize round-trip**
    - Generate random valid plain objects for each entity kind
    - Verify `Entity.deserialize(entity.serialize()).toPlain()` deep-equals the original data
    - Minimum 100 iterations
    - **Validates: Requirements 7.1**

  - [~] 16.3 Write property test for entity ID factory and comparison
    - **Property 3: Entity ID factory and comparison**
    - Generate random string pairs using fast-check
    - Verify `Entity.createId(a)` produces `{ id: a }` and `Entity.areSameId(createId(a), createId(b))` returns `true` iff `a === b`
    - Minimum 100 iterations
    - **Validates: Requirements 3.2, 7.3, 7.4**

- [~] 17. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- All code stays in `semio/js/index.ts` using regions — no separate test files per workspace rules
- The design uses TypeScript throughout, so all implementations use TypeScript
- Property tests use `fast-check` and are embedded in `index.ts`
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation after major phases (deletions, refactoring, testing)
- The `Semio` utility class replaces all free utility functions with WASM-delegating static methods
- Entity classes follow a uniform OO pattern: constructor, serialize/deserialize, toPlain/fromPlain, createId/areSameId
