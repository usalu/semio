/**
 * @emoji 🔗 Rewrites include_str!/include_bytes! paths from plugin-root 📚️examples to artifact ♻️reuse .semio leaves.
 */
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const ROOTS = [
  join(REPO, "✏️s/🔌️plugins"),
  join(REPO, "🧰️framework/🛍️products/💻️os"),
  join(REPO, "✏️s/🔨️modules"),
];

const slug = (name) => name.replace(/\uFE0F/g, "").replace(/^[^\p{L}\p{N}]+/u, "");

const semioPaths = [];
const walk = (dir) => {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    if (ent.name.startsWith(".") || ent.name === "node_modules" || ent.name === "target") continue;
    const p = join(dir, ent.name);
    if (ent.isDirectory()) walk(p);
    else if (ent.name.endsWith(".semio") && p.includes("📚️examples")) semioPaths.push(p);
  }
};

for (const r of ROOTS) {
  if (statSync(r).isDirectory()) walk(r);
}

const byBasename = new Map();
for (const p of semioPaths) {
  const leaf = p.split("/").pop();
  if (!byBasename.has(leaf)) byBasename.set(leaf, []);
  byBasename.get(leaf).push(p);
}

const walkSources = (dir, out = []) => {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    if (ent.name.startsWith(".") || ent.name === "node_modules" || ent.name === "target") continue;
    const p = join(dir, ent.name);
    if (ent.isDirectory()) walkSources(p, out);
    else if (/\.(rs|ts)$/.test(ent.name)) out.push(p);
  }
  return out;
};

let changed = 0;
for (const r of ROOTS) {
  for (const file of walkSources(r)) {
    let text = readFileSync(file, "utf8");
    if (!text.includes("📚️examples") && !text.includes(".writer") && !text.includes(".flow")) continue;
    const orig = text;
    text = text.replace(
      /include_str!\("([^"]*📚️examples[^"]+)"\)/g,
      (m, oldPath) => {
        const legacyFile = oldPath.split("/").pop();
        const ext = legacyFile.includes(".") ? legacyFile.slice(legacyFile.lastIndexOf(".")) : "";
        const stem = ext ? legacyFile.slice(0, -ext.length) : legacyFile;
        const match = [...semioPaths].find((sp) => {
          const name = sp.split("/").pop();
          return (
            name.includes(slug(stem)) ||
            name.includes(ext.replace(".", "")) ||
            legacyFile.includes(name.replace("🧬️component.", "").replace(".dsl.semio", ""))
          );
        });
        if (!match) return m;
        const fromDir = file.substring(0, file.lastIndexOf("/"));
        const rel = relativePath(fromDir, match);
        return `include_str!("${rel}")`;
      },
    );
    if (text !== orig) {
      writeFileSync(file, text);
      changed++;
    }
  }
}

function relativePath(from, to) {
  const fromParts = from.split("/");
  const toParts = to.split("/");
  let i = 0;
  while (i < fromParts.length && i < toParts.length && fromParts[i] === toParts[i]) i++;
  const up = fromParts.length - i;
  const rel = [...Array(up).fill(".."), ...toParts.slice(i)].join("/");
  return rel;
}

console.log(`[rewrite] updated ${changed} source file(s)`);
