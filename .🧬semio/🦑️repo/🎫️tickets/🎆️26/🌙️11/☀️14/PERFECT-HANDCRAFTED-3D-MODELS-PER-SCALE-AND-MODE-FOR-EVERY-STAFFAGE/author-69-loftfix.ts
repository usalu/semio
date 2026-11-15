/** 🧵 Repairs folded lofts of a human source: bun author-69-loftfix.ts <slug> [--write]. For every loft whose consecutive sections fold (loftFault), the tilt between the two sections is removed step by step (their rotations are blended towards each other, an end cap takes the rotation of its neighbour first) until the fold is gone; the section centres, radii and exponents are never changed. */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { loftFault } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
const TICKET = dirname(fileURLToPath(import.meta.url));
const slug = process.argv[2]!;
const write = process.argv.includes("--write");
const path = join(TICKET, "volumes", "humans", `${slug}.json`);
const src = JSON.parse(readFileSync(path, "utf8"));
type Q = [number, number, number, number];
const norm = (q: Q): Q => { const n = Math.hypot(...q); return q.map((x) => x / n) as Q; };
const blend = (a: Q, b: Q, t: number): Q => { const s = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3] < 0 ? -1 : 1; return norm(a.map((x, k) => x * (1 - t) + s * b[k]! * t) as Q); };
const round = (q: Q): Q => q.map((x) => Math.round(x * 1e4) / 1e4) as Q;
let fixed = 0, failed = 0;
for (const scale of Object.keys(src.volumes)) {
  const v = src.volumes[scale];
  if (v.status !== "authored") continue;
  for (const list of ["body", "thermal", "structure"]) for (const s of v[list] ?? []) {
    if (s.kind !== "loft" || s.of !== undefined) continue;
    let fault = loftFault(s.sections);
    if (!fault) continue;
    const original = JSON.stringify(s.sections);
    const base = JSON.parse(original);
    let guard = 0;
    while (fault && guard++ < 60) {
      const [i, j] = fault;
      const end = i === 0 ? 0 : j === s.sections.length - 1 ? j : -1;
      if (end >= 0) {
        const cap = s.sections[end], neighbour = s.sections[end === 0 ? 1 : end - 1];
        const keep = cap.rotation;
        let done = false;
        for (let t = 0.1; t <= 1.0001 && !done; t += 0.1) {
          cap.rotation = round(blend(base[end].rotation, neighbour.rotation, t));
          if (!loftFault(s.sections)?.some((k) => k === end)) done = true;
        }
        if (!done) cap.rotation = keep;
        fault = loftFault(s.sections);
        if (fault && fault[0] === i && fault[1] === j) break;
        continue;
      }
      const a = s.sections[i], b = s.sections[j];
      const [ra, rb] = [base[i].rotation, base[j].rotation];
      let done = false;
      for (let t = 0.05; t <= 0.5001 && !done; t += 0.05) {
        a.rotation = round(blend(ra, rb, t)); b.rotation = round(blend(rb, ra, t));
        const f2 = loftFault(s.sections);
        if (!f2 || f2[0] !== i) done = true;
      }
      fault = loftFault(s.sections);
      if (!done) break;
    }
    if (fault) {
      s.sections = JSON.parse(original);
      fault = loftFault(s.sections);
      let g2 = 0;
      while (fault && g2++ < 40) {
        const [i, j] = fault;
        const a = s.sections[i], b = s.sections[j];
        const end = i === 0 ? 0 : j === s.sections.length - 1 ? j : -1;
        if (end >= 0) { const other = end === 0 ? b : a; (end === 0 ? a : b).rotation = round(other.rotation as Q); }
        else { const m = blend(a.rotation, b.rotation, 0.5); a.rotation = round(blend(a.rotation, m, 0.6)); b.rotation = round(blend(b.rotation, m, 0.6)); }
        fault = loftFault(s.sections);
      }
      if (!fault) console.log(`(aggressive) 1:${scale} ${list} ${s.id}`);
    }
    if (fault) { failed++; s.sections = JSON.parse(original); console.log(`FAILED 1:${scale} ${list} ${s.id} sections ${fault[0]}/${fault[1]}`); }
    else { fixed++; console.log(`fixed 1:${scale} ${list} ${s.id}`); }
  }
}
console.log(`${fixed} fixed, ${failed} failed`);
if (write) writeFileSync(path, JSON.stringify(src));
