// 🔍️ Consistency of Cargo.lock dependency arrays against the manifests of the listed packages (names only; tables normal, dev, build, target). argv[2]: manifest list. Reports drift per package.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {loadCrate} from "./rtd-world.ts";
const ROOT = "/Users/ueli/Documents/semio";
const wsDeps: Record<string, any> = (Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any).workspace?.dependencies ?? {};
const lock = readFileSync(join(ROOT, "Cargo.lock"), "utf8");
let drift = 0, n = 0;
for (const m of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const dir = join(ROOT, m, "..");
  const t = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any;
  const names = new Set<string>();
  const add = (tbl: any) => {
    for (const [k, raw] of Object.entries(tbl ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const ws = spec.workspace === true ? wsDeps[spec.package ?? k] ?? wsDeps[k] : undefined;
      const p = spec.path ?? ws?.path;
      if (typeof p === "string") names.add(loadCrate(join(spec.path === undefined ? ROOT : dir, p))?.pkg ?? spec.package ?? k); else names.add(spec.package ?? ws?.package ?? k);
    }
  };
  add(t.dependencies); add(t["dev-dependencies"]); add(t["build-dependencies"]);
  for (const b of Object.values(t.target ?? {}) as any[]) { add(b.dependencies); add(b["dev-dependencies"]); add(b["build-dependencies"]); }
  const head = `[[package]]\nname = "${t.package.name}"\nversion = `;
  const at = lock.indexOf(head);
  if (at < 0) { console.log("NOT IN LOCK", t.package.name); drift++; continue; }
  const ds = lock.indexOf("dependencies = [\n", at), de = lock.indexOf("\n]\n", ds);
  const inLock = new Set(lock.slice(ds + 17, de).split("\n").map(l => l.trim()).filter(Boolean).map(l => l.replace(/^"|",?$/g, "").split(" ")[0]!));
  const missing = [...names].filter(x => !inLock.has(x)), extra = [...inLock].filter(x => !names.has(x));
  n++;
  if (missing.length || extra.length) { drift++; console.log(`DRIFT ${t.package.name.replace("semio-framework-os-renderer-", "")}: manifest-only [${missing.join(",")}] lock-only [${extra.join(",")}]`); }
}
console.log(n, "packages checked;", drift, "with drift");
