// 🧪 Lab for the patched energy oracle: the process runners are replaced by recorders; the plan, the arguments and the variables the oracle hands to uv run for real.
import {expect, mock, test} from "bun:test";
import {join} from "node:path";
const OV = process.env.LC_ROOT!;
const P = OV + "/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript";
const routing = await import(P + "/🧭️routing/🟦️.ts");
mock.module(P + "/🧭️routing/🟦️.ts", () => ({...routing, checkScriptInvocation: () => undefined}));
const calls: {command: string; args: string[]; options: any}[] = [];
const execution = await import(P + "/🎛️execution/🟦️.ts");
mock.module(P + "/🎛️execution/🟦️.ts", () => ({...execution, runCmd: (command: string, args: string[], options: any) => { calls.push({command, args, options}); }, getRepoMetaDir: (root: string) => join(root, ".meta")}));
const testExecution = await import(P + "/🧪️testing/🎛️execution/🟦️.ts");
mock.module(P + "/🧪️testing/🎛️execution/🟦️.ts", () => ({...testExecution, runRepositoryTestCommand: async (command: string, args: string[], options: any) => { calls.push({command, args, options}); }}));
const oracle = await import(OV + "/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts");
const packageRoot = join(OV, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python");
test("deps installs the managed 3.12 interpreter and syncs the planned environment of the oracle project only", async () => {
  calls.length = 0;
  await new oracle.DepsScript(packageRoot, OV, {} as any).run();
  expect(calls.map(call => [call.command, ...call.args])).toEqual([["uv", "python", "install", "3.12"], ["uv", "sync", "--locked", "--project", "✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python", "--group", "test"]]);
  for (const call of calls) {
    expect(call.options.cwd).toBe(OV);
    expect(call.options.env.UV_PROJECT_ENVIRONMENT).toContain(join(OV, ".cache/b/d/m/python/environments/energy-oracle-"));
    expect(call.options.env).toMatchObject({UV_PYTHON: "3.12", UV_PYTHON_PREFERENCE: "only-managed", UV_PYTHON_DOWNLOADS: "manual", UV_NO_CONFIG: "1", UV_PYTHON_INSTALL_DIR: join(OV, ".cache/b/t/m/python/interpreters")});
  }
});
test("run uses the same environment through uv run --no-sync and never a .venv", async () => {
  calls.length = 0;
  await oracle.runOraclePython(OV, packageRoot, ["status"]);
  expect(calls).toHaveLength(1);
  const {command, args, options} = calls[0]!;
  expect([command, ...args.slice(0, 5)]).toEqual(["uv", "run", "--locked", "--no-sync", "--project", packageRoot]);
  expect(options.cwd).toBe(packageRoot);
  expect(options.env.UV_PROJECT_ENVIRONMENT).toContain(join(OV, ".cache/b/d/m/python/environments/energy-oracle-"));
  expect(options.env.UV_PYTHON).toBe("3.12");
  expect(options.env.PYTHONDONTWRITEBYTECODE).toBe("1");
  expect(JSON.stringify(options.env)).not.toContain(".venv");
});
