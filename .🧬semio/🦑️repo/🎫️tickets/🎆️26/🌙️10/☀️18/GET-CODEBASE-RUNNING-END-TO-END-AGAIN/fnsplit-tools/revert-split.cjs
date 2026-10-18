// usage: node revert-split.cjs <spec.json> <original.ts>: removes the child files (and now-empty dirs) a splitter wrote from a spec and restores the root from the snapshot (own artifacts only)
const fs = require("fs"), path = require("path");
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const root = spec.childRoot || path.dirname(spec.file);
const dirs = new Set();
const add = (d) => d && dirs.add(d);
add(spec.kit?.dir); add(spec.helpers?.dir);
for (const g of spec.groups || spec.files || []) { add(g.dir); add(g.helpers?.dir); for (const p of g.parts || []) add(p.dir); }
for (const d of dirs) { const f = path.join(root, d, "🟦️.ts"); if (fs.existsSync(f)) fs.unlinkSync(f); const dd = path.join(root, d); if (fs.existsSync(dd) && fs.readdirSync(dd).length === 0) fs.rmdirSync(dd); else if (fs.existsSync(dd)) console.log("kept non-empty", dd); }
fs.copyFileSync(process.argv[3], spec.file);
console.log("reverted", dirs.size, "dirs");
