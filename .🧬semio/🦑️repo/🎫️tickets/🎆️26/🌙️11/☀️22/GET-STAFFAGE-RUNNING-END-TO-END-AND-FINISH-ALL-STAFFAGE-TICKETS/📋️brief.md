# 📋️ Finisher Brief: Staffage End to End

The dev's order (2026-11-22 20:20): "Get staffage running end to end. Many assets were interrupted while modeling, etc. Finish all existing tickets."

Every staffage wave of today stopped at about 17:30 when its session was cut off. No other agent is working on staffage now. You are a finisher: you take over ONE existing ticket (named in your prompt) as its coordinator and author, and bring it to a state in which it can be closed.

## Paths

- `R` = `/Users/ueli/Documents/semio` (read `R/CLAUDE.md` first; it is binding).
- `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage` (product; `S/README.md` is the contract).
- `U` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS` (umbrella ticket; your report goes here).
- `K` = your ticket (in your prompt). Its own brief, assignment tables, ledgers, requests and reports are the binding detail for the work; this brief only overrides them where it says so.

## State found at 20:23

- Catalog: 347 asset folders, all `completion: "authored"`; manifest `S/🗂️catalog/🔣️.json` has 276 entries. 71 catalog assets have no manifest entry: 40 humans, 30 plants, `used-bicycle-shelter`.
- Human wave: 12 rows in `📥️manifest-rows`, plant wave: 30 rows. A catalog file of an interrupted author may be half finished although it parses.
- Used/reclaimed waves: every candidate under `K/assets` is published; ledgers list running rows, open review points and requests.

## Your job, in this order

1. **Take stock.** Read `K` (ticket json description, brief, assignment/work board, ledgers, reports, requests). For every asset of the ticket decide: finished and proven / started but interrupted / never started. Trust only what you measure now (validator, check CLI), not a ledger line.
2. **Finish every started asset** to the quality bar of the ticket brief: full proof valid, product check CLI `✓`, sheets opened and looked at, report written, manifest row produced the way the ticket prescribes (publish tool under its lock, or a row file in `📥️manifest-rows`). An asset that was never started (no candidate, no generator, no catalog file) is NOT started now: the dev ordered at 16:29 to finish the started rows only. List them in your report.
3. **Do the coordinator's remaining steps of the ticket** (review pass over open points that are real defects of an asset, ledger/work board brought up to date, collections, the ticket's own verification scripts), except the two things reserved below.
4. **Clean up**: delete tool output you or the previous session generated for this ticket (`🗑️generated` of `K`, raw logs, result txt files); keep scripts, configs, briefs, reports, audits.
5. **Report**: write `U/<report name from your prompt>.md`: per asset the state before and after with the final proof lines, what you changed (every file created, updated or removed, as a list of repo-relative paths), what stays imperfect stated plainly, never-started assets, shared-code requests still open. Your final message is only the report path and at most six lines.

## Reserved for the umbrella coordinator (do not do these)

- Hand edits of `S/🗂️catalog/🔣️.json`. Humans and plants: row files only. Used/reclaimed waves: the ticket's `publish.ts` (it locks) is the only way.
- Shared engine, schema, validator, tests, fixtures, README, site code: do not edit. A defect there with a measurement goes to `U/requests.md` as a section `## <ticket> — <subject>` (re-read before appending) with file, measurement and proposal; work around it and continue. Exception: a finisher whose ticket IS about shared code (named in the prompt) edits the files that ticket owns.
- Opening, closing or reopening tickets and goals; the full catalog test suites and the site build (the umbrella coordinator runs them after all finishers).

## Rules

- One shared live worktree replicated to 50+ machines. NO git command that changes anything, no worktrees, no stash, no IDE, no `repo dashboard`. Re-read a file right before you write it. Ignore unrelated changes and keep going.
- Never weaken a rule, threshold, test or tool to pass.
- The host is shared (16 GB, 10 cores) and eight other finishers run now: exactly one tool process at a time, in the foreground; no vitest suites over the whole catalog; a killed tool: wait and retry. Use the pinned bun (`ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1`) or `repo` as the ticket brief says.
- OPEN every sheet you claim to have seen. Never report something as proven that you did not run to the end in this session.
- No sub-agents. Never stop to ask, never stop halfway: decide, note the decision in the report, continue until every started asset of your scope is finished.
- Temporary logs carry the `[DEBUG] ` prefix. No comments inside definitions; docstrings start with a unique fitting emoji. Paths stay under 256 characters. Texts are `{ en, de }`, English first.
