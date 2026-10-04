/** ✂️ Read-only planner of work package A2: for every architecture species the anchored edit (old text → new text, from the last repertoire entry to the end of the document) that adds the minimal second-round members; the edits themselves are made by hand with the editor. */
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const PETS = join(ROOT, "🎓️teaching", "🏛️architecture", "🐾️pets");
const GEAR: Record<string, string[]> = {
  sunny: [],
  cloudy: [],
  housy: ["ladder", "grapple"],
  solary: ["climb", "ladder", "parachute"],
  radiatory: ["climb", "ladder"],
  pumpy: ["parachute"],
  windowy: ["climb", "parachute"],
  waly: ["ladder"],
  battery: ["climb", "ladder", "grapple"],
  windy: ["parachute"],
  boily: ["ladder", "grapple"],
  roofy: ["ladder", "parachute"],
  insuly: ["climb", "parachute"],
  shady: ["ladder", "parachute"],
  venty: [],
  chilly: ["climb", "parachute"],
  kettly: ["grapple", "parachute"],
  flamy: [],
  thermy: ["grapple", "parachute"],
  servy: ["grapple"],
};
const BRINGS: Record<string, string[]> = { climb: ["climb", "mantle", "slide"], ladder: ["carry", "climb"], grapple: ["aim", "reel"], parachute: ["glide"] };
const ORDER = ["hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "purr", "dizzy", "shrug", "push"];
const round = (value: number) => Math.round(value * 2) / 2;

for (const entry of readdirSync(PETS, { withFileTypes: true })) {
  if (!entry.isDirectory()) continue;
  let text: string;
  try {
    text = readFileSync(join(PETS, entry.name, "🔣️.json"), "utf8");
  } catch {
    continue;
  }
  const species = JSON.parse(text);
  if (Object.hasOwn(species, "states")) {
    console.log(`#### ${entry.name}: already carries the second-round members`);
    continue;
  }
  const repertoire = species.repertoire as Record<string, string[]>;
  const idle = repertoire.idle![0]!;
  const walk = (repertoire.walk ?? repertoire.hop)![0]!;
  const fall = repertoire.fall?.[0] ?? idle;
  const fidgets = repertoire.fidget!;
  const cuddle = repertoire.cuddle![0]!;
  const gear = GEAR[species.id]!;
  const wanted = new Set(["hang", "tumble", "purr", "dizzy", "shrug", "push", ...gear.flatMap((owned) => BRINGS[owned]!)]);
  const clipOf: Record<string, string> = { hang: idle, tumble: fall, glide: idle, aim: fidgets[0]!, reel: fidgets[fidgets.length - 1]!, climb: walk, mantle: walk, slide: fall, carry: walk, purr: cuddle, dizzy: fidgets[fidgets.length - 1]!, shrug: fidgets[fidgets.length - 1]!, push: fidgets[0]! };
  const added = ORDER.filter((activity) => wanted.has(activity)).map((activity) => `"${activity}": ["${clipOf[activity]}"]`);
  const start = text.lastIndexOf('"sulk": [');
  const old = text.slice(start).replace(/\n$/u, "");
  const sulkEnd = old.indexOf("]") + 1;
  const multiline = old.slice(sulkEnd).startsWith("\n");
  const entries = multiline ? added.map((line) => `,\n    ${line}`).join("") : added.map((line) => `, ${line}`).join("");
  const members = [
    `"states": [{ "id": "resting", "name": { "en": "Resting", "de": "In Ruhe" } }]`,
    `"tricks": []`,
    `"purr": { "clip": "${cuddle}" }`,
    `"emitters": []`,
    `"gear": [${gear.map((owned) => `"${owned}"`).join(", ")}]`,
    `"grip": ${round(species.size.height * 0.9)}`,
    `"reach": ${round(species.size.width * 0.25)}`,
    `"mood": "content"`,
  ];
  const body = old.slice(0, sulkEnd) + entries + old.slice(sulkEnd);
  const next = body.replace(/\n\}$/u, `,\n  ${members.join(",\n  ")}\n}`);
  console.log(`#### ${entry.name}\n<<<<\n${old}\n====\n${next}\n>>>>`);
}
