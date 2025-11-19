$kit = Get-Content "assets\semio\kit_metabolism.json" -Raw | ConvertFrom-Json

Write-Host "`n=== CONNECTION VERIFICATION ===" -ForegroundColor Cyan

$totalConnections = 0
$connectionsWithoutPiece = 0
$connectionsWithoutPort = 0

foreach ($design in $kit.designs) {
    if ($design.connections) {
        foreach ($conn in $design.connections) {
            $totalConnections++
            
            # Check connected side
            if (-not $conn.connected.piece) {
                $connectionsWithoutPiece++
                if ($connectionsWithoutPiece -le 3) {
                    Write-Host "Missing piece in connected side of connection $($conn.guid) in design $($design.name)" -ForegroundColor Yellow
                }
            }
            if (-not $conn.connected.port) {
                $connectionsWithoutPort++
                if ($connectionsWithoutPort -le 3) {
                    Write-Host "Missing port in connected side of connection $($conn.guid) in design $($design.name)" -ForegroundColor Yellow
                }
            }
            
            # Check connecting side
            if (-not $conn.connecting.piece) {
                $connectionsWithoutPiece++
                if ($connectionsWithoutPiece -le 3) {
                    Write-Host "Missing piece in connecting side of connection $($conn.guid) in design $($design.name)" -ForegroundColor Yellow
                }
            }
            if (-not $conn.connecting.port) {
                $connectionsWithoutPort++
                if ($connectionsWithoutPort -le 3) {
                    Write-Host "Missing port in connecting side of connection $($conn.guid) in design $($design.name)" -ForegroundColor Yellow
                }
            }
        }
    }
}

Write-Host "`nTotal connections: $totalConnections" -ForegroundColor White
Write-Host "Connection sides missing piece: $connectionsWithoutPiece" -ForegroundColor $(if ($connectionsWithoutPiece -eq 0) { 'Green' } else { 'Red' })
Write-Host "Connection sides missing port: $connectionsWithoutPort" -ForegroundColor $(if ($connectionsWithoutPort -eq 0) { 'Green' } else { 'Red' })

if ($connectionsWithoutPiece -eq 0 -and $connectionsWithoutPort -eq 0) {
    Write-Host "`n✓ SUCCESS: All connection sides have piece and port references!" -ForegroundColor Green
} else {
    Write-Host "`n✗ FAILED: Some connection sides are missing piece or port references" -ForegroundColor Red
}
