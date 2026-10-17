// ✅️ Verifies that each "OWNER FEATURE" line names a feature declared in the owner's [features] table (or an optional dependency).
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
const ROOT = "/Users/ueli/Documents/semio/";
const pairs = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map(l => l.split(" ")).map(p => ({owner: p[1]!, feature: p[2]!}));
const names = [...new Set(pairs.map(p => p.owner))];
const found = execFileSync("bun", [import.meta.dir + "/ctc-find.ts", ...names], {encoding: "utf8"}).split("\n").filter(Boolean).map(l => l.split("\t") as [string, string]);
const manifest = new Map(found);
let bad = 0;
for (const p of pairs) {
  const m = manifest.get(p.owner);
  if (!m) { console.log("NOMANIFEST", p.owner); bad++; continue; }
  const t = Bun.TOML.parse(readFileSync(ROOT + m, "utf8")) as any;
  const ok = t.features && p.feature in t.features;
  if (!ok) { console.log("MISSING FEATURE", p.owner, p.feature, "in", m); bad++; }
}
console.log(bad === 0 ? "all features declared" : `${bad} problems`);
