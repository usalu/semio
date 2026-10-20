import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { loadTargets, ROOT } from "./targets.ts";
import { scanRust } from "./rustscan.ts";
const GATE = /env::var|std::env|var_os|if write|if bless|BLESS|bless|regenerate|update_fixtures|UPDATE|SEMIO_\w*WRITE|fn (write|bless|regenerate)\w*|cfg\(/i;
const dirs = new Set<string>();
for (const t of loadTargets()) { if (!t.def || !t.root.includes("🦀️rust")) continue; dirs.add(resolve(ROOT, t.cwd, "../..").replace(ROOT + "/", "")); }
const r = scanRust([...dirs]);
let gated = 0, ungated = 0;
for (const [d, hits] of r) for (const h of hits) {
  const lines = readFileSync(join(ROOT, h.file), "utf8").split("\n");
  const ctx = lines.slice(Math.max(0, h.line - 12), h.line).join("\n");
  if (GATE.test(ctx)) gated++; else { ungated++; console.log("UNGATED", h.file.slice(-90), h.line, h.text.slice(0, 120)); }
}
console.log({ gated, ungated });
