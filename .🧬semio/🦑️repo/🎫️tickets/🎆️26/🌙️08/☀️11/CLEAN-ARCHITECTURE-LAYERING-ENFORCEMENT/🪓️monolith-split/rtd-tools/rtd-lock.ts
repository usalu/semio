// 🔒️ Surgical Cargo.lock patch equal to `cargo update --workspace --offline` for this change: per touched package, drops the dependency entries whose last edge was removed and inserts the new producer entries in sorted position. Dry run unless --apply. argv: manifest list, baseline manifests dir.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {readFileSync, renameSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {loadCrate} from "./rtd-world.ts";

const ROOT = "/Users/ueli/Documents/semio";
const APPLY = process.argv.includes("--apply");
const [listFile, baseDir] = process.argv.slice(2, 4) as [string, string];
const wsDeps: Record<string, any> = (Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any).workspace?.dependencies ?? {};
const depNames = (file: string, dir: string): Set<string> => {
  const t = Bun.TOML.parse(readFileSync(file, "utf8")) as any;
  const out = new Set<string>();
  const add = (tbl: any) => {
    for (const [k, raw] of Object.entries(tbl ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const ws = spec.workspace === true ? wsDeps[spec.package ?? k] ?? wsDeps[k] : undefined;
      const p = spec.path ?? ws?.path;
      if (typeof p === "string") { const c = loadCrate(join(spec.path === undefined ? ROOT : dir, p)); out.add(c?.pkg ?? spec.package ?? k); } else out.add(spec.package ?? k);
    }
  };
  add(t.dependencies); add(t["dev-dependencies"]); add(t["build-dependencies"]);
  for (const b of Object.values(t.target ?? {}) as any[]) { add(b.dependencies); add(b["dev-dependencies"]); add(b["build-dependencies"]); }
  return out;
};
let lock = readFileSync(join(ROOT, "Cargo.lock"), "utf8");
const original = lock;
let changed = 0;
for (const m of readFileSync(listFile, "utf8").split("\n").filter(Boolean)) {
  const dir = join(ROOT, m, "..");
  const cur = depNames(join(ROOT, m), dir);
  const base = depNames(join(baseDir, m), dir);
  const added = [...cur].filter(x => !base.has(x)), removed = [...base].filter(x => !cur.has(x));
  const pkg = (Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any).package.name as string;
  const head = `[[package]]\nname = "${pkg}"\nversion = `;
  const at = lock.indexOf(head);
  if (at < 0) { console.log(`SKIP ${pkg}: not in Cargo.lock`); continue; }
  const dstart = lock.indexOf("dependencies = [\n", at);
  const next = lock.indexOf("\n[[package]]", at + 10);
  if (dstart < 0 || (next >= 0 && dstart > next)) { console.log(`SKIP ${pkg}: no dependencies array`); continue; }
  const dend = lock.indexOf("\n]\n", dstart);
  const entries = lock.slice(dstart + "dependencies = [\n".length, dend).split("\n").map(l => l.trim()).filter(Boolean).map(l => l.replace(/^"|",?$/g, ""));
  const nameOf = (e: string) => e.split(" ")[0]!;
  let set = entries.slice();
  for (const r of removed) set = set.filter(e => nameOf(e) !== r);
  set = set.filter(e => !(nameOf(e).startsWith("semio-framework-ui") && !cur.has(nameOf(e))));
  for (const a of added) if (!set.some(e => nameOf(e) === a)) set.push(a);
  set.sort((x, y) => (x < y ? -1 : x > y ? 1 : 0));
  const body = set.map(e => ` "${e}",`).join("\n");
  const patched = lock.slice(0, dstart + "dependencies = [\n".length) + body + lock.slice(dend);
  if (patched !== lock) { lock = patched; changed++; console.log(`${pkg.replace("semio-framework-os-renderer-", "")}: +${added.length} -${removed.length}  (${added.map(x => x.replace("semio-framework-", "")).join(",")} | ${removed.map(x => x.replace("semio-framework-", "")).join(",")})`); }
}
console.log(changed, "package block(s) patched");
if (APPLY && changed) {
  const now = readFileSync(join(ROOT, "Cargo.lock"), "utf8");
  if (now !== original) throw new Error("Cargo.lock changed under us; rerun");
  const tmp = join(ROOT, "Cargo.lock.rtd-tmp");
  writeFileSync(tmp, lock);
  renameSync(tmp, join(ROOT, "Cargo.lock"));
  console.log("Cargo.lock written");
}
