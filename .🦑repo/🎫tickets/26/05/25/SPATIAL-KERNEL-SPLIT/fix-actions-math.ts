import { readFileSync, writeFileSync } from "node:fs";

let s = readFileSync("spatial/js/core/index.ts", "utf8");
const start = s.indexOf("function builtinActionDefs(): ActionDef[] {");
const end = s.indexOf("// #endregion 🧮ActionRegistry");
const head = s.slice(0, start);
const body = s.slice(start, end);
const tail = s.slice(end);
let b = body;
b = b.replaceAll("run: (params) => {", "run: (params, ctx) => {\n\t\t\tconst pr = ctx.preview;");
b = b.replaceAll("Math.random().toString(36).slice(2, 9)", "pr.randomTag('').slice(pr.randomTag('').indexOf('-') + 1)");
b = b.replaceAll("`anchor-${", "`anchor-${pr.randomTag('anchor')}-");
b = b.replaceAll("`id-${", "`id-${pr.randomTag('id')}-");
b = b.replaceAll("`id-${kind}-", "`id-${pr.randomTag(kind)}-");
b = b.replaceAll("Math.atan2(", "pr.atan2(");
b = b.replaceAll("Math.cos(", "pr.cos(");
b = b.replaceAll("Math.sin(", "pr.sin(");
b = b.replaceAll("Math.hypot(", "pr.hypot3(");
// fix hypot3 calls that had 6 args - hypot3 takes 3 components as deltas
b = b.replace(/pr\.hypot3\((refA\[0\] - center\[0\]), (refA\[1\] - center\[1\]), (refA\[2\] - center\[2\])\)/g, "pr.hypot3($1, $2, $3)");
writeFileSync("spatial/js/core/index.ts", head + b + tail);
console.log("feature actions patched");
