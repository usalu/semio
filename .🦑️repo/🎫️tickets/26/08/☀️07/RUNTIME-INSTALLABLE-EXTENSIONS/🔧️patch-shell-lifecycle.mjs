import fs from "fs";
import path from "path";

function find(pred) {
  function walk(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "target", ".git"].includes(e.name)) continue;
      const p = path.join(dir, e.name);
      try {
        if (e.isDirectory()) {
          const hit = walk(p);
          if (hit) return hit;
        } else if (pred(p, e.name)) return p;
      } catch {}
    }
    return null;
  }
  return walk(".");
}

const shellPath = find((p, n) => p.includes("ShellHost") && n.endsWith(".tsx") && n.includes("component"));
let text = fs.readFileSync(shellPath, "utf8");
if (text.includes("const installExtension = useCallback")) {
  console.log("lifecycle already inserted");
  process.exit(0);
}

const snippet = fs.readFileSync(
  ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS/lifecycle-snippet.txt",
  "utf8",
);

const endRegionRe = /\n  \/\/#endregion [^\n]*PluginRuntime\n/;
const match = text.match(endRegionRe);
if (!match) throw new Error("endregion PluginRuntime not found");
text = text.replace(endRegionRe, `\n${snippet}  ${match[0].trim()}\n`);
fs.writeFileSync(shellPath, text);
console.log("inserted lifecycle");
