param(
    [string]$AssetsDir = "assets\semio",
    [string]$KitName = "kit_metabolism.json"
)

Write-Host "Assembling kit from individual files..." -ForegroundColor Cyan

# Initialize kit structure
$kit = @{
    guid = "f9f9f9f9-f9f9-f9f9-f9f9-f9f9f9f9f9f9"
    name = "Metabolism"
    createdAt = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
    updatedAt = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
    types = @()
    designs = @()
}

# Load all type files
$typeFiles = Get-ChildItem -Path $AssetsDir -Filter "type_*.json" | Sort-Object Name
Write-Host "Loading $($typeFiles.Count) type files..." -ForegroundColor White

foreach ($file in $typeFiles) {
    try {
        $type = Get-Content -Path $file.FullName -Raw | ConvertFrom-Json
        
        # Fix ports: if ports is a single object, convert to array
        if ($type.PSObject.Properties.Name -contains 'ports' -and $null -ne $type.ports) {
            if ($type.ports -isnot [Array]) {
                $type.ports = @($type.ports)
            }
        }
        
        $kit.types += $type
        Write-Host "  ✓ Loaded type: $($type.name)" -ForegroundColor Green
    } catch {
        Write-Warning "Failed to load type file: $($file.Name)"
    }
}

# Load all design files (excluding _flat versions)
$designFiles = Get-ChildItem -Path $AssetsDir -Filter "design_*.json" | 
    Where-Object { $_.Name -notmatch "_flat\.json$" } | 
    Sort-Object Name

Write-Host "Loading $($designFiles.Count) design files..." -ForegroundColor White

foreach ($file in $designFiles) {
    try {
        $design = Get-Content -Path $file.FullName -Raw | ConvertFrom-Json
        $kit.designs += $design
        Write-Host "  ✓ Loaded design: $($design.name)" -ForegroundColor Green
    } catch {
        Write-Warning "Failed to load design file: $($file.Name)"
    }
}

# Write kit file
$kitPath = Join-Path $AssetsDir $KitName
$json = $kit | ConvertTo-Json -Depth 100 -Compress:$false
Set-Content -Path $kitPath -Value $json -Encoding UTF8

Write-Host "" -ForegroundColor White
Write-Host "✓ Kit assembled successfully: $kitPath" -ForegroundColor Green
Write-Host "  Types: $($kit.types.Count)" -ForegroundColor Gray
Write-Host "  Designs: $($kit.designs.Count)" -ForegroundColor Gray
