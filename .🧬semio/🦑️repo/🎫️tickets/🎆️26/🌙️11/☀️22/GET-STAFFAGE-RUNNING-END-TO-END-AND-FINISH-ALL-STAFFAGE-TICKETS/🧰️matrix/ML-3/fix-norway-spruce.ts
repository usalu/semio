/** 🌲️ The multiview repair of `norway-spruce` at 1:200, the last step of its chain (after `STAFFAGE-PLANT-COLLECTION-WAVE/🧰️scripts/PW-07/norway_spruce.py`), idempotent: `bun fix-norway-spruce.ts [--in file] [--out file] [--write]`.
 *
 * Cause of `derived-view-in-band` front at 1:200: the individual has two leading sprays per whorl in the sagittal plane (west and east, on the bones `spray-w-NN` and `spray-e-NN`), which make the tree 8.48 m wide and 7.41 m deep. The 1:100 body keeps them as capsules beyond the rim of each skirt; the 1:200 body had merged them into the skirt by stretching every skirt of revolution along the sagittal axis (lateral scale 0.80 to 0.85 instead of 0.94 to 1.06), so the front elevation was the side elevation squeezed by one factor per tier and the spray bones drew nothing at 1:200.
 * The run gives the 1:200 band what it can print (a spray is 0.25 to 0.6 m thick and 1.5 to 4.6 m long: 1.2 to 3 mm by 7 to 23 mm on paper):
 * - every skirt `tier-NN` of `volumes.200.body` goes back to the reach of the skirt itself: its profile radii are scaled by (largest radius of the 1:100 skirt) / (its own largest radius) and its lateral scale becomes that of the 1:100 skirt (the depth of the skirt does not change, so its front outline stays); its elevation path and its plan path are scaled by the same factor about the axis of the skirt;
 * - the two leading sprays `tier-NN-spray-w` and `tier-NN-spray-e` of the 1:100 band (capsule solids on the spray bones, their thermal references and their capsule shapes in the three views) are set into the 1:200 band after their skirt, painted below it as at 1:100;
 * - the decision texts of the representation and of the volume follow.
 * The lateral combs (`-spray-nw`, `-spray-se`) and the texture strokes of 1:100 stay merged or omitted at 1:200.
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = flag("in") ?? fileOf("norway-spruce");
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const round = (value: number, digits = 4): number => Number(value.toFixed(digits));
const [fine, coarse] = [asset.volumes["100"], asset.volumes["200"]];
const [fineRep, coarseRep] = [asset.representations["100"], asset.representations["200"]];
const SPRAY = /^tier-(\d\d)-spray-[we]$/u;
const reach = (solid: any): number => Math.max(...solid.profile.map((p: number[]) => p[1]!));
const scalePath = (d: string, map: (x: number, y: number) => [number, number]): string => d.replace(/(-?\d+(?:\.\d+)?) (-?\d+(?:\.\d+)?)/gu, (_: string, x: string, y: string) => map(Number(x), Number(y)).map((n) => round(n)).join(" "));

let skirts = 0;
for (const skirt of coarse.body.filter((solid: any) => /^tier-\d\d$/u.test(solid.id))) {
  const model = fine.body.find((solid: any) => solid.id === skirt.id);
  if (model === undefined) throw new Error(`norway-spruce: no 1:100 skirt ${skirt.id}`);
  const factor = reach(model) / reach(skirt);
  if (Math.abs(factor - 1) < 1e-3 && skirt.lateralScale === model.lateralScale) continue;
  if (factor > 1) throw new Error(`norway-spruce: the 1:200 skirt ${skirt.id} is narrower than the 1:100 skirt`);
  const [oy] = [skirt.origin[1] as number];
  skirt.profile = skirt.profile.map((p: number[]) => [p[0], round(p[1]! * factor)]);
  skirt.lateralScale = model.lateralScale;
  const elevation = coarseRep.geometry.elevation.find((shape: any) => shape.id === skirt.id);
  const plan = coarseRep.geometry.plan.find((shape: any) => shape.id === skirt.id);
  elevation.d = scalePath(elevation.d, (x, y) => [x, oy + (y - oy) * factor]);
  plan.d = scalePath(plan.d, (x, y) => [-oy + (x + oy) * factor, y]);
  skirts++;
}

let sprays = 0;
for (const solid of fine.body.filter((candidate: any) => SPRAY.test(candidate.id))) {
  const tier = `tier-${SPRAY.exec(solid.id)![1]}`;
  const place = (list: any[], item: any, after: (entry: any) => boolean): void => {
    const at = list.findIndex((entry) => entry.id === item.id);
    if (at >= 0) { if (JSON.stringify(list[at]) !== JSON.stringify(item)) list[at] = item; return; }
    let anchor = -1;
    list.forEach((entry, i) => { if (after(entry)) anchor = i; });
    if (anchor < 0) throw new Error(`norway-spruce: no place for ${item.id}`);
    list.splice(anchor + 1, 0, item);
    sprays++;
  };
  const own = (entry: any): boolean => entry.id === tier || (typeof entry.id === "string" && entry.id.startsWith(`${tier}-spray-`));
  place(coarse.body, structuredClone(solid), own);
  const reference = fine.thermal.find((entry: any) => entry.of === solid.id);
  if (reference === undefined) throw new Error(`norway-spruce: no thermal reference of ${solid.id} at 1:100`);
  place(coarse.thermal, structuredClone(reference), (entry) => entry.id === `th-${tier}` || (typeof entry.id === "string" && entry.id.startsWith(`th-${tier}-spray-`)));
  for (const view of ["elevation", "front", "plan"] as const) {
    const shape = structuredClone(fineRep.geometry[view].find((candidate: any) => candidate.id === solid.id));
    const skirtShape = coarseRep.geometry[view].find((candidate: any) => candidate.id === tier);
    if (shape === undefined || skirtShape === undefined) throw new Error(`norway-spruce: ${solid.id} or ${tier} is not drawn in the ${view}`);
    shape.z = round(skirtShape.z - (solid.id.endsWith("-w") ? 0.0003 : 0.0006));
    place(coarseRep.geometry[view], shape, own);
  }
}

const bodyCount = coarse.body.length;
Object.assign(coarseRep.decisions, {
  silhouette: { en: "At 1:200 (1 m is 5 mm) each of the 15 tiers is a drooping skirt, layered from the stem outwards, with its two leading sprays (west and east) reaching beyond the rim in the side elevation; in the front elevation the same sprays point at the viewer and stay inside the narrower cone, so side and front are two drawings of one tree. Every shape is the true view of its 1:200 solid; the stem shows below 2.3 m.", de: "Bei 1:200 (1 m sind 5 mm) ist jede der 15 Etagen eine hängende Schürze, vom Stamm aus geschichtet, deren zwei Leitzweige (West und Ost) in der Seitenansicht über den Saum hinausreichen; in der Vorderansicht zeigen dieselben Zweige auf den Betrachter und bleiben im schmaleren Kegel, so dass Seite und Front zwei Zeichnungen eines Baums sind. Jede Form ist die wahre Ansicht ihres 1:200-Körpers; unter 2,3 m zeigt sich der Stamm." },
  retained: { en: "Retained: every tier with its hanging rim, the two sagittal sprays per tier, the slight eccentricity and ellipticity of each tier, the stem with its flare and the leader.", de: "Beibehalten: jede Etage mit hängendem Saum, die zwei sagittalen Leitzweige je Etage, die leichte Außermittigkeit und Elliptizität jeder Etage, der Stamm mit Anlauf und der Gipfeltrieb." },
  merged: { en: "Merged: the lateral sprays, upturned tips and branchlets of each tier into its skirt (10 cm deeper than at 1:50); the root flare into the lower stem.", de: "Vereinigt: die seitlichen Zweige, aufgebogenen Spitzen und Zweiglein jeder Etage in ihre Schürze (10 cm tiefer als bei 1:50); der Wurzelanlauf in den unteren Stamm." },
  thinElements: { en: "The leader and the sprays keep their true taper (a spray is 1.2 to 3 mm thick on paper); nothing is thickened.", de: "Der Gipfeltrieb und die Zweige behalten ihre wahre Verjüngung (ein Zweig ist auf dem Papier 1,2 bis 3 mm dick); nichts wird verdickt." },
  styles: { en: "Light line: the layered skirt outlines read as tiers, the sprays as their ragged ends. Dark solid: shaded lower tiers darker than the sunlit upper ones.", de: "Helle Linie: die geschichteten Schürzenumrisse lesen sich als Etagen, die Zweige als ihre ausgefransten Enden. Dunkle Fläche: beschattete untere Etagen dunkler als die besonnten oberen." },
  overlays: { en: "Thermal: each skirt and spray carries its region (needles-sun from 14 m), the stem bark-lower and bark-upper. Structure: stem segments and spray members.", de: "Thermisch: jede Schürze und jeder Zweig trägt seine Region (needles-sun ab 14 m), der Stamm bark-lower und bark-upper. Tragwerk: Stammabschnitte und Zweigglieder." },
});
coarse.decisions[0] = { en: `Body at 1:200 (30 mm): five stem solids, 15 skirts of revolution and two tapered spray capsules per tier on the spray bones; ${bodyCount} solids.`, de: `Körper bei 1:200 (30 mm): fünf Stammkörper, 15 Drehschürzen und zwei verjüngte Zweigkapseln je Etage an den Zweigknochen; ${bodyCount} Körper.` };

const written = rewrite(text, previous, next);
console.log(`norway-spruce: ${skirts} skirts brought back to their own reach, ${sprays} spray entries set (solids, thermal references and shapes), ${bodyCount} body solids at 1:200; ${written === text ? "unchanged" : "changed"}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || (out !== undefined && !existsSync(out))) writeFileSync(out ?? file, written);
}
