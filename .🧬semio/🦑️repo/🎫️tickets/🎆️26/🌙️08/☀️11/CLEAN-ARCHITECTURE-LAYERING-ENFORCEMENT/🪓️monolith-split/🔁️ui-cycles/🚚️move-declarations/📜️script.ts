/** 🚚 Moves top-level declarations from one TS module into another, rewriting imports on both sides and in every importer.
 * Usage: bun 📜️script.ts <spec.json> [--apply]
 * Spec: { source, target, names[], doc, keepExports?: "named"|"star"|"none", noClosure?, exportAll? }
 *   keepExports: how the SOURCE keeps exposing moved public names (barrels only). Default "none" (importers are rewritten).
 */
import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";
import { execFileSync } from "node:child_process";

type Spec = { source: string; target: string; names: string[]; doc: string; keepExports?: "named" | "star" | "none"; noClosure?: boolean; exportAll?: boolean };
const [specPath, ...flags] = process.argv.slice(2);
const apply = flags.includes("--apply");
const repo = process.cwd();
const spec: Spec = JSON.parse(readFileSync(specPath!, "utf8"));
const sourcePath = resolve(spec.source);
const targetPath = resolve(spec.target);
const sourceText = readFileSync(sourcePath, "utf8");
const kindOf = (path: string) => (path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const compilerOptions: ts.CompilerOptions = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, strict: true, isolatedModules: true, noResolve: true, types: [] };
const host = ts.createCompilerHost(compilerOptions);
const baseGetSourceFile = host.getSourceFile.bind(host);
host.getSourceFile = (fileName, languageVersion, ...rest) => {
  if (resolve(fileName) === sourcePath) return ts.createSourceFile(fileName, sourceText, languageVersion, true, kindOf(sourcePath));
  return baseGetSourceFile(fileName, languageVersion, ...rest);
};
const program = ts.createProgram([sourcePath], compilerOptions, host);
const checker = program.getTypeChecker();
const sf = program.getSourceFile(sourcePath)!;
const rel = (p: string) => relative(repo, p);

const declaredNames = (st: ts.Statement): string[] => {
  const out: string[] = [];
  if (ts.isVariableStatement(st)) {
    for (const d of st.declarationList.declarations) {
      if (ts.isIdentifier(d.name)) out.push(d.name.text);
      else for (const el of d.name.elements) if (ts.isBindingElement(el) && ts.isIdentifier(el.name)) out.push(el.name.text);
    }
  } else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st) || ts.isModuleDeclaration(st)) && st.name && ts.isIdentifier(st.name)) out.push(st.name.text);
  return out;
};
const isExported = (st: ts.Statement) => ts.canHaveModifiers(st) && (ts.getModifiers(st) ?? []).some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
const statementOf = new Map<string, ts.Statement[]>();
for (const st of sf.statements) for (const n of declaredNames(st)) statementOf.set(n, [...(statementOf.get(n) ?? []), st]);

const moved = new Set<ts.Statement>();
const queue: ts.Statement[] = [];
const addStatement = (st: ts.Statement, reason: string) => {
  if (moved.has(st)) return;
  moved.add(st);
  queue.push(st);
  if (reason !== "seed") console.log(`  closure +${declaredNames(st).join(",") || ts.SyntaxKind[st.kind]} (${reason})`);
};
for (const n of spec.names) {
  const sts = statementOf.get(n);
  if (!sts) throw new Error(`name not declared at top level: ${n}`);
  for (const st of sts) addStatement(st, "seed");
}
const topLevelOf = (node: ts.Node): ts.Statement | undefined => {
  let p: ts.Node = node;
  while (p.parent && p.parent !== sf) p = p.parent;
  return p.parent === sf ? (p as ts.Statement) : undefined;
};
const symbolOf = (id: ts.Identifier): ts.Symbol | undefined => {
  const parent = id.parent;
  if (ts.isShorthandPropertyAssignment(parent) && parent.name === id) return checker.getShorthandAssignmentValueSymbol(parent);
  if (ts.isExportSpecifier(parent)) {
    const decl = parent.parent.parent;
    if (decl.moduleSpecifier || (parent.propertyName ?? parent.name) !== id) return undefined;
    return checker.getExportSpecifierLocalTargetSymbol(parent);
  }
  return checker.getSymbolAtLocation(id);
};
const isRealReference = (id: ts.Identifier): boolean => {
  const p = id.parent;
  if (ts.isPropertyAccessExpression(p) && p.name === id) return false;
  if (ts.isQualifiedName(p) && p.right === id) return false;
  if ((ts.isPropertyAssignment(p) || ts.isPropertySignature(p) || ts.isMethodDeclaration(p) || ts.isMethodSignature(p) || ts.isPropertyDeclaration(p) || ts.isGetAccessor(p) || ts.isSetAccessor(p) || ts.isEnumMember(p)) && p.name === id) return false;
  if (ts.isJsxAttribute(p) && p.name === id) return false;
  if (ts.isLabeledStatement(p) || ts.isBreakOrContinueStatement(p)) return false;
  return true;
};
const importOf = new Map<ts.Symbol, ts.ImportDeclaration>();
for (const st of sf.statements) {
  if (!ts.isImportDeclaration(st) || !st.importClause) continue;
  const c = st.importClause;
  const bindings: ts.Identifier[] = [];
  if (c.name) bindings.push(c.name);
  if (c.namedBindings) {
    if (ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) bindings.push(e.name);
    else bindings.push(c.namedBindings.name);
  }
  for (const b of bindings) {
    const s = checker.getSymbolAtLocation(b);
    if (s) importOf.set(s, st);
  }
}
const usedImportBindings = new Map<ts.Symbol, { inMoved: boolean; inRemaining: boolean }>();
const movedUses = new Map<ts.Statement, Set<ts.Statement>>();
const refsOfMovedFromRemaining = new Set<string>();

const classify = (): void => {
  const walk = (node: ts.Node, owner: ts.Statement) => {
    if (ts.isImportDeclaration(node)) return;
    if (ts.isIdentifier(node) && isRealReference(node)) {
      const s = symbolOf(node);
      if (s) {
        if (importOf.has(s)) {
          const u = usedImportBindings.get(s) ?? { inMoved: false, inRemaining: false };
          if (moved.has(owner)) u.inMoved = true;
          else u.inRemaining = true;
          usedImportBindings.set(s, u);
        }
        for (const d of s.declarations ?? []) {
          if (d.getSourceFile() !== sf) continue;
          const top = topLevelOf(d);
          if (!top || ts.isImportDeclaration(top)) continue;
          if (top === owner) continue;
          if (moved.has(owner) && !moved.has(top) && !queue.includes(top)) {
            if (spec.noClosure) throw new Error(`moved statement references non-moved ${declaredNames(top).join(",")}`);
            addStatement(top, `needed by ${declaredNames(owner).join(",") || "stmt"}`);
          }
          if (!moved.has(owner) && moved.has(top)) refsOfMovedFromRemaining.add(node.text);
        }
      }
    }
    ts.forEachChild(node, (c) => walk(c, owner));
  };
  for (const st of sf.statements) walk(st, st);
};
for (let round = 0; ; round += 1) {
  const before = moved.size;
  usedImportBindings.clear();
  refsOfMovedFromRemaining.clear();
  classify();
  if (moved.size === before) break;
  if (round > 50) throw new Error("closure did not converge");
}
void movedUses;

const exportDeclarations = sf.statements.filter((s): s is ts.ExportDeclaration => ts.isExportDeclaration(s) && !s.moduleSpecifier && !!s.exportClause && ts.isNamedExports(s.exportClause));
const publicNames = new Map<string, string>();
const movedExportSplits = new Map<ts.ExportDeclaration, ts.ExportSpecifier[]>();
const movedLocalNames = new Set<string>();
for (const st of moved) for (const n of declaredNames(st)) movedLocalNames.add(n);
for (const st of moved) {
  if (isExported(st)) for (const n of declaredNames(st)) publicNames.set(n, n);
}
for (const ed of exportDeclarations) {
  const specifiers = (ed.exportClause as ts.NamedExports).elements;
  const taken = specifiers.filter((e) => movedLocalNames.has((e.propertyName ?? e.name).text));
  if (taken.length === 0) continue;
  movedExportSplits.set(ed, [...taken]);
  for (const e of taken) publicNames.set((e.propertyName ?? e.name).text, e.name.text);
}

const publicSet0 = new Set([...publicNames.values()]);
const locals = new Map<string, string>();
for (const [local, pub] of publicNames) locals.set(pub, local);

const leadingStart = (node: ts.Node): number => {
  const ranges = ts.getLeadingCommentRanges(sourceText, node.getFullStart()) ?? [];
  let start = node.getStart(sf);
  for (let i = ranges.length - 1; i >= 0; i -= 1) {
    const r = ranges[i]!;
    const text = sourceText.slice(r.pos, r.end);
    if (/^\/\/\s*#(end)?region/.test(text)) break;
    start = r.pos;
  }
  return start;
};

const targetDir = dirname(targetPath);
const sourceDir = dirname(sourcePath);
const retarget = (specText: string, fromDir: string, toDir: string): string => {
  if (!specText.startsWith(".")) return specText;
  let out = relative(toDir, resolve(fromDir, specText)).split(sep).join("/");
  if (!out.startsWith(".")) out = `./${out}`;
  return out;
};
const slice = (a: number, b: number) => sourceText.slice(a, b);

type Edit = { start: number; end: number; text: string };
const targetChunks: { pos: number; text: string }[] = [];
const sourceEdits: Edit[] = [];

const rewriteInnerSpecifiers = (node: ts.Node, base: number, text: string): string => {
  const edits: Edit[] = [];
  const visit = (n: ts.Node) => {
    if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) {
      const lit = n.argument.literal;
      const next = retarget(lit.text, sourceDir, targetDir);
      if (next !== lit.text) edits.push({ start: lit.getStart(sf) + 1 - base, end: lit.getEnd() - 1 - base, text: next });
    }
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0])) {
      const lit = n.arguments[0];
      const next = retarget(lit.text, sourceDir, targetDir);
      if (next !== lit.text) edits.push({ start: lit.getStart(sf) + 1 - base, end: lit.getEnd() - 1 - base, text: next });
    }
    if (ts.isMetaProperty(n)) console.log(`  note: moved code uses ${n.getText(sf)}`);
    ts.forEachChild(n, visit);
  };
  visit(node);
  let out = text;
  for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  return out;
};

const sortedMoved = [...moved].sort((a, b) => a.getStart(sf) - b.getStart(sf));
for (const st of sortedMoved) {
  const start = leadingStart(st);
  const end = st.getEnd();
  let text = slice(start, end);
  text = rewriteInnerSpecifiers(st, start, text);
  const names = declaredNames(st);
  const needsExport = names.some((n) => refsOfMovedFromRemaining.has(n) && !publicNames.has(n));
  if (needsExport && !isExported(st)) {
    const off = st.getStart(sf) - start;
    text = `${text.slice(0, off)}export ${text.slice(off)}`;
  }
  targetChunks.push({ pos: st.getStart(sf), text });
  let removeEnd = end;
  while (sourceText[removeEnd] === " " || sourceText[removeEnd] === "\t") removeEnd += 1;
  if (sourceText[removeEnd] === "\n") removeEnd += 1;
  let removeStart = start;
  while (removeStart > 0 && (sourceText[removeStart - 1] === " " || sourceText[removeStart - 1] === "\t")) removeStart -= 1;
  sourceEdits.push({ start: removeStart, end: removeEnd, text: "" });
}

const movedPublicFromRemainingNeeds = new Set<string>();
for (const ed of exportDeclarations) {
  const taken = movedExportSplits.get(ed);
  if (!taken) continue;
  const specifiers = (ed.exportClause as ts.NamedExports).elements;
  const rest = specifiers.filter((e) => !taken.includes(e));
  const typeOnly = ed.isTypeOnly ? "type " : "";
  const render = (list: readonly ts.ExportSpecifier[]) => `export ${typeOnly}{ ${list.map((e) => e.getText(sf)).join(", ")} };`;
  targetChunks.push({ pos: ed.getStart(sf), text: render(taken) });
  const start = leadingStart(ed);
  const end = ed.getEnd();
  if (rest.length === 0) {
    let removeEnd = end;
    if (sourceText[removeEnd] === "\n") removeEnd += 1;
    sourceEdits.push({ start, end: removeEnd, text: "" });
  } else sourceEdits.push({ start: ed.getStart(sf), end, text: render(rest) });
  for (const e of taken) movedPublicFromRemainingNeeds.add((e.propertyName ?? e.name).text);
}

type ImportRecord = { typeOnly: boolean; spec: string; defaultName?: string; namespace?: string; named: string[] };
const importRecords: ImportRecord[] = [];
const candidatesFor = (fromFile: string, specifier: string): string[] => {
  const base = resolve(dirname(fromFile), specifier);
  return [base, `${base}.ts`, `${base}.tsx`, `${base}/🟦️.ts`, `${base}/🟦️.tsx`];
};
const sourceImportEdits: Edit[] = [];
const bindingInfo = (decl: ts.ImportDeclaration) => {
  const c = decl.importClause!;
  const items: { sym: ts.Symbol; text: string; kind: "default" | "namespace" | "named" }[] = [];
  const add = (id: ts.Identifier, text: string, kind: "default" | "namespace" | "named") => {
    const sym = checker.getSymbolAtLocation(id);
    if (sym) items.push({ sym, text, kind });
  };
  if (c.name) add(c.name, c.name.text, "default");
  if (c.namedBindings) {
    if (ts.isNamespaceImport(c.namedBindings)) add(c.namedBindings.name, `* as ${c.namedBindings.name.text}`, "namespace");
    else for (const e of c.namedBindings.elements) add(e.name, e.getText(sf), "named");
  }
  return items;
};
for (const st of sf.statements) {
  if (!ts.isImportDeclaration(st) || !st.importClause) continue;
  const items = bindingInfo(st);
  const forTarget = items.filter((i) => usedImportBindings.get(i.sym)?.inMoved);
  const keepInSource = items.filter((i) => {
    const u = usedImportBindings.get(i.sym);
    return !u || u.inRemaining;
  });
  const dropped = items.filter((i) => !keepInSource.includes(i));
  if (forTarget.length > 0) {
    const specText = (st.moduleSpecifier as ts.StringLiteral).text;
    const selfImport = specText.startsWith(".") && candidatesFor(sourcePath, specText).includes(targetPath);
    if (!selfImport) {
      const next = retarget(specText, sourceDir, targetDir);
      importRecords.push({
        typeOnly: st.importClause.isTypeOnly,
        spec: next,
        defaultName: forTarget.find((i) => i.kind === "default")?.text,
        namespace: forTarget.find((i) => i.kind === "namespace")?.text,
        named: forTarget.filter((i) => i.kind === "named").map((i) => i.text),
      });
    }
  }
  if (dropped.length > 0) {
    if (keepInSource.length === 0) {
      let removeEnd = st.getEnd();
      if (sourceText[removeEnd] === "\n") removeEnd += 1;
      sourceImportEdits.push({ start: st.getStart(sf), end: removeEnd, text: "" });
    } else if (st.importClause.namedBindings && ts.isNamedImports(st.importClause.namedBindings)) {
      const c = st.importClause;
      const keepNamed = keepInSource.filter((i) => i.kind === "named").map((i) => i.text);
      const keepDefault = keepInSource.filter((i) => i.kind === "default").map((i) => i.text);
      const parts: string[] = [];
      if (keepDefault.length) parts.push(keepDefault[0]!);
      if (keepNamed.length) parts.push(`{ ${keepNamed.join(", ")} }`);
      const quote = (st.moduleSpecifier as ts.StringLiteral).getText(sf);
      const attrs = st.attributes ? ` ${st.attributes.getText(sf)}` : "";
      sourceImportEdits.push({ start: st.getStart(sf), end: st.getEnd(), text: `import ${c.isTypeOnly ? "type " : ""}${parts.join(", ")} from ${quote}${attrs};` });
    } else if (keepInSource.length > 0) {
      console.log(`  warning: partially-dropped default/namespace import left alone: ${st.getText(sf).slice(0, 80)}`);
    }
  }
}

const newImportForSource: string[] = [];
const targetModuleSpecForSource = (() => {
  let out = relative(sourceDir, targetPath).split(sep).join("/");
  if (!out.startsWith(".")) out = `./${out}`;
  return out;
})();
const typeOnlyDeclaration = (name: string): boolean => {
  const sts = statementOf.get(name) ?? [];
  return sts.length > 0 && sts.every((s) => ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s));
};
const remainingNeeds = [...refsOfMovedFromRemaining].filter((n) => movedLocalNames.has(n));
if (remainingNeeds.length > 0) {
  const parts = remainingNeeds.sort().map((n) => {
    const pub = publicNames.get(n);
    const body = pub && pub !== n ? `${pub} as ${n}` : n;
    return `${typeOnlyDeclaration(n) ? "type " : ""}${body}`;
  });
  newImportForSource.push(`import { ${parts.join(", ")} } from "${targetModuleSpecForSource}";`);
}
if (spec.keepExports && spec.keepExports !== "none") {
  const pubs = [...publicNames.entries()].map(([local, pub]) => `${typeOnlyDeclaration(local) ? "type " : ""}${pub}`);
  if (spec.keepExports === "star") newImportForSource.push(`export * from "${targetModuleSpecForSource}";`);
  else if (pubs.length) newImportForSource.push(`export { ${pubs.join(", ")} } from "${targetModuleSpecForSource}";`);
}
const lastImport = [...sf.statements].reverse().find((s) => ts.isImportDeclaration(s));
const insertAt = lastImport ? lastImport.getEnd() : 0;
if (newImportForSource.length > 0) sourceEdits.push({ start: insertAt, end: insertAt, text: (lastImport ? "\n" : "") + newImportForSource.join("\n") + (lastImport ? "" : "\n") });

const applyEdits = (text: string, edits: Edit[]): string => {
  const sorted = [...edits].sort((a, b) => b.start - a.start || b.end - a.end);
  let out = text;
  for (const e of sorted) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  return out;
};
const dropEmptyRegions = (text: string): string => text.replace(/^[ \t]*\/\/ ?#region ([^\n]*)\n(?:[ \t]*\n)*[ \t]*\/\/ ?#endregion [^\n]*\n?/gm, "");
const newSourceText = dropEmptyRegions(applyEdits(sourceText, [...sourceEdits, ...sourceImportEdits]));

targetChunks.sort((a, b) => a.pos - b.pos);
const targetExisting = existsSync(targetPath) ? readFileSync(targetPath, "utf8") : undefined;
const body = targetChunks.map((c) => c.text.replace(/^\s+/, "").replace(/\s+$/, "")).join("\n\n");
const renderImport = (r: ImportRecord): string => {
  const parts: string[] = [];
  if (r.defaultName) parts.push(r.defaultName);
  if (r.namespace) parts.push(r.namespace);
  if (r.named.length) parts.push(`{ ${r.named.join(", ")} }`);
  return `import ${r.typeOnly ? "type " : ""}${parts.join(", ")} from "${r.spec}";`;
};
const localNameOf = (specifierText: string): string => specifierText.replace(/^type\s+/, "").split(/\s+as\s+/).pop()!.trim();
const mergedRecords: ImportRecord[] = [];
for (const r of importRecords) {
  const same = mergedRecords.find((m) => m.spec === r.spec && m.typeOnly === r.typeOnly && !m.namespace && !r.namespace && (!m.defaultName || !r.defaultName));
  if (same) {
    same.defaultName ??= r.defaultName;
    for (const n of r.named) if (!same.named.some((x) => localNameOf(x) === localNameOf(n))) same.named.push(n);
  } else mergedRecords.push({ ...r, named: [...r.named] });
}
let newTargetText: string;
if (targetExisting === undefined) {
  const lines = mergedRecords.map(renderImport);
  newTargetText = `/** ${spec.doc} */\n${lines.length ? `${lines.join("\n")}\n\n` : "\n"}${body}\n`;
} else {
  const tsf = ts.createSourceFile(targetPath, targetExisting, ts.ScriptTarget.ESNext, true, kindOf(targetPath));
  const tEdits: Edit[] = [];
  const appended: string[] = [];
  const tImports = tsf.statements.filter((x): x is ts.ImportDeclaration => ts.isImportDeclaration(x));
  for (const d of tImports) {
    const dspec = (d.moduleSpecifier as ts.StringLiteral).text;
    if (!dspec.startsWith(".") || !candidatesFor(targetPath, dspec).includes(sourcePath)) continue;
    const c = d.importClause;
    if (!c?.namedBindings || !ts.isNamedImports(c.namedBindings)) continue;
    const keep = c.namedBindings.elements.filter((e) => !publicSet0.has((e.propertyName ?? e.name).text));
    if (keep.length === c.namedBindings.elements.length) continue;
    if (keep.length === 0 && !c.name) {
      let end = d.getEnd();
      if (targetExisting[end] === "\n") end += 1;
      tEdits.push({ start: d.getStart(tsf), end, text: "" });
    } else tEdits.push({ start: d.getStart(tsf), end: d.getEnd(), text: `import ${c.isTypeOnly ? "type " : ""}${[c.name?.text, keep.length ? `{ ${keep.map((e) => e.getText(tsf)).join(", ")} }` : undefined].filter(Boolean).join(", ")} from ${d.moduleSpecifier.getText(tsf)};` });
  }
  for (const r of mergedRecords) {
    const existing = tImports.find((d) => (d.moduleSpecifier as ts.StringLiteral).text === r.spec && (d.importClause?.isTypeOnly ?? false) === r.typeOnly && d.importClause?.namedBindings && ts.isNamedImports(d.importClause.namedBindings));
    if (existing && !r.namespace && !r.defaultName) {
      const bindings = existing.importClause!.namedBindings as ts.NamedImports;
      const have = new Set(bindings.elements.map((e) => e.name.text));
      const add = r.named.filter((n) => !have.has(localNameOf(n)));
      if (add.length === 0) continue;
      const all = [...bindings.elements.map((e) => e.getText(tsf)), ...add];
      const c = existing.importClause!;
      tEdits.push({ start: existing.getStart(tsf), end: existing.getEnd(), text: `import ${c.isTypeOnly ? "type " : ""}${[c.name?.text, `{ ${all.join(", ")} }`].filter(Boolean).join(", ")} from ${existing.moduleSpecifier.getText(tsf)};` });
    } else {
      const alreadyAll = r.named.length > 0 && !r.defaultName && !r.namespace && r.named.every((n) => tImports.some((d) => d.importClause?.namedBindings && ts.isNamedImports(d.importClause.namedBindings) && d.importClause.namedBindings.elements.some((e) => e.name.text === localNameOf(n))));
      if (!alreadyAll) appended.push(renderImport(r));
    }
  }
  const lastTImport = [...tImports].reverse()[0];
  const insertPos = lastTImport ? lastTImport.getEnd() : (/^\s*\/\*\*[\s\S]*?\*\/\s*\n/.exec(targetExisting)?.[0].length ?? 0);
  if (appended.length) tEdits.push({ start: insertPos, end: insertPos, text: (lastTImport ? "\n" : "") + appended.join("\n") + (lastTImport ? "" : "\n") });
  newTargetText = `${applyEdits(targetExisting, tEdits).replace(/\s+$/, "")}\n\n${body}\n`;
}

console.log(`moving ${sortedMoved.length} statements: ${[...movedLocalNames].join(", ")}`);
console.log(`public names: ${[...publicNames.entries()].map(([l, p]) => (l === p ? l : `${l} as ${p}`)).join(", ")}`);
console.log(`source keeps importing: ${remainingNeeds.join(", ") || "-"}`);

const toRepoScan = (): string[] => {
  const listed = execFileSync("git", ["ls-files", "-z", "--", "*.ts", "*.tsx", "*.mts", "*.cts"], { cwd: repo, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter(Boolean);
  return listed.filter((f) => !f.startsWith(".🧬semio/") && !f.endsWith(".d.ts"));
};
const sourceBase = sourcePath.split(sep).slice(-2).join("/");
const candidateTail = sourcePath.endsWith("🟦️.ts") || sourcePath.endsWith("🟦️.tsx") ? sourcePath.split(sep).slice(-2, -1)[0]! : sourcePath.split(sep).pop()!.replace(/\.(ts|tsx)$/, "");
void sourceBase;
const resolvesToSource = (fromFile: string, specifier: string): boolean => {
  if (!specifier.startsWith(".")) return false;
  const base = resolve(dirname(fromFile), specifier);
  const candidates = [base, `${base}.ts`, `${base}.tsx`, `${base}/🟦️.ts`, `${base}/🟦️.tsx`];
  return candidates.includes(sourcePath);
};
type FileResult = { path: string; text: string };
const importerResults: FileResult[] = [];
const manual: string[] = [];
const publicSet = new Set([...publicNames.values()]);
for (const f of toRepoScan()) {
  const abs = resolve(repo, f);
  if (abs === sourcePath || abs === targetPath) continue;
  let text: string;
  try {
    text = readFileSync(abs, "utf8");
  } catch {
    continue;
  }
  let touches = false;
  for (const m of text.matchAll(/["'](\.{1,2}\/[^"'\n]*)["']/g)) {
    if (resolvesToSource(abs, m[1]!)) {
      touches = true;
      break;
    }
  }
  if (!touches) continue;
  const file = ts.createSourceFile(abs, text, ts.ScriptTarget.ESNext, true, kindOf(abs));
  const edits: Edit[] = [];
  const addImports: string[] = [];
  const visit = (node: ts.Node) => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && resolvesToSource(abs, node.moduleSpecifier.text)) {
      const nextSpec = (() => {
        let out = relative(dirname(abs), targetPath).split(sep).join("/");
        if (!out.startsWith(".")) out = `./${out}`;
        return out;
      })();
      if (ts.isImportDeclaration(node)) {
        const c = node.importClause;
        if (!c) return;
        if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) {
          manual.push(`${f}: namespace import of source (${c.namedBindings.name.text}); check member uses`);
          return;
        }
        if (!c.namedBindings || !ts.isNamedImports(c.namedBindings)) return;
        const take = c.namedBindings.elements.filter((e) => publicSet.has((e.propertyName ?? e.name).text));
        if (take.length === 0) return;
        const keep = c.namedBindings.elements.filter((e) => !take.includes(e));
        const typeKw = c.isTypeOnly ? "type " : "";
        const moved = `import ${typeKw}{ ${take.map((e) => e.getText(file)).join(", ")} } from "${nextSpec}";`;
        const hasDefault = !!c.name;
        const remaining = keep.length > 0 || hasDefault ? `import ${typeKw}${[c.name?.text, keep.length ? `{ ${keep.map((e) => e.getText(file)).join(", ")} }` : undefined].filter(Boolean).join(", ")} from ${node.moduleSpecifier.getText(file)};\n` : "";
        edits.push({ start: node.getStart(file), end: node.getEnd(), text: `${remaining}${moved}` });
      } else {
        const clause = node.exportClause;
        if (!clause) {
          if (spec.keepExports && spec.keepExports !== "none") return;
          manual.push(`${f}: export * from source; moved public names disappear from this barrel`);
          edits.push({ start: node.getEnd(), end: node.getEnd(), text: `\nexport * from "${nextSpec}";` });
          return;
        }
        if (!ts.isNamedExports(clause)) return;
        const take = clause.elements.filter((e) => publicSet.has((e.propertyName ?? e.name).text));
        if (take.length === 0) return;
        const keep = clause.elements.filter((e) => !take.includes(e));
        const typeKw = node.isTypeOnly ? "type " : "";
        const movedExp = `export ${typeKw}{ ${take.map((e) => e.getText(file)).join(", ")} } from "${nextSpec}";`;
        const remaining = keep.length > 0 ? `export ${typeKw}{ ${keep.map((e) => e.getText(file)).join(", ")} } from ${node.moduleSpecifier.getText(file)};\n` : "";
        edits.push({ start: node.getStart(file), end: node.getEnd(), text: `${remaining}${movedExp}` });
      }
      return;
    }
    if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal) && resolvesToSource(abs, node.argument.literal.text) && (!node.qualifier || publicSet.has(node.qualifier.getText(file).split(".")[0]!)) && !(spec.keepExports && spec.keepExports !== "none")) {
      manual.push(`${f}: import("...") type query of source at ${file.getLineAndCharacterOfPosition(node.getStart(file)).line + 1}`);
    }
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0]) && resolvesToSource(abs, node.arguments[0].text)) {
      manual.push(`${f}: dynamic import of source at ${file.getLineAndCharacterOfPosition(node.getStart(file)).line + 1}`);
    }
    ts.forEachChild(node, visit);
  };
  visit(file);
  void addImports;
  if (edits.length > 0) importerResults.push({ path: abs, text: applyEdits(text, edits) });
}
console.log(`importers rewritten: ${importerResults.length}`);
for (const r of importerResults) console.log(`  ${rel(r.path)}`);
for (const m of manual) console.log(`  MANUAL: ${m}`);

if (apply) {
  mkdirSync(dirname(targetPath), { recursive: true });
  writeFileSync(targetPath, newTargetText);
  writeFileSync(sourcePath, newSourceText);
  for (const r of importerResults) writeFileSync(r.path, r.text);
  console.log("APPLIED");
} else {
  console.log("--- target preview (first 40 lines) ---");
  console.log(newTargetText.split("\n").slice(0, 40).join("\n"));
  console.log("DRY RUN (pass --apply)");
}
