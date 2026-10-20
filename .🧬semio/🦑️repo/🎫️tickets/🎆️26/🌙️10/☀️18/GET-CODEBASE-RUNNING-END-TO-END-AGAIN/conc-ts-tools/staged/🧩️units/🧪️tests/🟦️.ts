import {describe,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
import ts from "typescript";
import {checkTypeScriptUnitsV1,deriveTypeScriptImportsV1,planTypeScriptUnitsV1,readTypeScriptUnitsDocumentV1,runTypeScriptCompilerV1,stronglyConnectedV1,type TypeScriptImportsV1,type TypeScriptUnitsDocumentV1} from "../🟦️.ts";
import rules from "../🔣️.json" with {type: "json"};
import fixture from "./🔣️.json" with {type: "json"};

type Case = {name: string; files: string[]; manifests: string[]; edges: [string, string][]; json: [string, string][]; override?: Record<string, object>; expect: {units: Record<string, {files: string[]; dependencies: string[]; json: string[]}>; order: string[]; ambient: string[]}};
const documentFor = (override: Record<string, object> | undefined): TypeScriptUnitsDocumentV1 => {const base = JSON.parse(JSON.stringify(fixture.document)) as Record<string, object>; for (const [key, value] of Object.entries(override ?? {})) base[key] = {...base[key], ...value}; return readTypeScriptUnitsDocumentV1(base);};
const inputOf = (row: Case): TypeScriptImportsV1 => ({files: row.files, manifests: row.manifests, edges: row.edges, json: row.json});
const shape = (plan: ReturnType<typeof planTypeScriptUnitsV1>) => Object.fromEntries(plan.units.map(unit => [unit.id, {files: [...unit.files], dependencies: [...unit.dependencies], json: [...unit.json]}]));

describe("typescript check units: planner", () => {
  test("the rules document and the fixture document close against the schema", () => {
    readTypeScriptUnitsDocumentV1(rules);
    expect(() => readTypeScriptUnitsDocumentV1({...rules, cycles: {collapseFiles: 0, refineRounds: 1}})).toThrow();
    expect(() => readTypeScriptUnitsDocumentV1({...rules, extra: true})).toThrow();
    for (const language of rules.messages.languages) for (const key of Object.keys(rules.messages.texts)) expect((rules.messages.texts as Record<string, Record<string, string>>)[key]![language]).toBeTruthy();
  });
  for (const row of fixture.cases as unknown as Case[]) test(`fixture case ${row.name} equals the expectation`, () => {
    const plan = planTypeScriptUnitsV1(inputOf(row), documentFor(row.override), "tsconfig.json");
    expect(shape(plan)).toEqual(Object.fromEntries(Object.entries(row.expect.units).map(([id, unit]) => [id, {...unit, files: [...unit.files].sort()}])));
    expect(plan.order).toEqual(row.expect.order);
    expect(plan.ambient).toEqual(row.expect.ambient);
  });
  test("the independent python oracle answers every fixture case identically", () => {
    const run = spawnSync("python3", ["-I", join(import.meta.dir, "🐍️oracle", "🐍️.py")], {input: JSON.stringify(fixture), encoding: "utf8", maxBuffer: 64 * 1024 * 1024});
    if (run.error) return;
    expect(run.status).toBe(0);
    const oracle = JSON.parse(run.stdout) as Record<string, {order: string[]; ambient: string[]; units: {id: string; files: string[]; dependencies: string[]; json: string[]}[]}>;
    for (const row of fixture.cases as unknown as Case[]) {
      const plan = planTypeScriptUnitsV1(inputOf(row), documentFor(row.override), "tsconfig.json"), answer = oracle[row.name]!;
      expect(answer.order).toEqual([...plan.order]);
      expect(answer.ambient).toEqual([...plan.ambient]);
      expect(Object.fromEntries(answer.units.map(unit => [unit.id, {files: unit.files, dependencies: unit.dependencies, json: unit.json}]))).toEqual(shape(plan));
    }
  });
  test("random programs always yield an exact partition over an acyclic unit graph in topological order, and the oracle agrees", () => {
    let seed = 7;
    const random = () => (seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648;
    const cases: Case[] = [];
    for (let round = 0; round < 12; round++) {
      const modules = ["m0", "m1", "m2", "m3", "m4"], files: string[] = [];
      for (const module of modules) for (let i = 0; i < 6 + Math.floor(random() * 6); i++) files.push(`${module}/${["a", "b", "c"][Math.floor(random() * 3)]}/${i}.ts`, ...(random() < 0.2 ? [`${module}/🧪️tests/${i}.ts`] : []));
      const order = [...files].sort(() => random() - 0.5), rank = new Map(order.map((file, index) => [file, index])), edges: [string, string][] = [];
      for (let i = 0; i < files.length * 2; i++) {const a = files[Math.floor(random() * files.length)]!, b = files[Math.floor(random() * files.length)]!; if (rank.get(a)! > rank.get(b)!) edges.push([a, b]);}
      cases.push({name: "random" + round, files, manifests: modules, edges, json: [], override: {cycles: {collapseFiles: 1 + Math.floor(random() * 20), refineRounds: Math.floor(random() * 3)}, coarsen: {minimumFiles: Math.floor(random() * 6)}}, expect: {units: {}, order: [], ambient: []}});
    }
    const oracle = JSON.parse(spawnSync("python3", ["-I", join(import.meta.dir, "🐍️oracle", "🐍️.py")], {input: JSON.stringify({...fixture, cases}), encoding: "utf8", maxBuffer: 64 * 1024 * 1024}).stdout || "{}") as Record<string, {order: string[]; units: {id: string; files: string[]; dependencies: string[]}[]}>;
    for (const row of cases) {
      const plan = planTypeScriptUnitsV1(inputOf(row), documentFor(row.override), "tsconfig.json"), owner = new Map<string, string>();
      for (const unit of plan.units) for (const file of unit.files) {expect(owner.has(file)).toBe(false); owner.set(file, unit.id);}
      expect([...owner.keys()].sort()).toEqual([...row.files].sort());
      const position = new Map(plan.order.map((id, index) => [id, index])), adjacent = new Map<string, Set<string>>(plan.units.map(unit => [unit.id, new Set(unit.dependencies)]));
      expect(stronglyConnectedV1([...adjacent.keys()], adjacent).every(component => component.length === 1)).toBe(true);
      for (const unit of plan.units) for (const dependency of unit.dependencies) expect(position.get(dependency)!).toBeLessThan(position.get(unit.id)!);
      const consumed = new Set(plan.units.flatMap(unit => unit.dependencies)); for (const unit of plan.units) expect(unit.leaf).toBe(!consumed.has(unit.id));
      const real = new Set<string>(); for (const [a, b] of row.edges) if (owner.get(a) !== owner.get(b)) real.add(owner.get(a) + ">" + owner.get(b));
      expect(new Set(plan.units.flatMap(unit => unit.dependencies.map(dependency => unit.id + ">" + dependency)))).toEqual(real);
      expect(oracle[row.name]?.order).toEqual([...plan.order]);
    }
  });
});

const require = createRequire(import.meta.url);
const tscPath = require.resolve("typescript/bin/tsc");
const sha = (path: string) => createHash("sha256").update(readFileSync(path)).digest("hex");
const writeTree = (root: string, tree: Record<string, string>) => {for (const [path, text] of Object.entries(tree)) {mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text);}};
const syntheticTree = (): Record<string, string> => ({
  "tsconfig.json": JSON.stringify({compilerOptions: {target: "ESNext", module: "ESNext", moduleResolution: "bundler", strict: true, isolatedModules: true, allowImportingTsExtensions: true, resolveJsonModule: true, skipLibCheck: true, noEmit: true, types: []}, include: ["**/*.ts"], exclude: ["**/node_modules/**"]}),
  "package.json": JSON.stringify({name: "workspace", workspaces: ["core", "mid", "top"]}),
  "core/package.json": JSON.stringify({name: "@t/core", exports: {".": "./index.ts"}}),
  "core/index.ts": 'import {helper} from "./util.ts";\nimport data from "./data.json" with {type: "json"};\nexport const names = data.names;\nexport const x = 1;\nexport function f(a: number) {return {v: a, h: helper(a)};}\nexport class Box<T> {constructor(readonly value: T) {} map<U>(g: (t: T) => U) {return new Box(g(this.value));}}\nexport interface Shape {kind: "a" | "b"; size: number}\nexport function shape(): Shape {return {kind: "a", size: 1};}\n',
  "core/util.ts": "export function helper(n: number) {return [n, n + 1] as const;}\nexport function body(n: number) {return n * 2;}\n",
  "core/data.json": '{"names": ["a", "b"]}',
  "core/🌿️ambient/g.d.ts": "declare const GLOBAL_FLAG: boolean;\n",
  "mid/package.json": JSON.stringify({name: "@t/mid", exports: {".": "./index.ts"}}),
  "mid/index.ts": 'import {f, Box, type Shape, shape} from "@t/core";\nimport {helper} from "../core/util.ts";\nexport function g(n: number) {return {r: f(n), b: new Box(helper(n)), flag: GLOBAL_FLAG};}\nexport const s: Shape = shape();\n',
  "top/package.json": JSON.stringify({name: "@t/top", exports: {".": "./index.ts"}}),
  "top/index.ts": 'import {g, s} from "@t/mid";\nimport cfg from "../core/data.json" with {type: "json"};\nconst r = g(1);\nexport const ok: number = r.r.h[0] + r.b.value[1] + cfg.names.length;\nexport const bad1: string = r.r.v;\nexport const bad2: number = s.kind;\nexport const bad3: string = r.flag;\n',
});
const diagnosticsOf = (text: string): string[] => text.split("\n").filter(line => /error TS\d+/.test(line)).map(line => line.replace(/^.*?(?=[^/\\]+\(\d+,\d+\): error)/, "")).sort();

describe("typescript check units: compiler boundary against the compiler itself", () => {
  const workspace = realpathSync(mkdtempSync(join(tmpdir(), "units-ws-")));
  writeTree(workspace, syntheticTree());
  mkdirSync(join(workspace, "node_modules", "@t"), {recursive: true});
  for (const name of ["core", "mid", "top"]) symlinkSync(join(workspace, name), join(workspace, "node_modules", "@t", name));
  const parsed = ts.getParsedCommandLineOfConfigFile(join(workspace, "tsconfig.json"), {}, {...ts.sys, onUnRecoverableConfigFileDiagnostic: () => undefined})!;
  const document = readTypeScriptUnitsDocumentV1({...rules, coarsen: {minimumFiles: 0}, cycles: {collapseFiles: 100, refineRounds: 1}});
  const roots = parsed.fileNames.map(file => file.slice(workspace.length + 1)), read = (path: string) => {try {return readFileSync(path, "utf8");} catch {return undefined;}};
  const imports = () => deriveTypeScriptImportsV1(ts, {workspace, compilerOptions: parsed.options, roots, exists: path => existsSync(path), read});
  const hostFor = (state: string, session: string) => ({workspace, typeRoots: [] as string[], typeFiles: [] as string[], compiler: "typescript@" + ts.version, options: sha(join(workspace, "tsconfig.json")), environment: "synthetic", family: "tsconfig.json", store: join(state, "store"), session, state: join(state, "state"), digest: (path: string) => sha(join(workspace, path)), tsc: runTypeScriptCompilerV1(process.execPath, tscPath, workspace, process.env)});
  const state = mkdtempSync(join(tmpdir(), "units-state-"));
  const signal = new AbortController().signal;

  test("the derived import graph covers every workspace file the compiler lists and no other", () => {
    const listed = spawnSync(process.execPath, [tscPath, "-p", join(workspace, "tsconfig.json"), "--listFilesOnly"], {encoding: "utf8"}).stdout.split("\n").filter(line => line.startsWith(workspace + "/") && !line.includes("/node_modules/")).map(line => line.slice(workspace.length + 1));
    const graph = imports();
    expect([...graph.files, ...new Set(graph.json.map(([, to]) => to))].sort()).toEqual([...listed].sort());
  });
  test("unit-wise diagnostics equal the monolithic compiler's diagnostics", async () => {
    const monolithic = spawnSync(process.execPath, [tscPath, "-p", join(workspace, "tsconfig.json"), "--pretty", "false"], {encoding: "utf8", cwd: workspace});
    const plan = planTypeScriptUnitsV1(imports(), document, "tsconfig.json");
    expect(plan.units.length).toBeGreaterThanOrEqual(3);
    const result = await checkTypeScriptUnitsV1(plan, document, hostFor(state, "a"), {concurrency: 2, signal});
    expect(diagnosticsOf(result.diagnostics)).toEqual(diagnosticsOf(monolithic.stdout));
    expect(result.errors).toBe(3);
  }, 120000);
  test("a unit program lists its own sources, ambient files and only declaration outputs of its dependencies", () => {
    const plan = planTypeScriptUnitsV1(imports(), document, "tsconfig.json"), top = plan.units.find(unit => unit.files.includes("top/index.ts"))!, directory = join(state, "state", "sessions", "a", "configs");
    const configs = readdirSync(directory).map(name => ({name, config: JSON.parse(readFileSync(join(directory, name), "utf8")) as {files: string[]; references: unknown[]}})), own = configs.find(row => row.config.files.some(file => file.endsWith("top/index.ts")))!;
    const listed = spawnSync(process.execPath, [tscPath, "-p", join(directory, own.name), "--listFilesOnly"], {encoding: "utf8", cwd: workspace}).stdout.split("\n").filter(line => line && !line.includes("/node_modules/typescript/"));
    expect(top.dependencies.length).toBeGreaterThan(0);
    expect(own.config.references.length).toBe(top.dependencies.length);
    const allowedSources = new Set(own.config.files);
    for (const line of listed) expect(allowedSources.has(line) || (line.startsWith(join(state, "store", "declarations") + "/") && line.endsWith(".d.ts"))).toBe(true);
    expect(listed.some(line => line.endsWith("core/util.ts") || line.endsWith("mid/index.ts"))).toBe(false);
  });
  test("a second run replays every unit from the store without starting the compiler", async () => {
    const plan = planTypeScriptUnitsV1(imports(), document, "tsconfig.json"), counted = {runs: 0}, base = hostFor(state, "b");
    const result = await checkTypeScriptUnitsV1(plan, document, {...base, tsc: async (...args) => {counted.runs++; return base.tsc(...args);}}, {concurrency: 2, signal});
    expect(counted.runs).toBe(0);
    expect(result.units.every(unit => unit.status === "hit")).toBe(true);
    expect(result.errors).toBe(3);
  }, 120000);
  test("an implementation edit re-checks one unit; a signature edit re-checks its dependents (early cutoff by declaration digest)", async () => {
    const run = async (session: string) => {const plan = planTypeScriptUnitsV1(imports(), document, "tsconfig.json"), ran: string[] = [], base = hostFor(state, session); await checkTypeScriptUnitsV1(plan, document, {...base, tsc: async (config, abort) => {ran.push(config); return base.tsc(config, abort);}}, {concurrency: 1, signal}); return ran.length;};
    writeFileSync(join(workspace, "core/util.ts"), readFileSync(join(workspace, "core/util.ts"), "utf8").replace("n * 2", "n * 3"));
    expect(await run("c")).toBe(1);
    writeFileSync(join(workspace, "core/util.ts"), readFileSync(join(workspace, "core/util.ts"), "utf8") + "export function added(n: number) {return String(n);}\n");
    expect(await run("d")).toBeGreaterThanOrEqual(2);
  }, 180000);
  test("a cancelled run stops and publishes nothing for the unit in flight", async () => {
    const plan = planTypeScriptUnitsV1(imports(), document, "tsconfig.json"), controller = new AbortController(), fresh = mkdtempSync(join(tmpdir(), "units-cancel-")), base = hostFor(fresh, "x");
    controller.abort();
    await expect(checkTypeScriptUnitsV1(plan, document, base, {concurrency: 1, signal: controller.signal})).rejects.toThrow();
    expect(existsSync(join(fresh, "store", "declarations")) ? readdirSync(join(fresh, "store", "declarations")).filter(name => !name.endsWith(".tmp")).length : 0).toBe(0);
    rmSync(fresh, {recursive: true, force: true});
  });
  test("cleanup", () => {rmSync(workspace, {recursive: true, force: true}); rmSync(state, {recursive: true, force: true});});
});
