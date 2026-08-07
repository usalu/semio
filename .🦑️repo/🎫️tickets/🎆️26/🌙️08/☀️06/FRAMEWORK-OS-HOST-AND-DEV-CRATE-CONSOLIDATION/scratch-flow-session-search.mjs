import { readFileSync, readdirSync, statSync, existsSync } from "fs";
import { join } from "path";

const flow = readFileSync(join(process.argv[2], "🧪paths-e2e.txt"), "utf8").trim().split("\n")[0];
console.log("flow", flow);

function walk(dir, fn, depth = 0) {
  if (depth > 8 || !existsSync(dir)) return;
  for (const name of readdirSync(dir)) {
    if (["node_modules", "target", "pkg"].includes(name)) continue;
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, fn, depth + 1);
    else fn(p, name);
  }
}

walk(flow, (p, name) => {
  if (!name.endsWith(".rs") && !name.endsWith(".toml")) return;
  const t = readFileSync(p, "utf8");
  if (/FlowSession|crate-type.*cdylib|wasm-bindgen/.test(t)) {
    const hits = [...t.matchAll(/.*FlowSession.*|.*cdylib.*|.*wasm-bindgen.*/g)].slice(0, 15).map((m) => m[0]);
    console.log("\nFILE", p);
    console.log(hits.join("\n"));
  }
});

const glue = join(flow, "📦️packages/🦀️rust/📦️glue.rs");
const cargo = join(flow, "📦️packages/🦀️rust/Cargo.toml");
console.log("\nGLUE\n", readFileSync(glue, "utf8"));
console.log("\nCARGO\n", readFileSync(cargo, "utf8"));
