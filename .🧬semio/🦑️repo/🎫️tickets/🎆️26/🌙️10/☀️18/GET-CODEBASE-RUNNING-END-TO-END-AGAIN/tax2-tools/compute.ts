// 🧮 Converts ComputeOwnershipTestScript subclasses into BundleScript delegations. usage: bun compute.ts <list.txt> [--write]
import {readFileSync} from "node:fs";
import {guardedWrite, ledger, read, accepts, syntaxOk} from "./lib.ts";
const write = process.argv.includes("--write");
const tally: Record<string, number> = {};
const count = (k: string) => (tally[k] = (tally[k] ?? 0) + 1);
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const src = read(p);
  if (!src.includes("ComputeOwnershipTestScript")) continue;
  const cls = /class (\w+) extends ComputeOwnershipTestScript\{readonly source=("[^"]+");\}\n/u.exec(src);
  const routerImport = /import \{ ?ScriptRouter ?\} from "([^"]+)";\n/u.exec(src);
  if (!cls || !routerImport) { count("unmatched"); ledger({tool: "compute", p, result: "unmatched"}); continue; }
  let out = src.replace(cls[0], `class ${cls[1]} extends BundleScript{async run(segments:string[]):Promise<void>{await runComputeOwnershipTest({root:this.root,repoRoot:this.repoRoot},${cls[2]},segments);}}\n`);
  out = out.replace("{ComputeOwnershipTestScript}", "{runComputeOwnershipTest}").replace(routerImport[0], `import {BundleScript,ScriptRouter} from "${routerImport[1]}";\n`);
  if (!accepts(out) || !syntaxOk(p, out)) { count("invalid"); ledger({tool: "compute", p, result: "invalid"}); continue; }
  if (!write) { count("ok"); continue; }
  const r = guardedWrite(p, src, out); count(r); ledger({tool: "compute", p, result: r});
}
console.log(tally);
