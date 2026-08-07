import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const framework = fs.readdirSync(root).find((n) => n.endsWith("framework"));
console.log("framework=", JSON.stringify(framework));
function findFlowExt(dir, depth = 0) {
  if (depth > 12) return;
  let ents;
  try {
    ents = fs.readdirSync(dir, { withFileTypes: true });
  } catch {
    return;
  }
  for (const ent of ents) {
    if (["target", "node_modules", "pkg", ".git"].includes(ent.name)) continue;
    const p = path.join(dir, ent.name);
    if (ent.isDirectory()) findFlowExt(p, depth + 1);
    else if (ent.name.endsWith(".rs")) {
      const t = fs.readFileSync(p, "utf8");
      if (t.includes("FlowExtension {") && t.includes("enum Contribution")) {
        console.log("HIT", p);
        const lines = t.split(/\n/);
        for (let i = 0; i < lines.length; i++) {
          if (lines[i].includes("FlowExtension {")) {
            for (let j = Math.max(0, i - 5); j < Math.min(lines.length, i + 18); j++) console.log(`${j + 1}:${lines[j]}`);
          }
        }
      }
    }
  }
}
findFlowExt(path.join(root, framework));

// also list flow extension dirs
const flow = path.join(root, framework, "🛍️products", "💻️os", "🔨️modules");
const mods = fs.readdirSync(flow);
const flowMod = mods.find((n) => n.includes("flow"));
console.log("flowMod", JSON.stringify(flowMod));
const extRoot = path.join(flow, flowMod, fs.readdirSync(path.join(flow, flowMod)).find((n) => n.includes("extensions")));
console.log("extRoot", extRoot);
for (const n of fs.readdirSync(extRoot)) console.log("ext", JSON.stringify(n));
