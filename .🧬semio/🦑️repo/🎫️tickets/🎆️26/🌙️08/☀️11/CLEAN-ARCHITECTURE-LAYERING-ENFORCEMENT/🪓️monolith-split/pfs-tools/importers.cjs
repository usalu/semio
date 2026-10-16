// usage: node importers.cjs <target-rel> [...]   Lists files (from $G/pfs-allfiles.txt) with a relative string literal resolving to the target (with or without extension), or to its directory + basename
const fs = require("fs"), path = require("path");
const R = "/Users/ueli/Documents/semio";
const list = fs.readFileSync(R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/pfs-allfiles.txt", "utf8").split("\n").filter(Boolean);
const targets = process.argv.slice(2).map((t) => path.resolve(R, t));
const noext = (p) => p.replace(/\.(tsx?|rs)$/, "");
const re = /(["'`])((?:\.\.?\/)[^"'`\n]*?)\1/g;
const hits = new Map();
for (const rel of list) {
  const f = path.join(R, rel);
  let text; try { text = fs.readFileSync(f, "utf8"); } catch { continue; }
  if (text.length > 3_000_000) continue;
  const dir = path.dirname(f);
  let m; re.lastIndex = 0; const lines = [];
  while ((m = re.exec(text))) {
    const abs = path.resolve(dir, m[2].replace(/\?raw$/, ""));
    for (const t of targets) if (abs === t || abs === noext(t)) lines.push(text.slice(0, m.index).split("\n").length + ": " + m[2]);
  }
  if (lines.length) hits.set(rel, lines);
}
for (const [rel, ls] of hits) console.log(rel + "\n   " + [...new Set(ls)].join("\n   "));
console.error("importer files:", hits.size);
