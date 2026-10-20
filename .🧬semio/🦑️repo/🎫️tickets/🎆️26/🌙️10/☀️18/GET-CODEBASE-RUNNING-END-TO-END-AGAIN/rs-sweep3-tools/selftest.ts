import { existsSync, mkdirSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { initParser, findWasm } from "./parse.ts";
import { buildTree } from "./tree.ts";
import { Finding, loadPackage } from "./cargo.ts";
import { setTreeBuilder } from "./resolve.ts";
import { ROOT, checkManifest, checkSource, libIndex, Stats } from "./checks.ts";

const FX = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rs-sweep-tools/fx`;
const OUT = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rs-sweep-fx`;
mkdirSync(OUT, { recursive: true });
await initParser(findWasm(ROOT)!);

const EXPECT: Record<string, { rustc: string[]; kinds: string[] }> = {
  ok_controls: { rustc: [], kinds: [] },
  ok_include_mod: { rustc: [], kinds: [] },
  e0583_include_mod_wrong_dir: { rustc: ["E0583"], kinds: ["mod-missing"] },
  e0432_unresolved_import: { rustc: ["E0432"], kinds: ["unresolved-import"] },
  e0432_super_path: { rustc: ["E0432"], kinds: ["unresolved-import"] },
  e0603_private_module: { rustc: ["E0603"], kinds: ["private-item"] },
  e0603_private_fn: { rustc: ["E0603"], kinds: ["private-item"] },
  e0603_crate_visible: { rustc: ["E0603"], kinds: ["private-item"] },
  e0433_undeclared_crate: { rustc: ["E0433"], kinds: ["unresolved-qualifier"] },
  e0433_missing_module: { rustc: ["E0433"], kinds: ["unresolved-path"] },
  e0428_duplicate_fn: { rustc: ["E0428"], kinds: ["duplicate-name"] },
  e0252_duplicate_import: { rustc: ["E0252"], kinds: ["duplicate-name"] },
  e0255_import_vs_item: { rustc: ["E0255"], kinds: ["duplicate-name"] },
  e0119_conflicting_impl: { rustc: ["E0119"], kinds: ["duplicate-impl"] },
  e0592_duplicate_method: { rustc: ["E0592"], kinds: ["duplicate-method"] },
  e0583_missing_mod_file: { rustc: ["E0583"], kinds: ["mod-missing"] },
  e0412_bare_type: { rustc: ["E0425"], kinds: ["unresolved-type"] },
  ok_macro_export_path: { rustc: [], kinds: [] },
};

const trees = new Map<string, ReturnType<typeof buildTree>>();
const treeOf = (pkg: NonNullable<ReturnType<typeof loadPackage>>) => {
  const t = pkg.targets.find((x) => x.kind === "lib")!;
  const k = pkg.dir;
  if (!trees.has(k)) trees.set(k, buildTree(pkg, t, pkg.name));
  return trees.get(k)!;
};
setTreeBuilder((pkg) => (pkg.targets.some((t) => t.kind === "lib") ? treeOf(pkg) : null));
const alpha = loadPackage(`${FX}/alpha`)!;
libIndex.set("alpha", [alpha]);

const rustc = (args: string[]) => Bun.spawnSync(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--emit=metadata", "--error-format=json", ...args], { stderr: "pipe", stdout: "pipe" });
const a = rustc(["--crate-type", "lib", "--crate-name", "alpha", "-o", `${OUT}/libalpha.rmeta`, `${FX}/alpha/lib.rs`]);
if (a.exitCode !== 0) throw new Error("alpha fixture does not compile: " + a.stderr.toString().slice(0, 500));

let bad = 0;
const rows: string[] = [];
for (const name of readdirSync(`${FX}/cases`).sort()) {
  const exp = EXPECT[name];
  if (!exp) continue;
  const pkg = loadPackage(`${FX}/cases/${name}`)!;
  const out: Finding[] = [];
  const stats: Stats = { unknown: 0, checked: 0, unknownSamples: [] };
  checkManifest(pkg, out);
  checkSource(treeOf(pkg), out, stats);
  const kinds = [...new Set(out.filter((f) => f.sev === "error").map((f) => f.kind))].sort();
  const r = rustc(["--crate-type", "lib", "--crate-name", "case", "--extern", `alpha=${OUT}/libalpha.rmeta`, "-L", OUT, "-o", `${OUT}/c-${name}.rmeta`, `${FX}/cases/${name}/lib.rs`]);
  const codes = new Set<string>();
  for (const line of r.stderr.toString().split("\n")) {
    try {
      const j = JSON.parse(line);
      if (j.level === "error" && j.code?.code) codes.add(j.code.code);
    } catch {}
  }
  const okKinds = JSON.stringify(kinds) === JSON.stringify([...exp.kinds].sort());
  const okRustc = exp.rustc.every((c) => codes.has(c)) && (exp.rustc.length > 0 || r.exitCode === 0);
  if (!okKinds || !okRustc) bad++;
  rows.push(`${okKinds && okRustc ? "PASS" : "FAIL"} ${name.padEnd(28)} checker=[${kinds.join(",")}] rustc=[${[...codes].join(",")}] exit=${r.exitCode}`);
}
console.log(rows.join("\n"));
console.log(bad === 0 ? `all ${rows.length} fixture cases agree with rustc` : `${bad} of ${rows.length} fixture cases disagree`);
process.exit(bad === 0 ? 0 : 1);
