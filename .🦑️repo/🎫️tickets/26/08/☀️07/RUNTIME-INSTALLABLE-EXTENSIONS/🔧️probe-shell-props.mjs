import fs from "fs";
import path from "path";

function findShell(dir) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) {
      const hit = findShell(p);
      if (hit) return hit;
    } else if (e.name.includes("component") && p.includes("ShellHost") && p.endsWith(".tsx")) {
      return p;
    }
  }
  return null;
}

const shell = findShell(".");
console.log("shell", shell);
const lines = fs.readFileSync(shell, "utf8").split("\n");
for (let i = 560; i < 660; i++) console.log(`${i + 1}|${lines[i]}`);
for (let i = 0; i < lines.length; i++) {
  if (/const registry|plugins:|PluginRegistryEntry|readonly plugins/.test(lines[i])) {
    console.log("HIT", i + 1, lines[i].trim().slice(0, 160));
  }
}
