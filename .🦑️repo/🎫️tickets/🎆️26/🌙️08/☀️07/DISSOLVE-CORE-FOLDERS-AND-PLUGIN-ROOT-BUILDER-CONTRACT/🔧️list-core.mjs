import { readdirSync, readFileSync, statSync } from "fs";
import { join } from "path";

const core = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core";
function walk(dir, depth=0) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    const st = statSync(p);
    const hex = Buffer.from(name, "utf8").toString("hex");
    console.log(`${"  ".repeat(depth)}${st.isDirectory()?"DIR":"FILE"} ${name} hex=${hex} size=${st.size}`);
    if (st.isDirectory() && depth < 4) walk(p, depth+1);
  }
}
walk(core);

const uiDir = readdirSync(core).find(n => n.includes("ui") || Buffer.from(n).includes(0xf0));
console.log("\nfinding ui...");
for (const name of readdirSync(core)) {
  if (name.endsWith("ui") || name.includes("ui")) {
    const ui = join(core, name);
    console.log("UI PATH", ui);
    const rs = join(ui, "🦀️component.rs");
    try {
      const text = readFileSync(rs, "utf8");
      console.log("RS lines", text.split("\n").length);
      const lines = text.split("\n");
      for (let i=0;i<Math.min(80,lines.length);i++) console.log(`${i+1}: ${lines[i].slice(0,160)}`);
      console.log("--- regions ---");
      for (let i=0;i<lines.length;i++) {
        if (/#region|#endregion|pub mod |#\[path/.test(lines[i])) console.log(`${i+1}: ${lines[i].slice(0,160)}`);
      }
    } catch(e) { console.error(e); }
  }
}
