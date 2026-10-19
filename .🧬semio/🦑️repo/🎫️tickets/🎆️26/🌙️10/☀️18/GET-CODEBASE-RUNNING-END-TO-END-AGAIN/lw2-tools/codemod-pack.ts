import { readFileSync, writeFileSync } from "node:fs";
import { relative, resolve, dirname } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const files = process.argv.slice(2);
const OLD = /import \{ runExtensionComponentPackage \} from "((?:\.\.\/)+)🧰️framework\/🛍️products\/💻️os\/🔨️modules\/🔌️plugin\/🏪️store\/📦️packaging\/🟦️\.ts";/u;
let n = 0;
for (const f of files) {
  const p = resolve(ROOT, f);
  let t = readFileSync(p, "utf8");
  const m = OLD.exec(t);
  if (!m) { console.error("skip (no import)", f); continue; }
  t = t.replace(OLD, `import { packageAdmittedExtensionComponentV1 } from "${m[1]}🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/📤️publication/🟦️.ts";`);
  t = t.replace(/await runExtensionComponentPackage\(/gu, "await packageAdmittedExtensionComponentV1(");
  if (t.includes("runExtensionComponentPackage")) throw new Error("leftover " + f);
  writeFileSync(p, t); n++;
}
console.log("rewrote", n);
