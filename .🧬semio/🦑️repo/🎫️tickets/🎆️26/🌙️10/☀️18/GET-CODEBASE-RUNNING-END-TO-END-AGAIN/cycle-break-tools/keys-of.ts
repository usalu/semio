import { model } from "./lib.ts";
const fw = model.byName.get("@semio-tech/framework")!;
for (const d of process.argv.slice(2)) {
  console.log("==", d);
  for (const [k, v] of Object.entries(fw.exports)) { const s = typeof v === "string" ? v : JSON.stringify(v); if (s.startsWith("./" + d + "/") || s === "./" + d) console.log("  ", k, s); }
}
