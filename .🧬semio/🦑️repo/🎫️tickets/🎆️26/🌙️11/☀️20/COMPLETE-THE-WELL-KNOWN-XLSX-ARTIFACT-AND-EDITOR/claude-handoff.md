# Handoff: Get the XLSX Artifact Running End to End

Snapshot 2026-11-23 ~01:30 CET. Prompt for the next coordinating chat is at the bottom.

## Ticket
- Open ticket `26/11/20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR` (goal `RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-OS/RUNNING-S/RUNNING-PLUGINS/RUNNING-STDIO/RUNNING-XLSX`). Do not reopen/open another; work inside it, close it with `ticket_close` when done.
- Folder T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR`.

## Already green (before this session)
- Native Rust suites of all five XLSX packages (`native-history-green.md`): base 46, base-mutations 1, strict 21, strict-mutations 12, transitional 28.
- TS routing suites 146/146, format claims current, `validate-xlsx-routing.ts` passes (`xlsx-routing-fixes.md`).

## Done in this session
- Tooling/coverage audit: `claude-audit-tooling-and-coverage.md` (no published graph; managed graph loads restart on the live tree → native fallback needed for hosts).
- Oracle gaps closed (`claude-xlsx-oracles.md`): insert-cell oracle arm + fixture, shared-string calamine differential, third-party pair table for relationships/content types, `calamine-oracle` suite implemented, `read_workbook_grid` offset bug fixed, `set-relationship-base` params + openpyxl/lxml fixture pairs for strict and transitional, replayable generators moved into `📕️xlsx/🔮️oracles/🔭️third-party-generators/` (🐍️.py via `repo tools exec uv -- run --frozen --group test`, 🟦️.ts via `repo`), evidence versions jszip 3.10.2 / fast-xml-parser 5.11.2. Oracle crate tests: **30/30 pass**.
- PDF build defects fixed while building hub components (blocked xlsx components): missing `PresenceLane` bound on `native_work` + `semio-s-artifact-stdio-pdf-presence-lane` dep in the PDF 1.4 canvas crate; `pdf-snapshot-value` test crate missing view-lane/presence-lane deps. `🌎️hub/Cargo.lock` gains the new edge.

## In flight at handoff (background processes may still be running; don't kill others')
| Lane | State | Logs (T/🗑️generated/…) | Next |
|---|---|---|---|
| Native suites re-verify | five xlsx crates (`rust-five-3.log`), assembly test `every_bundled_artifact_definition_assembles_without_violations` (`assembly-2.log`), `check-stdio-format-claims` (`claims-2.log`) running; graph slice load slow | `claude-suites/` | Read results, fix at root, write `T/claude-suites-reverify.md` |
| Feature run base mutations | `repo run @semio-tech/repo-test-domain:test-subject -- quick --case 🔀️mutate-xlsx-ecma-376` (the case needs the full emoji directory name; the bare slug fails) relaunched (covers new insert-cell rows and `sut` shared-string law) | `claude-oracles2/feature-subject2.log` | Fix failures, append "Follow-up" with numbers to `claude-xlsx-oracles.md` |
| Standalone pane (6272) | native route; hub wasm build attempt 7 (`semio-hub-stdio-component`, `semio-hub-stdio-office-component`, wasm32-wasip2) attempt 7 failed in the GIF editor (`render` takes one argument, callers pass two — another agent's in-flight edit); `T/claude-pane-cargo-retry.sh` retries up to 12× every 2 min. If it stays broken, fix the GIF `render` callers at the root. Engine packages published | `claude-pane/cargo7.log` and retry logs | `claude-pane-publish.ts` → `claude-pane-activate.ts` → vite on 6272 (stdio-xlsx variant config) → `standalone-pane-journey.ts` (en, de, dialects, refusal) → `T/claude-standalone-pane.md` |
| Full S host (6070) | native route; space+stdio wasm build was running; vite had served 6070 once (`probe-en-US.png`) | `claude-full-s/` | `claude-fulls-publish.ts` / `claude-fulls-activate.ts` / `claude-fulls-serve.sh` → `claude-fulls-journey.ts` en+de (Open File, OS drop, Strict/Transitional mount, edit, save keeps name, reopen, refusal) → `T/claude-full-s-host.md` |

Native recipe reference: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/native-dev-serve-recipe.md`. Managed-host stall diagnosis: `T/browser-hosts.md`.

## Remaining to call the goal done
1. Native suites + claims green now (report). Assembly test `every_bundled_artifact_definition_assembles_without_violations` now passes (2026-11-23).
2. Base mutation feature run green. A run failed before any case with the infra error "Exact live repository partition cache owner required"; if it recurs, diagnose that repo test-domain/cache owner path at the root. The subject-only phase (`test-subject`) reports this case as "not exercised"; the parity phase (oracle plus subject) is the one that runs it. Its scenarios are tagged `@level-long`/`@level-exhaustive`, so run at level `exhaustive` (the `quick` level skips them all). One exhaustive run died after ~1 h of compiling when cargo stderr exceeded the harness's 65,536-line output cap (not a test failure); a fundamental fix is to make that cap stream/truncate instead of killing the run. The next parity run failed before the compiler child on "Separated capture and input require finite Original control and bounded output": the scenario requests 16,777,216 lines and separated capture refused any line ceiling above 65,536. That admission ceiling now follows the stored-string `maxLength` (16,777,216) while the 16 MiB byte ceiling stays. Rerun is `feature-parity4.log`.
3. Standalone pane journey all steps pass.
4. Full S journeys en + de pass.
5. Remaining coverage gap: unreferenced shared-string pool entries only checked by our own reader (openpyxl now available via uv → add differential).
6. Cleanup: delete `T/🗑️generated` and tool outputs (txt/log/jsonl), keep scripts and md, remove `[DEBUG]` lines added, `ticket_close` with summary and all changed files (see each report's file list).

## Prompt for the next chat

```text
Continue the open ticket 26/11/20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR in /Users/ueli/Documents/semio (follow CLAUDE.md strictly). Goal: get the xlsx artifact running end to end. First read T/claude-handoff.md (T = .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR), then the reports it links. Check which in-flight lanes (native suites, base mutation feature run, standalone pane 6272, full S host 6070) finished by reading their logs under T/🗑️generated and `ps`; resume the unfinished ones, fix defects at the root, finish the "Remaining" list, then clean up and close the ticket.

At the same time parallelly the shared work fleet mechanism is being developed. If it doesn't work, just fallback to native tools. Under no means get stuck or hold your work.
Use the main chat with Grok 4.7 256k High Non-Fast for main plan coordination, use multiple Grok 4.7 256k Medium Non-Fast agents for medium to high complexity task execution, use multiple Grok 4.7 256k Light Non-Fast Medium agents for read-only exploration, audits and low complexity tasks. Finish the goal as fast as you can (without using fast llm agent modes, just non-fast models) with all the resources you can get (agents, cpu, memory, etc) and make sure to never idle and always be saturated. When you find fundamental issues which are unrelated to the task fix them as well. Treat the goal completely end to end. There are other agents working on other logical parts, even if they work on the same files, don't stop and keep on to work in conjunction with them, just focus on your task.
```
