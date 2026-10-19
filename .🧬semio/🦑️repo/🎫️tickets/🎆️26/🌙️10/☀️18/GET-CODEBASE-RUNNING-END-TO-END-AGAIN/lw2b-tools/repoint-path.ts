import { readFileSync, writeFileSync } from "node:fs";
const OLD = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f";
const NEW = "🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f";
let changed = 0;
for (const file of process.argv.slice(2)) {
  const text = readFileSync(file, "utf8");
  const next = text.split(OLD).join(NEW);
  if (next !== text) { writeFileSync(file, next); changed += (text.split(OLD).length - 1); console.log(`${text.split(OLD).length - 1}\t${file}`); }
}
console.log("total", changed);
