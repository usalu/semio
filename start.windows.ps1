# SPDX-License-Identifier: AGPL-3.0-only
$ErrorActionPreference = "Stop"
$Root = $PSScriptRoot
Set-Location $Root
Write-Host "[start.windows] bun start.script.ts…"
bun start.script.ts
