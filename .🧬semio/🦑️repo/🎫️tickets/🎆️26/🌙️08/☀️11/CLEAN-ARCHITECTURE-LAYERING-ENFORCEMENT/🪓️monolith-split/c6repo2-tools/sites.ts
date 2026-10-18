import { ts } from "./lib.ts";
import { readFileSync, existsSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { dirname, resolve, extname } from "node:path";

export const ROOT = "/Users/ueli/Documents/semio";
const EXTS = new Set([".ts", ".tsx", ".mts", ".cts", ".js", ".mjs", ".cjs", ".jsx"]);

export function candidateFiles(): string[] {
  const out = new Set<string>();
  const run = (args: string[]) => execFileSync("git", ["-c", "core.fsmonitor=false", "-C", ROOT, ...args], { maxBuffer: 1 << 30 }).toString("utf8").split("\0").filter(Boolean);
  for (const f of run(["ls-files", "-z"])) out.add(f);
  for (const f of run(["ls-files", "-z", "-o", "--exclude-standard"])) out.add(f);
  const res: string[] = [];
  for (const f of out) {
    if (!EXTS.has(extname(f))) continue;
    if (f.endsWith(".d.ts") || f.endsWith(".d.mts")) continue;
    if (f.startsWith(".🧬semio/") || f.includes("/node_modules/")) continue;
    const abs = resolve(ROOT, f);
    if (existsSync(abs)) res.push(abs);
  }
  return res;
}

export interface Site {
  file: string;
  kind: "import" | "export" | "dynamic" | "importType";
  spec: string;
  start: number;
  end: number;
  node: string;
}

export function resolveSpec(fromFile: string, spec: string): string | undefined {
  if (!spec.startsWith(".")) return undefined;
  return resolve(dirname(fromFile), spec);
}

export function findSites(file: string, targets: Set<string>): Site[] {
  const text = readFileSync(file, "utf8");
  if (!text.includes("from") && !text.includes("import")) return [];
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const sites: Site[] = [];
  const check = (lit: import("typescript").Node, kind: Site["kind"], node: import("typescript").Node) => {
    if (!ts.isStringLiteralLike(lit)) return;
    const r = resolveSpec(file, lit.text);
    if (r && targets.has(r)) sites.push({ file, kind, spec: lit.text, start: lit.getStart(sf), end: lit.getEnd(), node: ts.SyntaxKind[node.kind] });
  };
  const visit = (n: import("typescript").Node) => {
    if (ts.isImportDeclaration(n)) check(n.moduleSpecifier, "import", n);
    else if (ts.isExportDeclaration(n) && n.moduleSpecifier) check(n.moduleSpecifier, "export", n);
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments.length >= 1) check(n.arguments[0], "dynamic", n);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument)) check(n.argument.literal, "importType", n);
    else if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === "require" && n.arguments.length === 1) check(n.arguments[0], "dynamic", n);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return sites;
}
