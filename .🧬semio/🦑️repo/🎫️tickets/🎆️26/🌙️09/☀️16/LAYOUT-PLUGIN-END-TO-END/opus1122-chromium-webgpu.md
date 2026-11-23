# Chromium WebGPU on This Host (Linux, SteamOS)

Probe: [opus1122-webgpu-probe.mjs](./opus1122-webgpu-probe.mjs), run with `repo tools exec bun -- ./opus1122-webgpu-probe.mjs [extra chromium flags]` from the ticket folder.

## Findings

- playwright 1.57 resolves chromium-1248 (156.0.8078.4) from `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` via `repoToolCacheEnv`.
- With the journey's existing flags (`--enable-unsafe-webgpu --ignore-gpu-blocklist --enable-gpu`), WebGPU works on a secure origin: adapter `google/swiftshader`, `requestDevice` succeeds.
- `about:blank` is not a secure context and exposes no `navigator.gpu`; the probe therefore routes a fulfilled `https://probe.test/`. `browser-journey.mjs` serves from `http://localhost` (secure), so it is unaffected.
- Optional: `--use-angle=vulkan --enable-features=Vulkan` yields the real hardware adapter `amd/rdna-3` with a working device. Not applied to the journey (swiftshader is deterministic and sufficient).
- No launch-arg change to `browser-journey.mjs` is needed.

## Earlier Launch Crash (Resolved)

- Symptom: `chrome_crashpad_handler: --database is required`, then `TargetClosedError` at `chromium.launch`.
- Cause: `/home` at 100% (0 B free); chromium could not create its crashpad database under `~/.config`. `repo tools exec` also refused to start (ENOSPC).
- After freeing disk (14 GB free), both probe variants launched through `repo tools exec bun` with no environment overrides and no crash.
