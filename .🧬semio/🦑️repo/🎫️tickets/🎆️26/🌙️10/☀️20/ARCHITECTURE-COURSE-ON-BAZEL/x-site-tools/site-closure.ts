import ts from "typescript";
import picomatch from "picomatch";
import { existsSync, readFileSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";

const repo = resolve(process.env.REPO ?? resolve(import.meta.dir, "../../../../../../../../"));
const course = join(repo, "🎓️teaching/🏛️architecture/📚️course");
const bundle = join(course, "📦️packages/🟦️typescript");
const manifestPath = process.env.COURSE_MANIFEST ?? join(bundle, "📋️project.json");
const config = ts.readConfigFile(join(bundle, "tsconfig.json"), ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(config.config, ts.sys, bundle);
const options: ts.CompilerOptions = { ...parsed.options, noEmit: true };
const entries = (process.env.ENTRIES ?? "🟦️.ts,🏗️builder/🌐️vite/🟦️.ts").split(",").map(entry => join(course, entry));
const parents = new Map<string, string>(), importers = new Map<string, string>(), seen = new Set<string>(), packages = new Set<string>(), unresolved: string[] = [];
const host: ts.ModuleResolutionHost = { fileExists: ts.sys.fileExists, readFile: ts.sys.readFile, realpath: ts.sys.realpath, directoryExists: ts.sys.directoryExists, getCurrentDirectory: () => bundle };
const valueSpecifiers = (file: string): string[] => {
  const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.ESNext, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const found: string[] = [];
  const typeOnly = (clause: ts.ImportClause | undefined): boolean => clause !== undefined && (clause.isTypeOnly || (clause.name === undefined && clause.namedBindings !== undefined && ts.isNamedImports(clause.namedBindings) && clause.namedBindings.elements.length > 0 && clause.namedBindings.elements.every(element => element.isTypeOnly)));
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && !typeOnly(node.importClause)) found.push(node.moduleSpecifier.text);
    else if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && !node.isTypeOnly) found.push(node.moduleSpecifier.text);
    else if (ts.isCallExpression(node) && node.arguments.length === 1 && ts.isStringLiteral(node.arguments[0]!) && (node.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(node.expression) && node.expression.text === "require"))) found.push((node.arguments[0] as ts.StringLiteral).text);
    ts.forEachChild(node, visit);
  };
  visit(source);
  return found;
};
const queue = [...entries];
while (queue.length) {
  const file = queue.pop()!;
  if (seen.has(file)) continue;
  seen.add(file);
  if (!/\.(?:tsx?|mts|js|mjs)$/u.test(file) || !existsSync(file)) continue;
  for (const specifier of valueSpecifiers(file)) {
    const resolved = ts.resolveModuleName(specifier, file, options, host).resolvedModule;
    if (!resolved) { if (!specifier.startsWith("node:") && !specifier.startsWith("bun:")) unresolved.push(`${relative(repo, file)} -> ${specifier}`); continue; }
    const real = resolved.resolvedFileName;
    if (real.includes(`${sep}node_modules${sep}`)) {
      const match = [...real.matchAll(/node_modules\/((?:@[^/]+\/)?[^/]+)/gu)].at(-1);
      if (match) { packages.add(match[1]!); importers.set(match[1]!, relative(repo, file)); }
      continue;
    }
    if (!parents.has(real)) parents.set(real, file);
    queue.push(real);
  }
}
const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { namedInputs: Record<string, unknown[]>; targets: Record<string, { inputs?: unknown[] }> };
const globsOf = (inputs: unknown[]): { positive: string[]; negative: string[] } => {
  const flat = inputs.flatMap(input => typeof input === "string" && manifest.namedInputs[input] ? manifest.namedInputs[input]! : [input]).filter((input): input is string => typeof input === "string" && input !== "sharedGlobals");
  const sub = (glob: string): string => glob.replaceAll("{workspaceRoot}/", "").replaceAll("{projectRoot}", relative(repo, bundle));
  return { positive: flat.filter(glob => !glob.startsWith("!")).map(sub), negative: flat.filter(glob => glob.startsWith("!")).map(glob => sub(glob.slice(1))) };
};
const target = process.env.TARGET ?? "build";
const { positive, negative } = globsOf(manifest.targets[target]!.inputs ?? ["default"]);
const covered = (path: string): boolean => positive.some(glob => picomatch(glob, { dot: true })(path)) && !negative.some(glob => picomatch(glob, { dot: true })(path));
const files = [...seen].map(file => relative(repo, file)).filter(path => !path.startsWith("..")).sort();
const generated = files.filter(path => path.includes("🤖️generated"));
const missing = files.filter(path => !generated.includes(path) && !covered(path));
const externalCovered = (name: string): boolean => (manifest.namedInputs.default.find((input): input is { externalDependencies: string[] } => typeof input === "object" && input !== null && "externalDependencies" in input)?.externalDependencies ?? []).includes(name);
const report = { entries: entries.map(entry => relative(repo, entry)), reached: files.length, generated: generated.length, missing, externalPackages: [...packages].sort(), importers: Object.fromEntries(importers), externalUndeclared: [...packages].filter(name => !externalCovered(name)).sort(), unresolved: unresolved.slice(0, 40) };
if (process.env.CHAIN) { let at = [...seen].find(file => file.includes(process.env.CHAIN!)); const chain: string[] = []; while (at) { chain.push(relative(repo, at)); at = parents.get(at); } console.log(chain.join("\n<- ")); }
writeFileSync(process.argv[2] ?? "/dev/stdout", JSON.stringify(report, null, 1));
