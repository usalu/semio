import fs from "fs";
import path from "path";
import { spawnSync } from "child_process";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
const SIX = ["core","math","text","logic","dictionary","list"];
const inv = {};
for (const id of SIX) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  const cargo = fs.readFileSync(path.join(pluginExt, dir, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
  const comp = fs.readFileSync(path.join(pluginExt, dir, "🦀️component.rs"), "utf8");
  const lines = comp.split(/\n/).length;
  const ops = (comp.match(/registry\.register_operator/g) || []).length;
  inv[id] = { dir, lines, ops, package: cargo.match(/package = "(semio:[^"]+)"/)?.[1], crate: cargo.match(/name = "([^"]+)"/)?.[1] };
}
const meta = spawnSync("cargo", ["metadata", "--no-deps", "--format-version", "1"], { cwd: root, encoding: "utf8", maxBuffer: 50e6 });
const names = meta.status===0 ? new Set(JSON.parse(meta.stdout).packages.map(p=>p.name)) : null;
fs.writeFileSync(path.join(root, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS/wave3a-inventory.json"), JSON.stringify({inv, metaOk: !!names, packages: SIX.map(id=>`semio-s-plugin-flow-extension-${id}`).map(n=>({n, present: names?.has(n)}))}, null, 2));
console.log(JSON.stringify({inv, metaOk: !!names}, null, 2));
