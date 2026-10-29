# Browser Readiness

2026-10-28. No browser/server/probe was launched. Build skill read. The earlier default user-cache miss was an environment mismatch; repository-owned pinned Chromium is already present.

## Owned Interface and Actual Proof

workspace:deps-browsers declares repo:script, no prerequisite targets, no cache replay and a side-effecting orchestration route to the native dependencies script sync browsers. Its implementation provisions chromium through the pinned workspace Playwright CLI and repoToolCacheEnv(workspace). The existing environment interface lives at framework/repo/library/process/environment and sets PLAYWRIGHT_BROWSERS_PATH to repoCacheDirectory(repoRoot,tools,ms-playwright), preserving an explicit caller setting. The browser executable is selected by the installed Playwright package; no system Chrome fallback or hardcoded executable is used.

A read-only repo tools exec --light bun evaluation applied that exact repoToolCacheEnv interface before dynamically importing Playwright, then called chromium.executablePath and existsSync. Result:

- PLAYWRIGHT_BROWSERS_PATH: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright
- Executable: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium-1248/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing
- exists: true

This proves the pinned executable is present at the owned resolved path, not that it has been launched successfully. Framework renderer browser tests use chromium.launch with the browser package default; they rely on the caller environment for the owned browser root.

## Minimal Ticket Correction

Updated only browser-journey.mjs browser provisioning hookup: import the existing repoToolCacheEnv interface; inside the already deadline-protected try, set PLAYWRIGHT_BROWSERS_PATH from the workspace resolver; dynamically import Playwright afterward so its registry observes the configured root; record resolved browser root/executable in the report. Latest Select, health and cancellation changes were preserved with a fresh content hash comparison immediately before writing. No runtime dependency or new browser resolver added.

repo tools exec --light node -- --check browser-journey.mjs passed exit0. This was syntax validation only; browser execution continues to use Bun as required by the build skill.

## Fleet and Provisioning Decision

repo --service=off fleet status was observed before any possible provisioning. It showed5running/27waiting, current activation/native work, memory warning and no explicitly named browser provisioning run. Most waiters were cargo; two generic run labels were truncated, so the snapshot alone is not exhaustive target-identity proof. Because the exact owned Chromium executable already exists, no deps-browsers request or duplicate provisioning was necessary and none was launched. If provisioning is later required, use only the declared repo --tool=bazel run workspace:deps-browsers route after checking fleet again.

No new request is pending from this packet. Main component activation/native prerequisites remain pending under their existing owners. No readiness claim is made for the live layout component or6079server.

## Exact Post-Activation Serve and Probe Doors

Read-only route audit; no server/build/browser started. Every command below runs with cwd /Users/ueli/Documents/semio.

After the already-running activation succeeds, start the declared continuous server:

```text
repo --tool=bazel run @semio-tech/framework-os-dev:serve-layout-react-prototype --output=stream
```

Equivalent cross-platform ticket driver (explicit backend forwarded to the child repo):

```text
repo --tool=bazel tools exec python -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/launch-layout.py serve --tool bazel
```

Generated serve target is cache=false, continuous=true, outputs=[] and depends only on plugin-registry:session-layout. It runs the owning script serve layout react prototype, with the selected catalog injected into SEMIO_PLUGIN_CATALOG. The ticket driver independently pins the same specific catalog and computes the workspace from its own location. Do not use the retained old zsh scripts: they contain raw Bun/Nx and obsolete dev profiles.

The actual ServeScript selects variant=layout/renderer=react/profile=prototype, reads its activation receipt and checks variant/profile/host plugin identity. It invokes activation only if the receipt or selected host is absent. The composition manifest declares react6079 and wgpu6179. Serve uses S_OS_PORT with default6079 and fixedPort=true, so an occupied6079must cause refusal rather than choosing another port. It applies the layout plugin/app environment and uses the contribution Vite config through the managed Vite owner. Local hub ownership follows the declared contribution and existing S_LOCAL_ONLY/S_HUB_URL policy; do not invent new flags or a separate server.

Expected listener is http://127.0.0.1:6079/ and probe URL is http://127.0.0.1:6079/?plugin=layout. A continuous serve process remains running; absence of exit0 is not a startup failure. Wait for its ordinary listener-ready output before the browser probe. HTTP delivery alone does not prove the guest booted: the journey additionally requires documentElement data-semio-os-ready=layout, no data-semio-os-error, focused Blueprint owner, painted owner canvas, and visible fault checks.

Once server readiness is established, run the actual journey through managed Bun:

```text
repo --tool=bazel tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/browser-journey.mjs
```

Or the equivalent catalog-pinned ticket driver:

```text
repo --tool=bazel tools exec python -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/launch-layout.py probe --tool bazel
```

Use the ordinary tools execution path for the expensive real browser probe, without --light. SEMIO_PROBE_URL optionally selects an explicit server URL and SEMIO_PROBE_OUT names the ticket generated subfolder; defaults are the6079layout URL and layout-current. The current journey resolves repoToolCacheEnv before dynamically importing Playwright, so both direct managed Bun and driver use the already-proven pinned browser path. Preserve the continuous server exec session for owned cancellation after the probe. No direct Vite/Bun/Playwright installer or dashboard route is required.

## Owned Download Output

The live journey now passes downloadsPath at its existing Chromium launch so original download files, as well as saved validation files, stay inside the ticket generated run folder. Browser-managed profile internals remain owned by the browser tool; no user profile is used. This is a harness output-location correction only; no live execution is claimed.
