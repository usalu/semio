#region 🧲️Header
#
# 2026 Ueli Saluz <ueli@semio-tech.com>
#
# Specs: Builds and installs the `repo` launcher on native Windows: the one small native program that provisions the pinned JavaScript runtime itself and starts the repository implementation, so neither `bun` nor a Go toolchain has to be installed by hand. Stage 0 of a bare machine: the pinned Go toolchain is found (PATH, the repository tools partition, the user cache) or fetched from the pins in `🔩️native\🥾️bootstrap\🧬️schema\🔣️.json` (https only, SHA-256 checked before it is unpacked, one atomic rename, progress, Ctrl+C leaves nothing). It needs only Windows PowerShell 5.1 or PowerShell 7 and no elevation.
#
# Summary: Invoke from a checkout with Windows PowerShell 5.1 or PowerShell 7:
# `powershell -NoProfile -ExecutionPolicy Bypass -File "🧰️framework\🛍️products\🦑️repo\🔨️modules\🚀️launcher\📥️install\🔵️.ps1" [-Destination <directory>]`
# The default destination is %LOCALAPPDATA%\semio\launcher\bin; put it on PATH. Environment: SEMIO_GO_ROOT (user cache of the toolchain, default %LOCALAPPDATA%\semio\go), SEMIO_GO_MIRROR (https or loopback http directory that holds the pinned archive).
# Der Launcher baut sich aus dem Checkout; das gepinnte Go holt sich das Skript bei Bedarf selbst (nur https, SHA-256 vor dem Entpacken).
# A running `repo.exe` (the launcher stays the parent of every build on Windows) cannot be overwritten or deleted, only renamed: the old file moves aside as `repo.exe.old-<id>` and the next install removes such copies once nothing runs them.
# Saved as UTF-8 with BOM: Windows PowerShell 5.1 decodes a BOM-less script with the ANSI code page and would turn every emoji path literal into a different path.
#
#endregion 🧲️Header

[CmdletBinding()]
param([string]$Destination = (Join-Path $env:LOCALAPPDATA "semio\launcher\bin"))

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)

$root = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot "..\..\..\..\..\.."))
$pinsFile = Join-Path $root "🧰️framework\🛍️products\🦑️repo\🔨️modules\🔩️native\🥾️bootstrap\🧬️schema\🔣️.json"

function Write-Say {
    param([string]$English, [string]$German)
    Write-Host "[semio] $English"
    Write-Host "[semio] $German"
}

function Stop-Say {
    param([string]$English, [string]$German)
    throw "$English / $German"
}

function Test-PinnedGo {
    param([string]$Path, [string]$Version)
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) { return $false }
    $report = (& $Path version 2>$null | Out-String)
    return $report -match ("go version go" + [Regex]::Escape($Version) + "\s")
}

function Get-GoPin {
    if (-not (Test-Path -LiteralPath $pinsFile -PathType Leaf)) { Stop-Say "The pinned toolchain table is missing: $pinsFile" "Die Tabelle der gepinnten Toolchains fehlt: $pinsFile" }
    $architecture = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
    $architecture = if ($architecture -eq "ARM64") { "arm64" } else { "x64" }
    $assets = (Get-Content -LiteralPath $pinsFile -Raw -Encoding UTF8 | ConvertFrom-Json).definitions.Pins.const.assets
    $pin = $assets | Where-Object { $_.tool -eq "go" -and $_.platform -eq "win32" -and $_.arch -eq $architecture } | Select-Object -First 1
    if ($null -eq $pin) { Stop-Say "No pinned Go for win32/$architecture in $pinsFile." "Kein gepinntes Go für win32/$architecture in $pinsFile." }
    return [pscustomobject]@{ Pin = $pin; Architecture = $architecture }
}

function Get-GoDownloadUrl {
    param($Pin)
    $mirror = $env:SEMIO_GO_MIRROR
    if ([string]::IsNullOrEmpty($mirror)) { return $Pin.url }
    if ($mirror -notmatch "^(https://|http://(127\.0\.0\.1|localhost|\[::1\])([:/]|$))") { Stop-Say "SEMIO_GO_MIRROR must be an https URL or a loopback http URL." "SEMIO_GO_MIRROR muss eine https-URL oder eine Loopback-http-URL sein." }
    return $mirror.TrimEnd("/") + "/" + ($Pin.url -split "/")[-1]
}

function Save-PinnedGoArchive {
    param([string]$Url, [string]$Destination)
    Add-Type -AssemblyName System.Net.Http
    [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
    $client = [System.Net.Http.HttpClient]::new()
    $client.Timeout = [TimeSpan]::FromMinutes(30)
    $stream = $null
    $file = $null
    try {
        $response = $client.GetAsync($Url, [System.Net.Http.HttpCompletionOption]::ResponseHeadersRead).GetAwaiter().GetResult()
        $response.EnsureSuccessStatusCode() | Out-Null
        $total = $response.Content.Headers.ContentLength
        $stream = $response.Content.ReadAsStreamAsync().GetAwaiter().GetResult()
        $file = [IO.File]::Create($Destination)
        $buffer = New-Object byte[] 262144
        $received = 0L
        $reported = [Diagnostics.Stopwatch]::StartNew()
        while (($count = $stream.Read($buffer, 0, $buffer.Length)) -gt 0) {
            $file.Write($buffer, 0, $count)
            $received += $count
            if ($reported.ElapsedMilliseconds -ge 2000) {
                $reported.Restart()
                $of = if ($total) { " / {0:N1} MiB ({1:N0} %)" -f ($total / 1MB), (100 * $received / $total) } else { "" }
                Write-Say ("Received {0:N1} MiB{1}" -f ($received / 1MB), $of) ("Empfangen {0:N1} MiB{1}" -f ($received / 1MB), $of)
            }
        }
    } finally {
        if ($null -ne $file) { $file.Dispose() }
        if ($null -ne $stream) { $stream.Dispose() }
        $client.Dispose()
    }
}

<#
.SYNOPSIS
🔒️ Creates the named mutex that serialises the toolchain install: the machine-wide `Global\` name first, so every session of the user excludes each other, and the `Local\` name of the own session when the account may not create the machine-wide one.
.LINK
https://learn.microsoft.com/dotnet/api/system.threading.mutex
#>
function New-InstallMutex {
    param([string]$Name)
    try { return [Threading.Mutex]::new($false, "Global\$Name") } catch [UnauthorizedAccessException] { return [Threading.Mutex]::new($false, "Local\$Name") }
}

function Install-PinnedGo {
    param($Pin, [string]$Target)
    $version = $Pin.version
    $parent = Split-Path -Parent $Target
    New-Item -ItemType Directory -Path $parent -Force | Out-Null
    $mutex = New-InstallMutex "semio-go-$version"
    $held = $false
    $partial = Join-Path $parent (".partial-" + [Guid]::NewGuid().ToString("N").Substring(0, 12))
    try {
        try { $held = $mutex.WaitOne(0) } catch [Threading.AbandonedMutexException] { $held = $true }
        if (-not $held) {
            Write-Say "Waiting for another process that installs the Go toolchain…" "Warte auf einen anderen Prozess, der die Go-Toolchain installiert…"
            try { $held = $mutex.WaitOne([TimeSpan]::FromMinutes(15)) } catch [Threading.AbandonedMutexException] { $held = $true }
        }
        if (-not $held) { Stop-Say "Another process has been installing the Go toolchain for 15 minutes." "Ein anderer Prozess installiert seit 15 Minuten die Go-Toolchain." }
        if (Test-PinnedGo -Path (Join-Path $Target "bin\go.exe") -Version $version) { return }
        New-Item -ItemType Directory -Path $partial -Force | Out-Null
        $url = Get-GoDownloadUrl -Pin $Pin
        $archive = Join-Path $partial ($url -split "/")[-1]
        Write-Say "Downloading Go $version (pinned, SHA-256 checked): $url" "Lade Go $version herunter (gepinnt, SHA-256 geprüft): $url"
        Save-PinnedGoArchive -Url $url -Destination $archive
        $actual = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($actual -ne $Pin.sha256) { Stop-Say "Checksum mismatch for $url (pinned $($Pin.sha256), received $actual); nothing was unpacked." "Prüfsumme von $url stimmt nicht (gepinnt $($Pin.sha256), erhalten $actual); nichts wurde entpackt." }
        Write-Say "Unpacking Go $version..." "Entpacke Go $version..."
        $unpacked = Join-Path $partial "unpacked"
        New-Item -ItemType Directory -Path $unpacked -Force | Out-Null
        $tar = Get-Command "tar.exe" -ErrorAction SilentlyContinue
        if ($null -ne $tar) {
            & $tar.Source -xf $archive -C $unpacked
            if ($LASTEXITCODE -ne 0) { Stop-Say "The Go archive could not be unpacked." "Das Go-Archiv konnte nicht entpackt werden." }
        } else {
            Expand-Archive -LiteralPath $archive -DestinationPath $unpacked -Force
        }
        if (-not (Test-PinnedGo -Path (Join-Path $unpacked "go\bin\go.exe") -Version $version)) { Stop-Say "The unpacked toolchain does not report Go $version." "Die entpackte Toolchain meldet nicht Go $version." }
        if (Test-Path -LiteralPath $Target) { Remove-Item -LiteralPath $Target -Recurse -Force }
        Move-Item -LiteralPath (Join-Path $unpacked "go") -Destination $Target
        Write-Say "Installed Go ${version}: $Target" "Go $version installiert: $Target"
    } finally {
        if (Test-Path -LiteralPath $partial) { Remove-Item -LiteralPath $partial -Recurse -Force -ErrorAction SilentlyContinue }
        if ($held) { $mutex.ReleaseMutex() }
        $mutex.Dispose()
    }
}

function Get-GoExecutable {
    $found = Get-GoPin
    $pin = $found.Pin
    $version = $pin.version
    $onPath = Get-Command "go.exe" -ErrorAction SilentlyContinue
    if ($null -ne $onPath -and (Test-PinnedGo -Path $onPath.Source -Version $version)) { return $onPath.Source }
    $partition = Join-Path $root (".cache\b\t\m\native\go\$version\win32-$($found.Architecture)-$($pin.sha256.Substring(0, 16))\bin\go.exe")
    if (Test-PinnedGo -Path $partition -Version $version) { return $partition }
    $cache = if ($env:SEMIO_GO_ROOT) { $env:SEMIO_GO_ROOT } else { Join-Path $env:LOCALAPPDATA "semio\go" }
    $target = Join-Path $cache "v$version"
    $executable = Join-Path $target "bin\go.exe"
    if (-not (Test-PinnedGo -Path $executable -Version $version)) { Install-PinnedGo -Pin $pin -Target $target }
    if (-not (Test-PinnedGo -Path $executable -Version $version)) { Stop-Say "The Go toolchain $version is not usable at $executable." "Die Go-Toolchain $version ist unter $executable nicht benutzbar." }
    return $executable
}

$goPath = Get-GoExecutable

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
        $env:GOWORK = "off"; $env:GOTOOLCHAIN = "local"; $env:CGO_ENABLED = "0"
        & $goPath build -trimpath -ldflags "-s -w" -o (Join-Path $work "repo.exe") ./cmd
        if ($LASTEXITCODE -ne 0) { throw "go build failed with exit code $LASTEXITCODE / go build ist mit Exit-Code $LASTEXITCODE fehlgeschlagen" }
    } finally { Pop-Location }

    New-Item -ItemType Directory -Path $Destination -Force | Out-Null
    $target = Join-Path $Destination "repo.exe"
    Get-ChildItem -LiteralPath $Destination -Filter "repo.exe.old-*" -File | ForEach-Object { Remove-Item -LiteralPath $_.FullName -Force -ErrorAction SilentlyContinue }
    Copy-Item -LiteralPath (Join-Path $work "repo.exe") -Destination "$target.partial" -Force
    if (Test-Path -LiteralPath $target) { Move-Item -LiteralPath $target -Destination ("$target.old-" + [Guid]::NewGuid().ToString("N").Substring(0, 12)) }
    Move-Item -LiteralPath "$target.partial" -Destination $target
    Write-Host "[semio] Installed the repo launcher: $target"
    Write-Host "[semio] Repo-Launcher installiert: $target"
    if (($env:Path -split ";") -notcontains $Destination) {
        Write-Host "[semio] Add to PATH: $Destination"
        Write-Host "[semio] Zum PATH hinzufügen: $Destination"
    }
} finally {
    Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue
}
