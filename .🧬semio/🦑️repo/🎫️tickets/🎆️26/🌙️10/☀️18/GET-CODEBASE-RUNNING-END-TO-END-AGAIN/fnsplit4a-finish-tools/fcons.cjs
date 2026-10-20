// 🔗 Consumer rewrite for exported declarations that a phase split moved into the shared file. usage: node fcons.cjs <spec.json> <snapshot> [--write]
// Finds importers by `git grep -l -F <name>` (one fixed string per moved name, index only), rewrites their import of the root so the moved names come from the shared file (no re-export left behind), copies each consumer into the overlay first (never writes through a symlink). Prints CONSUMERS: <repo-relative paths>.
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path"), cp = require("child_process");
const OV = "/private/tmp/fnsplit4a-finish/ov", REAL = "/Users/ueli/Documents/semio";
const [specPath, snapshot, flag, listFile] = process.argv.slice(2);
const WRITE = flag === "--write";
const spec = JSON.parse(fs.readFileSync(specPath, "utf8"));
const rootOv = spec.file, rootRel = path.relative(OV, rootOv), rootReal = path.join(REAL, rootRel);
const ext = path.extname(rootOv);
const parse = (f, t) => ts.createSourceFile(f, t, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const exportsOf = (sf) => {
  const out = new Map();
  for (const st of sf.statements) {
    if (!st.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)) continue;
    if (ts.isVariableStatement(st)) st.declarationList.declarations.forEach((d) => ts.isIdentifier(d.name) && out.set(d.name.text, false));
    else if (st.name) out.set(st.name.text, ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st));
  }
  return out;
};
const before = exportsOf(parse(rootOv, fs.readFileSync(snapshot, "utf8")));
const after = exportsOf(parse(rootOv, fs.readFileSync(rootOv, "utf8")));
const moved = [...before].filter(([n]) => !after.has(n));
if (!moved.length) { console.log("NO-MOVED-EXPORTS"); process.exit(0); }
const sharedOv = path.join(path.dirname(rootOv), spec.shared.dir, "🟦️" + ext);
const sharedSf = parse(sharedOv, fs.readFileSync(sharedOv, "utf8"));
const inShared = exportsOf(sharedSf);
for (const [n] of moved) if (!inShared.has(n)) { console.log("ERROR moved export not in shared: " + n); process.exit(3); }
const files = new Map();
for (const [n] of moved) {
  const r = cp.spawnSync("git", ["grep", "-l", "-F", "-e", n, "--", "*.ts", "*.tsx"], { cwd: REAL, encoding: "utf8" });
  for (const f of r.stdout.split("\n").filter(Boolean)) files.set(f, true);
}
const changed = [], notes = [];
const isRoot = (consumerReal, spec_) => { const p = path.resolve(path.dirname(consumerReal), spec_); return [p, p + ".ts", p + ".tsx"].includes(rootReal); };
for (const rel of files.keys()) {
  const real = path.join(REAL, rel);
  if (real === rootReal || rel.startsWith(".")) continue;
  const inOv = path.join(OV, rel);
  const ovExists = fs.existsSync(inOv) && !fs.lstatSync(inOv).isSymbolicLink() && fs.realpathSync(path.dirname(inOv)).startsWith(OV);
  const text = fs.readFileSync(ovExists ? inOv : real, "utf8");
  const sf = parse(real, text);
  let out = text, edits = [];
  for (const st of sf.statements) {
    if (!(ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier) || !st.moduleSpecifier.text.startsWith(".") || !isRoot(real, st.moduleSpecifier.text)) continue;
    const isExp = ts.isExportDeclaration(st);
    if (isExp && !(st.exportClause && ts.isNamedExports(st.exportClause))) { notes.push("ERROR STAR-REEXPORT-OF-ROOT " + rel); continue; }
    const nb = isExp ? st.exportClause : st.importClause?.namedBindings;
    if (!nb || !(isExp ? ts.isNamedExports(nb) : ts.isNamedImports(nb))) continue;
    const mv = nb.elements.filter((e) => moved.some(([n]) => n === (e.propertyName ?? e.name).text));
    if (!mv.length) continue;
    const keep = nb.elements.filter((e) => !mv.includes(e));
    const q = st.moduleSpecifier.getText(sf)[0];
    let r = path.relative(path.dirname(real), path.join(path.dirname(rootReal), spec.shared.dir, "🟦️" + ext)).split(path.sep).join("/");
    if (!r.startsWith(".")) r = "./" + r;
    const typeOnly = isExp ? !!st.isTypeOnly : !!st.importClause.isTypeOnly;
    const kw = isExp ? "export" : "import";
    const fresh = `${kw} ${typeOnly ? "type " : ""}{ ${mv.map((e) => e.getText(sf)).join(", ")} } from ${q}${r}${q};`;
    let old = "";
    if (keep.length || (!isExp && st.importClause.name)) {
      const bits = [];
      if (!isExp && st.importClause.name) bits.push(st.importClause.name.text);
      if (keep.length) bits.push(`{ ${keep.map((e) => e.getText(sf)).join(", ")} }`);
      old = `${kw} ${typeOnly ? "type " : ""}${bits.join(", ")} from ${st.moduleSpecifier.getText(sf)};\n`;
    }
    edits.push({ s: st.getStart(sf), e: st.getEnd(), t: old + fresh });
  }
  if (!edits.length) continue;
  for (const e of edits.sort((a, b) => b.s - a.s)) out = out.slice(0, e.s) + e.t + out.slice(e.e);
  changed.push(rel);
  if (WRITE) {
    if (!ovExists) {
      cp.spawnSync("python3", [path.join(path.dirname(process.argv[1]), "mkov.py"), OV, REAL, "--add", path.dirname(rel)], { stdio: "inherit" });
    }
    const dst = path.join(OV, rel);
    if (!fs.realpathSync(path.dirname(dst)).startsWith(OV)) { console.log("ERROR would write through a symlink: " + rel); process.exit(4); }
    fs.writeFileSync(dst, out);
  }
}
for (const n of notes) console.log(n);
console.log("MOVED " + moved.map(([n]) => n).join(","));
console.log("CONSUMERS " + changed.length);
if (listFile) fs.writeFileSync(listFile, changed.join("\n") + (changed.length ? "\n" : ""));
