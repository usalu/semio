# Managed Browser Cache Verification

Actual read-only probe68147 completed0 in the existing container. No provisioning, browser launch, image rebuild, native query or graph was executed.

The root target `workspace:deps-browsers` invokes the native dependency script with `sync browsers`. Its implementation calls the installed Playwright CLI with `install chromium` and the environment returned by `repoToolCacheEnv(workspace)`. That helper sets PLAYWRIGHT_BROWSERS_PATH to repoCacheDirectory(workspace, "tools", "ms-playwright") unless the caller supplies an explicit value.

The Playwright tool registry declares the same cache segments. Its actual `toolEnvironmentV1(findToolV1("playwright"), …, {root})` implementation was imported and evaluated in the container. The native `repoToolCacheEnv` helper was also imported and evaluated, without Source guard relaxation. Both actual results equal the devcontainer's configured path:

`/workspaces/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`

The probe's ambient PLAYWRIGHT_BROWSERS_PATH was unset. It applied the actual registry result before loading installed Playwright1.64.1. Playwright then resolved Chromium to:

`/workspaces/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium-1248/chrome-linux64/chrome`

Neither that executable nor its managed browser cache root exists. Thus browser absence is verified under the authored native-dependency, registry and devcontainer environments, rather than inferred from the default home cache. An explicit caller override remains possible by design; no such override was supplied in this container probe.

The earlier default-home-cache observation is retained only as historical probe context. The final GL probe had already used the configured devcontainer path, and this independent helper evaluation confirms the same exact destination. No further absence correction is required.

Evidence is temporarily retained in `🗑️generated/browser-managed-path-proof.json` and its directly streamed log. The reproduction input `browser-managed-path.ts` remains in the ticket. All probe handles are terminal; no child process remains live.

The prepared next-step plan is `container-source-revalidation-plan.md`. It waits for the Go owner's final freeze, checks hash/mode identity in the existing private Source view, preserves valid private dependency generations, and leaves the next setup/graph run to Root.
