/** 🥁️ Gives the 1:500 band of `used-timpani-pair` the felt heads of its two mallets back: `bun timpani.ts [file.json] [--out file] [--write]`.
 *
 * The declared height of the pair (0.928 m) is the top of the 38 mm felt head of the mallet that lies on each cover. The 1:500 band had merged the head into the 13 mm shaft, so the band drew 0.9159 m (12.1 mm short), and at the coarse end of its range (1:1000) the depiction reduces the 0.013 mm shaft to its axis (Douglas-Peucker at 0.01 mm of paper), which loses another 6.5 mm: 18.6 mm against the 18.56 mm the matrix allows (`paper-height`, 7 cells). The run sets, idempotently, the head `ball-a` and `ball-b` of the 1:200 band into the 1:500 band: the ellipsoid in `volumes.500.body` with its thermal and structure references, the three drawn shapes directly above the shaft of their mallet, and the decision texts that said the head was merged.
 * @see ./paper.ts — the measurement of the paper box
 * @see ./path.ts — the path items of the shaft at 1:500 and 1:1000
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0] !== undefined && !args[0].startsWith("--") ? args[0] : "used-timpani-pair");
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const [fine, coarse] = [asset.representations["200"], asset.representations["500"]];
const HEADS = ["ball-a", "ball-b"];
const after = (list: any[], id: string, entry: any): void => {
  const known = list.findIndex((item) => item.id === entry.id);
  if (known >= 0) list.splice(known, 1);
  list.splice(list.findIndex((item) => item.id === id) + 1, 0, entry);
};
for (const head of HEADS) {
  const shaft = head.replace("ball", "shaft");
  for (const view of ["elevation", "front", "plan"]) {
    const list: any[] = coarse.geometry[view];
    const under = list.find((shape) => shape.id === shaft);
    after(list, shaft, { ...structuredClone(fine.geometry[view].find((shape: any) => shape.id === head)), z: Math.round((under.z + 0.005) * 1e4) / 1e4 });
  }
  const [from, to] = [asset.volumes["200"], asset.volumes["500"]];
  after(to.body, shaft, structuredClone(from.body.find((solid: any) => solid.id === head)));
  after(to.thermal, `th-${shaft}`, structuredClone(from.thermal.find((entry: any) => entry.id === `th-${head}`)));
  after(to.structure, `st-${shaft}`, structuredClone(from.structure.find((entry: any) => entry.id === `st-${head}`)));
}
const TEXTS: [any, "en" | "de", string, string][] = [
  [coarse.decisions.silhouette, "en", "each mallet is a single stroke on its cover.", "each mallet is a single stroke with its felt head on its cover."],
  [coarse.decisions.silhouette, "de", "jeder Schlägel ist ein einzelner Strich auf seiner Haube.", "jeder Schlägel ist ein einzelner Strich mit seinem Filzkopf auf seiner Haube."],
  [coarse.decisions.retained, "en", "and the protruding pedals.", "and the protruding pedals; the 38 mm felt heads of the mallets, which set the height of the stored pair."],
  [coarse.decisions.retained, "de", "und die vorstehenden Pedale.", "und die vorstehenden Pedale; die 38-mm-Filzköpfe der Schlägel, die die Höhe des gelagerten Paars bestimmen."],
  [coarse.decisions.merged, "en", "base arms, the mallet head is part of its shaft, every", "base arms, every"],
  [coarse.decisions.merged, "de", "Fußarmen, der Schlägelkopf gehört zu seinem Schaft, jede", "Fußarmen, jede"],
  [asset.volumes["500"].decisions[0], "en", "and the mallet a single slender rod;", "and the mallet a slender rod with its 38 mm felt head, which sets the height of the stored pair;"],
  [asset.volumes["500"].decisions[0], "de", "und der Schlägel ein einzelner schlanker Stab;", "und der Schlägel ein schlanker Stab mit seinem 38-mm-Filzkopf, der die Höhe des gelagerten Paars bestimmt;"],
];
for (const [holder, language, old, text2] of TEXTS) {
  if (holder[language].includes(text2)) continue;
  if (!holder[language].includes(old)) throw new Error(`text not found: ${old}`);
  holder[language] = holder[language].replace(old, text2);
}
const out = flag("out");
const written = rewrite(text, previous, next);
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written === text && out === undefined) console.log("used-timpani-pair: unchanged");
  else { writeFileSync(out ?? file, written); console.log(`used-timpani-pair: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
} else console.log(`used-timpani-pair: ${written === text ? "unchanged" : "would change"} (no --write)`);
