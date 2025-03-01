if (Test-Path .\dist\semio_256x256.png) {
    Remove-Item .\dist\semio_256x256.png -Force
}
if (Test-Path .\dist\manifest.yml) {
    Remove-Item .\dist\manifest.yml -Force
}
if (-not (Test-Path .\dist)) {
    New-Item -ItemType Directory -Path .\dist
}
Copy-Item ..\icons\semio_256x256.png .\dist\semio_256x256.png
Copy-Item .\manifest.yml .\dist\manifest.yml
Set-Location .\dist
$yak = "C:\Program Files\Rhino 8\System\Yak.exe"
& $yak build --platform win
Set-Location ..