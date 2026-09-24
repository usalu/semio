"""Z2 one-off codemod: Windows bootstrap — UTF-8 BOM, UTF-8 console, long-path guidance, no user-global cargo/git config."""
import sys
P="🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1"
A="🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🖥️associations/🪟️windows/🔵️.ps1"
BOM="﻿"
s=open(P,encoding='utf-8').read()
def rep(a,b,n=1):
    global s
    c=s.count(a)
    if c!=n: sys.exit(f"count {c}!={n}: {a[:90]!r}")
    s=s.replace(a,b)
rep("# Summary: Windows-native bootstrap for the compose monorepo. Invoke `.\\🪟️script.ps1 setup` (full) or `.\\🪟️script.ps1 start` (IDE session).",
    "# Summary: Windows-native bootstrap for the compose monorepo. Invoke from a fresh clone with Windows PowerShell 5.1 or PowerShell 7:\n# `powershell -NoProfile -ExecutionPolicy Bypass -File \"🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\🔩️native\\🥾️bootstrap\\🔵️.ps1\" setup` (full) or `… start`\n# (IDE session); `bun ./📜️script.ts setup native` routes here once Bun exists. Saved as UTF-8 with BOM: Windows PowerShell 5.1\n# decodes a BOM-less script with the ANSI code page and would turn every emoji path literal below into a different path.")
rep("""Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
""","""Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
""")
rep("""function Configure-GitSafeDirectories {
    param([string]$RepoRoot)

    & git config --global --add safe.directory $RepoRoot | Out-Null
    $gitmodulesPath = Join-Path $RepoRoot ".gitmodules"
    if (-not (Test-Path -LiteralPath $gitmodulesPath)) {
        return
    }

    $submodulePaths = & git config -f $gitmodulesPath --get-regexp '^submodule\\..*\\.path$' 2>$null
    foreach ($line in $submodulePaths) {
        $parts = $line -split "\\s+", 2
        if ($parts.Count -eq 2 -and $parts[1]) {
            & git config --global --add safe.directory (Join-Path $RepoRoot $parts[1]) | Out-Null
        }
    }
}
""","""function Test-WindowsLongPathsEnabled {
    $setting = Get-ItemProperty -LiteralPath "HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem" -Name "LongPathsEnabled" -ErrorAction SilentlyContinue
    return ($null -ne $setting) -and ($setting.LongPathsEnabled -eq 1)
}

function Write-WindowsLongPathsGuidance {
    if (Test-WindowsLongPathsEnabled) {
        Write-Step "Windows long paths (LongPathsEnabled) are enabled."
        return
    }
    $enable = 'New-ItemProperty -Path "HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem" -Name LongPathsEnabled -Value 1 -PropertyType DWORD -Force'
    Write-Warning "[en] Windows long paths are disabled. This repository's fixtures and Rust build outputs reach 240+ characters below the clone root, so Python, CMake and MSVC steps fail at 260. Run once in an elevated PowerShell, then open a new terminal: $enable"
    Write-Warning "[de] Lange Windows-Pfade sind deaktiviert. Fixtures und Rust-Build-Ausgaben dieses Repositorys erreichen unterhalb des Klon-Verzeichnisses 240+ Zeichen, daher scheitern Python-, CMake- und MSVC-Schritte ab 260. Einmal in einer PowerShell mit Administratorrechten ausführen und danach ein neues Terminal öffnen: $enable"
}
""")
rep("""$repoRoot = Get-RepoRoot
Set-Location $repoRoot
""","""$repoRoot = Get-RepoRoot
Set-Location $repoRoot
Write-WindowsLongPathsGuidance
""")
rep("""    Invoke-RepoCommand -FilePath $rustupPath -ArgumentList @("target", "add", "wasm32-unknown-unknown") -WorkingDirectory $repoRoot
    $cargoConfigPath = Join-HomePath @(".cargo", "config.toml")
    @"
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
"@ | Set-Content -Path $cargoConfigPath -Encoding UTF8
""","""    Invoke-RepoCommand -FilePath $rustupPath -ArgumentList @("toolchain", "install") -WorkingDirectory $repoRoot
""")
rep("""    Configure-GitSafeDirectories -RepoRoot $repoRoot
    Stop-RepoPythonProcesses""","""    Stop-RepoPythonProcesses""")
rep("""Write-Step "Native bootstrap complete. Open a new shell to pick up the persisted PATH/env vars."
""","""Write-WindowsLongPathsGuidance
Write-Step "Native bootstrap complete. Open a new shell to pick up the persisted PATH/env vars."
""")
assert not s.startswith(BOM)
open(P,'w',encoding='utf-8',newline='').write(BOM+s)
a=open(A,encoding='utf-8').read()
if not a.startswith(BOM): open(A,'w',encoding='utf-8',newline='').write(BOM+a)
print("ok")
