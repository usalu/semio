import { readFileSync, writeFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { cliClosure, ROOT } from "../repoapi-tools/lib.ts";
const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad";
const changed = new Set(readFileSync(S + "/changed-files.txt", "utf8").split("\n").filter(Boolean));
const write = process.argv[2] === "revert";
for (let round = 1; round <= 8; round++) {
  const closure = [...cliClosure()].map(f => relative(ROOT, f));
  const hit = closure.filter(f => changed.has(f) && readFileSync(resolve(ROOT, f), "utf8") !== readFileSync(resolve(S, "before", f), "utf8"));
  console.log(JSON.stringify({ round, cliClosure: closure.length, changedInClosure: hit.length }));
  if (!write || !hit.length) break;
  for (const f of hit) writeFileSync(resolve(ROOT, f), readFileSync(resolve(S, "before", f)));
}
