/** 🏗️ Writes the authored volumes of `humans/elderly-with-cane` (all scales, all lists) into the source `volumes/humans/elderly-with-cane.json`.
 *
 * Usage: `bun r3-elderly-with-cane-build.ts`
 * @see ./r3-elderly-with-cane-s50.ts
 * @see ./r3-elderly-with-cane-coarse.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { sourcePath, type Json } from "./volumes-lib.ts";
import { ENTRY } from "./r3-elderly-with-cane-kit.ts";
import { DECISIONS_50, body50, structure50, thermal50 } from "./r3-elderly-with-cane-s50.ts";
import { DECISIONS, body100, body200, body500, structure100, structure200, structure500, thermal100, thermal200, thermal500 } from "./r3-elderly-with-cane-coarse.ts";

/** 📦 The three lists and the decisions of every scale. */
export function volumes(): Json {
  const b50 = body50();
  const b100 = body100();
  const b200 = body200();
  const b500 = body500();
  return {
    "50": { status: "authored", denominator: 50, body: b50, thermal: thermal50(b50), structure: structure50(b50), decisions: DECISIONS_50 },
    "100": { status: "authored", denominator: 100, body: b100, thermal: thermal100(b100), structure: structure100(b100), decisions: DECISIONS[100] },
    "200": { status: "authored", denominator: 200, body: b200, thermal: thermal200(b200), structure: structure200(b200), decisions: DECISIONS[200] },
    "500": { status: "authored", denominator: 500, body: b500, thermal: thermal500(b500), structure: structure500(b500), decisions: DECISIONS[500] },
  };
}
if (import.meta.main) {
  const path = sourcePath(ENTRY);
  const source = JSON.parse(readFileSync(path, "utf8")) as Json;
  source.volumes = volumes();
  writeFileSync(path, `${JSON.stringify(source)}\n`);
  for (const scale of Object.keys(source.volumes)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${source.volumes[scale][l]?.length}`).join(", "));
}
