import fs from "node:fs";
import { stripDebugTs } from "./strip-debug-ts.mjs";
const R = "/Users/ueli/Documents/semio/";
const files = JSON.parse(fs.readFileSync("debug-appendix.json", "utf8"));
for (const [s, p] of files) {
  if (!/\.(ts|tsx)$/.test(p)) continue;
  const t = fs.readFileSync(R + p, "utf8");
  const o = stripDebugTs(t, p, []);
  // print changed char windows
  let i = 0; while (i < t.length && t[i] === o[i]) i++;
  let j = 0; while (j < t.length - i && j < o.length - i && t[t.length - 1 - j] === o[o.length - 1 - j]) j++;
  console.log("--", p.split("/").slice(-3).join("/"));
  console.log("  -:", JSON.stringify(t.slice(Math.max(0,i-40), t.length - j + 20).slice(0, 700)));
  console.log("  +:", JSON.stringify(o.slice(Math.max(0,i-40), o.length - j + 20).slice(0, 300)));
}
