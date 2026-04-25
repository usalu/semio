$filePath = "semio/js/index.ts"
$lines = [System.IO.File]::ReadAllLines($filePath)
$total = $lines.Count
Write-Host "Total lines: $total"

# Build a set of line indices to DELETE (0-based)
$del = [System.Collections.Generic.HashSet[int]]::new()

function Mark([int]$from, [int]$to) {
    for ($i = $from; $i -le $to; $i++) { [void]$del.Add($i) }
    Write-Host "  DELETE lines $($from+1)..$($to+1) ($($to-$from+1) lines)"
}

function FindIdx([string]$pat, [int]$after = 0) {
    for ($i = $after; $i -lt $lines.Count; $i++) {
        if ($lines[$i] -match $pat) { return $i }
    }
    return -1
}

function FindClose([int]$start) {
    $d = 0; $s = $false
    for ($i = $start; $i -lt $lines.Count; $i++) {
        foreach ($c in $lines[$i].ToCharArray()) {
            if ($c -eq '{') { $d++; $s = $true }
            if ($c -eq '}') { $d-- }
            if ($s -and $d -eq 0) { return $i }
        }
    }
    return -1
}

# Find the start of a JSDoc/comment block before a given line
function FindDocStart([int]$idx) {
    $s = $idx
    while ($s -gt 0) {
        $prev = $lines[$s - 1].Trim()
        if ($prev -eq '' -or $prev.StartsWith('*') -or $prev.StartsWith('/**') -or $prev.StartsWith('//')) {
            $s--
            if ($prev.StartsWith('/**')) { break }
        } else { break }
    }
    # If we only found blank lines, don't include them
    if ($s -lt $idx -and -not ($lines[$s].Trim().StartsWith('/**') -or $lines[$s].Trim().StartsWith('//'))) {
        return $idx
    }
    return $s
}

# Delete a single exported const/function by name
function DelConst([string]$name, [int]$after = 0) {
    $pat = "^export (const|function) $name\b"
    $idx = FindIdx $pat $after
    if ($idx -lt 0) { return }
    $start = FindDocStart $idx
    $end = FindClose $idx
    if ($end -lt 0) {
        # single-line or semicolon-terminated
        $end = $idx
        while ($end -lt $lines.Count -and -not $lines[$end].TrimEnd().EndsWith(';')) { $end++ }
    }
    # Check for trailing semicolon on next line
    if (($end + 1) -lt $lines.Count -and $lines[$end + 1].Trim() -eq ';') { $end++ }
    Mark $start $end
}

# Delete a class by name
function DelClass([string]$name, [int]$after = 0) {
    $pat = "^(export )?class $name\b"
    $idx = FindIdx $pat $after
    if ($idx -lt 0) { Write-Host "  NOT FOUND: class $name"; return }
    $start = FindDocStart $idx
    $end = FindClose $idx
    if ($end -lt 0) { Write-Host "  WARN: no close for $name"; return }
    Mark $start $end
}


# ============================================================
# PHASE 1: Large region deletions
# ============================================================

# --- TASK 3.1: Delete KitImpl class (lines ~8479 to ~11575) ---
Write-Host "`n=== 3.1 KitImpl class ==="
DelClass "KitImpl"

# --- TASK 3.2: Delete KitEntity and related classes ---
Write-Host "`n=== 3.2 KitEntity classes ==="
# These are in the KitEntity region (lines ~11876 to ~12586)
# KitBackboneBridge is at line 12085
DelClass "KitBackboneBridge"

# KitEntity and sub-classes - they're all in the KitEntity region
# Let's find and delete each one
$keClasses = @("KitEntityIndexes","KitEntityCaches","KitInteractionsApi","KitInteractionEntity","KitEntityType","KitEntityPiece","KitEntityDesign","KitDocument","KitEntity")
foreach ($c in $keClasses) {
    Write-Host "Deleting $c..."
    DelClass $c
}

# --- TASK 3.3: Delete KitOps classes ---
Write-Host "`n=== 3.3 KitOps classes ==="
$opsClasses = @("KitTypesOps","KitDesignsOps","KitFamiliesOps","KitFilesOps","KitTagsOps","KitConceptsOps","KitAttributesOps","KitOps")
foreach ($c in $opsClasses) {
    Write-Host "Deleting $c..."
    DelClass $c
}

# --- TASK 3.4: Delete transaction classes ---
Write-Host "`n=== 3.4 Transaction classes ==="
DelClass "KitTransactionsCoordinator"
DelClass "KitActiveTransactionSurface"
DelClass "Transaction"

# DiffComposer - might be a class or function
$dcIdx = FindIdx "class DiffComposer\b|const DiffComposer\b"
if ($dcIdx -ge 0) {
    $s = FindDocStart $dcIdx
    $e = FindClose $dcIdx
    if ($e -ge 0) { Mark $s $e }
}

# recomputeTxNet
DelConst "recomputeTxNet"

# --- TASK 3.5: Delete InMemoryKitStore and legacy KitStore ---
Write-Host "`n=== 3.5 InMemoryKitStore ==="
# InMemoryKitStore region: lines 21169-21389
$imIdx = FindIdx "// #region.*InMemoryKitStore"
if ($imIdx -ge 0) {
    $imEnd = FindIdx "// #endregion.*InMemoryKitStore" $imIdx
    if ($imEnd -ge 0) { Mark $imIdx $imEnd }
}

# --- TASK 6.11: Delete semantic command classes ---
Write-Host "`n=== 6.11 Semantic command classes ==="
DelClass "FlattenDesignCommand"
DelClass "DeletePieceCommand"
DelClass "ChangePieceTypeCommand"
# expandSemanticCommandToDiff
DelConst "expandSemanticCommandToDiff"

# --- TASK 3.2 continued: Delete Backbone classes ---
Write-Host "`n=== Backbone classes ==="
DelClass "DevBackbone"
DelClass "LocalBackbone"
DelClass "RemoteBackbone"


# ============================================================
# PHASE 2: Delete entire regions that are all domain logic
# ============================================================

Write-Host "`n=== TASK 6.6: Delete Hash region (lines 13026-14712) ==="
$hashStart = FindIdx "// #region.*Hash$" 13000
if ($hashStart -ge 0) {
    # Find the matching endregion
    $hashEnd = FindIdx "// #endregion.*Hash$" $hashStart
    if ($hashEnd -ge 0) { Mark $hashStart $hashEnd }
}

Write-Host "`n=== TASK 6.10: Delete Validation region (lines 15467-16279) ==="
# Validation region
$valStart = FindIdx "// #region.*Validation$" 15460
if ($valStart -ge 0) {
    $valEnd = FindIdx "// #endregion.*Validation$" $valStart
    if ($valEnd -ge 0) { Mark $valStart $valEnd }
}

Write-Host "`n=== Delete KitImpl Diff Validation (lines 15102-15465) ==="
$kdvStart = FindIdx "// #region.*KitImpl Diff Validation" 15090
if ($kdvStart -ge 0) {
    $kdvEnd = FindIdx "// #endregion.*KitImpl Diff Validation" $kdvStart
    if ($kdvEnd -ge 0) { Mark $kdvStart $kdvEnd }
}

Write-Host "`n=== Delete SemioReport region (lines 14968-15086) ==="
$srStart = FindIdx "// #region.*SemioReport" 14960
if ($srStart -ge 0) {
    $srEnd = FindIdx "// #endregion.*SemioReport" $srStart
    if ($srEnd -ge 0) { Mark $srStart $srEnd }
}

Write-Host "`n=== Delete Design Family Helpers (lines 14928-14951) ==="
$dfhStart = FindIdx "// #region.*Design Family Helpers" 14920
if ($dfhStart -ge 0) {
    $dfhEnd = FindIdx "// #endregion.*Design Family Helpers" $dfhStart
    if ($dfhEnd -ge 0) { Mark $dfhStart $dfhEnd }
}

Write-Host "`n=== Delete Type Family Helpers (lines 14953-14966) ==="
$tfhStart = FindIdx "// #region.*Type Family Helpers" 14950
if ($tfhStart -ge 0) {
    $tfhEnd = FindIdx "// #endregion.*Type Family Helpers" $tfhStart
    if ($tfhEnd -ge 0) { Mark $tfhStart $tfhEnd }
}

Write-Host "`n=== Delete KitImpl Import/Export (lines 16312-18965) ==="
$kieStart = FindIdx "// #region.*KitImpl Import/Export" 16300
if ($kieStart -ge 0) {
    $kieEnd = FindIdx "// #endregion.*KitImpl Import/Export" $kieStart
    if ($kieEnd -ge 0) { Mark $kieStart $kieEnd }
}

Write-Host "`n=== Delete KitImpl Representation Export (lines 18967-19428) ==="
$kreStart = FindIdx "// #region.*KitImpl Representation Export" 18960
if ($kreStart -ge 0) {
    $kreEnd = FindIdx "// #endregion.*KitImpl Representation Export" $kreStart
    if ($kreEnd -ge 0) { Mark $kreStart $kreEnd }
}

Write-Host "`n=== Delete Geometric Insights (lines 19430-19631) ==="
$giStart = FindIdx "// #region.*Geometric Insights" 19420
if ($giStart -ge 0) {
    $giEnd = FindIdx "// #endregion.*Geometric Insights" $giStart
    if ($giEnd -ge 0) { Mark $giStart $giEnd }
}

Write-Host "`n=== Delete Benchmarks region (lines 29249-29414) ==="
$bmStart = FindIdx "// #region.*Benchmarks" 29240
if ($bmStart -ge 0) {
    $bmEnd = FindIdx "// #endregion.*Benchmarks" $bmStart
    if ($bmEnd -ge 0) { Mark $bmStart $bmEnd }
}


# ============================================================
# PHASE 3: Delete free functions (Tasks 5.1-5.8, 6.1-6.14)
# ============================================================

Write-Host "`n=== TASK 5.3: Delete free utility functions ==="
DelConst "cn"
# id - special case, single line
$idIdx = FindIdx "^export const id = " 0
if ($idIdx -ge 0) {
    $s = FindDocStart $idIdx
    Mark $s $idIdx
}
DelConst "normalize"
# round - the free function
$roundIdx = FindIdx "^export const round = " 0
if ($roundIdx -ge 0) {
    $s = FindDocStart $roundIdx
    Mark $s $roundIdx
}
DelConst "jaccard"
DelConst "deepEqual"
DelConst "arraysEqual"
DelConst "generateUniqueName"

Write-Host "`n=== TASK 6.9: Delete geometry 3D math ==="
DelConst "toThreeRotation"
DelConst "toSemioRotation"
DelConst "toThreeQuaternion"
DelConst "toSemioQuaternion"
DelConst "vectorToThree"
DelConst "planeToMatrix"
DelConst "matrixToPlane"
DelConst "averagePlane"

Write-Host "`n=== TASK 5.2: Delete free ID factory functions ==="
$entities = @("Attribute","Location","Author","File","Folder","Benchmark","Quality","Port","Prop","Representation","Connector","Type","Layer","Piece","Group","Connection","Stat","Design","Kit","Tag","Concept","Family")
foreach ($e in $entities) {
    DelConst "create${e}Id"
    DelConst "areSame${e}Id"
    DelConst "get${e}Id"
}

Write-Host "`n=== TASK 5.1: Delete free serialization functions ==="
$serEntities = @("Coordinate","Vec","Point","Vector","Plane","Camera","Attribute","Location","Author","File","Folder","Benchmark","Quality","Port","Family","Prop","Tag","Concept","Representation","Connector","Type","Layer","Piece","Group","Side","SideId","Connection","Stat","Design","Kit")
foreach ($e in $serEntities) {
    DelConst "serialize${e}"
    DelConst "deserialize${e}"
}
# Meta variants
$metaEntities = @("Author","File","Folder","Quality","Port","Family","Prop","Tag","Concept","Representation","Connector","Type","Layer","Piece","Group","Connection","Stat","Design","Attribute","Benchmark")
foreach ($e in $metaEntities) {
    DelConst "serialize${e}Meta"
    DelConst "deserialize${e}Meta"
}
# Shallow variants
$shallowEntities = @("Piece","Type","Connector","Design")
foreach ($e in $shallowEntities) {
    DelConst "serialize${e}Shallow"
    DelConst "deserialize${e}Shallow"
}

Write-Host "`n=== TASK 5.4: Delete free geometry helper functions ==="
DelConst "roundPlane"
DelConst "serializePlane"
DelConst "deserializePlane"
# Per-geometry diff/serialize/deserialize
$geoEntities = @("Coordinate","Vec","Point","Vector","Plane","Camera")
foreach ($e in $geoEntities) {
    DelConst "get${e}Diff"
    DelConst "apply${e}Diff"
    DelConst "inverse${e}Diff"
    DelConst "merge${e}Diff"
    DelConst "roundPlane"
}

Write-Host "`n=== TASK 5.5: Delete free design-diff builder functions ==="
$ddBuilders = @(
    "addPieceToDesignDiff","removePieceFromDesignDiff","addConnectionToDesignDiff",
    "removeConnectionFromDesignDiff","setPieceInDesignDiff","addPiecesToDesignDiff",
    "removePiecesFromDesignDiff","setPiecesInDesignDiff","addConnectionsToDesignDiff",
    "removeConnectionsFromDesignDiff","setConnectionInDesignDiff","setConnectionsInDesignDiff"
)
foreach ($f in $ddBuilders) {
    DelConst $f
}

Write-Host "`n=== TASK 5.6: Delete free collection-diff functions ==="
DelConst "getCollectionDiff"
DelConst "inverseCollectionDiff"
DelConst "applyCollectionDiff"
DelConst "mergeCollectionDiff"

Write-Host "`n=== TASK 5.7: Delete free backbone factory functions ==="
DelConst "createLocalBackbone"
DelConst "createDevBackbone"
DelConst "createRemoteBackbone"

Write-Host "`n=== TASK 5.8: Delete free kit conversion functions ==="
DelConst "asKitInstance"
DelConst "requireKit"
DelConst "duplicateKitForIsolation"
DelConst "stripNullsJsonClone"
DelConst "detachPieceForLocalMutation"
DelConst "detachConnectionForLocalMutation"
DelConst "detachDesignForLocalMutation"
DelConst "designWithDiff"
DelConst "duplicateDesignDiffForIsolation"


Write-Host "`n=== TASK 6.1: Delete flatten geometry functions ==="
DelConst "computeChildPlane"
DelConst "connectionPlacementTranslationBasis"
DelConst "flattenPlacementWalkDesignOrderRoots"
DelConst "buildFlattenPieceAdjacency"
DelConst "collectUndirectedComponentIds"
DelConst "moveTranslationWorldFromPiecePlane"
DelConst "childConnectorOriginWorld"
DelConst "solveConnectionOriginMinNorm"
DelConst "connectionDiffFromStructuralMoveVector"

Write-Host "`n=== TASK 6.2: Delete representation selection functions ==="
DelConst "selectBestRepresentation"
DelConst "filterRepresentationsByTagIds"
DelConst "getAvailableTagIdsForRepresentations"
DelConst "getAllTagIdsFromRepresentations"

Write-Host "`n=== TASK 6.3: Delete type/design/piece resolution functions ==="
DelConst "resolvePieceTypeForFlatten"
DelConst "findTypeInKit"
DelConst "findDesignInKit"
DelConst "findPieceInDesign"
DelConst "findConnectionInDesign"
DelConst "findPortInKit"
DelConst "findPieceTypeInDesign"
DelConst "findParentPieceInDesign"
DelConst "findParentConnectionForPieceInDesign"
DelConst "findChildrenPiecesInDesign"
DelConst "findUsedConnectorsByPieceInDesign"
DelConst "findReplacableTypesForPieceInDesign"
DelConst "findReplacableTypesForPiecesInDesign"
DelConst "sumQualityInDesign"

Write-Host "`n=== TASK 6.4: Delete connector compatibility functions ==="
DelConst "arePortsCompatible"
DelConst "areConnectorsCompatible"
DelConst "unifyConnectorPortsAndCompatiblePortsForTypes"

Write-Host "`n=== TASK 6.5: Delete diff computation/application/inversion/merge ==="
# Per-entity diff functions
$diffEntities = @("Type","Design","Kit","Piece","Connection","Attribute","Author","File","Folder","Benchmark","Quality","Port","Family","Prop","Tag","Concept","Representation","Connector","Layer","Group","Stat","Side","SideId")
foreach ($e in $diffEntities) {
    DelConst "get${e}Diff"
    DelConst "apply${e}Diff"
    DelConst "inverse${e}Diff"
    DelConst "merge${e}Diff"
}
# Additional diff variants
DelConst "applyDesignDiffCore"
DelConst "computeKitGraphDiffBetween"
DelConst "applyLedgerDiffToKitEntity"
DelConst "inverseKitGraphDiff"
DelConst "mergeKitGraphDiff"
DelConst "composeLedgerDiffs"
DelConst "getKitChange"

Write-Host "`n=== TASK 6.7: Delete copy/paste logic ==="
DelConst "copyDesign"
DelConst "pasteDesign"
DelConst "mergeDesigns"
DelConst "orientDesign"

Write-Host "`n=== TASK 6.8: Delete design mutation helpers ==="
DelConst "deletePiecesAndConnectionsInDesign"
DelConst "removePiecesAndConnectionsFromDesign"
DelConst "fixPieceInDesign"
DelConst "buildDragMoveStructuralContext"

Write-Host "`n=== TASK 6.10: Delete validation functions ==="
DelConst "validateKitEntityDiff"
DelConst "kitEntityDiffIsBlocking"
DelConst "validationReportFromGraph"
DelConst "graphValidationFromLedgerReport"

Write-Host "`n=== TASK 6.12: Delete wire projection/conversion functions ==="
DelConst "kitWireProjectionFromImpl"
DelConst "kitDataFromWireDto"
DelConst "emptyKitWireDto"
DelConst "kitGraphToPlainData"

Write-Host "`n=== TASK 6.13: Delete ledger diff functions ==="
DelConst "emptyLedgerDiff"
DelConst "normalizeLedgerDiff"
DelConst "squashLedgerChangesForward"
DelConst "squashLedgerChangesBackward"
DelConst "invertLedgerDiff"
DelConst "ledgerKitChangeFromGraph"
DelConst "graphKitChangeFromLedger"

Write-Host "`n=== TASK 6.14: Delete clusterable group functions ==="
DelConst "getClusterableGroups"
DelConst "getIncludedDesigns"

Write-Host "`n=== TASK 4.1: Delete caching code ==="
DelConst "ensureFlattenGeometryCache"
DelConst "getFlattenMerkleCache"
DelConst "invalidateFlattenMerkleCaches"
DelConst "piecesMetadataCached"
DelConst "cachedSqlJs"
# FlatMerkleCacheEntry type/interface
$fmcIdx = FindIdx "FlatMerkleCacheEntry" 0
if ($fmcIdx -ge 0 -and $lines[$fmcIdx] -match "^export (type|interface) FlatMerkleCacheEntry") {
    $s = FindDocStart $fmcIdx
    $e = FindClose $fmcIdx
    if ($e -lt 0) {
        $e = $fmcIdx
        while ($e -lt $lines.Count -and -not $lines[$e].TrimEnd().EndsWith(';')) { $e++ }
    }
    Mark $s $e
}


# ============================================================
# PHASE 4: Additional items to delete
# ============================================================

Write-Host "`n=== Additional deletions ==="

# Delete the Backbone interface and KitGraphChange (they reference KitImpl)
# Actually - Backbone interface is used by BackboneKitStores which we keep... 
# Let's check what references Backbone

# Delete KitLike type alias (references KitImpl)
$klIdx = FindIdx "^export type KitLike = " 0
if ($klIdx -ge 0) {
    $s = FindDocStart $klIdx
    Mark $s $klIdx
}

# Delete ValidationState type (references KitDiffValidationResult which is being deleted)
$vsIdx = FindIdx "^export type ValidationState = " 0
if ($vsIdx -ge 0) {
    $s = FindDocStart $vsIdx
    # Include the region markers
    $regionStart = $vsIdx
    while ($regionStart -gt 0 -and $lines[$regionStart - 1].Trim().StartsWith('// #region')) {
        $regionStart--
    }
    $regionEnd = $vsIdx
    while ($regionEnd + 1 -lt $lines.Count -and $lines[$regionEnd + 1].Trim().StartsWith('// #endregion')) {
        $regionEnd++
    }
    Mark ([Math]::Min($s, $regionStart)) $regionEnd
}

# Delete SemanticCommand interface if it exists
$scIdx = FindIdx "^export interface SemanticCommand\b" 0
if ($scIdx -ge 0) {
    $s = FindDocStart $scIdx
    $e = FindClose $scIdx
    if ($e -ge 0) { Mark $s $e }
}

# Delete KitEntityDesignId, KitEntityPieceId types if they exist
foreach ($t in @("KitEntityDesignId","KitEntityPieceId","KitEntityTypeId")) {
    $tIdx = FindIdx "^export type $t\b" 0
    if ($tIdx -ge 0) {
        $s = FindDocStart $tIdx
        $e = $tIdx
        while ($e -lt $lines.Count -and -not $lines[$e].TrimEnd().EndsWith(';')) { $e++ }
        Mark $s $e
    }
}

# Delete the Backbone interface itself (it references KitImpl)
$biIdx = FindIdx "^export interface Backbone\b" 0
if ($biIdx -ge 0) {
    $s = FindDocStart $biIdx
    $e = FindClose $biIdx
    if ($e -ge 0) { Mark $s $e }
}

# Delete KitGraphChange interface
$kgcIdx = FindIdx "^export interface KitGraphChange\b" 0
if ($kgcIdx -ge 0) {
    $s = FindDocStart $kgcIdx
    $e = FindClose $kgcIdx
    if ($e -ge 0) { Mark $s $e }
}

# Delete the old KitStore interface and related types (they reference KitImpl)
# KitStoreStatus, KitSyncState, KitStoreSnapshot, KitStore, UndoableKitStore, BlobAssetStore, ObservablePathStore
foreach ($iface in @("KitStoreStatus","KitSyncState","KitStoreSnapshot")) {
    $tIdx = FindIdx "^export type $iface\b" 0
    if ($tIdx -ge 0) {
        $s = FindDocStart $tIdx
        $e = FindClose $tIdx
        if ($e -lt 0) {
            $e = $tIdx
            while ($e -lt $lines.Count -and -not $lines[$e].TrimEnd().EndsWith(';')) { $e++ }
        }
        Mark $s $e
    }
}

foreach ($iface in @("KitStore","UndoableKitStore","BlobAssetStore","ObservablePathStore")) {
    $ifIdx = FindIdx "^export interface $iface\b" 0
    if ($ifIdx -ge 0) {
        $s = FindDocStart $ifIdx
        $e = FindClose $ifIdx
        if ($e -ge 0) { Mark $s $e }
    }
}

# Delete KitUndoEntry type
$kueIdx = FindIdx "KitUndoEntry" 0
if ($kueIdx -ge 0 -and $lines[$kueIdx] -match "^(export )?(type|interface) KitUndoEntry") {
    $s = FindDocStart $kueIdx
    $e = $kueIdx
    while ($e -lt $lines.Count -and -not $lines[$e].TrimEnd().EndsWith(';')) { $e++ }
    Mark $s $e
}

# Delete colorPortsForTypes
DelConst "colorPortsForTypes"

# Delete findAttributeValue
DelConst "findAttributeValue"

# Delete getColorForText (private function)
$gcIdx = FindIdx "^const getColorForText = " 0
if ($gcIdx -ge 0) {
    $s = FindDocStart $gcIdx
    $e = FindClose $gcIdx
    if ($e -ge 0) { Mark $s $e }
}

# Delete PiecePlacementMetadata type
$ppmIdx = FindIdx "^export type PiecePlacementMetadata = " 0
if ($ppmIdx -ge 0) {
    $s = FindDocStart $ppmIdx
    $e = FindClose $ppmIdx
    if ($e -ge 0) { Mark $s $e }
}

# Delete piecesMetadataCached region
$pmcStart = FindIdx "// #region.*Pieces Metadata Cached" 0
if ($pmcStart -ge 0) {
    $pmcEnd = FindIdx "// #endregion.*Pieces Metadata Cached" $pmcStart
    if ($pmcEnd -ge 0) { Mark $pmcStart $pmcEnd }
}

# Delete areSameConnection helper (used by design diff builders)
$ascIdx = FindIdx "^const areSameConnection\b|^export const areSameConnection\b" 0
if ($ascIdx -ge 0) {
    $s = FindDocStart $ascIdx
    $e = FindClose $ascIdx
    if ($e -lt 0) {
        $e = $ascIdx
        while ($e -lt $lines.Count -and -not $lines[$e].TrimEnd().EndsWith(';')) { $e++ }
    }
    Mark $s $e
}


# ============================================================
# PHASE 5: Write result
# ============================================================

Write-Host "`n=== Writing result ==="
$kept = [System.Collections.Generic.List[string]]::new()
for ($i = 0; $i -lt $lines.Count; $i++) {
    if (-not $del.Contains($i)) {
        $kept.Add($lines[$i])
    }
}

$deletedCount = $del.Count
Write-Host "Lines deleted: $deletedCount"
Write-Host "Lines remaining: $($kept.Count)"

[System.IO.File]::WriteAllLines($filePath, $kept.ToArray())
Write-Host "File written successfully."
