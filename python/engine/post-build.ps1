$exePath = "..\..\dotnet\Semio.Grasshopper\bin\Debug\net48\semio-engine.exe"
$internalPath = "..\..\dotnet\Semio.Grasshopper\bin\Debug\net48\_internal"
if (Test-Path $exePath) {
    Remove-Item $exePath
}
if (Test-Path $internalPath) {
    Remove-Item $internalPath -Recurse
}
Move-Item "dist\semio-engine\semio-engine.exe" $exePath
Move-Item "dist\semio-engine\_internal" $internalPath 