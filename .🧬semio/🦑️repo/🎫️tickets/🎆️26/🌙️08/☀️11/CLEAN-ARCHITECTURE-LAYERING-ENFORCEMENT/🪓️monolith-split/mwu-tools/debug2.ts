import { existsSync, readFileSync } from "node:fs";
import { join, posix } from "node:path";
import { Unmirror, } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🟦️.ts";
import { Universe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🟦️.ts";
const [root, artifact, ...files] = process.argv.slice(2);
const u = new Universe(root!);
for (const m of new Bun.Glob("**/Cargo.toml").scanSync({ cwd: join(root!, artifact!) })) {
  const manifest = posix.join(artifact!, m); const parsed = Bun.TOML.parse(readFileSync(join(root!, manifest), "utf8")) as any;
  const name = parsed.package?.name; if (!name?.startsWith("semio-s-artifact-")) continue;
  const dir = posix.dirname(manifest); const r = posix.normalize(posix.join(dir, parsed.lib?.path ?? "src/lib.rs"));
  if (existsSync(join(root!, r))) u.addCrate(name, dir, r);
}
const all = [...u.crates.values()];
for (const c of all) u.root(c.ident);
const facades = new Set(all.filter((c) => /\/🗿️artifacts\/[^/]+\/📦️packages\/🦀️rust$/u.test(c.dir)).map((c) => c.ident));
const e = new Unmirror(u, { apiRoots: facades, targets: new Set(all.map((c) => c.ident)), editable: () => true });
for (const f of files) { e.processFile(f); const t = e.report.edits.get(f); console.log("==", f.slice(-60)); console.log(t ? t.split("\n").slice(0, 30).join("\n") : "(no edit)"); }
if (process.env.PROBE) {
  const f = files[0]!;
  const info = u.files.get(f)!;
  const node = info.nodes[0]!;
  for (const probe of process.env.PROBE.split(";")) {
    const r = u.resolvePath(node, probe.split("::"));
    console.log("PROBE", probe.slice(-50), r ? JSON.stringify(r.hits.map((h) => ({ k: h.kind, c: h.node.crate.ident.slice(-14), n: h.node.name, rep: h.rep ? { i: h.rep.ident.slice(-14), s: h.rep.segs.length, cut: h.rep.cut } : null })) ) + " consumed " + r.consumed : "null");
  }
}
if (process.env.OWN) {
  const node = u.files.get(files[0]!)!.nodes[0]!;
  const r = u.resolvePath(node, process.env.OWN.split("::"))!;
  const m = r.hits[0]!.node;
  console.log("OWN", (e as any).ownNamespace(m), "items", m.items.size, "children", [...m.children.entries()].map(([k, v]) => k + ":" + v.map((c) => c.inline).join(",")), "uses", m.uses.map((b) => (b.leaf.glob ? "*" : "") + b.leaf.path.slice(-3).join("::") + " mirror=" + (e as any).isMirrorEdge(b) + " pub=" + b.statement.visibility));
}
console.log("UNRES", JSON.stringify(e.report.unresolvedMirrors));
