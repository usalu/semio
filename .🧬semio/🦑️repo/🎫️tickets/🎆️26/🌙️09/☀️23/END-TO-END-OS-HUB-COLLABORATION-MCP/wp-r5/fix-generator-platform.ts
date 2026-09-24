import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative } from "node:path";
import { execSync } from "node:child_process";
const root = "/Users/ueli/Documents/semio";
const target = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`;
const files = execSync(`git grep -l "platform: process.platform" -- '*📜️script.ts'`, { cwd: root, encoding: "utf8" }).split("\n").filter(Boolean);
for (const rel of files) {
  const abs = `${root}/${rel}`;
  let text = readFileSync(abs, "utf8");
  const spec = relative(dirname(abs), target).split("\\").join("/");
  const specifier = spec.startsWith(".") ? spec : `./${spec}`;
  const existing = new RegExp(`import \\{([^}]*)\\} from "${specifier.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}";`);
  const m = text.match(existing);
  if (m) { if (!/\bcurrentPlatform\b/.test(m[1])) text = text.replace(existing, `import {${m[1].replace(/\s*$/, "")}, currentPlatform } from "${specifier}";`); }
  else {
    const imports = [...text.matchAll(/^import [^;]+;\n/gm)];
    const last = imports.at(-1)!;
    const at = last.index! + last[0].length;
    text = `${text.slice(0, at)}import { currentPlatform } from "${specifier}";\n${text.slice(at)}`;
  }
  text = text.replaceAll("platform: process.platform", "platform: currentPlatform()");
  writeFileSync(abs, text);
  console.log(rel);
}
