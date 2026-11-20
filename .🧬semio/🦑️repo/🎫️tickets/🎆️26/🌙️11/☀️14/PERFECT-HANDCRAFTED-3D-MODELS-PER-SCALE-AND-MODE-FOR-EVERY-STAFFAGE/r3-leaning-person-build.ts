/** 🏗️ Writes the authored volumes of `humans/leaning-person` into the source `volumes/humans/leaning-person.json`; `--only 50` keeps the other scales of the source as they are.
 *
 * Usage: `bun r3-leaning-person-build.ts [--only 50]`
 * @see ./r3-leaning-person-s50.ts
 * @see ./r3-leaning-person-coarse.ts */
import { fileURLToPath } from "node:url";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import type { Json } from "./volumes-lib.ts";
import { ASSET } from "./r3-leaning-person-kit.ts";
import { SOURCE } from "./r3-leaning-person-io.ts";
import { body50, structure50, thermal50 } from "./r3-leaning-person-s50.ts";

/** ⚖️ The centres of mass of the heavier segments (bone-local x, y) with the radius of their mass solid. */
export function masses(radius: (kg: number) => number): Record<string, [number, number, number]> {
  const out: Record<string, [number, number, number]> = {};
  for (const b of ASSET.rig.bones) if (b.mass && b.mass.kg >= 2.5) out[b.id] = [b.mass.comX, b.mass.comY ?? 0, radius(b.mass.kg)];
  return out;
}
/** 📦 The three lists of the 1:50 model. */
export function volume50(): Json {
  const b = body50();
  return { status: "authored", denominator: 50, body: b, thermal: thermal50(b), structure: structure50(masses((kg) => Math.min(0.03, 0.012 + 0.0009 * kg))), decisions: [] };
}
/** 📚 Every authored scale, with the decisions of `r3-leaning-person-texts.json`. */
export async function volumes(base: Json): Promise<Json> {
  const all: Json = { ...base, "50": volume50() };
  if (existsSync(fileURLToPath(new URL("./r3-leaning-person-coarse.ts", import.meta.url)))) Object.assign(all, (await import("./r3-leaning-person-coarse.ts")).coarseVolumes());
  const texts = fileURLToPath(new URL("./r3-leaning-person-texts.json", import.meta.url));
  if (existsSync(texts)) for (const [scale, d] of Object.entries(JSON.parse(readFileSync(texts, "utf8")).volumes ?? {})) if (all[scale]) all[scale].decisions = d;
  for (const v of Object.values(all) as Json[]) if (v.status === "authored" && v.decisions.length === 0) v.decisions = [{ en: "Work in progress.", de: "In Arbeit." }];
  return all;
}
if (import.meta.main) {
  const source = JSON.parse(readFileSync(SOURCE, "utf8")) as Json;
  source.volumes = process.argv.includes("--only") ? { ...source.volumes, "50": (await volumes(source.volumes))["50"] } : await volumes(source.volumes);
  writeFileSync(SOURCE, `${JSON.stringify(source)}\n`);
  for (const scale of Object.keys(source.volumes)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${source.volumes[scale][l]?.length}`).join(", "));
}
