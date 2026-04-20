---
name: loc-command-refactor
overview: Refactor the repo client `loc` command to classify code/markup/data totals internally, add contributor filtering with history, include markdown and percentages, and strengthen existing tests.
todos:
 - id: ticket
   content: Open or reopen the appropriate repo ticket for the LOC command refactor.
   status: completed
 - id: snapshot
   content: Refactor LOC snapshot counting to internal hidden-folder-aware classification with custom data counting.
   status: completed
 - id: contributors
   content: Add `--by-contributor=<alias>` filtering that works with history.
   status: completed
 - id: rendering
   content: Update markdown/text/json renderers with category rows, LOC sorting, and percentages.
   status: completed
 - id: tests
   content: Extend existing `TestLocCommand` coverage for counting, filtering, rendering, and history.
   status: completed
 - id: verify
   content: Run focused repo client tests and close the ticket with touched files.
   status: in_progress
isProject: false
---

# LOC Command Refactor

## Scope

Update [repo/client/main.go](repo/client/main.go) and extend [repo/client/main_test.go](repo/client/main_test.go). No new source or test files.

## Ticket

Before editing, open a new ticket under `Repo CLI Filters` because the ticket inventory did not show an open ticket specifically covering the `loc` command refactor. Close it after verification with the touched files.

## Implementation Approach

- Replace the `cloc`-dependent snapshot path with an internal repo scanner so hidden folder segments (`.*`) are skipped consistently and JSON/YAML/TOML-style data can use custom counting.
- Keep the existing git `--numstat` history pipeline, but expand classification to include markdown/markup/data and add contributor alias filtering.
- Preserve current output modes (`md`, `text`, `json`) while changing rows to aggregate categories and percentages.

## Counting Rules

- `Code`: aggregate only the existing five code languages: TypeScript, Go, C#, Python, Rust.
- `Markup`: HTML, Markdown (`.md`, `.markdown`), MDX, and adjacent markup extensions.
- `Data`: JSON, YAML/YML, TOML, CSV, XML, and similar data/config formats. JSON LOC counts object keys, not physical lines, so single-line JSON still reflects its keys.
- `Total`: all included categories combined.
- Keep per-language detail where it helps history/debugging, but user-facing rows are category rows sorted by LOC descending with `Total` placed last if that reads better for totals.
- Every visible row includes percentage of total LOC in markdown/text/json.

## Contributor Filter

- Add `--by-contributor=<alias>` as a string flag alongside the existing `--by-contributors` boolean.
- Resolve git authors through the existing contributor alias path, then filter cumulative stats and history entries to that alias.
- With `--history --by-contributor=ueli`, history remains enabled and only shows the selected contributor’s history/cumulative rows.

## Rendering

- Update `LocLangStats` or add a small row model with `Loc`, `Percent`, `Edited`, `Added`, `Removed`.
- Change markdown/text table sorting from alphabetical to LOC descending.
- Include the `%` column in snapshot, contributor, and history tables.
- Keep JSON structured enough for API use, including rows sorted only at render time if maps remain internally convenient.

## Tests

Extend `TestLocCommand` in [repo/client/main_test.go](repo/client/main_test.go):

- Classification covers Markdown, HTML/MDX, JSON/YAML/TOML, hidden-folder skips, and the existing code languages.
- JSON key counting covers single-line and nested JSON.
- Aggregation produces `Code`, `Markup`, `Data`, and `Total` with percentages.
- Markdown/text rendering sorts by LOC and always shows percentage.
- `--by-contributor=ueli` filters cumulative stats and still produces history.
- Existing `loc` command registration and flags remain covered.

## Validation

Run the focused Go test for `TestLocCommand`, then run the broader `repo/client` test package if practical. After edits, check lints/diagnostics for the touched files.
