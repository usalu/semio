import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const [pkgsPath, scope = "🧰️framework/🛍️products/🦑️repo/", out] = process.argv.slice(2) as [string, string?, string?];
const packages: any[] = JSON.parse(readFileSync(pkgsPath, "utf8"));
const groups = new Map<string, { owners: Set<string>; targets: Set<string> }>();
const entryFiles = (value: unknown): string[] => typeof value === "string" ? [value] : Array.isArray(value) ? value.flatMap(entryFiles) : value && typeof value === "object" ? Object.values(value).flatMap(entryFiles) : [];
for (const pkg of packages) {
  const g = groups.get(pkg.sourceOwner) ?? { owners: new Set(), targets: new Set() };
  g.owners.add(pkg.owner);
  try { const manifest = JSON.parse(readFileSync(join(root, pkg.owner, "package.json"), "utf8")); for (const value of Object.values(manifest.exports ?? {})) for (const t of entryFiles(value)) if (t.startsWith("./") && !t.includes("*")) g.targets.add(normalize(join(pkg.owner, t))); } catch {}
  groups.set(pkg.sourceOwner, g);
}
const ownersSorted = [...groups.keys()].sort((a, b) => b.length - a.length);
const zoneOf = (path: string): string | null => {
  for (const owner of ownersSorted) if (path.startsWith(owner + "/") && !/(^|\/)node_modules(\/|$)/u.test(path.slice(owner.length + 1))) return owner;
  return null;
};
const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z", "--", scope], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?)$/u.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/") && existsSync(join(root, f)));
const violations: { from: string; to: string; owner: string; spec: string }[] = [];
for (const from of files) {
  const text = readFileSync(join(root, from), "utf8");
  const sf = ts.createSourceFile(from, text, ts.ScriptTarget.Latest, true, from.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const specs: string[] = [];
  const visit = (node: ts.Node) => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) specs.push(node.moduleSpecifier.text);
    else if (ts.isCallExpression(node) && node.arguments[0] && ts.isStringLiteralLike(node.arguments[0]) && (node.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(node.expression) && node.expression.text === "require"))) specs.push(node.arguments[0].text);
    else if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal)) specs.push(node.argument.literal.text);
    ts.forEachChild(node, visit);
  };
  visit(sf);
  const fromZone = zoneOf(from);
  for (const spec of specs) {
    if (!spec.startsWith(".")) continue;
    const to = normalize(join(dirname(from), spec.split("?")[0]!));
    const toZone = zoneOf(to);
    if (toZone === null || toZone === fromZone) continue;
    const g = groups.get(toZone)!;
    if (g.targets.has(to)) continue;
    if (!existsSync(join(root, to)) || !statSync(join(root, to)).isFile()) continue;
    violations.push({ from, to, owner: toZone, spec });
  }
}
const uniq = new Map(violations.map((v) => [v.from + "\0" + v.to, v]));
if (out) writeFileSync(out, JSON.stringify([...uniq.values()], null, 1));
const byOwner: Record<string, number> = {};
for (const v of uniq.values()) byOwner[v.owner] = (byOwner[v.owner] ?? 0) + 1;
console.log("files", files.length, "pairs", uniq.size, byOwner);
