import ts from "typescript";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, PKG_DIR, ROOT, rel } from "./lib.ts";
import { DOMAINS, entryOf } from "./rules.ts";

const glue = JSON.parse(readFileSync(resolve(GENERATED, "glue.json"), "utf8")) as { file: string }[];
void DOMAINS; void entryOf;
const cfgPath = resolve(PKG_DIR, "tsconfig.json");
const cfg = ts.getParsedCommandLineOfConfigFile(cfgPath, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: d => { throw Error(ts.flattenDiagnosticMessageText(d.messageText, "\n")); } })!;
const roots = glue.filter(g => g.file.endsWith(".ts") || g.file.endsWith(".mjs")).map(g => resolve(ROOT, g.file));
const program = ts.createProgram({ rootNames: roots, options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined, skipLibCheck: true } });
let bad = 0;
const t0 = Date.now();
for (const f of roots) {
  const sf = program.getSourceFile(f);
  if (!sf) { console.log("missing source", f); bad++; continue; }
  const diags = [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)];
  for (const d of diags) {
    bad++;
    const { line } = d.file ? d.file.getLineAndCharacterOfPosition(d.start ?? 0) : { line: 0 };
    console.log(`${rel(f).split("/").slice(-2).join("/")}:${line + 1} TS${d.code} ${ts.flattenDiagnosticMessageText(d.messageText, " ").slice(0, 240)}`);
  }
}
console.log(`glue files ${roots.length}, program files ${program.getSourceFiles().length}, diagnostics ${bad}, ${Date.now() - t0} ms`);
