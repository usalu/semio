param (
    [switch]$SkipPostBuild
)
if (-not (Test-Path ".venv")) {
    ux sync
}
.venv/Scripts/activate.ps1
.\generate-schemas.ps1
if (Test-Path "build") {
    Remove-Item "build" -Recurse
}
if (Test-Path "dist") {
    Remove-Item "dist" -Recurse
}
pyinstaller `
    --name "semio-engine" `
    --windowed `
    --clean `
    --noconfirm `
    --copy-metadata graphene `
    --copy-metadata sqlalchemy `
    --copy-metadata loguru `
    --hidden-import=loguru `
    --add-data "../../assets/icons/semio_512x512.png;icons/" `
    --icon "../../assets/icons/semio.ico" `
    engine.py

if (-not $SkipPostBuild) {
    .\post-build.ps1
}