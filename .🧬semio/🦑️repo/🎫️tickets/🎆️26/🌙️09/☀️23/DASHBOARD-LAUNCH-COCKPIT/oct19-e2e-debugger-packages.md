# Native Debugger Provisioning Evidence

Researched 2026-10-19. Package facts and design inputs are not cross-platform runtime results. Actual setup belongs in the existing owned developer-tools script, with Original process lifecycle, progress and cancellation.

## macOS

Probe the existing functional Apple LLDB first, without installing anything. Homebrew separated LLDB from llvm; llvm alone no longer installs the frontend. The keg-only lldb formula declares its matching Python dependency and enables scripting. Resolve the lldb prefix, not llvm/bin. Sources: [llvm formula](https://formulae.brew.sh/formula/llvm), [lldb formula](https://formulae.brew.sh/formula/lldb), [formula source](https://github.com/Homebrew/homebrew-core/blob/fe1dd04cb5b84112fcce31209c0d7035edc4ea4e/Formula/l/lldb.rb).

## Linux

Ubuntu noble lldb-18 directly depends on python3-lldb-18; Debian trixie lldb-19 directly depends on python3-lldb-19. Their distribution lldb metapackage therefore carries the matching binding via its versioned package. Fedora 43 lldb declares python3-libs and python3-lldb. Arch lldb declares Python as a runtime dependency. Sources: [Ubuntu](https://packages.ubuntu.com/noble/lldb-18), [Debian](https://packages.debian.org/trixie/lldb-19), [Fedora](https://packages.fedoraproject.org/pkgs/llvm/lldb/fedora-43-updates.html), [Arch](https://archlinux.org/packages/extra/x86_64/lldb/). openSUSE binding selection remains under primary-source verification.

Extend the existing declarative package arrays during initial host privilege setup. Later dashboard repair uses the owned typed target with root or noninteractive sudo -n. Detached process children do not have a controlling terminal, so interactive sudo is not a reliable password flow. Never place credentials in arguments, environment or logs.

## Windows

The reviewed fixed LLVM 22.1.8 Winget declaration includes LLDB and x64/arm64 installers. Its default directory is Documents\LLVM, so choose and resolve an explicit install location. The Python 3.11.9 declaration defaults PrependPath to enabled; override it and disable launcher/file association changes to preserve the repository default Python. Add matching runtime directories only to the LLDB child environment. Sources: [LLVM manifest](https://github.com/microsoft/winget-pkgs/blob/master/manifests/l/LLVM/LLVM/22.1.8/LLVM.LLVM.installer.yaml), [Python manifest](https://github.com/microsoft/winget-pkgs/blob/master/manifests/p/Python/Python/3/11/3.11.9/Python.Python.3.11.installer.yaml).

Official LLVM reports document missing python311.dll and scripting failures in Windows distributions. CMake selects a Python runtime filename for the configured ABI. An executable lookup or version probe cannot establish scripting health. Sources: [issue 201229](https://github.com/llvm/llvm-project/issues/201229), [issue 218138](https://github.com/llvm/llvm-project/issues/218138), [issue 214542](https://github.com/llvm/llvm-project/issues/214542), [CMake](https://github.com/llvm/llvm-project/blob/main/lldb/CMakeLists.txt).

## Required Probe

Run the selected frontend with --no-lldbinit --batch -o version -o "script print(6*7)" and require success plus the expected value. LLVM documents that the runtime must match the linked Python distribution. The existing native debug fixture also compiles a real C program, stops at a breakpoint, evaluates an expression and continues to exit. Sources: [LLVM scripting verification](https://lldb.llvm.org/resources/build.html), [LLVM Python caveats](https://lldb.llvm.org/resources/caveats.html).

The [official openSUSE Factory llvm spec](https://api.opensuse.org/public/source/openSUSE:Factory/llvm/llvm.spec), fetched successfully with HTTP 200, declares matching versioned lldb as required but only recommends python3-lldb when Python support is available. Explicitly install both lldb and python3-lldb for zypper rather than relying on optional recommendations. The python3-lldb metapackage requires its versioned matching binding.

## Winget Override Semantics

Independent primary-source review found that the Winget override path returns its complete override string before normal silent/location template substitution. Therefore the fixed Python installer override must itself include /quiet and its quoted TargetDir, along with the options preserving the default interpreter; --silent/--location alone do not supply those settings when override is used. Source: [Winget installer argument implementation](https://github.com/microsoft/winget-cli/blob/master/src/AppInstallerCLICore/Workflows/ShellExecuteInstallerHandler.cpp). The controls owner is correcting schema, neutral vectors and initial bootstrap together.
