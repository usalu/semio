// usage: BARREL=... node rewrite.cjs <owners.json> <scan.json> [APPLY]
// Re-points every named import / named re-export of the barrel to the owner module; merges into existing declarations of the same module.
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const BARREL = process.env.BARREL;
const RENDERER = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/";
const owners = JSON.parse(fs.readFileSync(process.argv[2], "utf8")).out;
const scan = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const APPLY = process.argv[4] === "APPLY";
const barrelDir = path.dirname(BARREL);
const MAXW = 140;
const kind = (p) => (p.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const resolveFile = (from, spec) => {
  const base = path.resolve(path.dirname(from), spec.replace(/\?.*$/, ""));
  for (const c of [base, base + ".ts", base + ".tsx", path.join(base, "🟦️.ts"), path.join(base, "🟦️.tsx")]) if (fs.existsSync(c) && fs.statSync(c).isFile()) return c;
  return null;
};
const isBarrelSpec = (from, spec) => spec.startsWith(".") && [BARREL, BARREL.replace(/x$/, ""), barrelDir, path.join(barrelDir, "🟦️")].includes(path.resolve(path.dirname(from), spec.replace(/\?.*$/, "")));
const ownerTarget = (name) => {
  const o = owners[name];
  if (!o) return null;
  if (o.kind === "external") return { key: "ext:" + o.spec, ext: o.spec, imported: o.original };
  let file = o.file;
  if (!file.startsWith(RENDERER)) {
    const via = resolveFile(BARREL, o.viaSpec ?? "");
    file = via ?? resolveFile(BARREL, path.relative(barrelDir, o.viaFile));
    if (!file) throw new Error("cannot route " + name);
    return { key: "f:" + file, file, imported: name };
  }
  return { key: "f:" + file, file, imported: o.original };
};
const relFrom = (fromDir, file) => { let r = path.relative(fromDir, file).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const fmtDecl = (head, items, spec, q, multi, trailing) => {
  const one = `${head}{ ${items.join(", ")} } from ${q}${spec}${q};`;
  if (!multi && one.length <= MAXW) return one;
  return `${head}{\n${items.map((i) => "  " + i).join(",\n")}${trailing ? "," : ""}\n} from ${q}${spec}${q};`;
};
const report = { files: [], manual: [], errors: [], namesRepointed: new Set(), declsRemoved: 0 };
const writes = [];
for (const imp of scan.importers) {
  const p = imp.file;
  if (!imp.sites.length || p === BARREL || !fs.existsSync(p)) continue;
  const t = fs.readFileSync(p, "utf8");
  const sf = ts.createSourceFile(p, t, ts.ScriptTarget.Latest, true, kind(p));
  const dir = path.dirname(p);
  const barrelDecls = [], others = [];
  const manual = [];
  const walk = (n) => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      if (isBarrelSpec(p, n.moduleSpecifier.text)) barrelDecls.push(n); else if (ts.isImportDeclaration(n)) others.push(n);
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && isBarrelSpec(p, n.arguments[0].text)) {
      manual.push(`${p}:${sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1} ${n.expression.getText(sf)}() of barrel`);
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && isBarrelSpec(p, n.argument.literal.text)) {
      manual.push(`${p}:${sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1} import() type of barrel`);
    }
    ts.forEachChild(n, walk);
  };
  walk(sf);
  report.manual.push(...manual);
  const edits = [];
  const generated = new Map();
  const mergeAdds = new Map();
  const perDeclOutput = new Map();
  let bad = false;
  const existingFor = (key, declType) => others.find((d) => {
    const c = d.importClause;
    if (!c || c.name || !c.namedBindings || !ts.isNamedImports(c.namedBindings) || !!c.isTypeOnly !== declType) return false;
    const sp = d.moduleSpecifier.text;
    return key.startsWith("ext:") ? "ext:" + sp === key : sp.startsWith(".") && "f:" + resolveFile(p, sp) === key;
  });
  const existingLocals = (d) => new Set(d.importClause.namedBindings.elements.map((e) => e.name.text + "|" + (e.propertyName ?? e.name).text));
  for (const d of barrelDecls) {
    const isImp = ts.isImportDeclaration(d);
    const clause = isImp ? d.importClause : d.exportClause;
    if (isImp && !d.importClause) { report.manual.push(`${p}: side-effect import of barrel`); bad = true; continue; }
    if (!isImp && !d.exportClause) { report.manual.push(`${p}: export * from barrel`); bad = true; continue; }
    const nb = isImp ? clause.namedBindings : clause;
    if ((isImp && clause.name) || !nb || !(ts.isNamedImports(nb) || ts.isNamedExports(nb))) { report.manual.push(`${p}: default/namespace use of barrel`); bad = true; continue; }
    const declType = isImp ? !!clause.isTypeOnly : !!d.isTypeOnly;
    const q = d.moduleSpecifier.getText(sf)[0];
    const out = [];
    for (const e of nb.elements) {
      const exported = (e.propertyName ?? e.name).text;
      const ot = ownerTarget(exported);
      if (!ot) { report.errors.push(`${p}: unknown name ${exported}`); bad = true; continue; }
      const local = e.name.text;
      const spelled = ot.imported === local ? local : `${ot.imported} as ${local}`;
      const text = (e.isTypeOnly ? "type " : "") + spelled;
      report.namesRepointed.add(exported);
      const spec = ot.ext ?? relFrom(dir, ot.file);
      const gkey = (isImp ? "i" : "e") + "|" + ot.key + "|" + declType;
      if (isImp) {
        const ex = existingFor(ot.key, declType);
        if (ex && existingLocals(ex).has(local + "|" + ot.imported)) continue;
        if (ex) {
          if (!mergeAdds.has(ex)) mergeAdds.set(ex, []);
          const list = mergeAdds.get(ex);
          if (!list.includes(text)) list.push(text);
          continue;
        }
      }
      let g = generated.get(gkey);
      if (!g) { g = { head: (isImp ? "import " : "export ") + (declType ? "type " : ""), spec, q, items: [], owner: d }; generated.set(gkey, g); }
      if (!g.items.includes(text)) g.items.push(text);
    }
  }
  if (bad) continue;
  const byOwner = new Map();
  for (const g of generated.values()) { if (!byOwner.has(g.owner)) byOwner.set(g.owner, []); byOwner.get(g.owner).push(g); }
  for (const d of barrelDecls) {
    const gs = byOwner.get(d) ?? [];
    const ls = t.lastIndexOf("\n", d.getStart(sf));
    const indent = t.slice(ls + 1, d.getStart(sf));
    const multi = /\n/.test(d.getText(sf));
    const nbd = ts.isImportDeclaration(d) ? d.importClause.namedBindings : d.exportClause;
    const text = gs.map((g) => fmtDecl(g.head, g.items, g.spec, g.q, multi && gs.length === 1, multi && nbd.elements.hasTrailingComma)).join("\n" + indent);
    if (text) edits.push({ start: d.getStart(sf), end: d.getEnd(), text });
    else {
      let end = d.getEnd();
      if (t[end] === "\n" && (ls + 1 === d.getStart(sf) || /^\s*$/.test(t.slice(ls + 1, d.getStart(sf))))) end += 1;
      const start = /^\s*$/.test(t.slice(ls + 1, d.getStart(sf))) ? ls + 1 : d.getStart(sf);
      edits.push({ start, end, text: "" });
      report.declsRemoved++;
    }
  }
  for (const [d, adds] of mergeAdds) {
    const nb = d.importClause.namedBindings;
    const items = [...nb.elements.map((e) => e.getText(sf)), ...adds];
    const multi = /\n/.test(d.getText(sf));
    const q = d.moduleSpecifier.getText(sf)[0];
    const head = "import " + (d.importClause.isTypeOnly ? "type " : "");
    edits.push({ start: d.getStart(sf), end: d.getEnd(), text: fmtDecl(head, items, d.moduleSpecifier.text, q, multi, multi && nb.elements.hasTrailingComma) });
  }
  let r = t;
  for (const e of edits.sort((a, b) => b.start - a.start)) r = r.slice(0, e.start) + e.text + r.slice(e.end);
  writes.push({ path: p, before: t, text: r });
  report.files.push(p);
}
const summary = { files: report.files.length, names: report.namesRepointed.size, declsRemovedOnly: report.declsRemoved, manual: report.manual, errors: report.errors };
console.log(JSON.stringify(summary, null, 1));
if (process.env.DRY_OUT) { fs.mkdirSync(process.env.DRY_OUT, { recursive: true }); writes.forEach((w, i) => { fs.writeFileSync(process.env.DRY_OUT + "/" + i + ".before", w.before); fs.writeFileSync(process.env.DRY_OUT + "/" + i + ".after", w.text); fs.appendFileSync(process.env.DRY_OUT + "/index.txt", i + "\t" + w.path + "\n"); }); }
if (APPLY) {
  for (const w of writes) {
    if (fs.readFileSync(w.path, "utf8") !== w.before) { console.error("CHANGED UNDER ME, skipped: " + w.path); continue; }
    fs.writeFileSync(w.path, w.text);
  }
  fs.writeFileSync(process.env.REPORT_OUT, JSON.stringify({ files: report.files, names: [...report.namesRepointed].sort() }, null, 1));
  console.log("APPLIED " + writes.length);
}
