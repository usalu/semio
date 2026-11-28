# Result H83: `repo fleet status` summary view (wave 1127)

## Problem and result
`repo fleet status --json` was 143 KB (workers 29.7 KB, rates 27.6 KB, machines 10.7 KB, findings 8.4 KB) and MCP clients truncated it. Now the default is an actionable summary, measured live 13:03: **11.8 KB** JSON (text 7-9 KB); `--origin=g1-m01 --section=workers` 3.6 KB; `--detail --json` unchanged 142 KB.

## Design (schema first, all validated before printing)
- Views: default = `Summary` (new closed schema); `--detail` = the full `Report` as before; `--origin=<id>` and/or `--section=<name>` (imply detail) = `Selection` (new closed schema, each section exactly as in the report; with an origin the workers, machines, counters and findings of that origin). Unknown origin/section are refused in English and German with the known names.
- The summary is derived from the detail report alone (`summarizeFleetReportV1(report, limit)`), so detail parity holds by construction and is tested.
- Summary content: totals per window (fleet counters non-zero only, top 16), per-origin counters only for the anomaly metrics list of `policy.summary.anomalousMetrics` (skips, refusals, fallbacks, cancellations, abnormal declines) non-zero in the first window, top 4 per origin, `omitted` counts the rest; findings grouped by id+severity with `count`, up to 8 `subjects`, `more`, text cut at 200 chars; workers: rows only for those needing a look (not ready, refusal, tree not ok, warden not calm), the healthy ones as one origin list; fleet: totals plus `attention` machines (pressure, refusing, no headroom, differing code); runs compact; queues, bazel, storage, sealed, generation, scheduler, offload (non-zero reasons) reduced to counts; sources with failures only.
- JSON rendering `renderFleetJsonV1`: sections expanded, rows one per line (valid JSON, 2-3x smaller than indent 2).
- Text: `formatFleetSummaryV1` (about 40 lines, `--limit` bounds runs) and the existing `formatFleetReportV1` for `--detail`, now with a `sections` set so `--section` works in text too (the sections partition the full text; tested). 21 new texts, English and German; `error.syntax` updated.
- MCP: parameters follow the operations catalogue: `fleet_status` now has `detail`, `origin`, `section` (closed enum) and `limit`; `--json` stays bound.
- Fixed on the way: the help renderer looked flags up by name only and showed the remote-execution `--origin` text for `fleet status`; it now prefers the entry whose `operations` names the requested operation (`units --only` had the same defect; snapshots regenerated).

## Tests (fleet-status 36/0)
Size law (16 origins, every one anomalous, both languages: JSON 12.9 KB, text < 10 KB, detail 174 KB, limit 16 KB); detail parity on 22 corpus cases + 20 seeded cases + the crowded fleet (counts, totals, rates, findings, workers, machines, sections, omitted arithmetic); origin filter and section gating (sections partition the text); command in both languages for every mode, refusals, argument parsing.
Typecheck passed (26 files), dangling imports 0, operations 16/0, options 22/0.

## Unverified / not mine
- Fleet-status guard tests (2) and mcp build-surface oracle (1) are red before and after H83; see `findings-H83.md` (F1 readers `spawnSync` and inputs/revision load; the mcp row is an uncovered catalogue entry).
- Scripts: `T/🧰️scripts/H83/` (schema.py, apply.py, tests.py, land.sh, prep.py, block.ts, tests-append.ts).
- MCP tool parameters were not exercised through a live MCP client.
