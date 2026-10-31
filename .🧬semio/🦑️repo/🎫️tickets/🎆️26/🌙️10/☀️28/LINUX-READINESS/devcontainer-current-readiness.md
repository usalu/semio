# Current Devcontainer Readiness

Read-only Source inspection on2026-10-30. No fresh container creation, setup, start or attach has been run by this inspection.

The Dockerfile selects Ubuntu24.04 devcontainer base, installs build tools, browser libraries, Vulkan/Mesa and fonts, generates en_US.UTF-8, and installs Node24.15.0 from architecture-specific checksum-verified x64/arm64 archives. The configured user is vscode. Compose mounts the checkout and declares no fixed service container name; its process is sleep infinity. The docker-in-docker feature independently supplies its own security requirements, so the overall container is not proven unprivileged by Compose alone.

The create hook transfers the two checkout cache-volume roots to vscode. The post-create hook invokes the checked-in native bootstrap with setup before subsequent hooks call registered repository targets. Start uses workspace:setup-devcontainer-start; attach uses workspace:setup-devcontainer-attach. Named volumes retain repository/native caches, client auth and viewer preferences. Rust feature version none leaves canonical toolchain preparation to repository ownership.

This establishes current declarations only. Fresh image/features build, native bootstrap, full setup, lifecycle hooks and warm repetition still require actual acceptance. Existing warm rootless container evidence does not prove those cold lifecycle gates. The current Course managed development pair is independent and must remain untouched.
