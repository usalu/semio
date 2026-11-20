/** 🏗️ Writes the authored volumes of `humans/parent-carrying-toddler` into the source `volumes/humans/parent-carrying-toddler.json`; `--only 50` keeps the other scales of the source as they are.
 *
 * Usage: `bun r3-parent-carrying-toddler-build.ts [--only 50]`
 * @see ./r3-parent-carrying-toddler-s50.ts
 * @see ./r3-parent-carrying-toddler-coarse.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { type Json } from "./volumes-lib.ts";
import { SOURCE } from "./r3-parent-carrying-toddler-io.ts";
import { DECISIONS_50, body50 } from "./r3-parent-carrying-toddler-s50.ts";

/** 📦 The three lists and the decisions of the 1:50 model. */
export async function volume50(): Promise<Json> {
  const b50 = body50();
  const lists = existsSync(new URL("./r3-parent-carrying-toddler-lists50.ts", import.meta.url)) ? await import("./r3-parent-carrying-toddler-lists50.ts") : null;
  return { status: "authored", denominator: 50, body: b50, thermal: lists ? lists.thermal50(b50) : [{ id: "th-coat", of: "coat", region: "coat-torso" }], structure: lists ? lists.structure50(b50) : [{ id: "st-coat", of: "coat", member: "torso", part: "member" }], decisions: DECISIONS_50 };
}
/** 📚 Every authored scale. */
export async function volumes(): Promise<Json> {
  const coarse = await import("./r3-parent-carrying-toddler-coarse.ts");
  const all: Json = { "50": await volume50(), ...coarse.coarseVolumes() };
  const texts = JSON.parse(readFileSync(new URL("./r3-parent-carrying-toddler-texts.json", import.meta.url), "utf8")).volumes as Json;
  for (const scale of Object.keys(all)) all[scale].decisions = texts[scale];
  return all;
}
if (import.meta.main) {
  const source = JSON.parse(readFileSync(SOURCE, "utf8")) as Json;
  if (process.argv.includes("--only")) source.volumes["50"] = await volume50();
  else source.volumes = await volumes();
  writeFileSync(SOURCE, `${JSON.stringify(source)}\n`);
  for (const scale of Object.keys(source.volumes)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${source.volumes[scale][l]?.length}`).join(", "));
}
