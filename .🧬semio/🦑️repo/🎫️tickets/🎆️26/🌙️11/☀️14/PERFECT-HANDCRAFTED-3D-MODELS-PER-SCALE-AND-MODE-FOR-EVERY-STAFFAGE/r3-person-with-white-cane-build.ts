/** 🏗️ Writes the authored volumes of `humans/person-with-white-cane` into the source `volumes/humans/person-with-white-cane.json` (`--only 50` writes that scale alone and keeps the others).
 *
 * Usage: `bun r3-person-with-white-cane-build.ts [--only N]`
 * @see ./r3-person-with-white-cane-s50.ts
 * @see ./r3-person-with-white-cane-coarse.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { sourcePath, type Json } from "./volumes-lib.ts";
import { ENTRY } from "./r3-person-with-white-cane-kit.ts";
import { DECISIONS_50, body50, structure50, thermal50 } from "./r3-person-with-white-cane-s50.ts";

/** 📦 The three lists and the decisions of every scale. */
export async function volumes(): Promise<Json> {
  const b50 = body50();
  const out: Json = { "50": { status: "authored", denominator: 50, body: b50, thermal: thermal50(b50), structure: structure50(b50), decisions: DECISIONS_50 } };
  if (existsSync(join(import.meta.dir, "r3-person-with-white-cane-coarse.ts"))) Object.assign(out, (await import("./r3-person-with-white-cane-coarse.ts")).coarseVolumes());
  const texts = join(import.meta.dir, "r3-person-with-white-cane-texts.json");
  if (existsSync(texts)) {
    const decisions = JSON.parse(readFileSync(texts, "utf8")).volumes as Json | undefined;
    for (const scale of Object.keys(out)) if (decisions?.[scale]) out[scale].decisions = decisions[scale];
  }
  return out;
}
if (import.meta.main) {
  const path = sourcePath(ENTRY);
  const source = JSON.parse(readFileSync(path, "utf8")) as Json;
  const made = await volumes();
  const only = process.argv.includes("--only") ? process.argv[process.argv.indexOf("--only") + 1] : undefined;
  for (const scale of Object.keys(made)) if (only === undefined || only === scale) source.volumes[scale] = made[scale];
  writeFileSync(path, `${JSON.stringify(source)}\n`);
  for (const scale of Object.keys(source.volumes)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${source.volumes[scale][l]?.length}`).join(", "));
}
