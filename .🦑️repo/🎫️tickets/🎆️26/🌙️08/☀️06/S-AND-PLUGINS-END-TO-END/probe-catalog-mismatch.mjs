import { readFileSync, existsSync, readdirSync, statSync } from "node:fs";
import { join, dirname } from "node:path";

const root = "/Users/ueli/Documents/semio";

function findFiles(dir, pred, depth = 0, acc = []) {
  if (depth > 16) return acc;
  let ents;
  try {
    ents = readdirSync(dir);
  } catch {
    return acc;
  }
  for (const name of ents) {
    if (["node_modules", "target", "dist", ".git", ".nx", "storybook-static"].includes(name)) continue;
    const p = join(dir, name);
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (st.isDirectory()) findFiles(p, pred, depth + 1, acc);
    else if (pred(name, p)) acc.push(p);
  }
  return acc;
}

function findDir(dir, pred, depth = 0) {
  if (depth > 12) return null;
  let ents;
  try {
    ents = readdirSync(dir);
  } catch {
    return null;
  }
  for (const name of ents) {
    if (["node_modules", "target", "dist", ".git", ".nx"].includes(name)) continue;
    const p = join(dir, name);
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (!st.isDirectory()) continue;
    if (pred(name, p)) return p;
    const hit = findDir(p, pred, depth + 1);
    if (hit) return hit;
  }
  return null;
}

const fwName = readdirSync(root).find((n) => n.includes("framework") && !n.startsWith(".") && readdirSync(join(root, n)).includes("🛍️products") && readdirSync(join(root, n)).includes("🔨️modules"));
const fwPath = join(root, fwName);
console.log("fw:", fwPath);

const actual = findFiles(fwPath, (n, p) => n.endsWith("playgrounds.json") && p.includes("generated"))[0];
console.log("actual catalog:", actual);
const data = JSON.parse(readFileSync(actual, "utf8"));
console.log("rows", data.length);
console.log(
  JSON.stringify(
    data.filter((r) => /puzzle|^3d$|^2d$|^5d$|3d|2d|5d/i.test(`${r.variant} ${(r.aliases || []).join(" ")}`)),
    null,
    2,
  ),
);

const idx = findFiles(fwPath, (n, p) => n === "index.ts" && p.includes("repo") && p.includes("lib") && p.includes("packages")).find((p) =>
  readFileSync(p, "utf8").includes("loadFrameworkOsPlaygroundCatalog"),
);
console.log("loader index:", idx);
const src = readFileSync(idx, "utf8");
const m = src.match(/join\(getWorkspaceRoot\(\),\s*"([^"]+playgrounds\.json)"\)/);
console.log("loader expects relative:", m[1]);
const expectedAbs = join(root, m[1].replace(/^\.\//, ""));
console.log("expected exists?", existsSync(expectedAbs));
console.log("expected abs:", expectedAbs);

const typeImport = [...src.matchAll(/from\s+"([^"]*playgrounds[^"]*)"/g)].map((x) => x[1]);
console.log("playground imports:", typeImport);

const pluginMod = findDir(fwPath, (n, p) => p.endsWith(join("🔨️modules", n)) && n.includes("plugin") && p.includes("os"));
// broader
const osProducts = join(fwPath, "🛍️products");
const osName = readdirSync(osProducts).find((n) => n.includes("os"));
const pluginParent = findDir(join(osProducts, osName), (n) => n.includes("plugin"));
console.log("plugin parent:", pluginParent);
if (pluginParent) console.log("plugin children:", readdirSync(pluginParent));

// resolve 3d
import { createRequire } from "node:module";
// inline resolve logic
function resolveFrameworkOsPlaygroundPlugin(catalog, segments) {
  if (segments.length === 0) return null;
  for (let len = segments.length; len >= 1; len--) {
    const alias = segments.slice(0, len).join(" ");
    const row = catalog.find((r) => r.variant === alias || (r.aliases || []).includes(alias));
    if (row) return { plugin: row.variant, rest: segments.slice(len), row };
  }
  return null;
}
console.log("resolve 3d:", resolveFrameworkOsPlaygroundPlugin(data, ["3d"]));
console.log("resolve puzzle 3d:", resolveFrameworkOsPlaygroundPlugin(data, ["puzzle", "3d"]));
console.log("all variants:", data.map((r) => r.variant).join(", "));
