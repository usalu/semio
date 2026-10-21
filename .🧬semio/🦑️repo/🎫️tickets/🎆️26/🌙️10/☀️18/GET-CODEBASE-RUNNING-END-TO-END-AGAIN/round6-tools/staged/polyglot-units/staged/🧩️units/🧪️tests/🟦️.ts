import {describe, expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import Ajv2020 from "ajv/dist/2020";
import {
  analyzePythonSourceV1, applyGoOverlayV1, bazelUnitsV1, canonicalJsonV1, checkPythonImportsV1, compareCodePointsV1, criticalPathV1, affectedUnitsV1, fileUnitsStoreV1, goConstraintHoldsV1, goExportDigestV1, goFileNameHoldsV1, goUnitExecutorV1,
  interfaceClosureV1, lexPythonV1, memoryUnitsStoreV1, nxUnitTargetsV1, outputClosureV1, parseGoModV1, parseGoWorkV1, planGoPackagesV1, planGoUnitsV1, planPythonUnitsV1, planUnitsV1, pythonUnitExecutorV1, readUnitGraphV1, readUnitsPolicyV1,
  runUnitsV1, rustUnitsFromMetadataV1, rustEntryOutcomeV1, scanGoSourceV1, sha256V1, typeScriptOutcomeV1, typeScriptPlanToUnitsV1, unitGraphFailuresV1, unitKeyV1, unitTargetNameV1, unitsMessageV1, unitsSchemaFailuresV1, UNITS_BZL_V1, condensationV1,
  type ExecuteRequestV1, type GoBuildContextV1, type UnitGraphV1, type UnitV1, type UnitsHostV1,
} from "../🟦️.ts";
import policyDocument from "../🔣️.json" with {type: "json"};
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import fixture from "./🔣️.json" with {type: "json"};

const policy = readUnitsPolicyV1(policyDocument);
const ORACLE = join(import.meta.dir, "🐍️oracle", "🐍️.py");
const oracle = (command: string, input: unknown): any => {
  const run = spawnSync("python3", ["-I", ORACLE, command], {input: JSON.stringify(input), encoding: "utf8", maxBuffer: 256 * 1024 * 1024});
  expect(run.status).toBe(0);
  return JSON.parse(run.stdout);
};
const have = (command: string, args: string[] = ["--version"]): boolean => spawnSync(command, args, {encoding: "utf8"}).status === 0;
const sortedJson = (rows: readonly unknown[]): string[] => rows.map(row => JSON.stringify(row)).sort();
const temporary = (): string => mkdtempSync(join(tmpdir(), "polyglot-units-"));
const noSignal = new AbortController().signal;

describe("schema, policy and graph laws", () => {
  test("the policy document closes against the schema and carries EN and DE texts", () => {
    expect(() => readUnitsPolicyV1({...policyDocument, extra: 1})).toThrow();
    expect(() => readUnitsPolicyV1({...policyDocument, key: {...policyDocument.key, order: "utf16"}})).toThrow();
    for (const language of policy.messages.languages) for (const name of Object.keys(policy.messages.texts) as (keyof typeof policy.messages.texts)[]) expect(unitsMessageV1(policy, language, name, {unit: "u", milliseconds: 1, dependency: "d", ran: 1, hit: 2, cutoff: 3, failed: 4})).toBeTruthy();
    expect(unitsMessageV1(policy, "de", "hit", {unit: "go:a"})).toBe("go:a: aus dem Cache");
    expect(unitsMessageV1(policy, "fr", "hit", {unit: "go:a"})).toBe("go:a: cached");
  });
  test("every language has an interface kind and every kind names its language and closure mode", () => {
    for (const language of policy.languages) expect(Object.values(policy.interfaceKinds).some(kind => kind.language === language)).toBe(true);
    for (const kind of Object.values(policy.interfaceKinds)) {expect(policy.languages).toContain(kind.language); expect(["transitive", "direct"]).toContain(kind.closure); expect(kind.artifact.length).toBeGreaterThan(0);}
  });
  test("the standard Ajv validator accepts and rejects exactly what the owned validator does", () => {
    const ajv = new Ajv2020({strict: false});
    const graphCheck = ajv.compile({definitions: schema.definitions, $ref: "#/definitions/Graph"}), policyCheck = ajv.compile({definitions: schema.definitions, $ref: "#/definitions/Policy"});
    for (const row of fixture.graphs as any[]) {
      const mine = unitsSchemaFailuresV1("Graph", row.graph).length === 0;
      expect(graphCheck(row.graph)).toBe(mine);
    }
    const broken = {version: 1, units: [{id: "go:a"}]};
    expect(graphCheck(broken)).toBe(false);
    expect(unitsSchemaFailuresV1("Graph", broken).length).toBeGreaterThan(0);
    expect(policyCheck(policyDocument)).toBe(true);
    expect(policyCheck({...policyDocument, languages: ["cobol"]})).toBe(false);
    expect(unitsSchemaFailuresV1("Policy", {...policyDocument, languages: ["cobol"]}).length).toBeGreaterThan(0);
  });
  const kinds = {
    cycle: "dependency cycle", "unknown-dependency": "unknown unit", "no-interface": "which has none", "shared-source": "is a source of both", kind: "uses interface kind", prefix: "does not start with its language", "duplicate-dependency": "twice", self: "depends on itself", duplicate: "duplicate unit",
  } as const;
  for (const row of fixture.graphs as any[]) test(`graph case ${row.name}`, () => {
    const failures = unitGraphFailuresV1(row.graph, policy);
    expect(failures.length === 0).toBe(row.expect.valid);
    if (!row.expect.valid) for (const kind of row.expect.kinds as (keyof typeof kinds)[]) expect(failures.some(text => text.includes(kinds[kind]))).toBe(true);
    else {
      const plan = planUnitsV1(readUnitGraphV1(row.graph, policy));
      expect([...plan.order]).toEqual(row.expect.order);
      expect(plan.layer).toEqual(row.expect.layer);
    }
  });
  test("the independent python oracle answers every graph case identically", () => {
    const answers = oracle("graph", fixture);
    for (const row of fixture.graphs as any[]) {
      const failures = unitGraphFailuresV1(row.graph, policy);
      expect(answers[row.name].valid).toBe(failures.length === 0);
      if (failures.length === 0) {const plan = planUnitsV1(row.graph); expect(answers[row.name].order).toEqual([...plan.order]); expect(answers[row.name].layer).toEqual(plan.layer);}
    }
  });
  test("planning helpers: affected units, critical path, condensation", () => {
    const row = (fixture.scenarios as any[])[0], plan = planUnitsV1(row.graph as UnitGraphV1);
    expect(affectedUnitsV1(plan, ["a.src"])).toEqual(["go:a", "go:b", "go:c", "go:c#test", "go:d"]);
    expect(affectedUnitsV1(plan, ["x.src"])).toEqual(["go:x"]);
    expect(affectedUnitsV1(plan, ["unrelated"])).toEqual([]);
    const weights: Record<string, number> = {"go:a": 5, "go:b": 1, "go:c": 1, "go:c#test": 10, "go:d": 2, "go:x": 100};
    expect(criticalPathV1(plan, id => weights[id]!)).toEqual({milliseconds: 100, path: ["go:x"]});
    expect(criticalPathV1(plan, id => id === "go:x" ? 0 : weights[id]!).path).toEqual(["go:a", "go:b", "go:c", "go:c#test"]);
    const adjacent = new Map<string, Set<string>>([["a", new Set(["b"])], ["b", new Set(["c"])], ["c", new Set(["a"])], ["d", new Set(["a"])]]);
    const {components, componentOf} = condensationV1(["a", "b", "c", "d"], adjacent);
    expect(components).toEqual([["a", "b", "c"], ["d"]]);
    expect(componentOf.get("c")).toBe("a");
  });
  test("code point order differs from UTF-16 order for astral characters", () => {
    expect(["🧰", "｡", "z"].sort()).toEqual(["z", "🧰", "｡"]);
    expect(["🧰", "｡", "z"].sort(compareCodePointsV1)).toEqual(["z", "｡", "🧰"]);
  });
});

describe("keys and closures", () => {
  const rows = (fixture.keys as any[]);
  for (const row of rows) test(`key vector ${row.name} equals the oracle value`, () => {
    const unit = row.unit as UnitV1, dependencies = new Map<string, {interfaceClosure: string | null; outputClosure: string}>(Object.entries(row.dependencies));
    expect(unitKeyV1(unit, {digest: path => row.digests[path], dependencies})).toBe(row.expect.key);
    expect(interfaceClosureV1(unit, row.interface, dependencies, policy, row.exposes ?? null)).toBe(row.expect.interfaceClosure);
    expect(outputClosureV1(unit, row.outputs, dependencies)).toBe(row.expect.outputClosure);
  });
  test("the python oracle reproduces the stored key vectors", () => {
    const answers = oracle("keys", fixture);
    for (const row of rows) expect(answers[row.name]).toEqual(row.expect);
  });
  test("the key moves with every declared part and ignores declared edges and unrelated dependency facts", () => {
    const row = rows[0], unit = row.unit as UnitV1, base = new Map<string, {interfaceClosure: string | null; outputClosure: string}>(Object.entries(row.dependencies));
    const key = (u: UnitV1, digests: Record<string, string> = row.digests, deps = base): string => unitKeyV1(u, {digest: path => digests[path]!, dependencies: deps});
    const original = key(unit);
    expect(key({...unit, toolchain: {...unit.toolchain, id: "rustc 1.96"}})).not.toBe(original);
    expect(key({...unit, action: {...unit.action, env: {...unit.action.env, C: "3"}}})).not.toBe(original);
    expect(key({...unit, externals: [...unit.externals, "x"]})).not.toBe(original);
    expect(key(unit, {...row.digests, "a/b.rs": sha256V1("changed")})).not.toBe(original);
    expect(key({...unit, sources: [...unit.sources].reverse(), externals: [...unit.externals].reverse()})).toBe(original);
    const moved = new Map(base); moved.set("rust:doc", {interfaceClosure: sha256V1("x"), outputClosure: sha256V1("y")});
    expect(key(unit, row.digests, moved)).toBe(original);
    const core = new Map(base); core.set("rust:core", {interfaceClosure: base.get("rust:core")!.interfaceClosure, outputClosure: sha256V1("other")});
    expect(key(unit, row.digests, core)).toBe(original);
    const mac = new Map(base); mac.set("rust:mac", {interfaceClosure: base.get("rust:mac")!.interfaceClosure, outputClosure: sha256V1("other")});
    expect(key(unit, row.digests, mac)).not.toBe(original);
    expect(() => key(unit, row.digests, new Map())).toThrow();
  });
  test("canonical JSON sorts keys by code point and refuses non-integers", () => {
    expect(canonicalJsonV1({b: 1, a: [true, null, "x"], "🧰": 2, "｡": 3})).toBe('{"a":[true,null,"x"],"b":1,"｡":3,"🧰":2}');
    expect(() => canonicalJsonV1({a: 1.5})).toThrow();
    expect(() => canonicalJsonV1({a: undefined})).toThrow();
  });
});

type Edit = {unit: string; effect: string};
type State = {body: number; iface: number; input: number; broken: boolean; hard: boolean};
/** The fake executor of the fixture text: sources hash to `path#body.iface`, inputs to `path#input`, outputs to `o|unit|body|iface|Broken`, the interface to `i|unit|iface` (none for a hard failure). */
const scenarioHost = (graph: UnitGraphV1, exposes: Record<string, string[]> | undefined) => {
  const state = new Map<string, State>(graph.units.map(unit => [unit.id, {body: 0, iface: 0, input: 0, broken: false, hard: false}])), initial = new Map([...state].map(([id, s]) => [id, {...s}]));
  const owner = new Map<string, {unit: string; input: boolean}>();
  for (const unit of graph.units) {for (const path of unit.sources) owner.set(path, {unit: unit.id, input: false}); for (const path of unit.inputs) owner.set(path, {unit: unit.id, input: true});}
  const executed: string[] = [];
  const host: UnitsHostV1 = {
    digest: path => {const row = owner.get(path)!, s = state.get(row.unit)!; return sha256V1(row.input ? `${path}#${s.input}` : `${path}#${s.body}.${s.iface}`);},
    execute: async (request: ExecuteRequestV1) => {
      const s = state.get(request.unit.id)!, id = request.unit.id;
      executed.push(id);
      return {ok: !s.broken, diagnostics: s.broken ? "error" : "", outputs: sha256V1(`o|${id}|${s.body}|${s.iface}|${s.broken ? "True" : "False"}`), interface: request.unit.interface && !s.hard ? sha256V1(`i|${id}|${s.iface}`) : null, exposes: exposes?.[id] ?? null, milliseconds: 1};
    },
  };
  const apply = (edits: readonly Edit[]): void => {
    for (const edit of edits) {
      const s = state.get(edit.unit)!;
      if (edit.effect === "body") s.body++;
      else if (edit.effect === "interface") {s.body++; s.iface++;}
      else if (edit.effect === "input") s.input++;
      else if (edit.effect === "fail") {s.body++; s.broken = true;}
      else if (edit.effect === "fail-hard") {s.body++; s.broken = true; s.hard = true;}
      else if (edit.effect === "fix") {s.body++; s.broken = false; s.hard = false;}
      else if (edit.effect === "revert") state.set(edit.unit, {...initial.get(edit.unit)!});
    }
  };
  return {host, apply, executed};
};
const summarize = (result: Awaited<ReturnType<typeof runUnitsV1>>) => {
  const ids = (status: string): string[] => result.units.filter(row => row.status === status).map(row => row.id).sort(compareCodePointsV1);
  return {ran: ids("ran"), hit: ids("hit"), failed: ids("failed"), blocked: ids("blocked"), cutoff: result.units.filter(row => row.cutoff).map(row => row.id).sort(compareCodePointsV1)};
};

describe("early cutoff engine", () => {
  for (const row of fixture.scenarios as any[]) test(`scenario ${row.name}: every step equals the hand-written expectation`, async () => {
    const plan = planUnitsV1(readUnitGraphV1(row.graph, policy)), store = memoryUnitsStore(), fake = scenarioHost(row.graph, row.exposes);
    for (const [index, step] of (row.steps as any[]).entries()) {
      fake.apply(step.edits);
      const before = fake.executed.length;
      const result = await runUnitsV1(plan, policy, fake.host, store, {concurrency: 3, signal: noSignal});
      expect(summarize(result)).toEqual(step.expect);
      const executed = new Set(fake.executed.slice(before));
      for (const id of step.expect.ran) expect(executed.has(id)).toBe(true);
      for (const id of step.expect.hit) expect(executed.has(id)).toBe(false);
      for (const id of step.expect.blocked) expect(executed.has(id)).toBe(false);
      expect(index).toBeGreaterThanOrEqual(0);
    }
  });
  test("the independent python oracle reproduces every scenario step", async () => {
    const answers = oracle("scenarios", fixture);
    for (const row of fixture.scenarios as any[]) for (const [index, step] of (row.steps as any[]).entries()) expect(answers[row.name][index]).toEqual(step.expect);
  });
  test("one signature edit re-checks the whole cone without exposure information but only its direct dependents with it", async () => {
    const chain = (fixture.scenarios as any[])[0], exposing = (fixture.scenarios as any[])[1];
    for (const [row, expectedRan] of [[chain, 5], [exposing, 2]] as const) {
      const plan = planUnitsV1(row.graph), store = memoryUnitsStore(), fake = scenarioHost(row.graph, row.exposes);
      await runUnitsV1(plan, policy, fake.host, store, {concurrency: 2, signal: noSignal});
      fake.apply([{unit: "go:a", effect: "interface"}]);
      const result = await runUnitsV1(plan, policy, fake.host, store, {concurrency: 2, signal: noSignal});
      expect(result.ran).toBe(expectedRan);
    }
  });
  test("concurrency is bounded by the option and independent units overlap", async () => {
    const graph: UnitGraphV1 = {version: 1, units: Array.from({length: 8}, (_, index) => ({id: `go:p${index}`, language: "go", kind: "library", label: "", sources: [`p${index}.go`], inputs: [], dependencies: [], externals: [], interface: {kind: "go-export-data"}, outputs: [], toolchain: {id: "t", parts: []}, action: {argv: ["x"], env: {}}}))};
    let running = 0, peak = 0;
    const host: UnitsHostV1 = {digest: path => sha256V1(path), execute: async () => {running++; peak = Math.max(peak, running); await new Promise(resolve => setTimeout(resolve, 15)); running--; return {ok: true, diagnostics: "", outputs: sha256V1("o"), interface: sha256V1("i")};}};
    await runUnitsV1(planUnitsV1(graph), policy, host, memoryUnitsStore(), {concurrency: 3, signal: noSignal});
    expect(peak).toBe(3);
  });
  test("progress reports every unit once with a rising counter, and `only` runs just the closure", async () => {
    const row = (fixture.scenarios as any[])[0], plan = planUnitsV1(row.graph), fake = scenarioHost(row.graph, undefined), seen: number[] = [], names: string[] = [];
    const result = await runUnitsV1(plan, policy, fake.host, memoryUnitsStore(), {concurrency: 2, signal: noSignal, only: ["go:c"], onProgress: progress => {seen.push(progress.done); names.push(progress.unit); expect(progress.total).toBe(3);}});
    expect(seen).toEqual([1, 2, 3]);
    expect(names.sort()).toEqual(["go:a", "go:b", "go:c"]);
    expect(result.units.map(unit => unit.id)).toEqual(["go:a", "go:b", "go:c"]);
    await expect(runUnitsV1(plan, policy, fake.host, memoryUnitsStore(), {concurrency: 1, signal: noSignal, only: ["go:nope"]})).rejects.toThrow();
  });
  test("cancellation stops execution, publishes no receipt for the aborted unit and marks dependents", async () => {
    const row = (fixture.scenarios as any[])[0], plan = planUnitsV1(row.graph), fake = scenarioHost(row.graph, undefined), store = memoryUnitsStore(), controller = new AbortController();
    const host: UnitsHostV1 = {digest: fake.host.digest, execute: async request => {controller.abort(); return fake.host.execute(request);}};
    const result = await runUnitsV1(plan, policy, host, store, {concurrency: 1, signal: controller.signal});
    expect(result.cancelled).toBe(true);
    expect(result.units.every(unit => unit.status === "cancelled")).toBe(true);
    expect(store.receipts.size).toBe(0);
  });
  test("an executor that contradicts the declared interface is refused", async () => {
    const graph: UnitGraphV1 = {version: 1, units: [{id: "go:a", language: "go", kind: "library", label: "", sources: ["a.go"], inputs: [], dependencies: [], externals: [], interface: {kind: "go-export-data"}, outputs: [], toolchain: {id: "t", parts: []}, action: {argv: ["x"], env: {}}}]};
    const host: UnitsHostV1 = {digest: () => sha256V1("a"), execute: async () => ({ok: true, diagnostics: "", outputs: sha256V1("o"), interface: null})};
    await expect(runUnitsV1(planUnitsV1(graph), policy, host, memoryUnitsStore(), {concurrency: 1, signal: noSignal})).rejects.toThrow("declares an interface");
  });
  test("the file store shares receipts between runs and sessions, ignores corrupt files and keeps latest facts", async () => {
    const row = (fixture.scenarios as any[])[0], plan = planUnitsV1(row.graph), directory = temporary();
    try {
      const first = scenarioHost(row.graph, undefined);
      await runUnitsV1(plan, policy, first.host, fileUnitsStoreV1(directory), {concurrency: 2, signal: noSignal});
      expect(first.executed.length).toBe(6);
      const second = scenarioHost(row.graph, undefined), again = await runUnitsV1(plan, policy, second.host, fileUnitsStoreV1(directory), {concurrency: 2, signal: noSignal});
      expect(second.executed.length).toBe(0);
      expect(again.hit).toBe(6);
      const receipts = readdirSync(join(directory, "receipts")).flatMap(prefix => readdirSync(join(directory, "receipts", prefix)).map(file => join(directory, "receipts", prefix, file)));
      expect(receipts.length).toBe(6);
      writeFileSync(receipts[0]!, "{not json");
      const third = scenarioHost(row.graph, undefined);
      await runUnitsV1(plan, policy, third.host, fileUnitsStoreV1(directory), {concurrency: 2, signal: noSignal});
      expect(third.executed.length).toBe(1);
    } finally {rmSync(directory, {recursive: true, force: true});}
  });
});
function memoryUnitsStore() {return memoryUnitsStoreV1();}
function unit(result: Awaited<ReturnType<typeof runUnitsV1>>, id: string) {return result.units.find(row => row.id === id)!;}

describe("emitters for Nx and Bazel", () => {
  const plan = planUnitsV1((fixture.scenarios as any[])[0].graph as UnitGraphV1);
  test("target names are pure, unique, ASCII and equal the oracle", () => {
    const answers = oracle("names", fixture);
    for (const row of fixture.names as any[]) {expect(unitTargetNameV1(row.id)).toBe(row.expect); expect(answers[row.id]).toBe(row.expect); expect(row.expect).toMatch(/^unit-[a-z0-9-]+-[0-9a-f]{8}$/);}
    expect(new Set(plan.order.map(unitTargetNameV1)).size).toBe(plan.order.length);
  });
  test("Nx targets list only the interface or output file of each dependency, never its sources", () => {
    const targets = nxUnitTargetsV1(plan, {outputRoot: ".cache/units", specRoot: "specs", command: ["repo", "units"]});
    expect(targets.map(target => target.unit)).toEqual([...plan.order]);
    for (const target of targets) {
      const unit = plan.byId.get(target.unit)!;
      expect(target.dependsOn).toEqual(unit.dependencies.map(dependency => unitTargetNameV1(dependency.unit)).sort());
      const own = new Set([...unit.sources, ...unit.inputs, `specs/${target.name}.json`].map(path => `{workspaceRoot}/${path}`));
      for (const input of target.inputs) if (typeof input === "string") expect(own.has(input)).toBe(true);
      const patterns = target.inputs.filter(input => typeof input !== "string").map(input => (input as {dependentTasksOutputFiles: string}).dependentTasksOutputFiles);
      expect(patterns.includes("**/iface.json")).toBe(unit.dependencies.some(dependency => dependency.use === "interface"));
      expect(patterns.includes("**/out.json")).toBe(unit.dependencies.some(dependency => dependency.use === "implementation"));
      expect(target.outputs.some(path => path.endsWith("/iface.json"))).toBe(unit.interface !== null);
      expect(target.command.slice(-5)).toEqual(["exec", "--spec", `specs/${target.name}.json`, "--out-root", ".cache/units"]);
      expect(target.inputs).toContain(`{workspaceRoot}/specs/${target.name}.json`);
    }
  });
  test("Bazel text: one repo_unit per unit, interface dependencies only as iface_deps, balanced, deterministic", () => {
    const text = bazelUnitsV1(plan, {runner: "//:repo", load: "//:units.bzl", specRoot: "specs", policy});
    expect(text).toBe(bazelUnitsV1(plan, {runner: "//:repo", load: "//:units.bzl", specRoot: "specs", policy}));
    expect(text.split("repo_unit(").length - 1).toBe(plan.order.length + 0);
    expect(text.split("(").length).toBe(text.split(")").length);
    expect(text.startsWith('load("//:units.bzl", "repo_unit")')).toBe(true);
    const blocks = text.split("\nrepo_unit(").slice(1);
    for (const block of blocks) {
      const id = /unit = "([^"]+)"/.exec(block)![1]!, unit = plan.byId.get(id)!;
      const iface = /iface_deps = \[([^\]]*)\]/.exec(block)?.[1] ?? "", impl = /impl_deps = \[([^\]]*)\]/.exec(block)?.[1] ?? "";
      for (const dependency of unit.dependencies) {
        const label = JSON.stringify(":" + unitTargetNameV1(dependency.unit));
        expect(iface.includes(label)).toBe(dependency.use === "interface");
        expect(impl.includes(label)).toBe(dependency.use === "implementation");
      }
      expect(block.includes("has_interface = True")).toBe(unit.interface !== null);
    }
    const goText = text, pythonGraph: UnitGraphV1 = {version: 1, units: plan.graph.units.map(unit => ({...unit, language: "python" as const, id: unit.id.replace("go:", "python:"), interface: unit.interface ? {kind: "python-api"} : null, dependencies: unit.dependencies.map(dependency => ({...dependency, unit: dependency.unit.replace("go:", "python:")}))}))};
    const pythonText = bazelUnitsV1(planUnitsV1(pythonGraph), {runner: "//:repo", load: "//:units.bzl", specRoot: "specs", policy});
    expect(goText.includes("dep_srcs")).toBe(true);
    expect(pythonText.includes("dep_srcs")).toBe(false);
    const blockOf = (id: string): string => goText.split("\nrepo_unit(").find(block => block.includes(`unit = "${id}"`))!;
    expect(blockOf("go:b")).toContain('"a.src"');
    expect(blockOf("go:a")).not.toContain("dep_srcs");
    expect(UNITS_BZL_V1).toContain("repo_unit = rule(");
    expect(UNITS_BZL_V1).toContain("use_default_shell_env = True");
    expect(UNITS_BZL_V1.split("(").length).toBe(UNITS_BZL_V1.split(")").length);
    expect(UNITS_BZL_V1.split("[").length).toBe(UNITS_BZL_V1.split("]").length);
  });
});

describe("go adapter", () => {
  const context = (row: any): GoBuildContextV1 => row;
  for (const row of (fixture.go.scan as any[])) test(`scan vector ${row.name}`, () => expect({...scanGoSourceV1(row.source)}).toEqual(row.expect));
  for (const row of (fixture.go.constraints as any[])) test(`constraint ${row.expr} on ${row.context.goos}/${row.context.goarch}`, () => expect(goConstraintHoldsV1(row.expr, context(row.context), policy)).toBe(row.expect));
  for (const row of (fixture.go.names as any[])) test(`file name ${row.name} on ${row.context.goos}/${row.context.goarch}`, () => expect(goFileNameHoldsV1(row.name, context(row.context), policy)).toBe(row.expect));
  test("go.mod and go.work parse", () => {
    for (const row of fixture.go.gomod as any[]) expect(JSON.parse(JSON.stringify(parseGoModV1(row.text)))).toEqual(row.expect);
    for (const row of fixture.go.work as any[]) expect(JSON.parse(JSON.stringify(parseGoWorkV1(row.text)))).toEqual(row.expect);
  });
  test("the build lists cover every platform of `go tool dist list`", () => {
    if (!have("go", ["version"])) return;
    const run = spawnSync("go", ["tool", "dist", "list"], {encoding: "utf8"});
    for (const line of run.stdout.split("\n").filter(Boolean)) {const [os, arch] = line.split("/"); expect(policy.go.goos).toContain(os!); expect(policy.go.goarch).toContain(arch!);}
  });
  const planned = (row: any) => {
    const files = row.files as Record<string, string>;
    return planGoUnitsV1({files: Object.keys(files).sort(), read: path => files[path]!}, {policy, context: context(row.context), toolchain: "go1.25.0 linux/amd64", tests: true});
  };
  for (const row of fixture.go.plan as any[]) test(`plan vector ${row.name}: packages and units`, () => {
    const {graph, packages} = planned(row);
    readUnitGraphV1(graph, policy);
    for (const [path, expected] of Object.entries(row.expect.packages) as [string, any][]) {
      const found = packages.find(item => item.importPath === path)!;
      expect(found.name).toBe(expected.name);
      expect([...found.goFiles]).toEqual(expected.goFiles);
      expect([...found.testGoFiles]).toEqual(expected.testGoFiles ?? []);
      expect([...found.xtestGoFiles]).toEqual(expected.xtestGoFiles ?? []);
      expect([...found.imports]).toEqual(expected.imports);
    }
    expect(graph.units.map(unit => unit.id).sort(compareCodePointsV1)).toEqual(Object.keys(row.expect.units).sort(compareCodePointsV1));
    for (const [id, expected] of Object.entries(row.expect.units) as [string, any][]) {
      const found = graph.units.find(unit => unit.id === id)!;
      expect(found.kind).toBe(expected.kind);
      expect(found.dependencies.map(dependency => [dependency.unit, dependency.use])).toEqual(expected.dependencies);
      expect([...found.externals]).toEqual(expected.externals);
      expect([...found.sources]).toEqual(expected.sources);
      expect([...found.inputs]).toEqual(expected.inputs ?? []);
      expect(found.interface === null).toBe(expected.kind !== "library");
    }
  });
  test("go list agrees with the source-derived packages of the fixture workspace (third-party oracle)", () => {
    if (!have("go", ["version"])) return;
    const row = (fixture.go.plan as any[])[0], directory = temporary();
    try {
      for (const [path, text] of Object.entries(row.files as Record<string, string>)) {mkdirSync(dirname(join(directory, path)), {recursive: true}); writeFileSync(join(directory, path), text);}
      const env = {...process.env, GOFLAGS: "-mod=mod", GOPROXY: "off", GOCACHE: join(directory, ".gocache"), GOPATH: join(directory, ".gopath"), GOOS: row.context.goos, GOARCH: row.context.goarch, CGO_ENABLED: row.context.cgo ? "1" : "0", GOWORK: "off"};
      const list = spawnSync("go", ["list", "-json", "-e", "./..."], {cwd: directory, env, encoding: "utf8", maxBuffer: 64 * 1024 * 1024});
      const listed = (list.stdout.replace(/}\s*{/g, "}\n{").split("\n{").map((text, index) => index === 0 ? text : "{" + text)).filter(text => text.trim()).map(text => JSON.parse(text));
      const {packages} = planned(row);
      const own = packages.filter(item => item.module === "example.com/lab");
      expect(listed.map(item => item.ImportPath).sort()).toEqual(own.map(item => item.importPath).sort());
      for (const item of listed) {
        const mine = own.find(entry => entry.importPath === item.ImportPath)!, base = (paths: readonly string[]): string[] => paths.map(path => path.slice(path.lastIndexOf("/") + 1)).sort();
        expect(mine.name).toBe(item.Name);
        expect(base([...mine.goFiles, ...mine.cgoFiles])).toEqual([...(item.GoFiles ?? []), ...(item.CgoFiles ?? [])].sort());
        expect(base(mine.testGoFiles)).toEqual([...(item.TestGoFiles ?? [])].sort());
        expect(base(mine.xtestGoFiles)).toEqual([...(item.XTestGoFiles ?? [])].sort());
        expect(base(mine.ignoredGoFiles)).toEqual([...(item.IgnoredGoFiles ?? [])].sort());
        expect([...mine.imports]).toEqual([...(item.Imports ?? [])].sort(compareCodePointsV1).filter((value, index, all) => all.indexOf(value) === index));
        expect([...mine.testImports]).toEqual([...(item.TestImports ?? [])].sort(compareCodePointsV1));
        expect([...mine.xtestImports]).toEqual([...(item.XTestImports ?? [])].sort(compareCodePointsV1));
      }
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 180000);
  test("tags and file names agree with go list for the same files on another platform", () => {
    if (!have("go", ["version"])) return;
    const directory = temporary(), files: Record<string, string> = {
      "go.mod": "module example.com/tags\n\ngo 1.25\n",
      "t/a.go": "package t\n", "t/a_linux.go": "package t\n", "t/a_darwin_arm64.go": "package t\n", "t/a_windows.go": "package t\n", "t/b.go": "//go:build !linux && cgo\n\npackage t\n",
      "t/c.go": "//go:build (darwin || ios) && go1.18 && !go1.99\n\npackage t\n", "t/d.go": "// +build linux darwin\n\npackage t\n", "t/e.go": "//go:build ignore\n\npackage main\n", "t/f_unix.go": "package t\n", "t/g.go": "package t\n\nimport \"C\"\n",
    };
    try {
      for (const [path, text] of Object.entries(files)) {mkdirSync(dirname(join(directory, path)), {recursive: true}); writeFileSync(join(directory, path), text);}
      for (const target of [{goos: "linux", goarch: "amd64", cgo: true}, {goos: "darwin", goarch: "arm64", cgo: false}, {goos: "windows", goarch: "amd64", cgo: false}, {goos: "ios", goarch: "arm64", cgo: true}]) {
        const env = {...process.env, GOFLAGS: "-mod=mod", GOPROXY: "off", GOCACHE: join(directory, ".gocache"), GOPATH: join(directory, ".gopath"), GOOS: target.goos, GOARCH: target.goarch, CGO_ENABLED: target.cgo ? "1" : "0", GOWORK: "off"};
        const list = spawnSync("go", ["list", "-json", "-e", "./t"], {cwd: directory, env, encoding: "utf8"});
        const item = JSON.parse(list.stdout) as {GoFiles?: string[]; CgoFiles?: string[]; IgnoredGoFiles?: string[]};
        const {packages} = planGoPackagesV1({files: Object.keys(files), read: path => files[path]!}, {policy, context: {...target, goMinor: 25, tags: []}});
        const mine = packages.find(row => row.importPath === "example.com/tags/t");
        const base = (paths: readonly string[]): string[] => paths.map(path => path.slice(path.lastIndexOf("/") + 1)).sort();
        expect(base([...(mine?.goFiles ?? []), ...(mine?.cgoFiles ?? [])])).toEqual([...(item.GoFiles ?? []), ...(item.CgoFiles ?? [])].sort());
      }
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 180000);
  test("export data digests: a body edit keeps it, a signature edit moves it, go list -export agrees", () => {
    if (!have("go", ["version"])) return;
    const directory = temporary(), cache = join(directory, ".gocache");
    const source = (body: string, signature: string): string => `package a\n\nimport "strings"\n\nfunc Big(s string) string {\n\tvar out []string\n\tfor i := 0; i < 10; i++ {\n\t\tout = append(out, strings.ToUpper(s))\n\t\tout = append(out, "${body}")\n\t}\n\treturn strings.Join(out, ",")\n}\n\nfunc Small(${signature}) int { return 1 }\n`;
    const digestOf = (): {built: string; listed: string} => {
      const env = {...process.env, GOFLAGS: "-mod=mod", GOPROXY: "off", GOCACHE: cache, GOPATH: join(directory, ".gopath"), GOWORK: "off"};
      const out = join(directory, "a.a");
      expect(spawnSync("go", ["build", "-trimpath", "-o", out, "./a"], {cwd: directory, env, encoding: "utf8"}).status).toBe(0);
      const listed = spawnSync("go", ["list", "-export", "-trimpath", "-f", "{{.Export}}", "./a"], {cwd: directory, env, encoding: "utf8"}).stdout.trim();
      return {built: goExportDigestV1(readFileSync(out)), listed: goExportDigestV1(readFileSync(listed))};
    };
    try {
      mkdirSync(join(directory, "a"), {recursive: true});
      writeFileSync(join(directory, "go.mod"), "module example.com/lab\n\ngo 1.25\n");
      const write = (body: string, signature: string): void => writeFileSync(join(directory, "a", "a.go"), source(body, signature));
      write("x", "x int");
      const base = digestOf();
      expect(base.built).toBe(base.listed);
      write("y", "x int");
      const body = digestOf();
      expect(body.built).toBe(base.built);
      write("y", "x int64");
      const signature = digestOf();
      expect(signature.built).not.toBe(base.built);
      expect(signature.built).toBe(signature.listed);
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 180000);
  test("engine end to end with the real go tool: a body edit cuts off at the library, a signature edit re-checks the importers", async () => {
    if (!have("go", ["version"])) return;
    const directory = temporary(), store = join(directory, ".store"), out = join(directory, ".out");
    const files: Record<string, string> = {
      "go.mod": "module example.com/lab\n\ngo 1.25\n",
      "a/a.go": "package a\n\nimport \"strings\"\n\nfunc Big(s string) string {\n\tvar out []string\n\tfor i := 0; i < 10; i++ {\n\t\tout = append(out, strings.ToUpper(s))\n\t}\n\treturn strings.Join(out, \",\")\n}\n\nfunc Small(x int) int { return x + 1 }\n",
      "a/a_test.go": "package a\n\nimport \"testing\"\n\nfunc TestBig(t *testing.T) { if Big(\"q\") == \"\" { t.Fatal(\"empty\") } }\n",
      "b/b.go": "package b\n\nimport \"example.com/lab/a\"\n\nfunc Use() string { return a.Big(\"q\") }\n",
      "c/c.go": "package main\n\nimport \"example.com/lab/b\"\n\nfunc main() { println(b.Use()) }\n",
    };
    try {
      for (const [path, text] of Object.entries(files)) {mkdirSync(dirname(join(directory, path)), {recursive: true}); writeFileSync(join(directory, path), text);}
      const env = {GOFLAGS: "-mod=mod", GOPROXY: "off", GOCACHE: join(directory, ".gocache"), GOPATH: join(directory, ".gopath"), GOWORK: "off"};
      const run = async (argv: readonly string[], options: {cwd: string; env: Readonly<Record<string, string>>}) => {const result = spawnSync(argv[0]!, argv.slice(1), {cwd: options.cwd, env: {...process.env, ...env, ...options.env}, encoding: "utf8"}); return {code: result.status ?? 1, output: (result.stdout ?? "") + (result.stderr ?? "")};};
      const snapshot = () => {const current = Object.fromEntries(Object.keys(files).map(path => [path, readFileSync(join(directory, path), "utf8")])); return {files: Object.keys(files).sort(), read: (path: string) => current[path]!};};
      const goContext = {goos: process.platform === "darwin" ? "darwin" : "linux", goarch: process.arch === "arm64" ? "arm64" : "amd64", goMinor: 25, cgo: false, tags: []};
      const go = (): UnitsHostV1 => {
        const executor = goUnitExecutorV1({workspace: directory, outputRoot: out, run: (argv, options) => run(argv, options)});
        return {digest: path => sha256V1(readFileSync(join(directory, path))), execute: executor};
      };
      const round = async () => {
        const {graph} = planGoUnitsV1(snapshot(), {policy, context: goContext, toolchain: "go1.25 test", tests: true}), plan = planUnitsV1(readUnitGraphV1(graph, policy));
        return summarize(await runUnitsV1(plan, policy, go(), fileUnitsStoreV1(store), {concurrency: 2, signal: noSignal}));
      };
      const cold = await round();
      expect(cold.failed).toEqual([]);
      expect(cold.ran).toEqual(["go:example.com/lab/a", "go:example.com/lab/a#test", "go:example.com/lab/b", "go:example.com/lab/c"]);
      expect((await round()).ran).toEqual([]);
      writeFileSync(join(directory, "a/a.go"), files["a/a.go"]!.replace("strings.ToUpper(s)", "strings.ToLower(s)"));
      const body = await round();
      expect(body.ran).toEqual(["go:example.com/lab/a", "go:example.com/lab/a#test"]);
      expect(body.cutoff).toEqual(["go:example.com/lab/a"]);
      expect(body.hit).toEqual(["go:example.com/lab/b", "go:example.com/lab/c"]);
      writeFileSync(join(directory, "a/a.go"), files["a/a.go"]!.replace("strings.ToUpper(s)", "strings.ToLower(s)").replace("func Small(x int) int", "func Small(x int64) int64"));
      const signature = await round();
      expect(signature.ran).toContain("go:example.com/lab/b");
      expect(signature.cutoff).not.toContain("go:example.com/lab/a");
      writeFileSync(join(directory, "a/a.go"), "package a\n\nfunc Big(s string) string {\n");
      const broken = await round();
      expect(broken.failed).toContain("go:example.com/lab/a");
      expect(broken.blocked.length).toBeGreaterThan(0);
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 120000);
  test("optional: the packages of every real module of this repository equal go list under the canonical overlay (set POLYGLOT_REAL_REPO=1)", async () => {
    if (process.env.POLYGLOT_REAL_REPO !== "1" || !have("go", ["version"])) return;
    const root = process.env.POLYGLOT_REPO_ROOT ?? join(import.meta.dir, "../".repeat(9));
    const tracked = spawnSync("git", ["ls-files", "-z", "--", "*.go", "*go.mod", "go.work", "*.c", "*.h", "*.s"], {cwd: root, encoding: "utf8", maxBuffer: 256 * 1024 * 1024}).stdout.split("\0").filter(Boolean);
    const defaults = spawnSync("go", ["env", "GOOS", "GOARCH", "CGO_ENABLED"], {cwd: root, encoding: "utf8"}).stdout.trim().split("\n");
    const ctx: GoBuildContextV1 = {goos: defaults[0]!, goarch: defaults[1]!, goMinor: Number(/go1\.(\d+)/.exec(spawnSync("go", ["version"], {encoding: "utf8"}).stdout)![1]), cgo: defaults[2] === "1", tags: []};
    const cache = new Map<string, string>(), read = (path: string): string => {let text = cache.get(path); if (text === undefined) {text = readFileSync(join(root, path), "utf8"); cache.set(path, text);} return text;};
    const moduleDirs = tracked.filter(path => path === "go.mod" || path.endsWith("/go.mod")).map(path => path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : "");
    const scratch = temporary(), overlayFile = join(scratch, "overlay.json");
    try {
      const {canonicalGoOverlayV1} = await import("../⌨️command/🟦️.ts");
      const {replace, excludedDirectories} = await canonicalGoOverlayV1(root, moduleDirs.map(dir => join(root, dir)));
      writeFileSync(overlayFile, JSON.stringify({Replace: replace}));
      const snapshot = applyGoOverlayV1({files: tracked, read}, root, replace, excludedDirectories);
      const {packages, modules} = planGoPackagesV1(snapshot, {policy, context: ctx});
      let compared = 0, skipped = 0;
      for (const module of modules) {
        const cwd = join(root, module.dir), list = spawnSync("go", ["list", `-overlay=${overlayFile}`, "-json", "-e", "./..."], {cwd, env: {...process.env, GOFLAGS: "", GOPROXY: "off", GOTOOLCHAIN: "local", ...(module.workspace ? {} : {GOWORK: "off"})}, encoding: "utf8", maxBuffer: 256 * 1024 * 1024});
        const items = list.stdout.replace(/}\s*{/g, "}\n{").split("\n{").map((text, index) => index === 0 ? text : "{" + text).filter(text => text.trim()).map(text => JSON.parse(text));
        const mine = packages.filter(item => item.module === module.path);
        const base = (paths: readonly string[]): string[] => paths.map(path => path.slice(path.lastIndexOf("/") + 1)).sort();
        for (let index = items.length - 1; index >= 0; index--) if (items[index].Error?.Err?.includes("malformed import path")) {skipped++; items.splice(index, 1);}
        expect(items.map(item => item.ImportPath).sort()).toEqual(mine.filter(item => !items.every(entry => entry.ImportPath !== item.importPath) || /^[A-Za-z0-9._~\/+-]+$/.test(item.importPath)).map(item => item.importPath).sort());
        for (const item of items) {
          const found = mine.find(entry => entry.importPath === item.ImportPath)!;
          expect(base([...found.goFiles, ...found.cgoFiles])).toEqual([...(item.GoFiles ?? []), ...(item.CgoFiles ?? [])].sort());
          expect(base(found.testGoFiles)).toEqual([...(item.TestGoFiles ?? [])].sort());
          expect(base(found.xtestGoFiles)).toEqual([...(item.XTestGoFiles ?? [])].sort());
          expect(base(found.ignoredGoFiles)).toEqual([...(item.IgnoredGoFiles ?? [])].sort());
          expect([...found.imports]).toEqual([...new Set<string>(item.Imports ?? [])].sort(compareCodePointsV1));
          expect([...found.testImports]).toEqual([...new Set<string>(item.TestImports ?? [])].sort(compareCodePointsV1));
          expect([...found.xtestImports]).toEqual([...new Set<string>(item.XTestImports ?? [])].sort(compareCodePointsV1));
          compared++;
        }
      }
      console.log(`[real repo, canonical overlay] modules ${modules.length}, packages planned ${packages.length}, compared with go list ${compared}, refused by go list ${skipped}`);
      expect(compared).toBeGreaterThan(0);
    } finally {rmSync(scratch, {recursive: true, force: true});}
  }, 900000);
});

describe("python adapter", () => {
  for (const row of fixture.python.analyze as any[]) test(`analysis vector ${row.name}`, () => {
    const facts = analyzePythonSourceV1(row.source, {init: row.init ?? false, unannotatedReturns: "ignore"});
    expect(facts.lexicalError).toBeNull();
    expect(sortedJson(facts.imports.map(item => [item.module, item.level, item.names]))).toEqual(sortedJson(row.expect.imports));
    expect([...facts.names]).toEqual(row.expect.names);
    expect(facts.dynamic).toBe(row.expect.dynamic);
  });
  test("the ast oracle agrees on imports, bound names and dynamic markers", () => {
    const answers = oracle("python", fixture);
    for (const row of fixture.python.analyze as any[]) {
      const facts = analyzePythonSourceV1(row.source, {init: row.init ?? false, unannotatedReturns: "ignore"});
      expect(sortedJson(answers[row.name].imports)).toEqual(sortedJson(facts.imports.map(item => [item.module, item.level, item.names])));
      expect(answers[row.name].names).toEqual([...facts.names]);
      expect(answers[row.name].dynamic).toBe(facts.dynamic);
    }
  });
  for (const row of fixture.python.pairs as any[]) test(`interface pair ${row.name}: ${row.same ? "same" : "different"}`, () => {
    const options = {init: row.init ?? false, unannotatedReturns: (row.unannotatedReturns ?? "ignore") as "ignore" | "body"};
    expect(analyzePythonSourceV1(row.before, options).api === analyzePythonSourceV1(row.after, options).api).toBe(row.same);
  });
  test("the ast oracle reaches the same verdict for every pair", () => {
    const answers = oracle("python", fixture);
    for (const row of fixture.python.pairs as any[]) expect(answers[row.name].same).toBe(row.same);
  });
  test("lexical errors are reported, valid sources are clean", () => {
    expect(lexPythonV1("x = 'abc\n").error).toContain("unterminated string");
    expect(lexPythonV1("x = '''abc\n").error).toContain("triple-quoted");
    expect(lexPythonV1("f(1,\n").error).toContain("EOF in multi-line");
    expect(lexPythonV1("if x:\n        a\n    b\n").error).toContain("unindent");
    expect(lexPythonV1("x = 1 $ 2\n").error).toContain("invalid character");
    expect(lexPythonV1("def f():\n\tpass\n").error).toBeNull();
    expect(lexPythonV1("x = (1,\n 2)\n# c\ny = 3").lines.map(line => line.tokens.map(token => token.text).join(" "))).toEqual(["x = ( 1 , 2 )", "y = 3"]);
  });
  const stdlib = (): string | undefined => {const run = spawnSync("python3", ["-I", "-c", "import sysconfig;print(sysconfig.get_paths()['stdlib'])"], {encoding: "utf8"}); return run.status === 0 ? run.stdout.trim() : undefined;};
  const corpus = (limit: number): string[] => {
    const root = stdlib();
    if (!root || !existsSync(root)) return [];
    const files: string[] = [];
    const walk = (directory: string, depth: number): void => {for (const entry of readdirSync(directory, {withFileTypes: true}).sort((a, b) => a.name < b.name ? -1 : 1)) {const path = join(directory, entry.name); if (entry.isDirectory()) {if (depth < 2 && !["site-packages", "test", "tests", "idlelib", "lib2to3", "__pycache__", "config-3.9-darwin", "turtledemo"].includes(entry.name)) walk(path, depth + 1);} else if (entry.name.endsWith(".py")) files.push(path);}};
    walk(root, 0);
    return files.filter((_, index) => index % Math.max(1, Math.floor(files.length / limit)) === 0).slice(0, limit);
  };
  test("the tokenizer reproduces the standard library tokenizer on a stdlib sample", () => {
    const files = corpus(120);
    if (!files.length) return;
    const run = spawnSync("python3", ["-I", ORACLE, "lexer", ...files], {encoding: "utf8", maxBuffer: 512 * 1024 * 1024});
    expect(run.status).toBe(0);
    const expected = JSON.parse(run.stdout) as Record<string, string[]>;
    let compared = 0;
    for (const file of files) {
      const lexed = lexPythonV1(readFileSync(file, "utf8"));
      if (!expected[file]) continue;
      expect(lexed.error).toBeNull();
      expect(lexed.lines.flatMap(line => line.tokens.map(token => token.text))).toEqual(expected[file]!);
      compared++;
    }
    expect(compared).toBeGreaterThan(50);
  }, 120000);
  test("stdlib sample: imports equal ast, bound names cover ast, body and comment edits keep the interface, a rename moves it, verdicts equal the ast oracle", () => {
    const files = corpus(100);
    if (!files.length) return;
    const directory = temporary();
    try {
      const run = spawnSync("python3", ["-I", ORACLE, "corpus", directory, ...files], {encoding: "utf8", maxBuffer: 256 * 1024 * 1024});
      expect(run.status).toBe(0);
      const manifest = JSON.parse(readFileSync(join(directory, "manifest.json"), "utf8")) as {path: string; init: boolean; imports: unknown[]; names: string[]; dynamic: boolean; mutations: Record<string, {file: string; same: boolean}>}[];
      expect(manifest.length).toBeGreaterThan(50);
      const tally = {extraNames: 0, mutated: 0, bodySame: 0, renameDifferent: 0, agree: 0};
      for (const row of manifest) {
        const text = readFileSync(row.path, "utf8"), options = {init: row.init, unannotatedReturns: "ignore" as const}, facts = analyzePythonSourceV1(text, options);
        expect(facts.lexicalError).toBeNull();
        expect(sortedJson(facts.imports.map(item => [item.module, item.level, item.names]))).toEqual(sortedJson(row.imports));
        for (const name of row.names) expect(facts.names).toContain(name);
        expect(facts.dynamic).toBe(row.dynamic);
        tally.extraNames += facts.names.length - row.names.length;
        for (const [kind, mutation] of Object.entries(row.mutations)) {
          const mutated = analyzePythonSourceV1(readFileSync(mutation.file, "utf8"), options);
          const same = mutated.api === facts.api;
          expect(same).toBe(mutation.same);
          tally.agree++; tally.mutated++;
          if (kind === "A" || kind === "B") {expect(same).toBe(true); tally.bodySame++;}
          if (kind === "C") {expect(same).toBe(false); tally.renameDifferent++;}
        }
      }
      expect(tally.bodySame).toBeGreaterThan(50);
      expect(tally.renameDifferent).toBeGreaterThan(10);
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 240000);
  test("no false import rejections across the standard library treated as one workspace", () => {
    const root = stdlib();
    if (!root || !existsSync(root)) return;
    const files: string[] = [];
    const walk = (directory: string, base: string): void => {for (const entry of readdirSync(directory, {withFileTypes: true})) {const path = join(directory, entry.name), relative = base ? base + "/" + entry.name : entry.name; if (entry.isDirectory()) {if (!["site-packages", "test", "tests", "idlelib", "lib2to3", "__pycache__", "lib-dynload", "turtledemo", "ensurepip"].includes(entry.name)) walk(path, relative);} else if (entry.name.endsWith(".py")) files.push(relative);}};
    walk(root, "");
    const texts = new Map<string, string>(), read = (path: string): string => {let text = texts.get(path); if (text === undefined) {text = readFileSync(join(root, path), "utf8"); texts.set(path, text);} return text;};
    const {modules} = planPythonUnitsV1({files, read}, {policy, roots: [""], toolchain: "python test", pins: {}});
    const byName = new Map(modules.map(module => [module.name, module] as const)), analyzed = new Map<string, ReturnType<typeof analyzePythonSourceV1>>();
    const facts = (module: {file: string; package: boolean}) => {let found = analyzed.get(module.file); if (!found) {found = analyzePythonSourceV1(read(module.file), {init: module.package, unannotatedReturns: "ignore"}); analyzed.set(module.file, found);} return found;};
    const diagnostics = checkPythonImportsV1(modules.map(module => ({file: module.file, module, package: module.package, imports: facts(module).imports})), byName, module => {const found = facts(module); return {names: new Set(found.names), dynamic: found.dynamic};});
    expect(diagnostics).toEqual([]);
    expect(modules.length).toBeGreaterThan(100);
    const probe = checkPythonImportsV1([{file: "probe.py", module: undefined, package: false, imports: [{module: "json", level: 0, names: ["definitely_not_there"], bound: [], line: 3, lazy: false, guarded: false}]}], byName, module => {const found = facts(module); return {names: new Set(found.names), dynamic: found.dynamic};});
    expect(probe).toEqual(["probe.py:3: cannot import name 'definitely_not_there' from 'json'"]);
  }, 240000);
  for (const row of fixture.python.plan as any[]) test(`plan vector ${row.name}`, () => {
    const files = row.files as Record<string, string>;
    const {graph, modules} = planPythonUnitsV1({files: Object.keys(files).sort(), read: path => files[path]!}, {policy, roots: row.roots, toolchain: "python3.9 test", pins: row.pins});
    readUnitGraphV1(graph, policy);
    expect(graph.units.map(unit => unit.id).sort(compareCodePointsV1)).toEqual(Object.keys(row.expect.units).sort(compareCodePointsV1));
    for (const [id, expected] of Object.entries(row.expect.units) as [string, any][]) {
      const found = graph.units.find(unit => unit.id === id)!;
      expect([...found.sources]).toEqual(expected.sources);
      expect(found.dependencies.map(dependency => dependency.unit)).toEqual(expected.dependencies);
      expect(found.dependencies.every(dependency => dependency.use === "interface")).toBe(true);
      expect([...found.externals]).toEqual(expected.externals);
    }
    expect(modules.find(module => module.name === "pkg.b")!.unit).toBe("python:pkg.b");
    expect(modules.find(module => module.name === "cyc2")!.unit).toBe("python:cycle:cyc1");
    expect(modules.find(module => module.name === "lib.n")!.stub).toBe("src/lib/n.pyi");
  });
  test("engine end to end with real python3: body edit cuts off, signature edit re-checks importers, a removed name is found, py_compile and runtime import agree", async () => {
    if (!have("python3")) return;
    const directory = temporary(), store = join(directory, ".store"), out = join(directory, ".out");
    const files: Record<string, string> = {
      "lab/__init__.py": "",
      "lab/a.py": "def big(x):\n    return x + 1\n\nCONSTANT = 3\n",
      "lab/b.py": "from lab.a import big, CONSTANT\n\ndef use():\n    return big(CONSTANT)\n",
      "lab/c.py": "from lab.b import use\n\nRESULT = use()\n",
    };
    try {
      const write = (): void => {for (const [path, text] of Object.entries(files)) {mkdirSync(dirname(join(directory, path)), {recursive: true}); writeFileSync(join(directory, path), text);}};
      write();
      const run = async (argv: readonly string[], options: {cwd: string; env: Readonly<Record<string, string>>}) => {const result = spawnSync(argv[0]!, argv.slice(1), {cwd: options.cwd, env: {...process.env, ...options.env}, encoding: "utf8"}); return {code: result.status ?? 1, output: (result.stdout ?? "") + (result.stderr ?? "")};};
      const round = async () => {
        const planned = planPythonUnitsV1({files: Object.keys(files).sort(), read: path => readFileSync(join(directory, path), "utf8")}, {policy, roots: [""], toolchain: "python3 test", pins: {}}), plan = planUnitsV1(readUnitGraphV1(planned.graph, policy));
        const executor = pythonUnitExecutorV1({workspace: directory, outputRoot: out, policy, modules: planned.modules, read: path => readFileSync(join(directory, path), "utf8"), run});
        const host: UnitsHostV1 = {digest: path => sha256V1(readFileSync(join(directory, path))), execute: executor};
        return runUnitsV1(plan, policy, host, fileUnitsStoreV1(store), {concurrency: 2, signal: noSignal});
      };
      const cold = summarize(await round());
      expect(cold.failed).toEqual([]);
      expect(cold.ran.length).toBe(4);
      expect(summarize(await round()).ran).toEqual([]);
      files["lab/a.py"] = files["lab/a.py"]!.replace("x + 1", "x + 2");
      write();
      const body = summarize(await round());
      expect(body.ran).toEqual(["python:lab.a"]);
      expect(body.cutoff).toEqual(["python:lab.a"]);
      files["lab/a.py"] = files["lab/a.py"]!.replace("def big(x):", "def big(x, y=0):");
      write();
      const signature = summarize(await round());
      expect(signature.ran).toContain("python:lab.b");
      files["lab/a.py"] = "CONSTANT = 3\n";
      write();
      const removed = await round();
      const b = removed.units.find(row => row.id === "python:lab.b")!;
      expect(b.status).toBe("failed");
      expect(b.diagnostics).toContain("cannot import name 'big' from 'lab.a'");
      const runtime = spawnSync("python3", ["-c", "import lab.b"], {cwd: directory, encoding: "utf8"});
      expect(runtime.status).not.toBe(0);
      expect(runtime.stderr).toContain("cannot import name 'big'");
      files["lab/a.py"] = "def big(x:\n";
      write();
      const syntax = await round();
      expect(syntax.units.find(row => row.id === "python:lab.a")!.status).toBe("failed");
      expect(spawnSync("python3", ["-m", "py_compile", "lab/a.py"], {cwd: directory, encoding: "utf8"}).status).not.toBe(0);
      expect(syntax.units.find(row => row.id === "python:lab.b")!.status).toBe("blocked");
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 120000);
});

describe("typescript and rust wrappers", () => {
  test("a TypeScript unit plan maps onto a valid neutral graph without re-deriving anything", () => {
    const plan = fixture.typescript.plan as any, graph = typeScriptPlanToUnitsV1(plan, {compiler: "tsc 5.9.3", options: "o", environment: "e"});
    readUnitGraphV1(graph, policy);
    const byId = new Map(graph.units.map(unit => [unit.id, unit]));
    expect(byId.get("typescript:dir:a")!.sources).toEqual(["a/i.ts"]);
    expect(byId.get("typescript:dir:a")!.inputs).toEqual(["package.json", "a/c.json", "🌿️ambient/g.d.ts"].sort(compareCodePointsV1));
    expect(byId.get("typescript:dir:t")!.interface).toBeNull();
    expect(byId.get("typescript:dir:b")!.dependencies).toEqual([{unit: "typescript:dir:a", use: "interface"}]);
    expect(byId.get("typescript:dir:t")!.dependencies.map(dependency => dependency.unit)).toEqual(["typescript:dir:a", "typescript:dir:b"]);
    const outcome = typeScriptOutcomeV1({errors: 0, boundary: 0, diagnostics: "", declarations: sha256V1("tree"), milliseconds: 5}, {leaf: false});
    expect(outcome.interface).toBe(sha256V1("tree"));
    expect(typeScriptOutcomeV1({errors: 2, boundary: 0, diagnostics: "x", declarations: sha256V1("tree"), milliseconds: 5}, {leaf: true})).toMatchObject({ok: false, interface: null});
  });
  test("the neutral engine keys a transitive dependent on the whole interface closure (the TypeScript engine keys direct declaration digests only)", async () => {
    const make = (id: string, dependencies: string[]): UnitV1 => ({id: "typescript:" + id, language: "typescript", kind: "library", label: id, sources: [id + ".ts"], inputs: [], dependencies: dependencies.map(unit => ({unit: "typescript:" + unit, use: "interface" as const})), externals: [], interface: {kind: "typescript-declarations"}, outputs: [], toolchain: {id: "tsc", parts: []}, action: {argv: ["x"], env: {}}});
    const graph: UnitGraphV1 = {version: 1, units: [make("a", []), make("b", ["a"]), make("c", ["b"])]};
    let revision = 0;
    const host: UnitsHostV1 = {digest: path => sha256V1(path + (path === "a.ts" ? revision : 0)), execute: async request => ({ok: true, diagnostics: "", outputs: sha256V1("o" + request.unit.id + (request.unit.id === "typescript:a" ? revision : 0)), interface: sha256V1(request.unit.id === "typescript:a" ? "a-types-" + revision : "stable-" + request.unit.id)})};
    const store = memoryUnitsStoreV1(), planned = planUnitsV1(readUnitGraphV1(graph, policy));
    await runUnitsV1(planned, policy, host, store, {concurrency: 1, signal: noSignal});
    revision = 1;
    const result = await runUnitsV1(planned, policy, host, store, {concurrency: 1, signal: noSignal});
    expect(result.units.map(unit => [unit.id, unit.status])).toEqual([["typescript:a", "ran"], ["typescript:b", "ran"], ["typescript:c", "ran"]]);
  });
  const metadataPath = join(import.meta.dir, "🧫️fixtures", "cargo-metadata.json");
  test("a cargo metadata document maps onto the neutral graph, and cargo tree agrees on the normal edges (third-party oracle)", () => {
    if (!existsSync(metadataPath)) return;
    const metadata = JSON.parse(readFileSync(metadataPath, "utf8")) as Parameters<typeof rustUnitsFromMetadataV1>[0] & {_files: string[]; _tree: string[]};
    const graph = rustUnitsFromMetadataV1(metadata, {policy, files: metadata._files, toolchain: "rustc test"});
    readUnitGraphV1(graph, policy);
    const edges = new Set<string>();
    for (const unit of graph.units) if (unit.kind !== "test") for (const dependency of unit.dependencies) {const from = unit.id.slice(5, unit.id.indexOf("@")), to = dependency.unit.slice(5, dependency.unit.indexOf("@")); if (from !== to) edges.add(`${from}->${to}`);}
    expect([...edges].sort()).toEqual([...metadata._tree].sort());
    const macro = graph.units.find(unit => unit.id.startsWith("rust:app@") && unit.kind === "library");
    if (macro) expect(macro.dependencies.some(dependency => dependency.use === "implementation")).toBe(metadata.packages.some(item => item.name === "mac"));
    expect(graph.units.some(unit => unit.kind === "binary")).toBe(true);
    expect(graph.units.every(unit => unit.id.startsWith("rust:"))).toBe(true);
  });
  test("a unit-cache entry maps onto the neutral outcome: the rmeta output is the interface", () => {
    const entry = {outputs: [{name: "libx-abc.rlib", sha256: sha256V1("rlib")}, {name: "libx-abc.rmeta", sha256: sha256V1("rmeta")}]};
    expect(rustEntryOutcomeV1(entry, {interface: {kind: "rust-rmeta"}}).interface).toBe(sha256V1("rmeta"));
    expect(rustEntryOutcomeV1(entry, {interface: null}).interface).toBeNull();
    expect(() => rustEntryOutcomeV1({outputs: [entry.outputs[0]!]}, {interface: {kind: "rust-rmeta"}})).toThrow();
  });
});
