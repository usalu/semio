/** 🧪️ Applies the TypeScript projection (types and parsers of every undeclared export) to a sandbox copy: `bun spd-sandbox-apply.ts <sandbox> <touched.json> [under]`. Refuses a root inside the repository. */
import { writeFileSync } from "node:fs";
import { realpathSync } from "node:fs";
const sandbox = realpathSync(process.argv[2]!);
if (!sandbox.includes("/scratchpad/") && !sandbox.startsWith("/tmp/")) throw new Error(`refusing to write outside a sandbox: ${sandbox}`);
if (sandbox.startsWith("/Users/ueli/Documents/semio")) throw new Error("refusing to write inside the repository");
const base = process.env.SPD_PP ?? "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection";
const { projectParsers } = await import(`${base}/🟦️.ts`);
const unders = (process.argv[4] ?? "✏️s,🌎️hub").split(",");
const touched: string[] = [];
const refused: string[] = [];
for (const under of unders) {
  const report = projectParsers({ root: sandbox, mode: "apply", under, formats: new Set(["🟦️typescript"]), closing: false, replace: new Set(), restate: new Set() });
  touched.push(...report.touched);
  refused.push(...report.refused);
  console.log(under, JSON.stringify({ added: report.added, touched: report.touched.length, refused: report.refused.length }));
}
writeFileSync(process.argv[3]!, JSON.stringify({ touched, refused }));
