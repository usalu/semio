import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const S = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const { assetPathsFromFiles, readStreamedAsset } = await import(`${S}/🗂️catalog/📥️files/🟦️.ts`);
const dir = process.env.VOLUMES_DIR ?? "🧊️volumes";
const { measureVolumes } = await import(`${S}/🗂️catalog/${dir}/🟦️.ts`);
const { worldSolids } = await import(`${S}/🗂️catalog/${dir}/🌓️silhouette/🟦️.ts`);
const { exposedArea } = await import(`${S}/🗂️catalog/${dir}/🧮️measures/🟦️.ts`);
const { evaluate } = await import(`${S}/🧮️evaluation/🟦️.ts`);
const [slugs, scale] = [process.argv[2]!.split(","), Number(process.argv[3] ?? 50)];
const out: Record<string, unknown> = {};
const entries = await assetPathsFromFiles();
for (const slug of slugs) {
  const entry = entries.find((e: { folder: string }) => e.folder === slug)!;
  const { asset } = await readStreamedAsset(entry);
  const volume = (asset as any).volumes[String(scale)];
  const state = evaluate(asset as never, { kind: "pose", pose: (asset as any).presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const measured = measureVolumes(asset as never, { scale, rules: ["volumes-thermal-coverage"] } as never) as { name: string; value: number }[];
  const worlds = new Map<string, any>((worldSolids(asset as never, state.bones as never, scale, "thermal") as any[]).map((w) => [w.id, w]));
  const emitters = new Set((asset as any).thermal.emitters.map((e: { id: string }) => e.id));
  const regions: Record<string, unknown> = {};
  for (const region of (asset as any).thermal.regions) {
    const own = volume.thermal.filter((s: any) => s.region === region.id);
    if (own.length === 0) continue;
    const outer = region.layers.length > 1 && scale === 50 ? region.layers[region.layers.length - 1].id : undefined;
    const shell = outer === undefined ? own : own.filter((s: any) => s.layer === outer);
    const picked = (shell.length > 0 ? shell : own).map((s: any) => worlds.get(s.id)).filter((w: any) => w !== undefined && w.envelope !== true && w.emitter !== true);
    regions[region.id] = {
      areaM2: region.areaM2,
      hidden: region.hidden !== undefined,
      exchange: region.areaBasis === "exchange",
      model: region.model,
      measured: measured.find((m) => m.name === `info:outerAreaM2:${region.id}`)?.value ?? null,
      deviation: measured.find((m) => m.name === `areaDeviation:${region.id}`)?.value ?? null,
      solids: picked.map((w: any) => ({ id: w.id, moduleArea: exposedArea([w], picked, [], (o: any) => o === w, 0.02 * 0.0075 * scale / 50), pairs: (process.env.DEBUG_SOLIDS ?? "").split(",").includes(w.id) ? Object.fromEntries(picked.filter((o: any) => o !== w).map((o: any) => [o.id, exposedArea([w], [o], [], () => false, 0.02 * 0.0075 * scale / 50)])) : undefined, total: exposedArea([w], [], [], () => false, 1e-4), positions: [...w.positions], indices: [...w.indices], sections: (w.sectionFaces ?? []).map((f: string) => { const a = w.frame[f.slice(1)]; const s = f[0] === "-" ? -1 : 1; return [s * a.x, s * a.y, s * a.z]; }) })),
    };
  }
  out[slug] = regions;
}
const here = dirname(fileURLToPath(import.meta.url));
mkdirSync(join(here, "🗑️generated", "validator3"), { recursive: true });
writeFileSync(process.argv[4] ?? join(here, "🗑️generated", "validator3", "c29-dump.json"), JSON.stringify(out));
console.log("dumped", Object.keys(out).join(","));
