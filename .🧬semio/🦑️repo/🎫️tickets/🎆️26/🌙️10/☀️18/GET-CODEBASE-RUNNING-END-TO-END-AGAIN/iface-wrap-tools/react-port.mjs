import fs from "node:fs";
import path from "node:path";
import ts from "typescript";
import { walk } from "./react-scan.mjs";

const ROOT = "/Users/ueli/Documents/semio/";
export const ELEMENTS = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements";
export const PORT_ABS = ELEMENTS + "/⚛️react-port/🟦️.ts";

export function portSpecifier(file) {
  const rel = path.relative(path.dirname(file), PORT_ABS).split(path.sep).join("/");
  return rel.startsWith(".") ? rel : "./" + rel;
}

function reactExports() {
  const probe = ELEMENTS + "/__react_probe__.ts";
  const options = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, strict: true, skipLibCheck: true, esModuleInterop: true, types: [] };
  const host = ts.createCompilerHost(options);
  const original = host.getSourceFile.bind(host);
  host.getSourceFile = (name, ...rest) => (name === probe ? ts.createSourceFile(name, 'import * as R from "react"; import * as D from "react-dom/client"; export { R, D };', ts.ScriptTarget.ESNext) : original(name, ...rest));
  host.fileExists = ((exists) => (name) => name === probe || exists(name))(host.fileExists.bind(host));
  const program = ts.createProgram([probe], options, host);
  const checker = program.getTypeChecker();
  const source = program.getSourceFile(probe);
  const out = {};
  for (const [key, specifier] of [["react", "react"], ["dom", "react-dom/client"]]) {
    const statement = source.statements.find((node) => ts.isImportDeclaration(node) && node.moduleSpecifier.text === specifier);
    const symbol = checker.getSymbolAtLocation(statement.moduleSpecifier);
    if (!symbol) throw Error("module not resolved: " + specifier);
    out[key] = new Map(checker.getExportsOfModule(symbol).map((exported) => [exported.name, (exported.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(exported) : exported).flags]));
  }
  return out;
}

export function analyseText(file, text, exportsOf, used, bare) {
  const source = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const port = portSpecifier(file);
  const fileEdits = [];
  const bindings = new Set();
  const note = (bucket, name) => {
    if (!exportsOf[bucket].has(name)) throw Error(`${bucket} does not export ${name}`);
    used[bucket].set(name, true);
  };
  const specifierOf = (literal) => (literal.text === "react" ? "react" : literal.text === "react-dom/client" ? "dom" : null);
  const replaceLiteral = (literal) => fileEdits.push([literal.getStart(source), literal.getEnd(), JSON.stringify(port)]);
  const visit = (node) => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && specifierOf(node.moduleSpecifier)) {
      const bucket = specifierOf(node.moduleSpecifier);
      const clause = node.importClause;
      if (!clause) throw Error("side-effect react import in " + file);
      if (clause.name) {
        bindings.add(clause.name.text);
        const named = clause.namedBindings;
        const head = `import * as ${clause.name.text} from ${JSON.stringify(port)};`;
        if (!named) fileEdits.push([node.getStart(source), node.getEnd(), head]);
        else if (ts.isNamedImports(named)) fileEdits.push([node.getStart(source), node.getEnd(), `${head}\nimport ${named.getText(source)} from ${JSON.stringify(port)};`]);
        else throw Error("default + namespace import in " + file);
        if (named && ts.isNamedImports(named)) for (const element of named.elements) note(bucket, (element.propertyName ?? element.name).text);
      } else {
        replaceLiteral(node.moduleSpecifier);
        const named = clause.namedBindings;
        if (named && ts.isNamespaceImport(named)) bindings.add(named.name.text);
        if (named && ts.isNamedImports(named)) for (const element of named.elements) note(bucket, (element.propertyName ?? element.name).text);
      }
    } else if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal) && specifierOf(node.argument.literal)) {
      replaceLiteral(node.argument.literal);
      let qualifier = node.qualifier;
      while (qualifier && ts.isQualifiedName(qualifier)) qualifier = qualifier.left;
      if (qualifier) note(specifierOf(node.argument.literal), qualifier.text);
    } else if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0]) && specifierOf(node.arguments[0])) {
      replaceLiteral(node.arguments[0]);
      const parent = node.parent?.parent;
      if (parent && ts.isVariableDeclaration(parent) && ts.isIdentifier(parent.name)) bindings.add(parent.name.text);
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  const members = (node) => {
    if (ts.isPropertyAccessExpression(node) && ts.isIdentifier(node.expression) && bindings.has(node.expression.text)) note("react", node.name.text);
    else if (ts.isQualifiedName(node) && ts.isIdentifier(node.left) && bindings.has(node.left.text)) note("react", node.right.text);
    else if (ts.isIdentifier(node) && bindings.has(node.text)) {
      const parent = node.parent;
      const qualified = (ts.isPropertyAccessExpression(parent) && parent.expression === node) || (ts.isQualifiedName(parent) && parent.left === node);
      const declaration = ts.isImportClause(parent) || ts.isNamespaceImport(parent) || ts.isVariableDeclaration(parent);
      if (!qualified && !declaration) bare.push(`${path.relative(ELEMENTS, file)}: ${node.text}`);
    }
    ts.forEachChild(node, members);
  };
  members(source);
  return fileEdits;
}

export function analyse() {
  const exportsOf = reactExports();
  const files = walk(ELEMENTS);
  const used = { react: new Map(), dom: new Map() };
  const bare = [];
  const rewritten = [];
  for (const file of files) {
    const text = fs.readFileSync(file, "utf8");
    if (!/["'](react|react-dom\/client)["']/.test(text)) continue;
    if (analyseText(file, text, exportsOf, used, bare).length) rewritten.push(file);
  }
  return { exportsOf, used, bare, files: rewritten };
}

export function rewriteText(file, text, exportsOf) {
  const used = { react: new Map(), dom: new Map() };
  return applyEdits(text, analyseText(file, text, exportsOf, used, []));
}

export function renderPort(analysis) {
  const { exportsOf, used } = analysis;
  const split = (bucket) => {
    const names = [...used[bucket].keys()].sort((a, b) => a.localeCompare(b));
    const values = names.filter((name) => exportsOf[bucket].get(name) & ts.SymbolFlags.Value);
    const types = names.filter((name) => !(exportsOf[bucket].get(name) & ts.SymbolFlags.Value));
    return { values, types };
  };
  const react = split("react");
  const dom = split("dom");
  const lines = [
    "/** ⚛️ The React port of the renderer elements: the only module under `🧱️elements` that names `react` and `react-dom/client`. Every element host reaches React through the explicit re-exports below, so the React version and the lane's whole React surface are one reviewed list; tests mount hosts with `react` directly.",
    " *",
    " * 🧭️ Part of the React lane implementation (`@semio-tech/framework-renderer-react`). The lane contract that leaves the lane carries no React type — see {@link ../🗣️Interpreter/🧩️host-contract/🟦️.ts}.",
    " */",
    "",
    `export { ${react.values.join(", ")} } from "react";`,
    `export type { ${react.types.join(", ")} } from "react";`,
    `export { ${dom.values.join(", ")} } from "react-dom/client";`,
    `export type { ${dom.types.join(", ")} } from "react-dom/client";`,
    "",
  ];
  return lines.join("\n");
}

export function applyEdits(text, fileEdits) {
  let next = text;
  for (const [start, end, replacement] of [...fileEdits].sort((a, b) => b[0] - a[0])) next = next.slice(0, start) + replacement + next.slice(end);
  return next;
}

if (import.meta.main) {
  const analysis = analyse();
  console.log(renderPort(analysis));
  console.log("files", analysis.files.length, "bare", analysis.bare);
}
