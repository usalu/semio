# Requirements Document

## Introduction

Refactor `compose/js` (`index.ts`, ~31k lines) from a monolithic TypeScript library containing domain logic, caching, and free functions into a pure object-oriented thin client that exclusively delegates to `compose/rs` via WASM. All domain logic (flatten geometry, piece placement, representation selection, diff computation, hashing, etc.) and all caching (flatten merkle cache, SQL.js cache) must be removed from `compose/js` and either already exist or be migrated to `compose/rs`. Every exported free function must become a method on an appropriate class. This is a greenfield repo with no backwards compatibility requirements.

## Glossary

- **Thin_Client**: The refactored `compose/js` library that contains zero domain logic and zero caching, delegating all computation to `compose/rs` via WASM.
- **WASM_Bridge**: The Worker + Comlink communication layer between `compose/js` and the `KitStoreHandle` class exposed by `@semio-tech/compose-rs-wasm`.
- **KitStoreHandle**: The Rust-side WASM class (`compose/rs/pkg`) that owns the live `KitGraph`, exposes GraphQL, read commands, change commands, VCS state, and domain operations.
- **KitStoreClient**: The TypeScript interface in `compose/js` that defines the boundary contract consumed by `@semio-tech/compose-react` and the sketchpad.
- **Domain_Logic**: Any computation that transforms, resolves, or derives data from kit entities — including flatten geometry, piece placement, child plane computation, representation selection (Jaccard scoring), quality aggregation, type/design/piece resolution, connector compatibility, diff computation, and merkle hashing.
- **Free_Function**: Any exported `const fn = () => ...` or `export function fn()` that is not a method on a class or a static method on a class.
- **Entity_Class**: An OO class representing a kit domain entity (e.g., `Coordinate`, `Plane`, `Type`, `Design`, `Piece`, `Connection`, `Kit`).
- **Diff_Function**: A free function that computes, applies, inverts, or merges diffs between entity instances (e.g., `getTypeDiff`, `applyDesignDiff`, `inverseKitDiff`, `mergeKitDiff`).
- **Hash_Function**: A free function that computes content-addressable hashes for entities or diffs (e.g., `hashType`, `hashDesignDiff`, `hashKit`).
- **Serialization_Function**: A free function that serializes or deserializes an entity to/from JSON (e.g., `serializeKit`, `deserializeKit`, `serializeType`).
- **Id_Factory_Function**: A free function that creates or compares entity ID wrappers (e.g., `createTypeId`, `areSameTypeId`, `getTypeId`).
- **Flatten_Cache**: The `#flattenMerkleByDesign` Map inside `KitImpl` that caches per-piece plane/center results to avoid recomputation.
- **FallbackKitStoreClient**: The in-process JSON mirror of `KitStoreHandle` used for Node/test environments when Worker is unavailable.
- **WorkerKitStoreClient**: The Comlink-backed client that communicates with `KitStoreHandle` running in a Web Worker.
- **InMemoryKitStore**: A lightweight in-memory kit store implementation for testing that does not support backbone/VCS operations.

## Requirements

### Requirement 1: Remove All Domain Logic from compose/js

**User Story:** As a developer, I want compose/js to contain zero domain logic, so that all computation is centralized in compose/rs and compose/js is a pure delegation layer.

#### Acceptance Criteria

1. THE Thin_Client SHALL NOT contain any flatten geometry computation (flatten placement walk, `computeChildPlane`, `connectionPlacementTranslationBasis`, `flattenPlacementWalkDesignOrderRoots`, `buildFlattenPieceAdjacency`, `collectUndirectedComponentIds`, or equivalent logic).
2. THE Thin_Client SHALL NOT contain any piece placement calculation (drag/move translation, structural move context, `moveTranslationWorldFromPiecePlane`, `childConnectorOriginWorld`, `solveConnectionOriginMinNorm`, `connectionDiffFromStructuralMoveVector`, or equivalent logic).
3. THE Thin_Client SHALL NOT contain any representation selection logic (Jaccard scoring via `jaccard`, `selectBestRepresentation`, `filterRepresentationsByTagIds`, `getAvailableTagIdsForRepresentations`, `getAllTagIdsFromRepresentations`, or equivalent logic).
4. THE Thin_Client SHALL NOT contain any type/design/piece resolution logic (`resolvePieceTypeForFlatten`, `findTypeInKit`, `findDesignInKit`, `findPieceInDesign`, `findConnectionInDesign`, or equivalent logic that traverses kit entity graphs).
5. THE Thin_Client SHALL NOT contain any connector compatibility logic (`arePortsCompatible`, `areConnectorsCompatible`, `unifyConnectorPortsAndCompatiblePortsForTypes`, or equivalent logic).
6. THE Thin_Client SHALL NOT contain any diff computation logic (`getTypeDiff`, `getDesignDiff`, `getKitDiff`, `computeKitGraphDiffBetween`, or equivalent diff-between functions).
7. THE Thin_Client SHALL NOT contain any diff application logic (`applyTypeDiff`, `applyDesignDiff`, `applyDesignDiffCore`, `applyKitDiff`, `applyLedgerDiffToKitEntity`, or equivalent apply functions).
8. THE Thin_Client SHALL NOT contain any diff inversion logic (`inverseTypeDiff`, `inverseDesignDiff`, `inverseKitDiff`, `inverseKitGraphDiff`, or equivalent inverse functions).
9. THE Thin_Client SHALL NOT contain any diff merge logic (`mergeTypeDiff`, `mergeDesignDiff`, `mergeKitDiff`, `mergeKitGraphDiff`, `composeLedgerDiffs`, or equivalent merge functions).
10. THE Thin_Client SHALL NOT contain any content-addressable hashing logic (`hashType`, `hashDesign`, `hashKit`, `hashPiece`, `hashConnection`, `HashWriter`, `sha256bytes`, `formatNumberForHash`, or equivalent hash functions).
11. THE Thin_Client SHALL NOT contain any flatten merkle hash computation (`hashPlaneRoot`, `hashPlaneChain`, `hashCenterRoot`, `hashCenterChain`, or equivalent merkle hash functions).
12. THE Thin_Client SHALL NOT contain any design copy/paste logic (`copyDesign`, `pasteDesign`, `mergeDesigns`, `orientDesign`, or equivalent functions).
13. THE Thin_Client SHALL NOT contain any design mutation helpers (`deletePiecesAndConnectionsInDesign`, `removePiecesAndConnectionsFromDesign`, `fixPieceInDesign`, `buildDragMoveStructuralContext`, or equivalent functions).
14. THE Thin_Client SHALL NOT contain any geometry conversion functions that perform 3D math (`planeToMatrix`, `matrixToPlane`, `averagePlane`, `toThreeRotation`, `toComposeRotation`, `toThreeQuaternion`, `toComposeQuaternion`, `vectorToThree`, or equivalent functions).
15. THE Thin_Client SHALL NOT contain any kit graph transaction or history management logic (`KitTransactionsCoordinator`, `KitActiveTransactionSurface`, `Transaction`, `DiffComposer`, `recomputeTxNet`, or equivalent classes/functions).
16. THE Thin_Client SHALL NOT contain any validation logic (`validateKitEntityDiff`, `kitEntityDiffIsBlocking`, `validationReportFromGraph`, `graphValidationFromLedgerReport`, or equivalent functions).
17. THE Thin_Client SHALL NOT contain any semantic command expansion logic (`expandSemanticCommandToDiff`, `FlattenDesignCommand`, `DeletePieceCommand`, `ChangePieceTypeCommand`, or equivalent classes/functions).
18. THE Thin_Client SHALL NOT contain any kit wire projection or conversion logic (`kitWireProjectionFromImpl`, `kitDataFromWireDto`, `emptyKitWireDto`, `kitGraphToPlainData`, or equivalent functions).
19. THE Thin_Client SHALL NOT contain any ledger diff operations (`emptyLedgerDiff`, `normalizeLedgerDiff`, `squashLedgerChangesForward`, `squashLedgerChangesBackward`, `invertLedgerDiff`, `ledgerKitChangeFromGraph`, `graphKitChangeFromLedger`, or equivalent functions).
20. THE Thin_Client SHALL NOT contain any clusterable group computation (`getClusterableGroups`, `getIncludedDesigns`, or equivalent functions).

### Requirement 2: Remove All Caching from compose/js

**User Story:** As a developer, I want compose/js to contain zero caching, so that cache management is centralized in compose/rs and compose/js has no stale-data risks.

#### Acceptance Criteria

1. THE Thin_Client SHALL NOT contain the `#flattenMerkleByDesign` Map or any equivalent in-memory flatten geometry cache.
2. THE Thin_Client SHALL NOT contain any `FlatMerkleCacheEntry` storage, `ensureFlattenGeometryCache`, `getFlattenMerkleCache`, `invalidateFlattenMerkleCaches`, or equivalent cache management methods.
3. THE Thin_Client SHALL NOT contain any `piecesMetadataCached` or equivalent methods that accept or return cache objects.
4. THE Thin_Client SHALL NOT contain any SQL.js cache (`cachedSqlJs` or equivalent).
5. THE Thin_Client SHALL NOT contain any undo/redo history stacks (`#historyDone`, `#historyUndone`, or equivalent) — undo/redo state is owned by `KitStoreHandle` in compose/rs.

### Requirement 3: Eliminate All Free Functions

**User Story:** As a developer, I want every exported function in compose/js to be a method on a class, so that the API is fully object-oriented and discoverable.

#### Acceptance Criteria

1. THE Thin_Client SHALL NOT export any free Serialization_Function (e.g., `serializeKit`, `deserializeKit`, `serializeType`, `deserializeType`, `serializeDesign`, `deserializeDesign`, and all per-entity `serialize*`/`deserialize*` variants). WHEN serialization is needed, THE Entity_Class SHALL provide it as an instance method or static method.
2. THE Thin_Client SHALL NOT export any free Id_Factory_Function (e.g., `createTypeId`, `areSameTypeId`, `getTypeId`, and all per-entity `create*Id`/`areSame*Id`/`get*Id` variants). WHEN ID creation or comparison is needed, THE Entity_Class SHALL provide it as a static method.
3. THE Thin_Client SHALL NOT export any free Diff_Function. WHEN diff operations are needed, THE Entity_Class SHALL provide them as instance or static methods, or THE Thin_Client SHALL delegate to compose/rs.
4. THE Thin_Client SHALL NOT export any free Hash_Function. WHEN hashing is needed, THE Entity_Class SHALL provide it as an instance method, or THE Thin_Client SHALL delegate to compose/rs.
5. THE Thin_Client SHALL NOT export any free utility functions (`normalize`, `round`, `deepEqual`, `arraysEqual`, `generateUniqueName`, `cn`, `id`). WHEN utility behavior is needed, THE Thin_Client SHALL provide it as a static method on an appropriate class.
6. THE Thin_Client SHALL NOT export any free geometry helper functions (`roundPlane`, `serializePlane`, `deserializePlane`, and all per-geometry `serialize*`/`deserialize*`/`get*Diff`/`apply*Diff`/`inverse*Diff`/`merge*Diff` variants). WHEN geometry operations are needed, THE Entity_Class SHALL provide them as instance or static methods.
7. THE Thin_Client SHALL NOT export any free design-diff builder functions (`addPieceToDesignDiff`, `removePieceFromDesignDiff`, `addConnectionToDesignDiff`, `removeConnectionFromDesignDiff`, `setPieceInDesignDiff`, and all variants). WHEN design diff building is needed, THE Thin_Client SHALL delegate to compose/rs.
8. THE Thin_Client SHALL NOT export any free collection-diff functions (`getCollectionDiff`, `inverseCollectionDiff`, `applyCollectionDiff`, `mergeCollectionDiff`). WHEN collection diff operations are needed, THE Thin_Client SHALL delegate to compose/rs.
9. THE Thin_Client SHALL NOT export any free backbone factory functions (`createLocalBackbone`, `createDevBackbone`, `createRemoteBackbone`). WHEN backbone creation is needed, THE Thin_Client SHALL provide it as a static method on a `Backbone` class or delegate to compose/rs.
10. THE Thin_Client SHALL NOT export any free kit conversion functions (`asKitInstance`, `requireKit`, `duplicateKitForIsolation`, `stripNullsJsonClone`, `detachPieceForLocalMutation`, `detachConnectionForLocalMutation`, `detachDesignForLocalMutation`, `designWithDiff`). WHEN conversion is needed, THE Entity_Class SHALL provide it as a method.

### Requirement 4: compose/js Exclusively Uses compose/rs for All Operations

**User Story:** As a developer, I want compose/js to delegate every operation to compose/rs via WASM, so that there is a single source of truth for all kit logic.

#### Acceptance Criteria

1. WHEN the KitStoreClient performs a flatten operation, THE Thin_Client SHALL delegate to `KitStoreHandle.flattenDesign` in compose/rs and SHALL NOT perform any local flatten computation.
2. WHEN the KitStoreClient retrieves piece metadata (planes, centers), THE Thin_Client SHALL delegate to `KitStoreHandle.getPiecesMetadata` in compose/rs and SHALL NOT compute placement locally.
3. WHEN the KitStoreClient performs undo or redo, THE Thin_Client SHALL delegate to `KitStoreHandle.undo` / `KitStoreHandle.redo` in compose/rs and SHALL NOT maintain local history stacks.
4. WHEN the KitStoreClient performs a drag, move, cluster, fix, expand, or paste operation, THE Thin_Client SHALL delegate to the corresponding `KitStoreHandle` method in compose/rs.
5. WHEN the KitStoreClient creates a connected piece, fixed piece, or hanging pieces, THE Thin_Client SHALL delegate to the corresponding `KitStoreHandle` method in compose/rs.
6. WHEN the KitStoreClient reads kit state (types, designs, pieces, connections, authors, metadata), THE Thin_Client SHALL delegate to the corresponding `KitStoreHandle` getter or `executeRead` in compose/rs.
7. WHEN the KitStoreClient performs VCS operations (vcsState, materializeAt, attachBackbone, detachBackbone, syncNow, listConflicts, resolveConflict), THE Thin_Client SHALL delegate to `KitStoreHandle.execute` in compose/rs.
8. WHEN the KitStoreClient validates a kit or computes a diff, THE Thin_Client SHALL delegate to `kitValidate` or equivalent compose/rs WASM functions.
9. WHEN the KitStoreClient needs to serialize or deserialize a kit, THE Thin_Client SHALL delegate to `kitToJson` / `kitFromJson` in compose/rs.
10. WHEN the KitStoreClient needs to compare kits for equality, THE Thin_Client SHALL delegate to `kitsAreEqual` in compose/rs.
11. WHEN the KitStoreClient needs to normalize a name, THE Thin_Client SHALL delegate to `composeNormalizeName` in compose/rs.
12. WHEN the KitStoreClient needs to generate an ID, THE Thin_Client SHALL delegate to `generateId` in compose/rs.
13. WHEN the KitStoreClient needs to round a numeric value, THE Thin_Client SHALL delegate to `composeRound` in compose/rs.

### Requirement 5: Preserve the WASM Bridge Architecture

**User Story:** As a developer, I want the Worker + Comlink communication layer to remain functional, so that WASM operations run off the main thread.

#### Acceptance Criteria

1. THE Thin_Client SHALL retain the `WorkerKitStoreClient` class that communicates with `KitStoreHandle` via a Web Worker and Comlink.
2. THE Thin_Client SHALL retain the `FallbackKitStoreClient` class that communicates with `KitStoreHandle` in-process for Node/test environments.
3. THE Thin_Client SHALL retain the `KitStoreClient` interface as the boundary contract consumed by `@semio-tech/compose-react` and the sketchpad.
4. THE Thin_Client SHALL retain the `createKitStoreClient` factory function (as a static method on an appropriate class) that selects between Worker and fallback modes.
5. THE Thin_Client SHALL retain the `kitWorkerApi` object (as a class) that hosts the WASM module in the Worker thread.
6. WHEN a `KitStoreClient` method is called, THE Thin_Client SHALL forward the call to `KitStoreHandle` via the WASM_Bridge without performing any local computation or transformation beyond JSON serialization.

### Requirement 6: Retain Pure Data Kind Definitions and Zod Schemas

**User Story:** As a developer, I want compose/js to still export the TypeScript kind definitions and Zod schemas for kit entities, so that consumers can validate and kind-check data received from compose/rs.

#### Acceptance Criteria

1. THE Thin_Client SHALL export Zod schemas for all kit entities (Kit, Type, Design, Piece, Connection, Port, Connector, Representation, Family, Quality, File, Folder, Location, Author, Attribute, Benchmark, Prop, Tag, Concept, Stat, Layer, Group, Side, Coordinate, Vec, Point, Vector, Plane, Camera).
2. THE Thin_Client SHALL export TypeScript kind aliases inferred from the Zod schemas for all kit entities.
3. THE Thin_Client SHALL export Zod schemas for Meta and Shallow projections of entities that have them.
4. THE Thin_Client SHALL export Zod schemas for Diff kinds of all entities.
5. THE Thin_Client SHALL export the `DiffStatus` enum and its Zod schema.
6. THE Thin_Client SHALL export wire kind definitions for kit store communication (`KitStoreWireBackboneConfig`, `KitStoreWireConflictResolution`, `KitStoreWireBackboneStatus`, `KitStoreWireKitConflict`, `KitStoreExecuteResult`, `SetResult`, `WriteStatus`).
7. THE Thin_Client SHALL export the `ReadCommandBatch` and `ReadCommandBatchResult` kinds for typed read command batching.

### Requirement 7: Entity Classes Are Object-Oriented with Methods

**User Story:** As a developer, I want each entity class to encapsulate its own serialization, comparison, and factory logic as methods, so that the API is cohesive and discoverable.

#### Acceptance Criteria

1. WHEN an Entity_Class needs serialization, THE Entity_Class SHALL provide a `serialize()` instance method and a static `deserialize(json: string)` method.
2. WHEN an Entity_Class needs JSON conversion, THE Entity_Class SHALL provide a `toPlain()` instance method that returns a plain object and a static `fromPlain(data)` factory method.
3. WHEN an Entity_Class needs ID creation, THE Entity_Class SHALL provide a static `createId(id: string)` method.
4. WHEN an Entity_Class needs ID comparison, THE Entity_Class SHALL provide a static `areSameId(a, b)` method.
5. THE Entity_Class `Coordinate`, `Vec`, `Point`, `Vector`, `Plane`, and `Camera` SHALL retain their existing instance methods (`serialize`, `deserialize`, `rounded`, `toPlain`) and SHALL NOT have free-function wrappers exported alongside them.

### Requirement 8: Embedded Tests Remain Functional

**User Story:** As a developer, I want the embedded test suite to continue working after the refactor, so that correctness is verified without separate test files.

#### Acceptance Criteria

1. THE Thin_Client SHALL retain the embedded test mechanism triggered by `COMPOSE_JS_RUN_EMBEDDED_TESTS=1`.
2. WHEN embedded tests exercise domain logic that has moved to compose/rs, THE Thin_Client SHALL update those tests to call through the WASM_Bridge instead of local JS functions.
3. IF an embedded test cannot be meaningfully executed through the WASM_Bridge, THEN THE Thin_Client SHALL remove that test and document the removal reason in a comment.

### Requirement 9: No Backwards Compatibility or Legacy Support

**User Story:** As a developer, I want the refactor to be a clean break with no legacy API surface, so that the codebase is free of dead code and compatibility shims.

#### Acceptance Criteria

1. THE Thin_Client SHALL NOT retain any deprecated function, class, or export that exists solely for backwards compatibility.
2. THE Thin_Client SHALL NOT retain any `@deprecated` annotations or compatibility aliases.
3. THE Thin_Client SHALL NOT retain the `KitImpl` class with its embedded domain logic. WHEN `KitImpl` functionality is needed, THE Thin_Client SHALL delegate to `KitStoreHandle` in compose/rs.
4. THE Thin_Client SHALL NOT retain the `InMemoryKitStore` class. WHEN in-memory kit store behavior is needed for tests, THE Thin_Client SHALL use `FallbackKitStoreClient` backed by `KitStoreHandle`.
5. THE Thin_Client SHALL NOT retain the `KitEntity`, `KitEntityIndexes`, `KitEntityCaches`, `KitInteractionsApi`, `KitInteractionEntity`, `KitEntityType`, `KitEntityPiece`, `KitEntityDesign`, `KitDocument`, or `KitBackboneBridge` classes — these contain domain logic that belongs in compose/rs.
6. THE Thin_Client SHALL NOT retain the `KitTypesOps`, `KitDesignsOps`, `KitFamiliesOps`, `KitFilesOps`, `KitTagsOps`, `KitConceptsOps`, `KitAttributesOps`, or `KitOps` classes — these contain domain logic that belongs in compose/rs.
