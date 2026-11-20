/** 🏗️ Writes the authored volumes of `humans/pregnant-person` into the source `volumes/humans/pregnant-person.json`: the 1:50 lists of `r3-pregnant-person-s50.ts` (body), `-th50.ts` (thermal) and `-st50.ts` (skeleton) and the coarser scales of `-coarse.ts`, each with the decisions of `r3-pregnant-person-texts.json`; `--only 50` keeps the other scales of the source as they are.
 *
 * Usage: `bun r3-pregnant-person-build.ts [--only 50]`
 * @see ./r3-pregnant-person-s50.ts */
import { fileURLToPath } from "node:url";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import type { Json } from "./volumes-lib.ts";
import { SOURCE } from "./r3-pregnant-person-io.ts";
import { body50 } from "./r3-pregnant-person-s50.ts";

const here = (f: string): string => fileURLToPath(new URL(f, import.meta.url));
/** 📦 The three lists of the 1:50 model (thermal and skeleton once their modules exist). */
export async function volume50(previous: Json): Promise<Json> {
  const body = body50();
  const thermal = existsSync(here("./r3-pregnant-person-th50.ts")) ? (await import("./r3-pregnant-person-th50.ts")).thermal50(body) : previous?.thermal ?? [];
  const structure = existsSync(here("./r3-pregnant-person-st50.ts")) ? (await import("./r3-pregnant-person-st50.ts")).structure50() : previous?.structure ?? [];
  return { status: "authored", denominator: 50, body, thermal, structure, decisions: [] };
}
/** 📚 Every authored scale, with the decisions of `r3-pregnant-person-texts.json`. */
export async function volumes(base: Json, only50: boolean): Promise<Json> {
  const all: Json = { ...base, "50": await volume50(base["50"]) };
  if (!only50 && existsSync(here("./r3-pregnant-person-coarse.ts"))) Object.assign(all, (await import("./r3-pregnant-person-coarse.ts")).coarseVolumes());
  const texts = here("./r3-pregnant-person-texts.json");
  if (existsSync(texts)) for (const [scale, d] of Object.entries(JSON.parse(readFileSync(texts, "utf8")).volumes ?? {})) if (all[scale]) all[scale].decisions = d;
  for (const v of Object.values(all) as Json[]) if (v.status === "authored" && v.decisions.length === 0) v.decisions = [{ en: "Work in progress.", de: "In Arbeit." }];
  return all;
}
if (import.meta.main) {
  const source = JSON.parse(readFileSync(SOURCE, "utf8")) as Json;
  source.volumes = await volumes(source.volumes, process.argv.includes("--only"));
  writeFileSync(SOURCE, `${JSON.stringify(source)}\n`);
  for (const scale of Object.keys(source.volumes)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${source.volumes[scale][l]?.length}`).join(", "));
}
