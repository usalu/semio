/** 🧭 Scoped facade-tail repoint planner: rewrites the listed consumer crates' facade paths of the tail modules to their owner crates and plans the owner dependencies. Writes a plan, never a source file.
 *  usage: bun repoint.ts --consumers consumers.json --out plan.json [--moved moved-tail.json] [--root DIR] */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { createHash } from "node:crypto";
import { relative } from "node:path";
import { initParser, findWasm } from "../../../rs-sweep-tools/parse.ts";
import { loadPackage, Package } from "../../../rs-sweep-tools/cargo.ts";
import { ROOT, loadOwners } from "./resolve-owner.ts";
import { Moved, Note, transform } from "./transform.ts";
import { planManifests } from "./manifests.ts";

const args = process.argv.slice(2);
const flag = (n: string) => (args.indexOf(n) >= 0 ? args[args.indexOf(n) + 1] : undefined);
const consumersFile = flag("--consumers")!;
const outFile = flag("--out")!;
const moved: Moved = JSON.parse(readFileSync(flag("--moved") ?? `${import.meta.dir}/moved-tail.json`, "utf8"));
const consumers: { dir: string; name: string; files: string[] }[] = JSON.parse(readFileSync(consumersFile, "utf8"));
await initParser(findWasm("/Users/ueli/Documents/semio")!);
const libs = [...new Set<string>([...Object.values(moved.modules), ...Object.values(moved.functions)])];
const owners = loadOwners(libs);
const ctx = { moved, facade: "semio_framework_value", verify: owners.verify, aliasHeads: ["pack", "protocol", "replication"], globHeads: ["protocol", "semio_framework_replication", "replication"] };
const sha = (s: string) => createHash("sha256").update(s).digest("hex");
const rel = (p: string) => relative(ROOT, p);
const sources: { path: string; before: string; after: string }[] = [];
const notes: { file: string; note: Note }[] = [];
const libsByPkg = new Map<string, { pkg: Package; libs: Set<string> }>();
let rewrites = 0;
for (const c of consumers) {
  const pkg = loadPackage(c.dir);
  for (const f of c.files) {
    const src = readFileSync(f, "utf8");
    const r = transform(src, ctx);
    for (const n of r.notes) notes.push({ file: rel(f), note: n });
    if (r.out === src) continue;
    sources.push({ path: rel(f), before: sha(src), after: r.out });
    rewrites += r.rewrites;
    if (pkg) {
      const e = libsByPkg.get(pkg.dir) ?? { pkg, libs: new Set<string>() };
      for (const l of r.libs) e.libs.add(l);
      libsByPkg.set(pkg.dir, e);
    }
  }
}
const sink = new Map<string, { before: string; after: string }>();
const man = planManifests(libsByPkg, owners, false, "", sink);
const manifests = [...sink].map(([p, v]) => ({ path: rel(p), before: sha(v.before), after: v.after }));
writeFileSync(outFile, JSON.stringify({ sources, manifests, notes, issues: man.issues, rewrites }, null, 1));
const kinds: Record<string, number> = {};
for (const n of notes) kinds[n.note.kind] = (kinds[n.note.kind] ?? 0) + 1;
console.log(JSON.stringify({ consumers: consumers.length, sources: sources.length, rewrites, manifests: manifests.length, wsDecls: man.wsDecls, issues: man.issues.length, noteKinds: kinds }));
