/** 👁️ Prints the handler class body (or the whole program head) a target runs, for the hand reads: `bun show.ts <id> [...]`. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
const G = join(import.meta.dir, "../🗑️generated/uncached-owners"), ROOT = "/Users/ueli/Documents/semio";
const recs = JSON.parse(readFileSync(join(G, "records.json"), "utf8"));
for (const id of process.argv.slice(2)) {
  const r = recs.find((r: any) => r.project === id.split(":")[0] && r.target === id.split(":").slice(1).join(":")) ?? recs.find((r: any) => (r.project + ":" + r.target).endsWith(id));
  if (!r) { console.log("== no record", id); continue; }
  const [file, cls] = r.handler.label.split("#"), text = readFileSync(join(ROOT, file.includes("📜️script.ts") && !file.startsWith("🧰") && !file.startsWith("✏") && !file.startsWith("🌎") && !file.startsWith("🎓") && !file.startsWith("♻") && !file.startsWith("🏢") ? r.script : r.script), "utf8");
  console.log(`== ${r.project}:${r.target}  [${r.handler.kind} ${r.handler.label.split("/").slice(-2).join("/")}]  ${r.command}`);
  if (cls) {
    const name = cls.replace(/^lazy:/, ""), i = text.search(new RegExp(`class ${name}\\b`));
    if (i >= 0) { let d = 0, j = text.indexOf("{", i), k = j; for (; k < text.length; k++) { if (text[k] === "{") d++; else if (text[k] === "}" && --d === 0) break; } console.log(text.slice(i, k + 1).slice(0, 2500)); continue; }
  }
  console.log(text.slice(0, 1800));
}
