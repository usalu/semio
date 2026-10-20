import { scan } from "../exports-stage-tools/scan.ts";
import { loadPolicy, ncprViolation } from "../exports-stage-tools/rules.ts";
const policy = loadPolicy();
const { edges } = scan();
const all = edges.filter(e => e.spec.includes("?"));
console.log("query imports", all.length);
for (const e of all) { const v = ncprViolation(policy, e.file, e.target); console.log(v ? "VIOL" : "ok  ", e.spec.split("?")[1], e.file, "->", e.target); }
