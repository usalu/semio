/** 🧲️ Brings structure solids that `volumes-structure-coverage` reports outside the body silhouette back into the body: per reported solid a descent over small translations in its bone frame (5, 10, 20 mm per axis) and a smaller section (radii, size across), which keeps the change with the smallest excess over all three views, until the rule passes at the scale. Merges the ops into the repair file of the person.
 *  bun nudge.ts <slug> [--scales 50,100] [--file <asset.json>] [--dry] [--only id,id] */
import { parseArgs } from "node:util";
import { checkVolumes, featureOf, measureVolumes } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { applyOp, applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" }, file: { type: "string" }, dry: { type: "boolean" }, only: { type: "string" } } });
const slug = positionals[0]!;
const asset = (await Bun.file(values.file ?? catalogFile(slug)).json()).asset;
const existing = await readOps(opsFile(slug));
applyOps(asset, existing);
const only = values.only === undefined ? undefined : new Set(values.only.split(","));
const RULE = ["volumes-structure-coverage"] as never;
const ops: Op[] = [];
for (const scale of (values.scales ?? "50,100,200,500").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored")) {
  const feature = featureOf(scale);
  const excess = (): number => (measureVolumes(asset, { scale: scale as never, rules: RULE }) as any[]).filter((m) => String(m.name).startsWith("outsideBodyM:")).reduce((sum, m) => sum + Math.max(0, m.value - feature), 0);
  const culprits = (): string[] => [...new Set(checkVolumes(asset, { scale: scale as never, rules: RULE }).flatMap((f) => [...f.message.en.matchAll(/"([^"]+)" [\d.]+ mm/gu)].map((m) => m[1]!)))].filter((id) => only === undefined || only.has(id));
  const tried = new Set<string>();
  for (let guard = 0; guard < 12; guard++) {
    const id = culprits().find((x) => !tried.has(x));
    if (id === undefined) break;
    tried.add(id);
    const solid = (asset.volumes[String(scale)].structure as any[]).find((s) => s.id === id);
    if (solid === undefined || solid.of !== undefined) { console.log(`[DEBUG] 1:${scale} ${id}: a reference to a body solid, not moved`); continue; }
    const start = excess();
    let current = start;
    const original = structuredClone(solid);
    for (let round = 0; round < 6 && current > 1e-6; round++) {
      const base = structuredClone(solid);
      let best: { value: number; state: any; what: string } | undefined;
      const trial = (what: string, change: () => void): void => {
        Object.keys(solid).forEach((k) => delete solid[k]); Object.assign(solid, structuredClone(base));
        change();
        const value = excess();
        if (value < (best?.value ?? current - 1e-5)) best = { value, state: structuredClone(solid), what };
      };
      for (const step of [0.005, 0.01, 0.02]) for (const axis of [0, 1, 2]) for (const dir of [1, -1]) trial(`${"uvw"[axis]} ${dir * step * 1000} mm`, () => { const t = [...(solid.transform?.translation ?? [0, 0, 0])]; t[axis] = Number((t[axis] + dir * step).toFixed(5)); solid.transform = { ...(solid.transform ?? {}), translation: t }; });
      if (Array.isArray(base.radii)) trial("radii x 0.7", () => { solid.radii = base.radii.map((r: number) => Number((r * 0.7).toFixed(5))); });
      if (base.kind === "box" && Array.isArray(base.size)) trial("section x 0.7", () => { const long = base.size.indexOf(Math.max(...base.size)); solid.size = base.size.map((x: number, k: number) => (k === long ? x : Number((x * 0.7).toFixed(5)))); });
      if (typeof base.r0 === "number") trial("radius x 0.7", () => { solid.r0 = Number((base.r0 * 0.7).toFixed(5)); solid.r1 = Number((base.r1 * 0.7).toFixed(5)); });
      Object.keys(solid).forEach((k) => delete solid[k]);
      if (best === undefined) { Object.assign(solid, base); break; }
      const chosen: { value: number; state: any; what: string } = best;
      Object.assign(solid, chosen.state);
      current = chosen.value;
      console.log(`[DEBUG] 1:${scale} ${id}: ${chosen.what} -> excess ${(current * 1000).toFixed(1)} mm`);
    }
    if (current < start - 1e-6) { ops.push({ op: "solid", scale: String(scale), list: "structure", id, solid: structuredClone(solid) }); console.log(`${slug} 1:${scale} ${id}: excess ${(start * 1000).toFixed(1)} -> ${(current * 1000).toFixed(1)} mm`); }
    else { Object.keys(solid).forEach((k) => delete solid[k]); Object.assign(solid, original); console.log(`${slug} 1:${scale} ${id}: no small change helps (excess ${(start * 1000).toFixed(1)} mm)`); }
  }
  for (const f of checkVolumes(asset, { scale: scale as never, rules: ["volumes-structure-coverage", "volumes-connected"] as never })) console.log(`   LEFT 1:${scale} ${f.path}: ${f.message.en}`.slice(0, 330));
}
if (!values.dry && ops.length > 0) await writeOps(opsFile(slug), mergeOps(existing, ops));
console.log(`${slug}: ${ops.length} solids moved${values.dry ? " (dry)" : ""}`);
