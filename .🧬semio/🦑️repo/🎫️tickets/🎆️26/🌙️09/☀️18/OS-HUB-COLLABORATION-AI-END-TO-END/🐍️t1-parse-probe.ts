#!/usr/bin/env bun
/** 🔬️ T1 probe: does the repo's TypeScript resolve, and do its parse diagnostics separate corruption from the repo's legal emoji idiom? */
import { createRequire } from "node:module";
const ts = createRequire(import.meta.url)("typescript");
console.log("typescript", ts.version, "ScriptKind?", typeof ts.ScriptKind);
const corrupt = `export type * as WasiCliEnvironmen🔬️t029 from './x.js';\n`;
const clean = `// existing \`🛠️dev🖥️s⚛️react\` launcher (no \`S_HUB_URL\`)\nconst x = "🏗️fem◻️2d-model";\nconst r = /🎆️26🌙️06☀️04📊️metric/;\nconst t = \`🛠️dev\${1}🧊️wgpu🌐️wasm\`;\nexport const J = () => <div>🖱️ui⚛️react</div>;\n`;
for (const [name, text, kind] of [["corrupt.ts", corrupt, ts.ScriptKind.TS], ["clean.tsx", clean, ts.ScriptKind.TSX]] as [string, string, number][]) {
  const sf = ts.createSourceFile(name, text, ts.ScriptTarget.Latest, false, kind);
  const diags = sf.parseDiagnostics ?? [];
  console.log(name, "parseDiagnostics:", diags.length, diags.map((d: { code: number }) => `TS${d.code}`).join(","));
}
