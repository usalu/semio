import { existsSync, lstatSync, readlinkSync, readdirSync, readFileSync } from "fs";
import { join, resolve } from "path";
const root = process.cwd();
const scopes = readFileSync(join(root, ".storybook/scopes.ts"), "utf8");
const m = scopes.match(/from "([^"]+index\.ts)"/);
console.log("scopes import:", m?.[1]);
console.log("scopes exists:", existsSync(resolve(root, ".storybook", m?.[1] ?? "")));
const link = join(root, "node_modules/@semio-tech/repo-lib");
console.log("repo-lib exists:", existsSync(link));
try {
  console.log("repo-lib isLink:", lstatSync(link).isSymbolicLink());
  console.log("repo-lib target:", readlinkSync(link));
} catch (e) {
  console.log("repo-lib link err:", e.message);
}
// find actual package.json for repo-lib
function findPkg(dir, depth=0, acc=[]) {
  if (depth>6) return acc;
  let ents; try { ents = readdirSync(dir,{withFileTypes:true}); } catch { return acc; }
  for (const e of ents) {
    if (["node_modules","target","dist",".git"].includes(e.name)) continue;
    const p = join(dir, e.name);
    if (e.isDirectory()) findPkg(p, depth+1, acc);
    else if (e.name === "package.json") {
      try {
        const j = JSON.parse(readFileSync(p,"utf8"));
        if (j.name === "@semio-tech/repo-lib") acc.push(p);
      } catch {}
    }
  }
  return acc;
}
const fw = readdirSync(root).find(x => x.includes("framework") && !x.startsWith("."));
const pkgs = findPkg(join(root, fw, "🛍️products", "🦑️repo"));
console.log("repo-lib package.json locations:", pkgs);
for (const p of pkgs) {
  const j = JSON.parse(readFileSync(p,"utf8"));
  console.log(p, "exports", j.exports || j.main);
}
