# Handoff Prompt: Continue the Semio-Tech Play CDN Redeploy

Paste the block below into the next chat.

```text
Continue the reopened ticket 26/09/23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT (folder `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT`). Do not open a new ticket.

Goal: get semio-tech play ready to be redeployed on a CDN as fast as you can. It doesn't matter for this redeploy if it contains bugs; it just needs to run every app. The shared work fleet mechanism is being developed in parallel. If it doesn't work, fall back to native tools. Under no means get stuck or hold your work.
Use the main chat with GPT 6.2 Astra Medium for main plan coordination, use multiple GPT 6.2 Sol Medium agents for medium to high complexity task execution, use multiple GPT 6.1 Terra Medium agents for read-only exploration, audits and low complexity tasks. Finish the goal as fast as you can with all the resources you can get, and make sure to never idle and always be saturated. When you find fundamental issues which are unrelated to the task, fix them as well. Treat the goal completely end to end. There are other agents working on other logical parts. Even if they work on the same files, don't stop; keep working in conjunction with them and focus on your task.

Read first: `📓️2026-11-17-redeploy.md` in the ticket folder (items 1–47 hold every fix and finding so far), then the other `📓️` notes there.

State at handoff (2026-11-19 23:40 CET):
- The graph service (`repo run`) is starved, so everything runs natively through ticket scripts: `native-play.sh` (stages prereqs|catalog|generators|boundaries|site|verify|routing|browsers|step), `native-build-launch.ts` (deadline-free build-caller envelope), `native-launch.ts`, `native-generators.py`, `native-source-boundary-preflight.ts`.
- A retry loop is RUNNING, detached: `(native-play.sh site && native-play.sh verify) >> 🗑️generated/native/site-run.out`. It is on run 3, attempt 3 of 8: the loop resumed at 23:35 after a host filesystem stall (item 47), and that attempt's site-build started at 23:37:13. Logs: `🗑️generated/native/site-run.out` (step events) and `🗑️generated/native/site-build.log` (driver progress; earlier attempts are rotated to `site-build-HHMMSS.log`). Do not start a second loop while it runs (`pgrep -fl "native-play.sh site"`).
- Best attempt so far: every Rust unit compiled (about 4,700) and component staging started (1/73 at 171 min); a full attempt takes about 3–4 h. Component staging, pages publication (`.cache/b/p/m/play/release/pages/...`) and verification (`native-play.sh verify`: routing, release-verify, test-release with 173 Playwright tests) have never completed.
- Fixed and tested today, so don't redo them: the telemetry sample ceiling (item 43), late nested borrower operations (44), the four norm component descriptor pairs (45), SQLITE_IOERR_VNODE lease reopen (46), the cpu-picker/host-capacity deadlock (42).
- Unverified risk: the BIM `RetainedOrderedMap` migration (about 610 call sites, 128 files, item 41, `📓️2026-11-19-bim-retained-ordered-map.md`) has never been compiled. The release build is its first check. If a BIM crate fails, fix the call sites against the map's `cold_insert`/`cold_remove`/`get_mut` API.
- Never run `repo cargo` checks or `components describe` while the release loop runs: they deadlock or race on shared state (items 42, 45). Run them in sequence only.
- A new plugin component without a committed descriptor pair fails release staging with "no committed descriptor pair". Fix it with `LAUNCH=native-build-launch.ts TASK=workspace:components OP=cargo JOBS=6 FORCE=1 bash native-play.sh step describe-X "" repo ./📜️script.ts components describe <plugin-id…>` when no loop is running.
- Other workers edit the shared tree live. Compile breaks from their mid-edits usually resolve within minutes; exit 75 restarts are expected.

Next steps: watch the loop. When a failure is in the repo, fix it, record it as the next numbered item in the redeploy report, and relaunch the loop if it is exhausted. After the site build succeeds, `verify` runs automatically; make every app pane boot. Then update the report, delete `🗑️generated`, and close the ticket with `ticket_close`, listing every file worked on (see the report items and the git status of the ticket folder).
```
