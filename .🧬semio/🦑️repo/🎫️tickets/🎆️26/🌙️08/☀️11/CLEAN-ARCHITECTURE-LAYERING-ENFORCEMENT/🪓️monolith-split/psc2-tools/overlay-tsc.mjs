/** 🧪️ Type-checks the tree as it was before this slice's edits, without touching the disk: the compiler host serves the pre-edit text of every snapshotted file and reports files created by the slice as missing. Usage: node overlay-tsc.mjs <tsconfig.json> <out.txt> <snapshot.json>... */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync } from "node:fs";
const [configPath, out, ...snaps] = process.argv.slice(2);
const ROOT = "/Users/ueli/Documents/semio/";
const before = new Map(); // absolute path -> text | null
for (const s of snaps) for (const [rel, text] of Object.entries(JSON.parse(readFileSync(s, "utf8")))) { const abs = (ROOT + rel).normalize("NFC"); if (!before.has(abs)) before.set(abs, text); }
const ov = p => { const n = p.normalize("NFC"); return before.has(n) ? before.get(n) : undefined; };
const cfg = ts.readConfigFile(configPath, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, { ...ts.sys, readDirectory: ts.sys.readDirectory, fileExists: p => ov(p) !== undefined ? ov(p) !== null : ts.sys.fileExists(p), readFile: p => ov(p) !== undefined ? ov(p) : ts.sys.readFile(p) }, "/Users/ueli/Documents/semio");
const host = ts.createCompilerHost(parsed.options, true);
const baseFileExists = host.fileExists.bind(host), baseReadFile = host.readFile.bind(host), baseGetSourceFile = host.getSourceFile.bind(host);
host.fileExists = p => ov(p) !== undefined ? ov(p) !== null : baseFileExists(p);
host.readFile = p => ov(p) !== undefined ? (ov(p) ?? undefined) : baseReadFile(p);
host.getSourceFile = (p, lang, onError, shouldCreate) => { const o = ov(p); if (o === null) return undefined; if (o !== undefined) return ts.createSourceFile(p, o, lang, true); return baseGetSourceFile(p, lang, onError, shouldCreate); };
const files = parsed.fileNames.filter(f => ov(f) !== null);
const program = ts.createProgram({ rootNames: files, options: parsed.options, host });
const diags = ts.getPreEmitDiagnostics(program);
const lines = [];
for (const d of diags) {
  const msg = ts.flattenDiagnosticMessageText(d.messageText, "\n").split("\n")[0];
  if (d.file) { const { line, character } = d.file.getLineAndCharacterOfPosition(d.start); lines.push(`${d.file.fileName.replace(ROOT, "")}(${line + 1},${character + 1}): error TS${d.code}: ${msg}`); }
  else lines.push(`error TS${d.code}: ${msg}`);
}
writeFileSync(out, lines.join("\n") + "\n");
console.log(`files=${files.length} diagnostics=${diags.length}`);
