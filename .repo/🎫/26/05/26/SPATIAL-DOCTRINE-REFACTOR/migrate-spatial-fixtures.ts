/** Unwrap legacy raw/analytic envelopes to a single spatial.model/v1 document. */
import { readFileSync, writeFileSync } from "node:fs";

for (const file of ["c:/git/compose/spatial/fixtures/simple.spatial.json", "c:/git/compose/spatial/fixtures/spatial.spatial.json"]) {
  const parsed = JSON.parse(readFileSync(file, "utf8")) as Record<string, unknown>;
  const model = (parsed.raw ?? parsed.model ?? parsed) as unknown;
  writeFileSync(file, `${JSON.stringify(model, null, 2)}\n`);
  console.log("[DEBUG] migrated", file);
}
