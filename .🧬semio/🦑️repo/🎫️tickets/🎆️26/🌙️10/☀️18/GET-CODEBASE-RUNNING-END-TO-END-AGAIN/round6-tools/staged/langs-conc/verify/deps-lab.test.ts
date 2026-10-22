// 🧪 Lab for the patched native dependency preparation: the issued-control checks are replaced by pass-through mocks, everything the patch changed (arguments, variables, SDK check) runs for real against a recording runner.
import {expect, mock, test} from "bun:test";
import {mkdtempSync, writeFileSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
const OV = process.env.LC_ROOT!;
const routing = OV + "/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const actual = await import(routing);
mock.module(routing, () => ({...actual, requireIssuedResourceControlInvocationV1: () => undefined}));
const {prepareDependencies} = await import(OV + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts");
const policy = (await import(OV + "/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🔣️.json", {with: {type: "json"}})).default;
const authority = {};
const phase = {leaseControl: {signal: new AbortController().signal}, authority: {original: authority}, advance: async () => undefined, budget: () => 1000};
const run = async (kind: string, files: Record<string, string> = {}, capture: Record<string, string> = {}) => {
  const root = mkdtempSync(join(tmpdir(), "deps-lab-")), calls: {command: string; args: string[]; env: Record<string, string | undefined>; cwd: string}[] = [];
  for (const [name, text] of Object.entries(files)) writeFileSync(join(root, name), text);
  try {
    await prepareDependencies(kind, root, {capabilities: authority}, {}, phase, async (command: string, args: string[], cwd: string, _signal: AbortSignal, captured: boolean, env: Record<string, string | undefined>) => {
      calls.push({command, args, env, cwd});
      return captured ? capture[command] ?? "" : "";
    }, {SEMIO_LANGUAGE: "en", PATH: "/usr/bin"}, async (base: Record<string, string>) => ({...base, ...policy.go.variables, GOROOT: "/pinned/go", PATH: "/pinned/go/bin:" + base.PATH}));
    return {root, calls};
  } finally { rmSync(root, {recursive: true, force: true}); }
};
test("python syncs the workspace project into its planned environment, never --all-packages", async () => {
  const {root, calls} = await run("python", {"pyproject.toml": "[project]\nname='x'\n", "uv.lock": "version = 1\n"});
  expect(calls.map(call => [call.command, ...call.args])).toEqual([["uv", "python", "install", "3.14"], ["uv", "sync", "--locked", "--project", ".", "--group", "dev", "--group", "test"]]);
  for (const call of calls) {
    expect(call.cwd).toBe(root);
    expect(call.env.UV_PROJECT_ENVIRONMENT).toContain("/.cache/b/d/m/python/environments/workspace-");
    expect(call.env.UV_PYTHON_INSTALL_DIR).toBe(join(root, ".cache/b/t/m/python/interpreters"));
    expect(call.env.UV_CACHE_DIR).toBe(join(root, ".cache/b/d/m/python/uv-cache"));
    expect(call.env).toMatchObject({UV_NO_CONFIG: "1", UV_PYTHON: "3.14", UV_PYTHON_DOWNLOADS: "manual", UV_PYTHON_PREFERENCE: "only-managed"});
    expect(call.env.UV_PROJECT_ENVIRONMENT!.startsWith(root)).toBe(true);
  }
  expect(calls.flatMap(call => call.args)).not.toContain("--all-packages");
});
test("python without a lock is refused instead of syncing an unlocked environment", async () => {
  await expect(run("python", {"pyproject.toml": "[project]\n"})).rejects.toThrow(/uv\.lock/u);
});
test("go downloads under the local pinned toolchain", async () => {
  const {root, calls} = await run("go");
  expect(calls.map(call => [call.command, ...call.args])).toEqual([["go", "mod", "download"]]);
  expect(calls[0]!.env).toMatchObject({GOTOOLCHAIN: "local", GOROOT: "/pinned/go", GOWORK: join(root, "go.work")});
  expect(calls[0]!.env.PATH!.startsWith("/pinned/go/bin:")).toBe(true);
});
test("dotnet accepts exactly the pinned SDK and refuses any other", async () => {
  const {calls} = await run("dotnet", {}, {dotnet: policy.dotnet.sdk + "\n"});
  expect(calls.map(call => [call.command, ...call.args])).toEqual([["dotnet", "--version"]]);
  await expect(run("dotnet", {}, {dotnet: "8.0.999"})).rejects.toThrow(/8\.0\.999 differs from the pin 8\.0\.421 \(global\.json\)/u);
  await expect(run("dotnet", {}, {dotnet: ""})).rejects.toThrow(/differs from the pin/u);
});
