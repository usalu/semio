import { createRequire } from "node:module";
import { dirname, posix } from "node:path";

const ts = createRequire("/Users/ueli/Documents/semio/package.json")("typescript") as typeof import("typescript");

export interface EditV1 { readonly start: number; readonly end: number; readonly text: string }
export interface ManualV1 { readonly line: number; readonly column: number; readonly rule: string; readonly snippet: string }
export interface TransformResultV1 { readonly text: string; readonly edits: number; readonly manual: ManualV1[]; readonly used: string[] }

const CALLS: Readonly<Record<string, string>> = {
  "TOML.parse": "runtimeParseTomlV1", "JSONC.parse": "runtimeParseJsoncV1", "YAML.parse": "runtimeParseYamlV1",
  which: "runtimeWhichV1", resolve: "runtimeResolveV1", resolveSync: "runtimeResolveV1", sleep: "runtimeSleepV1", spawn: "runtimeSpawnV1", spawnSync: "runtimeSpawnSyncV1",
  gc: "runtimeCollectGarbageV1", build: "runtimeBundleV1", "semver.satisfies": "runtimeSemverSatisfiesV1", "semver.order": "runtimeSemverOrderV1",
  "stdin.text": "runtimeStdinTextV1", "stdin.stream": "runtimeStdinStreamV1",
};
const CONSTRUCTORS: Readonly<Record<string, string>> = { Glob: "GlobV1", Transpiler: "TranspilerV1" };
const SPECIFIER = /^bun(?::[a-z]*)?$/u;

function unwrap(node: import("typescript").Node): import("typescript").Node {
  let current = node;
  while (ts.isParenthesizedExpression(current) || ts.isAsExpression(current) || ts.isNonNullExpression(current) || ts.isTypeAssertionExpression(current)) current = current.expression;
  return current;
}

function isGlobalThis(node: import("typescript").Node): boolean { const inner = unwrap(node); return ts.isIdentifier(inner) && inner.text === "globalThis"; }

/** 🪝️ A `Bun` root is the identifier or `globalThis.Bun`, through parentheses and casts. */
function isBunRoot(node: import("typescript").Node): boolean {
  const inner = unwrap(node);
  if (ts.isIdentifier(inner)) return inner.text === "Bun";
  return ts.isPropertyAccessExpression(inner) && inner.name.text === "Bun" && isGlobalThis(inner.expression);
}

/** 🔗️ Reads `Bun.a.b` as ["a","b"], or undefined when the chain does not start at a Bun root. */
function bunChain(node: import("typescript").Node): string[] | undefined {
  const names: string[] = [];
  let current: import("typescript").Node = node;
  for (;;) {
    const inner = unwrap(current);
    if (ts.isPropertyAccessExpression(inner) && !isBunRoot(inner)) { names.unshift(inner.name.text); current = inner.expression; continue; }
    return isBunRoot(inner) && names.length > 0 ? names : undefined;
  }
}

export function transformSource(path: string, text: string, importFor: (name: string) => string): TransformResultV1 {
  const kind = path.endsWith(".tsx") ? ts.ScriptKind.TSX : path.endsWith(".ts") || path.endsWith(".mts") || path.endsWith(".cts") ? ts.ScriptKind.TS : ts.ScriptKind.JS;
  const source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, kind);
  const edits: EditV1[] = [];
  const consumed: [number, number][] = [];
  const manual: ManualV1[] = [];
  const used = new Set<string>();
  const overlaps = (start: number, end: number): boolean => consumed.some(([a, b]) => start < b && end > a);
  const replace = (node: import("typescript").Node, replacement: string, name?: string): void => {
    const start = node.getStart(source), end = node.getEnd();
    if (overlaps(start, end)) return;
    consumed.push([start, end]);
    edits.push({ start, end, text: replacement });
    if (name !== undefined) used.add(name);
  };
  const note = (node: import("typescript").Node, rule: string): void => {
    const position = source.getLineAndCharacterOfPosition(node.getStart(source));
    manual.push({ line: position.line + 1, column: position.character + 1, rule, snippet: text.slice(node.getStart(source), Math.min(node.getEnd(), node.getStart(source) + 160)).replace(/\s+/gu, " ") });
  };
  const patchTarget = (call: import("typescript").CallExpression | import("typescript").NewExpression): void => {
    const first = call.arguments?.[0];
    if (first === undefined || !ts.isObjectLiteralExpression(first)) return;
    for (const property of first.properties) {
      if (!ts.isPropertyAssignment(property) || !ts.isIdentifier(property.name) || property.name.text !== "target") continue;
      const value = property.initializer;
      if (ts.isStringLiteral(value) && value.text === "bun") replace(value, value.getText(source).startsWith("'") ? "'runtime'" : '"runtime"');
    }
  };

  const visit = (node: import("typescript").Node): void => {
    if (ts.isNewExpression(node)) {
      const chain = bunChain(node.expression);
      if (chain !== undefined && chain.length === 1 && chain[0]! in CONSTRUCTORS) {
        replace(node.expression, CONSTRUCTORS[chain[0]!]!, CONSTRUCTORS[chain[0]!]!);
        if (chain[0] === "Transpiler") patchTarget(node);
      }
    } else if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "startsWith" && node.arguments.length === 1 && ts.isStringLiteral(node.arguments[0]!) && node.arguments[0]!.text === "bun:") {
      replace(node, `isRuntimeBuiltinSpecifierV1(${node.expression.expression.getText(source)})`, "isRuntimeBuiltinSpecifierV1");
    } else if (ts.isBinaryExpression(node) && (node.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken || node.operatorToken.kind === ts.SyntaxKind.ExclamationEqualsEqualsToken) && (ts.isStringLiteral(node.right) && /^bun:[a-z]+$/u.test(node.right.text) || ts.isStringLiteral(node.left) && /^bun:[a-z]+$/u.test(node.left.text))) {
      const literalRight = ts.isStringLiteral(node.right), other = literalRight ? node.left : node.right;
      replace(node, `${node.operatorToken.kind === ts.SyntaxKind.ExclamationEqualsEqualsToken ? "!" : ""}isRuntimeBuiltinSpecifierV1(${other.getText(source)})`, "isRuntimeBuiltinSpecifierV1");
    } else if (ts.isCallExpression(node)) {
      const chain = bunChain(node.expression);
      if (chain !== undefined && chain.join(".") in CALLS) {
        const name = CALLS[chain.join(".")]!;
        replace(node.expression, name, name);
        if (chain[0] === "build") patchTarget(node);
      } else if (node.expression.kind === ts.SyntaxKind.ImportKeyword) {
        const argument = node.arguments[0];
        if (argument !== undefined && ts.isStringLiteral(argument) && argument.text === "bun:sqlite" && node.arguments.length === 1) {
          const parent = node.parent;
          if (ts.isAwaitExpression(parent)) replace(parent, "{ Database: SqliteDatabaseV1 }", "SqliteDatabaseV1");
        }
      }
    } else if (ts.isPropertyAccessExpression(node)) {
      const target = unwrap(node.expression);
      if (ts.isMetaProperty(target) && target.keywordToken === ts.SyntaxKind.ImportKeyword) {
        if (node.name.text === "dir") replace(node, "import.meta.dirname");
        else if (node.name.text === "path") replace(node, "import.meta.filename");
        else if (node.name.text === "main") replace(node, "isMainModuleV1(import.meta.url)", "isMainModuleV1");
      }
    } else if (ts.isStringLiteral(node) && node.text === "bun" && ts.isCallExpression(node.parent) && node.parent.arguments.indexOf(node) >= 0 && node.parent.arguments.indexOf(node) <= 2 && (() => { const next = node.parent.arguments[node.parent.arguments.indexOf(node) + 1]; return next !== undefined && ts.isArrayLiteralExpression(next) && next.elements[0] !== undefined && ts.isStringLiteral(next.elements[0]!) && next.elements[0]!.text === "repo"; })()) {
      const array = node.parent.arguments[node.parent.arguments.indexOf(node) + 1] as import("typescript").ArrayLiteralExpression, first = array.elements[0]!, second = array.elements[1];
      replace(node, node.getText(source).startsWith("'") ? "'repo'" : '"repo"');
      const end = second === undefined ? first.getEnd() : second.getStart(source);
      if (!overlaps(first.getStart(source), end)) { consumed.push([first.getStart(source), end]); edits.push({ start: first.getStart(source), end, text: "" }); }
    } else if (ts.isStringLiteral(node) && node.text === "bun" && ts.isCallExpression(node.parent) && node.parent.arguments.indexOf(node) >= 0 && node.parent.arguments.indexOf(node) <= 2 && (() => { const next = node.parent.arguments[node.parent.arguments.indexOf(node) + 1]; return next !== undefined && ts.isArrayLiteralExpression(next) && next.elements[0] !== undefined && ts.isStringLiteral(next.elements[0]!) && next.elements[0]!.text === "test"; })()) {
      const array = node.parent.arguments[node.parent.arguments.indexOf(node) + 1] as import("typescript").ArrayLiteralExpression;
      replace(node, "runtimeExecutableV1()", "runtimeExecutableV1");
      const rest = array.elements.slice(1).map(element => element.getText(source)).join(", ");
      replace(array, `runtimeTestArgumentsV1([${rest}])`, "runtimeTestArgumentsV1");
    } else if (ts.isStringLiteral(node) && node.text === "bun" && ts.isCallExpression(node.parent) && node.parent.arguments.indexOf(node) >= 0 && node.parent.arguments.indexOf(node) <= 2) {
      const next = node.parent.arguments[node.parent.arguments.indexOf(node) + 1];
      const first = next !== undefined && ts.isArrayLiteralExpression(next) ? next.elements[0] : undefined;
      const script = first !== undefined && (!ts.isStringLiteral(first) && !ts.isNoSubstitutionTemplateLiteral(first) || /[/.]/u.test((first as import("typescript").StringLiteral).text));
      if (script) replace(node, "runtimeExecutableV1()", "runtimeExecutableV1");
    } else if (ts.isArrayLiteralExpression(node) && node.elements.length >= 2 && ts.isStringLiteral(node.elements[0]!) && node.elements[0]!.text === "bun" && ts.isStringLiteral(node.elements[1]!) && node.elements[1]!.text === "repo" && !(ts.isCallExpression(node.parent) && node.parent.arguments.some(argument => ts.isStringLiteral(argument)))) {
      const first = node.elements[0]!, second = node.elements[1]!;
      if (!overlaps(first.getStart(source), second.getStart(source))) { consumed.push([first.getStart(source), second.getStart(source)]); edits.push({ start: first.getStart(source), end: second.getStart(source), text: "" }); }
    } else if (ts.isTupleTypeNode(node) && node.elements.length >= 2 && ts.isLiteralTypeNode(node.elements[0]!) && ts.isStringLiteral((node.elements[0] as import("typescript").LiteralTypeNode).literal) && ((node.elements[0] as import("typescript").LiteralTypeNode).literal as import("typescript").StringLiteral).text === "bun" && ts.isLiteralTypeNode(node.elements[1]!) && ((node.elements[1] as import("typescript").LiteralTypeNode).literal as import("typescript").StringLiteral).text === "repo") {
      const first = node.elements[0]!, second = node.elements[1]!;
      if (!overlaps(first.getStart(source), second.getStart(source))) { consumed.push([first.getStart(source), second.getStart(source)]); edits.push({ start: first.getStart(source), end: second.getStart(source), text: "" }); }
    } else if (ts.isArrayLiteralExpression(node)) {
      const items = node.elements;
      const head = items[0], second = items[1];
      if (head !== undefined && ts.isStringLiteral(head) && head.text === "bun" && second !== undefined && ts.isCallExpression(node.parent) && (!ts.isStringLiteral(second) && !ts.isNoSubstitutionTemplateLiteral(second) || /[/.]/u.test((second as import("typescript").StringLiteral).text))) replace(head, "runtimeExecutableV1()", "runtimeExecutableV1");
      for (let at = 0; at + 1 < items.length; at++) {
        const flag = items[at]!, value = items[at + 1]!;
        if (ts.isStringLiteral(flag) && flag.text === "--types" && ts.isStringLiteral(value) && value.text === "bun") replace(value, "runtimeTypeEnvironmentV1()", "runtimeTypeEnvironmentV1");
      }
    } else if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && node.moduleSpecifier.text === "bun:test") {
      const named = node.importClause?.namedBindings;
      const API = ["test", "describe", "expect", "beforeAll", "afterAll", "beforeEach", "afterEach"];
      if (node.importClause?.name === undefined && named !== undefined && ts.isNamedImports(named) && named.elements.every(element => API.includes((element.propertyName ?? element.name).text))) replace(node.moduleSpecifier, `"${importFor("test")}"`);
    } else if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && node.moduleSpecifier.text === "bun:sqlite") {
      const clause = node.importClause;
      const named = clause?.namedBindings;
      if (clause !== undefined && clause.name === undefined && named !== undefined && ts.isNamedImports(named) && named.elements.length === 1 && (named.elements[0]!.propertyName ?? named.elements[0]!.name).text === "Database") {
        const local = named.elements[0]!.name.text;
        replace(node, `import { SqliteDatabaseV1 as ${local} } from "${importFor("SqliteDatabaseV1")}";`);
      }
    } else if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal) && node.argument.literal.text === "bun:sqlite" && node.qualifier !== undefined && ts.isIdentifier(node.qualifier) && node.qualifier.text === "Database") {
      replace(node, node.isTypeOf ? "typeof SqliteDatabaseV1" : "SqliteDatabaseV1", "SqliteDatabaseV1");
    }
    ts.forEachChild(node, visit);
  };
  visit(source);

  const scan = (node: import("typescript").Node): void => {
    const start = node.getStart(source), end = node.getEnd();
    if (ts.isIdentifier(node) && node.text === "Bun" && !overlaps(start, end)) {
      const parent = node.parent;
      if (!(ts.isPropertyAccessExpression(parent) && parent.name === node) && !(ts.isPropertyAssignment(parent) && parent.name === node)) note(node, "Bun");
    } else if (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node)) {
      if (SPECIFIER.test(node.text) && !overlaps(start, end)) note(node, "specifier");
    } else if (ts.isPropertyAccessExpression(node) && node.name.text === "bun" && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "versions" && !overlaps(start, end)) {
      note(node, "process.versions.bun");
    } else if (ts.isPropertyAccessExpression(node) && ts.isMetaProperty(unwrap(node.expression)) && ["dir", "path", "main"].includes(node.name.text) && !overlaps(start, end)) note(node, "import.meta");
    ts.forEachChild(node, scan);
  };
  scan(source);

  let result = text;
  for (const edit of [...edits].sort((a, b) => b.start - a.start)) result = result.slice(0, edit.start) + edit.text + result.slice(edit.end);

  if (used.size > 0) {
    const groups = new Map<string, string[]>();
    for (const name of [...used].sort()) { const specifier = importFor(name); groups.set(specifier, [...(groups.get(specifier) ?? []), name]); }
    const lines = [...groups].map(([specifier, names]) => `import { ${names.join(", ")} } from "${specifier}";`);
    const imports = source.statements.filter(ts.isImportDeclaration);
    const shift = (position: number): number => { let offset = position; for (const edit of edits) if (edit.end <= position) offset += edit.text.length - (edit.end - edit.start); return offset; };
    let at: number, prefix = "", suffix = "\n";
    if (imports.length > 0) { at = shift(imports.at(-1)!.getEnd()); prefix = "\n"; suffix = ""; }
    else { const first = source.statements[0]; at = first === undefined ? result.length : shift(first.getStart(source, true)); }
    result = result.slice(0, at) + prefix + lines.join("\n") + suffix + result.slice(at);
  }
  return { text: result, edits: edits.length, manual, used: [...used] };
}

export function importSpecifier(file: string, facade: string): string {
  const relative = posix.relative(dirname(file), facade);
  return relative.startsWith(".") ? relative : `./${relative}`;
}
