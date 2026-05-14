# SPDX-License-Identifier: AGPL-3.0-only
$ErrorActionPreference = "Stop"
$Root = $PSScriptRoot
Set-Location $Root

if (-not (Get-Command bun -ErrorAction SilentlyContinue)) {
  Write-Host "[setup.windows] Installing Bun…"
  irm "https://bun.sh/install.ps1" | iex
  $env:PATH = "$env:USERPROFILE\.bun\bin;$env:PATH"
}

Write-Host "[setup.windows] bun install…"
bun install

Write-Host "[setup.windows] bun setup.ts…"
bun setup.ts
