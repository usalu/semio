import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import { build } from "esbuild";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

const root = resolve(import.meta.dir, "../.."), entry = resolve(root, fixture.entry), registry = JSON.parse(readFileSync(resolve(root, fixture.registry), "utf8")) as { commands: { id: string; module: string; summary: Record<string, string>; requires: string }[] };

const checkHelp = (stdout: string): void => {
  const lines = stdout.trim().split("\n");
  expect(lines[0]!.startsWith(fixture.helpFirstLinePrefix)).toBe(true);
  expect(lines.at(-1)!.startsWith(fixture.helpLegendPrefix)).toBe(true);
  for (const id of fixture.commands) expect(lines.some((line) => line.startsWith(`  ${id}`)), id).toBe(true);
};

test("workspace help starts without loading unrelated command or test owners", () => {
  for (const argument of fixture.helpArguments) {
    const result = spawnSync(process.execPath, [entry, ...argument], { cwd: root, encoding: "utf8", timeout: 15_000 });
    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);
    checkHelp(result.stdout);
  }
});

test("command help names the command and what it needs without loading it", () => {
  for (const [id, ...argument] of fixture.commandHelpArguments) {
    const result = spawnSync(process.execPath, [entry, id!, ...argument], { cwd: root, encoding: "utf8", timeout: 15_000 });
    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);
    const row = registry.commands.find((command) => command.id === id)!, language = argument.includes("--lang=de") ? "de" : "en";
    expect(result.stdout.split("\n")[0]).toBe(`${id} - ${row.summary[language]}`);
  }
});

test("root repo route creates the same original build caller as the dedicated entry", () => {
  for (const args of fixture.backendHelpArguments) {
    const result = spawnSync(process.execPath, [entry, ...args], { cwd: root, encoding: "utf8", timeout: 15_000 });
    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);
    expect(result.stdout).toContain("repo [--tool=nx|bazel] <command>");
  }
}, 40_000);

test("developer command registration loads without its unrelated application gateway", () => {
  for (const module of fixture.isolatedCommandModules) {
    const source = `await import(${JSON.stringify(pathToFileURL(resolve(root, module)).href)})`;
    const result = spawnSync(process.execPath, ["-e", source], { cwd: root, encoding: "utf8", timeout: 15_000 });
    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);
  }
}, 30_000);

test("independent bundler confirms eager startup uses only platform, routing and registry owners", async () => {
  const compiled = await build({
    absWorkingDir: root, entryPoints: [fixture.entry], bundle: true, write: false, platform: "node", format: "esm", logLevel: "silent",
    plugins: [{ name: "command-boundary", setup(bundler) {
      bundler.onResolve({ filter: /.*/ }, args => {
        if (args.kind === "dynamic-import") return { path: args.path, external: true };
        if (args.importer === entry && !fixture.eagerOwners.some(prefix => args.path.startsWith(prefix))) throw Error(`Eager command dependency: ${args.path}`);
      });
    } }],
  });
  const source = readFileSync(entry, "utf8");
  expect(registry.commands.map((command) => command.id)).toEqual(fixture.commands);
  for (const command of registry.commands) expect(source.includes(`import("./${command.module}")`), command.id).toBe(true);
  const result = spawnSync(process.execPath, ["-", "--help", "--lang=en"], { cwd: root, encoding: "utf8", input: compiled.outputFiles[0]!.text, timeout: 15_000 });
  expect(result.status, result.stderr).toBe(0);
  checkHelp(result.stdout);
}, 30_000);
