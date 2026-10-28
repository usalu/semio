# Linux Target Path Portability Audit

Read-only audit on 2026-10-28 examined all 1,489 statically inventoried project manifests and their target options for Windows executables/shells, drive roots, macOS application/Homebrew roots, Xcode and batch commands. No matches appeared in target options. This is evidence about authored command declarations, not runtime graph execution.

Scanned checked-in build scripts, shell scripts, Go sources, CMake files and TypeScript runtime helpers outside dependency/cache/test fixture trees. No additional unconditional Windows/macOS path blocker was proven. Platform-specific code and portable .NET assembly names were inspected rather than classified from suffix alone.

## Reviewed Matches

| Source | Observed guard/meaning | Assessment |
|---|---|---|
| repo client MCP script.ts:61 | powershell.exe invocation inside process.platform===win32 | Linux excluded |
| repo build cmake Windows toolchain.ts:21–23 | Visual Studio/vswhere/vcvarsall/cmd paths after early non-win32 return | Linux excluded |
| repo bootstrap script.ts:457 | Windows process sampling selects PowerShell; other hosts ps | Portable branch; ps provided by Linux procps prerequisite |
| repo launcher runtime-registry Go:301 | Windows installation command inside runtime.GOOS==windows | Linux excluded |
| OS renderer wgpu native script.ts:26 | .exe suffix selected only on win32 | Portable executable suffix |
| hub local-bootstrap MCP support.ts:38 | Darwin open, Windows cmd.exe, Linux xdg-open | Linux browser opener requires xdg-utils; currently devcontainer includes it |
| framework styling dotnet builder script.ts:47 | Semio.Framework.Ui.Styling.dll expected from dotnet build | .NET assemblies use .dll on Linux too; no defect |
| repo tools wasm helper.ts:44 | accepts .so/.dylib/.dll members and platform-specific wasm-opt executable | Portable archive selection |
| repo native process observation.ts:155/175 | Darwin and Windows readers selected by their platform adapter | Platform-specific system libraries are intentional |

## Remaining Boundaries

Native Linux browser-opening flow calls xdg-open whereas bootstrap system package rows do not currently include xdg-utils (the devcontainer Dockerfile does). This is a concrete native desktop prerequisite gap for automatic opening, not a monorepo build blocker. The browser dependency provisioning changes must be reviewed for whether they add xdg-utils or otherwise declare the opener optional.

Explicit platform branches were preserved. Hard-coded libc.so.6 in Linux FFI and preflight glibc linker policy indicate glibc Linux support; this audit does not establish Alpine/musl support. Optional user-selected shell commands such as cmd or PowerShell do not imply unconditional Linux execution.

This scan cannot prove runtime readiness: generated graph resolution, compiler/artifact custody, system dependencies, package locks, executable permissions and complete target execution remain independent gates. No source edits were made.
