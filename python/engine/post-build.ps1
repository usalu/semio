$exePath = "dist\semio-engine\semio-engine.exe"
$internalPath = "dist\semio-engine\_internal"
$grasshopperBinPath = "..\..\dotnet\Semio.Grasshopper\bin\Debug\net48"
$grasshopperExePath = "$grasshopperBinPath\semio-engine.exe"
$grasshopperInternalPath = "$grasshopperBinPath\_internal"
if (Test-Path $grasshopperExePath) {
    Remove-Item $grasshopperExePath
}
if (Test-Path $grasshopperInternalPath) {
    Remove-Item $grasshopperInternalPath -Recurse
}
Move-Item $exePath $grasshopperExePath
Move-Item  $internalPath $grasshopperInternalPath