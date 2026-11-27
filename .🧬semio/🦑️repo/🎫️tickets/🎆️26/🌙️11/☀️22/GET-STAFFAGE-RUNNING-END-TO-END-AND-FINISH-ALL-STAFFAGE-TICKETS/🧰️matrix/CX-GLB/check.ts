import { readFileSync, readdirSync } from "node:fs";
import { checkGlb } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";
for (const f of readdirSync(process.argv[2]!).filter((n) => n.endsWith(".glb"))) {
  const bytes = new Uint8Array(readFileSync(`${process.argv[2]}/${f}`));
  const r = await checkGlb(bytes, f, f);
  console.log("[DEBUG]", f, bytes.length, "MB", (bytes.length / 1048576).toFixed(2), "errors", r.facts.errors, "warnings", r.facts.warnings, "tri", r.facts.triangles, "findings", JSON.stringify(r.findings.slice(0, 5)).slice(0, 600));
}
