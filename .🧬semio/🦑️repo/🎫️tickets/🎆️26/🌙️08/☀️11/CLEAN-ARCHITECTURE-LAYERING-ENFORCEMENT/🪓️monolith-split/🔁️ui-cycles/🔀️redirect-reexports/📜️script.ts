/** 🔀 Rewrites named imports of re-exported symbols so each imports from the module that truly declares it.
 * Usage: bun 📜️script.ts <tsconfig> <fromModulePathSubstring> <apply|dry> <file...>
 * Only imports whose resolved module path contains <fromModulePathSubstring> are redirected; symbols declared outside the program/node_modules are left alone.
 */
import ts from "typescript";
import { dirname, relative, resolve, sep } from "node:path";
import { readFileSync, writeFileSync } from "node:fs";

const [tsconfigPath, fromSub, mode, ...files] = process.argv.slice(2) as [string, string, string, ...string[]];
const cfg = ts.readConfigFile(tsconfigPath, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, dirname(resolve(tsconfigPath)));
const rootNames = [...new Set([...parsed.fileNames, ...files.map((f) => resolve(f))])];
const program = ts.createProgram(rootNames, parsed.options);
const checker = program.getTypeChecker();
const repo = process.cwd();

for (const file of files) {
  const abs = resolve(file);
  const sf = program.getSourceFile(abs);
  if (!sf) {
    console.log(`not in program: ${file}`);
    continue;
  }
  const text = sf.getFullText();
  const edits: { start: number; end: number; text: string }[] = [];
  for (const st of sf.statements) {
    if (!ts.isImportDeclaration(st) || !ts.isStringLiteral(st.moduleSpecifier) || !st.importClause?.namedBindings || !ts.isNamedImports(st.importClause.namedBindings)) continue;
    const resolved = ts.resolveModuleName(st.moduleSpecifier.text, abs, parsed.options, ts.sys).resolvedModule?.resolvedFileName;
    if (!resolved || !resolved.includes(fromSub)) continue;
    const groups = new Map<string, string[]>();
    const keep: string[] = [];
    for (const el of st.importClause.namedBindings.elements) {
      let sym = checker.getSymbolAtLocation(el.name);
      if (sym && sym.flags & ts.SymbolFlags.Alias) sym = checker.getAliasedSymbol(sym);
      const decl = sym?.declarations?.[0];
      const declFile = decl?.getSourceFile().fileName;
      if (!declFile || declFile === resolved || declFile.includes("/node_modules/") || declFile.endsWith(".d.ts")) {
        keep.push(el.getText(sf));
        continue;
      }
      const list = groups.get(declFile) ?? [];
      list.push(el.getText(sf));
      groups.set(declFile, list);
    }
    if (groups.size === 0) continue;
    const typeKw = st.importClause.isTypeOnly ? "type " : "";
    const lines: string[] = [];
    if (keep.length || st.importClause.name) lines.push(`import ${typeKw}${[st.importClause.name?.text, keep.length ? `{ ${keep.join(", ")} }` : undefined].filter(Boolean).join(", ")} from ${st.moduleSpecifier.getText(sf)};`);
    for (const [declFile, names] of groups) {
      let spec = relative(dirname(abs), declFile).split(sep).join("/");
      if (!spec.startsWith(".")) spec = `./${spec}`;
      lines.push(`import ${typeKw}{ ${names.join(", ")} } from "${spec}";`);
    }
    console.log(`${relative(repo, abs)}: ${st.moduleSpecifier.getText(sf)} -> ${[...groups.keys()].map((g) => relative(repo, g)).join(", ")}`);
    edits.push({ start: st.getStart(sf), end: st.getEnd(), text: lines.join("\n") });
  }
  if (mode === "apply" && edits.length) {
    let out = readFileSync(abs, "utf8");
    for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
    writeFileSync(abs, out);
  }
}
