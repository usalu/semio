# Launcher Bootstrap

The available cached `repo.exe` serves MCP only and rejects command arguments. The cached `semio-repo.exe` exposes an older domain CLI without the build/script routes needed for this transfer. The normal launcher installation directory has no launcher.

The system Go toolchain reports the repository-pinned `go1.26.9 windows/amd64`. `bootstrap-launcher.ps1` builds the tracked launcher through a Go overlay. The overlay references original sources directly, avoiding source copies. Temporary files, the compiler cache, and the launcher executable stay in this ticket's generated directory. No installation or machine configuration changes are required.

The resulting launcher supports `repo <script>.ts` for ticket scripts and `repo tools exec` for registered tools. The normal launcher provisions and verifies its pinned JavaScript runtime if needed.

Validation completed: the overlay build succeeded; `repo --version --lang=en` reported launcher 1.0.0, contract 1. The first script invocation downloaded and verified pinned Bun 1.4.2 into `🗑️generated/runtimes`, then executed `transfer.ts` successfully and hash-verified all 131 canonical files. Subsequent invocations must set `SEMIO_RUNTIME_CACHE` to that directory and select `SEMIO_LANGUAGE=en` (or another supported language).

Bootstrap outputs are disposable. The retained input is `bootstrap-launcher.ps1`; it can rebuild the launcher without copying tracked source files or changing machine configuration.
