import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join, relative } from "path";
const [, el] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
const iconsDir = join(el, readdirSync(el).filter((n) => n.endsWith("Icons")).sort((a, b) => a.length - b.length)[0]);
const treeDir = join(el, readdirSync(el).filter((n) => n.endsWith("Tree")).sort((a, b) => a.length - b.length)[0]);
const icons = join(iconsDir, readdirSync(iconsDir).find((n) => n.endsWith("component.tsx")));
const tree = join(treeDir, readdirSync(treeDir).find((n) => n.endsWith("component.tsx")));
let r = relative(iconsDir, tree).replaceAll("\\", "/");
if (!r.startsWith(".")) r = "./" + r;
let t = readFileSync(icons, "utf8");
if (!t.includes("import { uiSpacingLen }")) {
  t = t.replace("import { cn } from", `import { uiSpacingLen } from "${r}";\nimport { cn } from`);
  writeFileSync(icons, t);
  console.log("added uiSpacingLen", r);
} else console.log("uiSpacingLen import present");
const ptbDir = join(el, readdirSync(el).filter((n) => n.endsWith("PanelTabBar")).sort((a, b) => a.length - b.length)[0]);
const ptb = join(ptbDir, readdirSync(ptbDir).find((n) => n.endsWith("component.tsx")));
let p = readFileSync(ptb, "utf8");
const before = p;
p = p.replace('<Icon size={12} className="shrink-0" />', '<span className="shrink-0"><Icon size={12} /></span>');
if (p !== before) {
  writeFileSync(ptb, p);
  console.log("fixed Icon className");
} else console.log("Icon className already ok or pattern missing");
