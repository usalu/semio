import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";
import { classifyPackageSourceDisposition, clearDiscoveryCache, discoverPackageProblems, loadCatalogTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const scratch = process.env.SEMIO_AUDIT_SCRATCH;
if (!scratch) throw new Error("SEMIO_AUDIT_SCRATCH is required");
mkdirSync(scratch, { recursive: true });
const taxonomy = loadCatalogTaxonomy();
const disposition = taxonomy.packageSourceDispositions["root-script"]!;
const grammar = taxonomy.packageGlueGrammar[disposition.grammarId ?? "typescript"]!;
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const prefix = 'import { BundleScript, ScriptRouter, runBundleScriptMain, execute } from "./dep.ts";\n';
const suffix = '\nconst router = new ScriptRouter([Build]);\nawait runBundleScriptMain(import.meta, router);\n';
const dep = "export class BundleScript {}\nexport class ScriptRouter { constructor(..._values: unknown[]) {} register(..._values: unknown[]): this { return this } }\nexport async function runBundleScriptMain(..._values: unknown[]): Promise<void> {}\nexport async function execute(..._values: unknown[]): Promise<void> {}\nexport function dirname(value: string): string { return value }\nexport function fileURLToPath(value: string): string { return value }\nexport class SupportScript {}\nexport class MaterializeScript {}\n";
const globals = "declare const console: { log(...values: unknown[]): void; error(...values: unknown[]): void };\ndeclare const process: { exit(value?: unknown): void; argv: string[]; env: Record<string, string | undefined> };\ninterface ImportMeta { readonly url: string; readonly main: boolean }\n";

function semanticEvidence(source: string): { readonly diagnostics: readonly number[]; readonly executeBindings: readonly Readonly<{ readonly reference: number; readonly declarations: readonly number[] }>[] } {
  const root = join(scratch, "semantic");
  const router = join(root, "router.ts"), owner = join(root, "dep.ts"), ambient = join(root, "ambient.d.ts");
  const sources = new Map([[router, source], [owner, dep], [ambient, globals]]);
  const options: ts.CompilerOptions = { allowImportingTsExtensions: true, module: ts.ModuleKind.NodeNext, moduleResolution: ts.ModuleResolutionKind.NodeNext, noEmit: true, strict: true, target: ts.ScriptTarget.ESNext, types: [] };
  const native = ts.createCompilerHost(options);
  const host: ts.CompilerHost = {
    ...native,
    fileExists: (path) => sources.has(path) || native.fileExists(path),
    readFile: (path) => sources.get(path) ?? native.readFile(path),
    getSourceFile: (path, languageVersion) => {
      const text = sources.get(path);
      return text === undefined ? native.getSourceFile(path, languageVersion) : ts.createSourceFile(path, text, languageVersion, true);
    },
    getCurrentDirectory: () => root,
  };
  host.resolveModuleNames = (names, containing) => names.map((name) => name === "./dep.ts" && containing === router ? { resolvedFileName: owner, extension: ts.Extension.Ts, isExternalLibraryImport: false } : ts.resolveModuleName(name, containing, options, host).resolvedModule);
  const program = ts.createProgram({ rootNames: [...sources.keys()], options, host });
  const sourceFile = program.getSourceFile(router)!;
  const checker = program.getTypeChecker();
  const executeBindings: { reference: number; declarations: readonly number[] }[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isIdentifier(node) && node.text === "execute" && node.getStart(sourceFile) > source.indexOf("class Build")) {
      const symbol = checker.getSymbolAtLocation(node);
      executeBindings.push({ reference: sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile)).line + 1, declarations: (symbol?.declarations ?? []).filter((declaration) => declaration.getSourceFile() === sourceFile).map((declaration) => sourceFile.getLineAndCharacterOfPosition(declaration.getStart(sourceFile)).line + 1) });
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return { diagnostics: ts.getPreEmitDiagnostics(program).filter((row) => row.file?.fileName === router).map((row) => row.code).sort((left, right) => left - right), executeBindings };
}

const scoped = {
  ambient: 'class Build extends BundleScript { async run(args: string[]) { console.log(args); await execute(args); } }',
  blockShadow: 'class Build extends BundleScript { async run(args: string[]) { if (args.length) { const console: any = args; console.log(args); } await execute(args); } }',
  closureShadow: 'class Build extends BundleScript { async run(args: string[]) { const invoke = (execute: any) => execute(args); await invoke(args); } }',
  loopShadow: 'class Build extends BundleScript { async run(args: string[]) { for (const process of args) { process; } await execute(args); } }',
  siblingBlockAmbient: 'class Build extends BundleScript { async run(args: string[]) { if (args.length) { const console: any = args; } console.log(args); await execute(args); } }',
  siblingClassAmbient: 'import { BundleScript, ScriptRouter, runBundleScriptMain, execute } from "./dep.ts";\nclass Quiet extends BundleScript { async run(args: string[]) { const console: any = args; } }\nclass Build extends BundleScript { async run(args: string[]) { console.log(args); await execute(args); } }\nconst router = new ScriptRouter([Quiet, Build]);\nawait runBundleScriptMain(import.meta, router);\n',
  runtimeAlias: 'import { BundleScript as Base, ScriptRouter as Router, runBundleScriptMain as terminal, execute as delegated } from "./dep.ts";\nclass Build extends Base { async run(args: string[]) { await delegated(args); } }\nconst router = new Router([Build]);\nawait terminal(import.meta, router);\n',
  typeOnlyAlias: 'import { BundleScript, ScriptRouter, runBundleScriptMain } from "./dep.ts";\nimport { type execute as delegated } from "./dep.ts";\nclass Build extends BundleScript { async run(args: string[]) { await delegated(args); } }',
  tdzConsole: 'class Build extends BundleScript { async run(args: string[]) { console.log(args); const console: any = args; await execute(args); } }',
  tdzProcess: 'class Build extends BundleScript { async run(args: string[]) { process.exit(await execute(args)); const process: any = args; } }',
  tdzClosure: 'class Build extends BundleScript { async run(args: string[]) { const invoke = () => execute(args); const execute: any = args; await invoke(); } }',
  selfPendingRouter: 'import { ScriptRouter, runBundleScriptMain } from "./dep.ts";\nconst router = router;\nawait runBundleScriptMain(import.meta, router);\n',
  routerReceipt: 'import { dirname, fileURLToPath, MaterializeScript, ScriptRouter, runBundleScriptMain, SupportScript } from "./dep.ts";\nconst router = new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("support", SupportScript).register("materialize", MaterializeScript);\nif (import.meta.main) await runBundleScriptMain(router, import.meta.url);\n',
} as const;

const scopedResults = Object.entries(scoped).map(([id, body]) => {
  const source = ["runtimeAlias", "routerReceipt", "siblingClassAmbient", "selfPendingRouter"].includes(id) ? body : id === "typeOnlyAlias" ? body + suffix : prefix + body + suffix;
  return { id, disposition: classifyPackageSourceDisposition(source, disposition, grammar), ...semanticEvidence(source) };
});
const livePluginDisposition = classifyPackageSourceDisposition(readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin", "📦️packages", "🟦️typescript", "📜️script.ts"), "utf8"), disposition, grammar);

const packageRoot = join(scratch, "fallback", "🧰️framework", "🔨️modules", "🎫️ticket", "📦️packages", "🟦️typescript");
mkdirSync(packageRoot, { recursive: true });
writeFileSync(join(packageRoot, "package.json"), JSON.stringify({ name: "@semio/router-audit", semio: { role: "framework", id: "router-audit" } }));
const fallbackRows = [
  ["type-terminal", 'import type { runArtifactRustPackageMain } from "./owner/🟦️.ts";\nawait runArtifactRustPackageMain(import.meta.dir, "fixture");\n'],
  ["type-delegate", 'import type { execute } from "./owner/🟦️.ts";\nawait execute();\n'],
  ["runtime-terminal", 'import { runArtifactRustPackageMain } from "./owner/🟦️.ts";\nawait runArtifactRustPackageMain(import.meta.dir, "fixture");\n'],
] as const;
const fallbackResults = fallbackRows.map(([id, content]) => {
  writeFileSync(join(packageRoot, "📜️script.ts"), content);
  clearDiscoveryCache();
  const row = discoverPackageProblems(join(scratch, "fallback"), taxonomy).find((problem) => problem.path.endsWith("/📜️script.ts"));
  return { id, kind: row?.kind ?? null, message: row?.message ?? null };
});
clearDiscoveryCache();
rmSync(join(scratch, "fallback"), { recursive: true, force: true });
console.log(JSON.stringify({ scopedResults, livePluginDisposition, fallbackResults }, null, 2));
