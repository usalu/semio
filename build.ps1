$directories = @("logo", "icons", "dotnet", "python\engine", "yak")

foreach ($dir in $directories) {
    Set-Location $dir
    .\build.ps1
    $depth = $dir.Split('\').Length
    for ($i = 0; $i -lt $depth; $i++) {
        Set-Location ..
    }
}