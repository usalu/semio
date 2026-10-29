# Host Capabilities and Provisioned Container Evidence

The host is SteamOS3.9.5, uid1000(deck). sudo -n true returns1 with password required; no prompt was opened. Host cc/gcc/clang/pkg-config/pkgconf are absent from PATH and normal system locations. No pacman, apt or dnf package cache is present. The installed Flatpak org.freedesktop.Sdk25.08 contains real GCC/cc/c++/pkgconf; those require its SDK sandbox/sysroot and were not blindly placed on host PATH.

Docker is absent, but rootless Podman6.0.2 works, including user mappings, overlay storage and its Docker-compatible API socket. The repository registry already supports an explicit SEMIO_TOOL_DOCKER override. SEMIO_TOOL_DOCKER=/usr/bin/podman repo tools exec docker -- version passed; no wrapper script or source change was required. The repository filesystem initially had about155GiB user-available space, sufficient for the configured image.

The exact checked-in .devcontainer/Dockerfile built successfully through the repository Docker tool interface. Its Ubuntu24.04 base, package installation, locale setup and SHA256-verified pinned Node archive all passed. Result image: 8f71f06d6a99f056b69d57e5065522a35ccd92aaf35eebb90c7b6afbc5c1b494, tagged localhost/semio-linux-readiness.

Container1acb9d2de399 runs as vscode uid1000 under rootless keep-id. It binds the existing checkout and mounts isolated named node_modules, native cache and repository cache volumes, matching the declared devcontainer layout. The configured onCreate chown completed. Probes passed for genuine cc13.3.0, Cairo1.18.0, Pango1.52.1 and UUID2.39.3 through pkg-config, and Node24.15.0. No host-cache ownership was changed.

The checked-in native bootstrap is now running inside that provisioned container to install feature toolchains and exercise workspace setup. A full devcontainer CLI feature build is not claimed: this is the checked-in base Dockerfile followed by its checked-in Unix bootstrap. Additional lifecycle/build evidence will be appended after execution.

## Bootstrap Terminal Result

The initial container bootstrap exited1 at repo run repo-mcp:install after successful apt prerequisites, launcher installation, pinned Go1.26.9, Rust nightly-2026-07-20, .NET8.0.421, uv0.11.6 and Bun1.4.2 provisioning. Source admission then refused seven stale JavaScript dependency scopes: @cto.af/linebreak3.0.5 is missing from locks and removed generated workspace members remain in locks. The failure matches the independently observed host workspace-build gate. No bootstrap retry was started before reporting this terminal result.

## Native System Compile Proof

The retained native-system-smoke.c/.json/.cjs input test compiled C11 against the configured Cairo, Pango, UUID and ICU development libraries, linked and executed in the provisioned container. Runtime library versions independently matched pkg-config metadata: Cairo1.18.0, Pango1.52.1 and ICU74.2. UUID parsing/unparsing produced the declared lowercase UUID. The binary is a generated ticket artifact and must be removed during final cleanup; input sources remain.

## Preflight and Graphics Provisioning

The provisioned container launcher completed repo preflight with exit0 after source admission. The actual image-backed compiler, pkg-config, native libraries and cache filesystem satisfy preflight without relaxing its requirements. The host SteamOS native environment still requires privileged package provisioning; the successful result is the rootless Ubuntu container, not a claim that its host was modified.

Following two red regressions, all native Linux package rows now declare xdg-utils and the appropriate Vulkan loader (libvulkan1, vulkan-loader or vulkan-icd-loader). The Dockerfile now explicitly declares mesa-vulkan-drivers for software Vulkan in headless containers, without requiring a replacement vendor ICD on native desktops. The focused native suite passes 6 tests and 70 assertions. The new package was installed through the repository Docker interface in the retained provisioned container rather than discarding installed feature toolchains. Actual Vulkan loader/ICD smoke passed: the system loader created a VkInstance and enumerated the CPU llvmpipe (LLVM20.1.2,256bits) device, Vulkan device type4. The software ICD works without a GPU mount. No rendered native UI claim is made.

## Source Isolation During Concurrent Work

After successful native command installation, workspace setup began installing JavaScript dependencies in the private cache. Because the original Source bind was shared with a concurrent host course test, the owned bootstrap tree was interrupted with SIGINT before app-scope publication. It released leases and exited1. A host audit found the root node_modules directory unchanged (mtime2026-10-21) and all five app-scope node_modules endpoints absent; no app-scope binding was published. Only the task-owned bootstrap process tree was signalled.

The replacement container3affb81c6327 uses Podman Source overlay mount :O, with the original checkout as its lower layer and the same private named cache volumes. The actual kernel mount was verified as overlay, and a generated probe written inside Source was absent on the host before being removed. The quiescent provisioned root filesystem is preserved as local image f2e10d23dce232e65336b77b1accc2bf1bd8d3b9f94f470912106700fe5e64be, so language runtimes and the installed native command are reused. This avoids source copies/worktrees and prevents container dependency bindings from becoming visible on the shared host Source.

## Complete Scope Isolation Proof

All six host scope node_modules endpoints are now owned host-generation symlinks. Inside the private Source overlay, root was already absent; five inherited app-scope links were removed after verifying their raw targets matched the host snapshot. All six host endpoint identities (device, inode, mode, mtime and raw target) were unchanged afterward. Any valid private /workspaces generation links would have been retained; none existed in this snapshot. No source file or host link was removed.

The comprehensive Source endpoint union includes all six scope manifests and every declared package member, deduplicated to320 endpoints. A read-only audit found314 inherited package links in addition to the6 already-reset scope roots. Only these exact host-generated links were unlinked inside the private overlay; no valid private generation links or ordinary Source were touched. All320 host device/inode/mode/mtime/rawtarget records were unchanged afterward. The next bootstrap handle runs to a real terminal result.

## Targeted Installed Frontend Verification

The Source:O container's owned native installer completed with terminal status 0 in the fully captured diagnostic run. It executed the declared repo-mcp:build child, built and installed /home/vscode/.local/bin/repo and served native help. Complete transient output is retained in generated/container-native-install.log until ticket cleanup. An earlier targeted run failed with a truncated leading diagnostic; the complete retry did not reproduce it, so no Source or custody guard was changed on that evidence. This capture happened while another agent was updating Go search, and therefore is infrastructure evidence rather than final Go search parity proof. The focused real issued readmission regression is registered and currently awaiting host capacity.
