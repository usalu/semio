$directories = @("logo","icons", "dotnet", "python\engine", "yak")

foreach ($dir in $directories) {
    Set-Location $dir
    .\build.ps1
    Set-Location ..
}