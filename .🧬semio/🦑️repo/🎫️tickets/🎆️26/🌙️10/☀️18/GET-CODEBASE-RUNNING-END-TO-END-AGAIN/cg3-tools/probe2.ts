import { Universe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🟦️.ts";
import { Unmirror } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🟦️.ts";
import { facadeIdents } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🏃️run/🟦️.ts";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join, posix } from "node:path";
const repo = process.argv[2]!;
const art = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model";
const universe = new Universe(repo);
const walk = (dir: string, out: string[]): string[] => { for (const n of readdirSync(join(repo, dir))) { if (n === "node_modules") continue; const rel = `${dir}/${n}`; if (statSync(join(repo, rel)).isDirectory()) walk(rel, out); else if (n === "Cargo.toml") out.push(rel); } return out; };
for (const manifest of walk("✏️s/🔌️plugins/🏙️bim", [])) {
  const parsed = Bun.TOML.parse(readFileSync(join(repo, manifest), "utf8")) as any;
  const name = parsed.package?.name;
  if (!name?.startsWith("semio-s-artifact-")) continue;
  const dir = posix.dirname(manifest);
  const root = posix.normalize(posix.join(dir, parsed.lib?.path ?? "src/lib.rs"));
  if (existsSync(join(repo, root))) universe.addCrate(name, dir, root);
}
const all = [...universe.crates.values()];
const targets = all.filter((c) => c.dir.startsWith(`${art}/`));
for (const c of targets) universe.root(c.ident);
const engine = new Unmirror(universe, { apiRoots: facadeIdents(all), targets: new Set(targets.map((c) => c.ident)), editable: () => true });
const path = process.argv[3]!;
const probeNode = universe.files.get(path)!.nodes[0]!;
const names = "semio_s_artifact_bim_model_dispatch_context::editor::bim::transient::register".split("::");
const r = universe.resolvePath(probeNode, names);
console.error("[DEBUG] resolve", r ? `${r.consumed}/${names.length} ${r.hits.map((h) => `${h.kind}@${h.node.crate.ident}:${h.node.name}:${h.node.file.slice(-40)} opaque=${h.node.opaque} deep=${universe.opaqueModule(h.node)} uses=${h.node.uses.map((u) => u.leaf.path.join("::") + (u.leaf.glob ? "::*" : "")).join(",")} children=${[...h.node.children.keys()].join(",")}`).join(" | ")}` : "null");
engine.processFile(path);
const out = engine.report.edits.get(path);
console.error(`[DEBUG] edited ${out !== undefined}`);
if (out) { await Bun.write(process.argv[4]!, out); }
for (const r of engine.report.unresolvedMirrors) console.error(`[DEBUG] unresolved ${r}`);
