/** 🥚️ Rounds the shoulders and the two ends of every body that `../MV-3/body.ts --save` found: `bun caps.ts <raw ops.json> <ops.json> [--deep 0.1] [--round 1] [--reach 1] [--share 0.4] [--paper 0.1] [--samples 160] [--along 0] [--last 0.06] [--slim 0]`. The copy of round MV-L3 of `../MV-L2/caps.ts`, with the two last flags added and the plan bodies rounded across their width (`body.ts` gives a plan section its depth as `rx` and its width as `ry`; the copy of MV-L2 rounded the depth of the plan bodies with the large circle and left the shelves of their width, which showed as straight faces in the openings of the plan in the wind).
 *
 * A body of `body.ts` is a row of parallel elliptic sections: across, the lateral span inside the drawn outlines of its view; deep, the depth span of the elevation outlines. Where an outline has a level edge the span jumps from one station to the next, so the body has a flat shelf there, and it ends in a flat lid; when the wind tilts the mass a shelf or a lid shows as a straight cut as wide as the mass. This step makes the body round across, inside the body it came from:
 * - across (the direction the view draws): the low and the high edge of the sections along the stations are opened with a circle (erosion, then dilation; radius `round` times the largest half width of the body, at most half its length: a larger circle would leave a short wide mass narrow and deep at both ends, which showed as a tongue above and below it in `gale`), which rounds every shoulder above a step and leaves necks as they are, and both ends are drawn in to a dome over the distance D from the end (the half width there times `reach`, at most `share` of the length of the body) by sqrt(1 - (1 - d / D)²), down to 6 % at the end itself. The new span lies inside the old one at every station, so the default pose of the view draws what it drew before;
 * - deep (the direction the view looks along): the shoulders are opened with a circle of `deep` metres only, a share of the tolerance of the asset, because the depth of the body is what carries the extent of the elevation into the view when the bone turns (the gate compares them); at the two ends the depth takes the dome of the width, so an end is a rounded tip and not a deep blade (which showed as a tongue above and below a mass in `gale`);
 * - along (MV-L3): with `--along` metres every station first takes the span it shares with the stations within that distance, as far as they share one (the window is halved until they do), so the face of a shelf beside a level edge or an opening of the outline stands back by that distance and a mass that the wind shifts by less does not show a straight face in the opening; `--last` is the share of the width an end keeps (the lid of the dome); with `--slim` k a section is at most k times as deep as it is wide, so a narrow spur of a mass is no deep blade (a blade showed as a tongue hanging below its mass when the wind pitched it: `black-alder` front `gale`);
 * - the stations are the ones of the body and `samples` even ones; they are kept until the straight run between two of them is within `paper` millimetres on the paper of the scale of every edge.
 * The output is a pure function of the input, so the step is idempotent. Nothing else of the ops is touched.
 * @see ../MV-3/body.ts — the bodies and the ops format
 * @see ./rest.ts — what a body shows beside the drawn outlines in the default pose */
import { readFileSync, writeFileSync } from "node:fs";

type V3 = [number, number, number];
type Section = { at: number[]; rx: number; ry: number; rotation: number[] };
type Row = { s: number; x0: number; x1: number; y0: number; y1: number };
const args = process.argv.slice(2);
const flag = (name: string, value: number): number => (args.indexOf(`--${name}`) < 0 ? value : Number(args[args.indexOf(`--${name}`) + 1]));
const [DEEP, ROUND, REACH, SHARE, PAPER, SAMPLES, TINY, LAST] = [flag("deep", 0.1), flag("round", 1), flag("reach", 1), flag("share", 0.4), flag("paper", 0.1), flag("samples", 160), 5e-4, flag("last", 0.06)]
const ALONG = flag("along", 0);
const SLIM = flag("slim", 0);
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const dot = (a: number[], b: number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
const ops = JSON.parse(readFileSync(args[0]!, "utf8"));

/** 🧭️ The columns of the rotation of a unit quaternion `[x, y, z, w]`. */
function columns([x, y, z, w]: number[]): [V3, V3, V3] {
  return [[1 - 2 * (y! * y! + z! * z!), 2 * (x! * y! + z! * w!), 2 * (x! * z! - y! * w!)], [2 * (x! * y! - z! * w!), 1 - 2 * (x! * x! + z! * z!), 2 * (y! * z! + x! * w!)], [2 * (x! * z! + y! * w!), 2 * (y! * z! - x! * w!), 1 - 2 * (x! * x! + y! * y!)]];
}

/** ⚪️ An edge sampled at the stations `s`, opened with a circle of radius `r` (its hypograph eroded, then dilated), the edge continued level beyond both ends. */
function opened(s: number[], edge: number[], r: number): number[] {
  if (!(r > 0)) return [...edge];
  const step = (s[s.length - 1]! - s[0]!) / SAMPLES;
  const pad = Math.ceil(r / step) + 1;
  const where = [...Array.from({ length: pad }, (_, i) => s[0]! - (pad - i) * step), ...s, ...Array.from({ length: pad }, (_, i) => s[s.length - 1]! + (i + 1) * step)];
  const value = [...Array.from({ length: pad }, () => edge[0]!), ...edge, ...Array.from({ length: pad }, () => edge[edge.length - 1]!)];
  const eroded = where.map((at) => { let low = Infinity; for (let j = 0; j < where.length; j++) { const t = where[j]! - at; if (Math.abs(t) <= r) low = Math.min(low, value[j]! - Math.sqrt(r * r - t * t)); } return low; });
  return s.map((at, i) => { let high = -Infinity; for (let j = 0; j < where.length; j++) { const t = where[j]! - at; if (Math.abs(t) <= r) high = Math.max(high, eroded[j]! + Math.sqrt(r * r - t * t)); } return Math.min(edge[i]!, high); });
}

/** 🥚️ The sections of one body, rounded; `budget` is the allowed distance of a straight run from an edge in metres; `turned` says that the second radius of a section is the one the view draws across (the plan bodies of `body.ts`: `rx` is their depth, `ry` their width). */
function rounded(sections: Section[], budget: number, turned: boolean): Section[] {
  const [e1, e2, en] = columns(sections[0]!.rotation);
  const [ex, ey] = turned ? [e2, e1] : [e1, e2];
  const rows: Row[] = sections.map((section) => { const [hx, hy] = turned ? [section.ry, section.rx] : [section.rx, section.ry]; return { s: dot(section.at, en), x0: dot(section.at, ex) - hx, x1: dot(section.at, ex) + hx, y0: dot(section.at, ey) - hy, y1: dot(section.at, ey) + hy }; });
  if (rows[0]!.s > rows[rows.length - 1]!.s) rows.reverse();
  const length = rows[rows.length - 1]!.s - rows[0]!.s;
  if (!(length > 0) || rows.some((row, i) => i > 0 && row.s <= rows[i - 1]!.s)) return sections;
  const old = (s: number): Row => {
    let i = 0;
    while (i + 2 < rows.length && rows[i + 1]!.s < s) i++;
    const [a, b] = [rows[i]!, rows[i + 1]!];
    const t = Math.max(0, Math.min(1, (s - a.s) / (b.s - a.s)));
    return { s, x0: a.x0 + (b.x0 - a.x0) * t, x1: a.x1 + (b.x1 - a.x1) * t, y0: a.y0 + (b.y0 - a.y0) * t, y1: a.y1 + (b.y1 - a.y1) * t };
  };
  const s = [...new Set<number>([...rows.map((row) => row.s), ...Array.from({ length: SAMPLES + 1 }, (_, i) => rows[0]!.s + (length * i) / SAMPLES)])].sort((a, b) => a - b).filter((at, i, list) => i === 0 || at - list[i - 1]! > 1e-7);
  const level = s.map(old);
  const was = level.map((row, i) => {
    for (let reach = ALONG; reach > 1e-3; reach /= 2) {
      let [x0, x1] = [row.x0, row.x1];
      for (let j = 0; j < s.length; j++) if (Math.abs(s[j]! - row.s) <= reach) [x0, x1] = [Math.max(x0, level[j]!.x0), Math.min(x1, level[j]!.x1)];
      if (x1 - x0 > Math.min(0.5 * (row.x1 - row.x0), 4 * TINY)) return { ...row, x0, x1 };
    }
    return row;
  });
  const rx = Math.max(...was.map((row) => (row.x1 - row.x0) / 2));
  const wide = Math.min(rx, length / 2) * ROUND;
  const x1 = opened(s, was.map((row) => row.x1), wide), x0 = opened(s, was.map((row) => -row.x0), wide).map((value) => -value);
  const y1 = opened(s, was.map((row) => row.y1), DEEP), y0 = opened(s, was.map((row) => -row.y0), DEEP).map((value) => -value);
  const last = s.length - 1;
  const dome = (at: number, d0: number, d1: number): number => Math.max(LAST, Math.min(d0 > 0 && at - s[0]! < d0 ? Math.sqrt(Math.max(0, 1 - (1 - (at - s[0]!) / d0) ** 2)) : 1, d1 > 0 && s[last]! - at < d1 ? Math.sqrt(Math.max(0, 1 - (1 - (s[last]! - at) / d1) ** 2)) : 1));
  const across = (i: number): number => { const half = (x1[i]! - x0[i]!) / 2; return half <= 3 * TINY ? 0 : Math.min(half * REACH, SHARE * length); };
  const made = s.map((at, i) => ({ s: at, cx: (x0[i]! + x1[i]!) / 2, cy: (y0[i]! + y1[i]!) / 2, hx: Math.max(TINY, ((x1[i]! - x0[i]!) / 2) * dome(at, across(0), across(last))), hy: Math.max(TINY, ((y1[i]! - y0[i]!) / 2) * dome(at, across(0), across(last))) })).map((row) => (SLIM > 0 ? { ...row, hy: Math.max(TINY, Math.min(row.hy, SLIM * row.hx)) } : row));
  const kept = new Set<number>([0, last]);
  const split = (i: number, j: number): void => {
    let worst = budget, where = -1;
    for (let k = i + 1; k < j; k++) {
      const t = (made[k]!.s - made[i]!.s) / (made[j]!.s - made[i]!.s);
      const miss = Math.max(...(["cx", "cy", "hx", "hy"] as const).map((name) => Math.abs(made[k]![name] - made[i]![name] - t * (made[j]![name] - made[i]![name]))));
      if (miss > worst) { worst = miss; where = k; }
    }
    if (where < 0) return;
    kept.add(where);
    split(i, where);
    split(where, j);
  };
  split(0, last);
  return [...kept].sort((a, b) => a - b).map((k) => { const row = made[k]!; return { at: [0, 1, 2].map((axis) => round(row.s * en[axis]! + row.cx * ex[axis]! + row.cy * ey[axis]!)), rx: round(turned ? row.hy : row.hx), ry: round(turned ? row.hx : row.hy), rotation: sections[0]!.rotation }; });
}

let count = 0, before = 0, after = 0;
for (const key of Object.keys(ops)) for (const view of Object.keys(ops[key])) for (const entry of ops[key][view].bodies ?? []) { before += entry.shape.sections.length; entry.shape.sections = rounded(entry.shape.sections, ((PAPER / 1000) * Number(key)) / 2, view === "plan"); after += entry.shape.sections.length; count++; }
writeFileSync(args[1]!, `${JSON.stringify(ops)}\n`);
console.log(`${count} bodies rounded at their shoulders and ends (deep ${DEEP} m, round ${ROUND}, along ${ALONG} m, last ${LAST}, slim ${SLIM}; ${before} -> ${after} sections) -> ${args[1]}`);
