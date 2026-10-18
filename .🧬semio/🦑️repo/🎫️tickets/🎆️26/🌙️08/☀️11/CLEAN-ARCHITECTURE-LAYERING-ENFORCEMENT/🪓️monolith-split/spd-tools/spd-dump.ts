/** 🧮️ Dumps the full refusal list of `projectParsers` (report mode) as JSON lines: `bun spd-dump.ts <out.json> [under] [formats]`. */
import { writeFileSync } from "node:fs";
const root = process.cwd();
const { projectParsers, DEFAULT_FORMATS } = await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts`);
const out = process.argv[2]!;
const under = process.argv[3] ?? "✏️s";
const formats = new Set((process.argv[4] ?? DEFAULT_FORMATS.join(",")).split(","));
const closing = process.argv.includes("--closing");
const report = projectParsers({ root, mode: "report", under, formats, closing, replace: new Set(), restate: new Set() });
writeFileSync(out, JSON.stringify({ under, closing, refused: report.refused, rustAbsent: report.rustAbsent.length, added: report.added }, null, 0));
console.log(under, closing, "refused", report.refused.length);
