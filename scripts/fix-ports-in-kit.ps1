param(
    [string]$KitPath = "assets\semio\kit_metabolism.json"
)

Write-Host "Fixing ports in kit: $KitPath" -ForegroundColor Cyan

$kit = Get-Content -Path $KitPath -Raw | ConvertFrom-Json

$fixedCount = 0

foreach ($type in $kit.types) {
    if ($type.PSObject.Properties.Name -contains 'ports' -and $null -ne $type.ports) {
        if ($type.ports -isnot [Array]) {
            $type.ports = @($type.ports)
            $fixedCount++
        }
    }
}

Write-Host "Fixed $fixedCount types with non-array ports" -ForegroundColor Green

# Write back to file
$json = $kit | ConvertTo-Json -Depth 100 -Compress:$false
Set-Content -Path $KitPath -Value $json -Encoding UTF8

Write-Host "✓ Kit file updated successfully" -ForegroundColor Green
