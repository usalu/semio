/** 🩺️ SH2: evaluates the space script's `homeDirectoryIdentityRowsOracle` of `<root>` with every assertion soft, listing ALL
 * failing pins at once (the oracle throws on the first): `bun p2-oracle-soft.ts <repo-or-overlay-root>`. */
import { writeFileSync } from "node:fs";
import { readFileSync } from "node:fs";
const root = process.argv[2]!;
const script = `${root}/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`;
const source = readFileSync(script, "utf8");
const start = source.indexOf("export function homeDirectoryIdentityRowsOracle(");
const end = source.indexOf("return 54;\n}", start) + "return 54;\n}".length;
const body = source.slice(start, end).replace("export function", "function").replaceAll("import.meta.filename", JSON.stringify(script));
const module = `import { join } from "node:path";\nimport { existsSync, readFileSync } from "node:fs";\nconst fails: string[] = [];\nconst assert = Object.assign((c: unknown, m?: string) => { if (!c) fails.push(m ?? "(no message)"); }, { equal: (a: unknown, b: unknown, m?: string) => { if (a !== b) fails.push(m ?? \`equal \${String(a)} !== \${String(b)}\`); }, deepEqual: (a: unknown, b: unknown, m?: string) => { if (JSON.stringify(a) !== JSON.stringify(b)) fails.push(m ?? "deepEqual"); } });\n${body}\nhomeDirectoryIdentityRowsOracle(${JSON.stringify(root)});\nconsole.log(fails.length ? fails.map((f) => "FAIL " + f.slice(0, 200)).join("\\n") : "all pins hold");\n`;
const out = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-sh2-captures/p2-oracle-soft-eval.ts";
writeFileSync(out, module);
await import(out);
