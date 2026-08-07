import fs from "fs";
import path from "path";

function walk(dir, acc = []) {
  let entries;
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true });
  } catch {
    return acc;
  }
  for (const e of entries) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, acc);
    else if (/\.(ts|tsx)$/.test(e.name) && /I18n|i18n/.test(p)) acc.push(p);
  }
  return acc;
}

const files = walk("🧰️framework/🔨️modules/🖱️ui");
console.log(files.join("\n"));
for (const p of files) {
  const t = fs.readFileSync(p, "utf8");
  if (!t.includes("UiTranslationKey")) continue;
  console.log("\n====", p, "====");
  const m = t.match(/export type UiTranslationKey[\s\S]{0,400}/);
  console.log(m && m[0]);
  console.log("has ui.plugins", t.includes("ui.plugins"));
  const msgs = t.match(/ui\.plugins[\w.]*/g);
  console.log(msgs);
}
