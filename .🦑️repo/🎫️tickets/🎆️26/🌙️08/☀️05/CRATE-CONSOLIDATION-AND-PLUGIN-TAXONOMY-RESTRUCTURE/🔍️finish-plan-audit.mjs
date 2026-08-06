import { readdirSync, readFileSync, existsSync } from "fs";
import { join } from "path";

const base = ".🦑️repo/🎫️tickets/🎆️26";
let masterPath = "";
outer: for (const m of readdirSync(base)) {
  for (const d of readdirSync(join(base, m))) {
    for (const t of readdirSync(join(base, m, d))) {
      if (t === "CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE") {
        for (const f of readdirSync(join(base, m, d, t))) {
          if (f.endsWith("master.md")) masterPath = join(base, m, d, t, f);
        }
        break outer;
      }
    }
  }
}
const master = readFileSync(masterPath, "utf8");
const start = master.indexOf("## Wave status");
const end = master.indexOf("## Registrar Protocol");
console.log("=== WAVE CHECKBOXES ===");
for (const line of master.slice(start, end).split("\n")) {
  if (/\[(x| )\]/.test(line) || /^### /.test(line) || line.startsWith("## Wave")) console.log(line.slice(0, 240));
}

function walk(dir, acc = [], depth = 0) {
  if (depth > 25) return acc;
  let ents; try { ents = readdirSync(dir, { withFileTypes: true }); } catch { return acc; }
  for (const e of ents) {
    const n = e.name;
    if (["node_modules",".git","target",".venv","pkg","dist"].includes(n)) continue;
    const p = join(dir, n);
    if (e.isDirectory()) {
      if (n === "⚡️implementations" || n === "⚡️implementation") acc.push(p);
      else walk(p, acc, depth + 1);
    }
  }
  return acc;
}
const all = walk(".");
const groups = {};
for (const p of all) {
  const parts = p.split("/");
  let key;
  if (parts[0] === "✏️s" && parts[1] === "🔌️plugins") key = "plugin:" + parts[2];
  else if (parts[0] === "✏️s" && parts[1] === "🔨️modules") key = "s-module:" + parts[2];
  else if (parts[0].includes("hub")) key = "hub:" + parts.slice(1,3).join("/");
  else if (parts[0] === "🧰️framework" && parts[1] === "🛍️products") key = "product:" + parts.slice(2,4).join("/");
  else if (parts[0] === "🧰️framework" && parts[1] === "🔨️modules") key = "fw-mod:" + parts[2];
  else if (parts[0] === "♻️mit-bestand") key = "mit:" + parts.slice(1,3).join("/");
  else key = "other:" + parts.slice(0,3).join("/");
  groups[key] = (groups[key] || 0) + 1;
}
console.log("\n=== IMPLEMENTATIONS REMAINING ===");
console.log(Object.entries(groups).sort((a,b)=>b[1]-a[1]).map(([k,v]) => String(v).padStart(3) + " " + k).join("\n"));
console.log("TOTAL", all.length);

function findName(name, root=".", depth=0, acc=[]) {
  if (depth>12) return acc;
  let ents; try { ents=readdirSync(root,{withFileTypes:true}); } catch { return acc; }
  for (const e of ents) {
    if (["node_modules",".git","target","🎫️tickets"].includes(e.name)) continue;
    const p=join(root,e.name);
    if (e.isFile() && (e.name===name || e.name.endsWith(name))) acc.push(p);
    else if (e.isDirectory() && !e.name.startsWith(".")) findName(name,p,depth+1,acc);
  }
  return acc;
}
console.log("\n=== KEY FILES ===");
for (const n of ["taxonomy.json","discovery.ts","TEMPLATE.md","TEMPLATE-GO.md","TEMPLATE-TS.md","TEMPLATE-FAMILY.md","TEMPLATE-EXT.md"]) {
  console.log(n, findName(n).filter(p=>!p.includes("node_modules")).slice(0,15));
}

console.log("\n=== OPEN RELATED TICKETS ===");
for (const m of readdirSync(base)) {
  if (!m.includes("08")) continue;
  for (const d of readdirSync(join(base, m))) {
    for (const t of readdirSync(join(base, m, d))) {
      const j = JSON.parse(readFileSync(join(base, m, d, t, "🎫️ticket.json"), "utf8"));
      if (j.status !== "open") continue;
      if (/CRATE|SHAPE|FRAMEWORK|S-MODULE|HUB|MECHANISM|FEM|SPACE|RASTER|WRITER|REASONING|SOURCING|TRINITY|UI-ELEMENT|S-AND-PLUGINS|OS-|REPO-|PRINT|MIT-BESTAND|DEMONSTRATOR|DEPENDENCY|TAXONOMY|FINALIZATION|PERIPHERY|PROJECT-JSON|VERIFICATION|COMPILER-MODULE|CONVERGING-FLOW/.test(t)) {
        console.log("OPEN " + d + "/" + t);
      }
    }
  }
}

console.log("\n=== PLUGIN PKG vs IMPL ===");
const pluginsRoot = join("✏️s","🔌️plugins");
let withPkg=0, without=0;
for (const p of readdirSync(pluginsRoot)) {
  const root = join(pluginsRoot,p);
  const hasPkg = existsSync(join(root,"📦️packages"));
  const impls = walk(root);
  if (hasPkg) withPkg++; else without++;
  if (impls.length) console.log(" still-impl", p, impls.length, impls.slice(0,8));
}
console.log({withPkg, without});
