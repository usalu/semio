# M1: TUI Chart Primitives

Paths: `U` = `🧰️framework/🔨️modules/🖱️ui`, `E` = `U/🧱️elements`, `C` = `U/⌨️tui/📈️charts` (new crate `semio-ui-tui-charts`, wired outside the widgets crate because that crate sits at its source boundary, 4006 of 4000 active lines with the new modules).

## Where

| Member | File (`🎯️targets/⌨️tui/🦀️.rs`) | Module |
|---|---|---|
| `E/🧮️ChartScale` | data model, scaling math, labels, grid | `chart_scale` |
| `E/📈️Sparkline` | one or more rows of block glyphs | `sparkline` |
| `E/📉️LineChart` | Braille or block chart | `line_chart` |
| `E/🌡️Gauge` | value vs capacity | `gauge` |
| `E/🔢️StatCell` | value and unit formatting | `stat` |

Crate root `C/🦀️.rs`; dependencies only `semio-ui-tui-core` and `semio-ui-locale`. Every widget is `data -> Vec<Vec<Mark>>` (`*_grid`) plus `paint_*` that copies the grid into a `CellBuffer` with `Theme` roles. `grid_text(&grid)` gives the snapshot text.

## API

Data (`chart_scale`): `Stats {min,max,mean,last}`, `Bucket {start,end,stats: Option<Stats>}` (half-open ms span, `None` = gap, non-finite = gap; sorted ascending, `normalize` sorts), `Aggregate {Mean,Last,Min,Max}`, `Range {Auto, Fixed(a,b)}`, `XAxis {Time, Index}`, `Level {Ok,Warn,Critical}`, `Polarity {HighIsBad, LowIsBad}`, `Locale`, `chart_labels(Locale) -> &ChartLabels` (compile-checked en/de macro: no_data, no_value, now, level words, decimal separator, duration unit words).

Math: `extent`, `resolve_range`, `eighths(v,lo,hi,levels)` (1..=levels, flat range = middle), `row_fill`, `dot_row`, `domain`, `columns(buckets, axis, domain, width)` (overlap-weighted resampling, gap only when no data overlaps), `tier_starts`, `tier_columns`, `position_column`, `level_of`.

- Sparkline: `SparklineSpec {buckets, axis, range, aggregate, band, role}`; `sparkline_grid(spec, width, rows, GlyphSet)`, `paint_sparkline(spec, theme, rect, buf)` (rect height = rows; 8 steps per row; gap glyph `·` / `?`; band `░` fills between min and max next to the value, visible from 2 rows).
- LineChart: `ChartSeries {name,buckets,role,dash: Solid|Dashed|Dotted,aggregate,band}`, `LineChartSpec {series, axis, range, unit, marks: Braille|Blocks, area, legend}`; `line_chart_grid(spec,w,h,labels,glyphs)`, `paint_line_chart`. Layout: optional legend row, plot with y labels (max top, min bottom) and `│`, axis row (`└─`, `┴` tick per tier boundary, `╌` where no series has data), label row (age of oldest sample, ages at tier boundaries when they fit, `now`). `XAxis::Time` = x proportional to time on the shared domain; `XAxis::Index` = equal share per bucket of the first series with `┊` separators at tier boundaries. Narrow widths drop y labels, then axes; no data prints the `no data` label.
- Gauge: `GaugeSpec {label,value,capacity,unit,warn,critical,polarity}` (thresholds are fractions of capacity, inclusive); `gauge_state`, `gauge_grid`, `paint_gauge`. Row: `label bar value / capacity percent level-word`; threshold ticks `╎`; narrow widths drop numbers first, then the word.
- StatCell: `Unit {Count, Bytes, Percent, Rate, ByteRate, DurationMs}`, `format_value(v, unit, labels)` (bytes 1024 steps B..PiB with rounding carry, count k/M/G/T, percent, `/s`, durations `2.5s`, `1min 5s`, `1h 30min`, `1d 1h`, `Min/Std/T` in German, decimal comma), `StatSpec {title,value,unit,level}`, `stat_grid`, `paint_stat` (level adds a glyph, never colour alone).

ASCII glyph set is supported by all widgets (Braille falls back to blocks).

## Tests

`⌨️tui/🧪️tests/🔬️charts/🦀️.rs` (+ `🥒️.feature`), crate `semio-ui-tui-laws` (module `charts` in `🥣️companion/🦀️.rs`; dev-deps added: `semio-ui-tui-charts`, `serde_json`). 19 tests, all pass:
- language-agnostic fixture `⌨️tui/🧪️tests/🧫️fixtures/📈️charts/🔣️.json` written by the independent Python oracle `🧰️scripts/M1/oracle.py` (`python3 oracle.py <path>` regenerates; prints to stdout without argument; byte-identical now). It pins eighths, dot rows, gauge fill, levels, column resampling and tier columns for uniform, gap, tiered (time and index), narrow, wider-than-buckets and NaN series, 100+ unit-format vectors (en and de), and 17 sparkline renders (empty, all gaps, gaps, one point, flat, fixed range, ramp, spikes, band, narrow, tiers in both modes, clamping, aggregates). Rust output equals Python output.
- 36 render-to-text snapshots (empty, gaps, one point, flat, spikes, band, narrow widths, non-uniform tiers in time and index mode, two series, ASCII, German, gauge levels and widths, stat cells) plus paint tests on a `CellBuffer` (roles, bold state word).

## Verification route

`repo cargo --store=shared --configuration=prototype -- check -p semio-ui-tui-laws --tests` (run in `🧰️framework`, the crates belong to that workspace) wedged for 30 min on a cold `syn` unit: the `semio-build-scheduler` wrapper idled without a rustc child, as did two peers' (libc 28 min, syn 21 min). Stopped. The tests were run instead with the repo toolchain in a scratch standalone workspace that path-includes the real files (`🧰️scripts/M1/standalone-Cargo.toml`, `standalone-test.sh`, `standalone-tests-charts.rs`; offline, no rustc wrapper, scratch target deleted afterwards): `19 passed; 0 failed`, no warnings from the new files. The shared-store run of the laws crate is therefore still open: run `repo cargo --store=shared --configuration=prototype -- test -p semio-ui-tui-laws charts` once the scheduler wedge is cleared.

## Not done

Nothing is wired into the dashboard yet (views, `Kind::Fleet`, data feed are later lanes). `repo dashboard` was not run.
