import { readdirSync, readFileSync, writeFileSync, existsSync, statSync } from "fs";
import { join } from "path";
const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
const core = join(el, readdirSync(el).find((n) => n.includes("core")));
console.log(readdirSync(core));
const labelDir = readdirSync(core).find((n) => n.endsWith("Label") && !n.includes("UiLabel"));
const uiLabelDir = readdirSync(core).find((n) => n.includes("UiLabel"));
console.log({ labelDir, uiLabelDir });
if (labelDir) {
  const f = join(core, labelDir, readdirSync(join(core, labelDir)).find((n) => n.endsWith(".tsx")));
  console.log("Label exports", readFileSync(f, "utf8").split("\n").filter((l) => l.startsWith("export")).slice(0, 10));
}
// Find files that import Label from UiLabel path incorrectly
function walk(d, out = []) {
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (n.endsWith(".tsx")) out.push(p);
  }
  return out;
}
for (const f of walk(el)) {
  const t = readFileSync(f, "utf8");
  if (/import\s*\{[^}]*\bLabel\b[^}]*\}\s*from\s*"[^"]*UiLabel/.test(t)) {
    console.log("BAD Label from UiLabel", f);
  }
}
// ButtonGroup import section
const bg = join(el, readdirSync(el).find((n) => n.endsWith("ButtonGroup")));
const bgf = join(bg, readdirSync(bg).find((n) => n.endsWith("component.tsx")));
console.log("--- ButtonGroup head ---\n" + readFileSync(bgf, "utf8").split("\n").slice(0, 40).join("\n"));
const btn = join(el, readdirSync(el).find((n) => n.endsWith("Button") && !n.includes("Group")));
const btnf = join(btn, readdirSync(btn).find((n) => n.endsWith("component.tsx")));
console.log("--- Button head ---\n" + readFileSync(btnf, "utf8").split("\n").slice(0, 30).join("\n"));
