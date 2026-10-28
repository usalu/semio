# Rust Native Linux System Library Closure

Read-only Cargo manifest/build.rs and root Cargo.lock inspection. No bootstrap script changes while live setup runs. This report distinguishes compilation requirements from dynamic renderer runtime requirements.

## Established Build Inputs

No checked-in Cargo.toml or build.rs direct dependency/probe for ALSA, udev, OpenSSL or native-tls was found. Root Cargo.lock contains neither alsa-sys, libudev-sys, openssl-sys nor native-tls. It contains openssl-probe, which is certificate discovery rather than linking OpenSSL. Therefore libasound2-dev, libudev-dev and libssl-dev are not justified additions for this selected root dependency graph. Browser ALSA runtime requirements are separate. Independent nested Cargo locks can have additional tools and must be assessed when selected; this is not a universal claim about every external executable.

C/C++ compiler and pkg-config remain actual prerequisites, with Cairo/Pango development packages already present for owned native graphics/text integration. Bundled SQLite requires C compilation but not a separate system SQLite development package for that Rust dependency.

## Renderer Runtime Inputs

The Linux native UI host Cargo.toml selects backend_vulkan and winit0.30.12 with x11, wayland and wayland-dlopen. Equivalent feature rows occur in UI draw device, hosting and the OS native renderer frame/app/runtime crates. Vulkan adapter Cargo.toml requests ash0.38 without overriding defaults. [Ash primary source](https://raw.githubusercontent.com/ash-rs/ash/master/ash/Cargo.toml) declares default loaded/libloading and an opt-in linked feature. Thus current manifest does not require Vulkan SDK headers or link-time development library merely to compile ash. It does require a loadable Vulkan loader and functioning ICD to render.

| Capability | Ubuntu package | Current coverage |
|---|---|---|
| Vulkan loader libvulkan.so.1 | libvulkan1 | Dockerfile explicitly includes; native toolchain/browser rows omit |
| Software Vulkan ICD for container rendering | mesa-vulkan-drivers | No explicit Dockerfile/native declaration found; host vendor ICD can satisfy actual desktop |
| Wayland client runtime | libwayland-client0 | Not explicitly declared in Dockerfile/native rows; transitive installation is not established |
| Wayland cursor runtime | libwayland-cursor0 | Not explicitly declared |
| XKB runtime | libxkbcommon0 | Browser/Dockerfile coverage present |
| X11 cursor/input runtime | libxcursor1, libxi6 | Not explicitly declared in native rows; some browser installer closures can supply them |
| X11 XRandR/X11/XCB runtime | libxrandr2, libx11-6, libxcb1 | Non-apt browser rows explicitly cover main capabilities; apt delegates pinned Playwright |

The minimum proven new native prerequisite is Vulkan loader capability when Linux Vulkan renderer startup is expected. Its distro equivalents are Fedora vulkan-loader, Arch vulkan-icd-loader and openSUSE libvulkan1. A render test additionally needs an actual driver/ICD; mesa-vulkan-drivers (Ubuntu/Fedora), vulkan-swrast (Arch) or Mesa-vulkan-drivers (openSUSE) are software implementations, not mandatory replacements for an existing vendor driver. Validate availability against supported distro release before adding package rows.

Wayland-dlopen moves system-library validation to startup. Do not add Wayland development headers solely because the feature name appears. Verify each runtime path under a real Wayland session or intentionally choose X11/Xvfb; claiming both backends operational requires both runtime closures. Xvfb supplies an X server, not a Vulkan ICD.

## Verification Boundary

The actionable deficiency is runtime graphics provisioning, not a demonstrated Rust native compile failure from ALSA/udev/OpenSSL. Add narrowly scoped loader/driver runtime checks with cancellation and real renderer smoke execution. A cold container native UI test should prove software ICD discovery and startup; native desktop setup should accept an existing suitable vendor ICD. Full ELF closure and actual graphics startup were not run by this read-only audit.
