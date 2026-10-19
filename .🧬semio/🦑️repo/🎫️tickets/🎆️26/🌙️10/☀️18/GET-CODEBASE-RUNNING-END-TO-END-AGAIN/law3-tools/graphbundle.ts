import { build } from "esbuild";
import { resolve } from "node:path";
const root = process.cwd();
const execution = resolve(root, "🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts");
const inv = resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const program = `import {PreviewGeneratedScript} from ${JSON.stringify(execution)};import {createScriptProcessEnvelope,withScriptProcessEnvelope} from ${JSON.stringify(inv)};console.log(typeof PreviewGeneratedScript, typeof createScriptProcessEnvelope, typeof withScriptProcessEnvelope);`;
const result = await build({ stdin: { contents: program, resolveDir: root + "/🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite" }, bundle: true, platform: "node", format: "esm", write: false });
const text = result.outputFiles![0]!.text;
const lines = text.split("\n");
const hits: string[] = [];
let current = "";
for (const l of lines) { const m = l.match(/^\/\/ (.*)$/); if (m) current = m[1]!; if (l.includes("import.meta.url") || l.includes("import.meta.dir") || l.includes("__dirname")) hits.push(current + " :: " + l.trim().slice(0, 180)); }
console.log(hits.join("\n"));
