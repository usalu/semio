# SPDX-License-Identifier: AGPL-3.0-only
$ErrorActionPreference = "Stop"
$script = Join-Path $PSScriptRoot "setup.windows.script.ps1"
& $script @args
