import { readFileSync } from "node:fs";
import { manifests, ROOT, facts } from "./lib.ts";
import { extras } from "./extras.ts";
const want = Bun.argv[2];
let n = 0, total = 0;
for (const f of manifests()) {
  const p = facts(f, readFileSync(ROOT + "/" + f, "utf8"));
  const i = p.root.indexOf("/📦️packages/");
  const owner = i >= 0 ? p.root.slice(0, i) : p.root;
  const e = extras(p.root, owner, p.native ? "native" : "script");
  if (want && p.root === want) console.log(JSON.stringify(e, null, 1));
  if (e.globs.length) { n++; total += e.globs.length; }
}
console.log("projects with extras", n, "globs", total);
