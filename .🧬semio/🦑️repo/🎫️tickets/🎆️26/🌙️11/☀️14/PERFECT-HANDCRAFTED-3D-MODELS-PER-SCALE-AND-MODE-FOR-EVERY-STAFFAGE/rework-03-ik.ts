/** 🎯 Solves pose values of a candidate document so that bone-local points reach world targets (damped Gauss-Newton over the product's own `solveChain` with the root at the origin, which is what `evaluate` uses for a pose with `ground: "authored"`).
 *
 * Usage: `bun rework-03-ik.ts <job.json>` with `{ file, pose, vars: [[bone, field], …], targets: [{ bone, p: [u, v, w], to: [x, y, z], weight?: [wx, wy, wz] }], set?: { bone: { field: value } } }`.
 * `set` is written into the pose first; the solved values are printed as one JSON object `{ bone: { field: value } }` followed by the residual of every target in millimetres on stderr. Angles in degrees, `dx`/`dy` in metres.
 * @see ./rework-03-fk.ts */
import { readFileSync } from "node:fs";
import { solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const job = JSON.parse(readFileSync(process.argv[2], "utf8"));
const asset = JSON.parse(readFileSync(job.file, "utf8")).asset;
const pose = structuredClone(asset.poses.find((p: { id: string }) => p.id === job.pose));
pose.id = "ik-work";
asset.poses.push(pose);
for (const [bone, fields] of Object.entries(job.set ?? {})) pose.bones[bone] = { ...(pose.bones[bone] ?? {}), ...(fields as object) };
const vars: [string, string][] = job.vars;
const get = () => vars.map(([b, f]) => pose.bones[b]?.[f] ?? 0);
const put = (x: number[]) => vars.forEach(([b, f], i) => ((pose.bones[b] ??= {})[f] = x[i]));
function residual(x: number[]): number[] {
  put(x);
  const chain = solveChain(asset.rig, pose.bones, { x: 0, y: 0, angleDeg: 0 }) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
  const out: number[] = [];
  for (const t of job.targets) {
    const fr = chain[t.bone].frame;
    const wt = t.weight ?? [1, 1, 1];
    for (let i = 0; i < 3; i++) out.push(wt[i] * (fr.o[i] + fr.u[i] * t.p[0] + fr.v[i] * t.p[1] + fr.w[i] * (t.p[2] ?? 0) - t.to[i]));
  }
  return out;
}
function solve(a: number[][], b: number[]): number[] {
  const n = b.length;
  const m = a.map((row, i) => [...row, b[i]]);
  for (let c = 0; c < n; c++) {
    let p = c;
    for (let r = c + 1; r < n; r++) if (Math.abs(m[r][c]) > Math.abs(m[p][c])) p = r;
    [m[c], m[p]] = [m[p], m[c]];
    for (let r = 0; r < n; r++) {
      if (r === c || m[c][c] === 0) continue;
      const k = m[r][c] / m[c][c];
      for (let j = c; j <= n; j++) m[r][j] -= k * m[c][j];
    }
  }
  return m.map((row, i) => (row[i] === 0 ? 0 : row[n] / row[i]));
}
let x = get();
const step = vars.map(([, f]) => (f === "dx" || f === "dy" ? 1e-4 : 0.05));
const norm = (r: number[]) => r.reduce((s, v) => s + v * v, 0);
let lambda = 1e-3;
for (let iteration = 0; iteration < 80; iteration++) {
  const r0 = residual(x);
  const jac = vars.map((_, k) => {
    const xk = [...x];
    xk[k] += step[k];
    return residual(xk).map((v, i) => (v - r0[i]) / step[k]);
  });
  const n = vars.length;
  const ata = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => jac[i].reduce((s, v, k) => s + v * jac[j][k], 0) + (i === j ? lambda * (1e-8 + jac[i].reduce((s, v) => s + v * v, 0)) : 0)));
  const atb = jac.map((col) => -col.reduce((s, v, k) => s + v * r0[k], 0));
  const next = x.map((v, i) => v + solve(ata, atb)[i]);
  if (norm(residual(next)) < norm(r0)) {
    x = next;
    lambda = Math.max(lambda / 3, 1e-7);
  } else lambda *= 4;
  if (norm(residual(x)) < 1e-12) break;
}
const r = residual(x);
const result: Record<string, Record<string, number>> = {};
vars.forEach(([b, f], i) => ((result[b] ??= {})[f] = Math.round(x[i] * 1000) / 1000));
console.log(JSON.stringify(result));
job.targets.forEach((t: { bone: string }, k: number) => process.stderr.write(`[DEBUG] ${t.bone} residual mm ${r.slice(k * 3, k * 3 + 3).map((v) => (v * 1000).toFixed(2)).join(" ")}\n`));
