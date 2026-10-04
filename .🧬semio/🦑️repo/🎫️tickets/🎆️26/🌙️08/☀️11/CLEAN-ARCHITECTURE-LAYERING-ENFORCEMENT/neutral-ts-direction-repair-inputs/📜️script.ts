/** 🧭️ Retains the hand-authored IO and TIFF owner cohort for review without publishing source. */
import { existsSync, readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { createHash } from "node:crypto";
import ts from "typescript";
import Ajv from "ajv/dist/2020";
import * as jsonc from "jsonc-parser";

const io = "🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot";
const tiff = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot";
const read = (path: string): string => readFileSync(path, "utf8");
const json = (path: string): any => JSON.parse(read(path));
const render = (value: unknown): string => JSON.stringify(value, null, 2) + "\n";
const hash = (source: string | null): string | null => source === null ? null : createHash("sha256").update(source).digest("hex");
const rows: {path: string; before: string | null; beforeHash: string | null; authored: string; authoredHash: string | null; inverse: string | null}[] = [];
const add = (path: string, authored: string): void => {
  const before = existsSync(path) ? read(path) : null;
  if (before === authored) throw Error("Empty authored change: " + path);
  rows.push({path, before, beforeHash: hash(before), authored, authoredHash: hash(authored), inverse: before});
};
const importPath = (from: string, target: string): string => {
  const path = relative(dirname(from), target).replaceAll("\\", "/");
  return path.startsWith(".") ? path : "./" + path;
};
const testName = (node: ts.Statement): string | null => ts.isExpressionStatement(node) && ts.isCallExpression(node.expression) && node.expression.expression.getText() === "test" && ts.isStringLiteral(node.expression.arguments[0]!) ? node.expression.arguments[0]!.text : null;
const variables = (node: ts.Statement): string[] => ts.isVariableStatement(node) ? node.declarationList.declarations.map(value => value.name.getText()) : [];
const remove = (source: string, nodes: readonly ts.Statement[]): string => [...nodes].sort((left, right) => right.pos - left.pos).reduce((value, node) => value.slice(0, node.pos) + value.slice(node.end), source);

try {
  let workspace = process.cwd();
  while (!existsSync(workspace + "/.vscode/🧩️launch.seed.jsonc")) {
    const parent = dirname(workspace);
    if (parent === workspace) throw Error("Workspace source authority not found");
    workspace = parent;
  }
  process.chdir(workspace);
  if (process.argv[2] === "strict") {
    const proposal = json(import.meta.dir + "/io-owner-split-ready-2.json");
    const overlay = new Map<string, string>(proposal.rows.map((row: {path: string; authored: string}) => [resolve(row.path), row.authored]));
    const options: ts.CompilerOptions = {noEmit: true, strict: true, skipLibCheck: true, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, types: ["bun"]};
    const host = ts.createCompilerHost(options), originalRead = host.readFile.bind(host), originalExists = host.fileExists.bind(host), originalDirectory = host.directoryExists?.bind(host);
    host.readFile = path => overlay.get(resolve(path)) ?? originalRead(path);
    host.fileExists = path => overlay.has(resolve(path)) || originalExists(path);
    host.directoryExists = path => [...overlay.keys()].some(file => file.startsWith(resolve(path) + "/")) || !!originalDirectory?.(path);
    host.getSourceFile = (path, language) => {const source = host.readFile(path); return source === undefined ? undefined : ts.createSourceFile(path, source, language, true);};
    const sources = proposal.rows.filter((row: {path: string}) => row.path.endsWith(".ts") && !row.path.startsWith("✏️s/") || row.path.endsWith("🟦️.ts") && row.path.startsWith("✏️s/")).map((row: {path: string}) => resolve(row.path));
    const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram(sources, options, host)).map(value => ({path: value.file?.fileName, code: value.code, message: ts.flattenDiagnosticMessageText(value.messageText, " ")}));
    const output = import.meta.dir + "/../🗑️generated/goal-root/io-owner-split-strict-2";
    mkdirSync(output, {recursive: true});
    writeFileSync(output + "/strict.json", render({sources, diagnostics, sourceWrites: false}));
    console.log(JSON.stringify({strictSources: sources.length, diagnostics: diagnostics.length, first: diagnostics.slice(0,12)}));
    process.exit(diagnostics.length ? 1 : 0);
  }
  const frontierPath = io + "/🧪️tests/💰️frontiers/🟦️.ts", operationPath = io + "/🧪️tests/💰️operation/🟦️.ts";
  const frontier = read(frontierPath), operation = read(operationPath);
  const frontierTree = ts.createSourceFile(frontierPath, frontier, ts.ScriptTarget.Latest, true), operationTree = ts.createSourceFile(operationPath, operation, ts.ScriptTarget.Latest, true);
  const frontierLaws = frontierTree.statements.filter(node => testName(node)?.includes("TIFF")), operationLaws = operationTree.statements.filter(node => testName(node)?.includes("TIFF"));
  if (frontierLaws.length !== 8 || operationLaws.length !== 1) throw Error("Original TIFF law roster changed");
  const globals = frontierTree.statements.filter(node => variables(node).some(name => ["small", "corpus", "corpusFile"].includes(name)) || ts.isFunctionDeclaration(node) && node.name?.text === "bytes");
  const specificImport = (node: ts.Statement): boolean => ts.isImportDeclaration(node) && node.moduleSpecifier.getText().includes("✏️s");
  let neutralFrontier = remove(frontier, frontierTree.statements.filter(node => frontierLaws.includes(node) || globals.includes(node) || specificImport(node))).replace("Complete domain scalar, grammar, row and physical-tree cancellation laws.", "Shared authored-row, grammar and table-index cancellation laws.");
  const marker = 'test("actual shared authored-row';
  const insertion = neutralFrontier.indexOf(marker);
  if (insertion < 0) throw Error("Original shared authored-row law changed");
  const admission = 'test("shared authored-row frontier corpus is closed and independently admitted",()=>{\n const oracle=new Ajv({strict:true}).compile(schema);expect(oracle(fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);for(const hostile of [{...fixture,extra:1},{...fixture,builderRows:fixture.builderRows-1}]){expect(oracle(hostile)).toBe(false);expect(validateJsonSchemaSubset(schema,hostile).length).toBeGreaterThan(0);}\n});\n';
  neutralFrontier = neutralFrontier.slice(0, insertion) + admission + neutralFrontier.slice(insertion);
  const neutralOperation = remove(operation, operationTree.statements.filter(node => operationLaws.includes(node) || specificImport(node))).replace("Actual physical scalar work and complete domain operation authority.", "Physical scalar work and owned SQLite operation authority.").replaceAll("fixture.domain.projectionRequests", "fixture.allocationRequests");
  const tiffFrontierPath = tiff + "/🧪️tests/💰️frontiers/🟦️.ts", tiffOperationPath = tiff + "/🧪️tests/💰️operation/🟦️.ts";
  const header = (from: string, kind: string): string => `/** 🖼️ Complete TIFF ${kind} laws retain their original operation limits. */\nimport {expect,test} from "bun:test";\nimport Ajv from "ajv/dist/2020";\nimport {Database} from "bun:sqlite";\nimport fixture from "../../🧫️fixtures/💰️${kind}/🔣️.json";\nimport schema from "../../🧬️schema/💰️${kind}/🔣️.json";\nimport {validateJsonSchemaSubset} from ${JSON.stringify(importPath(from, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"))};\nimport {exportSqliteDatabase,SqliteOperation} from ${JSON.stringify(importPath(from, io + "/🟦️.ts"))};\nimport {tiffSnapshotToSqliteDatabase,tiffSnapshotToSqliteFile,tiffSnapshotFromSqliteFile} from "../../🟦️.ts";\n`;
  add(frontierPath, neutralFrontier);
  add(operationPath, neutralOperation);
  add(tiffFrontierPath, header(tiffFrontierPath, "frontiers") + globals.map(node => node.getText(frontierTree)).join("\n") + "\n" + frontierLaws.map(node => node.getText(frontierTree).replaceAll("finalFixture.groupingPass", "fixture.groupingPass").replaceAll("finalFixture.refusal", "fixture.refusal")).join("\n") + "\n");
  add(tiffOperationPath, header(tiffOperationPath, "operation") + 'test("complete TIFF operation grant corpus is closed and independently admitted",()=>{const admit=new Ajv({strict:true}).compile(schema);expect(admit(fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);for(const hostile of [{...fixture,extra:1},{...fixture,domain:{...fixture.domain,projectionRequests:[4]}}]){expect(admit(hostile)).toBe(false);expect(validateJsonSchemaSubset(schema,hostile).length).toBeGreaterThan(0);}});\n' + operationLaws[0]!.getText(operationTree) + "\n");

  const frontierFixture = json(io + "/🧫️fixtures/💰️frontiers/🔣️.json"), frontierSchema = json(io + "/🧬️schema/💰️frontiers/🔣️.json"), finalFixture = json(io + "/🧫️fixtures/🔎️frontiers/🔣️.json"), finalSchema = json(io + "/🧬️schema/🔎️frontiers/🔣️.json"), operationFixture = json(io + "/🧫️fixtures/💰️operation/🔣️.json"), operationSchema = json(io + "/🧬️schema/💰️operation/🔣️.json");
  const project = (fixture: any, schema: any, keys: string[], title: string): any => ({fixture: Object.fromEntries(keys.map(key => [key, fixture[key]])), schema: {...schema, title, required: [...keys], properties: Object.fromEntries(keys.map(key => [key, schema.properties[key]]))}});
  const neutralKeys = ["rowFrontier", "phases", "refusal", "sharedProgressRoster", "builderSql", "builderRows"], specificKeys = Object.keys(frontierFixture).filter(key => !["sharedProgressRoster", "builderSql", "builderRows"].includes(key));
  const neutralFrontierCorpus = project(frontierFixture, frontierSchema, neutralKeys, "Shared SQLite Authored Row Frontiers"), specificFrontierCorpus = project(frontierFixture, frontierSchema, specificKeys, "Complete TIFF Operation Frontiers");
  specificFrontierCorpus.fixture.groupingPass = finalFixture.groupingPass;
  specificFrontierCorpus.schema.required.push("groupingPass");
  specificFrontierCorpus.schema.properties.groupingPass = finalSchema.properties.groupingPass;
  const neutralOperationFixture = {...operationFixture, allocationRequests: operationFixture.domain.projectionRequests};
  delete neutralOperationFixture.domain;
  const neutralOperationSchema = {...operationSchema, title: "Owned SQLite Operation Grant", required: operationSchema.required.filter((key: string) => key !== "domain").concat("allocationRequests"), properties: {...operationSchema.properties, allocationRequests: {const: operationFixture.domain.projectionRequests}}};
  delete neutralOperationSchema.properties.domain;
  const specificOperationCorpus = {fixture: {domain: operationFixture.domain, refusal: operationFixture.refusal}, schema: {...operationSchema, title: "Complete TIFF Operation Grant", required: ["domain", "refusal"], properties: {domain: operationSchema.properties.domain, refusal: operationSchema.properties.refusal}}};
  delete finalFixture.groupingPass;
  finalSchema.required = finalSchema.required.filter((key: string) => key !== "groupingPass");
  delete finalSchema.properties.groupingPass;
  const corpora = [[io, "💰️frontiers", neutralFrontierCorpus], [tiff, "💰️frontiers", specificFrontierCorpus], [io, "💰️operation", {fixture: neutralOperationFixture, schema: neutralOperationSchema}], [tiff, "💰️operation", specificOperationCorpus], [io, "🔎️frontiers", {fixture: finalFixture, schema: finalSchema}]] as const;
  for (const [owner, kind, corpus] of corpora) {
    const validate = new Ajv({strict: true, allErrors: true}).compile(corpus.schema);
    if (!validate(corpus.fixture)) throw Error(JSON.stringify(validate.errors));
    add(owner + "/🧫️fixtures/" + kind + "/🔣️.json", render(corpus.fixture));
    add(owner + "/🧬️schema/" + kind + "/🔣️.json", render(corpus.schema));
  }

  const tiffScriptPath = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🟦️typescript/📜️script.ts", tiffScript = read(tiffScriptPath), suiteEnd = '🧪️tests/🪶️sqlite/🟦️.ts"]';
  if (tiffScript.split(suiteEnd).length !== 2) throw Error("TIFF suite registration changed");
  const tiffSuiteRoot = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot";
  add(tiffScriptPath, tiffScript.replace(suiteEnd, '🧪️tests/🪶️sqlite/🟦️.ts",' + JSON.stringify(tiffSuiteRoot + "/🧪️tests/💰️frontiers/🟦️.ts") + "," + JSON.stringify(tiffSuiteRoot + "/🧪️tests/💰️operation/🟦️.ts") + "]"));
  const packageRoot = io + "/📦️packages/🦀️rust", scriptPath = packageRoot + "/📜️script.ts", script = read(scriptPath);
  const operationClass = '/** 💰️ Checks the complete neutral SQLite operation and frontier laws. */\nclass OperationScript extends BundleScript {\n async run(segments:string[]):Promise<void>{\n  if(segments.length)throw Error("test-operation accepts no arguments");\n  const sources=["💰️operation","💰️frontiers"].map(kind=>resolve(this.root,"../../🧪️tests/"+kind+"/🟦️.ts"));\n  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...sources],{cwd:this.repoRoot,budgetMs:cmdBudgetMs(),throwOnFailure:true});\n  await runBudgetedTestCommand(process.execPath,["test",...sources],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});\n }\n}\n\n';
  add(scriptPath, script.replace("const router = ", operationClass + "const router = ").replace('.register("test-refusal", RefusalScript)', '.register("test-operation",OperationScript).register("test-refusal", RefusalScript)'));
  const projectPath = packageRoot + "/📋️project.json", ownerProject = json(projectPath);
  ownerProject.targets["test-operation"] = {executor: "nx:run-commands", cache: false, inputs: ["default"], options: {cwd: packageRoot, command: "bun ./📜️script.ts test-operation"}};
  add(projectPath, render(ownerProject));
  const packagePath = packageRoot + "/package.json", ownerPackage = json(packagePath);
  ownerPackage.scripts["test-operation"] = "bun nx run @semio-tech/framework-sqlite-snapshot-rs:test-operation";
  add(packagePath, render(ownerPackage));
  const guiPath = ".vscode/🧩️launch.seed.jsonc", gui = read(guiPath), tree = jsonc.parseTree(gui)!;
  const configurations = jsonc.findNodeAtLocation(tree, ["configurations"])!, index = configurations.children!.findIndex(node => jsonc.getNodeValue(node).name === "⚖️gate🪶️sqlite⚠️refusal");
  if (index < 0) throw Error("GUI anchor changed");
  const anchor = configurations.children![index]!, entry = {...jsonc.getNodeValue(anchor), name: "⚖️gate🪶️sqlite💰️operation", command: "bun nx run @semio-tech/framework-sqlite-snapshot-rs:test-operation --skip-nx-cache", presentation: {group: "9_gates", order: 900.058535}};
  const addition = JSON.stringify(entry, null, 4).split("\n").map((line, index) => index ? "        " + line : line).join("\n") + ",\n        ";
  const authoredGui = gui.slice(0, anchor.offset) + addition + gui.slice(anchor.offset), parsedGui = jsonc.parse(authoredGui);
  if (parsedGui.configurations.length !== configurations.children!.length + 1) throw Error("GUI row count");
  for (let oldIndex = 0; oldIndex < configurations.children!.length; oldIndex++) if (JSON.stringify(jsonc.getNodeValue(configurations.children![oldIndex]!)) !== JSON.stringify(parsedGui.configurations[oldIndex + (oldIndex >= index ? 1 : 0)])) throw Error("Foreign GUI row changed");
  add(guiPath, authoredGui);
  for (const row of rows.filter(row => row.path.endsWith(".ts"))) if ((ts.createSourceFile(row.path, row.authored, ts.ScriptTarget.Latest, true) as any).parseDiagnostics.length) throw Error("Authored TypeScript grammar: " + row.path);
  const output = {stage: "IO And TIFF Test Ownership", sourceWrites: false, rows, originalLaws: {frontiers: frontierLaws.map(testName), operation: operationLaws.map(testName)}, retainedSpecificLawBodies: [...frontierLaws.map(node => ({name: testName(node), body: node.getText(frontierTree)})), ...operationLaws.map(node => ({name: testName(node), body: node.getText(operationTree)}))], schemaOracle: {cases: 5, pass: 5}, guiOriginalRows: configurations.children!.length, guiNewRows: parsedGui.configurations.length, parseDiagnostics: 0, pending: ["independent source review", "actual complete neutral and TIFF registered suites", "live framework-neutral direction"]};
  mkdirSync(import.meta.dir, {recursive: true});
  writeFileSync(import.meta.dir + "/io-owner-split-ready-2.json", render(output));
  console.log(JSON.stringify({proposalRows: rows.length, originalSpecificLaws: output.originalLaws, parseDiagnostics: 0, schemaOracle: 5, guiAdded: 1, sourceWrites: false}));
} catch (error) {
  console.error(error);
  process.exit(1);
}
