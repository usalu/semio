# Live Host Prerequisite Audit

2026-11-20, macOS arm64. The historical native-runner and verification audits describe a Windows host with no modern repo on PATH; they do not describe this host. Here repo --help succeeds and exposes current run and cargo commands. The registry reports Bun 1.4.2, Node 24.15.0, Cargo and rustup, and Playwright as available.

Independent executable probes through repo tools exec --light: Bun reports 1.4.2; rustup reports the workspace override nightly-2026-07-20-aarch64-apple-darwin; Apple clang 21.0.0 executes; Playwright reports 1.64.1. This establishes command availability, not successful compilation or host interaction. Cargo --version queued for 3584 MB under the capacity gate and was cancelled, not failed.

Playwright package executablePath checks show its expected Chromium 1248, Firefox 1555 and WebKit 2370 binaries absent in the user cache. This blocks those default Playwright browser launches unless browsers are installed or a separately available executable is explicitly configured; no installability or network conclusion follows from this check. Other registry entries marked missing were not established as requirements for the current host and are not asserted to be blockers.

During inspection the capacity gate delayed even a short node invocation behind concurrent layout tests and host activation. This is scheduling pressure, not evidence of a build failure. No duplicate build or installation was attempted.
