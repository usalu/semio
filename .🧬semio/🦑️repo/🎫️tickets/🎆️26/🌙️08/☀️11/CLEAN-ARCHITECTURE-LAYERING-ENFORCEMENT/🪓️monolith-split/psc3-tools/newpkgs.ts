/** 🆕 Derives the TypeScript sibling packages the Rust-only owners need. Prints plan arguments, one per line (`--owner S` and `--ts-package S=dir:name`). Usage: bun newpkgs.ts <policy.json> <scan.json> [--min <edges>] [--exclude <sourceOwner>]... */
import { readFileSync } from "node:fs";
import { loadPolicy, groupsOf, nfc } from "./model.ts";
import { edgeSkip } from "./skip.ts";
const args = process.argv.slice(2);
const [policyPath, scanPath] = args;
const min = Number(args[args.indexOf("--min") + 1] || 1);
const exclude = args.flatMap((a, i) => (a === "--exclude" ? [nfc(args[i + 1])] : []));
const policy = loadPolicy(policyPath);
const groups = groupsOf(policy);
const names = new Set(policy.packages.map(p => p.name));
const count = new Map<string, number>();
for (const e of JSON.parse(readFileSync(scanPath, "utf8")) as any[]) if (!edgeSkip(e)) count.set(e.sourceOwner, (count.get(e.sourceOwner) ?? 0) + 1);
const out: string[] = [];
for (const g of groups) {
  if (g.tsPackage || !count.has(g.sourceOwner) || (count.get(g.sourceOwner) ?? 0) < min || exclude.includes(g.sourceOwner)) continue;
  const rust = g.packages.find(p => p.owner === g.sourceOwner + "/📦️packages/🦀️rust");
  if (!rust) continue;
  let base = rust.name.replace(/-rs$/u, "");
  if (base.startsWith("semio-")) base = base.slice("semio-".length);
  if (!base.startsWith("@")) base = "@semio-tech/" + base;
  const name = base + "-ts";
  if (names.has(name)) { console.error("NAME TAKEN", name); continue; }
  out.push(`--owner\n${g.sourceOwner}\n--ts-package\n${g.sourceOwner}=${g.sourceOwner}/📦️packages/🟦️typescript:${name}`);
  console.error(`${count.get(g.sourceOwner)}\t${g.sourceOwner}\t${name}`);
}
console.log(out.join("\n"));
