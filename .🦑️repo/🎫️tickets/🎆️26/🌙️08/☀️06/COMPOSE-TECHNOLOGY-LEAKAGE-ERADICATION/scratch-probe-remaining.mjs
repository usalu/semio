import { readdirSync, readFileSync, existsSync, statSync } from "fs";
import { join } from "path";

function walkDirs(dir, depth = 0, match) {
  if (depth > 8) return [];
  const out = [];
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    if (!e.isDirectory()) continue;
    const p = join(dir, e.name);
    if (e.name === match) out.push(p);
    else out.push(...walkDirs(p, depth + 1, match));
  }
  return out;
}

const tickets = walkDirs(".🦑️repo/🎫️tickets", 0, "COMPOSE-TECHNOLOGY-LEAKAGE-ERADICATION");
console.log("ticket", tickets[0]);
const t = tickets[0];
console.log("files", readdirSync(t));
console.log("---inventory---");
console.log(readFileSync(join(t, "🧾inventory.md"), "utf8"));

const assets = join("�onnaisframework", "🔨️modules", "🖼️assets");
// resolve assets by listing repo root
const rootEntries = readdirSync(".");
const fw = rootEntries.find((n) => n.includes("framework"));
console.log("framework dir", fw);
const assetsDir = join(fw, [...readdirSync(join(fw))].find((n) => n.includes("modules")), [...readdirSync(join(fw, readdirSync(join(fw)).find((n) => n.includes("modules"))))].find((n) => n.includes("assets")));
console.log("assetsDir guess failed? trying walk");

function findNamed(dir, needle, depth = 0) {
  if (depth > 5) return null;
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    if (e.name.includes(needle) && e.isDirectory()) return join(dir, e.name);
    if (e.isDirectory() && !e.name.startsWith(".") && e.name !== "node_modules") {
      const hit = findNamed(join(dir, e.name), needle, depth + 1);
      if (hit) return hit;
    }
  }
  return null;
}

const assetsPath = findNamed(fw, "assets");
console.log("assetsPath", assetsPath);
console.log("children", readdirSync(assetsPath));
for (const child of readdirSync(assetsPath)) {
  const p = join(assetsPath, child);
  if (!statSync(p).isDirectory()) continue;
  const hits = readdirSync(p).filter((n) => /compose/i.test(n));
  if (hits.length) console.log("compose hits in", child, hits);
}
const meta = join(assetsPath, readdirSync(assetsPath).find((n) => n.includes("metabolism")));
console.log("metabolism children", readdirSync(meta));
console.log("has .compose", existsSync(join(meta, ".compose")));

const composeDir = readdirSync(assetsPath).find((n) => n.includes("compose"));
console.log("compose brand dir present?", !!composeDir, composeDir);
