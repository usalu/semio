import { readFileSync, writeFileSync } from 'fs';

const file = 'semio/js/index.ts';
let content = readFileSync(file, 'utf8');
const startLines = content.split('\n').length;

// Helper: remove a method from a class by matching its JSDoc + signature + body
function removeMethod(methodPattern) {
  content = content.replace(methodPattern, '');
}

// ============================================================
// 1. Remove diff methods from entity classes (diffTo, inverseDiff, mergeDiff, applyDiff)
// ============================================================

// For geometry classes (Coordinate, Vec, Point, Vector, Plane, Camera)
// and entity classes - remove diffTo, inverseDiff, static mergeDiff, applyDiff

// Generic pattern for instance methods with JSDoc
function removeDiffMethods() {
  // Remove diffTo methods (instance)
  content = content.replace(/  \/\*\* [^\n]*delta from this[^\n]*\*\/\n  diffTo\([^)]*\)[^{]*\{[\s\S]*?\n  \}/g, '');
  
  // Remove inverseDiff methods (instance)
  content = content.replace(/  \/\*\* [^\n]*reverse[^\n]*delta[^\n]*\*\/\n  inverseDiff\([^)]*\)[^{]*\{[\s\S]*?\n  \}/g, '');
  
  // Remove static mergeDiff methods
  content = content.replace(/  \/\*\* [^\n]*[Mm]erge two[^\n]*delta[^\n]*\*\/\n  static mergeDiff\([^)]*\)[^{]*\{[\s\S]*?\n  \}/g, '');
  
  // Remove applyDiff methods (instance)
  content = content.replace(/  \/\*\* [^\n]*[Aa]pply[^\n]*delta[^\n]*\*\/\n  applyDiff\([^)]*\)[^{]*\{[\s\S]*?\n  \}/g, '');
}

removeDiffMethods();

// ============================================================
// 2. Remove Plane geometry computation methods (averageWith, average, rounded)
// ============================================================
// averageWith
content = content.replace(/  \/\*\* [^\n]*[Aa]verage this plane[^\n]*\*\/\n  averageWith\([^)]*\)[^{]*\{[\s\S]*?\n  \}/g, '');

// static average
content = content.replace(/  \/\*\* [^\n]*[Aa]verage a plane[^\n]*\*\/\n  static average\([^)]*\)[^{]*\{[\s\S]*?\n  \}/g, '');

// rounded
content = content.replace(/  \/\*\* [^\n]*[Rr]ound plane[^\n]*\*\/\n  rounded\(\)[^{]*\{[\s\S]*?\n  \}/g, '');

// ============================================================
// 3. Remove representation selection logic
// ============================================================
const repSelectionFunctions = [
  'selectBestRepresentation', 'filterRepresentationsByTagIds',
  'getAvailableTagIdsForRepresentations', 'getAllTagIdsFromRepresentations',
  'findRepresentation', 'areSameRepresentation',
  'isSupportedRepresentationExtension', 'validateRepresentationFile'
];

for (const fn of repSelectionFunctions) {
  // Export const pattern
  const pat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const ${fn} = [\\s\\S]*?};`,
    'g'
  );
  content = content.replace(pat, '');
}

// Remove SUPPORTED_3D_EXTENSIONS
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\s*\nexport const SUPPORTED_3D_EXTENSIONS = \[[\s\S]*?\] as const;/g, '');

// Remove Supported3DExtension type
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\s*\nexport type Supported3DExtension = [^;]+;/g, '');

// Remove RepresentationFileValidation interface
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\s*\nexport interface RepresentationFileValidation \{[\s\S]*?\}/g, '');

// ============================================================
// 4. Remove connector compatibility logic
// ============================================================
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\s*\nexport const unifyConnectorPortsAndCompatiblePortsForTypes = [^;]+;\s*\n\s*return \{ updated: \[\] \};\s*\n\};/g, '');
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\s*\nexport const areConnectorsCompatible = [^;]+;\s*\n\s*return true;\s*\n\};/g, '');
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\s*\nexport const arePortsCompatible = [^}]+\};/g, '');
content = content.replace(/\nexport const getKitPorts = [^;]+;/g, '');
content = content.replace(/\nexport const findKitPortFamily = [^\n]+\n[^\n]+/g, '');

// ============================================================
// 5. Remove free finder/helper functions
// ============================================================
const finderFunctions = [
  'findConnector', 'findConnectorInType', 'findPiece', 'findPieceConnections',
  'findConnectorForPieceInConnection', 'findConnection', 'areSameConnection',
  'findTag', 'findConcept', 'isFixedPiece',
  'getPieceRepresentationFileIds', 'getPieceRepresentationUrls',
  'areSameSide'
];

for (const fn of finderFunctions) {
  const pat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const ${fn} = [\\s\\S]*?};`,
    'g'
  );
  content = content.replace(pat, '');
}

// Remove resolvePieceTypeForFlatten (private)
content = content.replace(/\/\*\* Flatten helpers[^\n]*\n[^\n]*\nconst resolvePieceTypeForFlatten = [^;]+;/g, '');

// ============================================================
// 6. Remove design-diff builder functions
// ============================================================
const designDiffBuilders = [
  'addPieceToDesignDiff', 'setPieceInDesignDiff', 'removePieceFromDesignDiff',
  'addPiecesToDesignDiff', 'setPiecesInDesignDiff', 'removePiecesFromDesignDiff',
  'addConnectionToDesignDiff', 'setConnectionInDesignDiff', 'removeConnectionFromDesignDiff',
  'addConnectionsToDesignDiff', 'setConnectionsInDesignDiff', 'removeConnectionsFromDesignDiff',
  'mergeDesigns', 'orientDesign', 'duplicateDesignDiffForIsolation'
];

for (const fn of designDiffBuilders) {
  const pat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const ${fn} = [\\s\\S]*?};`,
    'g'
  );
  content = content.replace(pat, '');
}

// ============================================================
// 7. Remove entity class domain methods
// ============================================================

// Type.findConnector
content = content.replace(/\n  findConnector\(connectorId: string\)[^{]*\{[^}]*\}/g, '');

// Type.toMeta
content = content.replace(/\n  toMeta\(\): TypeMetadataDto \{[^}]*\}/g, '');

// Type.toShallow (multiline)
content = content.replace(/\n  toShallow\(\): TypeShallow \{[\s\S]*?\n  \}/g, '');

// Piece.wireTypeId
content = content.replace(/\n  wireTypeId\(\): TypeId \| undefined \{[^}]*\}/g, '');

// Piece.wireDesignAsPieceId
content = content.replace(/\n  wireDesignAsPieceId\(\): DesignId \| undefined \{[^}]*\}/g, '');

// Piece.toMeta
content = content.replace(/\n  toMeta\(\): PieceMetadataDto \{[^}]*\}/g, '');

// Piece.toShallow
content = content.replace(/\n  toShallow\(\): PieceShallow \{[\s\S]*?\n  \}/g, '');

// Fix Piece.toPlain to not reference wireTypeId/wireDesignAsPieceId
content = content.replace(
  /  toPlain\(\): PiecePlain \{\n    return PieceSchema\.parse\(\{\n      \.\.\.\(this as unknown as PiecePlain\),\n      type: this\.wireTypeId\(\),\n      design: this\.wireDesignAsPieceId\(\),\n    \}\);/g,
  '  toPlain(): PiecePlain {\n    return PieceSchema.parse(this as unknown as PiecePlain);'
);

// Design domain methods
content = content.replace(/\n  findPiece\(lookup[^{]*\{[\s\S]*?\n  \}/g, '');
content = content.replace(/\n  requirePiece\(lookup[^{]*\{[\s\S]*?\n  \}/g, '');
content = content.replace(/\n  findConnection\(connectionId[^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  requireConnection\(connectionId[^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  getPieces\(\)[^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  getConnections\(\)[^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  connections\(\)[^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  toMeta\(\): DesignMetadataDto \{[^}]*\}/g, '');
content = content.replace(/\n  toShallow\(\): DesignShallow \{[\s\S]*?\n  \}/g, '');

// Side domain methods
content = content.replace(/\n  syncPieceFromWire\([^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  syncDesignPieceFromWire\([^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  syncConnectorFromWire\([^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  wirePieceId\(\)[^{]*\{[^}]*\}/g, '');
content = content.replace(/\n  wireDesignPieceId\(\)[^{]*\{[^}]*\}/g, '');

// Connection diff methods (diffTo, inverseDiff, applyDiff)
// These are larger methods - use more specific patterns
content = content.replace(/  \/\*\* [^\n]*connection delta from this[^\n]*\*\/\n  diffTo\(after: Connection\)[^{]*\{[\s\S]*?\n  \}/g, '');
content = content.replace(/  \/\*\* [^\n]*reverse connection delta[^\n]*\*\/\n  inverseDiff\(appliedDiff: ConnectionDiff\)[^{]*\{[\s\S]*?\n  \}/g, '');
content = content.replace(/  \/\*\* [^\n]*[Aa]pply a connection delta[^\n]*\*\/\n  applyDiff\(diff: ConnectionDiff\)[^{]*\{[\s\S]*?\n  \}/g, '');

// Side diff methods
content = content.replace(/  \/\*\* [^\n]*side endpoint delta[^\n]*\*\/\n  diffTo\(after: Side\)[^{]*\{[\s\S]*?\n  \}/g, '');
content = content.replace(/  \/\*\* [^\n]*reverse side delta[^\n]*\*\/\n  inverseDiff\(appliedDiff: SideDiff\)[^{]*\{[\s\S]*?\n  \}/g, '');
content = content.replace(/  \/\*\* [^\n]*[Aa]pply a side delta[^\n]*\*\/\n  applyDiff\(diff: SideDiff\)[^{]*\{[\s\S]*?\n  \}/g, '');

// ============================================================
// 8. Remove legacy classes and functions
// ============================================================

// Remove KitInProcessFacade type and createKitInProcessFacade function
content = content.replace(/\/\*\* [^\n]*Legacy in-process[^\n]*\*\/\nexport type KitInProcessFacade = \{[\s\S]*?\};/g, '');
content = content.replace(/\nfunction createKitInProcessFacade\([^)]*\): KitInProcessFacade \{[\s\S]*?\n\}/g, '');

// Remove Kit.ensure static method
content = content.replace(/\n  \/\*\* [^\n]*Algorithms[^\n]*\*\/\n  static ensure\([^)]*\)[^{]*\{[\s\S]*?\n  \}/g, '');

// Remove KitLike type
// Keep it - it's used by createKitStoreClient

// Remove asKitInstance
content = content.replace(/\/\*\* [^\n]*Normalizes plain[^\n]*\*\/\nexport function asKitInstance\([^)]*\)[^{]*\{[\s\S]*?\n\}/g, '');

// Remove InMemoryKitStore
content = content.replace(/\/\*\* [^\n]*Small synchronous[^\n]*\*\/\nexport class InMemoryKitStore implements KitStore \{[\s\S]*?\n\}/g, '');

// Remove KitStore interface
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\nexport interface KitStore \{[\s\S]*?\n\}/g, '');

// Remove UndoableKitStore interface
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\nexport interface UndoableKitStore extends KitStore \{[\s\S]*?\n\}/g, '');

// Remove BlobAssetStore interface
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\nexport interface BlobAssetStore \{[\s\S]*?\n\}/g, '');

// Remove ObservablePathStore interface
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\nexport interface ObservablePathStore \{[\s\S]*?\n\}/g, '');

// Remove KitStoreSnapshot type
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\nexport type KitStoreSnapshot = \{[\s\S]*?\};/g, '');

// Remove KitSyncState type
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\nexport type KitSyncState = \{[\s\S]*?\};/g, '');

// Remove KitStoreStatus type
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\nexport type KitStoreStatus = [^;]+;/g, '');

// Remove PASTE_DESIGN_ANCHORING_KINDS
content = content.replace(/\nexport const PASTE_DESIGN_ANCHORING_KINDS = [^;]+;\s*\nexport type PasteDesignAnchoringKind = [^;]+;/g, '');

// ============================================================
// 9. Clean up empty lines (collapse multiple blank lines to max 2)
// ============================================================
content = content.replace(/\n{4,}/g, '\n\n\n');

const finalLines = content.split('\n').length;
console.log(`Phase 2: ${startLines} -> ${finalLines} lines`);

writeFileSync(file, content, 'utf8');
console.log('Phase 2 complete');
