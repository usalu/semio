import fs from "node:fs";
import { scan } from "./scan-comments.mjs";
const R = "/Users/ueli/Documents/semio/";
const files = JSON.parse(fs.readFileSync("cz-gs-files.json", "utf8"));
for (const f of files) {
  if (!/\.(ts|tsx|mjs|mts)$/.test(f)) continue;
  const t = fs.readFileSync(R + f, "utf8");
  const hits = scan(t, f);
  const keep = hits.filter((h) => h.directive);
  const hs = hits.filter((h) => !h.directive);
  if (!hs.length && !keep.length) continue;
  console.log("== " + f.split("/").slice(-3).join("/"), "directives kept:", keep.length);
  for (const h of hs) console.log(h.line + ": " + h.body.replace(/\n/g, "\\n"));
}
