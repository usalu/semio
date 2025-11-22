# Migration script to update Semio JSON files to new schema
# This script migrates old JSON format to match the TypeScript schema

param(
    [string]$Path = "assets\semio",
    [switch]$DryRun = $false
)

function New-Guid {
    return [System.Guid]::NewGuid().ToString()
}

function New-DeterministicGuid {
    param(
        [string]$Seed
    )
    
    # Replace special characters to handle backslash, slash, and other problematic chars
    $encodedSeed = $Seed -replace '\\', '_BACKSLASH_' -replace '/', '_SLASH_' -replace '\|', '_PIPE_'
    
    # Create deterministic GUID from input string using SHA256
    $sha256 = [System.Security.Cryptography.SHA256]::Create()
    $hashBytes = $sha256.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($encodedSeed))
    
    # Take first 16 bytes for GUID
    $guidBytes = $hashBytes[0..15]
    
    # Set version (4) and variant bits according to RFC 4122
    $guidBytes[6] = ($guidBytes[6] -band 0x0F) -bor 0x40  # Version 4
    $guidBytes[8] = ($guidBytes[8] -band 0x3F) -bor 0x80  # Variant 10
    
    # Convert byte array to GUID string format
    $guidString = "{0:x2}{1:x2}{2:x2}{3:x2}-{4:x2}{5:x2}-{6:x2}{7:x2}-{8:x2}{9:x2}-{10:x2}{11:x2}{12:x2}{13:x2}{14:x2}{15:x2}" -f @($guidBytes)
    
    return $guidString
}

function Get-CurrentTimestamp {
    return (Get-Date).ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
}

function Migrate-Attribute {
    param($attr)
    
    if ($null -eq $attr) { return $null }
    
    $migrated = @{
        guid = New-Guid
    }
    
    # Handle old 'name' field or new 'key' field
    if ($attr.PSObject.Properties.Name -contains 'key') {
        $migrated.key = $attr.key
    } elseif ($attr.PSObject.Properties.Name -contains 'name') {
        $migrated.key = $attr.name
    }
    
    if ($attr.PSObject.Properties.Name -contains 'value' -and $null -ne $attr.value) {
        $migrated.value = $attr.value
    }
    if ($attr.PSObject.Properties.Name -contains 'definition' -and $null -ne $attr.definition) {
        $migrated.definition = $attr.definition
    }
    
    return $migrated
}

function Migrate-Attributes {
    param($attributes)
    
    if ($null -eq $attributes) { return $null }
    
    # Handle both array and single object
    if ($attributes -is [array]) {
        if ($attributes.Count -eq 0) { return $null }
        $result = @($attributes | ForEach-Object { Migrate-Attribute $_ })
    } else {
        # Single object - convert to array
        $result = @(Migrate-Attribute $attributes)
    }
    
    # Ensure we return an actual array object that PowerShell won't collapse
    return ,$result
}

function Migrate-Point {
    param($point)
    
    if ($null -eq $point) { return $null }
    
    return @{
        x = $point.x
        y = $point.y
        z = $point.z
    }
}

function Migrate-Vector {
    param($vector)
    
    if ($null -eq $vector) { return $null }
    
    return @{
        x = $vector.x
        y = $vector.y
        z = $vector.z
    }
}

function Migrate-Plane {
    param($plane)
    
    if ($null -eq $plane) { return $null }
    
    return @{
        origin = Migrate-Point $plane.origin
        xAxis = Migrate-Vector $plane.xAxis
        yAxis = Migrate-Vector $plane.yAxis
    }
}

function Migrate-Location {
    param($location)
    
    if ($null -eq $location) { return $null }
    
    $migrated = @{
        guid = New-Guid
        longitude = $location.longitude
        latitude = $location.latitude
    }
    
    if ($location.PSObject.Properties.Name -contains 'altitude' -and $null -ne $location.altitude) {
        $migrated.altitude = $location.altitude
    }
    
    $attrs = Migrate-Attributes $location.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Prop {
    param($prop)
    
    if ($null -eq $prop) { return $null }
    
    $migrated = @{
        guid = New-Guid
        key = $prop.key
        value = $prop.value
    }
    
    if ($prop.PSObject.Properties.Name -contains 'unit' -and $null -ne $prop.unit) {
        $migrated.unit = $prop.unit
    }
    
    $attrs = Migrate-Attributes $prop.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Props {
    param($props)
    
    if ($null -eq $props -or $props.Count -eq 0) { return $null }
    
    return @($props | ForEach-Object { Migrate-Prop $_ })
}

function Migrate-Model {
    param($rep)
    
    if ($null -eq $rep) { return $null }
    
    $migrated = @{
        guid = New-Guid
        file = if ($rep.PSObject.Properties.Name -contains 'url') { $rep.url } else { $rep.file }
    }
    
    if ($rep.PSObject.Properties.Name -contains 'tags' -and $null -ne $rep.tags -and $rep.tags.Count -gt 0) {
        $migrated.tags = $rep.tags
    }
    if ($rep.PSObject.Properties.Name -contains 'description' -and $null -ne $rep.description) {
        $migrated.description = $rep.description
    }
    
    $attrs = Migrate-Attributes $rep.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Models {
    param($models)
    
    if ($null -eq $models -or $models.Count -eq 0) { return $null }
    
    return @($models | ForEach-Object { Migrate-Model $_ })
}

function Migrate-Port {
    param($port)
    
    if ($null -eq $port) { return $null }
    
    $migrated = @{
        guid = if ($port.PSObject.Properties.Name -contains 'guid') { $port.guid } else { New-Guid }
        t = $port.t
        point = Migrate-Point $port.point
        direction = Migrate-Vector $port.direction
    }
    
    # Name field - migrate from id_ or id
    if ($port.PSObject.Properties.Name -contains 'name' -and $null -ne $port.name -and $port.name -ne '') {
        $migrated.name = $port.name
    } elseif ($port.PSObject.Properties.Name -contains 'id_' -and $null -ne $port.id_ -and $port.id_ -ne '') {
        $migrated.name = $port.id_
    } elseif ($port.PSObject.Properties.Name -contains 'id' -and $null -ne $port.id -and $port.id -ne '') {
        $migrated.name = $port.id
    }
    
    if ($port.PSObject.Properties.Name -contains 'description' -and $null -ne $port.description) {
        $migrated.description = $port.description
    }
    if ($port.PSObject.Properties.Name -contains 'interface' -and $null -ne $port.interface) {
        $migrated.interface = $port.interface
    }
    if ($port.PSObject.Properties.Name -contains 'mandatory' -and $null -ne $port.mandatory) {
        $migrated.mandatory = $port.mandatory
    }
    if ($port.PSObject.Properties.Name -contains 'compatibleInterfaces' -and $null -ne $port.compatibleInterfaces -and $port.compatibleInterfaces.Count -gt 0) {
        $migrated.compatibleInterfaces = $port.compatibleInterfaces
    }
    
    $props = Migrate-Props $port.props
    if ($null -ne $props) {
        $migrated.props = $props
    }
    
    $attrs = Migrate-Attributes $port.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Ports {
    param($ports)
    
    if ($null -eq $ports -or $ports.Count -eq 0) { return $null }
    
    return @($ports | ForEach-Object { Migrate-Port $_ })
}

function Migrate-Type {
    param($type, $typeNameToGuidMap, $authorEmailToObjectMap, $portNameToDataMap = $null)
    
    if ($null -eq $type) { return $null }
    
    $timestamp = Get-CurrentTimestamp
    
    # Determine the actual name (variant becomes the name if present)
    $actualName = $type.name
    $variant = ''
    if ($type.PSObject.Properties.Name -contains 'variant' -and $null -ne $type.variant -and $type.variant -ne '') {
        $actualName = $type.variant
        $variant = $type.variant
    }
    
    # Determine parent for unique key
    $parentValue = $null
    if ($type.PSObject.Properties.Name -contains 'variant' -and $null -ne $type.variant -and $type.variant -ne '') {
        # Map metabolism types to new hierarchy
        switch ($type.name) {
            "Capsule" { $parentValue = "Box" }
            "Ellipsoid Capsule" { $parentValue = "Ellipsoid" }
            "Trapezoid Capsule" { $parentValue = "Trapezoid" }
            "Capsule with Balcony" { $parentValue = "Balcony" }
            "Tambour" { $parentValue = "Tambour" }
            "Cylindric Tambour" { $parentValue = "Cylindric Tambour" }
            default { $parentValue = $type.name }
        }
    } elseif ($type.PSObject.Properties.Name -contains 'parent' -and $null -ne $type.parent) {
        $parentValue = $type.parent
    }
    $parentKey = if ($null -ne $parentValue) { $parentValue } else { '' }
    
    # Create unique key: name|variant|parent
    $uniqueKey = "$($type.name)|$variant|$parentKey"
    
    # Always generate deterministic GUID to fix duplicate GUID issues
    $typeGuid = New-DeterministicGuid -Seed "type:$uniqueKey"
    
    # Store mapping with unique key (for pieces to reference)
    if ($null -ne $typeNameToGuidMap) {
        if (-not $typeNameToGuidMap.ContainsKey($uniqueKey)) {
            $typeNameToGuidMap[$uniqueKey] = $typeGuid
        }
        # Also store with name|variant key for piece lookups
        $pieceKey = "$($type.name)|$variant"
        if (-not $typeNameToGuidMap.ContainsKey($pieceKey)) {
            $typeNameToGuidMap[$pieceKey] = $typeGuid
        }
        # Also store with just name for simple lookups
        if (-not $typeNameToGuidMap.ContainsKey($type.name)) {
            $typeNameToGuidMap[$type.name] = $typeGuid
        }
    }
    
    $migrated = @{
        guid = $typeGuid
        name = $actualName
        createdAt = $timestamp
        updatedAt = $timestamp
    }
    
    # Optional fields
    if ($null -ne $parentValue) {
        # Parent will be resolved in third pass (name to GUID)
        $migrated.parent = $parentValue
    }
    if ($type.PSObject.Properties.Name -contains 'isAbstract' -and $null -ne $type.isAbstract) {
        $migrated.isAbstract = $type.isAbstract
    }
    if ($type.PSObject.Properties.Name -contains 'folder' -and $null -ne $type.folder) {
        $migrated.folder = $type.folder
    }
    if ($type.PSObject.Properties.Name -contains 'stock' -and $null -ne $type.stock) {
        $migrated.stock = $type.stock
    }
    if ($type.PSObject.Properties.Name -contains 'virtual' -and $null -ne $type.virtual) {
        $migrated.virtual = $type.virtual
    }
    if ($type.PSObject.Properties.Name -contains 'unit' -and $null -ne $type.unit) {
        $migrated.unit = $type.unit
    }
    if ($type.PSObject.Properties.Name -contains 'icon' -and $null -ne $type.icon) {
        $migrated.icon = $type.icon
    }
    if ($type.PSObject.Properties.Name -contains 'image' -and $null -ne $type.image) {
        $migrated.image = $type.image
    }
    if ($type.PSObject.Properties.Name -contains 'description' -and $null -ne $type.description) {
        $migrated.description = $type.description
    }
    if ($type.PSObject.Properties.Name -contains 'authors' -and $null -ne $type.authors -and $type.authors.Count -gt 0) {
        # Authors might be strings (references) or objects (inline definitions)
        if ($type.authors[0] -is [string]) {
            # Already references, keep as-is (TODO: might need to convert to { guid } format)
            $migrated.authors = $type.authors
        } else {
            # Inline author objects - collect unique authors and convert to ID references
            $migratedAuthors = @($type.authors | ForEach-Object { 
                $migratedAuthor = Migrate-Author $_
                # Store in map by email for deduplication
                if ($null -ne $authorEmailToObjectMap -and $null -ne $migratedAuthor.email) {
                    if (-not $authorEmailToObjectMap.ContainsKey($migratedAuthor.email)) {
                        $authorEmailToObjectMap[$migratedAuthor.email] = $migratedAuthor
                    }
                }
                $migratedAuthor
            })
            $migrated.authors = @($migratedAuthors | ForEach-Object { @{ guid = $_.guid } })
        }
    }
    if ($type.PSObject.Properties.Name -contains 'concepts' -and $null -ne $type.concepts -and $type.concepts.Count -gt 0) {
        $migrated.concepts = $type.concepts
    }
    
    $reps = Migrate-Models $type.models
    if ($null -ne $reps) {
        $migrated.models = $reps
    }
    
    # Migrate ports, preserving GUIDs and names from kit if available
    if ($type.PSObject.Properties.Name -contains 'ports' -and $null -ne $type.ports) {
        $migratedPorts = @()
        foreach ($port in $type.ports) {
            $migratedPort = Migrate-Port $port
            
            # If we have port data from kit, use it to ensure consistency
            if ($null -ne $portNameToDataMap) {
                $portName = $migratedPort.name
                if ($null -ne $portName -and $portNameToDataMap.ContainsKey($portName)) {
                    $kitPortData = $portNameToDataMap[$portName]
                    $migratedPort.guid = $kitPortData.guid
                    $migratedPort.name = $kitPortData.name
                }
            }
            
            $migratedPorts += $migratedPort
        }
        if ($migratedPorts.Count -gt 0) {
            $migrated.ports = $migratedPorts
        }
    }
    
    $props = Migrate-Props $type.props
    if ($null -ne $props) {
        $migrated.props = $props
    }
    
    # Convert location to ID reference
    if ($type.PSObject.Properties.Name -contains 'location' -and $null -ne $type.location) {
        $migratedLocation = Migrate-Location $type.location
        if ($null -ne $migratedLocation) {
            $migrated.location = @{ guid = $migratedLocation.guid }
        }
    }
    
    $attrs = Migrate-Attributes $type.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Piece {
    param($piece, $typeNameToGuidMap, $designNameToGuidMap, $pieceNameToGuidMap = $null)
    
    if ($null -eq $piece) { return $null }
    
    # Determine piece name first
    $pieceName = $null
    if ($piece.PSObject.Properties.Name -contains 'name' -and $null -ne $piece.name -and $piece.name -ne '') {
        $pieceName = $piece.name
    } elseif ($piece.PSObject.Properties.Name -contains 'id_' -and $null -ne $piece.id_ -and $piece.id_ -ne '') {
        $pieceName = $piece.id_
    } elseif ($piece.PSObject.Properties.Name -contains 'id' -and $null -ne $piece.id -and $piece.id -ne '') {
        $pieceName = $piece.id
    }
    
    # Generate GUID: use mapped GUID if available (for flat designs), otherwise existing or new
    $pieceGuid = $null
    if ($null -ne $pieceNameToGuidMap -and $null -ne $pieceName -and $pieceNameToGuidMap.ContainsKey($pieceName)) {
        $pieceGuid = $pieceNameToGuidMap[$pieceName]
    } elseif ($piece.PSObject.Properties.Name -contains 'guid') {
        $pieceGuid = $piece.guid
    } else {
        $pieceGuid = New-Guid
    }
    
    $migrated = @{
        guid = $pieceGuid
    }
    
    # Name field
    if ($null -ne $pieceName) {
        $migrated.name = $pieceName
    }
    
    # Type reference - convert name to guid reference
    if ($piece.PSObject.Properties.Name -contains 'type' -and $null -ne $piece.type) {
        $typeName = $null
        $typeVariant = ''
        if ($piece.type -is [string]) {
            $typeName = $piece.type
        } else {
            # Old format with name/variant object
            $typeName = $piece.type.name
            if ($piece.type.PSObject.Properties.Name -contains 'variant' -and $null -ne $piece.type.variant) {
                $typeVariant = $piece.type.variant
            }
        }
        
        if ($null -ne $typeName) {
            # Try name+variant first, then fall back to name only
            $mapKey = "$typeName|$typeVariant"
            if ($typeNameToGuidMap.ContainsKey($mapKey)) {
                $migrated.type = @{ guid = $typeNameToGuidMap[$mapKey] }
            } elseif ($typeNameToGuidMap.ContainsKey($typeName)) {
                $migrated.type = @{ guid = $typeNameToGuidMap[$typeName] }
            } else {
                Write-Warning "  [PIECE] Type '$typeName' (variant: '$typeVariant') not found in map with $($typeNameToGuidMap.Keys.Count) keys"
            }
        }
    }
    
    # Design reference - convert name to guid reference
    if ($piece.PSObject.Properties.Name -contains 'design' -and $null -ne $piece.design) {
        $designName = $null
        $designVariant = ''
        $designView = ''
        if ($piece.design -is [string]) {
            $designName = $piece.design
        } else {
            # Old format with name/variant/view object
            $designName = $piece.design.name
            if ($piece.design.PSObject.Properties.Name -contains 'variant' -and $null -ne $piece.design.variant) {
                $designVariant = $piece.design.variant
            }
            if ($piece.design.PSObject.Properties.Name -contains 'view' -and $null -ne $piece.design.view) {
                $designView = $piece.design.view
            }
        }
        
        if ($null -ne $designName) {
            # Try name+variant+view first, then fall back to name only
            $mapKey = "$designName|$designVariant|$designView"
            if ($designNameToGuidMap.ContainsKey($mapKey)) {
                $migrated.design = @{ guid = $designNameToGuidMap[$mapKey] }
            } elseif ($designNameToGuidMap.ContainsKey($designName)) {
                $migrated.design = @{ guid = $designNameToGuidMap[$designName] }
            }
        }
    }
    
    # Optional fields
    if ($piece.PSObject.Properties.Name -contains 'plane' -and $null -ne $piece.plane) {
        $migrated.plane = Migrate-Plane $piece.plane
    }
    if ($piece.PSObject.Properties.Name -contains 'center' -and $null -ne $piece.center) {
        $migrated.center = @{
            u = if ($piece.center.PSObject.Properties.Name -contains 'u') { $piece.center.u } else { $piece.center.x }
            v = if ($piece.center.PSObject.Properties.Name -contains 'v') { $piece.center.v } else { $piece.center.y }
        }
    }
    if ($piece.PSObject.Properties.Name -contains 'scale' -and $null -ne $piece.scale) {
        $migrated.scale = $piece.scale
    }
    if ($piece.PSObject.Properties.Name -contains 'mirrorPlane' -and $null -ne $piece.mirrorPlane) {
        $migrated.mirrorPlane = Migrate-Plane $piece.mirrorPlane
    }
    if ($piece.PSObject.Properties.Name -contains 'isHidden' -and $null -ne $piece.isHidden) {
        $migrated.isHidden = $piece.isHidden
    } elseif ($piece.PSObject.Properties.Name -contains 'hidden' -and $null -ne $piece.hidden) {
        $migrated.isHidden = $piece.hidden
    }
    if ($piece.PSObject.Properties.Name -contains 'isLocked' -and $null -ne $piece.isLocked) {
        $migrated.isLocked = $piece.isLocked
    } elseif ($piece.PSObject.Properties.Name -contains 'locked' -and $null -ne $piece.locked) {
        $migrated.isLocked = $piece.locked
    }
    if ($piece.PSObject.Properties.Name -contains 'color' -and $null -ne $piece.color) {
        $migrated.color = $piece.color
    }
    if ($piece.PSObject.Properties.Name -contains 'description' -and $null -ne $piece.description) {
        $migrated.description = $piece.description
    }
    
    $props = Migrate-Props $piece.props
    if ($null -ne $props) {
        $migrated.props = $props
    }
    
    $attrs = Migrate-Attributes $piece.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Pieces {
    param($pieces, $typeNameToGuidMap, $designNameToGuidMap, $pieceNameToGuidMap = $null)
    
    if ($null -eq $pieces -or $pieces.Count -eq 0) { return $null }
    
    return @($pieces | ForEach-Object { Migrate-Piece $_ $typeNameToGuidMap $designNameToGuidMap $pieceNameToGuidMap })
}

function Find-PortOnType {
    param($type, $portId, $kitTypes)
    
    if ($null -eq $type) { return $null }
    
    $visited = @{}
    $currentType = $type
    
    while ($null -ne $currentType -and -not $visited.ContainsKey($currentType.guid)) {
        $visited[$currentType.guid] = $true
        
        if ($null -ne $currentType.ports -and $currentType.ports.Count -gt 0) {
            if ([string]::IsNullOrEmpty($portId)) {
                return $currentType.ports[0]
            }
            
            $port = $currentType.ports | Where-Object { 
                ($_.PSObject.Properties.Name -contains 'name' -and $_.name -eq $portId) -or
                ($_.PSObject.Properties.Name -contains 'id_' -and $_.id_ -eq $portId) -or
                ($_.PSObject.Properties.Name -contains 'id' -and $_.id -eq $portId)
            } | Select-Object -First 1
            
            if ($null -ne $port) {
                return $port
            }
        }
        
        if ($null -ne $currentType.parent) {
            $parentGuid = if ($currentType.parent -is [string]) { $currentType.parent } else { $currentType.parent.guid }
            $currentType = $kitTypes | Where-Object { $_.guid -eq $parentGuid } | Select-Object -First 1
        } else {
            $currentType = $null
        }
    }
    
    return $null
}

function Migrate-Connection {
    param($conn, $pieceIdToGuidMap, $portIdToGuidMap, $pieces = $null, $kitTypes = $null)
    
    if ($null -eq $conn) { return $null }
    
    $migrated = @{
        guid = if ($conn.PSObject.Properties.Name -contains 'guid') { $conn.guid } else { New-Guid }
    }
    
    # Connected side
    if ($conn.PSObject.Properties.Name -contains 'connected' -and $null -ne $conn.connected) {
        $connectedSide = @{}
        
        if ($conn.connected -is [string]) {
            # Simple string reference to piece
            if ($pieceIdToGuidMap.ContainsKey($conn.connected)) {
                $connectedSide.piece = @{ guid = $pieceIdToGuidMap[$conn.connected] }
            }
        } else {
            # Object with piece and optionally port/designPiece
            if ($conn.connected.PSObject.Properties.Name -contains 'piece' -and $null -ne $conn.connected.piece) {
                $pieceId = $null
                if ($conn.connected.piece -is [string]) {
                    $pieceId = $conn.connected.piece
                } elseif ($conn.connected.piece.PSObject.Properties.Name -contains 'guid') {
                    # Already migrated with guid
                    $connectedSide.piece = @{ guid = $conn.connected.piece.guid }
                } elseif ($conn.connected.piece.PSObject.Properties.Name -contains 'id_') {
                    $pieceId = $conn.connected.piece.id_
                } elseif ($conn.connected.piece.PSObject.Properties.Name -contains 'id') {
                    $pieceId = $conn.connected.piece.id
                }
                if ($null -ne $pieceId) {
                    if ($pieceIdToGuidMap.ContainsKey($pieceId)) {
                        $connectedSide.piece = @{ guid = $pieceIdToGuidMap[$pieceId] }
                    } else {
                        Write-Warning "  [CONNECTION] Piece ID '$pieceId' not found in map (connected side)"
                    }
                }
            }
            $portId = $null
            $portAlreadySet = $false
            if ($conn.connected.PSObject.Properties.Name -contains 'port' -and $null -ne $conn.connected.port) {
                if ($conn.connected.port -is [string]) {
                    $portId = $conn.connected.port
                } elseif ($conn.connected.port.PSObject.Properties.Name -contains 'guid') {
                    $connectedSide.port = @{ guid = $conn.connected.port.guid }
                    $portAlreadySet = $true
                } elseif ($conn.connected.port.PSObject.Properties.Name -contains 'id_') {
                    $portId = $conn.connected.port.id_
                } elseif ($conn.connected.port.PSObject.Properties.Name -contains 'id') {
                    $portId = $conn.connected.port.id
                }
            }
            
            if (-not $portAlreadySet -and $null -ne $pieces -and $null -ne $kitTypes -and $null -ne $connectedSide.piece) {
                $pieceGuid = $connectedSide.piece.guid
                $piece = $pieces | Where-Object { $_.guid -eq $pieceGuid } | Select-Object -First 1
                if ($null -ne $piece -and $null -ne $piece.type) {
                    $currentType = $kitTypes | Where-Object { $_.guid -eq $piece.type.guid } | Select-Object -First 1
                    $port = Find-PortOnType $currentType $portId $kitTypes
                    if ($null -ne $port) {
                        $connectedSide.port = @{ guid = $port.guid }
                    } else {
                        Write-Warning "  [CONNECTION] Port '$portId' not found on type hierarchy (connected side, piece: $pieceGuid)"
                    }
                }
            } elseif (-not $portAlreadySet -and $null -ne $portId -and $portIdToGuidMap.ContainsKey($portId)) {
                $connectedSide.port = @{ guid = $portIdToGuidMap[$portId] }
            } elseif (-not $portAlreadySet -and $null -ne $portId) {
                Write-Warning "  [CONNECTION] Port ID '$portId' not found in map (connected side)"
            }
            if ($conn.connected.PSObject.Properties.Name -contains 'designPiece' -and $null -ne $conn.connected.designPiece) {
                $designPieceId = $null
                if ($conn.connected.designPiece -is [string]) {
                    $designPieceId = $conn.connected.designPiece
                } elseif ($conn.connected.designPiece.PSObject.Properties.Name -contains 'id_') {
                    $designPieceId = $conn.connected.designPiece.id_
                } elseif ($conn.connected.designPiece.PSObject.Properties.Name -contains 'id') {
                    $designPieceId = $conn.connected.designPiece.id
                }
                if ($null -ne $designPieceId -and $pieceIdToGuidMap.ContainsKey($designPieceId)) {
                    $connectedSide.designPiece = @{ guid = $pieceIdToGuidMap[$designPieceId] }
                }
            }
        }
        
        $migrated.connected = $connectedSide
    }
    
    # Connecting side
    if ($conn.PSObject.Properties.Name -contains 'connecting' -and $null -ne $conn.connecting) {
        $connectingSide = @{}
        
        if ($conn.connecting -is [string]) {
            # Simple string reference to piece
            if ($pieceIdToGuidMap.ContainsKey($conn.connecting)) {
                $connectingSide.piece = @{ guid = $pieceIdToGuidMap[$conn.connecting] }
            }
        } else {
            # Object with piece and optionally port/designPiece
            if ($conn.connecting.PSObject.Properties.Name -contains 'piece' -and $null -ne $conn.connecting.piece) {
                $pieceId = $null
                if ($conn.connecting.piece -is [string]) {
                    $pieceId = $conn.connecting.piece
                } elseif ($conn.connecting.piece.PSObject.Properties.Name -contains 'guid') {
                    # Already migrated with guid
                    $connectingSide.piece = @{ guid = $conn.connecting.piece.guid }
                } elseif ($conn.connecting.piece.PSObject.Properties.Name -contains 'id_') {
                    $pieceId = $conn.connecting.piece.id_
                } elseif ($conn.connecting.piece.PSObject.Properties.Name -contains 'id') {
                    $pieceId = $conn.connecting.piece.id
                }
                if ($null -ne $pieceId) {
                    if ($pieceIdToGuidMap.ContainsKey($pieceId)) {
                        $connectingSide.piece = @{ guid = $pieceIdToGuidMap[$pieceId] }
                    }
                }
            }
            $portId = $null
            $portAlreadySet = $false
            if ($conn.connecting.PSObject.Properties.Name -contains 'port' -and $null -ne $conn.connecting.port) {
                if ($conn.connecting.port -is [string]) {
                    $portId = $conn.connecting.port
                } elseif ($conn.connecting.port.PSObject.Properties.Name -contains 'guid') {
                    $connectingSide.port = @{ guid = $conn.connecting.port.guid }
                    $portAlreadySet = $true
                } elseif ($conn.connecting.port.PSObject.Properties.Name -contains 'id_') {
                    $portId = $conn.connecting.port.id_
                } elseif ($conn.connecting.port.PSObject.Properties.Name -contains 'id') {
                    $portId = $conn.connecting.port.id
                }
            }
            
            if (-not $portAlreadySet -and $null -ne $pieces -and $null -ne $kitTypes -and $null -ne $connectingSide.piece) {
                $pieceGuid = $connectingSide.piece.guid
                $piece = $pieces | Where-Object { $_.guid -eq $pieceGuid } | Select-Object -First 1
                if ($null -ne $piece -and $null -ne $piece.type) {
                    $currentType = $kitTypes | Where-Object { $_.guid -eq $piece.type.guid } | Select-Object -First 1
                    $port = Find-PortOnType $currentType $portId $kitTypes
                    if ($null -ne $port) {
                        $connectingSide.port = @{ guid = $port.guid }
                    } else {
                        Write-Warning "  [CONNECTION] Port '$portId' not found on type hierarchy (connecting side, piece: $pieceGuid)"
                    }
                }
            } elseif (-not $portAlreadySet -and $null -ne $portId -and $portIdToGuidMap.ContainsKey($portId)) {
                $connectingSide.port = @{ guid = $portIdToGuidMap[$portId] }
            } elseif (-not $portAlreadySet -and $null -ne $portId) {
                Write-Warning "  [CONNECTION] Port ID '$portId' not found in map (connecting side)"
            }
            if ($conn.connecting.PSObject.Properties.Name -contains 'designPiece' -and $null -ne $conn.connecting.designPiece) {
                $designPieceId = $null
                if ($conn.connecting.designPiece -is [string]) {
                    $designPieceId = $conn.connecting.designPiece
                } elseif ($conn.connecting.designPiece.PSObject.Properties.Name -contains 'id_') {
                    $designPieceId = $conn.connecting.designPiece.id_
                } elseif ($conn.connecting.designPiece.PSObject.Properties.Name -contains 'id') {
                    $designPieceId = $conn.connecting.designPiece.id
                }
                if ($null -ne $designPieceId -and $pieceIdToGuidMap.ContainsKey($designPieceId)) {
                    $connectingSide.designPiece = @{ guid = $pieceIdToGuidMap[$designPieceId] }
                }
            }
        }
        
        $migrated.connecting = $connectingSide
    }
    
    # Translation/rotation parameters
    if ($conn.PSObject.Properties.Name -contains 'gap' -and $null -ne $conn.gap) {
        $migrated.gap = $conn.gap
    }
    if ($conn.PSObject.Properties.Name -contains 'shift' -and $null -ne $conn.shift) {
        $migrated.shift = $conn.shift
    }
    if ($conn.PSObject.Properties.Name -contains 'rise' -and $null -ne $conn.rise) {
        $migrated.rise = $conn.rise
    }
    if ($conn.PSObject.Properties.Name -contains 'rotation' -and $null -ne $conn.rotation) {
        $migrated.rotation = $conn.rotation
    }
    if ($conn.PSObject.Properties.Name -contains 'turn' -and $null -ne $conn.turn) {
        $migrated.turn = $conn.turn
    }
    if ($conn.PSObject.Properties.Name -contains 'tilt' -and $null -ne $conn.tilt) {
        $migrated.tilt = $conn.tilt
    }
    
    # Diagram positioning
    if ($conn.PSObject.Properties.Name -contains 'x' -and $null -ne $conn.x) {
        $migrated.x = $conn.x
    }
    if ($conn.PSObject.Properties.Name -contains 'y' -and $null -ne $conn.y) {
        $migrated.y = $conn.y
    }
    
    if ($conn.PSObject.Properties.Name -contains 'description' -and $null -ne $conn.description) {
        $migrated.description = $conn.description
    }
    
    $attrs = Migrate-Attributes $conn.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Connections {
    param($connections, $pieceIdToGuidMap, $portIdToGuidMap, $pieces = $null, $kitTypes = $null)
    
    if ($null -eq $connections -or $connections.Count -eq 0) { return $null }
    
    return @($connections | ForEach-Object { Migrate-Connection $_ $pieceIdToGuidMap $portIdToGuidMap $pieces $kitTypes })
}

function Migrate-Design {
    param($design, $designNameToGuidMap, $typeNameToGuidMap, $portIdToGuidMap, $authorEmailToObjectMap, $pieceNameToGuidMap = $null, $kitTypes = $null)
    
    if ($null -eq $design) { return $null }
    
    $timestamp = Get-CurrentTimestamp
    
    # Determine the actual name (variant becomes the name if present, view is ignored as it's deprecated)
    $actualName = $design.name
    $variant = ''
    $view = ''
    if ($design.PSObject.Properties.Name -contains 'variant' -and $null -ne $design.variant -and $design.variant -ne '') {
        $actualName = $design.variant
        $variant = $design.variant
    }
    if ($design.PSObject.Properties.Name -contains 'view' -and $null -ne $design.view -and $design.view -ne '') {
        $view = $design.view
    }
    
    # Create unique key: name|variant|view
    $uniqueKey = "$($design.name)|$variant|$view"
    
    # Always generate deterministic GUID to fix duplicate GUID issues
    $designGuid = New-DeterministicGuid -Seed "design:$uniqueKey"
    
    # Store mapping with unique key (for pieces to reference)
    if ($null -ne $designNameToGuidMap -and -not $designNameToGuidMap.ContainsKey($uniqueKey)) {
        $designNameToGuidMap[$uniqueKey] = $designGuid
    }
    
    $migrated = @{
        guid = $designGuid
        name = $actualName
        createdAt = $timestamp
        updatedAt = $timestamp
    }
    
    # Store mapping for later reference resolution
    # Store actualName for parent lookups and backwards compatibility
    if ($null -ne $designNameToGuidMap) {
        # Store actualName for parent lookups and standalone designs
        if (-not $designNameToGuidMap.ContainsKey($actualName)) {
            $designNameToGuidMap[$actualName] = $designGuid
        }
    }
    
    # NOTE: "view" field is deprecated and not migrated to output
    
    # Optional fields
    # If has variant or view, parent is the base design name; otherwise check for explicit parent
    $parentValue = $null
    if ($design.PSObject.Properties.Name -contains 'variant' -and $null -ne $design.variant -and $design.variant -ne '') {
        # Variant becomes child, parent is the base name
        $parentValue = $design.name
    } elseif ($design.PSObject.Properties.Name -contains 'view' -and $null -ne $design.view -and $design.view -ne '') {
        # View becomes child, parent might be variant or base name
        if ($design.PSObject.Properties.Name -contains 'variant' -and $null -ne $design.variant -and $design.variant -ne '') {
            $parentValue = $design.variant
        } else {
            $parentValue = $design.name
        }
    } elseif ($design.PSObject.Properties.Name -contains 'parent' -and $null -ne $design.parent) {
        $parentValue = $design.parent
    }
    if ($null -ne $parentValue) {
        # Parent will be resolved in third pass (name to GUID)
        $migrated.parent = $parentValue
    }
    if ($design.PSObject.Properties.Name -contains 'isAbstract' -and $null -ne $design.isAbstract) {
        $migrated.isAbstract = $design.isAbstract
    }
    if ($design.PSObject.Properties.Name -contains 'folder' -and $null -ne $design.folder) {
        $migrated.folder = $design.folder
    }
    # NOTE: "view" field is explicitly NOT migrated (deprecated)
    if ($design.PSObject.Properties.Name -contains 'unit' -and $null -ne $design.unit) {
        $migrated.unit = $design.unit
    }
    if ($design.PSObject.Properties.Name -contains 'icon' -and $null -ne $design.icon) {
        $migrated.icon = $design.icon
    }
    if ($design.PSObject.Properties.Name -contains 'image' -and $null -ne $design.image) {
        $migrated.image = $design.image
    }
    if ($design.PSObject.Properties.Name -contains 'description' -and $null -ne $design.description) {
        $migrated.description = $design.description
    }
    if ($design.PSObject.Properties.Name -contains 'authors' -and $null -ne $design.authors -and $design.authors.Count -gt 0) {
        # Authors might be strings (references) or objects (inline definitions)
        if ($design.authors[0] -is [string]) {
            # Already references, keep as-is (TODO: might need to convert to { guid } format)
            $migrated.authors = $design.authors
        } else {
            # Inline author objects - collect unique authors and convert to ID references
            $migratedAuthors = @($design.authors | ForEach-Object { 
                $migratedAuthor = Migrate-Author $_
                # Store in map by email for deduplication
                if ($null -ne $authorEmailToObjectMap -and $null -ne $migratedAuthor.email) {
                    if (-not $authorEmailToObjectMap.ContainsKey($migratedAuthor.email)) {
                        $authorEmailToObjectMap[$migratedAuthor.email] = $migratedAuthor
                    }
                }
                $migratedAuthor
            })
            $migrated.authors = @($migratedAuthors | ForEach-Object { @{ guid = $_.guid } })
        }
    }
    if ($design.PSObject.Properties.Name -contains 'concepts' -and $null -ne $design.concepts -and $design.concepts.Count -gt 0) {
        $migrated.concepts = $design.concepts
    }
    
    # First pass: migrate pieces and build piece ID to GUID map
    $pieceIdToGuidMap = @{}
    $pieces = Migrate-Pieces $design.pieces $typeNameToGuidMap $designNameToGuidMap $pieceNameToGuidMap
    if ($null -ne $pieces) {
        $migrated.pieces = $pieces
        # Build mapping from old piece IDs to new GUIDs
        for ($i = 0; $i -lt $design.pieces.Count; $i++) {
            $oldId = if ($design.pieces[$i].PSObject.Properties.Name -contains 'id_') { $design.pieces[$i].id_ } else { $design.pieces[$i].id }
            if ($null -ne $oldId) {
                $pieceIdToGuidMap[$oldId] = $pieces[$i].guid
            }
        }
    }
    
    # Second pass: migrate connections using the piece ID map and kit types for port lookup
    $connections = Migrate-Connections $design.connections $pieceIdToGuidMap $portIdToGuidMap $pieces $kitTypes
    if ($null -ne $connections -and $connections.Count -gt 0) {
        $migrated.connections = $connections
    }
    
    $props = Migrate-Props $design.props
    if ($null -ne $props) {
        $migrated.props = $props
    }
    
    # Convert location to ID reference
    if ($design.PSObject.Properties.Name -contains 'location' -and $null -ne $design.location) {
        $migratedLocation = Migrate-Location $design.location
        if ($null -ne $migratedLocation) {
            $migrated.location = @{ guid = $migratedLocation.guid }
        }
    }
    
    $attrs = Migrate-Attributes $design.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    # TODO: stats, layers, activeLayer, groups, canScale, canMirror
    
    return $migrated
}

function Migrate-File {
    param($file, $kitGuid)
    
    if ($null -eq $file) { return $null }
    
    $timestamp = Get-CurrentTimestamp
    
    $migrated = @{
        guid = New-Guid
        name = $file.name
        createdAt = $timestamp
        updatedAt = $timestamp
    }
    
    if ($file.PSObject.Properties.Name -contains 'remote' -and $null -ne $file.remote) {
        $migrated.remote = $file.remote
    }
    if ($file.PSObject.Properties.Name -contains 'folder' -and $null -ne $file.folder) {
        $migrated.folder = $file.folder
    }
    if ($file.PSObject.Properties.Name -contains 'size' -and $null -ne $file.size) {
        $migrated.size = $file.size
    }
    if ($file.PSObject.Properties.Name -contains 'hash' -and $null -ne $file.hash) {
        $migrated.hash = $file.hash
    }
    
    return $migrated
}

function Migrate-Author {
    param($author)
    
    if ($null -eq $author) { return $null }
    
    $migrated = @{
        guid = if ($author.PSObject.Properties.Name -contains 'guid') { $author.guid } else { New-Guid }
        name = $author.name
        email = $author.email
    }
    
    $attrs = Migrate-Attributes $author.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Quality {
    param($quality)
    
    if ($null -eq $quality) { return $null }
    
    $migrated = @{
        guid = New-Guid
        key = $quality.key
        name = $quality.name
    }
    
    # Optional fields
    if ($quality.PSObject.Properties.Name -contains 'description' -and $null -ne $quality.description) {
        $migrated.description = $quality.description
    }
    if ($quality.PSObject.Properties.Name -contains 'uri' -and $null -ne $quality.uri) {
        $migrated.uri = $quality.uri
    }
    if ($quality.PSObject.Properties.Name -contains 'kind' -and $null -ne $quality.kind) {
        $migrated.kind = $quality.kind
    }
    if ($quality.PSObject.Properties.Name -contains 'folder' -and $null -ne $quality.folder) {
        $migrated.folder = $quality.folder
    }
    if ($quality.PSObject.Properties.Name -contains 'canScale' -and $null -ne $quality.canScale) {
        $migrated.canScale = $quality.canScale
    }
    if ($quality.PSObject.Properties.Name -contains 'defaultSiUnit' -and $null -ne $quality.defaultSiUnit) {
        $migrated.defaultSiUnit = $quality.defaultSiUnit
    }
    if ($quality.PSObject.Properties.Name -contains 'defaultImperialUnit' -and $null -ne $quality.defaultImperialUnit) {
        $migrated.defaultImperialUnit = $quality.defaultImperialUnit
    }
    if ($quality.PSObject.Properties.Name -contains 'min' -and $null -ne $quality.min) {
        $migrated.min = $quality.min
    }
    if ($quality.PSObject.Properties.Name -contains 'isMinExcluded' -and $null -ne $quality.isMinExcluded) {
        $migrated.isMinExcluded = $quality.isMinExcluded
    }
    if ($quality.PSObject.Properties.Name -contains 'max' -and $null -ne $quality.max) {
        $migrated.max = $quality.max
    }
    if ($quality.PSObject.Properties.Name -contains 'isMaxExcluded' -and $null -ne $quality.isMaxExcluded) {
        $migrated.isMaxExcluded = $quality.isMaxExcluded
    }
    if ($quality.PSObject.Properties.Name -contains 'defaultValue' -and $null -ne $quality.defaultValue) {
        $migrated.defaultValue = $quality.defaultValue
    }
    if ($quality.PSObject.Properties.Name -contains 'formula' -and $null -ne $quality.formula) {
        $migrated.formula = $quality.formula
    }
    if ($quality.PSObject.Properties.Name -contains 'icon' -and $null -ne $quality.icon) {
        $migrated.icon = $quality.icon
    }
    if ($quality.PSObject.Properties.Name -contains 'image' -and $null -ne $quality.image) {
        $migrated.image = $quality.image
    }
    if ($quality.PSObject.Properties.Name -contains 'unit' -and $null -ne $quality.unit) {
        $migrated.unit = $quality.unit
    }
    
    # TODO: benchmarks
    
    $attrs = Migrate-Attributes $quality.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Kit {
    param($kit)
    
    if ($null -eq $kit) { return $null }
    
    $timestamp = Get-CurrentTimestamp
    $kitGuid = New-Guid
    
    $migrated = @{
        guid = $kitGuid
        name = $kit.name
        version = $kit.version
        createdAt = $timestamp
        updatedAt = $timestamp
    }
    
    # Optional fields
    if ($kit.PSObject.Properties.Name -contains 'remote' -and $null -ne $kit.remote) {
        $migrated.remote = $kit.remote
    }
    if ($kit.PSObject.Properties.Name -contains 'homepage' -and $null -ne $kit.homepage) {
        $migrated.homepage = $kit.homepage
    }
    if ($kit.PSObject.Properties.Name -contains 'license' -and $null -ne $kit.license) {
        $migrated.license = $kit.license
    }
    if ($kit.PSObject.Properties.Name -contains 'icon' -and $null -ne $kit.icon) {
        $migrated.icon = $kit.icon
    }
    if ($kit.PSObject.Properties.Name -contains 'image' -and $null -ne $kit.image) {
        $migrated.image = $kit.image
    }
    if ($kit.PSObject.Properties.Name -contains 'description' -and $null -ne $kit.description) {
        $migrated.description = $kit.description
    }
    
    # Create name->guid mappings for types and designs
    $typeNameToGuidMap = @{}
    $designNameToGuidMap = @{}
    $portIdToGuidMap = @{}
    $authorEmailToObjectMap = @{}
    $locationIdToGuidMap = @{}
    
    # Note: NOT pre-loading types from standalone files as they may have already been migrated
    # with incorrect structure. Kit migration should be self-contained.
    
    # First pass: migrate types and build type name map and port ID map
    if ($kit.PSObject.Properties.Name -contains 'types' -and $null -ne $kit.types -and $kit.types.Count -gt 0) {
        $typesByGuid = @{}
        $migrated.types = @($kit.types | ForEach-Object { 
            $migratedType = Migrate-Type $_ $typeNameToGuidMap $authorEmailToObjectMap
            # Build port ID to GUID map from this type's ports
            if ($_.PSObject.Properties.Name -contains 'ports' -and $null -ne $_.ports) {
                for ($i = 0; $i -lt $_.ports.Count; $i++) {
                    $oldPortId = if ($_.ports[$i].PSObject.Properties.Name -contains 'id_') { $_.ports[$i].id_ } else { $null }
                    if ($null -ne $oldPortId -and $null -ne $migratedType.ports -and $i -lt $migratedType.ports.Count) {
                        $portIdToGuidMap[$oldPortId] = $migratedType.ports[$i].guid
                    }
                }
            }
            # Deduplicate by GUID - only keep first occurrence
            if ($null -ne $migratedType.guid -and -not $typesByGuid.ContainsKey($migratedType.guid)) {
                $typesByGuid[$migratedType.guid] = $true
                $migratedType
            }
        })
    }
    
    # Second pass: migrate designs with type/design/port maps, passing migrated types for port lookup
    if ($kit.PSObject.Properties.Name -contains 'designs' -and $null -ne $kit.designs -and $kit.designs.Count -gt 0) {
        $migrated.designs = @($kit.designs | ForEach-Object { Migrate-Design $_ $designNameToGuidMap $typeNameToGuidMap $portIdToGuidMap $authorEmailToObjectMap $null $migrated.types })
    }
    
    # 2.5 pass: Create abstract parent types/designs for bases that have children but don't exist
    if ($null -ne $migrated.types) {
        $parentNames = @{}
        foreach ($type in $migrated.types) {
            # Types are hashtables, not PSObjects, so use .Keys instead of .PSObject.Properties.Name
            if ($type.Keys -contains 'parent' -and $null -ne $type.parent -and $type.parent -ne '') {
                # Parent is a string at this point (before third pass conversion)
                $parentNameStr = [string]$type.parent
                if ($parentNameStr -ne '') {
                    $parentNames[$parentNameStr] = $true
                }
            }
        }
        
        $newParentTypes = @()
        $timestamp = Get-CurrentTimestamp
        
        # First create top-level abstract parents (like "Capsule")
        # These are needed by the intermediate abstract types we're about to create
        $topLevelParents = @("Capsule")
        foreach ($parentName in $topLevelParents) {
            if (-not $typeNameToGuidMap.ContainsKey($parentName)) {
                $parentGuid = New-Guid
                $newParentTypes += @{
                    guid = $parentGuid
                    name = $parentName
                    isAbstract = $true
                    createdAt = $timestamp
                    updatedAt = $timestamp
                }
                $typeNameToGuidMap[$parentName] = $parentGuid
            }
        }
        
        # Then create metabolism-specific intermediate abstract types
        $metabolismParents = @(
            @{ name = "Box"; parent = "Capsule"; isAbstract = $true }
            @{ name = "Ellipsoid"; parent = "Capsule"; isAbstract = $true }
            @{ name = "Trapezoid"; parent = "Capsule"; isAbstract = $true }
            @{ name = "Balcony"; parent = "Capsule"; isAbstract = $true }
        )
        
        foreach ($metaParent in $metabolismParents) {
            # Only create if referenced
            if ($parentNames.ContainsKey($metaParent.name)) {
                $parentGuid = New-Guid
                $parentNameStr = [string]$metaParent.name
                $parentParentStr = [string]$metaParent.parent
                $newParentType = @{
                    guid = $parentGuid
                    name = $parentNameStr
                    parent = $parentParentStr
                    isAbstract = $true
                    createdAt = $timestamp
                    updatedAt = $timestamp
                }
                $newParentTypes += $newParentType
                $typeNameToGuidMap[$parentNameStr] = $parentGuid
                # Mark this parent as handled
                $parentNames[$parentNameStr] = $false
            }
        }
        
        # Create any other missing parents
        foreach ($parentName in $parentNames.Keys) {
            # Skip if already created or if it exists
            if ($parentNames[$parentName] -eq $false -or $typeNameToGuidMap.ContainsKey($parentName)) {
                continue
            }
            # Create abstract parent type  
            $parentGuid = New-Guid
            # Explicitly convert parent name to string to avoid reference issues
            $parentNameString = [string]$parentName
            $newParentTypes += @{
                guid = $parentGuid
                name = $parentNameString
                isAbstract = $true
                createdAt = $timestamp
                updatedAt = $timestamp
            }
            $typeNameToGuidMap[$parentNameString] = $parentGuid
        }
        
        if ($newParentTypes.Count -gt 0) {
            $migrated.types = @($newParentTypes) + @($migrated.types)
        }
    }
    
    if ($null -ne $migrated.designs) {
        $newParentDesigns = @()
        foreach ($design in $migrated.designs) {
            # Designs are hashtables, not PSObjects, so use .Keys instead of .PSObject.Properties.Name
            if ($design.Keys -contains 'parent' -and $null -ne $design.parent -and $design.parent -ne '') {
                # Parent is a string at this point (before third pass conversion)
                $parentNameStr = [string]$design.parent
                if ($parentNameStr -ne '' -and -not $designNameToGuidMap.ContainsKey($parentNameStr)) {
                    # Create abstract parent design
                    $timestamp = Get-CurrentTimestamp
                    $parentGuid = New-Guid
                    $newParentDesigns += @{
                        guid = $parentGuid
                        name = $parentNameStr
                        isAbstract = $true
                        createdAt = $timestamp
                        updatedAt = $timestamp
                    }
                    $designNameToGuidMap[$parentNameStr] = $parentGuid
                    Write-Host "  Created abstract parent design: @{guid=$parentGuid}" -ForegroundColor DarkGray
                }
            }
        }
        
        if ($newParentDesigns.Count -gt 0) {
            $migrated.designs = @($newParentDesigns) + @($migrated.designs)
        }
    }
    
    # Third pass: resolve parent references in types and designs
    if ($null -ne $migrated.types) {
        foreach ($type in $migrated.types) {
            # Types are hashtables, not PSObjects, so use .Keys and .Remove() instead
            if ($type.Keys -contains 'parent' -and $null -ne $type.parent) {
                # Parent is currently a name, convert to GUID reference
                if ($typeNameToGuidMap.ContainsKey($type.parent)) {
                    $parentGuid = $typeNameToGuidMap[$type.parent]
                    $type.parent = @{ guid = $parentGuid }
                } else {
                    # Parent not found, remove the reference
                    $type.Remove('parent') | Out-Null
                }
            }
        }
    }
    
    if ($null -ne $migrated.designs) {
        foreach ($design in $migrated.designs) {
            # Designs are hashtables, not PSObjects, so use .Keys and .Remove() instead
            if ($design.Keys -contains 'parent' -and $null -ne $design.parent) {
                # Parent is currently a name, convert to GUID reference
                if ($designNameToGuidMap.ContainsKey($design.parent)) {
                    $parentGuid = $designNameToGuidMap[$design.parent]
                    $design.parent = @{ guid = $parentGuid }
                } else {
                    # Parent not found, remove the reference
                    $design.Remove('parent') | Out-Null
                }
            }
        }
    }
    
    # Migrate other collections
    if ($kit.PSObject.Properties.Name -contains 'qualities' -and $null -ne $kit.qualities -and $kit.qualities.Count -gt 0) {
        $migrated.qualities = @($kit.qualities | ForEach-Object { Migrate-Quality $_ })
    }
    
    if ($kit.PSObject.Properties.Name -contains 'files' -and $null -ne $kit.files -and $kit.files.Count -gt 0) {
        $migrated.files = @($kit.files | ForEach-Object { Migrate-File $_ $kitGuid })
    }
    
    # Collect authors from all types and designs into Kit.authors
    if ($authorEmailToObjectMap.Count -gt 0) {
        $migrated.authors = @($authorEmailToObjectMap.Values)
        Write-Host "  Collected $($authorEmailToObjectMap.Count) unique authors" -ForegroundColor Cyan
    }
    
    # If kit already has authors at root level, add them too
    if ($kit.PSObject.Properties.Name -contains 'authors' -and $null -ne $kit.authors -and $kit.authors.Count -gt 0) {
        # Kit.authors should store full Author objects (not ID references)
        if ($kit.authors[0] -is [string]) {
            # Keep as strings (references) - this shouldn't happen for Kit
            $migrated.authors = $kit.authors
        } else {
            # Migrate author objects and merge with collected authors
            $kitAuthors = @($kit.authors | ForEach-Object { Migrate-Author $_ })
            foreach ($author in $kitAuthors) {
                if ($null -ne $author.email -and -not $authorEmailToObjectMap.ContainsKey($author.email)) {
                    $authorEmailToObjectMap[$author.email] = $author
                }
            }
            $migrated.authors = @($authorEmailToObjectMap.Values)
        }
    }
    
    # Convert location to ID reference if present
    if ($kit.PSObject.Properties.Name -contains 'location' -and $null -ne $kit.location) {
        $migratedLocation = Migrate-Location $kit.location
        if ($null -ne $migratedLocation) {
            $migrated.location = @{ guid = $migratedLocation.guid }
        }
    }
    
    $attrs = Migrate-Attributes $kit.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    # TODO: concepts
    
    return $migrated
}

function Migrate-JsonFile {
    param(
        [string]$FilePath,
        [switch]$DryRun
    )
    
    Write-Host "Processing: $FilePath" -ForegroundColor Cyan
    
    try {
        $content = Get-Content -Path $FilePath -Raw | ConvertFrom-Json
        
        # Determine what kind of file this is
        $migrated = $null
        
        if ($FilePath -match 'kit_') {
            Write-Host "  Detected: Kit" -ForegroundColor Gray
            $migrated = Migrate-Kit $content
        }
        elseif ($FilePath -match 'type_') {
            Write-Host "  Detected: Type" -ForegroundColor Gray
            # Pre-load type GUIDs and port data from kit to ensure consistency
            $typeNameToGuidMap = @{}
            $portNameToDataMap = @{}
            $authorEmailToObjectMap = $null
            
            # Extract type name from filename (e.g., "type_bridge.json" -> "Bridge")
            $filename = Split-Path -Leaf $FilePath
            $typeName = ($filename -replace 'type_', '' -replace '.json', '').Split('_') | ForEach-Object {
                $_.Substring(0,1).ToUpper() + $_.Substring(1)
            }
            $typeName = $typeName -join ' '
            
            # Load type GUIDs and port data from kit file if it exists
            $kitPath = Join-Path (Split-Path $FilePath) "kit_metabolism.json"
            if (Test-Path $kitPath) {
                try {
                    $kitContent = Get-Content -Path $kitPath -Raw | ConvertFrom-Json
                    if ($kitContent.PSObject.Properties.Name -contains 'types' -and $null -ne $kitContent.types) {
                        foreach ($kitType in $kitContent.types) {
                            if ($kitType.PSObject.Properties.Name -contains 'name' -and $kitType.PSObject.Properties.Name -contains 'guid') {
                                $typeNameToGuidMap[$kitType.name] = $kitType.guid
                                
                                # If this is the type we're migrating, collect its port data
                                if ($kitType.name -eq $typeName -and $kitType.PSObject.Properties.Name -contains 'ports' -and $null -ne $kitType.ports) {
                                    foreach ($kitPort in $kitType.ports) {
                                        if ($kitPort.PSObject.Properties.Name -contains 'name' -and $kitPort.PSObject.Properties.Name -contains 'guid') {
                                            $portNameToDataMap[$kitPort.name] = @{
                                                guid = $kitPort.guid
                                                name = $kitPort.name
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Write-Host "  Pre-loaded $($typeNameToGuidMap.Count) type GUIDs and $($portNameToDataMap.Count) ports from kit" -ForegroundColor DarkGray
                    }
                } catch {
                    Write-Warning "  Could not pre-load types from kit: $_"
                }
            }
            
            $migrated = Migrate-Type $content $typeNameToGuidMap $authorEmailToObjectMap $portNameToDataMap
        }
        elseif ($FilePath -match 'design_') {
            Write-Host "  Detected: Design" -ForegroundColor Gray
            # For standalone design files, load design/type/port/piece GUIDs from kit (authoritative source)
            $designNameToGuidMap = @{}
            $typeNameToGuidMap = @{}
            $portIdToGuidMap = @{}
            $pieceNameToGuidMap = @{}
            $authorEmailToObjectMap = $null
            
            # Find kit file in the same directory
            $directory = Split-Path -Path $FilePath
            $kitPath = Join-Path $directory "kit_metabolism.json"
            
            # Extract design name from filename (e.g., "design_nakagin-capsule-tower_flat.json")
            $filename = Split-Path -Leaf $FilePath
            $designFileName = ($filename -replace 'design_', '' -replace '.json', '')
            $isFlatDesign = $designFileName -match '_flat$'
            $baseDesignName = if ($isFlatDesign) { $designFileName -replace '_flat$', '' } else { $designFileName }
            
            if (Test-Path $kitPath) {
                try {
                    $kitContent = Get-Content -Path $kitPath -Raw | ConvertFrom-Json
                    
                    # Load designs from kit
                    if ($kitContent.PSObject.Properties.Name -contains 'designs' -and $null -ne $kitContent.designs) {
                        foreach ($kitDesign in $kitContent.designs) {
                            if ($kitDesign.PSObject.Properties.Name -contains 'name' -and $kitDesign.PSObject.Properties.Name -contains 'guid') {
                                $designNameToGuidMap[$kitDesign.name] = $kitDesign.guid
                                
                                # If this is a flat design, load piece name->GUID mapping from the base design
                                if ($isFlatDesign) {
                                    # Match base design name (e.g., "Nakagin Capsule Tower" for "nakagin-capsule-tower_flat")
                                    $normalizedKitName = $kitDesign.name -replace '[^a-zA-Z0-9]', '' -replace ' ', ''
                                    $normalizedBaseName = $baseDesignName -replace '[^a-zA-Z0-9]', '' -replace '-', '' -replace '_', ''
                                    
                                    if ($normalizedKitName -eq $normalizedBaseName) {
                                        # Load piece name->GUID mapping from this design
                                        if ($kitDesign.PSObject.Properties.Name -contains 'pieces' -and $null -ne $kitDesign.pieces) {
                                            foreach ($piece in $kitDesign.pieces) {
                                                if ($piece.PSObject.Properties.Name -contains 'name' -and $null -ne $piece.name -and $piece.name -ne '' -and $piece.PSObject.Properties.Name -contains 'guid') {
                                                    $pieceNameToGuidMap[$piece.name] = $piece.guid
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    # Load types from kit
                    if ($kitContent.PSObject.Properties.Name -contains 'types' -and $null -ne $kitContent.types) {
                        foreach ($kitType in $kitContent.types) {
                            if ($kitType.PSObject.Properties.Name -contains 'name' -and $kitType.PSObject.Properties.Name -contains 'guid') {
                                $typeName = $kitType.name
                                $typeGuid = $kitType.guid
                                $typeNameToGuidMap[$typeName] = $typeGuid
                                
                                # Also map with name|variant composite key (variant defaults to empty string if null)
                                $typeVariant = if ($kitType.PSObject.Properties.Name -contains 'variant' -and $null -ne $kitType.variant) { $kitType.variant } else { '' }
                                $compositeKey = "$typeName|$typeVariant"
                                $typeNameToGuidMap[$compositeKey] = $typeGuid
                                
                                # Load ports from this type - create mapping from old port id_ to new GUID
                                if ($kitType.PSObject.Properties.Name -contains 'ports' -and $null -ne $kitType.ports) {
                                    foreach ($port in $kitType.ports) {
                                        if ($port.PSObject.Properties.Name -contains 'guid' -and $port.PSObject.Properties.Name -contains 'name' -and $null -ne $port.name -and $port.name -ne '') {
                                            # Map old port name (which comes from id_) to new port GUID
                                            $portIdToGuidMap[$port.name] = $port.guid
                                        }
                                    }
                                }
                            }
                        }
                        if ($isFlatDesign) {
                            Write-Host "  Pre-loaded $($designNameToGuidMap.Count) designs, $($typeNameToGuidMap.Count) types, $($portIdToGuidMap.Count) ports, and $($pieceNameToGuidMap.Count) pieces from kit" -ForegroundColor DarkGray
                        } else {
                            Write-Host "  Pre-loaded $($designNameToGuidMap.Count) designs, $($typeNameToGuidMap.Count) types, and $($portIdToGuidMap.Count) ports from kit" -ForegroundColor DarkGray
                        }
                    }
                } catch {
                    Write-Warning "  Could not pre-load from kit: $_"
                }
            } else {
                Write-Warning "  Kit file not found, falling back to standalone type files"
                # Fallback: load from standalone type files
                $typeFiles = Get-ChildItem -Path $directory -Filter 'type_*.json'
                foreach ($typeFile in $typeFiles) {
                    try {
                        $typeContent = Get-Content -Path $typeFile.FullName -Raw | ConvertFrom-Json
                        $typeName = $typeContent.name
                        $typeGuid = $typeContent.guid
                        $typeNameToGuidMap[$typeName] = $typeGuid
                    } catch {
                        Write-Warning "  Failed to pre-load type file $($typeFile.Name): $_"
                    }
                }
            }
            
            # Load kit types for port lookup during connection migration
            $kitTypes = if ($null -ne $kitContent -and $kitContent.PSObject.Properties.Name -contains 'types') { $kitContent.types } else { $null }
            Write-Host "  Loaded $($kitTypes.Count) kit types for port lookup" -ForegroundColor DarkGray
            $migrated = Migrate-Design $content $designNameToGuidMap $typeNameToGuidMap $portIdToGuidMap $authorEmailToObjectMap $pieceNameToGuidMap $kitTypes
        }
        else {
            Write-Warning "  Unknown file type, skipping"
            return
        }
        
        if ($null -ne $migrated) {
            $json = $migrated | ConvertTo-Json -Depth 100 -Compress:$false -EscapeHandling EscapeNonAscii
            
            # Fix single-object arrays that PowerShell might have collapsed
            # This is a workaround for PowerShell's ConvertTo-Json behavior
            
            if ($DryRun) {
                Write-Host "  [DRY RUN] Would write migrated JSON" -ForegroundColor Yellow
            } else {
                Set-Content -Path $FilePath -Value $json -Encoding UTF8
                Write-Host "  ✓ Migrated successfully" -ForegroundColor Green
            }
        }
    }
    catch {
        Write-Error "Failed to process $FilePath : $_"
    }
}

# Main execution
Write-Host "Semio JSON Migration Script" -ForegroundColor Magenta
Write-Host "============================" -ForegroundColor Magenta
Write-Host ""

if ($DryRun) {
    Write-Host "Running in DRY RUN mode - no files will be modified" -ForegroundColor Yellow
    Write-Host ""
}

$jsonFiles = Get-ChildItem -Path $Path -Filter "*.json" -File

Write-Host "Found $($jsonFiles.Count) JSON files" -ForegroundColor White
Write-Host ""

# Process only kit files (types and designs are embedded within the kit)
# Standalone type/design files will be skipped to avoid duplication issues
$kitFiles = $jsonFiles | Where-Object { $_.Name -match '^kit_' }

Write-Host "Processing: Kit files only" -ForegroundColor Cyan
Write-Host "  Kit files: $($kitFiles.Count)" -ForegroundColor DarkCyan
Write-Host "  Note: Standalone type/design files are skipped (all data is in kit)" -ForegroundColor DarkGray
Write-Host ""

Write-Host "Migrating kit file..." -ForegroundColor Yellow
foreach ($file in $kitFiles) {
    Migrate-JsonFile -FilePath $file.FullName -DryRun:$DryRun
}

Write-Host ""
Write-Host "Migration complete!" -ForegroundColor Green
