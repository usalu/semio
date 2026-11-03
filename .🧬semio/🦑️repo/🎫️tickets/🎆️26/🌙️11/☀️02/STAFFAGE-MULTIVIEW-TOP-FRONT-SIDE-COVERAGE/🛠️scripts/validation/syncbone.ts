import { spawn } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join, resolve } from "node:path";
const L = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { loadCatalog, scaleSelector, authoredOf, extentOf } = await import(`${L}/🧪️tests/🔭️multiview-coverage/🧰️.ts`);
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const args = process.argv.slice(2);
const flag = (n: string) => { const i = args.indexOf(`--${n}`); return i < 0 ? undefined : args[i + 1]; };
const out = resolve(flag("out") ?? ".");
const shard = flag("shard");
type Row = { asset: string; category: string; bone: string; pair: string; deltaMm: number; subject: string; scale: number; tolMm: number };
if (shard !== undefined) {
  const [i, n] = shard.split("/").map(Number) as [number, number];
  const rows: Row[] = [];
  for (const asset of (loadCatalog() as any[]).filter((_, j) => j % n === i)) {
    const maxDim = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
    const tol = Math.max(0.015, 0.02 * maxDim) * 1000;
    const key = ["500", "200", "100", "50"].filter((s) => authoredOf(asset, s)).find((s) => Number(s) === asset.presentation.defaultScale) ?? ["100", "200", "50", "500"].find((s) => authoredOf(asset, s));
    if (!key) continue;
    const worst = new Map<string, Row>();
    for (const pose of asset.poses) {
      let st: any;
      try { st = evaluate(asset, { kind: "pose", pose: pose.id }, scaleSelector(Number(key))); } catch { continue; }
      const byBone = (v: string) => { const m = new Map<string, any>(); for (const e of st.shapes[v]) { if (e.shape.role === "ground") continue; const g = m.get(e.shape.bone) ?? []; g.push(e); m.set(e.shape.bone, g); } return new Map([...m].map(([b, g]) => [b, extentOf(g)])); };
      const E = byBone("elevation"), F = byBone("front"), P = byBone("plan");
      const rec = (bone: string, pair: string, a: number, b: number, c: number, d: number) => {
        const delta = Math.max(Math.abs(a - c), Math.abs(b - d)) * 1000;
        const k = `${bone}|${pair}`; const prev = worst.get(k);
        if (!prev || delta > prev.deltaMm) worst.set(k, { asset: asset.id, category: asset.category, bone, pair, deltaMm: delta, subject: `pose ${pose.id}`, scale: Number(key), tolMm: tol });
      };
      for (const [bone, e] of E) {
        const f = F.get(bone), p = P.get(bone);
        if (e && f) rec(bone, "elevation/front height", e.minY, e.maxY, f.minY, f.maxY);
        if (e && p) rec(bone, "elevation/plan forward", e.minX, e.maxX, p.minX, p.maxX);
      }
      for (const [bone, f] of F) { const p = P.get(bone); if (f && p) rec(bone, "front/plan lateral", f.minX, f.maxX, p.minY, p.maxY); }
    }
    rows.push(...worst.values());
  }
  writeFileSync(join(out, `bone-${i}.json`), JSON.stringify(rows));
  process.exit(0);
}
mkdirSync(out, { recursive: true });
const jobs = 8;
await Promise.all(Array.from({ length: jobs }, (_, i) => new Promise<void>((done) => spawn(process.execPath, [fileURLToPath(import.meta.url), "--out", out, "--shard", `${i}/${jobs}`], { stdio: ["ignore", "inherit", "inherit"] }).on("exit", () => done()))));
const rows = Array.from({ length: jobs }, (_, i) => JSON.parse(readFileSync(join(out, `bone-${i}.json`), "utf8")) as Row[]).flat();
writeFileSync(join(out, "bone.json"), JSON.stringify(rows));
console.error("[done]", rows.length);
