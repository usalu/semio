# 🌳 Root and bootstrap: Go aligned with its pin, the C++ lane keyed by real inputs, CMake and Ninja no longer taken from the host package managers (the repository provisions its pinned ones), the .NET SDK installed at exactly the version global.json pins on every operating system.
import json, os
from layout import *
from patchlib import chain

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.environ.get("LC_STAGED") or os.path.join(HERE, "staged")
BS = REPO + "/🔩️native/🥾️bootstrap"
SH = BS + "/🐚️.sh"
PS1 = BS + "/🔵️.ps1"
PINS = BS + "/🧬️schema/🔣️.json"
FIXTURE = L + "/🧪️tests/🧫️fixtures/🥾️cross-platform-bootstrap/🔣️.json"
GO_WORK = ROOT + "/go.work"
PROJECT = ROOT + "/📋️project.json"
DOCKERFILE = ROOT + "/.devcontainer/Dockerfile"
DEVCONTAINER = ROOT + "/.devcontainer/devcontainer.json"
SCRIPT = 'bun \\"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts\\" fingerprint cmake'
INSTALL = "5147e32300a8e908f5d737c8cff63a76b4b63531"
INSTALL_PS1_SHA = "bb1ce92f4397e24d4736a4658b9728fb8f9db64a0d3f8e636ba408a866a6661d"
INSTALL_SH_ROW_END = '/src/dotnet-install.sh","sha256":"082f7685e156738a1b2e2ed8381a621870d4ce8e8c59278034556f05c186eb2e"}\n        ]'

def cmake_environment():
    return json.load(open(STAGED + "/toolchains-policy.json", encoding="utf8"))["cmake"]["environment"]

def go_work(text):
    return chain(text, [("go-work", "go 1.25\n\nuse (\n", "go 1.26.9\n\nuse (\n")])

def project(text):
    names = ",\n".join('      { "env": "%s" }' % name for name in cmake_environment())
    declared = ('    ],\n    "cmakeSources": [\n      "{workspaceRoot}/CMakeLists.txt",\n      "{workspaceRoot}/CMakePresets.json",\n      "{workspaceRoot}/**/*.{c,cc,cpp,cxx,h,hh,hpp,hxx,ixx}"\n    ],\n'
                '    "cmakeToolchain": [\n      { "runtime": "' + SCRIPT + '" },\n' + names + '\n    ]\n  },\n  "targets": {\n    "test-workspace-build": {')
    return chain(text, [
        ("project-named-inputs", '      "{workspaceRoot}/🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts"\n    ]\n  },\n  "targets": {\n    "test-workspace-build": {',
         '      "{workspaceRoot}/🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts"\n' + declared),
        ("project-cpp-custody", '    "cpp-custody": {\n      "executor": "repo:script",\n      "cache": true,\n      "dependsOn": [],\n      "outputs": [],\n',
         '    "cpp-custody": {\n      "executor": "repo:script",\n      "cache": true,\n      "dependsOn": [],\n      "outputs": [],\n      "inputs": ["cmakeSources", "cmakeToolchain", "sharedGlobals"],\n'),
        ("project-cpp-test-toolchain", '    "cpp-test-toolchain": {\n      "executor": "repo:script",\n      "cache": true,\n      "options": {',
         '    "cpp-test-toolchain": {\n      "executor": "repo:script",\n      "cache": true,\n      "inputs": ["cmakeSources", "cmakeToolchain", "sharedGlobals"],\n      "options": {'),
    ])

def pins(text):
    ps1 = lambda arch: '{"tool":"dotnet-install-powershell","version":"%s","platform":"win32","arch":"%s","url":"https://raw.githubusercontent.com/dotnet/install-scripts/%s/src/dotnet-install.ps1","sha256":"%s"}' % (INSTALL, arch, INSTALL, INSTALL_PS1_SHA)
    return chain(text, [
        ("pins-min", '      "minItems": 25,\n      "maxItems": 25,\n', '      "minItems": 27,\n      "maxItems": 27,\n'),
        ("pins-enum", '            "dotnet-install"\n          ]', '            "dotnet-install",\n            "dotnet-install-powershell"\n          ]'),
        ("pins-rows", INSTALL_SH_ROW_END, '/src/dotnet-install.sh","sha256":"082f7685e156738a1b2e2ed8381a621870d4ce8e8c59278034556f05c186eb2e"},\n          ' + ps1("x64") + ',\n          ' + ps1("arm64") + '\n        ]'),
    ])

def sh(text):
    return chain(text, [
        ("sh-apt", "  apt-get) printf '%s\\n' build-essential cmake ninja-build pkg-config uuid-dev", "  apt-get) printf '%s\\n' build-essential pkg-config uuid-dev"),
        ("sh-dnf", "  dnf) printf '%s\\n' gcc gcc-c++ make cmake ninja-build pkgconf-pkg-config", "  dnf) printf '%s\\n' gcc gcc-c++ make pkgconf-pkg-config"),
        ("sh-pacman", "  pacman) printf '%s\\n' base-devel cmake ninja pkgconf", "  pacman) printf '%s\\n' base-devel pkgconf"),
        ("sh-zypper", "  zypper) printf '%s\\n' gcc gcc-c++ make cmake ninja pkg-config", "  zypper) printf '%s\\n' gcc gcc-c++ make pkg-config"),
        ("sh-brew", '      brew install cmake ninja pkg-config || log "Homebrew could not install cmake, ninja and pkg-config."\n', '      brew install pkg-config || log "Homebrew could not install pkg-config."\n'),
        ("sh-brew-missing", '      log "Homebrew not available; install CMake, Ninja and pkg-config before running native C++ builds."\n', '      log "Homebrew not available; install pkg-config before running native C++ builds (CMake and Ninja are provisioned by the repository)."\n'),
        ("sh-linux-hint", 'install a C/C++ compiler, make, CMake, Ninja, pkg-config, libuuid, ICU, curl, git and unzip."', 'install a C/C++ compiler, make, pkg-config, libuuid, ICU, curl, git and unzip (CMake and Ninja are provisioned by the repository)."'),
        ("sh-report", "  report_missing_tools cc c++ make cmake ninja pkg-config curl git unzip\n", "  report_missing_tools cc c++ make pkg-config curl git unzip\n"),
        ("sh-channel", 'DOTNET_CHANNEL="${DOTNET_CHANNEL:-8.0}"\n', ''),
        ("sh-dotnet", '''ensure_dotnet() {
  export DOTNET_ROOT="${DOTNET_ROOT:-$HOME/.dotnet}" DOTNET_CLI_TELEMETRY_OPTOUT=1
  export PATH="$LOCAL_BIN:$PATH"
  if command -v dotnet >/dev/null 2>&1 && dotnet --list-runtimes 2>/dev/null | grep -q "^Microsoft.NETCore.App ${DOTNET_CHANNEL}\\."; then
    return 0
  fi
  local row installer
''', '''dotnet_sdk_pin() {
  sed -n 's/^[[:space:]]*"version"[[:space:]]*:[[:space:]]*"\\([0-9][0-9.]*\\)".*/\\1/p' "$REPO_ROOT/global.json" | sed -n '1p'
}

ensure_dotnet() {
  export DOTNET_ROOT="${DOTNET_ROOT:-$HOME/.dotnet}" DOTNET_CLI_TELEMETRY_OPTOUT=1
  export PATH="$LOCAL_BIN:$PATH"
  local row installer sdk
  sdk="$(dotnet_sdk_pin)"
  [ -n "$sdk" ] || {
    log "No SDK version pinned in $REPO_ROOT/global.json."
    return 1
  }
  if command -v dotnet >/dev/null 2>&1 && dotnet --list-sdks 2>/dev/null | grep -q "^${sdk} "; then
    return 0
  fi
'''),
        ("sh-dotnet-install", '''  log "Installing .NET SDK $DOTNET_CHANNEL into $DOTNET_ROOT…"''', '''  log "Installing .NET SDK $sdk into $DOTNET_ROOT…"'''),
        ("sh-dotnet-version", '''  bash "$installer" --channel "$DOTNET_CHANNEL" --install-dir "$DOTNET_ROOT" --no-path || {''', '''  bash "$installer" --version "$sdk" --install-dir "$DOTNET_ROOT" --no-path || {'''),
        ("sh-dotnet-main", '''ensure_dotnet || log ".NET SDK ${DOTNET_CHANNEL} provisioning failed; install it from https://dot.net and rerun setup."''', '''ensure_dotnet || log ".NET SDK provisioning failed; install the SDK version pinned in $REPO_ROOT/global.json from https://dot.net and rerun setup."'''),
    ])

PS1_FUNCTION = '''function Test-DotnetSdkInstalled {
    param([string]$Version)

    $dotnet = Get-FirstCommandPath @("dotnet.exe")
    if (-not $dotnet) { return $false }
    $sdks = & $dotnet "--list-sdks" 2>$null
    return [bool]($sdks | Where-Object { $_ -match ("^" + [Regex]::Escape($Version) + " ") })
}

function Install-PinnedDotnetSdk {
    param([string]$RepoRoot)

    $sdk = (Get-Content -LiteralPath (Join-Path $RepoRoot "global.json") -Raw -Encoding UTF8 | ConvertFrom-Json).sdk.version
    $dotnetRoot = Join-Path $env:LOCALAPPDATA "Microsoft\\dotnet"
    if (-not (Test-DotnetSdkInstalled -Version $sdk)) {
        $asset = Get-PinnedAsset -Tool "dotnet-install-powershell"
        $scratch = New-ScratchDirectory
        try {
            $installer = Join-Path $scratch "dotnet-install.ps1"
            Save-VerifiedDownload -Url $asset.url -Sha256 $asset.sha256 -Destination $installer
            Write-Step "Installing .NET SDK $sdk..."
            & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $installer -Version $sdk -InstallDir $dotnetRoot -NoPath | Out-Host
            if ($LASTEXITCODE -ne 0) { throw "dotnet-install failed with exit code $LASTEXITCODE" }
        } finally {
            Remove-Item -LiteralPath $scratch -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
    return $dotnetRoot
}

'''

def ps1(text):
    return chain(text, [
        ("ps1-functions", "function Install-PinnedToolchain {\n", PS1_FUNCTION + "function Install-PinnedToolchain {\n"),
        ("ps1-return", "    }\n    return $paths\n}\n\nfunction Install-UvTool {", "    }\n    $paths += (Install-PinnedDotnetSdk -RepoRoot $RepoRoot)\n    return $paths\n}\n\nfunction Install-UvTool {"),
        ("ps1-cmake", '    Sync-WingetPackage -Id "Kitware.CMake" -Label "CMake"\n    Sync-WingetPackage -Id "Ninja-build.Ninja" -Label "Ninja"\n', ''),
        ("ps1-sdk8", '    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.8" -Label ".NET SDK 8.0"\n', ''),
    ])

def fixture(text):
    return chain(text, [("fixture-required-tools", '"requiredTools": ["cc", "c++", "make", "cmake", "ninja", "pkg-config", "curl", "git", "unzip"]', '"requiredTools": ["cc", "c++", "make", "pkg-config", "curl", "git", "unzip"]')])

def dockerfile(text):
    return chain(text, [("docker-cmake", "    build-essential \\\n    cmake \\\n    ninja-build \\\n    pkg-config \\\n", "    build-essential \\\n    pkg-config \\\n")])

def devcontainer(text):
    return chain(text, [("devcontainer-dotnet", '    "ghcr.io/devcontainers/features/dotnet:2": {\n      "version": "8.0",\n', '    "ghcr.io/devcontainers/features/dotnet:2": {\n      "version": "8.0.421",\n')])

def edits():
    return [(GO_WORK, go_work), (PROJECT, project), (PINS, pins), (SH, sh), (PS1, ps1), (FIXTURE, fixture), (DOCKERFILE, dockerfile), (DEVCONTAINER, devcontainer)]

DOTNET_SCRIPT = ROOT + "/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts"

def dotnet_script(text):
    return chain(text, [
        ("dotnet-imports", 'import { createHash } from "node:crypto";\nimport { spawnSync } from "node:child_process";\n', 'import { toolchainIdentityV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";\n'),
        ("dotnet-state", '`${process.platform}-${process.arch}-${createHash("sha256").update(spawnSync("dotnet",["--version"],{encoding:"utf8"}).stdout??"").digest("hex").slice(0,12)}`', '`${process.platform}-${process.arch}-${toolchainIdentityV1("dotnet").digest.slice(0,12)}`'),
    ])

_edits = edits
def edits():
    return _edits() + [(DOTNET_SCRIPT, dotnet_script)]

GO_PLAN = L + "/🧪️testing/🐹️go-plan/🟦️.ts"
GO_IDENTITY = BZ + "/🐹️go/🟦️.ts"
NATIVE_LIB = "../../⚡️caching/🚀️bootstrap/🛠️tools/🧰️native"

def go_plan(text):
    return chain(text, [
        ("go-plan-import", 'import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";\n', 'import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";\nimport { pinnedGoEnvironmentV1 } from "' + NATIVE_LIB + '/🟦️.ts";\nimport { systemNativeToolPortV1 } from "' + NATIVE_LIB + '/🔌️port/🟦️.ts";\nimport { repositoryCachePartitionRootV1 } from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";\n'),
        ("go-plan-env", '    await runRepositoryTestCommand("go", ["test", `-overlay=${overlay}`, ...args, ...packages], { cwd: moduleRoot, env: { ...(opts.env ?? process.env), SEMIO_GO_OVERLAY_OWNER: owner }, budgetMs: opts.budgetMs, signal: opts.signal, throwOnFailure: true });',
         '    const root = getWorkspaceRoot(), base = opts.env ?? process.env, pinned = await pinnedGoEnvironmentV1(base, repositoryCachePartitionRootV1(root, "tools"), systemNativeToolPortV1(root, base.CARGO_NET_OFFLINE === "true"), opts.signal ?? new AbortController().signal);\n    await runRepositoryTestCommand("go", ["test", `-overlay=${overlay}`, ...args, ...packages], { cwd: moduleRoot, env: { ...pinned, SEMIO_GO_OVERLAY_OWNER: owner }, budgetMs: opts.budgetMs, signal: opts.signal, throwOnFailure: true });'),
        ("go-build-environment", "environment = repoToolCacheEnv(root, opts.env ?? process.env), plan = canonicalGoPlan(moduleRoot);", "environment = await pinnedGoEnvironmentV1(repoToolCacheEnv(root, opts.env ?? process.env), repositoryCachePartitionRootV1(root, \"tools\"), systemNativeToolPortV1(root, (opts.env ?? process.env).CARGO_NET_OFFLINE === \"true\"), control.signal), plan = canonicalGoPlan(moduleRoot);"),
    ])

def go_identity(text):
    return chain(text, [
        ("go-identity-import", 'import {taskFamilyV1} from "../../../🏃️execution/🟦️.ts";\n', 'import {taskFamilyV1} from "../../../🏃️execution/🟦️.ts";\nimport {nativeToolBinaryDirectoryV1,toolchainHostV1,toolchainRootsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";\n'),
        ("go-identity-binary", ' const name=platform.startsWith("win32")?policy.binary.windows:policy.binary.posix,candidates=[...(environment.GOROOT?[join(environment.GOROOT,"bin",name)]:[]),...(environment.PATH??"").split(delimiter).filter(Boolean).map(directory=>join(directory,name))];\n',
         ' const name=platform.startsWith("win32")?policy.binary.windows:policy.binary.posix,[system,architecture]=platform.split("-"),pinned=environment.SEMIO_WORKSPACE_ROOT===undefined?undefined:join(nativeToolBinaryDirectoryV1("go",toolchainHostV1(system,architecture),toolchainRootsV1(environment.SEMIO_WORKSPACE_ROOT).tools),name),candidates=pinned!==undefined?[pinned]:[...(environment.GOROOT?[join(environment.GOROOT,"bin",name)]:[]),...(environment.PATH??"").split(delimiter).filter(Boolean).map(directory=>join(directory,name))];\n'),
    ])

SH_GO = """go_version() {
  go version 2>/dev/null | sed -nE 's/^go version go([0-9]+\\.[0-9]+(\\.[0-9]+)?).*/\\1/p'
}

ensure_go() {
  export PATH="$LOCAL_BIN:$PATH"
  local row version installed archive root download
  row="$(host_pin go)" || return 1
  version="$(pin_field "$row" version)"
  installed="$(go_version)"
  if [ -n "$installed" ] && version_at_least "$installed" "$version"; then
    return 0
  fi
  archive="$(pin_field "$row" url)"
  archive="${archive##*/}"
  root="${XDG_DATA_HOME:-$HOME/.local/share}/semio/go/go$version"
  if [ ! -x "$root/go/bin/go" ]; then
    log "Installing Go $version into $root…"
    download="$(mktemp -d)"
    fetch_pinned "$row" "$download/$archive" || {
      rm -rf "$download"
      return 1
    }
    mkdir -p "$root" && tar -xzf "$download/$archive" -C "$root" || {
      rm -rf "$download"
      return 1
    }
    rm -rf "$download"
  fi
  mkdir -p "$LOCAL_BIN"
  ln -sf "$root/go/bin/go" "$LOCAL_BIN/go"
  ln -sf "$root/go/bin/gofmt" "$LOCAL_BIN/gofmt"
}

"""
PS1_GO = """    $go = Get-PinnedAsset -Tool "go"
    $goRoot = Join-Path $toolsRoot "go\\go$($go.version)"
    $installedGo = Get-InstalledVersion "go.exe"
    if ($null -eq $installedGo -or [version]$installedGo -lt [version]$go.version) {
        Write-Step "Installing Go $($go.version)..."
        Install-PinnedDirectory -Asset $go -Destination $goRoot -Child "go"
    }
    if (Test-Path -LiteralPath $goRoot) { $paths += (Join-Path $goRoot "bin") }

"""

def sh_go(text):
    return chain(text, [
        ("sh-go-functions", SH_GO, ""),
        ("sh-go-main", 'ensure_go || log "Go provisioning failed; install the Go version pinned in $PINS_FILE from https://go.dev/dl and rerun setup."\n', ""),
        ("sh-go-header", "rustup with the `rust-toolchain.toml` pin, Go, .NET, and uv,", "rustup with the `rust-toolchain.toml` pin, .NET, and uv (Go is provisioned by the repository into its tools partition),"),
    ])

def ps1_go(text):
    return chain(text, [
        ("ps1-go-block", PS1_GO, ""),
        ("ps1-go-header", "installs Bun, Node.js, Go, uv and rustup from checksum-pinned archives", "installs Bun, Node.js, uv and rustup from checksum-pinned archives (Go is provisioned by the repository into its tools partition)"),
    ])

_edits_go = edits
def edits():
    return _edits_go() + [(GO_PLAN, go_plan), (GO_IDENTITY, go_identity), (SH, sh_go), (PS1, ps1_go)]
