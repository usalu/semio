import { applyRemovals } from "../keep.ts";
import { planTs, tsSignature } from "../ts-engine.ts";
import { initRust, planRs, rsSignature } from "../rs-engine.ts";
import { readFileSync } from "node:fs";
await initRust("/Users/ueli/Documents/semio/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm");
const d = import.meta.dir;
for (const f of ["a.tsx", "a.rs", "b.ts"]) {
  const t = readFileSync(`${d}/${f}`, "utf8");
  const p = f.endsWith("rs") ? planRs(t) : planTs(f, t);
  const n = applyRemovals(t, p.removals);
  console.log(`----- ${f} removed=${p.removals.length} kept=${p.kept} err=${p.error}`);
  console.log(n);
  const sa = f.endsWith("rs") ? rsSignature(t) : tsSignature(f, t);
  const sb = f.endsWith("rs") ? rsSignature(n) : tsSignature(f, n);
  console.log("sig equal:", JSON.stringify(sa) === JSON.stringify(sb));
}
