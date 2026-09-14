# GRAMMAR-CORE — status

Owner of the kernel packages `semio-viz-data`, `semio-viz-transform`, `semio-viz-scale`,
`semio-viz-format`, `semio-viz-theme` plus the visualization-palette part of
`🔨️modules/🎨print-design-token-paints/🟦️.ts`, and of the ten kernel test cases.

**Status: done.** Every deliverable is implemented, compiled and checked value-by-value against
its d3 reference. All fifteen local probe documents compile with zero LaTeX errors.

Probe command used throughout (MiKTeX, until the repo tectonic path is up):

```
cd "<ticket>/🗑️generated/GRAMMAR-CORE" \
  && export TEXINPUTS="C:/git/semio/🧰️framework/🛍️products/📓️print/🖋️latex;" \
  && xelatex -interaction=nonstopmode <probe>.tex
```

---

## 1. What is implemented

### `semio-viz-format.sty` — d3-format / d3-time-format grammar

- `\SemioVizFormat{spec}{value}`, `\SemioVizTimeFormat{spec}{iso}`, `\SemioVizLocale{en|de}`.
- Internals for renderers that want the string, not typeset output:
  `\semio_viz_format:nnN {spec}{value} str` and `\semio_viz_time_format:nnN {spec}{iso} str`
  (variants `nVN, VnN, VVN`).
- Full specifier grammar `[[fill]align][sign][symbol][0][width][,][.precision][~][type]`,
  types `d f e g s r p n b o x X c %`, `#` prefixes, `$` currency, `(` accounting sign, `~` trim,
  `^ < > =` alignment, zero-fill grouping, the SI prefixes `yzafpnµm` / `kMGTPEZY`.
- Numeric primitives rebuilt on l3fp: `toFixed`, `toExponential`, `toPrecision`, `formatRounded`,
  `formatPrefixAuto`, `formatTrim`, `group`.
- Time directives `%Y %y %m %d %e %H %I %M %S %p %j %a %A %b %B %U %W %Z %%` over
  `YYYY-MM-DD[THH:MM[:SS]]` read as UTC; weekday and day-of-year from the Fliegel–Van Flandern
  Julian day number; `%U`/`%W` by the C rule `(yday + 7 − wday) / 7`.
- Minus sign is `−` (U+2212), d3-format's own default and the correct minus for print.

### `semio-viz-scale.sty` — all fifteen taxonomy §78 kinds

- `\SemioVizScale{name}{kind}{domain}{range}[options]`; kinds `linear log pow sqrt symlog identity
  ordinal band point quantile quantize threshold sequential diverging temporal`.
- Documented option vocabulary in a `%region 🔖️Keys` block: `nice clamp padding paddingInner
  paddingOuter align base exponent constant ticks tickValues tickFormat interpolator scheme
  reverse round interval`.
- `\semio_viz_scale_map:nnN`, `\semio_viz_scale_invert:nnN`, `\semio_viz_scale_ticks:nN`,
  `\semio_viz_scale_ticks_count:nnN`, `\semio_viz_scale_tick_labels:nN`,
  `\semio_viz_scale_band_width:nN`, `\semio_viz_scale_band_step:nN`,
  `\semio_viz_scale_band_edges:nnNN`, `\semio_viz_scale_band_span:nnnNN`,
  `\semio_viz_scale_ordinal_item:nnN`, `\semio_viz_scale_binned_item:nnN`,
  `\semio_viz_scale_thresholds:n`, `\semio_viz_scale_nice_pair:nnnNN`,
  `\semio_viz_scale_tick_values:nnnN`, `\semio_viz_scale_tick_increment:nnnN`.
- Ticks are d3-array's `tickSpec`/`tickIncrement` verbatim; log ticks are d3-scale's decade rule;
  pow/sqrt/symlog are linearish, as in d3. `nice` is d3-array's `nice` loop.
- Band geometry is d3's `rescale()` including `round`, reversed ranges and `align`.
- Temporal: ISO ↔ epoch milliseconds, `interval=auto|day|week|month|year` calendar ticks
  (week is Sunday based, matching d3's `utcWeek`), civil date from a day number by Hinnant's
  algorithm.
- Colour: hex parsing, sRGB ↔ linear, **OKLab** (the library default), **CIE Lab** on d3-color's
  D50 white point, **HCL** with shortest-path hue, plain **sRGB**; plus
  `\semio_viz_color_ramp:nnnN` for multi-stop ramps (d3-interpolate's `piecewise`).
- The legacy entry points the not-yet-rewritten families still call
  (`\semio_viz_scale_define:nnnn` and its `nnxx` variant, `numeric_prep`, `range_from_index`,
  `band_range_prep`, `band_map`, `palette_categorical`, `scale_color`, `linear_apply`) are kept
  and behave as before for the default (zero padding) case.

### `semio-viz-data.sty` — tables, structures, CSV, functions, demo data

- Kept: `\SemioVizTable`, `\SemioVizRow`, the `demo` table with its original column shape.
- Added typed access and aggregates: `\semio_viz_table_col_values:nnN`, `…_col_count:nnN`,
  `…_col_median:nnN`, `…_col_quantile:nnnN` beside the existing min/max/sum/mean.
- `\SemioVizTableFromCSV{name}{file}[delimiter=,quote=,header=,columns=]` — l3file reader with
  RFC 4180 quoting (a quoted field may contain the delimiter; a doubled quote is one literal
  quote), positional `c1…cn` names for a headerless file.
- `\SemioVizHierarchy`, `\SemioVizGraph`, `\SemioVizMatrix`, `\SemioVizGeometry` declare a table's
  *kind* and its *column roles*; renderers read `\semio_viz_table_role:nn {table}{role}` and
  `\semio_viz_table_kind:n {table}` instead of hard-coding column names.
- `\SemioVizFunction{name}{expr}[domain=,samples=]` samples an fp expression in `x`.
- Demo tables: `demo` (unchanged), `demo-series` (3 series × 8 points), `demo-time` (12 ISO
  months), `demo-hierarchy` (id/parent/value/label), `demo-graph` (9 weighted edges),
  `demo-flow` (7 source/target/value flows), `demo-matrix` (4×4), `demo-geo` (3 lon/lat rings),
  `demo-distribution` (48 values in 3 groups).

### `semio-viz-transform.sty` — the taxonomy §78 transform vocabulary

`\SemioVizTransform{out}{in}[…]` with `filter sort descending group rollup pivot fold join window
bin thresholds stack order offset normalize quantile probabilities kde bandwidth kernel samples
regression degree`, applied in one documented fixed order. Highlights:

- `filter` binds bare column names inside an fp predicate.
- `rollup` = `sum mean min max count median`; `window` = `movingAverage cumulativeSum rank lag`.
- `bin` reproduces d3-array's whole rule (nice → ticks → the "extend or drop the last threshold"
  case → trim), with `thresholds=sturges | count:n | width:w | a;b;c`.
- `stack` reproduces d3-shape: orders `none ascending descending reverse appearance inside-out`,
  offsets `none expand diverging silhouette wiggle`.
- `kde` with gaussian/epanechnikov kernels and Silverman's rule when no bandwidth is given.
- `regression` = `linear poly exp log pow`; `exp` implements d3-regression's *weighted* fit, not
  the naive fit of `ln y`.
- `contour` stays with GEO-SPATIAL, as agreed in the architecture.

### `semio-viz-theme.sty` — the single colour authority of the fleet

- `\SemioVizTheme{default|grayscale|pattern}[keys]`, `\SemioVizThemeSet[keys]`; keys
  `appearance palette scheme interpolator grayscale patterns stroke font`.
- `\semio_viz_theme_color:nN {index} tl` — categorical slot → colour name (12 presence hues,
  light/dark aware; `palette=brand` for the 7 brand colours; grayscale ramp when asked).
- `\semio_viz_theme_color_count:N`, `\semio_viz_theme_scheme_color:nnN {scheme}{t} tl` (defines an
  xcolor colour on demand and returns its name), `\semio_viz_theme_scheme_hex:nnN`,
  `\semio_viz_theme_ramp_color:nN`, `\semio_viz_theme_scheme_stops:nN`.
- Schemes: sequential `primary secondary tertiary grays viridis-like`, diverging
  `primary-secondary danger-success`, per appearance.
- `\semio_viz_theme_stroke:n {hairline|default|focus}` reads `\semio@stroke@…` from the tokens;
  `\semio_viz_theme_font:n {sans|serif|mono}`; `\semio_viz_theme_pattern:nN` gives grayscale-safe
  hatches.

### `🔨️modules/🎨print-design-token-paints/🟦️.ts` — generator

`renderVisualizationPalette()` (called from `renderPrintLatexTokenStylesheet`) now emits into
`semio-tokens.sty`:

- `\definecolor{semio-presence-<light|dark>-<0..11>}` from `presence.hues` × `presence.<theme>`
  (HSL → hex, byte-identical to the CSS palette the UI ships),
- `\newcommand{\semio@viz@presence@count}`,
- `\csname semio@viz@scheme@<name>@<theme>\endcsname` stop lists for the seven schemes, computed
  from the tokens with the repo's own `oklabMix`.

Regenerated with `bun ./📜️script.ts generate` in `📓️print/📦️packages/🟦️typescript` — exit 0.

---

## 2. Tests

Ten cases under `🧰️framework/🛍️products/📓️print/🧪️tests/`, each with `🥒️.feature`, `🟦️.ts` and
committed `🧫️fixtures/`, written against the TESTS-HARNESS contract (§1 of
`📓️status-TESTS-HARNESS.md`): `compileVizProbe` + `probeProjection` + `roundProbeNumbers`,
`@comparison-viz-probe-v1`, vectors in the Gherkin data tables.

| case | oracle | scenarios |
|---|---|---|
| `format-number` | `d3-format` | en-locale, de-locale (28 specifiers each) |
| `format-time` | `d3-time-format` | en-locale, de-locale (12 directives each) |
| `scale-continuous` | `d3-scale` | linear, log, pow, symlog, nice |
| `scale-discrete` | `d3-scale` | band, point, ordinal, quantize, quantile, threshold |
| `scale-color` | `d3-scale`, `d3-interpolate`, `d3-color` | sequential-position, diverging-position, interpolate-rgb, interpolate-lab, interpolate-hcl, ramp-stops |
| `scale-temporal` | `d3-scale`, `d3-time` | map, day-ticks, week-ticks, month-ticks, year-ticks |
| `transform-stack` | `d3-shape` | order-none, order-ascending, order-reverse, offset-expand, offset-silhouette, offset-diverging |
| `transform-bin` | `d3-array` | sturges, count-five, count-twenty, explicit-thresholds |
| `transform-statistics` | `d3-array`, `d3-regression`, `@no-oracle-kernel-density` | aggregates, quantiles, kde, regression |
| `data-csv` | `d3-dsv` | header-row, quoted-fields, custom-delimiter |

**Verification actually performed.** The repo test platform could not be run from here (the
`compileVizProbe` tectonic path is TESTS-HARNESS's deliverable and was not finished while this
work ran), so every fixture was compiled locally with xelatex and its `.probe.jsonl` was diffed
against the very oracle expression the adapter uses, run through node against the repo's own
`node_modules`. Result: **every value matches**, including

- all 28 number specifiers in both locales, character for character
  (`.2s`→`420µ`, `012,.2f`→`0,001,234.50`, `(.2f`→`(3.50)`, de `,.2f`→`1.234.567,89`);
- all 12 time directives in both locales (`%A %B %e, %Y` → `Saturday September  5, 2026`,
  de → `Samstag September  5, 2026`; `%j %U %W %Z` on 2024-02-29 → `060 08 09 +0000`);
- every continuous map/invert/tick/nice value, e.g. `nice([0.1,0.9],10) = [0.1,0.9]`,
  `nice([12,87],4) = [0,100]`, the 28 log ticks of `[1,1000]`;
- every band start/width/step under four padding and direction settings;
- the quantize/quantile/threshold buckets *and* their thresholds;
- the rgb/lab/hcl mixes and piecewise ramps, hex for hex;
- the temporal positions and the day/week/month/year tick dates;
- all six stack scenarios (`y0` and `y1` of every series);
- all four binnings (`x0`, `x1`, `count`);
- mean/median/sum/min/max/count, R-7 quantiles, both kernels, and all five regressions;
- the CSV parse including a quoted comma and a doubled quote.

Final regression, all fifteen probe documents (five exploratory + ten fixtures):

```
f1 errors=0                 scale-continuous errors=0     transform-stack errors=0
s1 errors=0                 scale-discrete errors=0       transform-bin errors=0
d1 errors=0                 scale-color errors=0          transform-statistics errors=0
x1 errors=0                 scale-temporal errors=0       data-csv errors=0
h1 errors=0                 format-number errors=0        format-time errors=0
```

`\usepackage{semio-viz}` (the full bundle) loads and produces a PDF; the only errors left in that
run are the local font lookup and a `semio-window` chrome box that follows from it — nothing from
the five kernel packages.

---

## 3. Requests to other agents

**TESTS-HARNESS** — two things:

1. Please register a `noOracleDecisions` entry for **`kernel-density`**. Justification: d3 ships no
   kernel density estimator at version 3, so `transform-statistics`'s `kde` scenario is checked
   against an independent second implementation of the published formula
   `(1/(n·h)) · Σ K((x − xᵢ)/h)` written in the adapter (`🧪️tests/transform-statistics/🟦️.ts`),
   with the standard Gaussian and Epanechnikov kernels — not against a re-derivation of the LaTeX
   code. Everything else in that case has a real oracle.
2. My ten cases are written against §1 of your status file and use `extraSources` only in
   `data-csv` (it needs `cities.csv` and `places.tsv` staged next to the probe). If `compileVizProbe`
   resolves fixture siblings automatically, that argument can be dropped.

**CHARTS-B** — `semio-viz-charts-distribution.sty` reimplements d3's bin, the R-7 quantile and the
1-2-5 tick factor locally. Those now exist in the kernel and are oracle-tested:
`\SemioVizTransform{out}{in}[bin=<col>,thresholds=…]`, `\semio_viz_tr_quantile_of:nN`,
`\semio_viz_table_col_quantile:nnnN`, `\semio_viz_scale_tick_values:nnnN`,
`\semio_viz_scale_tick_increment:nnnN`, `\semio_viz_scale_nice_pair:nnnNN`. Please move onto them
so there is one binning implementation rather than two.

**Everyone** — colours come from `semio-viz-theme` only:
`\semio_viz_theme_color:nN {index} tl` for a categorical slot and
`\semio_viz_theme_scheme_color:nnN {scheme}{t} tl` for a ramp (it defines the xcolor colour for you
and hands back its name). Do not call `\semio_viz_scale_color:nnN` (the legacy 7-colour fallback)
or name `semio-…` token colours directly in a renderer.

---

## 4. Decisions worth knowing

- **No default language, and no `semio-core` dependency in the kernel.** `semio-viz-format` and
  `semio-viz-theme` read `\l_semio_language_tl` / `\l_semio_theme_tl` *if the variable exists* and
  otherwise take the explicit override (`\SemioVizLocale`, `appearance=`); the locale has no
  fallback and errors out, the appearance falls back to `light`. That keeps kernel probe documents
  font-free — a probe is `\documentclass{article}` + `\usepackage{semio-viz-<pkg>}`, nothing else.
- **`%`, `#` and `~` in probe fixtures.** `%` cannot appear literally in a `.tex`. The idiom used
  in the format fixtures is `\char_set_catcode_other:n { 37 }` … `\char_set_catcode_comment:n { 37 }`
  around the block; for a single specifier, `\exp_args:Nx \cmd { … \c_percent_str … }` (likewise
  `\c_hash_str`, `\c_tilde_str`).
- **String transport in the probe protocol.** The protocol classifies a bare decimal as a JSON
  number, which would silently collapse `4.200e+1` to `42`. Every string-valued probe in these
  cases is therefore prefixed with `|` and the oracle mirrors it; padded results encode their fill
  spaces as `_`. Both are undone on the oracle side, so the comparison stays exact.
- **Rounding.** l3fp breaks ties to even, JavaScript breaks them away from zero, so
  `\semio_viz_format_fixed:nnN` and every place mirroring `Math.round` compute `floor(x + 0.5)`
  explicitly. l3fp is a decimal arithmetic and IEEE-754 is not, so a value whose decimal expansion
  sits exactly on a rounding boundary of a non-tie double can differ in the last digit; that is
  documented in the `format-number` feature and the vectors avoid the ambiguity.
- **Catcodes in key values.** `:` is a letter inside `\ExplSyntaxOn` and other inside a document,
  so `stack=t:series:value` written at document level would not split against a package-internal
  colon. `\semio_viz_tr_args:N` stringifies the value and splits on `\c_colon_str`. Any agent
  splitting a key value on a punctuation character must do the same.
- **No digits in expl3 variable names.** Under `\ExplSyntaxOn` a digit terminates a control
  sequence name, so `\l_…_x0_fp` silently becomes `\l_…_x` followed by `0_fp`. Names here use
  `xlo/xhi/ylo/yhi`, `eten/efive/etwo`, `ione/itwo`. Worth knowing — the failure mode is a
  confusing "already defined" error far from the cause.
- **`\bool_do_while:nn` runs its body before the first test.** Two trimming loops were silently
  eating an element until they were changed to `\bool_while_do:nn`.
- **`\fp_compare:nNnTF` and `\int_compare:nNnTF` take a single relation token**, so `>=` must be
  written as `\fp_compare:nTF { a >= b }`.

## 5. Files touched

Created or rewritten:

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-format.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scale.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-transform.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-theme.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-data.sty` (extended; `demo` unchanged)
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-tokens.sty` (generated output)
- `🧰️framework/🛍️products/📓️print/🧪️tests/{format-number,format-time,scale-continuous,
  scale-discrete,scale-color,scale-temporal,transform-stack,transform-bin,transform-statistics,
  data-csv}/{🥒️.feature,🟦️.ts,🧫️fixtures/*}`

Not touched: the loader `semio-viz.sty`, the family registry, any other agent's package,
`📦️packages/🟦️typescript/package.json` (the d3 devDependencies TESTS-HARNESS added already cover
every oracle these cases need — no new dependency was required).
