import fs from "node:fs";
import path from "node:path";
const R = "/Users/ueli/Documents/semio/";
const SDK = "🧰️framework/📦️packages/🟦️typescript/";
const pkg = JSON.parse(fs.readFileSync(R + SDK + "package.json", "utf8")).exports;
const write = process.argv[2] === "--write";
const files = process.argv.slice(3);
for (const f of files) {
  const abs = R + f; const text = fs.readFileSync(abs, "utf8");
  let n = 0;
  const next = text.replace(/(from\s+")@semio-tech\/framework(\/[^"]*)?(")/g, (m, a, sub, c) => {
    const key = sub ? "." + sub : ".";
    const target = pkg[key]; if (!target) throw new Error(`no export ${key} for ${f}`);
    let rel = path.relative(path.dirname(abs), R + SDK + target.slice(2)).normalize("NFC");
    if (!rel.startsWith(".")) rel = "./" + rel;
    n++; return a + rel + c;
  });
  console.log(`${write ? "WRITE" : "DRY"} ${f.replace(/^.*?engine\//u, "E/")}: ${n} specifier(s)`);
  if (write && n) { if (fs.readFileSync(abs, "utf8") !== text) { console.log("CHANGED UNDER US", f); continue; } fs.writeFileSync(abs, next); }
}
