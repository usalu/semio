# Migration script to update Semio JSON files to new schema
# This script migrates old JSON format to match the TypeScript schema

param(
    [string]$Path = "assets\semio",
    [switch]$DryRun = $false
)

function New-Guid {
    return [System.Guid]::NewGuid().ToString()
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
        guid = New-Guid
        t = $port.t
        point = Migrate-Point $port.point
        direction = Migrate-Vector $port.direction
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
    param($type, $typeNameToGuidMap, $authorEmailToObjectMap)
    
    if ($null -eq $type) { return $null }
    
    $timestamp = Get-CurrentTimestamp
    
    # Determine the actual name (variant becomes the name if present)
    $actualName = $type.name
    if ($type.PSObject.Properties.Name -contains 'variant' -and $null -ne $type.variant -and $type.variant -ne '') {
        $actualName = $type.variant
    }
    
    # Use pre-loaded GUID if it exists, otherwise use existing GUID or generate new one
    $typeGuid = $null
    if ($null -ne $typeNameToGuidMap -and $typeNameToGuidMap.ContainsKey($actualName)) {
        $typeGuid = $typeNameToGuidMap[$actualName]
    } elseif ($type.PSObject.Properties.Name -contains 'guid') {
        $typeGuid = $type.guid
    } else {
        $typeGuid = New-Guid
    }
    
    # Store mapping (for pieces to reference)
    if ($null -ne $typeNameToGuidMap -and -not $typeNameToGuidMap.ContainsKey($actualName)) {
        $typeNameToGuidMap[$actualName] = $typeGuid
    }
    
    $migrated = @{
        guid = $typeGuid
        name = $actualName
        createdAt = $timestamp
        updatedAt = $timestamp
    }
    
    # Store mapping for later reference resolution
    # Store BOTH old format (name|variant) and new format (actualName) for lookups
    if ($null -ne $typeNameToGuidMap) {
        $variant = if ($type.PSObject.Properties.Name -contains 'variant') { $type.variant } else { '' }
        
        # Store old format: "name|variant" for piece lookups
        $oldMapKey = "$($type.name)|$variant"
        $typeNameToGuidMap[$oldMapKey] = $typeGuid
        
        # Store new format: actualName for parent lookups and standalone types
        $typeNameToGuidMap[$actualName] = $typeGuid
    }
    
    # Optional fields
    # Metabolism-specific hierarchy mapping
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
    
    $ports = Migrate-Ports $type.ports
    if ($null -ne $ports) {
        $migrated.ports = $ports
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
    param($piece, $typeNameToGuidMap, $designNameToGuidMap)
    
    if ($null -eq $piece) { return $null }
    
    $migrated = @{
        guid = if ($piece.PSObject.Properties.Name -contains 'guid') { $piece.guid } else { New-Guid }
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
                Write-Host "[DEBUG-PIECE] Assigned type: $($migrated.type | ConvertTo-Json -Compress)" -ForegroundColor Green
            } elseif ($typeNameToGuidMap.ContainsKey($typeName)) {
                $migrated.type = @{ guid = $typeNameToGuidMap[$typeName] }
                Write-Host "[DEBUG-PIECE] Assigned type (fallback): $($migrated.type | ConvertTo-Json -Compress)" -ForegroundColor Green
            } else {
                Write-Host "[DEBUG-PIECE] Type NOT FOUND: $mapKey" -ForegroundColor Red
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
            x = $piece.center.x
            y = $piece.center.y
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
    
    Write-Host "[DEBUG-PIECE] Returning piece with keys: $($migrated.Keys -join ', ')" -ForegroundColor Cyan
    if ($migrated.ContainsKey('type')) {
        Write-Host "[DEBUG-PIECE] Piece HAS type: $($migrated.type | ConvertTo-Json -Compress)" -ForegroundColor Green
    } else {
        Write-Host "[DEBUG-PIECE] Piece MISSING type!" -ForegroundColor Red
    }
    
    return $migrated
}

function Migrate-Pieces {
    param($pieces, $typeNameToGuidMap, $designNameToGuidMap)
    
    if ($null -eq $pieces -or $pieces.Count -eq 0) { return $null }
    
    # Keep as hashtables - do NOT convert to PSCustomObject
    $migratedPieces = @($pieces | ForEach-Object { 
        Migrate-Piece $_ $typeNameToGuidMap $designNameToGuidMap
    })
    
    return $migratedPieces
}

function Migrate-Connection {
    param($conn, $pieceIdToGuidMap, $portIdToGuidMap)
    
    if ($null -eq $conn) { return $null }
    
    $migrated = @{
        guid = if ($conn.PSObject.Properties.Name -contains 'guid') { $conn.guid } else { New-Guid }
    }
    
    # Connected side
    if ($conn.PSObject.Properties.Name -contains 'connected' -and $null -ne $conn.connected) {
        $connectedSide = @{
            guid = New-Guid
        }
        
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
                } elseif ($conn.connected.piece.PSObject.Properties.Name -contains 'id_') {
                    $pieceId = $conn.connected.piece.id_
                } elseif ($conn.connected.piece.PSObject.Properties.Name -contains 'id') {
                    $pieceId = $conn.connected.piece.id
                }
                if ($null -ne $pieceId -and $pieceIdToGuidMap.ContainsKey($pieceId)) {
                    $connectedSide.piece = @{ guid = $pieceIdToGuidMap[$pieceId] }
                } else {
                    Write-Warning "  [CONNECTION] Piece ID '$pieceId' not found in map (connected side)"
                }
            }
            if ($conn.connected.PSObject.Properties.Name -contains 'port' -and $null -ne $conn.connected.port) {
                $portId = $null
                if ($conn.connected.port -is [string]) {
                    $portId = $conn.connected.port
                } elseif ($conn.connected.port.PSObject.Properties.Name -contains 'id_') {
                    $portId = $conn.connected.port.id_
                } elseif ($conn.connected.port.PSObject.Properties.Name -contains 'id') {
                    $portId = $conn.connected.port.id
                }
                if ($null -ne $portId -and $portIdToGuidMap.ContainsKey($portId)) {
                    $connectedSide.port = @{ guid = $portIdToGuidMap[$portId] }
                } else {
                    Write-Warning "  [CONNECTION] Port ID '$portId' not found in map (connected side)"
                }
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
        $connectingSide = @{
            guid = New-Guid
        }
        
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
                } elseif ($conn.connecting.piece.PSObject.Properties.Name -contains 'id_') {
                    $pieceId = $conn.connecting.piece.id_
                } elseif ($conn.connecting.piece.PSObject.Properties.Name -contains 'id') {
                    $pieceId = $conn.connecting.piece.id
                }
                if ($null -ne $pieceId -and $pieceIdToGuidMap.ContainsKey($pieceId)) {
                    $connectingSide.piece = @{ guid = $pieceIdToGuidMap[$pieceId] }
                }
            }
            if ($conn.connecting.PSObject.Properties.Name -contains 'port' -and $null -ne $conn.connecting.port) {
                $portId = $null
                if ($conn.connecting.port -is [string]) {
                    $portId = $conn.connecting.port
                } elseif ($conn.connecting.port.PSObject.Properties.Name -contains 'id_') {
                    $portId = $conn.connecting.port.id_
                } elseif ($conn.connecting.port.PSObject.Properties.Name -contains 'id') {
                    $portId = $conn.connecting.port.id
                }
                if ($null -ne $portId -and $portIdToGuidMap.ContainsKey($portId)) {
                    $connectingSide.port = @{ guid = $portIdToGuidMap[$portId] }
                }
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
    param($connections, $pieceIdToGuidMap, $portIdToGuidMap)
    
    if ($null -eq $connections -or $connections.Count -eq 0) { return $null }
    
    return @($connections | ForEach-Object { Migrate-Connection $_ $pieceIdToGuidMap $portIdToGuidMap })
}

function Migrate-Design {
    param($design, $designNameToGuidMap, $typeNameToGuidMap, $portIdToGuidMap, $authorEmailToObjectMap)
    
    if ($null -eq $design) { return $null }
    
    $timestamp = Get-CurrentTimestamp
    
    # Determine the actual name (variant becomes the name if present, view is ignored as it's deprecated)
    $actualName = $design.name
    if ($design.PSObject.Properties.Name -contains 'variant' -and $null -ne $design.variant -and $design.variant -ne '') {
        $actualName = $design.variant
    }
    
    # Use pre-loaded GUID if it exists, otherwise use existing GUID or generate new one
    $designGuid = $null
    if ($null -ne $designNameToGuidMap -and $designNameToGuidMap.ContainsKey($actualName)) {
        $designGuid = $designNameToGuidMap[$actualName]
    } elseif ($design.PSObject.Properties.Name -contains 'guid') {
        $designGuid = $design.guid
    } else {
        $designGuid = New-Guid
    }
    
    # Store mapping (for pieces to reference)
    if ($null -ne $designNameToGuidMap -and -not $designNameToGuidMap.ContainsKey($actualName)) {
        $designNameToGuidMap[$actualName] = $designGuid
    }
    
    $migrated = @{
        guid = $designGuid
        name = $actualName
        createdAt = $timestamp
        updatedAt = $timestamp
    }
    
    # Store mapping for later reference resolution
    # Store BOTH old format (name|variant|view) and new format (actualName) for lookups
    if ($null -ne $designNameToGuidMap) {
        $variant = if ($design.PSObject.Properties.Name -contains 'variant') { $design.variant } else { '' }
        $view = if ($design.PSObject.Properties.Name -contains 'view') { $design.view } else { '' }
        
        # Store old format: "name|variant|view" for piece lookups
        $oldMapKey = "$($design.name)|$variant|$view"
        $designNameToGuidMap[$oldMapKey] = $designGuid
        
        # Store new format: actualName for parent lookups and standalone designs
        $designNameToGuidMap[$actualName] = $designGuid
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
    $pieces = Migrate-Pieces $design.pieces $typeNameToGuidMap $designNameToGuidMap
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
    
    # Migrate connections using the piece and port mappings
    $connections = Migrate-Connections $design.connections $pieceIdToGuidMap $portIdToGuidMap
    if ($null -ne $connections) {
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
    
    # Pre-load existing GUIDs from standalone type/design files to ensure consistency
    $kitDir = Split-Path -Path $PSCommandPath
    $assetsDir = Join-Path (Split-Path $kitDir) "assets\semio"
    if (Test-Path $assetsDir) {
        # Load type GUIDs
        $typeFiles = Get-ChildItem -Path $assetsDir -Filter "type_*.json" -File -ErrorAction SilentlyContinue
        foreach ($typeFile in $typeFiles) {
            try {
                $typeContent = Get-Content -Path $typeFile.FullName -Raw | ConvertFrom-Json
                if ($typeContent.PSObject.Properties.Name -contains 'guid' -and $typeContent.PSObject.Properties.Name -contains 'name') {
                    $typeName = $typeContent.name
                    $typeVariant = if ($typeContent.PSObject.Properties.Name -contains 'variant') { $typeContent.variant } else { '' }
                    $actualName = if ($typeVariant -ne '') { $typeVariant } else { $typeName }
                    $typeNameToGuidMap[$actualName] = $typeContent.guid
                    Write-Host "  Pre-loaded type GUID: $actualName -> $($typeContent.guid)" -ForegroundColor DarkGray
                }
            } catch {
                # Silently skip files that can't be read
            }
        }
        
        # Load design GUIDs
        $designFiles = Get-ChildItem -Path $assetsDir -Filter "design_*.json" -File -ErrorAction SilentlyContinue
        foreach ($designFile in $designFiles) {
            try {
                $designContent = Get-Content -Path $designFile.FullName -Raw | ConvertFrom-Json
                if ($designContent.PSObject.Properties.Name -contains 'guid' -and $designContent.PSObject.Properties.Name -contains 'name') {
                    $designName = $designContent.name
                    $designVariant = if ($designContent.PSObject.Properties.Name -contains 'variant') { $designContent.variant } else { '' }
                    $actualName = if ($designVariant -ne '') { $designVariant } else { $designName }
                    $designNameToGuidMap[$actualName] = $designContent.guid
                    Write-Host "  Pre-loaded design GUID: $actualName -> $($designContent.guid)" -ForegroundColor DarkGray
                }
            } catch {
                # Silently skip files that can't be read
            }
        }
    }
    
    # First pass: migrate types and build type name map and port ID map
    if ($kit.PSObject.Properties.Name -contains 'types' -and $null -ne $kit.types -and $kit.types.Count -gt 0) {
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
            $migratedType
        })
    }
    
    # Second pass: migrate designs with type/design/port maps
    if ($kit.PSObject.Properties.Name -contains 'designs' -and $null -ne $kit.designs -and $kit.designs.Count -gt 0) {
        $migrated.designs = @($kit.designs | ForEach-Object { Migrate-Design $_ $designNameToGuidMap $typeNameToGuidMap $portIdToGuidMap $authorEmailToObjectMap })
    }
    
    # 2.5 pass: Create abstract parent types/designs for bases that have children but don't exist
    if ($null -ne $migrated.types) {
        $parentNames = @{}
        foreach ($type in $migrated.types) {
            # Types are hashtables, not PSObjects, so use .Keys instead of .PSObject.Properties.Name
            if ($type.Keys -contains 'parent' -and $null -ne $type.parent -and $type.parent -ne '') {
                $parentNames[$type.parent] = $true
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
        $parentNames = @{}
        foreach ($design in $migrated.designs) {
            # Designs are hashtables, not PSObjects, so use .Keys instead of .PSObject.Properties.Name
            if ($design.Keys -contains 'parent' -and $null -ne $design.parent -and $design.parent -ne '') {
                $parentNames[$design.parent] = $true
            }
        }
        
        $newParentDesigns = @()
        foreach ($parentName in $parentNames.Keys) {
            # Check if this parent exists as a design
            if (-not $designNameToGuidMap.ContainsKey($parentName)) {
                # Create abstract parent design
                $timestamp = Get-CurrentTimestamp
                $parentGuid = New-Guid
                $newParentDesigns += @{
                    guid = $parentGuid
                    name = $parentName
                    isAbstract = $true
                    createdAt = $timestamp
                    updatedAt = $timestamp
                }
                $designNameToGuidMap[$parentName] = $parentGuid
                Write-Host "  Created abstract parent design: $parentName" -ForegroundColor Cyan
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
            # Pre-load type GUIDs from kit to ensure consistency
            $typeNameToGuidMap = @{}
            $authorEmailToObjectMap = $null
            
            # Load type GUIDs from kit file if it exists
            $kitPath = Join-Path (Split-Path $FilePath) "kit_metabolism.json"
            if (Test-Path $kitPath) {
                try {
                    $kitContent = Get-Content -Path $kitPath -Raw | ConvertFrom-Json
                    if ($kitContent.PSObject.Properties.Name -contains 'types' -and $null -ne $kitContent.types) {
                        foreach ($kitType in $kitContent.types) {
                            if ($kitType.PSObject.Properties.Name -contains 'name' -and $kitType.PSObject.Properties.Name -contains 'guid') {
                                $typeNameToGuidMap[$kitType.name] = $kitType.guid
                            }
                        }
                        Write-Host "  Pre-loaded $($typeNameToGuidMap.Count) type GUIDs from kit" -ForegroundColor DarkGray
                    }
                } catch {
                    Write-Warning "  Could not pre-load types from kit: $_"
                }
            }
            
            $migrated = Migrate-Type $content $typeNameToGuidMap $authorEmailToObjectMap
        }
        elseif ($FilePath -match 'design_') {
            Write-Host "  Detected: Design" -ForegroundColor Gray
            # For standalone design files, load design/type/port GUIDs from kit (authoritative source)
            $designNameToGuidMap = @{}
            $typeNameToGuidMap = @{}
            $portIdToGuidMap = @{}
            $authorEmailToObjectMap = $null
            
            # Find kit file in the same directory
            $directory = Split-Path -Path $FilePath
            $kitPath = Join-Path $directory "kit_metabolism.json"
            
            if (Test-Path $kitPath) {
                try {
                    $kitContent = Get-Content -Path $kitPath -Raw | ConvertFrom-Json
                    
                    # Load designs from kit
                    if ($kitContent.PSObject.Properties.Name -contains 'designs' -and $null -ne $kitContent.designs) {
                        foreach ($kitDesign in $kitContent.designs) {
                            if ($kitDesign.PSObject.Properties.Name -contains 'name' -and $kitDesign.PSObject.Properties.Name -contains 'guid') {
                                $designNameToGuidMap[$kitDesign.name] = $kitDesign.guid
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
                                
                                # Load ports from this type - map port GUID to itself for lookup
                                if ($kitType.PSObject.Properties.Name -contains 'ports' -and $null -ne $kitType.ports) {
                                    foreach ($port in $kitType.ports) {
                                        if ($port.PSObject.Properties.Name -contains 'guid') {
                                            # Since kit is already migrated, port GUIDs are final
                                            # We map guid->guid so connections can reference them
                                            $portIdToGuidMap[$port.guid] = $port.guid
                                        }
                                    }
                                }
                            }
                        }
                        Write-Host "  Pre-loaded $($designNameToGuidMap.Count) designs, $($typeNameToGuidMap.Count) types, and $($portIdToGuidMap.Count) ports from kit" -ForegroundColor DarkGray
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
            
            $migrated = Migrate-Design $content $designNameToGuidMap $typeNameToGuidMap $portIdToGuidMap $authorEmailToObjectMap
        }
        else {
            Write-Warning "  Unknown file type, skipping"
            return
        }
        
        if ($null -ne $migrated) {
            # Debug: Check if pieces have type before serialization
            if ($migrated.PSObject.Properties.Name -contains 'pieces' -and $null -ne $migrated.pieces -and $migrated.pieces.Count -gt 0) {
                $firstPiece = $migrated.pieces[0]
                $pieceType = $firstPiece.GetType().FullName
                Write-Host "[DEBUG-JSON] First piece type: $pieceType" -ForegroundColor Cyan
                
                # Check for type property (works for both hashtable and PSCustomObject)
                $hasType = $false
                if ($firstPiece -is [hashtable]) {
                    $hasType = $firstPiece.ContainsKey('type')
                    $propNames = $firstPiece.Keys -join ', '
                } else {
                    $hasType = $null -ne ($firstPiece.PSObject.Properties.Name -contains 'type')
                    $propNames = $firstPiece.PSObject.Properties.Name -join ', '
                }
                
                if ($hasType) {
                    Write-Host "[DEBUG-JSON] First piece HAS type before serialization: $($firstPiece.type | ConvertTo-Json -Compress)" -ForegroundColor Green
                } else {
                    Write-Host "[DEBUG-JSON] First piece MISSING type before serialization! Properties: $propNames" -ForegroundColor Red
                }
            }
            
            # Ensure single-item arrays are preserved
            $json = $migrated | ConvertTo-Json -Depth 100 -Compress:$false -EscapeHandling EscapeNonAscii
            
            # Debug: Check if type exists in JSON string
            if ($json -match '"type"') {
                Write-Host "[DEBUG-JSON] Type field FOUND in JSON string" -ForegroundColor Green
            } else {
                Write-Host "[DEBUG-JSON] Type field MISSING from JSON string!" -ForegroundColor Red
            }
            
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

# Process kit files FIRST to establish authoritative GUIDs
$kitFiles = $jsonFiles | Where-Object { $_.Name -match '^kit_' }
$otherFiles = $jsonFiles | Where-Object { $_.Name -notmatch '^kit_' }

Write-Host "Processing order: Kit files first, then others" -ForegroundColor Cyan
Write-Host "  Kit files: $($kitFiles.Count)" -ForegroundColor DarkCyan
Write-Host "  Other files: $($otherFiles.Count)" -ForegroundColor DarkCyan
Write-Host ""

foreach ($file in $kitFiles) {
    Migrate-JsonFile -FilePath $file.FullName -DryRun:$DryRun
}

foreach ($file in $otherFiles) {
    Migrate-JsonFile -FilePath $file.FullName -DryRun:$DryRun
}

Write-Host ""
Write-Host "Migration complete!" -ForegroundColor Green
