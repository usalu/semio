import fs from "fs";
import path from "path";

const engine = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine";
const elements = fs.readdirSync(engine).find((n) => n.includes("elements"));
const shell = path.join(engine, elements, "ShellHost", "🟦️component.tsx");
const lines = fs.readFileSync(shell, "utf8").split("\n");
for (let i = 0; i < 80; i++) console.log(`${i + 1}|${lines[i]}`);
console.log("---");
for (let i = 838; i < 848; i++) console.log(`${i + 1}|${lines[i]}`);
console.log("---");
for (let i = 0; i < lines.length; i++) {
  if (lines[i].includes("PluginRegistryEntry") || lines[i].includes("createExtensionSource") || lines[i].includes("multiplexPluginSources")) {
    console.log(i + 1, lines[i].trim().slice(0, 160));
  }
}
