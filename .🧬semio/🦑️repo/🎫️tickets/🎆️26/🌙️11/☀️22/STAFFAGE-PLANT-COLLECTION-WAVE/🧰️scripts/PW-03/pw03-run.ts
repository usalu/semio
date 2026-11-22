/** 🚚️ PW-03 run: assembles one species from its base document and design, writes the catalog asset and its manifest row, and with flags validates (`--validate`), measures the volume rules (`--volumes`), runs the catalog check (`--check`) and renders previews (`--render`).
 * Usage: `repo tools exec bun -- pw03-run.ts <species id> [--validate] [--volumes] [--check] [--render] [--no-write]`
 * @see ./pw03-engine.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { STAFFAGE, type Json } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";
import { SCALES, contextOf, drawScale, mantle, symbols, volumeOf, type Species } from "./pw03-engine.ts";

const id = process.argv[2]!;
const flags = new Set(process.argv.slice(3));
const HERE = import.meta.dir;
const TICKET = join(HERE, "../..");
const GEN = join(TICKET, "🗑️generated/PW-03", id);
mkdirSync(GEN, { recursive: true });
const sp: Species = (await import(`./pw03-${id}.ts`)).species;
const base: Json = JSON.parse(readFileSync(join(GEN, "base.json"), "utf8"));
const skeleton: Json = JSON.parse(readFileSync(join(GEN, "skeleton.json"), "utf8"));
const asset: Json = base.asset;
const ctx = contextOf(asset, skeleton);
const { evaluate } = await import(join(STAFFAGE, "🔨️modules/🧮️evaluation/🟦️.ts"));

const { worldSolids, extentOf } = await import(join(STAFFAGE, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const models = SCALES.map((s) => sp.model(s, ctx));
const scratch: Record<string, { bone: string; local: [number, number, number] }> = {};
const bodyExtent = (m: (typeof models)[number]): { min: number[]; max: number[] } => {
  asset.volumes = { ...(asset.volumes ?? {}), [String(m.scale)]: volumeOf(sp, ctx, m, [], scratch) };
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
  return extentOf(worldSolids(asset, state.bones, m.scale).filter((w: Json) => w.texture !== true));
};
const goal = bodyExtent(models[0]!);
for (const m of models.slice(1)) {
  for (let pass = 0; pass < 4; pass++) {
    const e = bodyExtent(m);
    const mid = [0, 1, 2].map((k) => (e.min[k]! + e.max[k]!) / 2);
    const base = 2.0;
    const k = { xp: (goal.max[0]! - mid[0]!) / (e.max[0]! - mid[0]!), xm: (mid[0]! - goal.min[0]!) / (mid[0]! - e.min[0]!), zp: (goal.max[2]! - mid[2]!) / (e.max[2]! - mid[2]!), zm: (mid[2]! - goal.min[2]!) / (mid[2]! - e.min[2]!), y: (goal.max[1]! - base) / (e.max[1]! - base) };
    console.log(`[DEBUG] 1:${m.scale} pass ${pass} extent dx ${(e.min[0]! - goal.min[0]!).toFixed(3)} ${(e.max[0]! - goal.max[0]!).toFixed(3)} dy ${(e.max[1]! - goal.max[1]!).toFixed(3)} dz ${(e.min[2]! - goal.min[2]!).toFixed(3)} ${(e.max[2]! - goal.max[2]!).toFixed(3)}`);
    const move = (p: number[]): [number, number, number] => [mid[0]! + (p[0]! - mid[0]!) * (p[0]! > mid[0]! ? k.xp : k.xm), p[1]! > base ? base + (p[1]! - base) * k.y : p[1]!, mid[2]! + (p[2]! - mid[2]!) * (p[2]! > mid[2]! ? k.zp : k.zm)];
    for (const l of [...m.lobes, ...m.extras, ...m.hiddenExtras]) {
      if (l.kind === "goblet") l.base = move(l.base);
      else l.c = move(l.c);
    }
  }
}
const anchorsOut: Record<string, { bone: string; local: [number, number, number] }> = {};
const counts: Record<number, Json> = {};
for (const m of models) counts[m.scale] = drawScale(sp, ctx, m, asset.representations[String(m.scale)]);
asset.volumes = {};
for (const m of models) {
  const t = sp.texts(m.scale, counts[m.scale]);
  asset.volumes[String(m.scale)] = volumeOf(sp, ctx, m, t.volume, anchorsOut);
  asset.representations[String(m.scale)].decisions = t.drawn;
}
for (const a of asset.anchors) {
  const moved = anchorsOut[a.id];
  if (!moved) continue;
  if (moved.bone !== a.bone) throw new Error(`anchor ${a.id} would change its bone`);
  [a.x, a.y, a.z] = moved.local.map((v) => Math.round(v * 1e4) / 1e4);
}
const m50 = models[0]!;
const bark: Record<string, number> = {};
for (const t of [...m50.wood, ...m50.twigs]) {
  const region = t.group === "twig" && sp.twigRegion ? sp.twigRegion : skeleton.skeleton[t.bone]?.region ?? "limb-bark";
  bark[region] = (bark[region] ?? 0) + mantle(t);
}
const overrides: Record<string, number> = sp.areaOverrides ?? {};
for (const r of asset.thermal.regions) if (r.areaBasis !== "exchange") r.areaM2 = Math.round((overrides[r.id] ?? bark[r.id] ?? r.areaM2) * 100) / 100;

const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
const final = Object.fromEntries(models.map((m) => [m.scale, extentOf(worldSolids(asset, state.bones, m.scale).filter((w: Json) => w.texture !== true))]));
for (const m of models) console.log(`[DEBUG] 1:${m.scale} body ${[0, 1, 2].map((k) => (final[m.scale].max[k] - final[m.scale].min[k]).toFixed(3)).join(" / ")}`);
const lateral = state.shapes.plan.flatMap((s: Json) => s.rings.flat().map((p: Json) => p.y));
const zs = [...m50.lobes, ...m50.extras.filter((e) => e.kind === "lobe")].flatMap((l: Json) => { const h = Math.hypot(...l.axes.map((a: number[], i: number) => a[2]! * l.radii[i]!)); return [l.c[2] - h, l.c[2] + h]; });
const planDepth = Math.max(...lateral) - Math.min(...lateral), bodyDepth = Math.max(...zs) - Math.min(...zs);
console.log(`[DEBUG] depth plan ${planDepth.toFixed(3)} body ${bodyDepth.toFixed(3)}`);
const size = (k: number): number => Math.round((final[50].max[k] - final[50].min[k]) * 1000) / 1000;
console.log(`[DEBUG] drawn 1:50 ${(state.bounds.maxX - state.bounds.minX).toFixed(3)} x ${(state.bounds.maxY - state.bounds.minY).toFixed(3)}`);
asset.dimensions = { heightM: size(1), widthM: size(0), depthM: size(2), massKg: asset.dimensions.massKg };
asset.symbols = symbols(sp, models[1]!, evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 100 } }));
const ordered: Json = {};
for (const k of ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]) ordered[k] = asset[k];
const doc = { $schema: base.$schema, version: base.version, asset: ordered };
const report: string[] = [`[DEBUG] ${id} dims ${JSON.stringify(asset.dimensions)} bark ${JSON.stringify(bark)}`];
for (const m of models) report.push(`[DEBUG] 1:${m.scale} body ${asset.volumes[m.scale].body.length} thermal ${asset.volumes[m.scale].thermal.length} structure ${asset.volumes[m.scale].structure.length} shapes ${["elevation", "front", "plan"].map((v) => asset.representations[m.scale].geometry[v].length).join("/")} seen ${JSON.stringify(counts[m.scale].seen)}`);
for (const m of models) {
  const ext = (axis: number, sign: number): number => Math.max(...[...m.lobes, ...m.extras.filter((e) => e.kind === "lobe")].map((l: Json) => sign * l.c[axis] + Math.hypot(...l.axes.map((a: number[], i: number) => a[axis]! * l.radii[i]!))));
  report.push(`[DEBUG] 1:${m.scale} crown x ${(-ext(0, -1)).toFixed(3)}..${ext(0, 1).toFixed(3)} y ..${ext(1, 1).toFixed(3)} z ${(-ext(2, -1)).toFixed(3)}..${ext(2, 1).toFixed(3)}`);
}
for (const b of asset.rig.bones.filter((x: Json) => x.id.startsWith("fol-"))) {
  const f = ctx.F[b.id]!;
  const p = [f.o[0] + f.u[0] * b.mass.comX + f.v[0] * b.mass.comY, f.o[1] + f.u[1] * b.mass.comX + f.v[1] * b.mass.comY, f.o[2]];
  const depth = Math.max(...m50.lobes.map((l) => 1 - Math.hypot(...l.axes.map((a, i) => (a[0] * (p[0]! - l.c[0]) + a[1] * (p[1]! - l.c[1]) + a[2] * (p[2]! - l.c[2])) / l.radii[i]!))));
  if (depth < 0.1) report.push(`[DEBUG] mass of ${b.id} at ${p.map((v) => v.toFixed(2)).join(" ")} depth ${depth.toFixed(2)}`);
}
console.log(report.join("\n"));
const target = join(STAFFAGE, "🗂️catalog/🌳️vegetation", id, "🔣️.json");
const text = JSON.stringify(doc, null, 2) + "\n";
writeFileSync(join(GEN, "candidate.json"), text);
if (!flags.has("--no-write")) {
  mkdirSync(join(STAFFAGE, "🗂️catalog/🌳️vegetation", id), { recursive: true });
  writeFileSync(target, text);
  const row = {
    id, category: "vegetation", title: asset.title, heightM: asset.dimensions.heightM, widthM: asset.dimensions.widthM, powerW: 0,
    poses: asset.poses.filter((p: Json) => !p.key).map((p: Json) => p.id), clips: asset.clips.map((c: Json) => c.id),
    representations: Object.fromEntries(["50", "100", "200", "500"].map((k) => [k, asset.representations[k].status])),
    symbols: asset.symbols.map((s: Json) => s.id), analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction: asset.summary, completion: "authored",
  };
  mkdirSync(join(TICKET, "📥️manifest-rows"), { recursive: true });
  writeFileSync(join(TICKET, "📥️manifest-rows", `${id}.json`), JSON.stringify(row, null, 1) + "\n");
}

if (flags.has("--validate") || flags.has("--volumes")) {
  const { validateAsset } = await import(join(STAFFAGE, "🔨️modules/🗂️catalog/🟦️.ts"));
  const { loadEnvironments } = await import(join(STAFFAGE, "🔨️modules/🗂️catalog/📥️files/🟦️.ts"));
  const envs = new Set(((await loadEnvironments())?.presets ?? []).map((e: Json) => e.id));
  const issues = validateAsset(asset, { environments: envs });
  console.log(`[DEBUG] validateAsset ${issues.length} issues`);
  for (const i of issues.slice(0, 60)) console.log(`  ASSET ${i.code} ${i.path}: ${i.message.en}`.slice(0, 400));
}
if (flags.has("--volumes")) {
  const { checkVolumes } = await import(join(STAFFAGE, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
  const t0 = Date.now();
  const findings = checkVolumes(asset, {}, { progress: (p: Json) => process.stderr.write(`[DEBUG] ${JSON.stringify(p).slice(0, 160)}\n`) });
  console.log(`[DEBUG] checkVolumes ${findings.length} findings in ${((Date.now() - t0) / 1000).toFixed(0)} s`);
  for (const f of findings) console.log(`  VOL ${f.rule} ${f.code} ${f.path}: ${f.message.en} ${JSON.stringify(f.data ?? {})}`.slice(0, 700));
}
if (flags.has("--check")) {
  const { checkAssetDetailed } = await import(join(STAFFAGE, "🔨️modules/🗂️catalog/🩺️check/🟦️.ts"));
  const r = checkAssetDetailed(asset, { samples: 9 });
  console.log(`${r.problems.length === 0 ? "✓" : "✗"} ${id}`);
  for (const p of r.problems) console.log(`  ${p}`.slice(0, 400));
  for (const l of r.lines) console.log(`  ${l}`);
}
if (flags.has("--render")) {
  const { runPreview } = await import(join(STAFFAGE, "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts"));
  const out = join(GEN, "png");
  const file = join(GEN, "candidate.json");
  for (const scale of [50, 100, 200, 500]) {
    await runPreview(["--asset", id, "--file", file, "--scale", String(scale), "--view", "light-solid", "--views", "elevation,front,plan,axonometric", "--projection", "third", "--language", "en", "--dpi", "150", "--out", out]);
  }
  if (flags.has("--thermal")) await runPreview(["--asset", id, "--file", file, "--scale", "100", "--view", "thermal", "--viewpoint", "elevation", "--flow", "none", "--language", "en", "--dpi", "150", "--out", out]);
  console.log(`[DEBUG] rendered into ${out}`);
}
