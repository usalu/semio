# i18n-full.ps1
# Complete i18n validation and fix process

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

Write-Host "🔍 Running complete i18n process..." -ForegroundColor Cyan
Write-Host ""

# Step 1: Validate and generate report
Write-Host "Step 1: Validating i18n entries..." -ForegroundColor Yellow
& ".\scripts\i18n.ps1"

# Step 2: Remove unused keys
Write-Host "`nStep 2: Removing unused keys..." -ForegroundColor Yellow
node .\scripts\i18n-fix.mjs

# Step 3: Add missing entries  
Write-Host "`nStep 3: Adding missing entries..." -ForegroundColor Yellow
node .\scripts\i18n-add.mjs

# Step 4: Final validation
Write-Host "`nStep 4: Final validation..." -ForegroundColor Yellow
& ".\scripts\i18n.ps1"

Write-Host "`n✅ I18N process complete!" -ForegroundColor Green
