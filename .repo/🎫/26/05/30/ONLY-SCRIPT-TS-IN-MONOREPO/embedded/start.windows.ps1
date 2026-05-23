# SPDX-License-Identifier: AGPL-3.0-only
$ErrorActionPreference = "Stop"
$Root = $PSScriptRoot
Set-Location $Root
Write-Host "[start.windows] script.ps1 start…"
& (Join-Path $Root "script.ps1") start
