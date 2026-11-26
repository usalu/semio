/** 🧪️ Runs the cases of a staffage suite with recording assertions (`soft-vitest.ts`) and prints, per case, every assertion that is red with its numbers: soft.ts <suite folder name> <work dir> [<case filter regexp>]. The suite source is read as it is and only its imports are re-pointed; nothing of the suite is changed. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const [suite, work, filter = "."] = process.argv.slice(2);
const file = join(process.env.S!, "🧪️tests", suite!, "🟦️.ts");
const shim = join(import.meta.dir, "soft-vitest.ts");
const source = readFileSync(file, "utf8")
  .replace(/from "(\.\.?\/[^"]+)"/g, (_, relative: string) => `from ${JSON.stringify(resolve(dirname(file), relative))}`)
  .replace(/from "vitest"/g, `from ${JSON.stringify(shim)}`)
  .replaceAll("import.meta.url", JSON.stringify(pathToFileURL(file).href));
mkdirSync(work!, { recursive: true });
const copy = join(work!, "soft-suite.ts");
writeFileSync(copy, source);
await import(copy);
const { cases, failures } = await import(shim);
let red = 0;
for (const c of cases.filter((c: { name: string }) => new RegExp(filter).test(c.name))) {
  failures.length = 0;
  let thrown = "";
  try {
    await c.run();
  } catch (error) {
    thrown = String((error as Error).message ?? error).slice(0, 200);
  }
  const lines = new Map<string, number>();
  for (const f of failures) {
    const line = `${f.label} | ${f.matcher} | got ${JSON.stringify(f.got)} | want ${JSON.stringify(f.want)}`;
    lines.set(line, (lines.get(line) ?? 0) + 1);
  }
  if (lines.size || thrown) red++;
  console.log(`${lines.size || thrown ? "RED  " : "GREEN"} ${c.name}`);
  for (const [line, count] of [...lines].slice(0, 40)) console.log(`      ${line}${count > 1 ? ` (x${count})` : ""}`);
  if (lines.size > 40) console.log(`      … ${lines.size - 40} more distinct assertions`);
  if (thrown) console.log(`      ended by: ${thrown}`);
}
console.log(`soft: ${red} red cases`);
