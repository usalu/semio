# Native Platform Boundaries

Read-only inspection of executable setup/build configuration on 2026-10-28 found no Windows-only .NET target framework or unconditional Windows CMake compiler selection.

## .NET

Exactly two checked-in csproj files were found outside cache/dependency trees: framework UI styling package and repository test host. Both target net8.0 with Microsoft.NET.Sdk, not net8.0-windows, WindowsDesktop, WPF or WinForms. Both are expected to build on native Linux with the pinned SDK and ICU dependencies. Directory.Build.props derives artifacts from MSBuildThisFileDirectory into .cache/b/p/m/dotnet/raw rather than a drive-specific path. Expected .dll output names are portable managed assemblies. No Windows project exclusion is required for these two projects.

## CMake

CMakePresets.json defines prototype/release presets for Linux, Darwin and Windows, each guarded by hostSystemName equality. All use Ninja and a caller-owned SEMIO_CMAKE_BINARY_DIR. Root CMakeLists.txt uses C++23 and selects MSVC flags only under if(MSVC), with portable warning flags otherwise. The repository Windows toolchain helper returns immediately for non-Windows hosts before resolving Visual Studio/vswhere/cmd paths. Linux should select linux-prototype or linux-release and requires actual C++23 compiler plus provisioned CMake/Ninja. No preset itself forces Windows/macOS compiler paths on Linux.

## Rust Graphics

framework UI host Cargo.toml selects backend_metal only under target_os=macos, backend_d3d12 only under target_os=windows, and backend_vulkan plus winit X11/Wayland features only under target_os=linux. The Vulkan adapter manifest scopes ash/raw-window-handle to Linux; Metal scopes objc2-metal to macOS; D3D12 scopes windows crate to Windows. These native system dependencies must remain excluded from Linux through Cargo target predicates, rather than deleting platform packages from the monorepo.

Linux graphics readiness needs Vulkan loader/driver and a real X11/Wayland session or a deliberate headless test mode; compiling does not prove actual renderer startup. winit Linux rows select wayland-dlopen, so runtime shared-library presence remains a gate even when no corresponding development package is needed. Native renderer frame-transaction and UI draw/hosting manifests repeat the Linux backend feature selection.

Browser wasm32-unknown-unknown and WASI p2 dependencies are separately target-guarded throughout OS/plugin crates. Native builds should exclude their JS/WIT host branches according to their target predicates; source/native/wasm test matrices must exercise all supported branches separately.

## Names That Must Not Be Excluded

The sourcing extension named 🪟️windows is architectural windows typology: its manifest describes windows typology and demo catalogue kinds and has portable first-party dependencies. It is not a Microsoft Windows-only extension. Likewise shell-windows and UI editor 🪟️windows directories mean application windows, not an OS restriction. Excluding these by path/name would incorrectly remove Linux product functionality.

## Verification Boundary

This inspection establishes authored platform predicates and expected native behavior, not actual cross-platform linking. Preserve target-specific dependencies and verify with native Linux selected graph/build, then actual macOS/Windows runs. No source files were changed.

## Machine-specific Native Frontend Pointer

.🧬semio/🦑️repo/native-repo-path is generated installation output, despite currently being tracked. MCP installer script.ts67 writes the current absolute plan.executable plus LF with mode0600. Installed frontend proof at library/🧪️testing/📦️installed-frontend/🟦️.ts73–74 reads it and requires parity with the local installation receipt. Micro-commit hook resolver54–61 optionally reads it, then falls back to launcher installation and PATH. The root .gitignore lacks its exclusion.

Long-term ownership: source policy should own only the relative pointer location and format; the installer owns local absolute content. Remove the tracked machine-specific artifact, ignore the exact generated location, retain local installer generation/provenance checks, and centralize its location in installation schema consumed by all three owners. A stale Windows pointer becoming a Linux home path is expected local installation behavior and must not create a version-controlled machine setting. No source or index changes made by this audit.
