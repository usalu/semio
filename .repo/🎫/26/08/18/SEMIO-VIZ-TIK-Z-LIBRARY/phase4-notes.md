# Phase 4 notes

Independent review: [Opus viz library review](7846cec0-7eff-4f20-ade5-3b479d1a2d94) (Opus 5 High).

Follow-up fixes after reopen:

P0
- `test:print:viz` / `test:print:viz:full` in root package.json; launch `🧪test🖨️print📊viz` and `🧪test🖨️print📊viz🐢full`
- Hand-written `viz-api.tex` exercises every public command; `runVizCoverage` asserts those commands appear in galleries

P1
- Chrome no longer resets legend kind inside axis draw; layout passes `scale`/`orient` into axis
- Axis ticks come from `\semio_viz_scale_ticks:nN` for linear / log / band
- Layouts map y through a figure-bound `viz-y` scale; ymin padded to 0 so the smallest bar is visible
- `symlog` uses `ln(1+|x|)` compression; quantize / quantile / threshold have real branches; categorical kinds skip numeric prep
- Path demos reset mark keys only when draw is empty; `points` drive polyline/step/spline/filled-path; mark `size` is millimetres for dots too
- `runVizBuild` always recompiles and compares `pdfStableHash` after inflating Flate streams so Tectonic `/ID` bytes inside ObjStm/XRef no longer look like drift

P2
- Grid optional keys reach the axis keyset; dead data helpers removed; demo table seeded once; unknown-demo message; unit tests no longer duplicate the coverage gate
