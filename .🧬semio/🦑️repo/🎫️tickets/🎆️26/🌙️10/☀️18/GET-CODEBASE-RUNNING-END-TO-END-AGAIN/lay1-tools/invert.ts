import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, relative, resolve, join } from "node:path";

const apply = process.argv.includes("--apply");
const rel = (fromDir: string, to: string) => { let r = relative(fromDir, to); if (!r.startsWith(".")) r = "./" + r; return r; };
const kind = (f: string) => (f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
for (const mod of process.argv.slice(2).filter((a) => !a.startsWith("--"))) {
  const text = readFileSync(mod, "utf8");
  const sf = ts.createSourceFile(mod, text, ts.ScriptTarget.Latest, true, kind(mod));
  const modDir = dirname(mod);
  type B = { spec: string; text: string; typeOnly: boolean; kind: "named" | "default" | "ns"; imported: string };
  const imports = new Map<string, B>();
  const destructured = new Map<string, { obj: string; prop: string }>();
  const locals = new Set<string>();
  for (const s of sf.statements) {
    if (ts.isImportDeclaration(s) && s.importClause && ts.isStringLiteral(s.moduleSpecifier)) {
      const c = s.importClause, spec = s.moduleSpecifier.text;
      if (c.name) imports.set(c.name.text, { spec, text: c.name.text, typeOnly: c.isTypeOnly, kind: "default", imported: "default" });
      if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) imports.set(c.namedBindings.name.text, { spec, text: `* as ${c.namedBindings.name.text}`, typeOnly: c.isTypeOnly, kind: "ns", imported: "*" });
      if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) imports.set(e.name.text, { spec, text: e.getText(), typeOnly: c.isTypeOnly || e.isTypeOnly, kind: "named", imported: (e.propertyName ?? e.name).text });
    } else if (ts.isVariableStatement(s)) {
      for (const d of s.declarationList.declarations) {
        if (ts.isObjectBindingPattern(d.name) && d.initializer && ts.isIdentifier(d.initializer)) for (const el of d.name.elements) { if (ts.isIdentifier(el.name)) destructured.set(el.name.text, { obj: d.initializer.text, prop: el.propertyName && ts.isIdentifier(el.propertyName) ? el.propertyName.text : el.name.text }); }
        else if (ts.isIdentifier(d.name)) locals.add(d.name.text);
      }
    } else if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isTypeAliasDeclaration(s)) && s.name) locals.add(s.name.text);
  }
  const ifs = sf.statements.find((s): s is ts.IfStatement => ts.isIfStatement(s) && /import\.meta\.vitest/u.test(s.expression.getText()) && s.expression.getText().trim() === "import.meta.vitest");
  if (!ifs) { console.log(mod, "no hook"); continue; }
  const block = ifs.thenStatement.getText();
  const imp = /await import\(\s*["']([^"']+)["']\s*\)/u.exec(block);
  const callM = /await (registerTests\d+)\(import\.meta\.vitest(?:,\s*\{([^}]*)\}\s*,\s*(\{[^}]*\}))?\s*\)/u.exec(block);
  if (!imp || !callM) { console.log(mod, "hook shape unknown", block.slice(0, 200)); continue; }
  const testFile = resolve(modDir, imp[1]!);
  const fn = callM[1]!;
  const names = (callM[2] ?? "").split(",").map((x) => x.trim()).filter(Boolean);
  const third = callM[3] ?? "";
  const testDir = dirname(testFile);
  const modRel = rel(testDir, mod);
  const importLines = new Map<string, { value: string[]; type: string[] }>();
  const addImp = (spec: string, item: string, typeOnly: boolean) => { const e = importLines.get(spec) ?? { value: [], type: [] }; (typeOnly ? e.type : e.value).push(item); importLines.set(spec, e); };
  const fixSpec = (spec: string) => (spec.startsWith(".") ? rel(testDir, resolve(modDir, spec)) : spec);
  const entries: string[] = [];
  const need = (n: string, asValue = true) => {
    const b = imports.get(n);
    if (b) { addImp(fixSpec(b.spec), b.text, false); return n; }
    return undefined;
  };
  const problems: string[] = [];
  for (const n of names) {
    if (imports.has(n)) { need(n); entries.push(n); }
    else if (destructured.has(n)) { const d = destructured.get(n)!; if (!imports.has(d.obj)) problems.push(`destructure source ${d.obj} local`); else need(d.obj); entries.push(d.prop === n ? `${n}: ${d.obj}.${n}` : `${n}: ${d.obj}.${d.prop}`); }
    else if (locals.has(n)) { addImp(modRel, n, false); entries.push(n); }
    else problems.push(`unknown ${n}`);
  }
  if (problems.length) { console.log(mod, "PROBLEMS", problems.join("; ")); continue; }
  const thirdUrl = third.replace(/import\.meta\.url/u, `new URL("${modRel}", import.meta.url).href`);
  const testText = readFileSync(testFile, "utf8");
  const depTypeM = /dependencies:\s*([A-Za-z0-9_]+TestDependencies)\b/u.exec(testText);
  const depType = depTypeM?.[1];
  const buildGrouped = () => [...importLines.entries()].map(([spec, e]) => {
    const dedup = (a: string[]) => [...new Set(a)];
    const vals = dedup(e.value);
    const star = vals.filter((v) => v.startsWith("* as ")), def = vals.filter((v) => !v.startsWith("* as ") && !v.includes("{") && !/^[A-Za-z_$][\w$]*( as .*)?$/u.test(v));
    const plain = vals.filter((v) => !v.startsWith("* as "));
    const lines: string[] = [];
    if (star.length) lines.push(`import ${star[0]} from "${spec}";`);
    if (plain.length) lines.push(`import { ${plain.join(", ")} } from "${spec}";`);
    return lines.join("\n");
  });
  const entry = third ? `\nconst dependencies${depType ? `: ${depType}` : ""} = { ${entries.join(", ")} };\nawait ${fn}(vitestRuntime, dependencies, ${thirdUrl});\n` : `\nawait ${fn}(vitestRuntime);\n`;
  const testSf = ts.createSourceFile(testFile, testText, ts.ScriptTarget.Latest, true, kind(testFile));
  const bound = new Set<string>();
  for (const st of testSf.statements) {
    if (ts.isImportDeclaration(st) && st.importClause) { const c = st.importClause; if (c.name) bound.add(c.name.text); if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) bound.add(c.namedBindings.name.text); if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) bound.add(e.name.text); }
    else if (ts.isFunctionDeclaration(st) && st.name) bound.add(st.name.text);
    else if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) if (ts.isIdentifier(d.name)) bound.add(d.name.text);
  }
  const clash = [...new Set([...importLines.values()].flatMap((e) => e.value).map((v) => v.replace(/^\* as /u, "").split(" as ").pop()!.trim()))].filter((n) => bound.has(n) || n === "vitestRuntime");
  const upgrade: string[] = [];
  for (const n of clash) {
    const decl = testSf.statements.find((st): st is ts.ImportDeclaration => ts.isImportDeclaration(st) && !!st.importClause?.isTypeOnly && !!st.importClause.namedBindings && ts.isNamedImports(st.importClause.namedBindings) && st.importClause.namedBindings.elements.length === 1 && st.importClause.namedBindings.elements[0]!.name.text === n);
    if (!decl) { console.log("  UNRESOLVED CLASH", n); continue; }
    upgrade.push(n);
    for (const [spec, e] of importLines) e.value = e.value.filter((v) => v.split(" as ").pop()!.trim() !== n);
  }
  console.log(`${mod.split("/").slice(-3).join("/")}: ${names.length} deps -> ${testFile.split("/").slice(-2).join("/")}`);
  if (apply) {
    const lastImp = [...testSf.statements].filter(ts.isImportDeclaration).pop();
    const at = lastImp ? lastImp.getEnd() : 0;
    let patched = testText;
    for (const n of upgrade) patched = patched.replace(new RegExp(`import type \\{ ${n} \\}`, "u"), `import { ${n} }`);
    const at2 = patched.length - testText.length + at;
    const importBlock = `import * as vitestRuntime from "vitest";\n${buildGrouped().filter(Boolean).join("\n")}`;
    const withImports = patched.slice(0, at2) + (lastImp ? "\n" : "") + importBlock + patched.slice(at2);
    writeFileSync(testFile, withImports.trimEnd() + "\n" + entry);
    let out = text;
    const regionRe = /\/\/ #region 🧪️Tests\n([\s\S]*?)\/\/ #endregion 🧪️Tests\n?/u;
    const start = ifs.getStart(), end = ifs.getEnd();
    const rm = regionRe.exec(out);
    if (rm && rm[1]!.includes("import.meta.vitest") && rm[1]!.replace(ifs.getText(), "").trim().length < 40) out = out.replace(regionRe, "");
    else out = out.slice(0, start) + out.slice(end);
    writeFileSync(mod, out.replace(/\n{3,}$/u, "\n"));
  }
}
