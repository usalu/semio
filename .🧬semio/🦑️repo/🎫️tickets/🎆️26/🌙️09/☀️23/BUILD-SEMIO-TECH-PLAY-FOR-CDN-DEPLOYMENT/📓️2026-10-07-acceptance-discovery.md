# Acceptance Discovery

The scoped Playwright discovery gate passed on 2026-10-07 with exit code 0: **149 Chromium tests in one acceptance file**, comprising the overview and all 148 pane cases. The acceptance module, neutral interaction fixture, production interception fixture, and Playwright config loaded without module import errors. This is test discovery, not runtime acceptance or release readiness.

Executed through the existing Nx runner:

```sh
PLAYWRIGHT_BASE_URL=http://127.0.0.1:6033 NX_TUI=false NX_DAEMON=false bun nx exec --projects=@semio-tech/semio-tech-play --excludeTaskDependencies -- bun /Users/ueli/Documents/semio/node_modules/playwright/cli.js test --list --config /Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts
```

The successful generated log is `🗑️generated/play-acceptance-discovery-scoped.log`. A first unscoped `nx exec` attempt selected the entire repository and failed before Playwright on the unrelated `value-rs` / `value-derive-rs` cycle. A subsequent unscoped attempt with Nx's documented cycle flag was cancelled before CLI output; the final successful scoped run used no cycle exemption. No application tests or coverage requirements were relaxed.

Browser execution against the fresh release remains outstanding. The source and catalog contract audit is in `📓️2026-10-07-all-pane-interaction-contracts.md`; targeted mutation candidates are in `📓️2026-10-07-editor-mutation-regression-targets.md`.

The latest neutral role/introduction contract also passed the independent Ajv schema validator after browser discovery: **2 passed, 18 skipped** in the runtime source suite. Executed `bun nx run @semio-tech/semio-tech-play:test --excludeTaskDependencies --skip-nx-cache -- '🔨️modules/🧩️runtime/🟦️.ts' --testNamePattern=independent` with `NX_TUI=false NX_DAEMON=false`; log `🗑️generated/play-role-contract-source-filter.log`. A preceding attempt filtered the separate registration module instead of its in-source suite and therefore found no tests; that attempt is not counted as passing.
