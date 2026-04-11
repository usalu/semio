$file = "c:\git\semio\semio\rs\lib.rs"
$content = [System.IO.File]::ReadAllText($file, [System.Text.Encoding]::UTF8)
$fixCount = 0

# ============================================================
# SECTION 1: Fix all doc comment summary lines
# Pattern: /// EMOJI1<summary>EMOJI2Name holds the data fields for a Name record.</summary>
# or: /// <summary>EMOJI2Name holds the data fields for a Name record.</summary>
# Also remove duplicate <summary> lines
# ============================================================

# Entity canonical emojis and descriptions
$entityMap = @{
    "Attribute" = @{ emoji = [char]::ConvertFromUtf32(0x1F48E); desc = "a key-value metadata entry with optional definition" }
    "AttributeId" = @{ emoji = [char]::ConvertFromUtf32(0x1F48E); desc = "identifies an attribute entity by GUID" }
    "Location" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CD); desc = "a geographic point with longitude, latitude and optional altitude" }
    "LocationId" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CD); desc = "identifies a location entity by GUID" }
    "Author" = @{ emoji = [char]0x270D + [char]0xFE0F; desc = "a named contributor with email and custom attributes" }
    "AuthorId" = @{ emoji = [char]0x270D + [char]0xFE0F; desc = "identifies an author entity by GUID" }
    "File" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C4); desc = "a named binary resource with optional remote URL and folder" }
    "FileId" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C4); desc = "identifies a file entity by GUID" }
    "Folder" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C1); desc = "a named directory for organizing files" }
    "FolderId" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C1); desc = "identifies a folder entity by GUID" }
    "Benchmark" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CF); desc = "a named metric range with min/max bounds and optional icon" }
    "BenchmarkId" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CF); desc = "identifies a benchmark entity by GUID" }
    "Quality" = @{ emoji = [char]::ConvertFromUtf32(0x1F52C); desc = "a measurable property with formula, units and benchmarks" }
    "QualityId" = @{ emoji = [char]::ConvertFromUtf32(0x1F52C); desc = "identifies a quality entity by GUID" }
    "QualityKind" = @{ emoji = [char]::ConvertFromUtf32(0x1F52C); desc = "the numeric kind of a quality (integer, float or boolean)" }
    "Port" = @{ emoji = [char]0x2693; desc = "a named connection interface with compatible ports" }
    "PortId" = @{ emoji = [char]0x2693; desc = "identifies a port entity by GUID" }
    "Prop" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CA); desc = "a quality measurement value with optional unit" }
    "PropId" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CA); desc = "identifies a prop entity by GUID" }
    "Tag" = @{ emoji = [char]::ConvertFromUtf32(0x1F3F7) + [char]0xFE0F; desc = "a named categorization label with optional description and icon" }
    "TagId" = @{ emoji = [char]::ConvertFromUtf32(0x1F3F7) + [char]0xFE0F; desc = "identifies a tag entity by GUID" }
    "Concept" = @{ emoji = [char]::ConvertFromUtf32(0x1F4A1); desc = "a named categorization concept with optional description and icon" }
    "ConceptId" = @{ emoji = [char]::ConvertFromUtf32(0x1F4A1); desc = "identifies a concept entity by GUID" }
    "Model" = @{ emoji = [char]::ConvertFromUtf32(0x1F5FF); desc = "a 3D model reference linking a file with tags and description" }
    "ModelId" = @{ emoji = [char]::ConvertFromUtf32(0x1F5FF); desc = "identifies a model entity by GUID" }
    "Connector" = @{ emoji = [char]::ConvertFromUtf32(0x1F50C); desc = "a connection point on a type with position, direction and parameter" }
    "ConnectorId" = @{ emoji = [char]::ConvertFromUtf32(0x1F50C); desc = "identifies a connector entity by GUID" }
    "Type" = @{ emoji = [char]::ConvertFromUtf32(0x1F9F1); desc = "a reusable element blueprint with connectors, models and props" }
    "TypeId" = @{ emoji = [char]::ConvertFromUtf32(0x1F9F1); desc = "identifies a type entity by GUID" }
    "Layer" = @{ emoji = [char]::ConvertFromUtf32(0x1F3A8); desc = "a named visibility and color layer within a design" }
    "LayerId" = @{ emoji = [char]::ConvertFromUtf32(0x1F3A8); desc = "identifies a layer entity by GUID" }
    "Piece" = @{ emoji = [char]::ConvertFromUtf32(0x1F9E9); desc = "a positioned instance of a type within a design" }
    "PieceId" = @{ emoji = [char]::ConvertFromUtf32(0x1F9E9); desc = "identifies a piece entity by GUID" }
    "Group" = @{ emoji = [char]::ConvertFromUtf32(0x1F465); desc = "a named collection of pieces within a design" }
    "GroupId" = @{ emoji = [char]::ConvertFromUtf32(0x1F465); desc = "identifies a group entity by GUID" }
    "Side" = @{ emoji = [char]0x2194 + [char]0xFE0F; desc = "one side of a connection identifying a piece and optional connector" }
    "Connection" = @{ emoji = [char]::ConvertFromUtf32(0x1F517); desc = "a spatial relationship between two pieces with gap, shift and rotation" }
    "ConnectionId" = @{ emoji = [char]::ConvertFromUtf32(0x1F517); desc = "identifies a connection entity by GUID" }
    "Stat" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C8); desc = "a statistical quality measurement with min/max bounds and unit" }
    "StatId" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C8); desc = "identifies a stat entity by GUID" }
    "Design" = @{ emoji = [char]::ConvertFromUtf32(0x1F4D0); desc = "an assembly of pieces, connections, layers and groups" }
    "DesignId" = @{ emoji = [char]::ConvertFromUtf32(0x1F4D0); desc = "identifies a design entity by GUID" }
    "Kit" = @{ emoji = [char]::ConvertFromUtf32(0x1F4E6); desc = "the root container for all domain entities" }
    "Coord" = @{ emoji = [char]::ConvertFromUtf32(0x1F4FA); desc = "a 2D coordinate with U and V components" }
    "Vec" = @{ emoji = [char]0x27A1 + [char]0xFE0F; desc = "a 2D vector with U and V components" }
    "Point" = @{ emoji = [char]0x2716 + [char]0xFE0F; desc = "a 3D point with X, Y and Z components" }
    "Vector" = @{ emoji = [char]0x2197 + [char]0xFE0F; desc = "a 3D vector with X, Y and Z components" }
    "Plane" = @{ emoji = [char]0x25FB + [char]0xFE0F; desc = "a plane defined by origin point and two axis vectors" }
    "Camera" = @{ emoji = [char]::ConvertFromUtf32(0x1F3A5); desc = "a camera defined by position, forward and up vectors" }
}

# Value type section emojis
$sectionEntityEmojis = @{
    "Attribute" = [char]::ConvertFromUtf32(0x1F48E)
    "Coord" = [char]::ConvertFromUtf32(0x1F4FA)
    "Vector" = [char]0x2197 + [char]0xFE0F
    "Plane" = [char]0x25FB + [char]0xFE0F
    "Camera" = [char]::ConvertFromUtf32(0x1F3A5)
    "Location" = [char]::ConvertFromUtf32(0x1F4CD)
    "Author" = [char]0x270D + [char]0xFE0F
    "File" = [char]::ConvertFromUtf32(0x1F4C4)
    "Folder" = [char]::ConvertFromUtf32(0x1F4C1)
    "Quality" = [char]::ConvertFromUtf32(0x1F52C)
    "Port" = [char]0x2693
    "Tag" = [char]::ConvertFromUtf32(0x1F3F7) + [char]0xFE0F
    "Concept" = [char]::ConvertFromUtf32(0x1F4A1)
    "Prop" = [char]::ConvertFromUtf32(0x1F4CA)
    "Model" = [char]::ConvertFromUtf32(0x1F5FF)
    "Connector" = [char]::ConvertFromUtf32(0x1F50C)
    "Type" = [char]::ConvertFromUtf32(0x1F9F1)
    "Layer" = [char]::ConvertFromUtf32(0x1F3A8)
    "Piece" = [char]::ConvertFromUtf32(0x1F9E9)
    "Group" = [char]::ConvertFromUtf32(0x1F465)
    "Side" = [char]0x2194 + [char]0xFE0F
    "Connection" = [char]::ConvertFromUtf32(0x1F517)
    "Stat" = [char]::ConvertFromUtf32(0x1F4C8)
    "Design" = [char]::ConvertFromUtf32(0x1F4D0)
    "Kit" = [char]::ConvertFromUtf32(0x1F4E6)
    "Benchmark" = [char]::ConvertFromUtf32(0x1F4CF)
}

# Diff descriptions
$diffDescMap = @{
    "AttributeDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F48E); desc = "a partial update to attribute's fields" }
    "PropDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CA); desc = "a partial update to prop's fields" }
    "ConnectorDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F50C); desc = "a partial update to connector's fields" }
    "ModelDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F5FF); desc = "a partial update to model's fields" }
    "TypeDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F9F1); desc = "a partial update to type's fields" }
    "SideDiff" = @{ emoji = [char]0x2194 + [char]0xFE0F; desc = "a partial update to side's fields" }
    "ConnectionDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F517); desc = "a partial update to connection's fields" }
    "PieceDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F9E9); desc = "a partial update to piece's fields" }
    "LayerDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F3A8); desc = "a partial update to layer's fields" }
    "GroupDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F465); desc = "a partial update to group's fields" }
    "StatDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C8); desc = "a partial update to stat's fields" }
    "DesignDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4D0); desc = "a partial update to design's fields" }
    "TagDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F3F7) + [char]0xFE0F; desc = "a partial update to tag's fields" }
    "ConceptDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4A1); desc = "a partial update to concept's fields" }
    "PortDiff" = @{ emoji = [char]0x2693; desc = "a partial update to port's fields" }
    "QualityDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F52C); desc = "a partial update to quality's fields" }
    "FileDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C4); desc = "a partial update to file's fields" }
    "FolderDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C1); desc = "a partial update to folder's fields" }
    "AuthorDiff" = @{ emoji = [char]0x270D + [char]0xFE0F; desc = "a partial update to author's fields" }
    "KitDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4E6); desc = "a partial update to kit's fields" }
    "BenchmarkDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CF); desc = "a partial update to benchmark's fields" }
    "LocationDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CD); desc = "a partial update to location's fields" }
}

# sDiff = CollectionDiff / batched
$sDiffDescMap = @{
    "AttributesDiff" = @{ emoji = [char]::ConvertFromUtf32(0x1F48E); desc = "batched attribute additions, removals and updates" }
}

# Change descriptions
$changeDescMap = @{
    "AttributeChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F48E); desc = "tracks attribute modifications in a kit change" }
    "AuthorChange" = @{ emoji = [char]0x270D + [char]0xFE0F; desc = "tracks author modifications in a kit change" }
    "FileChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C4); desc = "tracks file modifications in a kit change" }
    "FolderChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C1); desc = "tracks folder modifications in a kit change" }
    "QualityChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F52C); desc = "tracks quality modifications in a kit change" }
    "PortChange" = @{ emoji = [char]0x2693; desc = "tracks port modifications in a kit change" }
    "PropChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CA); desc = "tracks prop modifications in a kit change" }
    "TagChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F3F7) + [char]0xFE0F; desc = "tracks tag modifications in a kit change" }
    "ConceptChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F4A1); desc = "tracks concept modifications in a kit change" }
    "ModelChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F5FF); desc = "tracks model modifications in a kit change" }
    "ConnectorChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F50C); desc = "tracks connector modifications in a kit change" }
    "TypeChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F9F1); desc = "tracks type modifications in a kit change" }
    "LayerChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F3A8); desc = "tracks layer modifications in a kit change" }
    "PieceChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F9E9); desc = "tracks piece modifications in a kit change" }
    "GroupChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F465); desc = "tracks group modifications in a kit change" }
    "SideChange" = @{ emoji = [char]0x2194 + [char]0xFE0F; desc = "tracks side modifications in a kit change" }
    "ConnectionChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F517); desc = "tracks connection modifications in a kit change" }
    "StatChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C8); desc = "tracks stat modifications in a kit change" }
    "DesignChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F4D0); desc = "tracks design modifications in a kit change" }
    "KitChange" = @{ emoji = [char]::ConvertFromUtf32(0x1F4E6); desc = "tracks kit-level modifications" }
}

# Meta type descriptions
$metaDescMap = @{
    "AttributeMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F48E); desc = "scalar-only view of attribute excluding nested arrays" }
    "StatMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C8); desc = "scalar-only view of stat excluding nested arrays" }
    "TagMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F3F7) + [char]0xFE0F; desc = "scalar-only view of tag excluding nested arrays" }
    "ConceptMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4A1); desc = "scalar-only view of concept excluding nested arrays" }
    "PropMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CA); desc = "scalar-only view of prop excluding nested arrays" }
    "AuthorMeta" = @{ emoji = [char]0x270D + [char]0xFE0F; desc = "scalar-only view of author excluding nested arrays" }
    "FileMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C4); desc = "scalar-only view of file excluding nested arrays" }
    "FolderMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4C1); desc = "scalar-only view of folder excluding nested arrays" }
    "QualityMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F52C); desc = "scalar-only view of quality excluding nested arrays" }
    "PortMeta" = @{ emoji = [char]0x2693; desc = "scalar-only view of port excluding nested arrays" }
    "ConnectorMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F50C); desc = "scalar-only view of connector excluding nested arrays" }
    "ModelMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F5FF); desc = "scalar-only view of model excluding nested arrays" }
    "TypeMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F9F1); desc = "scalar-only view of type excluding nested arrays" }
    "LayerMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F3A8); desc = "scalar-only view of layer excluding nested arrays" }
    "PieceMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F9E9); desc = "scalar-only view of piece excluding nested arrays" }
    "GroupMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F465); desc = "scalar-only view of group excluding nested arrays" }
    "SideMeta" = @{ emoji = [char]0x2194 + [char]0xFE0F; desc = "scalar-only view of side excluding nested arrays" }
    "ConnectionMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F517); desc = "scalar-only view of connection excluding nested arrays" }
    "DesignMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4D0); desc = "scalar-only view of design excluding nested arrays" }
    "KitMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4E6); desc = "scalar-only view of kit excluding nested arrays" }
    "BenchmarkMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CF); desc = "scalar-only view of benchmark excluding nested arrays" }
    "LocationMeta" = @{ emoji = [char]::ConvertFromUtf32(0x1F4CD); desc = "scalar-only view of location excluding nested arrays" }
}

# Merge all entity maps for lookup
$allDescMap = @{}
foreach ($key in $entityMap.Keys) { $allDescMap[$key] = $entityMap[$key] }
foreach ($key in $diffDescMap.Keys) { $allDescMap[$key] = $diffDescMap[$key] }
foreach ($key in $changeDescMap.Keys) { $allDescMap[$key] = $changeDescMap[$key] }
foreach ($key in $metaDescMap.Keys) { $allDescMap[$key] = $metaDescMap[$key] }

# Process line by line
$lines = $content -split "`n"
$newLines = [System.Collections.Generic.List[string]]::new($lines.Count)
$skipNext = $false

for ($i = 0; $i -lt $lines.Count; $i++) {
    $line = $lines[$i]
    
    if ($skipNext) {
        $skipNext = $false
        continue
    }
    
    # ============================================================
    # Fix 1: Summary doc comments with double emojis and generic descriptions
    # Pattern: /// EMOJI1<summary>EMOJI2EntityName holds the data fields for a EntityName record.</summary>
    # Also: /// <summary>EMOJIEntityName holds the data fields for a EntityName record.</summary>
    # ============================================================
    
    # Match summary lines - detect any emoji prefix before <summary>, and emoji inside
    if ($line -match '^\s*///\s*.{0,8}<summary>.+holds the data fields.+</summary>') {
        # Extract entity name from the pattern
        if ($line -match '<summary>.+?(\w+)\s+holds the data fields') {
            $entityName = $Matches[1]
            $indent = ""
            if ($line -match '^(\s*)') { $indent = $Matches[1] }
            
            $info = $null
            if ($allDescMap.ContainsKey($entityName)) {
                $info = $allDescMap[$entityName]
            }
            
            if ($info) {
                $newLine = "$indent/// <summary>$($info.emoji)$entityName represents $($info.desc).</summary>"
                $newLines.Add($newLine)
                $fixCount++
                
                # Check next line for duplicate <summary>
                if ($i + 1 -lt $lines.Count -and $lines[$i + 1] -match '^\s*///\s*.{0,8}<summary>.+holds the data fields') {
                    $skipNext = $true
                    $fixCount++
                }
                continue
            }
        }
    }
    
    # ============================================================
    # Fix 2: Default impl doc comments
    # Pattern: /// EMOJI<summary>EMOJIDefault holds the data fields for a Default record.</summary>
    # These are impl Default blocks - use the entity's emoji from context
    # ============================================================
    if ($line -match '^\s*///\s*.{0,8}<summary>.+Default\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        # Look ahead to find what entity this Default is for
        $entityFound = $null
        for ($j = $i + 1; $j -lt [Math]::Min($i + 5, $lines.Count); $j++) {
            if ($lines[$j] -match 'impl Default for (\w+)') {
                $entityFound = $Matches[1]
                break
            }
        }
        if ($entityFound -and $sectionEntityEmojis.ContainsKey($entityFound)) {
            $emoji = $sectionEntityEmojis[$entityFound]
            $newLine = "$indent/// <summary>${emoji}Default implementation for $entityFound.</summary>"
            $newLines.Add($newLine)
            $fixCount++
            continue
        }
    }

    # ============================================================
    # Fix 3: impl block doc comments (non-Default)
    # Pattern: /// EMOJI<summary>EMOJIEntityName holds the data fields for a EntityName record.</summary>
    # on impl blocks
    # ============================================================
    if ($line -match '^\s*///\s*.{0,8}<summary>.+?(\w+)\s+holds the data fields.+</summary>' -and $line -notmatch 'Default') {
        $entityName = $Matches[1]
        if ($sectionEntityEmojis.ContainsKey($entityName)) {
            $indent = ""
            if ($line -match '^(\s*)') { $indent = $Matches[1] }
            $emoji = $sectionEntityEmojis[$entityName]
            $newLine = "$indent/// <summary>${emoji}${entityName} represents $($entityMap[$entityName].desc).</summary>"
            # Already handled above, skip to avoid double
        }
    }

    # ============================================================  
    # Fix 4: Duplicate <summary> lines (not preceded by ///)
    # These are extra <summary> lines that need removal
    # ============================================================
    if ($line -match '^\s*///\s*<summary>.+holds the data fields.+</summary>' -and $lines[$i - 1] -match '^\s*///\s*<summary>.+holds the data fields') {
        $fixCount++
        continue  # Skip duplicate
    }

    # ============================================================
    # Fix 5: Random non-summary doc comments with emojis  
    # Pattern: /// 🔻<remarks>
    # ============================================================
    if ($line -match '^\s*///\s*[^\s/].{0,4}<remarks>') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <remarks>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 6: Various function doc comments (finder, serialization, etc.)
    # Pattern: /// 🔖<summary>🔖funcname holds the data fields for a funcname record.</summary>
    # ============================================================
    if ($line -match '^\s*///\s*.{0,8}<summary>.{0,8}(\w+)\s+holds the data fields for a \w+ record\.</summary>') {
        $funcName = $Matches[1]
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        
        # Check if this is already handled by entity map
        if (-not $allDescMap.ContainsKey($funcName)) {
            # Determine appropriate emoji and description for functions
            $funcEmoji = [char]::ConvertFromUtf32(0x1F527)  # 🔧 default for functions
            $funcDesc = $funcName -replace '_', ' '
            
            # Finder functions
            if ($funcName -match '^find_(\w+)_in_') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F50D)  # 🔍
                $funcDesc = "finds $($funcName -replace '_', ' ') by GUID"
            }
            # Serialization functions
            elseif ($funcName -match '^(serialize|deserialize)_') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4BE)  # 💾
                $target = ($funcName -split '_')[1]
                $funcDesc = "$($funcName -replace '_', ' ')"
            }
            # Apply diff functions
            elseif ($funcName -match '^apply_(\w+)_diff') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F504)  # 🔄
                $funcDesc = "applies a diff to update $($Matches[1])"
            }
            # Check/validate functions
            elseif ($funcName -match '^check_') {
                $funcEmoji = [char]0x2705  # ✅
                $funcDesc = "$($funcName -replace '_', ' ')"
            }
            # Equality functions
            elseif ($funcName -match '^are_(\w+)_equal') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F504)  # 🔄
                $funcDesc = "compares two $($Matches[1]) entities for deep equality"
            }
            # Import/export functions
            elseif ($funcName -match '^(import|export)_') {
                if ($funcName -match '^import') { $funcEmoji = [char]::ConvertFromUtf32(0x1F4E5) } # 📥
                else { $funcEmoji = [char]::ConvertFromUtf32(0x1F4E4) } # 📤
                $funcDesc = "$($funcName -replace '_', ' ')"
            }
            # Edit functions
            elseif ($funcName -match '^edit_') {
                $funcEmoji = [char]0x270F + [char]0xFE0F  # ✏️
                $funcDesc = "$($funcName -replace '_', ' ')"
            }
            # Hash functions
            elseif ($funcName -match '^hash_') {
                $target = ($funcName -split '_')[1]
                if ($sectionEntityEmojis.ContainsKey($target)) {
                    $funcEmoji = $sectionEntityEmojis[$target]
                } else {
                    $funcEmoji = [char]::ConvertFromUtf32(0x1F5DD) # 🗝
                }
                $funcDesc = "computes SHA-256 hash of $target"
            }
            # Utility functions
            elseif ($funcName -eq "guid") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F511)  # 🔑 
                $funcDesc = "generates a new v7 UUID string"
            }
            elseif ($funcName -eq "normalize") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4D0)  # 📐
                $funcDesc = "rounds a float to the given number of decimal places"
            }
            elseif ($funcName -eq "round") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4D0)  # 📐
                $funcDesc = "rounds a float to 3 decimal places"
            }
            elseif ($funcName -eq "jaccard") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4CA)  # 📊
                $funcDesc = "computes Jaccard similarity between two sets"
            }
            elseif ($funcName -eq "deep_equal") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F504)  # 🔄
                $funcDesc = "compares two serializable values for deep equality"
            }
            elseif ($funcName -eq "generate_unique_name") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4DD)  # 📝
                $funcDesc = "generates a unique name avoiding collisions with existing names"
            }
            elseif ($funcName -eq "flatten_design") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4D0)  # 📐
                $funcDesc = "flattens nested design references into a single design"
            }
            elseif ($funcName -eq "CollectionDiff") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F504)  # 🔄
                $funcDesc = "batched entity additions, removals and updates"
            }
            # Other generic
            elseif ($funcName -eq "SUPPORTED_MODEL_EXTENSIONS") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F5FF)  # 🗿
                $funcDesc = "the list of supported 3D model file extensions"
            }
            elseif ($funcName -eq "validate_kit") {
                $funcEmoji = [char]0x2705  # ✅
                $funcDesc = "validates a kit for structural and referential integrity"
            }
            elseif ($funcName -eq "sqlite") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4BE)  # 💾
                $funcDesc = "SQLite database import and export operations"
            }
            elseif ($funcName -eq "zip_roundtrip") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4E6)  # 📦
                $funcDesc = "ZIP archive round-trip import and export"
            }
            elseif ($funcName -eq "wasm") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F310)  # 🌐
                $funcDesc = "WebAssembly bindings for the semio library"
            }
            elseif ($funcName -eq "tests") {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F9EA)  # 🧪
                $funcDesc = "unit and integration tests for the semio library"
            }
            elseif ($funcName -eq "planes_equal_approx") {
                $funcEmoji = [char]0x25FB + [char]0xFE0F  # ◻️ 
                $funcDesc = "compares two planes for approximate equality"
            }
            elseif ($funcName -match '^compute_') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4BB)  # 💻
                $funcDesc = "$($funcName -replace '_', ' ')"
            }
            elseif ($funcName -match '^make_') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F527)  # 🔧
                $funcDesc = "$($funcName -replace '_', ' ')"
            }
            elseif ($funcName -match '^get_') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F50D)  # 🔍
                $funcDesc = "$($funcName -replace '_', ' ')"
            }
            elseif ($funcName -match '^quat_to_') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F504)  # 🔄
                $funcDesc = "converts quaternion to a 4x4 rotation matrix"
            }
            elseif ($funcName -match '^apply_matrix') {
                $funcEmoji = [char]::ConvertFromUtf32(0x1F4BB)  # 💻
                $funcDesc = "applies a 4x4 matrix to a 3D vector"
            }
            
            $newLine = "$indent/// <summary>$funcEmoji$funcDesc.</summary>"
            $newLines.Add($newLine)
            $fixCount++
            continue
        }
    }

    # ============================================================
    # Fix 7: Guid type doc comment
    # ============================================================
    if ($line -match '^\s*///\s*<summary>.+Guid\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F511) + "Guid represents a UUID string identifier.</summary>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 8: SemioError doc comment
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+SemioError\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <summary>" + [char]0x274C + "SemioError represents a domain error with context message.</summary>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 9: Result type doc comment
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+Result\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <summary>" + [char]0x2705 + "Result represents a success or SemioError outcome.</summary>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 10: RemovedItem doc comment
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+RemovedItem\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F5D1) + [char]0xFE0F + "RemovedItem represents an entity marked for removal by GUID.</summary>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 11: DiffUpdate doc comment
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+DiffUpdate\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F504) + "DiffUpdate represents a before-after pair for entity updates.</summary>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 12: Change doc comment
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+Change\s+holds the data fields for a Change record') {
        $funcNameMatch = [regex]::Match($line, '(\w+)\s+holds the data fields')
        $chgName = $funcNameMatch.Groups[1].Value
        if ($chgName -eq "Change") {
            $indent = ""
            if ($line -match '^(\s*)') { $indent = $Matches[1] }
            $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F504) + "Change represents a tracked modification with timestamp and author.</summary>")
            $fixCount++
            continue
        }
    }

    # ============================================================
    # Fix 13: ValidationProblem doc comment  
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+ValidationProblem\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <summary>" + [char]0x26A0 + [char]0xFE0F + "ValidationProblem represents a validation issue with severity and location.</summary>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 14: ValidationFix doc comment
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+ValidationFix\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F527) + "ValidationFix represents a suggested fix for a validation problem.</summary>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 15: ValidationResult doc comment
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+ValidationResult\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// <summary>" + [char]0x2705 + "ValidationResult represents the outcome of a kit validation.</summary>")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 16: HasGuid trait doc comments
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+HasGuid\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        # Look ahead to find what entity this is for
        $implEntity = $null
        for ($j = $i + 1; $j -lt [Math]::Min($i + 3, $lines.Count); $j++) {
            if ($lines[$j] -match 'impl HasGuid for (\w+)') {
                $implEntity = $Matches[1]
                break
            }
        }
        if ($implEntity -and $sectionEntityEmojis.ContainsKey($implEntity)) {
            $emoji = $sectionEntityEmojis[$implEntity]
            $newLines.Add("$indent/// <summary>${emoji}HasGuid implementation for $implEntity.</summary>")
        } else {
            $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F511) + "HasGuid provides GUID access for an entity.</summary>")
        }
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 17: DiffHasGuid trait doc comments
    # ============================================================
    if ($line -match '^\s*///\s*.+<summary>.+DiffHasGuid\s+holds the data fields') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $implEntity = $null
        for ($j = $i + 1; $j -lt [Math]::Min($i + 3, $lines.Count); $j++) {
            if ($lines[$j] -match 'impl DiffHasGuid for (\w+)') {
                $implEntity = $Matches[1]
                break
            }
        }
        if ($implEntity) {
            # Strip "Diff" suffix to get entity name
            $baseEntity = $implEntity -replace 'Diff$', ''
            if ($sectionEntityEmojis.ContainsKey($baseEntity)) {
                $emoji = $sectionEntityEmojis[$baseEntity]
                $newLines.Add("$indent/// <summary>${emoji}DiffHasGuid implementation for $implEntity.</summary>")
            } else {
                $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F511) + "DiffHasGuid implementation for $implEntity.</summary>")
            }
        } else {
            $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F511) + "DiffHasGuid provides GUID access for a diff entity.</summary>")
        }
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 18: Non-doc comment summary lines (on functions with <summary> tag)
    # Also: /// 🔖<summary>📖find_type_in_kit ....
    # from finder_functions section
    # ============================================================
    if ($line -match '^\s*///\s*<summary>.+holds the data fields for a') {
        $funcNameMatch = [regex]::Match($line, '<summary>.{0,8}(\w+)\s+holds the data fields')
        if ($funcNameMatch.Success) {
            $funcName = $funcNameMatch.Groups[1].Value
            $indent = ""
            if ($line -match '^(\s*)') { $indent = $Matches[1] }
            
            if ($allDescMap.ContainsKey($funcName)) {
                $info = $allDescMap[$funcName]
                $newLines.Add("$indent/// <summary>$($info.emoji)$funcName represents $($info.desc).</summary>")
            } else {
                # Already handled by fix 6 logic above, but just in case
                $newLines.Add($line)
            }
            $fixCount++
            continue
        }
    }

    # ============================================================
    # Fix 19: Lines with random emojis that aren't <summary> but are doc comments
    # like: /// 💘All valid KitKind values.
    # ============================================================
    if ($line -match '^\s*///\s*[^\s<].{0,4}All valid KitKind') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        $newLines.Add("$indent/// " + [char]::ConvertFromUtf32(0x1F4E6) + "All valid KitKind values.")
        $fixCount++
        continue
    }

    # ============================================================
    # Fix 20: Section markers with wrong emojis (both opening and closing)
    # e.g. // 📆Model Types - Attribute → // 💎Model Types - Attribute
    # ============================================================
    
    # Opening section marker: // EMOJIModel Types - Entity
    if ($line -match '^\s*//\s*.{1,4}Model Types - (\w+)' -and $line -notmatch '^\s*///') {
        $sectionEntity = $Matches[1]
        if ($sectionEntityEmojis.ContainsKey($sectionEntity)) {
            $emoji = $sectionEntityEmojis[$sectionEntity]
            $indent = ""
            if ($line -match '^(\s*)') { $indent = $Matches[1] }
            # Preserve the rest of the line after the entity name
            $rest = ""
            if ($line -match "Model Types - $sectionEntity(.*)$") {
                $rest = $Matches[1]
            }
            $newLines.Add("$indent// ${emoji}Model Types - $sectionEntity$rest")
            $fixCount++
            continue
        }
    }
    
    # Closing section marker: } // EMOJIModel Types - Entity
    if ($line -match '^\}\s*//\s*.{1,4}Model Types - (\w+)') {
        $sectionEntity = $Matches[1]
        if ($sectionEntityEmojis.ContainsKey($sectionEntity)) {
            $emoji = $sectionEntityEmojis[$sectionEntity]
            $rest = ""
            if ($line -match "Model Types - $sectionEntity(.*)$") {
                $rest = $Matches[1]
            }
            $newLines.Add("} // ${emoji}Model Types - $sectionEntity$rest")
            $fixCount++
            continue
        }
    }

    # ============================================================
    # Fix 21: Non-summary doc lines with random emoji prefixes in various sections
    # e.g. "/// 📚<summary>🔖Converts a nalgebra..."
    # These are narrative but wrong. Fix the leading emoji
    # ============================================================
    if ($line -match '^\s*///\s*.{1,4}<summary>.{0,8}(Converts|Preserves|Assigns|Selects)') {
        $indent = ""
        if ($line -match '^(\s*)') { $indent = $Matches[1] }
        # Extract content after <summary> removing inner emoji
        if ($line -match '<summary>.{0,8}((?:Converts|Preserves|Assigns|Selects).+)</summary>') {
            $desc = $Matches[1]
            $newLines.Add("$indent/// <summary>" + [char]::ConvertFromUtf32(0x1F527) + "$desc</summary>")
            $fixCount++
            continue
        }
    }

    # Default: keep line unchanged
    $newLines.Add($line)
}

# Join and write
$output = $newLines -join "`n"
[System.IO.File]::WriteAllText($file, $output, [System.Text.Encoding]::UTF8)

Write-Host "Fixed $fixCount doc comment issues in $file"
