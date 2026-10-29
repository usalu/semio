# Course Managed Browser Preparation — 2026-10-28

Source-only inspection; no browser installer, network request, package setup, browser launch or heavy gate was run.

The current course E2E runner automatically prepares the browser when it reaches its body. runCourseEndToEnd selects repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH and calls ensureBrowser before acquiring the proctor artifact or starting stacks. ensureBrowser resolves the installed Playwright CLI's own playwright-core package, reads that package's browsers.json, and checks INSTALLATION_COMPLETE for both chromium and chromium-headless-shell revisions. A missing revision invokes the same installed Playwright CLI with install chromium through the owned repository command interface, the original invocation/transport and a bounded600second budget.

This is version-derived preparation, not an assumption that any existing system Chromium is acceptable. The reported missing managed revision1248 is an actual external readiness observation; source inspection here does not establish its installation. Earlier graph or authorizer refusals can stop the managed target before ensureBrowser executes.

## Canonical Paths

| Need | Canonical path |
| --- | --- |
| Normal complete browser gate | Existing release E2E command in completion-gates; automatically calls ensureBrowser |
| Explicit browser-only preparation, if coordinated separately | repo tools exec playwright -- install chromium |
| Package provisioning when the declared Playwright package itself is missing | Registry declares repo tools setup all; ordinary managed dependency preparation remains preferred |
| Debian CI browser OS packages | Declared course ci-browser-prepare target invokes Playwright install-deps chromium under Original control; it is a platform-specific CI operation, not a command to apply indiscriminately to native hosts |

The registered Playwright tool derives its version from package.json's Playwright dependency, locates the package CLI through Bun, and uses tools/ms-playwright as the repository cache directory. repoToolCacheEnv preserves an explicit caller PLAYWRIGHT_BROWSERS_PATH; otherwise it sets the same repository cache. Do not point this at a user-global cache or substitute a system executable to claim the managed revision is ready.

The prepared browser-only token array is [repo,tools,exec,playwright,--,install,chromium], using the common SEMIO_LANGUAGE=en and ticket environment. It is an input ledger entry only, not an additional launched gate. The normal course command already performs this step, so do not duplicate installation while it is running.

The E2E configuration provides no executable-path fallback. Its boot warmup explicitly uses one worker; the complete gate still uses serial topologies with capacity-governed browser workers. A system Chromium smoke is separate diagnostic evidence and cannot replace actual Playwright revision launch and the course's complete browser assertions.

The testEnv named-input infrastructure repair belongs to the Linux readiness owner. No testEnv, ToolRegistry, devcontainer, runtime or dependency source was edited here.

Sources inspected: course e2e runner ensureBrowser/runCourseEndToEnd and config; process repoToolCacheEnv; registered Playwright tool declaration and adapter. All frontend semantic/unit/watcher/full-browser and qualified/persistent-stack requirements remain pending as recorded.
