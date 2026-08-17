import { readdirSync, readFileSync, statSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins";
function walk(d, acc = []) {
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    if (statSync(p).isDirectory()) walk(p, acc);
    else if (n.endsWith("component.grammar.semio") || n.endsWith("component.protocol.semio")) acc.push(p);
  }
  return acc;
}
const pat = /pixel |shape |tile |blocks |frames |fence|shot |source \{|paragraph|paint-layer/;
for (const p of walk(ROOT)) {
  const t = readFileSync(p, "utf8");
  if (pat.test(t) && !t.includes('layer = IDENT "@"')) console.log(p);
}

const plan = readFileSync("/Users/ueli/Documents/semio/.cursor/plans/per-artifact_grammars_and_protocols_8b0fe9ad.plan.md", "utf8");
plan.split("\n").forEach((line, i) => {
  const l = line.toLowerCase();
  if (["w4", "scene", "embed", "artifact-specific", "refine", "family-scene", "handcraft"].some((k) => l.includes(k))) {
    console.log(`${i + 1}:${line.slice(0, 180)}`);
  }
});
