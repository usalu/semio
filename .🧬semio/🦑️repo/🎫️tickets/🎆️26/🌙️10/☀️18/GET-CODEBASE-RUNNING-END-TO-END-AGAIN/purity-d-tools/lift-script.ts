import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, posix, relative, resolve } from "node:path";
import ts from "typescript";

/**
 * usage: bun lift-script.ts <script repo-relative path> <module repo-relative path> [apply]
 * Splits a router script into the router skeleton (router imports, router construction, terminal call) and an owner module
 * that holds every other statement exported. Relative specifiers are re-expressed from the module's location; the script
 * imports the lifted names from the module.
 */
const root = "/Users/ueli/Documents/semio/";
const [, , scriptPath, modulePath, mode = "dry"] = process.argv;
if (!scriptPath || !modulePath) throw new Error("usage: lift-script.ts <script> <module> [apply]");
const source = readFileSync(root + scriptPath, "utf8").replace(/^import ?\{ ?isMainModuleV1 ?\} ?from ?"[^"]*";?\n/mu, "").replace(/if ?\(isMainModuleV1\(import\.meta\.url\)\) ?\{\n([\s\S]*?)\n\}\s*$/mu, (_m, body: string) => `${body.replace(/^  /gmu, "")}\n`).replace(/if ?\(isMainModuleV1\(import\.meta\.url\)\) ?(await )/mu, "$1");
const file = ts.createSourceFile(scriptPath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const ROUTER_NAMES = new Set(["ScriptRouter", "receiveScriptProcessInvocation", "runScriptMain"]);
const TERMINAL = /\b(?:receiveScriptProcessInvocation|runScriptMain)\b/u;

const scriptDir = dirname(scriptPath), moduleDir = dirname(modulePath);
const reSpecifier = (specifier: string): string => {
  if (!specifier.startsWith(".")) return specifier;
  const target = posix.normalize(posix.join(scriptDir, specifier));
  const next = posix.relative(moduleDir, target);
  return next.startsWith(".") ? next : `./${next}`;
};
const backSpecifier = (specifier: string): string => {
  const target = posix.normalize(posix.join(moduleDir, specifier));
  const next = posix.relative(scriptDir, target);
  return next.startsWith(".") ? next : `./${next}`;
};

const importTexts: { text: string; names: string[]; specifier: string; typeOnly: boolean; node: ts.ImportDeclaration }[] = [];
const bodies: ts.Statement[] = [];
let shebang = "";
if (source.startsWith("#!")) shebang = source.slice(0, source.indexOf("\n") + 1);
for (const statement of file.statements) {
  if (ts.isImportDeclaration(statement)) {
    const specifier = (statement.moduleSpecifier as ts.StringLiteral).text;
    const names: string[] = [];
    const clause = statement.importClause;
    if (clause?.name) names.push(clause.name.text);
    if (clause?.namedBindings) {
      if (ts.isNamespaceImport(clause.namedBindings)) names.push(clause.namedBindings.name.text);
      else for (const element of clause.namedBindings.elements) names.push(element.name.text);
    }
    importTexts.push({ text: statement.getFullText(file), names, specifier, typeOnly: !!clause?.isTypeOnly, node: statement });
  } else bodies.push(statement);
}
const isTerminal = (statement: ts.Statement): boolean => TERMINAL.test(statement.getText(file)) && (ts.isExpressionStatement(statement) || ts.isIfStatement(statement));
const declaredNames = (statement: ts.Statement): string[] => {
  if (ts.isClassDeclaration(statement) || ts.isFunctionDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isEnumDeclaration(statement)) return statement.name ? [statement.name.text] : [];
  if (ts.isVariableStatement(statement)) return statement.declarationList.declarations.flatMap((declaration) => ts.isIdentifier(declaration.name) ? [declaration.name.text] : []);
  return [];
};
const routerStatements = bodies.filter((statement) => ts.isVariableStatement(statement) && /\bnew ScriptRouter\b/u.test(statement.getText(file)) || isTerminal(statement));
const moved = bodies.filter((statement) => !routerStatements.includes(statement));
const movedNames = moved.flatMap(declaredNames);
const routerText = routerStatements.map((statement) => statement.getText(file)).join("\n");
const usedByRouter = movedNames.filter((name) => new RegExp(`\\b${name}\\b`, "u").test(routerText));
const exportedAlready = new Set(moved.filter((statement) => ts.canHaveModifiers(statement) && ts.getModifiers(statement)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)).flatMap(declaredNames));

const scriptImports = importTexts.filter((entry) => entry.names.some((name) => ROUTER_NAMES.has(name) || new RegExp(`\\b${name}\\b`, "u").test(routerText)));
const scriptImportLines: string[] = [];
const routerOnly = new Set<string>();
for (const entry of scriptImports) {
  const clause = entry.node.importClause!;
  const keep = clause.namedBindings && ts.isNamedImports(clause.namedBindings) ? clause.namedBindings.elements.filter((element) => ROUTER_NAMES.has(element.name.text) || new RegExp(`\\b${element.name.text}\\b`, "u").test(routerText)).map((element) => element.getText(file)) : [];
  for (const element of keep) routerOnly.add(element);
  if (keep.length > 0) scriptImportLines.push(`import { ${keep.join(", ")} } from ${JSON.stringify(entry.specifier)};`);
}
const moduleImports = importTexts.map((entry) => {
  const clause = entry.node.importClause;
  let text = entry.text.replace(/^#!.*\n/u, "").trim();
  text = text.replace(/from\s+(["'])([^"']+)\1/u, (_m, q: string, spec: string) => `from ${q}${reSpecifier(spec)}${q}`);
  const bare = /^import\s+(["'])([^"']+)\1/u.exec(text);
  if (bare) text = text.replace(bare[0], `import ${bare[1]}${reSpecifier(bare[2]!)}${bare[1]}`);
  void clause;
  return text;
});
const exportedBodies = moved.map((statement) => {
  const full = statement.getFullText(file);
  const names = declaredNames(statement);
  if (/import\.meta/u.test(statement.getText(file))) console.log(`WARN import.meta inside moved statement ${names.join(",") || statement.getText(file).slice(0, 40)}`);
  if (names.length === 0 || exportedAlready.has(names[0]!)) return full.replace(/^\s*\n/u, "");
  const offset = statement.getStart(file) - statement.getFullStart();
  return `${full.slice(0, offset).replace(/^\s*\n/u, "")}export ${full.slice(offset)}`;
});
const importSpecifierToModule = JSON.stringify(backSpecifier(`./${posix.basename(modulePath)}`));
const lifted = [...new Set(usedByRouter)].sort();
const scriptText = `${shebang}${scriptImportLines.join("\n")}\n${lifted.length > 0 ? `import { ${lifted.join(", ")} } from ${importSpecifierToModule};\n` : ""}\n${routerText}\n`;
const moduleText = `${moduleImports.join("\n")}\n\n${exportedBodies.join("\n").replace(/^\n+/u, "")}\n`;
console.log(`script ${scriptPath}: routerStatements=${routerStatements.length} moved=${moved.length} lifted=${lifted.join(",")}`);
if (routerStatements.length === 0) throw new Error("no router statements found");
if (mode === "apply") {
  if (existsSync(root + modulePath)) throw new Error(`module exists: ${modulePath}`);
  mkdirSync(dirname(root + modulePath), { recursive: true });
  writeFileSync(root + modulePath, moduleText);
  writeFileSync(root + scriptPath, scriptText);
  console.log("applied");
} else {
  console.log("---- script\n" + scriptText.slice(0, 1500));
  console.log("---- module (head)\n" + moduleText.slice(0, 800));
}
void join; void resolve; void relative;
