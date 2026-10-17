// usage: bun exthost.ts <label> <entry-relative-to-package-root> <prototype|release>
// Bundles the extension-host test entry exactly as the vscode builder configures it (vite, cjs, vscode and node external), loads the bundle with the mocha globals
// replaced by a collector and a stub `vscode`, and runs every collected test with a 2 s guard. Writes G/c4ts-ext-<label>.json: { tree, outcomes }.
import { build } from "vite";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
const [label, entry, mode] = process.argv.slice(2) as [string, string, "prototype" | "release"];
const R = "/Users/ueli/Documents/semio";
const V = `${R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode`;
const packageRoot = `${V}/📦️packages/🟦️typescript`;
const G = process.env.G!;
const { extensionBuildConfig } = await import(`${V}/🏗️builder/🟦️.ts`);
const outDir = `${G}/c4ts-ext-${label}`;
mkdirSync(outDir, { recursive: true });
const config = extensionBuildConfig(packageRoot, entry, outDir, "extension.test.js", mode);
if (process.env.EXT_TREESHAKE === "1") { config.build.minify = false as never; config.build.rollupOptions = { ...config.build.rollupOptions, treeshake: true } as never; }
await build({ ...config, logLevel: "silent" });
const code = readFileSync(`${outDir}/extension.test.js`, "utf8");

type Node = { title: string; children: Node[]; tests: string[] };
const root: Node = { title: "", children: [], tests: [] };
const stack: Node[] = [root];
const registered: Array<{ path: string[]; title: string; fn: (this: unknown) => unknown }> = [];
let hooks = 0;
const ctx = { timeout() { return ctx; }, slow() { return ctx; }, retries() { return ctx; } };
const suite = (title: string, body: (this: unknown) => void) => { const node: Node = { title, children: [], tests: [] }; stack[stack.length - 1]!.children.push(node); stack.push(node); try { body.call(ctx); } finally { stack.pop(); } };
const test = (title: string, fn: (this: unknown) => unknown) => { stack[stack.length - 1]!.tests.push(title); registered.push({ path: stack.slice(1).map((n) => n.title), title, fn }); };
const hook = () => { hooks += 1; };
const stub = (): unknown => new Proxy(function () {}, {
  get: (_t, key) => (key === "then" || key === Symbol.toPrimitive || key === "toJSON" ? undefined : key === Symbol.iterator ? undefined : stub()),
  apply: () => stub(),
  construct: () => stub() as object,
  has: () => true,
});
const nodeRequire = createRequire(import.meta.url);
const vscodeNames = new Set([...code.matchAll(/vscode\w*\.([A-Za-z_]\w*)/g)].map((m) => m[1]!));
const vscodeStub = Object.fromEntries([...vscodeNames].map((name) => [name, stub()]));
const fakeRequire = (id: string) => (id === "vscode" ? vscodeStub : nodeRequire(id));
const moduleBox: { exports: Record<string, unknown> } = { exports: {} };
new Function("require", "module", "exports", "suite", "test", "suiteSetup", "setup", "teardown", "suiteTeardown", "__dirname", "__filename", code)(fakeRequire, moduleBox, moduleBox.exports, suite, test, hook, hook, hook, hook, packageRoot, `${packageRoot}/extension.test.js`);

const outcomes: Record<string, string> = {};
for (const row of registered) {
  const key = [...row.path, row.title].join(" > ");
  const guard = new Promise<string>((done) => setTimeout(() => done("TIMEOUT"), 2000));
  const run = (async () => { await row.fn.call(ctx); return "PASS"; })().catch((e: unknown) => `FAIL ${String((e as Error)?.message ?? e).replace(/\s+/g, " ").slice(0, 100)}`);
  outcomes[key] = await Promise.race([run, guard]);
}
writeFileSync(`${G}/c4ts-ext-${label}.json`, JSON.stringify({ tree: root, hooks, outcomes }, null, 1));
const counts: Record<string, number> = {};
for (const v of Object.values(outcomes)) counts[v.split(" ")[0]!] = (counts[v.split(" ")[0]!] ?? 0) + 1;
console.log(`${label}: suites=${JSON.stringify(root).split('"children"').length - 1} tests=${registered.length} hooks=${hooks}`, JSON.stringify(counts));
