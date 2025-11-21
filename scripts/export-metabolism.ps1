# Export Metabolism Kit to assets/metabolism.zip
# This script runs the kit import/export test with EXPORT_TO_ASSETS flag

Write-Host "Exporting Metabolism Kit..." -ForegroundColor Cyan

# Set environment variable and run test
$env:EXPORT_TO_ASSETS = "true"
cd js/js
npx vitest run --no-coverage -t "roundtrip export and import"
cd ../..

if ($LASTEXITCODE -eq 0) {
    if (Test-Path "assets/metabolism.zip") {
        $size = (Get-Item "assets/metabolism.zip").Length / 1KB
        Write-Host "✓ Successfully exported metabolism.zip ($([math]::Round($size, 2)) KB)" -ForegroundColor Green
    } else {
        Write-Host "✗ File not created" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "✗ Export failed" -ForegroundColor Red
    exit 1
}
