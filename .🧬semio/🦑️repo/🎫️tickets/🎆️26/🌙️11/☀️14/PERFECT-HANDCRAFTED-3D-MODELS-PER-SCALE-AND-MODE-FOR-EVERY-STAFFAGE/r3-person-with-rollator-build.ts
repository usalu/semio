/** 🏗️ Writes the authored volumes of `humans/person-with-rollator` into the source `volumes/humans/person-with-rollator.json`: every scale whose lists are stated (`--only 50` writes that scale alone and leaves the others as they are).
 *
 * Usage: `bun r3-person-with-rollator-build.ts [--only N]`
 * @see ./r3-person-with-rollator-s50.ts
 * @see ./r3-person-with-rollator-coarse.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { sourcePath, type Json } from "./volumes-lib.ts";
import { ENTRY } from "./r3-person-with-rollator-kit.ts";
import { body50, structure50, thermal50 } from "./r3-person-with-rollator-s50.ts";

const texts = (): Json => {
  const path = join(import.meta.dir, "r3-person-with-rollator-texts.json");
  return existsSync(path) ? (JSON.parse(readFileSync(path, "utf8")) as Json) : {};
};
/** 📦 The three lists and the decisions of every scale. */
export async function volumes(only?: string): Promise<Json> {
  const decisions = texts().volumeDecisions ?? {};
  const out: Json = {};
  if (only === undefined || only === "50") {
    const b = body50();
    out["50"] = { status: "authored", denominator: 50, body: b, thermal: thermal50(b), structure: structure50(b), decisions: decisions["50"] ?? [{ en: "In work.", de: "In Arbeit." }] };
  }
  if (only === undefined || only !== "50") {
    const coarse = await import("./r3-person-with-rollator-coarse.ts");
    for (const scale of [100, 200, 500] as const) {
      if (only !== undefined && only !== String(scale)) continue;
      const b = coarse.body(scale);
      out[String(scale)] = { status: "authored", denominator: scale, body: b, thermal: coarse.thermal(scale, b), structure: coarse.structure(scale, b), decisions: decisions[String(scale)] ?? [{ en: "In work.", de: "In Arbeit." }] };
    }
  }
  return out;
}
if (import.meta.main) {
  const only = process.argv.includes("--only") ? process.argv[process.argv.indexOf("--only") + 1] : undefined;
  const path = sourcePath(ENTRY);
  const source = JSON.parse(readFileSync(path, "utf8")) as Json;
  Object.assign(source.volumes, await volumes(only));
  writeFileSync(path, `${JSON.stringify(source)}\n`);
  for (const scale of Object.keys(source.volumes)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${source.volumes[scale][l]?.length}`).join(", "));
}
