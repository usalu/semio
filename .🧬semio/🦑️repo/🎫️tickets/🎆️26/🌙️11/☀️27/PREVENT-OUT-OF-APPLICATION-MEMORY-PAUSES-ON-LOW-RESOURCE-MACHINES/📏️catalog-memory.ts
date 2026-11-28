/** 📏️ Memory per asset of the model-catalog guard, in one process: `bun 📏️catalog-memory.ts <id,id,...> [--no-gc]` prints rss, heap, external and array buffers after each asset. */
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { VIEWS } = await import(`${S}/🔨️modules/🔗️contracts/🟦️.ts`);
const { exportModels, plannedNames } = await import(`${S}/🔨️modules/📤️export/📦️batch/🟦️.ts`);
const { assetPathsFromFiles, readStreamedAsset } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { auditFiles, auditGeometry } = await import(`${S}/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts`);
const ids = process.argv[2]!.split(",");
const gc = !process.argv.includes("--no-gc");
const FBX = ["light-solid", "thermal", "forces"];
const mb = (n: number) => Math.round(n / 1048576);
import { spawnSync } from "node:child_process";
const fp = (): number => {
  const m = /Footprint: ([\d.]+) (KB|MB|GB)/.exec(spawnSync("footprint", ["-p", String(process.pid)], { encoding: "utf8" }).stdout);
  return m === null ? NaN : Math.round(Number(m[1]) * ({ KB: 1 / 1024, MB: 1, GB: 1024 } as const)[m[2] as "KB"]);
};
const row = (label: string) => {
  if (gc) (globalThis as any).Bun?.gc(true);
  const m = process.memoryUsage();
  console.log(`${label.padEnd(34)} rss ${mb(m.rss)} fp ${fp()} heap ${mb(m.heapUsed)} ext ${mb(m.external)} ab ${mb(m.arrayBuffers)} maxRSS ${mb(process.resourceUsage().maxRSS * 1024)}`);
};
let peakFp = 0;
const entries = (await assetPathsFromFiles()).filter((e: any) => ids.includes(e.folder));
row("start");
for (const entry of entries) {
  const { asset } = await readStreamedAsset(entry);
  row(`${asset.id} read`);
  const geometry = await auditGeometry(asset);
  row(`${asset.id} geometry`);
  const directory = mkdtempSync(join(tmpdir(), "staffage-models-"));
  try {
    let lastPct = -1;
    await exportModels({ out: directory, assets: [asset.id], views: FBX }, { progress: (done: number, total: number, label: any) => { const pct = Math.floor((done / total) * 20); if (pct !== lastPct) { lastPct = pct; const m = process.memoryUsage(); if (!process.argv.includes("--trace")) return; console.log(`  ${String(pct * 5).padStart(3)} % rss ${mb(m.rss)} fp ${fp()} heap ${mb(m.heapUsed)} ext ${mb(m.external)} ${label?.en ?? ""}`); } } });
    row(`${asset.id} export`);
    const manifest = JSON.parse(readFileSync(join(directory, "manifest.json"), "utf8"));
    await auditFiles(asset, manifest.files, (f: string) => { peakFp = Math.max(peakFp, fp()); return new Uint8Array(readFileSync(join(directory, f))); }, plannedNames(asset, { views: FBX }));
    row(`${asset.id} files (peak fp while reading ${peakFp})`);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
  row(`${asset.id} done`);
}
