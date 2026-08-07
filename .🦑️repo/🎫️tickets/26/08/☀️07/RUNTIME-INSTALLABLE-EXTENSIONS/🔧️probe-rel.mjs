import fs from "fs";
import path from "path";

function walk(dir, pred, acc = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    try {
      if (e.isDirectory()) walk(p, pred, acc);
      else if (pred(p, e.name)) acc.push(p);
    } catch {}
  }
  return acc;
}

const gens = walk(".", (p, n) => p.includes("registry") && p.includes("generated") && n.includes("plugins") && n.endsWith(".ts"));
console.log(gens);
const shell = walk(".", (p, n) => p.includes("ShellHost") && n.endsWith(".tsx"))[0];
console.log("shell", shell);
console.log("rel", path.relative(path.dirname(shell), gens.find((p) => p.endsWith(".ts") && !p.includes(".test"))));
