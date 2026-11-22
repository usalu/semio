/** 🎻️ Publishes one used-instrument candidate of this ticket into the staffage catalog: guards it with the fast validator, copies the document to the catalog folder of its category (objects or appliances) and inserts or replaces its manifest row and its membership of the collection `used-instruments`, serialised by the lock folder shared with the sibling used-component tickets so that parallel authors of all waves never interleave their manifest writes.
 *
 * `$BUN publish.ts <slug>` reads `assets/<slug>/🔣️.json` and `assets/<slug>/distinction.json` (`{ "en", "de" }`) next to this file.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts — `validateManifest`, the rules the row must satisfy */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";

const HERE = import.meta.dir;
let held = false;
const ROOT = resolve(HERE, "../../../../../../..");
const CATALOG = join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const TOOLS = resolve(HERE, "../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE");
const COLLECTION = {
  id: "used-instruments",
  title: { en: "Used Instruments", de: "Gebrauchte Instrumente" },
  description: {
    en: "Second-hand musical instruments and music equipment as a used-instrument dealer, a music school depot or a rehearsal room stores and offers them: strings, keyboards, woodwinds, brass, percussion, folk instruments, amplification and stage gear, each with its traces of playing, its case or stand, how it is carried and how it is played again.",
    de: "Gebrauchte Musikinstrumente und Musiktechnik, wie ein Gebrauchtinstrumentenhandel, das Lager einer Musikschule oder ein Proberaum sie lagert und anbietet: Saiten-, Tasten-, Holzblas-, Blechblas- und Schlaginstrumente, Volksinstrumente, Verstärker und Bühnentechnik, jeweils mit Spielspuren, Koffer oder Ständer, Tragen und erneutem Spiel.",
  },
};

/** 💥️ Ends the run with a message on stderr. */
function fail(message: string): never {
  console.error(message);
  if (held) rmdirSync(join(HERE, "../RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/publish.lock"));
  process.exit(1);
}

/** 🔒️ Runs `work` while holding the lock folder, waiting up to two minutes for another publisher. */
function locked(work: () => void): void {
  const lock = join(HERE, "../RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/publish.lock");
  for (let attempt = 0; ; attempt++) {
    try {
      mkdirSync(lock);
      held = true;
      break;
    } catch {
      if (attempt > 600) fail(`publish.lock is held for more than two minutes: ${lock}`);
      Bun.sleepSync(200);
    }
  }
  try {
    work();
  } finally {
    held = false;
    rmdirSync(lock);
  }
}

const slug = process.argv[2] ?? fail("usage: publish.ts <slug>");
const candidate = join(HERE, "assets", slug, "🔣️.json");
const distinctionFile = join(HERE, "assets", slug, "distinction.json");
if (!existsSync(candidate)) fail(`no candidate: ${candidate}`);
if (!existsSync(distinctionFile)) fail(`no distinction: ${distinctionFile}`);
const bytes = readFileSync(candidate, "utf8");
const asset = JSON.parse(bytes).asset;
const distinction = JSON.parse(readFileSync(distinctionFile, "utf8"));
if (asset.id !== slug) fail(`asset.id "${asset.id}" is not the slug "${slug}"`);
if (!["objects", "appliances"].includes(asset.category)) fail(`asset.category must be "objects" or "appliances"`);
for (const tag of ["used", "instrument", "music", "second-hand"]) if (!asset.tags.includes(tag)) fail(`asset.tags must include "${tag}"`);
if (typeof distinction.en !== "string" || typeof distinction.de !== "string" || distinction.en.trim() === "" || distinction.de.trim() === "") fail("distinction.json needs non-empty en and de");

const guard = spawnSync(process.execPath, ["r3-validate.ts", "--file", candidate, "--no-print"], { cwd: TOOLS, encoding: "utf8" });
const verdict = guard.stdout.trim().split("\n").at(-1) ?? "";
if (guard.status !== 0 || !verdict.startsWith("VALID")) fail(`candidate is not valid, nothing published:\n${guard.stdout}${guard.stderr}`);

const analytics = [...(asset.thermal.regions.length > 0 ? ["thermal"] : []), ...(asset.thermal.emitters.length > 0 ? ["transport"] : []), "structural"];
const entry = {
  id: asset.id,
  category: asset.category,
  title: asset.title,
  heightM: asset.dimensions.heightM,
  widthM: asset.dimensions.widthM,
  powerW: Math.round(totalPowerW(asset) * 10) / 10,
  poses: asset.poses.filter((pose: { key?: boolean }) => !pose.key).map((pose: { id: string }) => pose.id),
  clips: asset.clips.map((clip: { id: string }) => clip.id),
  representations: Object.fromEntries(["50", "100", "200", "500"].map((scale) => [scale, asset.representations[scale].status])),
  symbols: asset.symbols.map((symbol: { id: string }) => symbol.id),
  analytics,
  exports: asset.exports,
  distinction: { en: distinction.en, de: distinction.de },
  completion: asset.completion,
};

locked(() => {
  const manifestFile = join(CATALOG, "🔣️.json");
  const before = readFileSync(manifestFile, "utf8");
  const manifest = JSON.parse(before);
  const folder = manifest.categories.find((row: { id: string }) => row.id === asset.category)?.folder ?? fail(`category "${asset.category}" not in the manifest`);
  const target = join(CATALOG, folder, slug, "🔣️.json");
  const replaced = manifest.entries.some((row: { id: string }) => row.id === slug);
  const collection = manifest.collections.find((row: { id: string }) => row.id === COLLECTION.id) ?? { ...COLLECTION, assets: [] };
  const hadCollection = manifest.collections.some((row: { id: string }) => row.id === COLLECTION.id);
  if (!collection.assets.includes(slug)) collection.assets.push(slug);
  let text = replaced ? swap(before, slug, block(entry)) : after(before, before.lastIndexOf(`"category": "${asset.category}"`), block(entry));
  text = hadCollection ? swap(text, COLLECTION.id, block(collection)) : after(text, text.lastIndexOf("\n    }", text.indexOf('\n  "entries"')) - 1, block(collection));
  const written = JSON.parse(text);
  const row = written.entries.filter((candidate: { id: string }) => candidate.id === slug);
  const members = written.collections.filter((candidate: { id: string }) => candidate.id === COLLECTION.id);
  if (row.length !== 1 || JSON.stringify(row[0]) !== JSON.stringify(entry) || members.length !== 1 || !members[0].assets.includes(slug) || written.entries.length !== manifest.entries.length + (replaced ? 0 : 1) || written.collections.length !== manifest.collections.length + (hadCollection ? 0 : 1)) fail("manifest edit did not produce the intended document, nothing written to the manifest");
  if (process.argv.includes("--dry")) return console.log(`dry run: ${slug} would be published, manifest row ${replaced ? "replaced" : "inserted"}`);
  mkdirSync(dirname(target), { recursive: true });
  writeFileSync(target, bytes);
  writeFileSync(manifestFile, text);
  console.log(`published ${slug}: ${target}`);
  console.log(`manifest row ${replaced ? "replaced" : "inserted"}; collection ${COLLECTION.id} has ${collection.assets.length} assets; powerW ${entry.powerW}`);
});

/** 🧱️ One manifest array item as text at the indentation of the manifest's rows. */
function block(value: unknown): string {
  return JSON.stringify(value, null, 2).split("\n").map((line) => `    ${line}`).join("\n");
}

/** ➕️ Inserts a row after the row that contains the text position `inside`. */
function after(text: string, inside: number, row: string): string {
  const end = text.indexOf("\n    }", inside);
  if (inside < 0 || end < 0) fail("manifest layout not recognised, nothing written to the manifest");
  return `${text.slice(0, end + 6)},\n${row}${text.slice(end + 6)}`;
}

/** 🔁️ Replaces the row whose first member is the id `id`. */
function swap(text: string, id: string, row: string): string {
  const start = text.indexOf(`\n    {\n      "id": "${id}",`);
  const end = text.indexOf("\n    }", start + 1);
  if (start < 0 || end < 0) fail(`manifest row "${id}" not found in the expected layout, nothing written to the manifest`);
  return `${text.slice(0, start + 1)}${row}${text.slice(end + 6)}`;
}
