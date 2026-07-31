# Ticket

## Todos

- [x] Gather sketchpad detail-panel and test context
- [x] Remove the design utility `settings` and `chat` panel paths
- [x] Replace the existing chat-panel Playwright coverage with a regression test for removal
- [x] Run the relevant sketchpad Playwright checks
- [ ] Close the ticket with the final file list

## Changes

- Removed the design app registrations that injected `compose.sketchpad.app.design.settings` and `compose.sketchpad.app.design.chat` into the right-side panel.
- Kept the dedicated sketchpad navbar toggles and shell panel modes for `settings` and `chat`, but left the design app-specific side-tab registrations removed so they no longer appear as embedded design detail tabs.
- Tightened the existing Playwright panel assertions to reject only the design-specific embedded utility tabs and replaced the old chat-panel test with a regression test that verifies the navbar toggles remain available while the design tab IDs and content IDs stay absent.

## Log

- `./repo/cli/cli tree sketchpad detail panel tabs chat settings tests` timed out without returning output.
- The compose CLI binary did not produce usable non-interactive output in this environment, so the task is being tracked directly in this ticket file.
- `npm run dev:sketchpad` served the sketchpad locally on `http://127.0.0.1:5173`.
- `npx playwright test sketchpad.test.ts --grep "Panels|Design Utility Tabs Stay Removed" --reporter=line` passed (`2 passed`) after the design-specific regression assertions were narrowed away from unrelated Home/Kit/Type utility tabs.
- A follow-up correction restored the navbar `settings` and `chat` controls after the requirement was clarified.
- The existing process on `http://127.0.0.1:5173` appeared stale during the follow-up rerun, so it was not reliable for validating the correction.
- A fresh server was started on `http://127.0.0.1:5175`, but `PLAYWRIGHT_BASE_URL=http://127.0.0.1:5175 npx playwright test sketchpad.test.ts --grep "Panels|Design Utility Tabs Stay Removed" --reporter=line` failed before test execution because Chromium crashed at launch with `sandbox_host_linux.cc:41 ... shutdown: Operation not permitted`.

## Summary

The design app-specific `settings` and `chat` detail tabs remain removed, while the sketchpad navbar `settings` and `chat` controls were restored. The post-correction Playwright validation is currently blocked by a local Chromium sandbox launch failure.
