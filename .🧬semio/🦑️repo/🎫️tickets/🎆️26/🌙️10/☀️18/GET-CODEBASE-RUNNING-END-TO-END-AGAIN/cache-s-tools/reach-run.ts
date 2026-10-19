import { readFileSync } from "node:fs";
import { manifests, ROOT, facts } from "./lib.ts";
import { reach } from "./reach.ts";
let n = 0;
for (const f of manifests()) {
  const p = facts(f, readFileSync(ROOT + "/" + f, "utf8"));
  if (!p.native) continue;
  const r = reach(p.root, "");
  n++;
  if (Bun.argv[2] === f.split("/").slice(0, -1).join("/")) console.log(JSON.stringify(r, null, 1));
  else if (!Bun.argv[2]) console.log(r.dirs.length, r.indirect ? "INDIRECT" : "", r.unresolved.length, p.root);
}
console.log(n);
