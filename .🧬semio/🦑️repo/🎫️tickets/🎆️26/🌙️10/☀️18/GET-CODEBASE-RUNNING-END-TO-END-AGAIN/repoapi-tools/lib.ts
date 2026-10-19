import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";

export const ROOT = resolve(import.meta.dir, "../../../../../../../..");
export const TICKET = resolve(import.meta.dir, "..");
export const GENERATED = resolve(TICKET, "🗑️generated/repoapi");
export const LIB_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
export const LIB = resolve(ROOT, LIB_REL);
export const PKG_DIR = resolve(LIB, "📦️packages/🟦️typescript");
export const PKG_NAME = "@semio-tech/repo-lib";
export const FROZEN = [`${LIB_REL}/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/`, `${LIB_REL}/🏗️build/📥️inputs/🔏️revision/`];
export const SKIP_SEGMENTS = ["/node_modules/", "/target/", "/dist/", "/📤️dist/", "/🗑️generated/", "/🤖️generated/", "/.cache/", "/pkg/"];
export const SOURCE_EXT = /\.(ts|tsx|mts|cts|mjs|js|cjs)$/u;

export const posix = (p: string): string => p.split("\\").join("/");
export const rel = (abs: string): string => posix(relative(ROOT, abs));

export function readJson<T = any>(path: string): T {
  return JSON.parse(readFileSync(path, "utf8")) as T;
}
export function writeJson(path: string, value: unknown): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify(value, null, 1) + "\n");
}

export function fileList(refresh = false): string[] {
  const cache = resolve(GENERATED, "files.txt");
  if (!refresh && existsSync(cache) && Date.now() - statSync(cache).mtimeMs < 10 * 60_000) return readFileSync(cache, "utf8").split("\n").filter(Boolean);
  const out = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: ROOT, maxBuffer: 1 << 28, encoding: "utf8" });
  if (out.status !== 0) throw Error("git ls-files failed");
  const files = out.stdout.split("\0").filter(f => f && SOURCE_EXT.test(f) && !f.startsWith(".🧬semio/") && !SKIP_SEGMENTS.some(s => ("/" + f).includes(s)) && existsSync(resolve(ROOT, f)));
  mkdirSync(GENERATED, { recursive: true });
  writeFileSync(cache, files.join("\n") + "\n");
  return files;
}

export const isFrozen = (relPath: string): boolean => FROZEN.some(f => relPath.startsWith(f));
export const inLib = (relPath: string): boolean => relPath.startsWith(LIB_REL + "/");
export const libRel = (relPath: string): string => relPath.slice(LIB_REL.length + 1);

/** Files inside LIB that belong to the nested nx-bootstrap-tools package or to repo-lib itself are never rewritten (same source owner or zero-touch bootstrap). */
export const isOutsidePackage = (relPath: string): boolean => !inLib(relPath);

export type EdgeKind = "import" | "type-import" | "export-from" | "export-star" | "dynamic" | "require" | "import-type" | "side-effect";
export type Edge = {
  file: string;
  spec: string;
  target: string;
  kind: EdgeKind;
  names: string[];
  star: boolean;
  start: number;
  end: number;
  unused?: string[];
  json?: boolean;
};

export function resolveSpec(fromAbs: string, spec: string, known: Set<string>): string | undefined {
  if (!spec.startsWith(".")) return undefined;
  const base = resolve(dirname(fromAbs), spec.split("?")[0]!);
  const stem = base.replace(/\.(js|mjs|cjs)$/u, "");
  for (const candidate of [base, base + ".ts", base + ".tsx", base + "/🟦️.ts", base + "/🟦️.tsx", stem + ".ts", stem + ".tsx"]) {
    const r = rel(candidate);
    if (known.has(r)) return r;
  }
  return undefined;
}

export function parse(file: string, text: string): ts.SourceFile {
  const kind = file.endsWith(".tsx") ? ts.ScriptKind.TSX : file.endsWith(".js") || file.endsWith(".mjs") || file.endsWith(".cjs") ? ts.ScriptKind.JS : ts.ScriptKind.TS;
  return ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, kind);
}

function namespaceMembers(sf: ts.SourceFile, local: string, skip: ts.Node): string[] | "*" {
  const members = new Set<string>();
  let escapes = false;
  const visit = (n: ts.Node): void => {
    if (n === skip) return;
    if (ts.isIdentifier(n) && n.text === local) {
      const p = n.parent;
      if (ts.isPropertyAccessExpression(p) && p.expression === n) members.add(p.name.text);
      else if (ts.isQualifiedName(p) && p.left === n) members.add(p.right.text);
      else if (ts.isElementAccessExpression(p) && p.expression === n && ts.isStringLiteralLike(p.argumentExpression)) members.add(p.argumentExpression.text);
      else if (ts.isTypeQueryNode(p) || ts.isTypeReferenceNode(p)) escapes = escapes || false;
      else if (!(ts.isPropertyAccessExpression(p) && p.name === n) && !(ts.isPropertyAssignment(p) && p.name === n) && !(ts.isBindingElement(p) && p.propertyName === n)) escapes = true;
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return escapes ? "*" : [...members].sort();
}

/** Extracts every import-like edge of one file whose specifier resolves to a known file. */
export function edgesOf(file: string, text: string, known: Set<string>): Edge[] {
  const abs = resolve(ROOT, file);
  const sf = parse(file, text);
  const out: Edge[] = [];
  const usedOutsideImports = new Set<string>();
  const collectUsed = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) usedOutsideImports.add(n.text);
    ts.forEachChild(n, collectUsed);
  };
  collectUsed(sf);
  const push = (spec: string, kind: EdgeKind, names: string[], star: boolean, node: ts.Node): void => {
    const target = resolveSpec(abs, spec, known);
    if (target) out.push({ file, spec, target, kind, names, star, start: node.getStart(sf), end: node.getEnd() });
  };
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      const c = n.importClause;
      const spec = n.moduleSpecifier.text;
      if (!c) push(spec, "side-effect", [], false, n.moduleSpecifier);
      else {
        const names: string[] = [];
        const unused: string[] = [];
        let star = false;
        if (c.name) { names.push("default"); if (!usedOutsideImports.has(c.name.text)) unused.push("default"); }
        if (c.namedBindings) {
          if (ts.isNamespaceImport(c.namedBindings)) {
            const members = namespaceMembers(sf, c.namedBindings.name.text, n);
            if (members === "*") star = true; else names.push(...members);
            if (!usedOutsideImports.has(c.namedBindings.name.text)) { unused.push("*"); star = false; names.length = 0; }
          } else for (const e of c.namedBindings.elements) { names.push((e.propertyName ?? e.name).text); if (!usedOutsideImports.has(e.name.text)) unused.push((e.propertyName ?? e.name).text); }
        }
        push(spec, c.isTypeOnly ? "type-import" : "import", names, star, n.moduleSpecifier);
        const last = out[out.length - 1];
        if (last && last.spec === spec && last.start === n.moduleSpecifier.getStart(sf)) { last.unused = unused; last.json = spec.endsWith(".json"); }
      }
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      const spec = n.moduleSpecifier.text;
      if (n.exportClause && ts.isNamedExports(n.exportClause)) push(spec, "export-from", n.exportClause.elements.map(e => (e.propertyName ?? e.name).text), false, n.moduleSpecifier);
      else push(spec, "export-star", [], true, n.moduleSpecifier);
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) {
      const spec = (n.arguments[0] as ts.StringLiteralLike).text;
      if (n.expression.kind === ts.SyntaxKind.ImportKeyword) {
        const names: string[] = [];
        let star = true;
        const awaited = n.parent && ts.isAwaitExpression(n.parent) ? n.parent : undefined;
        const holder = awaited?.parent;
        if (holder && ts.isVariableDeclaration(holder) && ts.isObjectBindingPattern(holder.name) && holder.initializer === awaited) {
          star = false;
          for (const e of holder.name.elements) if (e.dotDotDotToken) star = true; else names.push(((e.propertyName ?? e.name) as ts.Identifier).text);
        } else if (holder && ts.isPropertyAccessExpression(holder) && holder.expression === awaited) { star = false; names.push(holder.name.text); }
        else if (awaited?.parent && ts.isParenthesizedExpression(awaited.parent) && ts.isPropertyAccessExpression(awaited.parent.parent) && awaited.parent.parent.expression === awaited.parent) { star = false; names.push(awaited.parent.parent.name.text); }
        else if (awaited?.parent && ts.isVariableDeclaration(awaited.parent) && ts.isIdentifier(awaited.parent.name)) {
          const members = namespaceMembers(sf, awaited.parent.name.text, awaited.parent);
          if (members !== "*") { star = false; names.push(...members); }
        } else if (n.parent && ts.isPropertyAccessExpression(n.parent) && n.parent.name.text === "then") {
          const call = n.parent.parent;
          const fn = ts.isCallExpression(call) ? call.arguments[0] : undefined;
          if (fn && (ts.isArrowFunction(fn) || ts.isFunctionExpression(fn)) && fn.parameters[0]) {
            const p = fn.parameters[0]!.name;
            if (ts.isObjectBindingPattern(p)) { star = false; for (const e of p.elements) if (e.dotDotDotToken) star = true; else names.push(((e.propertyName ?? e.name) as ts.Identifier).text); }
            else if (ts.isIdentifier(p)) { const members = namespaceMembers(sf, p.text, fn.parameters[0]!); if (members !== "*") { star = false; names.push(...members); } }
          }
        }
        push(spec, "dynamic", names, star, n.arguments[0]!);
      } else if (ts.isIdentifier(n.expression) && n.expression.text === "require") push(spec, "require", [], true, n.arguments[0]!);
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) {
      const names = n.qualifier ? [ts.isIdentifier(n.qualifier) ? n.qualifier.text : (n.qualifier as ts.QualifiedName).left.getText(sf)] : [];
      push(n.argument.literal.text, "import-type", names, names.length === 0, n.argument.literal);
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return out;
}

export function knownFiles(files: string[]): Set<string> {
  const known = new Set(files);
  return known;
}

/** JSON files and other non-source targets inside the library are also valid import targets. */
export function libTargets(refresh = false): Set<string> {
  const cache = resolve(GENERATED, "lib-files.txt");
  if (!refresh && existsSync(cache) && Date.now() - statSync(cache).mtimeMs < 10 * 60_000) return new Set(readFileSync(cache, "utf8").split("\n").filter(Boolean));
  const out = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z", "--", LIB_REL], { cwd: ROOT, maxBuffer: 1 << 28, encoding: "utf8" });
  if (out.status !== 0) throw Error("git ls-files failed");
  const files = out.stdout.split("\0").filter(f => f && existsSync(resolve(ROOT, f)));
  mkdirSync(GENERATED, { recursive: true });
  writeFileSync(cache, files.join("\n") + "\n");
  return new Set(files);
}

/** Import closure of the build CLI entry (`bun repo`): these files must stay relative, zero-touch. */
export function cliClosure(): Set<string> {
  const entry = resolve(LIB, "🏗️build/📜️script.ts");
  const seen = new Set<string>();
  const queue = [entry];
  const pattern = /((?:import|export)\b[^;'"`]*?from\s*|import\s*\(\s*|import\s+|require\(\s*)(["'])([^"'\n]+)\2/gm;
  const cand = (p: string): string[] => [p, p + ".ts", p + "/🟦️.ts", p.replace(/\.js$/u, ".ts")];
  while (queue.length) {
    const f = queue.pop()!;
    if (seen.has(f) || !existsSync(f)) continue;
    seen.add(f);
    if (!/\.(ts|tsx|mts)$/u.test(f)) continue;
    let text: string;
    try { text = readFileSync(f, "utf8"); } catch { continue; }
    for (const m of text.matchAll(pattern)) {
      const s = m[3]!;
      if (!s.startsWith(".")) continue;
      const base = resolve(dirname(f), s.split("?")[0]!);
      const hit = cand(base).find(c => existsSync(c) && !statSync(c).isDirectory());
      if (hit) queue.push(hit);
    }
  }
  return new Set([...seen].map(rel));
}
