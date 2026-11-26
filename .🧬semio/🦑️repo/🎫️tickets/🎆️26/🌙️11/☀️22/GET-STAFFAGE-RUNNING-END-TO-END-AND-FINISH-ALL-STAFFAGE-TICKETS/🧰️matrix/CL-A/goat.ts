/** 🐐️ The closing repair of `animals/goat` (idempotent, the last step of its chain after `../MA/lfinish.sh goat`): `bun goat.ts <in.json|goat> <out.json> [--horn keep|auto|measured]`.
 *
 * 1. Torso junctions (pin `goatBarrel` of `🧊️model-skeleton`): `pelvic-barrel` ends in the very section `abdominal-barrel` starts with, and `abdominal-barrel` ends in the section `costal-barrel` starts with, at every scale. Since the trunk was re-cut on 2026-11-22 each barrel ran one station into its neighbour, so the end and start sections lay 46.1 mm and 55.0 mm apart with unequal radii. The doubled station is dropped where the station before it is the shared one, else the last station becomes the shared one; the place is the neighbour's first section carried into the bone frame through `evaluate` (8 decimals). The drawings are not touched: the drawn lofts `croup` or `pelvic-barrel` and `abdominal-barrel` keep their run of one station into the neighbour, because a drawn loft that ends in the very section its neighbour starts with shows that section as a seam in the line view (measured on the first trial of 2026-11-26 14:15: a line across the rump in the plan, two ticks on the back line of the side elevation); the union of the drawn lofts is the union of the solids.
 * 2. Horns at 1:500 (pin "retains the measured horn stations" of `🧊️model-silhouette`), only on request: `--horn measured` writes stations 0, 3, 6, 11 and 15 of the sixteen measured 1:50 stations with their measured radii in the place of the three stations of a constant 13.8 mm, `--horn auto` does so only when `volumes-feature` then has no finding for the horns, and the default `keep` leaves them. Measured on 2026-11-26: the measured horn is 23.0 mm thin where the thinnest drawing of bone `head` asks 26.4 mm, so it is a finding of `volumes-feature` since that rule judges every scale on its own; the case pins the allowance that was removed (request in `requests.md`).
 * @see ../MR/lib.ts */
import { join } from "node:path";
import { S, readDoc, writeDoc } from "../MR/lib.ts";

const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { featureRule } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts"));
const [source, out, ...rest] = process.argv.slice(2);
const horn = rest.includes("--horn") ? rest[rest.indexOf("--horn") + 1]! : "keep";
const read = readDoc(source!);
const asset = read.doc.asset;
type Section = { at: number[]; rx: number; ry: number; exponent?: number; rotation?: number[] };
type Loft = { id: string; bone: string; kind: string; sections: Section[] };
const frames = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }).bones;
const axes = (bone: string) => { const f = frames[bone].frame; return [f.o, f.u, f.v, f.w].map((p: { x: number; y: number; z: number }) => [p.x, p.y, p.z]); };
const world = (bone: string, at: readonly number[]): number[] => { const [o, u, v, w] = axes(bone); return [0, 1, 2].map((k) => o![k]! + at[0]! * u![k]! + at[1]! * v![k]! + at[2]! * w![k]!); };
const local = (bone: string, p: readonly number[]): number[] => { const [o, u, v, w] = axes(bone); return [u, v, w].map((a) => Number(([0, 1, 2].reduce((s, k) => s + (p[k]! - o![k]!) * a![k]!, 0)).toFixed(8)) + 0); };
const gap = (a: readonly number[], b: readonly number[]): number => Math.hypot(a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!);

/** 🔗️ Lets `a` end in the first section of `b`. */
function join2(a: Loft, b: Loft): string {
  const first = b.sections[0]!, place = world(b.bone, first.at);
  const meets = (s: Section | undefined) => s !== undefined && s.rx === first.rx && s.ry === first.ry && gap(world(a.bone, s.at), place) < 1e-4;
  const before = gap(world(a.bone, a.sections.at(-1)!.at), place);
  const how = meets(a.sections.at(-1)) ? "kept" : meets(a.sections.at(-2)) ? "dropped the doubled station" : "last station moved to the shared one";
  if (how === "dropped the doubled station") a.sections.pop();
  const last = a.sections.at(-1)!;
  a.sections[a.sections.length - 1] = { at: local(a.bone, place), rx: first.rx, ry: first.ry, ...(first.exponent === undefined ? {} : { exponent: first.exponent }), ...(last.rotation === undefined ? {} : { rotation: last.rotation }) };
  return `${a.id} -> ${b.id}: ${how}, ${a.sections.length} sections, gap ${(before * 1000).toFixed(3)} mm -> ${(gap(world(a.bone, a.sections.at(-1)!.at), place) * 1000).toFixed(6)} mm`;
}

for (const scale of Object.keys(asset.volumes)) {
  const volume = asset.volumes[scale];
  if (volume.status !== "authored") continue;
  const part = (id: string): Loft => volume.body.find((s: Loft) => s.id === id);
  const [rump, loin, chest] = ["pelvic-barrel", "abdominal-barrel", "costal-barrel"].map(part) as [Loft, Loft, Loft];
  console.log(`1:${scale} ${join2(rump, loin)}; ${join2(loin, chest)}`);
}

const coarse = asset.volumes["500"], fine = asset.volumes["50"];
const TEXT = [
  ["neck, head, horns and ears thickened to what the drawing shows", "neck, head and ears thickened to what the drawing shows, the horns as five of their sixteen measured stations with the measured radii"],
  ["Hals, Kopf, Hörner und Ohren auf das verdickt, was die Zeichnung zeigt", "Hals, Kopf und Ohren auf das verdickt, was die Zeichnung zeigt, die Hörner als fünf ihrer sechzehn gemessenen Stationen mit den gemessenen Radien"],
] as const;
if (horn !== "keep") {
  const was = ["horn-n", "horn-f"].map((id) => structuredClone(coarse.body.find((s: Loft) => s.id === id).sections));
  const paths: string[] = [];
  for (const id of ["horn-n", "horn-f"]) {
    const [c, f] = [coarse, fine].map((v) => v.body.find((s: Loft) => s.id === id)) as [Loft, Loft];
    c.sections = [0, 3, 6, 11, 15].map((i) => structuredClone(f.sections[i]!));
    paths.push(`volumes.500.body[${coarse.body.indexOf(c)}]`);
  }
  const findings = featureRule(asset, [500]).findings.filter((f: { path: string }) => paths.includes(f.path));
  console.log(`horns at 1:500 as measured stations: ${findings.length} findings of volumes-feature${findings.map((f: { message: { en: string } }) => `\n  ${f.message.en}`).join("")}`);
  const keep = horn === "measured" || findings.length === 0;
  if (!keep) ["horn-n", "horn-f"].forEach((id, i) => { coarse.body.find((s: Loft) => s.id === id).sections = was[i]; });
  const text = coarse.decisions[0];
  if (keep) { text.en = text.en.replace(TEXT[0][0], TEXT[0][1]); text.de = text.de.replace(TEXT[1][0], TEXT[1][1]); }
  console.log(`horns at 1:500: ${keep ? "measured stations written" : "kept as they were"}`);
}
const NOTE = {
  en: " Croup, loin and chest meet in one shared section each, so the three barrels close on each other without a doubled station.",
  de: " Kruppe, Lende und Brust treffen sich in je einem gemeinsamen Schnitt, sodass die drei Rumpfkörper ohne doppelte Station aneinander schließen.",
};
const body = fine.decisions[0];
if (!body.en.includes(NOTE.en.trim())) { body.en += NOTE.en; body.de += NOTE.de; }
console.log(writeDoc(read, out) ? `written ${out}` : "unchanged");
