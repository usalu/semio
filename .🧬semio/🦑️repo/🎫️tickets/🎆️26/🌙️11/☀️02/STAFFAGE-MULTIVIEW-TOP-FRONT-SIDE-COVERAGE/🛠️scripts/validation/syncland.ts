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
type Row = { asset: string; category: string; kind: string; id: string; pair: string; deltaMm: number; subject: string; scale: number; tolMm: number };
const { boneSync, anchorLandmarks } = await import(`${L}/🧪️tests/🔭️multiview-coverage/⚓️landmarks.ts`);
if (shard !== undefined) {
  const [i, n] = shard.split("/").map(Number) as [number, number];
  const rows: Row[] = [];
  for (const asset of (loadCatalog() as any[]).filter((_, j) => j % n === i)) {
    const maxDim = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
    const tol = Math.max(0.015, 0.02 * maxDim);
    const key = ["100", "200", "50", "500"].find((s) => authoredOf(asset, s) && Number(s) === asset.presentation.defaultScale) ?? ["100", "200", "50", "500"].find((s) => authoredOf(asset, s));
    if (!key) continue;
    const worst = new Map<string, Row>();
    const keep = (kind: string, id: string, pair: string, deltaMm: number, subject: string) => { const k = `${kind}|${id}|${pair}`; const prev = worst.get(k); if (!prev || deltaMm > prev.deltaMm) worst.set(k, { asset: asset.id, category: asset.category, kind, id, pair, deltaMm, subject, scale: Number(key), tolMm: tol * 1000 }); };
    for (const pose of asset.poses) {
      let st: any;
      try { st = evaluate(asset, { kind: "pose", pose: pose.id }, scaleSelector(Number(key))); } catch { continue; }
      for (const f of boneSync(st, tol)) keep(f.tier, f.bone, f.pair, f.deltaMm, `pose ${pose.id}`);
      for (const f of anchorLandmarks(st, tol)) keep("anchor", f.anchor, f.view, f.deltaMm, `pose ${pose.id}`);
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
