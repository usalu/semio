import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const owner = "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️composition";
const fixture = JSON.parse(readFileSync(resolve(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
let bad = 0;
for (const s of fixture.suites) {
  const src = s.tree === true ? readModuleTreeSource(resolve(owner, s.path)) : readFileSync(resolve(owner, s.path), "utf8");
  const sha = createHash("sha256").update(src).digest("hex");
  if (sha !== s.sha256) { bad++; console.log("STALE", s.path, "tree=" + !!s.tree, s.sha256.slice(0, 8), "->", sha.slice(0, 8)); }
}
console.log("stale suites:", bad, "of", fixture.suites.length);
let badUnits = 0;
for (const u of fixture.retainedArtifactUnits) {
  const sha = createHash("sha256").update(readFileSync(resolve(owner, "../../..", u.path), "utf8")).digest("hex");
  if (sha !== u.sha256) { badUnits++; console.log("STALE-UNIT", u.path, u.sha256.slice(0, 8), "->", sha.slice(0, 8)); }
}
console.log("stale retained units:", badUnits, "of", fixture.retainedArtifactUnits.length);
