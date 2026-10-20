/** 🔬 Maps every census hit to the needle that matched it. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED } from "./lib.ts";
const needles = readFileSync(resolve(GENERATED, "needles.txt"), "utf8").split("\n").filter(Boolean);
const hits = readFileSync(resolve(GENERATED, "ref-hits.txt"), "utf8").split("\n").filter(Boolean);
for (const h of hits) { const ns = needles.filter(n => h.includes(n)).sort((a, b) => b.length - a.length); console.log(h.replace(/^[^:]*library\//u, "LIB/").replace(/^.*typescript\/📋️project/u, "WGPU/project").slice(0, 150).replace(/:\s+/u, ": ") , "\n    <= ", ns[0]); }
