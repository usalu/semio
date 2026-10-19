// 🌉 Converts mutation-bridge scripts into the canonical router shape. usage: bun bridge.ts <list.txt> [--write]
import {readFileSync, writeFileSync} from "node:fs";
import {accepts, guardedWrite, ledger, read, rel, ROUTING, syntaxOk} from "./lib.ts";
const write = process.argv.includes("--write"); const dump = process.argv.includes("--dump");
const paths = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const tally: Record<string, number> = {};
const count = (k: string) => (tally[k] = (tally[k] ?? 0) + 1);
for (const p of paths) {
  const src = read(p);
  const routing = /import\s*\{[^}]*receiveScriptProcessInvocation[^}]*\}\s*from\s*"([^"]+)"/u.exec(src)?.[1] ?? rel(p, ROUTING);
  const imports = `import { BundleScript, ScriptRouter, runScriptMain } from "${routing}";\n`;
  const bridgeImport = /^import \{ runRepositoryMutationBridgeScriptV1 \} from "[^"]+";\n/mu.exec(src);
  let out: string | null = null, form = "";
  const simple = /const BRIDGE_VERSION=1;\nif\(import\.meta\.main\)await runRepositoryMutationBridgeScriptV1\(import\.meta\.dir,("[^"]+")\);\nexport \{BRIDGE_VERSION\};\n/u.exec(src);
  if (simple && bridgeImport && !src.includes("receiveScriptProcessInvocation")) {
    form = "simple";
    const head = src.slice(0, bridgeImport.index + bridgeImport[0].length) + `import { receiveScriptProcessInvocation } from "${routing}";\n` + imports;
    const mid = src.slice(bridgeImport.index + bridgeImport[0].length, simple.index);
    out = head + mid + `/** 🚪️ \`list-mutations\` — delivers the production mutation inventory of this owner through its bridge binary. */\nclass ListMutationsScript extends BundleScript {\n  async run(): Promise<void> {\n    await runRepositoryMutationBridgeScriptV1(this.root, ${simple[1]});\n  }\n}\nconst router = new ScriptRouter(import.meta.dir).register("list-mutations", ListMutationsScript);\nif (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));\n` + src.slice(simple.index + simple[0].length);
  }
  const check = /if\(process\.argv\[2\]==="check"\)\{\n if\(process\.argv\.length!==3\)throw Error\("check accepts no overrides"\);\n await receiveScriptProcessInvocation\(process\.env,original=>runAdmittedCargoCommandV1\(original,createIssuedResourceControlV1\(original,"cargo:bridge",0\),findWorkspaceRoot\(\),(\[[^\n]*?\]),import\.meta\.dir,process\.env,collector\.maximumMilliseconds\)\);\n\}else await runRepositoryMutationBridgeScriptV1\(packageRoot,binary,\{\.\.\.collector,version:1\}\);\n/u.exec(src);
  if (!out && check) {
    form = "check";
    const args = check[1]!;
    const after = src.slice(check.index + check[0].length);
    const cls = `/** 🚪️ \`list-mutations\` — delivers the production mutation inventory of this owner through its bridge binary. */\nclass ListMutationsScript extends BundleScript {\n  async run(): Promise<void> {\n    await runRepositoryMutationBridgeScriptV1(packageRoot, binary, { ...collector, version: 1 });\n  }\n}\n/** 🩺️ \`check\` — runs the bridge binary's own tests under the collector budget. */\nclass CheckScript extends BundleScript {\n  async run(segments: string[]): Promise<void> {\n    if (segments.length) throw new Error("check accepts no overrides");\n    await runAdmittedCargoCommandV1(this.invocation, createIssuedResourceControlV1(this.invocation, "cargo:bridge", 0), findWorkspaceRoot(), ${args}, this.root, process.env, collector.maximumMilliseconds);\n  }\n}\nconst router = new ScriptRouter(import.meta.dir).register("list-mutations", ListMutationsScript).register("check", CheckScript);\nif (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));\n`;
    const importAt = src.indexOf("\nimport collector");
    const lineEnd = src.indexOf("\n", importAt + 1);
    out = src.slice(0, lineEnd + 1) + imports + src.slice(lineEnd + 1, check.index) + cls + after;
  }
  if (!out) { count("unmatched"); ledger({tool: "bridge", p, result: "unmatched"}); continue; }
  if (dump) writeFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tax2/dump-" + form + ".ts", out);
  const ok = accepts(out) && syntaxOk(p, out);
  if (!ok) { count("invalid-" + form); ledger({tool: "bridge", p, form, result: "invalid"}); continue; }
  if (!write) { count("ok-" + form); continue; }
  const r = guardedWrite(p, src, out); count(r + "-" + form); ledger({tool: "bridge", p, form, result: r});
}
console.log(tally);
