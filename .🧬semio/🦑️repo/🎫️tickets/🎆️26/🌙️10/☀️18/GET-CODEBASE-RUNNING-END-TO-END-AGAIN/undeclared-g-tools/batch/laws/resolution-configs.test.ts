/** 🧰️ Contract: the command closure of a script names the module resolution configuration bun reads for the modules it loads (the nearest `package.json` and `tsconfig.json` above each module and the `tsconfig.json` files an `extends` names) and nothing else. The independent oracle is the macOS kernel: bun runs the fixture script under the Seatbelt read oracle; with only the modules declared it reads exactly the files the rule names, with the rule's files declared it reads nothing undeclared.
 * run: `bun test ./<this file>` (UNDECLARED_G_MODULE=<absolute path> tests a staged copy of the module instead of `../🟨️.mjs`) */
import {afterAll, beforeAll, expect, test} from "bun:test";
import {mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join, relative} from "node:path";

const LIBRARY = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const module_ = await import(process.env.UNDECLARED_G_MODULE ?? "../🟨️.mjs");
const {parserTypescriptV1} = await import(`${LIBRARY}/🏗️build/🚀️bootstrap/📦️parsers/🟨️.mjs`);
const {runReadOracleV1} = await import(`${LIBRARY}/🏗️build/📥️inputs/🔭️read-oracle/🟦️.ts`);

const tree: Record<string, string> = {
  "package.json": "{}",
  "tsconfig.json": "{}",
  "app/package.json": '{"name":"app","type":"module"}',
  "app/script.ts": 'import {deep} from "../lib/deep/mod.ts";\nimport {leaf} from "../plain/leaf.ts";\nconsole.log(deep + leaf);\n',
  "lib/package.json": '{"name":"lib","type":"module"}',
  "lib/tsconfig.json": '{"extends":"../base/tsconfig.base.json"}',
  "lib/deep/mod.ts": 'export const deep = "d";\n',
  "base/tsconfig.base.json": '{"extends":"./tsconfig.root.json","compilerOptions":{"strict":true}}',
  "base/tsconfig.root.json": '{"compilerOptions":{"target":"esnext"}}',
  "plain/leaf.ts": 'export const leaf = "l";\n',
  "unrelated/package.json": "{}",
  "unrelated/tsconfig.json": "{}",
};
const modules = ["app/script.ts", "lib/deep/mod.ts", "plain/leaf.ts"], shared = ["package.json", "tsconfig.json"];
const expected = ["app/package.json", "base/tsconfig.base.json", "base/tsconfig.root.json", "lib/package.json", "lib/tsconfig.json"];
let workspace = "";
beforeAll(() => {
  workspace = realpathSync(mkdtempSync(join(tmpdir(), "resolution-configs-")));
  for (const [path, text] of Object.entries(tree)) { mkdirSync(dirname(join(workspace, path)), {recursive: true}); writeFileSync(join(workspace, path), text); }
});
afterAll(() => { rmSync(workspace, {recursive: true, force: true}); });

test("the rule names the nearest package.json and tsconfig.json of every module, follows extends, stops below the workspace root", () => {
  const compiler = parserTypescriptV1();
  const found = module_.resolutionConfigsV1(compiler, compiler.sys, workspace, new Set(modules.map((path) => join(workspace, path)))).map((path: string) => relative(workspace, path));
  expect(found).toEqual(expected);
});

test.skipIf(process.platform !== "darwin")("the kernel agrees: bun reads exactly those files besides the modules and the shared root files, and nothing undeclared once they are declared", async () => {
  const request = (allowed: string[]) => ({version: 1, language: "en", root: workspace, cwd: "app", allowed, ambient: [], names: [], command: [process.execPath, "script.ts"], environment: {}, rounds: 8, timeoutMs: 60000, backend: "seatbelt", compare: "exit"});
  const bare = await runReadOracleV1(request([...modules, ...shared]));
  expect(bare.learned.slice().sort()).toEqual(expected);
  const declared = await runReadOracleV1(request([...modules, ...shared, ...expected]));
  expect(declared.learned).toEqual([]);
  expect(declared.passed).toBe(true);
}, 120000);
