/** 🏗️ Writes the authored volumes of `humans/person-with-umbrella` into the source `volumes/humans/person-with-umbrella.json`: every scale that its file already states (`--only 50` writes that scale alone and keeps the others).
 *
 * Usage: `bun r3-person-with-umbrella-build.ts [--only N]`
 * @see ./r3-person-with-umbrella-s50.ts
 * @see ./r3-person-with-umbrella-coarse.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { sourcePath, type Json } from "./volumes-lib.ts";
import { ENTRY } from "./r3-person-with-umbrella-kit.ts";
import { DECISIONS_50, body50, structure50, thermal50 } from "./r3-person-with-umbrella-s50.ts";

/** 📦 The three lists and the decisions of every scale. */
export async function volumes(): Promise<Json> {
  const b50 = body50();
  const out: Json = { "50": { status: "authored", denominator: 50, body: b50, thermal: thermal50(b50), structure: structure50(b50), decisions: DECISIONS_50 } };
  if (existsSync(join(import.meta.dir, "r3-person-with-umbrella-coarse.ts"))) {
    const c = await import("./r3-person-with-umbrella-coarse.ts");
    for (const scale of [100, 200, 500] as const) {
      const body = c.body(scale);
      out[String(scale)] = { status: "authored", denominator: scale, body, thermal: c.thermal(scale, body), structure: c.structure(scale, body), decisions: c.DECISIONS[scale] };
    }
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
