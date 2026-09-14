// 🔬 Compares the locally compiled CHARTS-B probe fixtures against their d3 oracles, the same
// comparison the 🧪️tests adapters make once the repo test platform runs them.
// Run from the repository root: node ".🧬semio/🦑️repo/🎫️tickets/…/🔬verify-charts-b.mjs" <fx-dir>
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { bin, ticks as d3ticks, quantileSorted, deviation } from 'd3-array'
import { pie as d3pie } from 'd3-shape'
import { scaleBand } from 'd3-scale'

const dir = process.argv[2]
const read = (name) =>
  Object.fromEntries(
    readFileSync(join(dir, name), 'utf8')
      .trim()
      .split('\n')
      .map((line) => JSON.parse(line))
      .reduce((acc, r) => { (acc.get(r.key) ?? acc.set(r.key, []).get(r.key)).push(...r.values); return acc }, new Map())
  )

const DIST = [
  4.2, 5.1, 4.8, 6.3, 5.6, 5.9, 6.8, 4.5, 7.1, 5.4, 6.0, 5.2, 6.6, 4.9, 5.8, 7.4, 6.1, 5.5, 4.4, 6.9,
  8.3, 7.6, 9.1, 8.8, 7.9, 8.1, 9.6, 7.2, 8.5, 9.3, 8.0, 7.7, 8.9, 9.8, 8.2, 7.4, 8.6, 9.0, 7.8, 8.4,
  3.1, 2.6, 3.8, 2.9, 3.4, 2.2, 3.6, 2.8,
]
const PARTS = [34, 26, 18, 12, 7, 3]
const SCORES = [0.95, 0.90, 0.86, 0.81, 0.77, 0.71, 0.66, 0.60, 0.55, 0.48, 0.42, 0.37, 0.31, 0.25, 0.18, 0.09]
const LABELS = [1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0]
const MATRIX = [4, 7, 2, 9, 6, 1, 8, 3, 5, 9, 4, 6, 2, 3, 7, 8]

let failures = 0
const near = (a, b, tol = 1e-9) => Math.abs(a - b) <= tol * Math.max(1, Math.abs(a), Math.abs(b))
function check(name, subject, oracle, tol = 1e-9) {
  const ok = subject.length === oracle.length && subject.every((v, i) => near(Number(v), Number(oracle[i]), tol))
  console.log((ok ? 'PASS ' : 'FAIL ') + name + '  n=' + subject.length)
  if (!ok) {
    failures++
    console.log('  subject', subject.slice(0, 12).join(','))
    console.log('  oracle ', oracle.slice(0, 12).join(','))
  }
}

// 📊 histogram bins against d3-array's bin
for (const [file, count] of [['bins', 10], ['bins-coarse', 4]]) {
  const p = read(`charts-histogram-density__${file}.probe.jsonl`)
  const b = bin().thresholds(count)(DIST)
  check(`hist/${file} lower`, p['bin-lower'], b.map((d) => d.x0))
  check(`hist/${file} upper`, p['bin-upper'], b.map((d) => d.x1))
  check(`hist/${file} count`, p['bin-count'], b.map((d) => d.length))
}

// 🪜 ticks against d3-array's ticks
{
  const p = read('charts-histogram-density__ticks.probe.jsonl')
  check('hist/ticks-a', p['ticks-a'], d3ticks(0, 12, 10))
  check('hist/ticks-b', p['ticks-b'], d3ticks(2.4, 11.3, 7))
  check('hist/ticks-c', p['ticks-c'], d3ticks(0, 1, 5), 1e-12)
}

// 🌊 gaussian KDE against the TypeScript oracle built on d3-array's deviation and quantileSorted
{
  const p = read('charts-histogram-density__density.probe.jsonl')
  const sorted = [...DIST].sort((a, b) => a - b)
  const iqr = (quantileSorted(sorted, 0.75) - quantileSorted(sorted, 0.25)) / 1.349
  const h = 1.06 * Math.min(deviation(DIST), iqr) * Math.pow(DIST.length, -0.2)
  const grid = Array.from({ length: 9 }, (_, i) => 2 + (10 - 2) * (i / 8))
  const kde = (band) => grid.map((x) => DIST.reduce((a, xi) => a + Math.exp(-0.5 * ((x - xi) / band) ** 2), 0) / (DIST.length * band * Math.sqrt(2 * Math.PI)))
  check('hist/kde bandwidth', p['bandwidth'], [h], 1e-12)
  check('hist/kde grid', p['grid'], grid)
  check('hist/kde density', p['density'], kde(h), 1e-12)
  check('hist/kde fixed', p['fixed-density'], kde(0.75), 1e-12)
}

// 📶 the empirical CDF against the sorted sample
{
  const p = read('charts-histogram-density__ecdf.probe.jsonl')
  const sorted = [...DIST].sort((a, b) => a - b)
  check('hist/ecdf x', p['ecdf-x'], sorted)
  check('hist/ecdf y', p['ecdf-y'], sorted.map((_, i) => (i + 1) / sorted.length), 1e-12)
}

// 📦 quantiles against d3-array's quantileSorted
{
  const p = read('charts-box-violin__quartiles.probe.jsonl')
  const sorted = [...DIST].sort((a, b) => a - b)
  check('box/quantiles', p['quantile'], [0, 0.05, 0.25, 0.5, 0.75, 0.95, 1].map((q) => quantileSorted(sorted, q)), 1e-12)
}

// 📦 the box summaries of the three groups against d3-array's quantiles plus Tukey whiskers
{
  const p = read('charts-box-violin__whiskers.probe.jsonl')
  const groups = [DIST.slice(0, 20), DIST.slice(20, 40), DIST.slice(40)]
  const oracle = groups.flatMap((g) => {
    const s = [...g].sort((a, b) => a - b)
    const q1 = quantileSorted(s, 0.25), q2 = quantileSorted(s, 0.5), q3 = quantileSorted(s, 0.75)
    const iqr = q3 - q1
    const lo = Math.min(...s.filter((v) => v >= q1 - 1.5 * iqr))
    const hi = Math.max(...s.filter((v) => v <= q3 + 1.5 * iqr))
    return [lo, q1, q2, q3, hi]
  })
  check('box/whiskers', p['geometry/box'], oracle, 1e-12)
}

// 🥧 pie angles against d3-shape's pie
{
  for (const [file, sort, start, end, pad] of [
    ['pie', 'desc', 0, 2 * Math.PI, 0],
    ['pie-unsorted', 'none', 0, 2 * Math.PI, 0],
    ['pie-padded', 'desc', 0.5, 3.641592653589793, 0.02],
  ]) {
    const p = read(`charts-pie-donut__${file}.probe.jsonl`)
    const layout = d3pie().value((d) => d).startAngle(start).endAngle(end).padAngle(pad)
    if (sort === 'none') layout.sort(null); else layout.sort((a, b) => b - a)
    const arcs = layout(PARTS)
    check(`pie/${file} start`, p['start-angle'], arcs.map((a) => a.startAngle), 1e-12)
    check(`pie/${file} end`, p['end-angle'], arcs.map((a) => a.endAngle), 1e-12)
  }
}

// 🔥 heatmap cell rectangles against d3-scale's band scales
{
  const p = read('charts-heatmap-matrix__heatmap.probe.jsonl')
  const x0 = 4, x1 = 60, y0 = 4, y1 = 36
  const cols = scaleBand().domain(['C1', 'C2', 'C3', 'C4']).range([x0, x1])
  const rows = scaleBand().domain(['R1', 'R2', 'R3', 'R4']).range([y0, y1])
  const oracle = []
  for (let r = 0; r < 4; r++) for (let c = 0; c < 4; c++) oracle.push(cols(`C${c + 1}`), rows(`R${r + 1}`), cols.bandwidth(), rows.bandwidth())
  check('heatmap/rects', p['geometry/rect'], oracle, 1e-12)
  const min = Math.min(...MATRIX), max = Math.max(...MATRIX)
  const cells = []
  for (let r = 0; r < 4; r++) for (let c = 0; c < 4; c++) { const v = MATRIX[r * 4 + c]; cells.push(r + 1, c + 1, v, (v - min) / (max - min)) }
  check('heatmap/cells', p['geometry/cell'], cells, 1e-12)
}

// 🔥 the padded heatmap insets every cell by the padding fraction
{
  const p = read('charts-heatmap-matrix__heatmap-padded.probe.jsonl')
  const x0 = 4, x1 = 60, y0 = 4, y1 = 36, pad = 0.2
  const w = (x1 - x0) / 4, h = (y1 - y0) / 4
  const oracle = []
  for (let r = 0; r < 4; r++) for (let c = 0; c < 4; c++) oracle.push(x0 + c * w + (w * pad) / 2, y0 + r * h + (h * pad) / 2, w * (1 - pad), h * (1 - pad))
  check('heatmap/padded', p['geometry/rect'], oracle, 1e-12)
}

// 📈 ROC, PR and gain sweeps against the TypeScript oracle built on d3-array's ordering
{
  const order = SCORES.map((s, i) => i).sort((a, b) => SCORES[b] - SCORES[a])
  const P = LABELS.filter((l) => l > 0.5).length, N = LABELS.length - P
  const roc = { x: [0], y: [0] }, pr = { x: [], y: [] }, gain = { x: [0], y: [0] }
  let tp = 0, fp = 0, auc = 0
  order.forEach((i, k) => {
    if (LABELS[i] > 0.5) tp++
    else { auc += tp / (P * N); fp++ }
    roc.x.push(fp / N); roc.y.push(tp / P)
    pr.x.push(tp / P); pr.y.push(tp / (tp + fp))
    gain.x.push((k + 1) / LABELS.length); gain.y.push(tp / P)
  })
  const r = read('charts-evaluation-curves__roc.probe.jsonl')
  check('eval/roc fpr', r['fpr'], roc.x, 1e-12)
  check('eval/roc tpr', r['tpr'], roc.y, 1e-12)
  check('eval/roc auc', r['auc'], [auc], 1e-12)
  const q = read('charts-evaluation-curves__pr.probe.jsonl')
  check('eval/pr recall', q['recall'], pr.x, 1e-12)
  check('eval/pr precision', q['precision'], pr.y, 1e-12)
  const g = read('charts-evaluation-curves__gain.probe.jsonl')
  check('eval/gain population', g['population'], gain.x, 1e-12)
  check('eval/gain captured', g['captured'], gain.y, 1e-12)
  const c = read('charts-evaluation-curves__confusion.probe.jsonl')
  const sorted = [...SCORES].sort((a, b) => a - b)
  const t = quantileSorted(sorted, 0.5)
  let TP = 0, FP = 0, FN = 0, TN = 0
  SCORES.forEach((s, i) => { if (s >= t) { LABELS[i] > 0.5 ? TP++ : FP++ } else { LABELS[i] > 0.5 ? FN++ : TN++ } })
  check('eval/confusion', c['geometry/confusion'], [TP, FP, FN, TN])
}

// 📦 letter values and the violin's inner box against d3-array's quantiles
{
  const G = [DIST.slice(0, 20), DIST.slice(20, 40), DIST.slice(40)]
  const letters = (g, d) => {
    const s = [...g].sort((a, b) => a - b)
    return Array.from({ length: d }, (_, k) => { const p = 0.5 ** (k + 2); return [k + 1, quantileSorted(s, p), quantileSorted(s, 1 - p)] }).flat()
  }
  const summary = (g) => {
    const s = [...g].sort((a, b) => a - b)
    const q1 = quantileSorted(s, 0.25), q2 = quantileSorted(s, 0.5), q3 = quantileSorted(s, 0.75), i = q3 - q1
    return [Math.min(...s.filter((v) => v >= q1 - 1.5 * i)), q1, q2, q3, Math.max(...s.filter((v) => v <= q3 + 1.5 * i))]
  }
  check('box/letter', read('charts-box-violin__letter.probe.jsonl')['geometry/letter'], G.flatMap((g) => letters(g, 3)), 1e-12)
  check('box/violin-inner', read('charts-box-violin__violin.probe.jsonl')['geometry/box'], G.flatMap(summary), 1e-12)
}

// 🥧 the donut annulus against d3-shape's angles inside the frame's inscribed circle
{
  const arcs = d3pie().value((d) => d).sort((a, b) => b - a)(PARTS)
  check('pie/donut', read('charts-pie-donut__donut.probe.jsonl')['geometry/arc'], arcs.flatMap((a) => [30, 17, 14 * 0.55, 14, a.startAngle, a.endAngle]), 1e-12)
}

// 🌀 the polar coordinate against d3-scale's radius scale plus the two projections
{
  const TAU = 2 * Math.PI
  const SPEED = [4.2, 5.6, 6.8, 3.9, 5.1, 7.4, 6.2, 4.7]
  const COUNT = [12, 9, 17, 7, 14, 21, 16, 10]
  const PROFILE = [8, 4, 6, 9, 3, 5]
  const f = { cx: 30, cy: 20, radius: 16 }
  const scaleLinear = (d0, d1, r0, r1) => (v) => r0 + ((v - d0) / (d1 - d0)) * (r1 - r0)
  let r = scaleLinear(0, Math.max(...SPEED), f.radius * 0.15, f.radius)
  check('polar/scatter', read('charts-polar-radar__polar-scatter.probe.jsonl')['geometry/circle'],
    SPEED.flatMap((v, i) => { const a = (TAU * i) / 8; return [f.cx + r(v) * Math.sin(a), f.cy + r(v) * Math.cos(a), 0.9] }), 1e-9)
  r = scaleLinear(0, Math.max(...SPEED), f.radius * 0.2, f.radius)
  check('polar/bars', read('charts-polar-radar__polar-bars.probe.jsonl')['geometry/arc'],
    SPEED.flatMap((v, i) => [f.cx, f.cy, f.radius * 0.2, r(v), (TAU * i) / 8 + 0.03, (TAU * (i + 1)) / 8 - 0.03]), 1e-9)
  const ring = PROFILE.flatMap((_, i) => { const a = (TAU * i) / 6; return [f.cx + f.radius * Math.sin(a), f.cy + f.radius * Math.cos(a)] })
  const profile = PROFILE.flatMap((v, i) => { const a = (TAU * i) / 6, rr = (f.radius * v) / 9; return [f.cx + rr * Math.sin(a), f.cy + rr * Math.cos(a)] })
  check('polar/radar', read('charts-polar-radar__radar.probe.jsonl')['geometry/polygon'], [...ring, ...profile], 1e-9)
  check('polar/coxcomb', read('charts-polar-radar__coxcomb.probe.jsonl')['geometry/arc'],
    COUNT.flatMap((v, i) => [f.cx, f.cy, 0, f.radius * Math.sqrt(v / 21), (TAU * i) / 8, (TAU * (i + 1)) / 8]), 1e-9)
}

// 🎛️ quadrant grids against d3-scale's band scale, table layout against its specification
{
  const quad = (w, h, n) => {
    const d = Array.from({ length: n }, (_, i) => String(i))
    const c = scaleBand().domain(d).range([0, w]), rr = scaleBand().domain(d).range([0, h])
    const o = []
    for (const cc of d) for (const r2 of d) o.push(c(cc), rr(r2), c.bandwidth(), rr.bandwidth())
    return o
  }
  check('quadrant/2x2', read('charts-quadrant-table__quadrant.probe.jsonl')['geometry/rect'], quad(40, 40, 2), 1e-12)
  check('quadrant/3x3', read('charts-quadrant-table__risk.probe.jsonl')['geometry/rect'], quad(45, 45, 3), 1e-12)
  const t = read('charts-quadrant-table__table.probe.jsonl')
  check('table/anchors', [t['geometry/text'][0], t['geometry/text'][1], t['geometry/text'][8], t['geometry/text'][9], t['geometry/text'][48], t['geometry/text'][49]], [0, 40, 0, 32, 0, 12])
  check('table/rule', t['geometry/line'], [0, 37.6, 60, 37.6])
  check('table/bars', read('charts-quadrant-table__table-bars.probe.jsonl')['geometry/rect'],
    [20, 31.1, 6.2, 2.2, 20, 27.1, 18.6, 2.2, 20, 23.1, 0, 2.2, 20, 19.1, 14.3375, 2.2, 20, 15.1, 9.6875, 2.2, 20, 11.1, 11.2375, 2.2], 1e-9)
}

console.log(failures === 0 ? '\nALL CHECKS PASS' : `\n${failures} CHECK(S) FAILED`)
process.exit(failures === 0 ? 0 : 1)
