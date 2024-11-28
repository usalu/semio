if (Test-Path .\dist) {
    Remove-Item .\dist -Recurse -Force
}
New-Item -ItemType Directory -Path .\dist
Copy-Item ..\icons\semio_256x256.png .\dist\semio_256x256.png
Copy-Item .\manifest.yml .\dist\manifest.yml
Copy-Item ..\dotnet\Semio.Grasshopper\bin\Debug\net48\* .\dist -Recurse
Set-Location .\dist
$yak = "C:\Program Files\Rhino 8\System\Yak.exe"
& $yak build --platform win
Set-Location ..