// 🔬 c5d TypeScript name-resolution oracle: a TS program over the given root files with an optional overlay directory (staged files shadow the repo), reporting only diagnostics that a wrong split can cause:
// 2304 unknown name, 2305/2459/2724/2614 missing export, 2300/2451 duplicate, 6133/6192/6196 unused import/local, 2307 unresolved RELATIVE module, 2503 namespace, 2552 did you mean. External modules stay unresolved (their names are `any`).
// usage: bun tscheck.ts <overlay-dir|-> <rootfile-list.txt|abs files...>     prints one line per diagnostic, then a count summary.
import {existsSync, readFileSync, statSync} from "node:fs";
import {join, relative, resolve} from "node:path";
const ts = (await import("/Users/ueli/.bun/install/cache/typescript@5.9.3@@@1/lib/typescript.js")).default as typeof import("typescript");
const REPO = "/Users/ueli/Documents/semio";
const [overlayArg, ...rest] = process.argv.slice(2);
const overlay = overlayArg === "-" ? undefined : resolve(overlayArg!);
let roots: string[] = [];
for (const r of rest) { if (r.endsWith(".txt")) roots.push(...readFileSync(r, "utf8").split("\n").filter(Boolean)); else roots.push(r); }
roots = roots.map(r => resolve(REPO, r));
const ovPath = (p: string) => overlay ? join(overlay, relative(REPO, p)) : undefined;
const host = ts.createCompilerHost({}, true);
const origRead = host.readFile.bind(host), origExists = host.fileExists.bind(host), origSource = host.getSourceFile.bind(host);
host.fileExists = p => { const o = ovPath(p); return (o && existsSync(o) && statSync(o).isFile()) || origExists(p); };
const origDir = host.directoryExists!.bind(host);
host.directoryExists = p => { const o = ovPath(p); return (o && existsSync(o) && statSync(o).isDirectory()) || origDir(p); };
host.realpath = p => p;
host.readFile = p => { const o = ovPath(p); return o && existsSync(o) && statSync(o).isFile() ? readFileSync(o, "utf8") : origRead(p); };
host.getSourceFile = (p, lv, onErr, shouldCreate) => { const text = p.startsWith(REPO) ? host.readFile(p) : undefined; return text !== undefined ? ts.createSourceFile(p, text, lv, true) : origSource(p, lv, onErr, shouldCreate); };
const options: ts.CompilerOptions = {noEmit: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, skipLibCheck: true, noUnusedLocals: true, types: [], strict: false, allowJs: false, resolveJsonModule: true, esModuleInterop: true, noResolve: false, lib: ["lib.esnext.d.ts", "lib.dom.d.ts"]};
const program = ts.createProgram({rootNames: roots, options, host});
const KEEP = new Set([2304, 2305, 2459, 2724, 2614, 2300, 2451, 6133, 6192, 6196, 2503, 2552, 2307, 2339, 2724, 2693, 2395, 2440]);
const counts = new Map<number, number>();
let total = 0;
for (const f of roots) {
  const sf = program.getSourceFile(f);
  if (!sf) { console.log("NOFILE", relative(REPO, f)); continue; }
  for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) {
    if (!KEEP.has(d.code) && d.category !== ts.DiagnosticCategory.Error) continue;
    const msg = ts.flattenDiagnosticMessageText(d.messageText, " ");
    if (d.code === 2307 && !/['"]\.{1,2}\//.test(msg)) continue;
    if (d.code === 2339 || d.code === 2693) continue;
    if (!KEEP.has(d.code)) continue;
    const { line } = sf.getLineAndCharacterOfPosition(d.start ?? 0);
    console.log(`${relative(REPO, f).split("/").slice(-3).join("/")}:${line + 1} TS${d.code} ${msg.slice(0, 160)}`);
    counts.set(d.code, (counts.get(d.code) ?? 0) + 1); total++;
  }
}
console.log("files", roots.length, "diagnostics", total, JSON.stringify(Object.fromEntries(counts)));
