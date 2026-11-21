/** 🔦️ Splits the outer area of every thermal region of one scale by reach (air, cover, cavity; E3) per solid, so an author sees which faces count as contents of a closed cavity.
 *
 * Usage: `bun r5-glazed-probe.ts <category>/<slug> --scale N [--region ID]`
 * @see ./r5-glazed-cavities.md */
import { STAFFAGE, entryOf, loadSource, merged } from "./volumes-lib.ts";

const V = `${STAFFAGE}/🔨️modules/🗂️catalog/🧊️volumes`;
const { exposedShares, outsideOf, SEAM_FEATURES } = await import(`${V}/🧮️measures/🟦️.ts`);
const { worldSolids } = await import(`${V}/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${V}/🧱️rules/🟦️.ts`);
const { evaluate } = await import(`${STAFFAGE}/🔨️modules/🧮️evaluation/🟦️.ts`);

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
const { entry, asset: catalog } = entryOf(args[0]!);
const source = loadSource(entry);
const asset = source === null ? catalog : merged(catalog, source);
const d = Number(flag("--scale") ?? 50);
const only = flag("--region");
const pose = asset.presentation.defaultPose;
const state = evaluate(asset, { kind: "pose", pose }, asset.representations[`${d}`]?.status === "authored" ? { kind: "scale", scale: { denominator: d } } : undefined);
const worlds = worldSolids(asset, state.bones, d, "thermal");
const volume = asset.volumes[`${d}`];
const feature = featureOf(d, asset.presentation?.printProfile?.minFeatureMm ?? 0.15);
const probe = Math.max(1e-4, 0.02 * feature);
const exchange = new Set(asset.thermal.regions.filter((r: any) => r.areaBasis === "exchange").map((r: any) => r.id));
const porous = new Set(volume.thermal.filter((s: any) => exchange.has(s.region)).map((s: any) => s.of ?? s.id));
const walls = worlds.filter((w: any) => !porous.has(w.id) && w.envelope !== true && w.emitter !== true);
const outside = outsideOf(walls, feature, probe);
const regionOf = new Map(volume.thermal.map((s: any) => [s.id, s.region]));
for (const region of asset.thermal.regions) {
  if (only !== undefined && region.id !== only) continue;
  const own = worlds.filter((w: any) => regionOf.get(w.id) === region.id && w.emitter !== true && w.envelope !== true);
  const ownSet = new Set(own);
  const reach = (b: never, n: never) => outside.reach(b, n, (w: unknown) => ownSet.has(w));
  const total = exposedShares(own, own, [], () => false, probe, SEAM_FEATURES * feature, region.hidden === undefined ? reach : undefined);
  process.stdout.write(`${region.id} (${region.kind}${region.hidden === undefined ? "" : ", hidden"}) areaM2 ${region.areaM2}: air ${total.air.toFixed(4)} cover ${total.cover.toFixed(4)} cavity ${total.cavity.toFixed(4)}\n`);
  if (region.hidden !== undefined) continue;
  for (const w of own) {
    const s = exposedShares([w], own, [], () => false, probe, SEAM_FEATURES * feature, reach);
    if (s.cavity > 1e-5) process.stdout.write(`   ${w.id}: air ${s.air.toFixed(4)} cover ${s.cover.toFixed(4)} cavity ${s.cavity.toFixed(4)}\n`);
  }
}
