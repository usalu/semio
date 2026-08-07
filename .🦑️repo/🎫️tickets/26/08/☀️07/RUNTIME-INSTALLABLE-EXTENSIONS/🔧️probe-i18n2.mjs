import fs from "fs";
import path from "path";
function walk(dir, acc = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, acc);
    else if (/\.(ts|tsx)$/.test(e.name)) acc.push(p);
  }
  return acc;
}
for (const root of ["🧰️framework/🔨️modules/🖱️ui", "🧰️framework/🛍️products/💻️os"]) {
  for (const p of walk(root)) {
    const t = fs.readFileSync(p, "utf8");
    if (/UiTranslationKey|ui\.plugins|plugins\.action/.test(t) && /type UiTranslationKey|"ui\.panel\.document"/.test(t)) {
      console.log(p);
    }
  }
}
