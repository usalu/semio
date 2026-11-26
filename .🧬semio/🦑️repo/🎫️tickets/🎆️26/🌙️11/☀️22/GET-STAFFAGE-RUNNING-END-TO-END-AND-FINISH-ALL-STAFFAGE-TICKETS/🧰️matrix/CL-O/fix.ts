/** 🩹️ The repair of the closing lane CL-O, the last step of the chain of its five assets, idempotent: `bun fix.ts <id> [--out <file.json>] [--write]` restores the detail that `🧪️tests/🧊️model-skeleton` and `🧪️tests/📐️plan-dimension` pin and prints one line per change. Without `--write` and `--out` it only reports; `--out` writes the repaired document to a trial copy, `--write` to the catalog file (refused when the file changed on disk since it was read).
 *
 * - `gas-hob`: the knob stands 26 mm proud of the fascia (grip ridge 10 mm instead of 9 at 1:50, stub 26 mm at 1:100), so the drawn plan depth is the declared 0.52 m (plate to y 0.260, knob to y −0.260) as the 1:200 drawing and the coarse volumes already say; the texts name the plate depth that is drawn (0.496 m).
 * - `city-bicycle`: the eight nipple solids of 1:200 and 1:500 (`ops/city-bicycle.json`) with their thermal and structure references, seated on their spokes in the rim band.
 * - `scissor-lift`: every bushing bored 20.25 mm in radius for its 20 mm pin at 1:50 and 1:100; at 1:200 the arms are the bored box sections of 1:100 and carry their bushings.
 * - `excavator`: the 1:200 idler cross pin lists its shaft radius first (the same closed profile), and 1:500 carries the pin as one round of its head diameter inside hub and yokes.
 * @see ./pins.ts — the measuring copy of the pinned cases */
import { parseArgs } from "node:util";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readDoc, writeDoc } from "../MR/lib.ts";

type Solid = Record<string, any>;
type Volume = { body: Solid[]; thermal: Solid[]; structure: Solid[]; decisions: { en: string; de: string }[] };
const log: string[] = [];
const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b);
const set = (target: Solid, key: string, value: unknown, what: string): void => {
  if (same(target[key], value)) return;
  log.push(`${what}: ${key} ${JSON.stringify(target[key])} -> ${JSON.stringify(value)}`);
  target[key] = value;
};
const after = (list: Solid[], id: string, entry: Solid, what: string): void => {
  if (list.some((s) => s.id === entry.id)) return;
  const at = list.findIndex((s) => s.id === id);
  if (at < 0) throw new Error(`${what}: no "${id}" to place "${entry.id}" after`);
  list.splice(at + 1, 0, entry);
  log.push(`${what}: ${entry.id} added after ${id}`);
};
const sentence = (decision: { en: string; de: string }, from: { en: string; de: string }, to: { en: string; de: string }, what: string): void => {
  for (const language of ["en", "de"] as const) {
    if (decision[language].includes(to[language])) continue;
    if (!decision[language].includes(from[language])) throw new Error(`${what}: the ${language} text to replace is not there: ${from[language]}`);
    decision[language] = decision[language].replace(from[language], to[language]);
    log.push(`${what}: ${language} text`);
  }
};
const append = (decision: { en: string; de: string }, text: { en: string; de: string }, what: string): void => {
  for (const language of ["en", "de"] as const) {
    if (decision[language].includes(text[language])) continue;
    decision[language] = `${decision[language]} ${text[language]}`;
    log.push(`${what}: ${language} text added`);
  }
};

const repairs: Record<string, (asset: any) => void> = {
  "gas-hob": (asset) => {
    for (const scale of ["50", "100"]) {
      const geometry = asset.representations[scale].geometry;
      const plan = geometry.plan.find((s: Solid) => s.id === "plan-knob");
      set(plan, "y", -0.26, `1:${scale} plan-knob`);
      set(plan, "h", 0.026, `1:${scale} plan-knob`);
      for (const view of ["front", "elevation"]) set(geometry[view].find((s: Solid) => s.id === "knob").solid, "z0", -0.26, `1:${scale} ${view} knob solid`);
    }
    const fine = asset.volumes["50"] as Volume, ridge = fine.body.find((s) => s.id === "knob-ridge")!, pointer = fine.body.find((s) => s.id === "knob-pointer")!;
    set(ridge, "size", [0.009, 0.036, 0.01], "1:50 knob-ridge");
    set(ridge.transform, "translation", [0, 0, -0.255], "1:50 knob-ridge");
    set(pointer.transform, "translation", [0, 0.012, -0.2601], "1:50 knob-pointer");
    set(asset.volumes["100"].body.find((s: Solid) => s.id === "knob"), "profile", [[0, 0], [0, 0.02], [0.023, 0.02], [0.026, 0.017], [0.026, 0]], "1:100 knob");
    sentence(asset.provenance.notes, { en: "0.497 m plate and the 25 mm knob in front of it", de: "0,497 m Platte und davor der 25 mm tiefe Knebel" }, { en: "0.496 m plate and the knob that stands 26 mm proud of the fascia, 24 mm in front of the plate", de: "0,496 m Platte und der Knebel, der 26 mm vor der Blende steht, 24 mm vor der Platte" }, "provenance note");
    const plate = (text: { en: string; de: string }, what: string): void => {
      for (const [language, from, to] of [["en", "0.497 m", "0.496 m"], ["de", "0,497 m", "0,496 m"]] as const) {
        if (!text[language].includes(from)) continue;
        text[language] = text[language].replaceAll(from, to);
        log.push(`${what}: ${language} plate depth ${to}`);
      }
    };
    plate(asset.representations["50"].decisions.silhouette, "1:50 silhouette decision");
    for (const scale of ["50", "100", "200", "500"]) plate(asset.volumes[scale].decisions[0], `1:${scale} volume decision`);
  },
  "city-bicycle": (asset) => {
    const { nipples } = JSON.parse(readFileSync(join(import.meta.dir, "ops/city-bicycle.json"), "utf8")) as { nipples: Solid[] };
    for (const scale of ["200", "500"]) {
      const volume = asset.volumes[scale] as Volume;
      for (const nipple of nipples) {
        const spoke = nipple.id.replace("nipple-", "spoke-"), wheel = nipple.id.includes("-f-") ? "wheel-f" : "wheel-r";
        after(volume.body, spoke, structuredClone({ ...nipple, bone: wheel }), `1:${scale} body`);
        after(volume.thermal, `th-${spoke}`, { id: `th-${nipple.id}`, of: nipple.id, region: "rims-spokes" }, `1:${scale} thermal`);
        after(volume.structure, spoke, { id: `st-${nipple.id}`, of: nipple.id, member: wheel, part: "member" }, `1:${scale} structure`);
      }
    }
    sentence(asset.volumes["200"].decisions[4], { en: "the spoke nipples are left out and the bearing sleeve", de: "die Speichennippel entfallen und die Lagerhülse" }, { en: "the 36 spoke nipples keep their bored seats in the rim band (below the feature, carried as texture) and the bearing sleeve", de: "die 36 Speichennippel behalten ihre gebohrten Sitze im Felgenband (unter dem Merkmal, als Textur geführt) und die Lagerhülse" }, "1:200 decision");
    sentence(asset.volumes["500"].decisions[4], { en: "the nipples are left out;", de: "die Nippel entfallen;" }, { en: "the 36 nipples keep their bored seats in the rim band (below the feature, carried as texture);", de: "die 36 Nippel behalten ihre gebohrten Sitze im Felgenband (unter dem Merkmal, als Textur geführt);" }, "1:500 decision");
  },
  "scissor-lift": (asset) => {
    const ring = [[0, 0.02025], [0.05, 0.02025], [0.05, 0.0235], [0, 0.0235], [0, 0.02025]];
    for (const scale of ["50", "100"]) for (const solid of (asset.volumes[scale] as Volume).body) if (solid.id.startsWith("sleeve-")) set(solid, "profile", ring, `1:${scale} ${solid.id}`);
    const fine = asset.volumes["100"] as Volume, coarse = asset.volumes["200"] as Volume;
    for (const side of ["a", "b"]) for (const index of [1, 2, 3, 4]) {
      const name = `${side}${index}`, at = coarse.body.findIndex((s) => s.id === `arm-${name}`), bored = fine.body.find((s) => s.id === `arm-${name}`)!;
      if (coarse.body[at]!.kind !== "polyhedron") {
        coarse.body[at] = structuredClone(bored);
        log.push(`1:200 arm-${name}: the bored box section of 1:100 in the place of the plain tube`);
      }
      for (const station of [0, 1, 2]) {
        const id = `sleeve-${name}-${station}`, source = fine.body.find((s) => s.id === id)!, before = (stem: string) => (station === 0 ? `${stem}arm-${name}` : `${stem}sleeve-${name}-${station - 1}`);
        after(coarse.body, before(""), { ...structuredClone(source), profile: ring }, "1:200 body");
        after(coarse.thermal, before("th-"), { id: `th-${id}`, of: id, region: "arms" }, "1:200 thermal");
        after(coarse.structure, before("st-"), { id: `st-${id}`, of: id, member: name, part: "joint" }, "1:200 structure");
      }
    }
    sentence(asset.volumes["50"].decisions[3], { en: "pivot bushings have a 7.5 mm wall and the cross tubes are", de: "die Gelenkbuchsen haben 7,5 mm Wand und die Querrohre sind" }, { en: "pivot bushings keep their real section (40.5 mm bore for the 40 mm pin, 47 mm outside, seated in the 47 mm bores of the arms) and the cross tubes are", de: "die Gelenkbuchsen behalten ihren wirklichen Querschnitt (40,5 mm Bohrung für den 40-mm-Bolzen, 47 mm außen, in den 47-mm-Bohrungen der Arme) und die Querrohre sind" }, "1:50 decision");
    sentence(asset.volumes["100"].decisions[3], { en: "bushings with 15 mm wall,", de: "Buchsen mit 15 mm Wand," }, { en: "bushings in their real section (40.5 mm bore, 47 mm outside),", de: "Buchsen im wirklichen Querschnitt (40,5 mm Bohrung, 47 mm außen)," }, "1:100 decision");
    sentence(asset.volumes["200"].decisions[3], { en: "arms as 90 x 50 mm bars with chamfered ends, 40 mm pivot pins without bushings and heads,", de: "Arme als 90 x 50 mm Stäbe mit gefasten Enden, 40-mm-Gelenkbolzen ohne Buchsen und Köpfe," }, { en: "arms as the bored 90 x 50 mm box sections of 1:100 (their 38 mm cavity and their 47 mm bores are above the feature), 40 mm pivot pins in their bushings without heads,", de: "Arme als die gebohrten 90 x 50 mm Kastenprofile von 1:100 (ihr 38-mm-Hohlraum und ihre 47-mm-Bohrungen liegen über dem Merkmal), 40-mm-Gelenkbolzen in ihren Buchsen ohne Köpfe," }, "1:200 decision");
  },
  excavator: (asset) => {
    const pin = (asset.volumes["200"] as Volume).body.find((s) => s.id === "idler-cross-pin")!;
    set(pin, "profile", [[0, 0.035], [0.3, 0.035], [0.3, 0], [0, 0]], "1:200 idler-cross-pin");
    const coarse = asset.volumes["500"] as Volume;
    after(coarse.body, "idler-yoke-inner", { id: "idler-cross-pin", kind: "revolve", bone: "undercarriage", material: "track-steel", role: "detail", tone: 0.48, profile: [[0, 0.043], [0.305, 0.043], [0.305, 0], [0, 0]], axis: [0, 0, 1], origin: [1.825, 0.45, 1.04], repeat: { count: 2, translation: [0, 0, -2.38] } }, "1:500 body");
    after(coarse.thermal, "th-idler-yoke-inner", { id: "th-idler-cross-pin", of: "idler-cross-pin", region: "tracks" }, "1:500 thermal");
    append(coarse.decisions[4], { en: "The idler cross pin is one round of its 86 mm head diameter through hub and yoke arms.", de: "Der Querbolzen des Leitrads ist ein Rundstab seines 86-mm-Kopfdurchmessers durch Nabe und Gabelarme." }, "1:500 decision");
  },
};

const { values, positionals } = parseArgs({ allowPositionals: true, options: { out: { type: "string" }, write: { type: "boolean", default: false } } });
const read = readDoc(positionals[0]!), repair = repairs[read.doc.asset.id];
if (repair === undefined) throw new Error(`${read.doc.asset.id} is no asset of lane CL-O`);
repair(read.doc.asset);
for (const line of log.slice(0, Number(process.env.SHOW ?? 400))) console.log(`   ${line}`);
const wrote = values.out !== undefined ? writeDoc(read, values.out) : values.write ? writeDoc(read) : false;
console.log(`${read.doc.asset.id}: ${log.length} changes${values.out !== undefined ? `, trial copy ${values.out}` : values.write ? (wrote ? ", catalog file written" : ", catalog file unchanged") : ", nothing written"}`);
