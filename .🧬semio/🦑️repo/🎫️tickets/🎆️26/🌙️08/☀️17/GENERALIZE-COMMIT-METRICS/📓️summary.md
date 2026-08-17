# Generalize Commit Metrics — Summary

## Done

- Replaced `📊️uloc` commit footers with extensible `📊️metric` envelope.
- Added `📃uloc` and `💾size` metric kinds with registry (`METRIC_KINDS`, `formatMetricBody`).
- Footer layout: repo totals per kind, then per-language pairs (`📊️metric🦀️rust📃uloc…` / `📊️metric🦀️rust💾size…`).
- Size scan uses `stat.size` (binaries and large files included); uloc keeps text scan rules.
- Byte deltas from git blob sizes between revisions.
- Cache v5: `.git/compose-metrics-cache.json` (uloc + size maps).
- Bundle scope and day lines append inline `📊️metric📃uloc…📊️metric💾size…`.
- Updated `.agents/skills/micro-commit/SKILL.md` and `.agents/skills/commit/SKILL.md`.
- Extended `index.test.ts` for new formatting and bundle metrics.

## Primary file

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` region `🔖️commit-metrics`.
