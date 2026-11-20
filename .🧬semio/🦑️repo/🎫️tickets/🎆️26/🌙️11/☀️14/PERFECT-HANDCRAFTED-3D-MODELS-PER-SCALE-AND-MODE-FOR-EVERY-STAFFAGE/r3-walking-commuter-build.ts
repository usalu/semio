/** 🏗️ Writes the authored volumes of `humans/walking-commuter` into the source `volumes/humans/walking-commuter.json`; `--only 50` keeps the other scales of the source as they are.
 *
 * Usage: `bun r3-walking-commuter-build.ts [--only 50]`
 * @see ./r3-walking-commuter-s50.ts
 * @see ./r3-walking-commuter-coarse.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { sourcePath, type Json } from "./volumes-lib.ts";
import { ENTRY } from "./r3-walking-commuter-kit.ts";
import { DECISIONS_50, body50, structure50, thermal50 } from "./r3-walking-commuter-s50.ts";

/** 📦 The three lists and the decisions of the 1:50 model. */
export function volume50(): Json {
  const b50 = body50();
  return { status: "authored", denominator: 50, body: b50, thermal: thermal50(b50), structure: structure50(b50), decisions: DECISIONS_50 };
}
/** 📚 Every authored scale. */
export async function volumes(): Promise<Json> {
  const coarse = await import("./r3-walking-commuter-coarse.ts");
  const all: Json = { "50": volume50(), ...coarse.coarseVolumes() };
  const texts = JSON.parse(readFileSync(new URL("./r3-walking-commuter-texts.json", import.meta.url), "utf8")).volumes as Json;
  for (const scale of Object.keys(all)) all[scale].decisions = texts[scale];
  return all;
}
if (import.meta.main) {
  const path = sourcePath(ENTRY);
  const source = JSON.parse(readFileSync(path, "utf8")) as Json;
  if (process.argv.includes("--only")) source.volumes["50"] = volume50();
  else source.volumes = await volumes();
  writeFileSync(path, `${JSON.stringify(source)}\n`);
  for (const scale of Object.keys(source.volumes)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${source.volumes[scale][l]?.length}`).join(", "));
}
