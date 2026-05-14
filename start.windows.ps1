# SPDX-License-Identifier: AGPL-3.0-only
$ErrorActionPreference = "Stop"
$Root = $PSScriptRoot
Set-Location $Root
Write-Host "[start.windows] setup.windows.script.ps1 -SessionStart…"
& (Join-Path $Root "setup.windows.script.ps1") -SessionStart
