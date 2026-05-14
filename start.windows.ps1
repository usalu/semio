# SPDX-License-Identifier: AGPL-3.0-only
$ErrorActionPreference = "Stop"
$Root = $PSScriptRoot
Set-Location $Root
Write-Host "[start.windows] bun start.ts…"
bun start.ts
