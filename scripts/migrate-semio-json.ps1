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

function Migrate-Representation {
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

function Migrate-Representations {
    param($representations)
    
    if ($null -eq $representations -or $representations.Count -eq 0) { return $null }
    
    return @($representations | ForEach-Object { Migrate-Representation $_ })
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
    if ($port.PSObject.Properties.Name -contains 'family' -and $null -ne $port.family) {
        $migrated.family = $port.family
    }
    if ($port.PSObject.Properties.Name -contains 'mandatory' -and $null -ne $port.mandatory) {
        $migrated.mandatory = $port.mandatory
    }
    if ($port.PSObject.Properties.Name -contains 'compatibleFamilies' -and $null -ne $port.compatibleFamilies -and $port.compatibleFamilies.Count -gt 0) {
        $migrated.compatibleFamilies = $port.compatibleFamilies
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
    param($type)
    
    if ($null -eq $type) { return $null }
    
    $timestamp = Get-CurrentTimestamp
    
    $migrated = @{
        guid = New-Guid
        name = $type.name
        createdAt = $timestamp
        updatedAt = $timestamp
    }
    
    # Optional fields
    if ($type.PSObject.Properties.Name -contains 'parent' -and $null -ne $type.parent) {
        $migrated.parent = $type.parent
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
            $migrated.authors = $type.authors
        } else {
            $migrated.authors = @($type.authors | ForEach-Object { Migrate-Author $_ })
        }
    }
    if ($type.PSObject.Properties.Name -contains 'concepts' -and $null -ne $type.concepts -and $type.concepts.Count -gt 0) {
        $migrated.concepts = $type.concepts
    }
    
    $reps = Migrate-Representations $type.representations
    if ($null -ne $reps) {
        $migrated.representations = $reps
    }
    
    $ports = Migrate-Ports $type.ports
    if ($null -ne $ports) {
        $migrated.ports = $ports
    }
    
    $props = Migrate-Props $type.props
    if ($null -ne $props) {
        $migrated.props = $props
    }
    
    $location = Migrate-Location $type.location
    if ($null -ne $location) {
        $migrated.location = $location
    }
    
    $attrs = Migrate-Attributes $type.attributes
    if ($null -ne $attrs) {
        $migrated.attributes = $attrs
    }
    
    return $migrated
}

function Migrate-Piece {
    param($piece)
    
    if ($null -eq $piece) { return $null }
    
    $migrated = @{
        guid = if ($piece.PSObject.Properties.Name -contains 'guid') { $piece.guid } else { New-Guid }
        id = if ($piece.PSObject.Properties.Name -contains 'id_') { $piece.id_ } else { $piece.id }
    }
    
    # Type reference
    if ($piece.PSObject.Properties.Name -contains 'type' -and $null -ne $piece.type) {
        if ($piece.type -is [string]) {
            $migrated.type = $piece.type
        } else {
            # Old format with name/variant object
            $migrated.type = $piece.type.name
        }
    }
    
    # Design reference
    if ($piece.PSObject.Properties.Name -contains 'design' -and $null -ne $piece.design) {
        if ($piece.design -is [string]) {
            $migrated.design = $piece.design
        } else {
            # Old format with name/variant object
            $migrated.design = $piece.design.name
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
    
    return $migrated
}

function Migrate-Pieces {
    param($pieces)
    
    if ($null -eq $pieces -or $pieces.Count -eq 0) { return $null }
    
    return @($pieces | ForEach-Object { Migrate-Piece $_ })
}

function Migrate-Connection {
    param($conn)
    
    if ($null -eq $conn) { return $null }
    
    $migrated = @{
        guid = if ($conn.PSObject.Properties.Name -contains 'guid') { $conn.guid } else { New-Guid }
    }
    
    # Connected/connecting can be piece IDs or port references
    if ($conn.PSObject.Properties.Name -contains 'connected' -and $null -ne $conn.connected) {
        if ($conn.connected -is [string]) {
            $migrated.connected = @{ piece = $conn.connected }
        } else {
            $migrated.connected = @{
                piece = $conn.connected.piece
            }
            if ($conn.connected.PSObject.Properties.Name -contains 'port' -and $null -ne $conn.connected.port) {
                $migrated.connected.port = $conn.connected.port
            }
            if ($conn.connected.PSObject.Properties.Name -contains 'designPiece' -and $null -ne $conn.connected.designPiece) {
                $migrated.connected.designPiece = $conn.connected.designPiece
            }
        }
    }
    
    if ($conn.PSObject.Properties.Name -contains 'connecting' -and $null -ne $conn.connecting) {
        if ($conn.connecting -is [string]) {
            $migrated.connecting = @{ piece = $conn.connecting }
        } else {
            $migrated.connecting = @{
                piece = $conn.connecting.piece
            }
            if ($conn.connecting.PSObject.Properties.Name -contains 'port' -and $null -ne $conn.connecting.port) {
                $migrated.connecting.port = $conn.connecting.port
            }
            if ($conn.connecting.PSObject.Properties.Name -contains 'designPiece' -and $null -ne $conn.connecting.designPiece) {
                $migrated.connecting.designPiece = $conn.connecting.designPiece
            }
        }
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
    param($connections)
    
    if ($null -eq $connections -or $connections.Count -eq 0) { return $null }
    
    return @($connections | ForEach-Object { Migrate-Connection $_ })
}

function Migrate-Design {
    param($design)
    
    if ($null -eq $design) { return $null }
    
    $timestamp = Get-CurrentTimestamp
    
    $migrated = @{
        guid = New-Guid
        name = $design.name
        createdAt = $timestamp
        updatedAt = $timestamp
    }
    
    # Optional fields
    if ($design.PSObject.Properties.Name -contains 'parent' -and $null -ne $design.parent) {
        $migrated.parent = $design.parent
    }
    if ($design.PSObject.Properties.Name -contains 'isAbstract' -and $null -ne $design.isAbstract) {
        $migrated.isAbstract = $design.isAbstract
    }
    if ($design.PSObject.Properties.Name -contains 'folder' -and $null -ne $design.folder) {
        $migrated.folder = $design.folder
    }
    if ($design.PSObject.Properties.Name -contains 'view' -and $null -ne $design.view) {
        $migrated.view = Migrate-Plane $design.view
    }
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
            $migrated.authors = $design.authors
        } else {
            $migrated.authors = @($design.authors | ForEach-Object { Migrate-Author $_ })
        }
    }
    if ($design.PSObject.Properties.Name -contains 'concepts' -and $null -ne $design.concepts -and $design.concepts.Count -gt 0) {
        $migrated.concepts = $design.concepts
    }
    
    $pieces = Migrate-Pieces $design.pieces
    if ($null -ne $pieces) {
        $migrated.pieces = $pieces
    }
    
    $connections = Migrate-Connections $design.connections
    if ($null -ne $connections) {
        $migrated.connections = $connections
    }
    
    $props = Migrate-Props $design.props
    if ($null -ne $props) {
        $migrated.props = $props
    }
    
    $location = Migrate-Location $design.location
    if ($null -ne $location) {
        $migrated.location = $location
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
    
    # Migrate nested collections
    if ($kit.PSObject.Properties.Name -contains 'types' -and $null -ne $kit.types -and $kit.types.Count -gt 0) {
        $migrated.types = @($kit.types | ForEach-Object { Migrate-Type $_ })
    }
    
    if ($kit.PSObject.Properties.Name -contains 'designs' -and $null -ne $kit.designs -and $kit.designs.Count -gt 0) {
        $migrated.designs = @($kit.designs | ForEach-Object { Migrate-Design $_ })
    }
    
    if ($kit.PSObject.Properties.Name -contains 'qualities' -and $null -ne $kit.qualities -and $kit.qualities.Count -gt 0) {
        $migrated.qualities = @($kit.qualities | ForEach-Object { Migrate-Quality $_ })
    }
    
    if ($kit.PSObject.Properties.Name -contains 'files' -and $null -ne $kit.files -and $kit.files.Count -gt 0) {
        $migrated.files = @($kit.files | ForEach-Object { Migrate-File $_ $kitGuid })
    }
    
    if ($kit.PSObject.Properties.Name -contains 'authors' -and $null -ne $kit.authors -and $kit.authors.Count -gt 0) {
        # Authors might be strings or objects
        if ($kit.authors[0] -is [string]) {
            # Keep as strings (references)
            $migrated.authors = $kit.authors
        } else {
            # Migrate author objects
            $migrated.authors = @($kit.authors | ForEach-Object { Migrate-Author $_ })
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
            $migrated = Migrate-Type $content
        }
        elseif ($FilePath -match 'design_') {
            Write-Host "  Detected: Design" -ForegroundColor Gray
            $migrated = Migrate-Design $content
        }
        else {
            Write-Warning "  Unknown file type, skipping"
            return
        }
        
        if ($null -ne $migrated) {
            # Ensure single-item arrays are preserved
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

foreach ($file in $jsonFiles) {
    Migrate-JsonFile -FilePath $file.FullName -DryRun:$DryRun
}

Write-Host ""
Write-Host "Migration complete!" -ForegroundColor Green
