import fs from "node:fs";
import { stripDebugRs } from "./strip-debug-rs.mjs";
const R = "/Users/ueli/Documents/semio/";
const files = JSON.parse(fs.readFileSync("debug-appendix.json", "utf8"));
for (const [s, p] of files) {
  if (!/\.rs$/.test(p)) continue;
  const t = fs.readFileSync(R + p, "utf8");
  const notes = [];
  const o = stripDebugRs(t, p, notes);
  console.log(p.split("/").slice(-4).join("/"), t.split("[DEBUG]").length - 1, "->", o.split("[DEBUG]").length - 1, notes.join("; "));
}
