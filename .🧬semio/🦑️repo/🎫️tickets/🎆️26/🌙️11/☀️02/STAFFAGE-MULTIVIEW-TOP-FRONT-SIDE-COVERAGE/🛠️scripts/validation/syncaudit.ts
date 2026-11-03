import { spawn } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join, resolve } from "node:path";
const L = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { loadCatalog, subjects, scaleSelector, extentOf, authoredOf } = await import(`${L}/🧪️tests/🔭️multiview-coverage/🧰️.ts`);
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const args = process.argv.slice(2);
const flag = (n: string) => { const i = args.indexOf(`--${n}`); return i < 0 ? undefined : args[i + 1]; };
const out = resolve(flag("out") ?? ".");
const shard = flag("shard");
type Row = { asset: string; category: string; check: string; pair: string; deltaMm: number; subject: string; scale: number; withGroundMm: number; maxDimM: number };
if (shard !== undefined) {
  const [i, n] = shard.split("/").map(Number) as [number, number];
  const rows: Row[] = [];
  for (const [k, asset] of (loadCatalog() as any[]).filter((_, j) => j % n === i).entries()) {
    console.error(`[sync ${shard}] ${asset.id}`);
    const maxDim = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
    const worst = new Map<string, Row>();
    const scalesAuthored = ["500", "200", "100", "50"].filter((s) => authoredOf(asset, s));
    const dflt = scalesAuthored.includes(String(asset.presentation.defaultScale)) ? String(asset.presentation.defaultScale) : scalesAuthored[0];
    for (const key of scalesAuthored) {
      const subs = (subjects(asset, Number(key)) as any[]).filter((s) => s.query.kind === "pose" || (key === dflt && s.query.kind === "clip"));
      for (const s of subs) {
        let st: any;
        try { st = evaluate(asset, s.query, scaleSelector(Number(key))); } catch { continue; }
        const ex = (v: string, g: boolean) => { const shapes = st.shapes[v].filter((x: any) => g || x.shape.role !== "ground"); return extentOf(g ? st.shapes[v].map((x: any) => ({ ...x, shape: { ...x.shape, role: "body" } })) : shapes); };
        for (const g of [false, true]) {
          const e = ex("elevation", g), f = ex("front", g), p = ex("plan", g);
          if (!e || !f || !p) continue;
          const pairs: [string, string, number][] = [
            ["sync-forward", "elevation/plan", Math.max(Math.abs(e.minX - p.minX), Math.abs(e.maxX - p.maxX))],
            ["sync-height", "elevation/front", Math.max(Math.abs(e.minY - f.minY), Math.abs(e.maxY - f.maxY))],
            ["sync-lateral", "front/plan", Math.max(Math.abs(f.minX - p.minY), Math.abs(f.maxX - p.maxY))],
          ];
          for (const [check, pair, d] of pairs) {
            const id = `${check}|${key}`;
            const prev = worst.get(id);
            if (!g) { if (!prev || d * 1000 > prev.deltaMm) worst.set(id, { asset: asset.id, category: asset.category, check, pair, deltaMm: d * 1000, subject: s.id, scale: Number(key), withGroundMm: Number.NaN, maxDimM: maxDim }); }
            else if (prev && s.id === prev.subject) prev.withGroundMm = d * 1000;
          }
        }
      }
    }
    rows.push(...worst.values());
  }
  writeFileSync(join(out, `sync-${i}.json`), JSON.stringify(rows));
  process.exit(0);
}
mkdirSync(out, { recursive: true });
const jobs = 14;
await Promise.all(Array.from({ length: jobs }, (_, i) => new Promise<void>((done) => spawn(process.execPath, [fileURLToPath(import.meta.url), "--out", out, "--shard", `${i}/${jobs}`], { stdio: ["ignore", "inherit", "inherit"] }).on("exit", () => done()))));
const rows = Array.from({ length: jobs }, (_, i) => JSON.parse(readFileSync(join(out, `sync-${i}.json`), "utf8")) as Row[]).flat();
writeFileSync(join(out, "sync.json"), JSON.stringify(rows));
console.error("[done]", rows.length);
