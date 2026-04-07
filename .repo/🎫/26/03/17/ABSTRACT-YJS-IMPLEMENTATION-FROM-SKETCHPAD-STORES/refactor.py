"""Mechanical refactoring script to abstract Yjs from Sketchpad stores."""
import re
import sys

def refactor_file(filepath: str) -> None:
    with open(filepath, 'r') as f:
        content = f.read()
    original = content

    # === PHASE 1: Replace Y.Map/Y.Array/Y.Doc type references ===
    # Replace Y.YMapEvent<...> with RMapEvent (not generic)
    content = re.sub(r'Y\.YMapEvent<[^>]*>', 'RMapEvent', content)

    # Replace Y.Map<...> with RMap<...> - careful not to match YLeafMapString etc
    content = re.sub(r'\bY\.Map<', 'RMap<', content)
    # Replace standalone Y.Map (without generic) 
    content = re.sub(r'\bY\.Map\b(?!<)', 'RMap', content)
    
    # Replace Y.Array<...> with RArray<...>
    content = re.sub(r'\bY\.Array<', 'RArray<', content)
    # Replace standalone Y.Array
    content = re.sub(r'\bY\.Array\b(?!<)', 'RArray', content)
    
    # Replace Y.Doc with RDoc
    content = re.sub(r'\bY\.Doc\b', 'RDoc', content)

    # === PHASE 2: Replace instanceof checks ===
    # instanceof Y.Map → isRMap(expr) - need to capture the expression before instanceof
    content = re.sub(r'(\w+)\s+instanceof\s+RMap\b', r'isRMap(\1)', content)
    # instanceof Y.Array
    content = re.sub(r'(\w+)\s+instanceof\s+RArray\b', r'isRArray(\1)', content)

    # === PHASE 3: Replace new Y.Map/new Y.Array/new Y.Doc constructors ===
    # new Y.Map<...>() → this.rDoc.createMap<...>() or rDoc.createMap<...>()
    # We'll mark these for manual review with a placeholder
    content = re.sub(r'new RMap<([^>]*)>\(\)', r'this.rDoc.createMap<\1>()', content)
    content = re.sub(r'new RMap\(\)\s*as\s+', r'this.rDoc.createMap() as ', content)
    content = re.sub(r'new RMap\(\)', r'this.rDoc.createMap()', content)
    content = re.sub(r'new RArray<([^>]*)>\(\)', r'this.rDoc.createArray<\1>()', content)
    content = re.sub(r'new RArray\(\)\s*as\s+', r'this.rDoc.createArray() as ', content)  
    content = re.sub(r'new RArray\(\)', r'this.rDoc.createArray()', content)
    content = re.sub(r'new RDoc\(\)', r'this.docFactory()', content)
    
    # === PHASE 4: Rename Y-prefixed type aliases ===
    # Order matters: longer names first to avoid partial matches
    ytypes = [
        # From shared.ts
        ('YPathSegment', 'RxPathSegment'),
        ('YPath', 'RxPath'),
        ('YProviderFactory', 'RxProviderFactory'),
        ('YUuidArray', 'RxUuidArray'),
        ('YUuid', 'RxUuid'),
        ('YConcepts', 'RxConcepts'),  # before YConcept
        ('YConcept', 'RxConcept'),
        ('YStringArray', 'RxStringArray'),
        ('YLeafMapString', 'RxLeafMapString'),
        ('YLeafMapNumber', 'RxLeafMapNumber'),
        ('YAttributes', 'RxAttributes'),
        # From Sketchpad.tsx 
        ('YAttributeVal', 'RxAttributeVal'),
        ('YAttribute', 'RxAttribute'),
        ('YCoordVal', 'RxCoordVal'),
        ('YCoord', 'RxCoord'),
        ('YVecVal', 'RxVecVal'),
        ('YVec', 'RxVec'),
        ('YPointVal', 'RxPointVal'),
        ('YPoint', 'RxPoint'),
        ('YVectorVal', 'RxVectorVal'),
        ('YVector', 'RxVector'),
        ('YPlaneVal', 'RxPlaneVal'),
        ('YPlane', 'RxPlane'),
        ('YCameraVal', 'RxCameraVal'),
        ('YCamera', 'RxCamera'),
        ('YLocationVal', 'RxLocationVal'),
        ('YLocation', 'RxLocation'),
        ('YAuthorVal', 'RxAuthorVal'),
        ('YAuthorUuids', 'RxAuthorUuids'),
        ('YAuthorUuid', 'RxAuthorUuid'),
        ('YAuthors', 'RxAuthors'),
        ('YAuthor', 'RxAuthor'),
        ('YFiles', 'RxFiles'),
        ('YFile', 'RxFile'),
        ('YFolders', 'RxFolders'),
        ('YFolder', 'RxFolder'),
        ('YBenchmarks', 'RxBenchmarks'),
        ('YBenchmark', 'RxBenchmark'),
        ('YQualities', 'RxQualities'),
        ('YQuality', 'RxQuality'),
        ('YProps', 'RxProps'),
        ('YProp', 'RxProp'),
        ('YModelVal', 'RxModelVal'),
        ('YModels', 'RxModels'),
        ('YModel', 'RxModel'),
        ('YConnectorVal', 'RxConnectorVal'),
        ('YConnectors', 'RxConnectors'),
        ('YConnector', 'RxConnector'),
        ('YTypeVal', 'RxTypeVal'),
        ('YTypes', 'RxTypes'),
        ('YType', 'RxType'),
        ('YLayers', 'RxLayers'),
        ('YLayer', 'RxLayer'),
        ('YPieceVal', 'RxPieceVal'),
        ('YPieces', 'RxPieces'),
        ('YPiece', 'RxPiece'),
        ('YGroupVal', 'RxGroupVal'),
        ('YGroups', 'RxGroups'),
        ('YGroup', 'RxGroup'),
        ('YSideVal', 'RxSideVal'),
        ('YSides', 'RxSides'),
        ('YSide', 'RxSide'),
        ('YConnectionVal', 'RxConnectionVal'),
        ('YConnections', 'RxConnections'),
        ('YConnection', 'RxConnection'),
        ('YStats', 'RxStats'),
        ('YStat', 'RxStat'),
        ('YDesignVal', 'RxDesignVal'),
        ('YDesigns', 'RxDesigns'),
        ('YDesign', 'RxDesign'),
        ('YConceptVal', 'RxConceptVal'),
        ('YIdMap', 'RxIdMap'),
        ('YKitVal', 'RxKitVal'),
        ('YKits', 'RxKits'),
        ('YKit', 'RxKit'),
        ('YKitAppVal', 'RxKitAppVal'),
        ('YKitApps', 'RxKitApps'),
        ('YKitApp', 'RxKitApp'),
        ('YKitMetadatas', 'RxKitMetadatas'),
        ('YKitMetadata', 'RxKitMetadata'),
        ('YSketchpadVal', 'RxSketchpadVal'),
        ('YSketchpad', 'RxSketchpad'),
    ]
    
    for old, new in ytypes:
        # Use word boundary to avoid partial matches
        content = re.sub(r'\b' + old + r'\b', new, content)

    # === PHASE 5: Rename Y-prefixed functions ===
    yfuncs = [
        ('yPathMapKey', 'rxPathMapKey'),
        ('yPathArrayIndex', 'rxPathArrayIndex'),
        ('createYjsSyncActor', 'createRxSyncActor'),
        ('createYjsFieldSyncActor', 'createRxFieldSyncActor'),
    ]
    for old, new in yfuncs:
        content = re.sub(r'\b' + old + r'\b', new, content)

    # === PHASE 6: Rename yDoc/yKit etc field names ===
    # yDoc → rDoc (field/variable/parameter)
    content = re.sub(r'\byDoc\b', 'rDoc', content)
    # yKit → rKit
    content = re.sub(r'\byKit\b', 'rKit', content)
    # ySketchpad → rSketchpad
    content = re.sub(r'\bySketchpad\b', 'rSketchpad', content)
    # yKits → rKits
    content = re.sub(r'\byKits\b', 'rKits', content)
    # yKitApps → rKitApps
    content = re.sub(r'\byKitApps\b', 'rKitApps', content)
    # yDesign → rDesign
    content = re.sub(r'\byDesign\b', 'rDesign', content)
    # yDesigns → rDesigns
    content = re.sub(r'\byDesigns\b', 'rDesigns', content)
    # yTypes → rTypes
    content = re.sub(r'\byTypes\b', 'rTypes', content)
    # yType → rType
    content = re.sub(r'\byType\b', 'rType', content)
    # yPiece → rPiece
    content = re.sub(r'\byPiece\b', 'rPiece', content)
    # yPieces → rPieces
    content = re.sub(r'\byPieces\b', 'rPieces', content)
    # yConnection → rConnection
    content = re.sub(r'\byConnection\b', 'rConnection', content)
    # yConnections → rConnections
    content = re.sub(r'\byConnections\b', 'rConnections', content)
    # yAttribute → rAttribute
    content = re.sub(r'\byAttribute\b', 'rAttribute', content)
    # yAttributes → rAttributes
    content = re.sub(r'\byAttributes\b', 'rAttributes', content)
    # yAuthor → rAuthor
    content = re.sub(r'\byAuthor\b', 'rAuthor', content)
    # yAuthors → rAuthors
    content = re.sub(r'\byAuthors\b', 'rAuthors', content)
    # yFile → rFile
    content = re.sub(r'\byFile\b', 'rFile', content)
    # yFiles → rFiles
    content = re.sub(r'\byFiles\b', 'rFiles', content)
    # yFolder → rFolder
    content = re.sub(r'\byFolder\b', 'rFolder', content)
    # yFolders → rFolders
    content = re.sub(r'\byFolders\b', 'rFolders', content)
    # yQuality → rQuality
    content = re.sub(r'\byQuality\b', 'rQuality', content)
    # yQualities → rQualities
    content = re.sub(r'\byQualities\b', 'rQualities', content)
    # yConcepts → rConcepts
    content = re.sub(r'\byConcepts\b', 'rConcepts', content)
    # yConcept → rConcept
    content = re.sub(r'\byConcept\b', 'rConcept', content)
    # yBenchmarks → rBenchmarks
    content = re.sub(r'\byBenchmarks\b', 'rBenchmarks', content)
    # yBenchmark → rBenchmark
    content = re.sub(r'\byBenchmark\b', 'rBenchmark', content)
    # yConnector → rConnector
    content = re.sub(r'\byConnector\b', 'rConnector', content)
    # yMap → rMap (the Store base class field)
    content = re.sub(r'\byMap\b', 'rMap', content)
    # yModel → rModel
    content = re.sub(r'\byModel\b', 'rModel', content)
    # yGroup → rGroup
    content = re.sub(r'\byGroup\b', 'rGroup', content)
    # yGroups → rGroups
    content = re.sub(r'\byGroups\b', 'rGroups', content)
    # yLayer → rLayer
    content = re.sub(r'\byLayer\b', 'rLayer', content)
    # yLayers → rLayers
    content = re.sub(r'\byLayers\b', 'rLayers', content)
    # yStats → rStats
    content = re.sub(r'\byStats\b', 'rStats', content)
    # yStat → rStat
    content = re.sub(r'\byStat\b', 'rStat', content)
    # yProp → rProp
    content = re.sub(r'\byProp\b', 'rProp', content)
    # yProps → rProps
    content = re.sub(r'\byProps\b', 'rProps', content)
    # ySide → rSide
    content = re.sub(r'\bySide\b', 'rSide', content)
    # ySides → rSides (if exists)
    content = re.sub(r'\bySides\b', 'rSides', content)
    # yStack → rStack
    content = re.sub(r'\byStack\b', 'rStack', content)
    # yPanelVisibility → rPanelVisibility
    content = re.sub(r'\byPanelVisibility\b', 'rPanelVisibility', content)
    # yOrigin → rOrigin
    content = re.sub(r'\byOrigin\b', 'rOrigin', content)
    # yXAxis → rXAxis
    content = re.sub(r'\byXAxis\b', 'rXAxis', content)
    # yYAxis → rYAxis
    content = re.sub(r'\byYAxis\b', 'rYAxis', content)
    # yPosition → rPosition
    content = re.sub(r'\byPosition\b', 'rPosition', content)
    # yForward → rForward
    content = re.sub(r'\byForward\b', 'rForward', content)
    # yUp → rUp
    content = re.sub(r'\byUp\b', 'rUp', content)
    # yPoint → rPoint
    content = re.sub(r'\byPoint\b', 'rPoint', content)
    # yDirection → rDirection
    content = re.sub(r'\byDirection\b', 'rDirection', content)
    # yPlane → rPlane
    content = re.sub(r'\byPlane\b', 'rPlane', content)
    # yCenter → rCenter
    content = re.sub(r'\byCenter\b', 'rCenter', content)
    # yMirrorPlane → rMirrorPlane
    content = re.sub(r'\byMirrorPlane\b', 'rMirrorPlane', content)
    # yLocation → rLocation
    content = re.sub(r'\byLocation\b', 'rLocation', content)
    # yTags → rTags
    content = re.sub(r'\byTags\b', 'rTags', content)
    # yConnected → rConnected
    content = re.sub(r'\byConnected\b', 'rConnected', content)
    # yConnecting → rConnecting
    content = re.sub(r'\byConnecting\b', 'rConnecting', content)
    # yTutorials → rTutorials
    content = re.sub(r'\byTutorials\b', 'rTutorials', content)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Refactored: {filepath}")
    else:
        print(f"No changes: {filepath}")

if __name__ == '__main__':
    for fp in sys.argv[1:]:
        refactor_file(fp)
