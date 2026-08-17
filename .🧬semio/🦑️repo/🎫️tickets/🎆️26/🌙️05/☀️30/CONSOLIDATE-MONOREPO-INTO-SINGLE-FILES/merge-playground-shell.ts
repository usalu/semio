import { readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../../framework/playground/renderer/react");
const shell = readFileSync(join(root, "shell.tsx"), "utf8");
const testsMatch = shell.match(/\n\/\/#region 🧪️Tests[\s\S]*$/);
const shellCore = testsMatch ? shell.slice(0, testsMatch.index!) : shell;
const shellTests = testsMatch?.[0] ?? "";
let body = readFileSync(join(root, "index.tsx"), "utf8");
body = body.replace(/^export \* from "\.\/shell\.tsx";\n\n?/, "");
writeFileSync(join(root, "index.tsx"), shellCore + "\n" + body + shellTests);
unlinkSync(join(root, "shell.tsx"));
console.log("[merge] playground renderer → single index.tsx");
