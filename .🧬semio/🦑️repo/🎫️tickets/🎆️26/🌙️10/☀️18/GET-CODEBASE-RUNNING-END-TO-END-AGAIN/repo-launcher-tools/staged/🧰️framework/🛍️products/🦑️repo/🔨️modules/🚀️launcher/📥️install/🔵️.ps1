#region 🧲️Header
#
# 2026 Ueli Saluz <ueli@semio-tech.com>
#
# Specs: Builds and installs the `repo` launcher on native Windows: the one small native program that provisions the pinned JavaScript runtime itself and starts the repository implementation, so no `bun` has to be installed. It needs only Go (the native bootstrap installs the pinned Go) and no elevation.
#
# Summary: Invoke from a checkout with Windows PowerShell 5.1 or PowerShell 7:
# `powershell -NoProfile -ExecutionPolicy Bypass -File "🧰️framework\🛍️products\🦑️repo\🔨️modules\🚀️launcher\📥️install\🔵️.ps1" [-Destination <directory>]`
# The default destination is %LOCALAPPDATA%\semio\launcher\bin; put it on PATH. Der Launcher baut sich aus dem Checkout; Voraussetzung ist nur Go.
# Saved as UTF-8 with BOM: Windows PowerShell 5.1 decodes a BOM-less script with the ANSI code page and would turn every emoji path literal into a different path.
#
#endregion 🧲️Header

[CmdletBinding()]
param([string]$Destination = (Join-Path $env:LOCALAPPDATA "semio\launcher\bin"))

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)

$go = Get-Command "go.exe" -ErrorAction SilentlyContinue
if ($null -eq $go) {
    throw "Go 1.25 or newer is required to build the launcher; the native bootstrap installs it. / Go 1.25 oder neuer ist zum Bauen des Launchers nötig; das native Bootstrap installiert es."
}

$module = Join-Path $PSScriptRoot "..\📦️packages\🐹️go"
$work = Join-Path ([IO.Path]::GetTempPath()) ("semio-launcher-" + [Guid]::NewGuid().ToString("N").Substring(0, 12))
New-Item -ItemType Directory -Path (Join-Path $work "cmd") -Force | Out-Null
try {
    [IO.File]::WriteAllText((Join-Path $work "go.mod"), "module github.com/usalu/semio/repo/launcher`n`ngo 1.25`n", [Text.UTF8Encoding]::new($false))
    $index = 0
    $sources = @(Join-Path $module "🐹️.go") + @(Get-ChildItem -LiteralPath $module -Directory | Where-Object { $_.Name -ne "📦️main" } | ForEach-Object { Join-Path $_.FullName "🐹️.go" } | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf })
    foreach ($source in $sources) {
        $index++
        Copy-Item -LiteralPath $source -Destination (Join-Path $work "s$index.go")
    }
    Copy-Item -LiteralPath (Join-Path $module "📦️main\🐹️.go") -Destination (Join-Path $work "cmd\main.go")

    Push-Location $work
    try {
        $env:GOWORK = "off"; $env:CGO_ENABLED = "0"
        & $go.Source build -trimpath -ldflags "-s -w" -o (Join-Path $work "repo.exe") ./cmd
        if ($LASTEXITCODE -ne 0) { throw "go build failed with exit code $LASTEXITCODE / go build ist mit Exit-Code $LASTEXITCODE fehlgeschlagen" }
    } finally { Pop-Location }

    New-Item -ItemType Directory -Path $Destination -Force | Out-Null
    $target = Join-Path $Destination "repo.exe"
    Copy-Item -LiteralPath (Join-Path $work "repo.exe") -Destination "$target.partial" -Force
    Move-Item -LiteralPath "$target.partial" -Destination $target -Force
    Write-Host "[semio] Installed the repo launcher: $target"
    Write-Host "[semio] Repo-Launcher installiert: $target"
    if (($env:Path -split ";") -notcontains $Destination) {
        Write-Host "[semio] Add to PATH: $Destination"
        Write-Host "[semio] Zum PATH hinzufügen: $Destination"
    }
} finally {
    Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue
}
