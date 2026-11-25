/** 🪞️ The derived-view measure of plants of batch ML-3 without any allowlist: `bun derived.ts <id|file.json>… [--worst]` prints per authored band and per symbol the findings of the suite (`derivedFindings` with empty allowlists) and the normalised Hausdorff distance of front against elevation, plan against elevation and plan against front under the best candidate transform (flagged below 0.03); `--worst` adds, for front against elevation, the point that carries the distance in metres of its own view, the number of raw points and the stride of the sample. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { bestTransform, pointSet, TRANSFORMS, MAX_POINTS } = await import(join(T, "🪞️derived.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const worst = args.includes("--worst");
const NONE = { derived: [], bandCopy: [] };
type P = { x: number; y: number };
const frame = (shapes: readonly any[]): { side: number; cx: number; cy: number; raw: number } => {
  const raw: P[] = [];
  for (const e of shapes) {
    if (e.shape.role === "ground") continue;
    for (const ring of e.rings) ring.forEach((p: P, i: number) => { raw.push(p); if ((ring[i + 1] ?? (e.closed ? ring[0] : undefined)) !== undefined) raw.push(p); });
  }
  const xs = raw.map((p) => p.x), ys = raw.map((p) => p.y);
  const [x0, x1, y0, y1] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  return { side: Math.max(x1 - x0, y1 - y0), cx: (x0 + x1) / 2, cy: (y0 + y1) / 2, raw: raw.length };
};
const carrier = (a: readonly P[], b: readonly P[], m: readonly number[]): { from: "a" | "b"; p: P; d: number } => {
  const moved = b.map((p) => ({ x: m[0]! * p.x + m[1]! * p.y, y: m[2]! * p.x + m[3]! * p.y }));
  let out = { from: "a" as "a" | "b", p: a[0]!, d: -1 };
  const pass = (from: "a" | "b", ps: readonly P[], qs: readonly P[], back: readonly P[]): void => ps.forEach((p, i) => { let best = Infinity; for (const q of qs) best = Math.min(best, Math.hypot(p.x - q.x, p.y - q.y)); if (best > out.d) out = { from, p: back[i]!, d: best }; });
  pass("a", a, moved, a);
  pass("b", moved, a, b);
  return out;
};
for (const arg of args.filter((a) => !a.startsWith("--"))) {
  const asset = JSON.parse(readFileSync(fileOf(arg), "utf8")).asset;
  const pose = { kind: "pose", pose: asset.presentation.defaultPose } as const;
  const report = (label: string, shapes: any, findings: readonly any[]): void => {
    const sets: Record<string, P[] | undefined> = { elevation: pointSet(shapes.elevation), front: pointSet(shapes.front), plan: pointSet(shapes.plan) };
    const pair = (a: string, b: string): string => { if (!sets[a] || !sets[b]) return `${a}/${b} -`; const best = bestTransform(sets[a], sets[b]); return `${a}/${b} ${best.distance.toFixed(4)} ${best.name}${Math.round(best.distance * 1e6) < 30000 ? " FLAG" : ""}`; };
    console.log(`${asset.id} ${label} | ${pair("front", "elevation")} | ${pair("plan", "elevation")} | ${pair("plan", "front")} | findings without allowlist: ${findings.length === 0 ? "none" : findings.map((f) => `${f.check} ${f.view} against ${f.against}`).join("; ")}`);
    if (worst && sets.front && sets.elevation) {
      const best = bestTransform(sets.front, sets.elevation);
      const c = carrier(sets.front, sets.elevation, TRANSFORMS.find((t: any) => t.name === best.name).m);
      const f = frame(c.from === "a" ? shapes.front : shapes.elevation);
      const [ff, fe] = [frame(shapes.front), frame(shapes.elevation)];
      console.log(`    worst point in the ${c.from === "a" ? "front" : "elevation"} at (${(c.p.x * f.side + f.cx).toFixed(2)}, ${(c.p.y * f.side + f.cy).toFixed(2)}) m, ${(c.d * f.side).toFixed(3)} m from the other view; front ${ff.raw} raw points (stride ${Math.max(1, Math.ceil(ff.raw / MAX_POINTS))}, side ${ff.side.toFixed(2)} m), elevation ${fe.raw} (stride ${Math.max(1, Math.ceil(fe.raw / MAX_POINTS))}, side ${fe.side.toFixed(2)} m)`);
    }
  };
  for (const key of Object.keys(asset.representations).filter((k) => engine.authoredOf(asset, k) !== undefined).sort((a, b) => Number(a) - Number(b)))
    report(`1:${key}`, evaluate(asset, pose, engine.scaleSelector(Number(key))).shapes, engine.derivedFindings(asset, key, Number(key), undefined, NONE));
  for (const symbol of asset.symbols ?? [])
    report(`symbol ${symbol.id}`, evaluate(asset, pose, { kind: "symbol", symbol: symbol.id }).shapes, engine.derivedFindings(asset, Object.keys(asset.representations)[0]!, 100, symbol.id, NONE));
}
