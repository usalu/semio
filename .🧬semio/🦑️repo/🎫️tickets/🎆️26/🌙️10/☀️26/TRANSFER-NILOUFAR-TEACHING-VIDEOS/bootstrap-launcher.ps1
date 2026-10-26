$ErrorActionPreference = 'Stop'
$workspace = 'C:\git\semio'
$generated = Join-Path $PSScriptRoot '🗑️generated'
$build = Join-Path $generated 'launcher'
$module = Join-Path $workspace '🧰️framework\🛍️products\🦑️repo\🔨️modules\🚀️launcher\📦️packages\🐹️go'
@($build, (Join-Path $build 'cmd'), (Join-Path $build 'tmp'), (Join-Path $build 'cache')) | ForEach-Object { [IO.Directory]::CreateDirectory($_) | Out-Null }
[IO.File]::WriteAllText((Join-Path $build 'go.mod'), "module github.com/usalu/semio/repo/launcher`n`ngo 1.25`n", [Text.UTF8Encoding]::new($false))
$replace = [ordered]@{}
$index = 0
$sources = @(Join-Path $module '🐹️.go') + @(Get-ChildItem -LiteralPath $module -Directory | Where-Object { $_.Name -ne '📦️main' } | ForEach-Object { Join-Path $_.FullName '🐹️.go' } | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf })
foreach ($source in $sources) {
    $index++
    $replace[(Join-Path $build "s$index.go")] = $source
}
$replace[(Join-Path $build 'cmd\main.go')] = Join-Path $module '📦️main\🐹️.go'
$overlay = Join-Path $build 'overlay.json'
[IO.File]::WriteAllText($overlay, (@{ Replace = $replace } | ConvertTo-Json -Depth 5), [Text.UTF8Encoding]::new($false))
$env:GOWORK = 'off'
$env:GOTOOLCHAIN = 'local'
$env:CGO_ENABLED = '0'
$env:GOTELEMETRY = 'off'
$env:GOCACHE = Join-Path $build 'cache'
$env:GOTMPDIR = Join-Path $build 'tmp'
$env:TEMP = $env:GOTMPDIR
$env:TMP = $env:GOTMPDIR
Push-Location $build
try {
    & 'C:\Program Files\Go\bin\go.exe' build "-overlay=$overlay" -trimpath -ldflags '-s -w' -o (Join-Path $build 'repo.exe') './cmd'
    if ($LASTEXITCODE -ne 0) { throw "Launcher build failed: $LASTEXITCODE" }
} finally { Pop-Location }
