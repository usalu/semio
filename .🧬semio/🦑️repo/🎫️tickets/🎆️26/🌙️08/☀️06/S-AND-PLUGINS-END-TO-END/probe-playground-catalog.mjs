import { readdirSync, readFileSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";

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

const fwDirs = readdirSync(root).filter((n) => n.includes("framework"));
console.log("framework dirs:", fwDirs);

for (const fw of fwDirs) {
  const fwPath = join(root, fw);
  console.log("\n==", fwPath);
  try {
    console.log("children:", readdirSync(fwPath));
  } catch (e) {
    console.log("err", e.message);
    continue;
  }
}

const catalogHits = findFiles(root, (n) => n.includes("playgrounds"));
console.log("\nplaygrounds hits:\n" + catalogHits.join("\n"));

const idxHits = findFiles(root, (n, p) => n === "index.ts" && p.includes("repo") && p.includes("lib") && p.includes("typescript") && p.includes("packages"));
console.log("\nlib index hits:\n" + idxHits.join("\n"));

for (const idx of idxHits) {
  const src = readFileSync(idx, "utf8");
  if (!src.includes("loadFrameworkOsPlaygroundCatalog")) continue;
  const m = src.match(/join\(getWorkspaceRoot\(\),\s*"([^"]+)"\)/);
  console.log("\ncatalog path literal:", m?.[1]);
  if (m) {
    const abs = join(root, m[1].replace(/^\.\//, ""));
    console.log("abs exists?", existsSync(abs), abs);
    if (existsSync(abs)) {
      const data = JSON.parse(readFileSync(abs, "utf8"));
      console.log("rows", data.length);
      console.log(
        data
          .filter((r) => /puzzle|3d|5d|2d/i.test(JSON.stringify(r)))
          .map((r) => ({ variant: r.variant, aliases: r.aliases, plugin: r.plugin })),
      );
    } else {
      // walk up
      let cur = abs;
      const { dirname } = await import("node:path");
      while (cur !== "/" && !existsSync(cur)) {
        console.log("missing:", cur);
        cur = dirname(cur);
      }
      console.log("first existing:", cur, existsSync(cur) ? readdirSync(cur).slice(0, 40) : []);
    }
  }
}
