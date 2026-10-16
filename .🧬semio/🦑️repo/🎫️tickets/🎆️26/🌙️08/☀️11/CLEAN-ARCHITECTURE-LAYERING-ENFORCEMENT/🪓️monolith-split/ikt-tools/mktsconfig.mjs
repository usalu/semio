import fs from "node:fs";
const [out, ...lists] = process.argv.slice(2);
const R = "/Users/ueli/Documents/semio";
const files = new Set();
for (const l of lists) {
  if (l.startsWith("+")) { files.add(l.slice(1)); continue; }
  for (const line of fs.readFileSync(l, "utf8").split("\n")) {
    const f = line.split("\t")[0].replace(/:\d+$/, "");
    if (f && /\.(tsx?|mts)$/.test(f)) files.add(f.startsWith("/") ? f : `${R}/${f}`);
  }
}
const cfg = { extends: `${R}/🧰️framework/🛍️products/💻️os/tsconfig.json`, compilerOptions: { noEmit: true, incremental: false, tsBuildInfoFile: null }, include: [], files: [...files].filter((f) => fs.existsSync(f)).sort() };
fs.writeFileSync(out, JSON.stringify(cfg, null, 1));
console.log("files", cfg.files.length);
