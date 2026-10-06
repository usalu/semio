import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import ts from "typescript";

const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const ticket = dirname(dirname(import.meta.dir));
const output = join(ticket, "🗑️generated/computed-plugin-reader-audit");
const excluded = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"];
const paths = [
  ...new Bun.Glob("**/🧪️tests/*/🟦️.ts").scanSync({ cwd: root, onlyFiles: true }),
  ...new Bun.Glob("**/🧪️tests/*/🟦️.tsx").scanSync({ cwd: root, onlyFiles: true }),
].filter((path) => (path.startsWith("✏️s/") || path.startsWith("🧰️framework/")) && !excluded.some((prefix) => path.startsWith(prefix))).sort();
type Entry = { source: string; line: number; kind: "read" | "enumerate"; expression: string; target?: string; exists?: boolean };
const entries: Entry[] = [];
const missing: Entry[] = [];
const unresolved: Entry[] = [];

for (const path of paths) {
  const absolute = join(root, path);
  const source = ts.createSourceFile(absolute, readFileSync(absolute, "utf8"), ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const declaration = (name: string, node: ts.Node): ts.Expression | undefined => {
    for (let parent: ts.Node | undefined = node.parent; parent; parent = parent.parent) {
      if (ts.isFunctionLike(parent) && parent.parameters.some((parameter) => ts.isIdentifier(parameter.name) && parameter.name.text === name)) return undefined;
      if (!ts.isBlock(parent) && !ts.isSourceFile(parent)) continue;
      const candidates = parent.statements.flatMap((statement) => ts.isVariableStatement(statement) ? statement.declarationList.declarations : []).filter((candidate) => ts.isIdentifier(candidate.name) && candidate.name.text === name && candidate.initializer);
      const prior = candidates.filter((candidate) => candidate.getStart(source) < node.getStart(source));
      const selected = prior.at(-1) ?? candidates.at(-1);
      if (selected?.initializer) return selected.initializer;
    }
    return undefined;
  };
  const value = (node: ts.Expression, seen = new Set<ts.Node>()): string | undefined => {
    if (seen.has(node)) return undefined;
    seen.add(node);
    if (ts.isStringLiteralLike(node) || ts.isNoSubstitutionTemplateLiteral(node)) return node.text;
    if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isTypeAssertionExpression(node) || ts.isNonNullExpression(node)) return value(node.expression, seen);
    if (ts.isTemplateExpression(node)) {
      let result = node.head.text;
      for (const span of node.templateSpans) {
        const part = value(span.expression, seen);
        if (part === undefined) return undefined;
        result += part + span.literal.text;
      }
      return result;
    }
    if (ts.isBinaryExpression(node) && node.operatorToken.kind === ts.SyntaxKind.PlusToken) {
      const left = value(node.left, seen), right = value(node.right, seen);
      return left === undefined || right === undefined ? undefined : left + right;
    }
    if (ts.isIdentifier(node)) { const initializer = declaration(node.text, node); return initializer ? value(initializer, seen) : undefined; }
    if (ts.isPropertyAccessExpression(node)) {
      if (node.getText(source) === "import.meta.dir" || node.getText(source) === "import.meta.dirname") return dirname(absolute);
      if (node.getText(source) === "import.meta.url") return pathToFileURL(absolute).href;
      const object = ts.isIdentifier(node.expression) ? declaration(node.expression.text, node.expression) : node.expression;
      if (node.name.text === "url" || node.name.text === "href") { const resolved = value(node.expression, seen); if (resolved?.startsWith("file:")) return resolved; }
      if (object && ts.isObjectLiteralExpression(object)) {
        const property = object.properties.find((candidate) => ts.isPropertyAssignment(candidate) && candidate.name.getText(source).replaceAll(/[\"']/g, "") === node.name.text);
        return property && ts.isPropertyAssignment(property) ? value(property.initializer, seen) : undefined;
      }
      return undefined;
    }
    if (ts.isElementAccessExpression(node) && ts.isObjectLiteralExpression(node.expression) && node.argumentExpression) {
      const key = value(node.argumentExpression, seen);
      const property = node.expression.properties.find((candidate) => ts.isPropertyAssignment(candidate) && candidate.name.getText(source).replaceAll(/[\"']/g, "") === key);
      return property && ts.isPropertyAssignment(property) ? value(property.initializer, seen) : undefined;
    }
    if (ts.isCallExpression(node)) {
      const name = node.expression.getText(source);
      const arguments_ = node.arguments.map((argument) => value(argument, seen));
      if (arguments_.some((argument) => argument === undefined)) return undefined;
      const args = arguments_ as string[];
      if (["join", "posix.join"].includes(name)) return join(...args);
      if (name === "resolve") return resolve(...args);
      if (name === "dirname") return dirname(args[0]!);
      if (name === "fileURLToPath") { try { return fileURLToPath(args[0]!); } catch { return undefined; } }
      if (name === "pathToFileURL") return pathToFileURL(args[0]!).href;
    }
    if (ts.isNewExpression(node) && node.expression.getText(source) === "URL" && node.arguments?.[0]) {
      const args = node.arguments.map((argument) => value(argument, seen));
      if (args.some((argument) => argument === undefined)) return undefined;
      try { return new URL(args[0]!, args[1]).href; } catch { return undefined; }
    }
    return undefined;
  };
  const record = (node: ts.CallExpression, kind: Entry["kind"], argument: ts.Expression): void => {
    const raw = value(argument);
    const target = raw?.startsWith("file:") ? (() => { try { return fileURLToPath(raw); } catch { return undefined; } })() : raw && (isAbsolute(raw) ? raw : undefined);
    const entry: Entry = { source: path, line: source.getLineAndCharacterOfPosition(node.getStart(source)).line + 1, kind, expression: argument.getText(source), ...(target ? { target: relative(root, target), exists: existsSync(target) } : {}) };
    entries.push(entry);
    if (!target) unresolved.push(entry);
    else if (!entry.exists && !target.includes("/🗑️generated/")) missing.push(entry);
  };
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node)) {
      const name = node.expression.getText(source);
      if (["readFileSync", "readFile", "Bun.file"].includes(name) && node.arguments[0]) record(node, "read", node.arguments[0]);
      if (["readdirSync", "readdir"].includes(name) && node.arguments[0]) record(node, "enumerate", node.arguments[0]);
      if (["scanSync", "scan"].includes(name) && node.arguments[0] && ts.isObjectLiteralExpression(node.arguments[0])) {
        const cwd = node.arguments[0].properties.find((property) => ts.isPropertyAssignment(property) && property.name.getText(source) === "cwd");
        if (cwd && ts.isPropertyAssignment(cwd)) record(node, "enumerate", cwd.initializer);
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
}
mkdirSync(output, { recursive: true });
writeFileSync(join(output, "result.json"), JSON.stringify({ files: paths.length, entries, missing, unresolved }, null, 2) + "\n");
console.log(JSON.stringify({ files: paths.length, entries: entries.length, missing: missing.length, unresolved: unresolved.length }));
