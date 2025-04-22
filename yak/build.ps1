if (Test-Path .\dist\semio_512x512.png) {
    Remove-Item .\dist\semio_512x512.png -Force
}
if (Test-Path .\dist\manifest.yml) {
    Remove-Item .\dist\manifest.yml -Force
}
if (-not (Test-Path .\dist)) {
    New-Item -ItemType Directory -Path .\dist
}
Copy-Item ..\assets\icons\semio_512x512.png .\dist\semio_512x512.png
Copy-Item .\manifest.yml .\dist\manifest.yml
Set-Location .\dist
$yak = "C:\Program Files\Rhino 8\System\Yak.exe"
& $yak build --platform win
Set-Location ..