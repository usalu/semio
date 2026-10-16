/** 🔬️ Semantic diagnostics of one file whose text is served from `git show HEAD:<path>` (other files from disk). Usage: bun tsc-head-file.ts <tsconfig.json> <repo-relative file> */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { execFileSync } from "node:child_process";
const [cfgPath, file] = process.argv.slice(2);
const ROOT = "/Users/ueli/Documents/semio";
const cfg = ts.readConfigFile(cfgPath!, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, ROOT);
const head = execFileSync("git", ["-c", "core.fsmonitor=false", "show", `HEAD:${file}`], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 });
const host = ts.createCompilerHost(parsed.options);
const read = host.readFile.bind(host);
host.readFile = (name) => (name === `${ROOT}/${file}`.normalize("NFC") || name.normalize("NFC") === `${ROOT}/${file}`.normalize("NFC") ? head : read(name));
const gf = host.getSourceFile.bind(host);
host.getSourceFile = (name, lang, onError, should) => name.normalize("NFC") === `${ROOT}/${file}`.normalize("NFC") ? ts.createSourceFile(name, head, lang) : gf(name, lang, onError, should);
const program = ts.createProgram({ rootNames: parsed.fileNames, options: parsed.options, host });
const sf = program.getSourceFile(`${ROOT}/${file}`)!;
for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) {
  const { line, character } = d.file!.getLineAndCharacterOfPosition(d.start ?? 0);
  console.log(`(${line + 1},${character + 1}) TS${d.code}: ${ts.flattenDiagnosticMessageText(d.messageText, " ").slice(0, 160)}`);
}
