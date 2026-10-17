// 🧪️ Cross-crate cfg(test) check on a kept combined layout sandbox (`new crates <pkg> --check-applied --keep`): for every crate X, all other crates are compiled with `cfg(test)` switched off while X keeps it, and the new resolution errors located in X's files are the references X makes to another crate's cfg(test) items. Usage: `bun cfgtest.ts <combinedRoot>`; writes only inside copies below `<combinedRoot>-cfg`.
import {cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync} from "node:fs";
import {execFileSync, spawnSync} from "node:child_process";
import {dirname, join, resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root = resolve(process.argv[2]!);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const mounts = [...readFileSync(join(root, "__combined.rs"), "utf8").matchAll(/#\[path = "([^"]+)"\]\s*pub mod (\w+);/gu)].map(m => ({file: m[1]!, module: m[2]!}));
const read = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
const closures = new Map<string, string[]>();
for (const m of mounts) {
  const entry = resolve(root, m.file);
  const inputs = await readRustSourceClosureV1([entry], root, [...host, "debug_assertions", "test"], read, {manifestDirectory: dirname(entry), generated: () => undefined});
  closures.set(m.module, inputs.filter(i => i.kind === "source").map(i => resolve(root, (i as any).path)));
}
const run = (dir: string): string[] => {
  const r = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "combined", "--emit=metadata", "--error-format=short", "-o", "/dev/null", "--test", join(dir, "__combined.rs")], {encoding: "utf8", maxBuffer: 1 << 30, cwd: dir});
  return (r.stderr ?? "").split("\n").filter(l => /: error/u.test(l)).map(l => l.replaceAll(dir, "<r>"));
};
const key = (l: string) => l.replace(/:\d+:\d+:/u, ":").replace(/^.*?(?=✏️s\/)/u, "");
const baseline = run(root);
const baseCount = new Map<string, number>();
for (const l of baseline) baseCount.set(key(l), (baseCount.get(key(l)) ?? 0) + 1);
const out = `${root}-cfg`;
let bad = 0;
for (const x of mounts) {
  rmSync(out, {recursive: true, force: true});
  cpSync(root, out, {recursive: true});
  for (const y of mounts) {
    if (y.module === x.module) continue;
    for (const f of closures.get(y.module)!) {
      const target = f.replace(root, out);
      if (!existsSync(target)) continue;
      const t = readFileSync(target, "utf8");
      const u = t.replace(/cfg\(\s*test\s*\)/gu, "cfg(any())");
      if (u !== t) writeFileSync(target, u);
    }
  }
  const after = run(out);
  const seen = new Map<string, number>();
  const fresh: string[] = [];
  for (const l of after) { const k = key(l); const n = (seen.get(k) ?? 0) + 1; seen.set(k, n); if (n > (baseCount.get(k) ?? 0)) fresh.push(l); }
  const mine = fresh.filter(l => closures.get(x.module)!.some(f => l.includes(f.replace(root, "<r>"))));
  console.log(`${x.module}: ${mine.length} new errors in its own files (${fresh.length} total new)`);
  for (const l of mine.slice(0, 15)) console.log("   " + l.slice(0, 260));
  bad += mine.length;
}
rmSync(out, {recursive: true, force: true});
console.log("cross-crate cfg(test) references:", bad);
