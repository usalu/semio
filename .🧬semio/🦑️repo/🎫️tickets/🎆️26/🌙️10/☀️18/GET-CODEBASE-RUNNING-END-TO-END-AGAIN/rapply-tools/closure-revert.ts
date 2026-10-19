import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { readRelativeClosureV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🔗️relative-closure/🟦️.ts";
const ROOT = resolve(import.meta.dir, "../../../../../../../..");
const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad";
const changed = new Set(readFileSync(S + "/changed-files.txt", "utf8").split("\n").filter(Boolean));
const entry = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts";
const write = process.argv[2] === "revert";
for (let round = 1; round <= 5; round++) {
  const closure = readRelativeClosureV1(ROOT, entry);
  const hit = closure.files.filter(f => changed.has(f) && readFileSync(resolve(ROOT, f), "utf8") !== readFileSync(resolve(S, "before", f), "utf8"));
  console.log(JSON.stringify({ round, closureFiles: closure.files.length, workspaceSpecifiers: closure.workspaceSpecifiers.length, changedInClosure: hit.length }));
  if (!write || !hit.length) break;
  for (const f of hit) writeFileSync(resolve(ROOT, f), readFileSync(resolve(S, "before", f)));
}
