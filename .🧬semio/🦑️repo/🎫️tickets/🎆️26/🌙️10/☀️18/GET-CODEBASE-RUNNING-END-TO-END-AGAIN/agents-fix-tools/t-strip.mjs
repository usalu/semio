import fs from "node:fs";
import { stripDebugTs } from "./strip-debug-ts.mjs";
const R = "/Users/ueli/Documents/semio/";
const files = JSON.parse(fs.readFileSync("debug-appendix.json", "utf8"));
for (const [s, p] of files) {
  if (!/\.(ts|tsx)$/.test(p)) continue;
  const t = fs.readFileSync(R + p, "utf8");
  const notes = [];
  const o = stripDebugTs(t, p, notes);
  console.log(p.split("/").slice(-3).join("/"), "removed", t.split("[DEBUG]").length - 1, "->", o.split("[DEBUG]").length - 1, "len", t.length, o.length, notes.join("; "));
}
