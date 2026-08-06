import fs from "fs";
import path from "path";
function walk(dir, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", "dist", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isSymbolicLink()) continue;
    if (e.isDirectory()) walk(p, out);
    else if (/\.(ts|tsx)$/.test(e.name)) out.push(p);
  }
  return out;
}
for (const file of walk(".")) {
  const lines = fs.readFileSync(file, "utf8").split("\n");
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    if (!l.includes("ephemeral")) continue;
    if (/Set<[^>]*>>\(/.test(l) || /Map<[^>]*>>\(/.test(l) || /\| null = null;/.test(l) || /ephemeralBox<\(\(\)>/.test(l) || /", >/.test(l)) {
      console.log(file + ":" + (i + 1), l.trim().slice(0, 180));
    }
  }
}
