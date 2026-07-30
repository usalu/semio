---
name: Explicit Commit Uloc
overview: Extend every commit `📊uloc` line (footer total, per-language, bundle, and day) with explicit `💯` total, net trend (`📉`/`📈`), and `➗` ratio vs previous total — omitting empty parts and formatting ratios with three significant figures.
todos:
  - id: format-helpers
    content: Add formatMetricRatio + extend formatMetricLocCount (M); unify formatUlocMetricsBody with omit-empty + net/percent
    status: completed
  - id: footer-rows
    content: Rewrite formatMicroCommitMetricsLines/MetricLine to 📊uloc💯… and 📊uloc{emoji}{slug}💯…
    status: completed
  - id: bundle-day
    content: Extend formatBundleUlocSuffix/header/date with scoped 💯 and full metrics body
    status: completed
  - id: detectors-skills
    content: Update uloc line detectors, 🔢 guards, and micro-commit/commit skill examples
    status: completed
  - id: tests
    content: Extend index.test.ts for ratio/loc formatting, footer, and bundle/day full lines
    status: completed
isProject: false
---

# Explicit Commit Uloc Metrics

## Target shape

Every `📊uloc` line uses one full template; omit any segment that is empty/zero:

```
📊uloc[🧾json]💯{bloc}[📉|📈{net}➗{ratio}][➕{a}][✏️{e}][➖{r}][🟰{a+e+r}]
```

Examples (from your spec):

- Total: `📊uloc💯1.3M📉7000➗0.01➕8117✏️636➖15902🟰24655`
- Language: `📊uloc🧾json💯769k📈60000➗0.1➕80000✏️53➖20000🟰201`
- Bundle/day: same `📊uloc…` suffix after the existing label/date prefix (e.g. `🏘️compose✍️sketchpad📊uloc💯…`, `🎆26🌙06☀️04📊uloc💯…`)

## Semantics (chosen)

| Segment | Rule |
| --- | --- |
| `💯` | Current total bloc for that row (repo / language / bundle-scoped). Always present when `> 0`. |
| Net | `net = ➕ − ➖`. `📈\|net\|` when `net > 0`, `📉\|net\|` when `net < 0`. Omit trend + `➗` when `net === 0`. |
| `➗` | `\|net\| / previous`, `previous = 💯 − net` (edits do not change bloc). Omit when `previous <= 0` or `net === 0`. |
| `➕` `✏️` `➖` | Omit each when `0`. |
| `🟰` | `➕+✏️+➖`; omit when `0`. |
| Language id | `📊uloc` + emoji + lowercase slug (`json`, `rust`, `typescript`, `csharp`) — not the old bare `🦀200k` row. |
| Total row | `📊uloc💯…` (no language slug; drop obsolete `🔢` total-row emoji). |

## Number formatting

- Line counts (`➕` `✏️` `➖` net `🟰`): raw integers.
- Bloc (`💯`): extend [`formatMetricLocCount`](🧰/🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts) with `M` (`1_300_000 → 1.3M`, `769_000 → 769k`).
- Ratio (`➗`): new `formatMetricRatio` via **3 significant figures**, then strip noise:
  - `0.001` → `0.001`
  - `10.061` → `10.1`
  - `121.001` → `121`
  - Avoid scientific notation in the string (clamp/format manually if `toPrecision` emits `e`).

## Implementation locus

Primary: [`🧰/🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts`](🧰/🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts) region `🔖uloc-metrics` (~2180–2778).

Refactor formatting around one helper used everywhere:

1. `formatUlocMetricsBody({ code, added, edited, removed, langEmoji?, langSlug? })` → full `📊uloc…` body.
2. Replace:
   - `formatMicroCommitMetricLine` / `formatMicroCommitMetricsLines` (footer total + languages)
   - `formatBundleUlocSuffix` / `formatBundleHeaderLine` / `formatBundleDateLine` (bundle + day)
3. Bundle/day `💯`: sum current unified LOC under that bundle’s path prefixes (`buildBundlePathPrefixSets` + existing uloc scan / filtered count). Day lines reuse the parent bundle’s scoped `💯` with day-specific deltas.
4. Update detectors that still expect old rows:
   - `MICRO_COMMIT_ULOC_ROW_RE` / `isMicroCommitUlocLine`
   - stdin guards that mention `🔢`
   - skill examples in [`.agents/skills/micro-commit/SKILL.md`](.agents/skills/micro-commit/SKILL.md) and [`.agents/skills/commit/SKILL.md`](.agents/skills/commit/SKILL.md)

## Tests

Extend existing cases in [`🧰/🛍️/🦑/🔨/lib/⚡️/🟦/index.test.ts`](🧰/🛍️/🦑/🔨/lib/⚡️/🟦/index.test.ts) (do not add new test files):

- `formatMetricLocCount` / new ratio formatter (`0.001`, `10.061→10.1`, `121.001→121`, `1.3M`)
- `formatMicroCommitMetricsLines` full footer shape + omit-empty
- `formatBundleUlocSuffix` / date/header lines include full body
- Keep delta-sum validation (`➕✏️➖` still sum across languages / days / bundles)

## Ticket / goal

On execute: reopen or open a ticket under goal `AI-OPTIMIZED-REPO`, keep temps in `.repo/🎫/…`, close with summary + touched files. (Repo MCP auth may be required first.)
