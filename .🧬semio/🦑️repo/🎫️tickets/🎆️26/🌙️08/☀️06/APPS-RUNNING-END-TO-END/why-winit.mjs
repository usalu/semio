import { spawnSync } from "child_process";
import { readdirSync, writeFileSync, readFileSync, existsSync } from "fs";
import { join } from "path";
const root = process.cwd();
const sDir = readdirSync(root).find(n => n.startsWith("✏️") || n.includes("s"));
// find fem cargo
function findCargo(base, needle, depth=0, acc=[]) {
  if (depth>8) return acc;
  let ents; try { ents=readdirSync(base,{withFileTypes:true}); } catch { return acc; }
  for (const e of ents) {
    if (["node_modules","target",".git"].includes(e.name)) continue;
    const p=join(base,e.name);
    if (e.isFile() && e.name==="Cargo.toml" && p.includes(needle) && p.includes("rust")) acc.push(p);
    if (e.isDirectory()) findCargo(p, needle, depth+1, acc);
  }
  return acc;
}
const fems = findCargo(join(root, sDir), "fem");
console.log("fem cargos", fems);
const cargo = fems.find(p => p.includes("packages") && p.includes("rust")) || fems[0];
console.log("using", cargo);
const pkgDir = cargo.replace(/\/Cargo\.toml$/,"");
// cargo tree for winit on wasip2
const r = spawnSync("cargo", ["tree", "-p", "semio-plugin-fem", "--target", "wasm32-wasip2", "-i", "winit"], {
  cwd: root,
  encoding: "utf8",
  env: { ...process.env, CARGO_TERM_COLOR: "never" },
  timeout: 120000,
});
const out = (r.stdout||"") + (r.stderr||"");
console.log(out.slice(0,4000));
writeFileSync(join(process.argv[2], "why-winit.txt"), out);
