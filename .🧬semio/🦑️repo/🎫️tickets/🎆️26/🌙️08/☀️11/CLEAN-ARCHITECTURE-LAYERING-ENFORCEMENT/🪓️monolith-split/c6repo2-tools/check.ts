import { ts } from "./lib.ts";
import { readFileSync } from "node:fs";

const INTEREST = new Set([2339, 2304, 2305, 2307, 2724, 2459, 2460, 2614, 2300, 2451, 2395, 2440, 2552, 2503, 2693, 2749, 2686, 1192, 1214, 2661, 2441]);

export function diagnose(files: string[], ignoreMsgs: Set<string>, overlay?: Map<string, string>, hidden?: Set<string>) {
  const options: import("typescript").CompilerOptions = {
    noEmit: true, lib: ["lib.esnext.d.ts"], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler,
    allowImportingTsExtensions: true, skipLibCheck: true, types: [], jsx: ts.JsxEmit.Preserve, strict: false, allowJs: false, resolveJsonModule: true,
  };
  const host = ts.createCompilerHost(options, true);
  if (overlay || hidden) {
    const fe = host.fileExists.bind(host), rf = host.readFile.bind(host), gs = host.getSourceFile.bind(host);
    const de = host.directoryExists?.bind(host);
    const dirs = new Set<string>();
    for (const f of overlay?.keys() ?? []) { let d = f; while ((d = d.slice(0, d.lastIndexOf("/"))) && !dirs.has(d)) dirs.add(d); }
    host.directoryExists = (d) => dirs.has(d) || (de ? de(d) : true);
    host.fileExists = (f) => (overlay?.has(f) ? true : hidden?.has(f) ? false : fe(f));
    host.readFile = (f) => (overlay?.has(f) ? overlay.get(f) : hidden?.has(f) ? undefined : rf(f));
    host.getSourceFile = (f, lang, ...rest) => (overlay?.has(f) ? ts.createSourceFile(f, overlay.get(f)!, lang, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : hidden?.has(f) ? undefined : gs(f, lang, ...rest));
  }
  const program = ts.createProgram(files, options, host);
  const out: { file: string; line: number; code: number; msg: string; name?: string }[] = [];
  const want = new Set(files);
  for (const sf of program.getSourceFiles()) {
    if (!want.has(sf.fileName)) continue;
    const diags = [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)];
    for (const d of diags) {
      if (!d.file) continue;
      const msg = ts.flattenDiagnosticMessageText(d.messageText, "\n");
      if (d.code === 1005 || d.code === 1109 || d.code < 2000 && d.code !== 1192 && d.code !== 1214) { out.push({ file: sf.fileName, line: d.file.getLineAndCharacterOfPosition(d.start ?? 0).line + 1, code: d.code, msg }); continue; }
      if (!INTEREST.has(d.code)) continue;
      if (d.code === 2307 && !/'\.\.?\//.test(msg)) continue;
      if (d.code === 2339 && !/typeof import\(/.test(msg)) continue;
      const m = msg.match(/Cannot find name '([^']+)'/);
      if (ignoreMsgs.has(msg)) continue;
      out.push({ file: sf.fileName, line: d.file.getLineAndCharacterOfPosition(d.start ?? 0).line + 1, code: d.code, msg, name: m?.[1] });
    }
  }
  return out;
}

if (import.meta.main) {
  const baseline = process.argv[2];
  const files = process.argv.slice(3);
  const base = diagnose([baseline], new Set());
  const names = new Set(base.map((d) => d.msg));
  console.log(`baseline ${baseline}: ${base.length} diag(s) of interest, ignored`);
  const res = diagnose(files, names);
  for (const d of res.slice(0, 200)) console.log(`${d.file}:${d.line} TS${d.code} ${d.msg}`);
  console.log(`${res.length} diagnostics of interest across ${files.length} files`);
}
