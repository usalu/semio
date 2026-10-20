import { expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createScriptProcessEnvelope, Script, ScriptRouter, withScriptProcessEnvelope, type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { diskZeroTouchTreeV1, scanModuleSpecifiersV1 } from "../../../🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
import { readRootCommandHelpRequestV1, readRootCommandRegistryV1, registerRootCommandsV1, renderRootCommandHelpV1, rootCommandHelpV1, rootCommandRegistryV1, type RootCommandV1 } from "../🟦️.ts";
import { buildOwnedInvocationV1, dependencyGateV1, provisionEnvironmentV1, type DependencyProvisionPortsV1 } from "../🥾️provision/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import data from "../🔣️.json" with { type: "json" };
import fixture from "./🔣️.json" with { type: "json" };

const root = resolve(import.meta.dir, "../../../../../../../.."), registry = rootCommandRegistryV1();
type Patch = Readonly<{ op: "set" | "delete"; path: readonly (string | number)[]; value?: unknown }>;

const patched = (patches: readonly Patch[]): unknown => {
  const copy = structuredClone(data) as Record<string, any>;
  for (const patch of patches) {
    const parent = patch.path.slice(0, -1).reduce((value, key) => value[key], copy), key = patch.path.at(-1)!;
    if (patch.op === "delete") delete parent[key]; else parent[key] = patch.value;
  }
  return copy;
};

const exportedNames = (path: string, text: string): ReadonlySet<string> => {
  const source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true), names = new Set<string>();
  for (const statement of source.statements) {
    if (ts.isExportDeclaration(statement) && statement.exportClause && ts.isNamedExports(statement.exportClause)) for (const element of statement.exportClause.elements) names.add(element.name.text);
    else if (ts.canHaveModifiers(statement) && ts.getModifiers(statement)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) {
      if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) { if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text); }
      else if ((ts.isClassDeclaration(statement) || ts.isFunctionDeclaration(statement)) && statement.name) names.add(statement.name.text);
    }
  }
  return names;
};

test("the registry satisfies its schema for the independent validator and for our reader", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(data), JSON.stringify(validate.errors)).toBe(true);
  expect(readRootCommandRegistryV1(data).commands.length).toBe(fixture.expectedIds.length);
});

test("the registry declares exactly the expected commands and the expected requirements", () => {
  expect(registry.commands.map((command) => command.id)).toEqual(fixture.expectedIds);
  for (const requires of ["none", "dependencies"] as const) expect(registry.commands.filter((command) => command.requires === requires).map((command) => command.id)).toEqual(fixture.expectedRequires[requires]);
});

for (const row of fixture.invalidRegistries) {
  test(`an invalid registry is refused: ${row.name}`, () => {
    const value = patched(row.patch as readonly Patch[]);
    expect(() => readRootCommandRegistryV1(value)).toThrow();
    expect(new Ajv({ strict: true }).compile(schema)(value), "independent validator").toBe(!row.schema);
  });
}

test("every command module exists and exports its command class, as the system scan and the TypeScript compiler agree", () => {
  for (const command of registry.commands) {
    const path = join(root, command.module), text = readFileSync(path, "utf8");
    expect(existsSync(path), command.module).toBe(true);
    const scanned = new Bun.Transpiler({ loader: "ts" }).scan(text).exports;
    expect(scanned, `${command.id} scan`).toContain(command.export);
    expect(exportedNames(command.module, text).has(command.export), `${command.id} compiler`).toBe(true);
  }
});

test("the root script loads every registry module lazily and none eagerly", () => {
  const tree = diskZeroTouchTreeV1(root), entry = tree.read("📜️script.ts")!, rows = scanModuleSpecifiersV1("📜️script.ts", entry);
  const dynamic = new Set(rows.filter((row) => row.kind === "dynamic").map((row) => row.specifier.replace(/^\.\//u, "")));
  for (const command of registry.commands) expect(dynamic.has(command.module), `${command.id} dynamic`).toBe(true);
  const eager = new Set(rows.filter((row) => row.kind === "static").map((row) => row.specifier.replace(/^\.\//u, "")));
  for (const command of registry.commands) expect(eager.has(command.module), `${command.id} static`).toBe(false);
});

for (const row of fixture.helpRequests) {
  test(`help request ${JSON.stringify(row.argv)}`, () => {
    expect(readRootCommandHelpRequestV1(row.argv) ?? null).toEqual(row.request);
  });
}

for (const row of fixture.languages) {
  test(`help language ${JSON.stringify(row.environment)} ${JSON.stringify(row.argv)}`, () => {
    const help = rootCommandHelpV1(registry, [...row.argv, "--help"], row.environment)!;
    expect(help.split("\n")[0]).toBe(`bun ./📜️script.ts - ${registry.labels.tagline![row.language]}`);
  });
}

test("help renders every command, its requirement and its summary in every declared language", () => {
  for (const language of registry.languages) {
    const top = renderRootCommandHelpV1(registry, undefined, language);
    expect(top.split("\n").at(-1)).toBe(registry.labels.legend![language]!);
    for (const command of registry.commands) {
      expect(top.includes(command.summary[language]!), `${language} ${command.id} summary`).toBe(true);
      const own = renderRootCommandHelpV1(registry, command.id, language);
      expect(own.split("\n")[0]).toBe(`${command.id} - ${command.summary[language]}`);
      expect(own.includes(registry.labels[command.requires === "none" ? "requiresNone" : "requiresDependencies"]![language]!), `${language} ${command.id} requirement`).toBe(true);
      expect(top.split("\n").some((line) => line.startsWith(`  ${command.id}${command.requires === "dependencies" ? " *" : " "}`)), `${language} ${command.id} marker`).toBe(true);
    }
  }
  expect(() => renderRootCommandHelpV1(registry, "nonexistent", "en")).toThrow("Unknown command: nonexistent");
});

const run = async <T>(capabilities: object, body: (invocation: ScriptInvocation) => Promise<T>): Promise<T> => withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "root-command-gate-law", maximumElapsedMilliseconds: 30_000 }, capabilities, Date.now()), body);

test("the router runs the gate before it loads a command and the loaders must match the registry", async () => {
  const order: string[] = [], small = readRootCommandRegistryV1(patched([{ op: "set", path: ["commands"], value: data.commands.slice(0, 2) }]));
  class First extends Script { run(): void { order.push("run"); } }
  const loaders = { repo: async () => { order.push("load repo"); return First; }, os: async () => First };
  const router = registerRootCommandsV1(new ScriptRouter(root, root), small, loaders, async (command) => { order.push(`gate ${command.id}`); });
  await run({}, async (invocation) => { await router.run(["repo"], invocation); });
  expect(order).toEqual(["gate repo", "load repo", "run"]);
  expect(() => registerRootCommandsV1(new ScriptRouter(root, root), small, { repo: loaders.repo }, async () => {})).toThrow("missing: os");
  expect(() => registerRootCommandsV1(new ScriptRouter(root, root), small, { ...loaders, extra: loaders.os }, async () => {})).toThrow("unknown: extra");
});

test("the provisioning environment is a fresh build CLI birth and keeps everything else", () => {
  const environment = provisionEnvironmentV1({ PATH: "/bin", SEMIO_SCRIPT_PROCESS_INVOCATION: "{}", SEMIO_SCRIPT_POLICY: "{}", SEMIO_SCRIPT_CAPABILITIES: "{}", SEMIO_LANGUAGE: "de" });
  expect(environment).toEqual({ PATH: "/bin", SEMIO_LANGUAGE: "de" });
});

test("only an invocation issued by the build CLI for this workspace counts as provisioned", async () => {
  await run({}, async (invocation) => expect(buildOwnedInvocationV1(root, invocation)).toBe(false));
  await run({ workspaceRoot: root }, async (invocation) => expect(buildOwnedInvocationV1(root, invocation)).toBe(true));
  await run({ repositoryRoot: root }, async (invocation) => expect(buildOwnedInvocationV1(root, invocation)).toBe(true));
  await run({ workspaceRoot: join(root, "elsewhere") }, async (invocation) => expect(buildOwnedInvocationV1(root, invocation)).toBe(false));
});

for (const row of fixture.gate) {
  test(`dependency gate: ${row.name}`, async () => {
    const calls: { command: string; args: readonly string[]; cwd: string; label: string; environment: Readonly<Record<string, string | undefined>> }[] = [], lines: string[] = [];
    let clock = 0;
    const ports: DependencyProvisionPortsV1 = {
      run: async (command, args, cwd, label, environment) => { calls.push({ command, args, cwd, label, environment }); clock += 2500; if ("fail" in row && row.fail) throw new Error("boom"); },
      write: (line) => { lines.push(line); },
      now: () => clock,
    };
    const environment = { PATH: "/bin", SEMIO_SCRIPT_PROCESS_INVOCATION: "{}", ...("environment" in row ? row.environment : {}) };
    const command = { ...registry.commands.find((candidate) => candidate.id === "verify")!, requires: row.requires } as RootCommandV1;
    const capabilities = JSON.parse(JSON.stringify(row.capabilities).replaceAll("@root", root.replaceAll("\\", "\\\\")));
    const gate = dependencyGateV1(root, registry, environment, ports);
    const attempt = run(capabilities, (invocation) => gate(command, invocation));
    if ("fail" in row && row.fail) {
      const error = await attempt.then(() => undefined, (failure: Error) => failure);
      for (const part of fixture.gateFailure) expect(error?.message.includes(part), part).toBe(true);
    } else await attempt;
    expect(calls.length).toBe(row.provisions ? 1 : 0);
    if (!row.provisions) return expect(lines).toEqual([]);
    const owner = registry.commands.find((candidate) => candidate.id === "repo")!;
    expect(calls[0]!.command).toBe(process.execPath);
    expect(calls[0]!.args).toEqual([join(root, owner.module), "repo", "dependencies", "ensure"]);
    expect(calls[0]!.cwd).toBe(root);
    expect(calls[0]!.label).toBe("dependencies");
    expect(Object.keys(calls[0]!.environment)).toEqual(["PATH", ...("environment" in row ? Object.keys(row.environment!) : [])]);
    const language = "environment" in row ? "de" : "en", progress = fixture.gateProgress[language as "en" | "de"];
    expect(lines[0]!.includes(progress[0]!), lines[0]).toBe(true);
    if (!("fail" in row && row.fail)) { expect(lines[1]!.includes(progress[1]!), lines[1]).toBe(true); expect(lines[1]!.includes("2.5")).toBe(true); }
  });
}

test("an aborted invocation stops before anything is provisioned", async () => {
  const controller = new AbortController(), calls: string[] = [], ports: DependencyProvisionPortsV1 = { run: async () => { calls.push("run"); }, write: () => {}, now: () => 0 };
  const command = registry.commands.find((candidate) => candidate.id === "verify")!, gate = dependencyGateV1(root, registry, {}, ports);
  const error = await run({}, async (invocation) => { controller.abort(); return gate(command, { ...invocation, control: { ...invocation.control, signal: controller.signal } }); }).then(() => undefined, (failure: Error) => failure);
  expect(error).toBeDefined();
  expect(calls).toEqual([]);
});

test("the registry data points at its schema and every reason is written in every declared language", () => {
  expect(join(import.meta.dir, "..", data.$schema)).toBe(join(import.meta.dir, "..", "🧬️schema/🔣️.json"));
  expect(JSON.parse(readFileSync(join(import.meta.dir, "..", data.$schema), "utf8"))).toEqual(schema);
  for (const command of registry.commands) for (const language of registry.languages) expect(command.reason[language]?.length, `${command.id} ${language}`).toBeGreaterThan(8);
});
