import { Universe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🟦️.ts";
import { Unmirror } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🟦️.ts";
import { facadeIdents } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🏃️run/🟦️.ts";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join, posix } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const art = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model";
const universe = new Universe(repo);
const files = execFileSync("git", ["ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "✏️s"], { cwd: repo, maxBuffer: 1 << 29 }).toString().split("\0").filter((p) => p.endsWith("/Cargo.toml") && p.includes("🏙️bim"));
for (const manifest of files) {
  const text = readFileSync(join(repo, manifest), "utf8");
  const parsed = Bun.TOML.parse(text) as any;
  const name = parsed.package?.name;
  if (!name?.startsWith("semio-s-artifact-")) continue;
  const dir = posix.dirname(manifest);
  const root = posix.normalize(posix.join(dir, parsed.lib?.path ?? "src/lib.rs"));
  if (existsSync(join(repo, root))) universe.addCrate(name, dir, root);
}
const all = [...universe.crates.values()];
const targets = all.filter((c) => c.dir.startsWith(`${art}/`));
for (const c of targets) universe.root(c.ident);
console.error(`[DEBUG] crates ${all.length} targets ${targets.length} files ${universe.files.size}`);
const engine = new Unmirror(universe, { apiRoots: facadeIdents(all), targets: new Set(targets.map((c) => c.ident)), editable: () => true });
const path = process.argv[2] ?? `${art}/🦀️.rs`;
const t = Date.now();
if (process.argv[3] === "col") {
  const root = universe.files.get(path)!.nodes[0]!;
  for (const bound of root.uses) {
    const hits = universe.leafHits(bound);
    console.error(`[DEBUG] use ${bound.leaf.path.join("::")}${bound.leaf.glob ? "::*" : ""} -> ${hits.map((h) => `${h.kind}@${h.node.crate.ident}:${h.node.file.slice(-40)}:${h.node.name}`).join(" | ")}`);
    if (bound.leaf.glob) for (const hit of hits) if (hit.kind === "module") { const m = universe.member(hit.node, "Column", root); console.error(`[DEBUG]   Column in ${hit.node.name}: ${m.length}; ignoring: ${universe.memberIgnoringVisibility(hit.node, "Column", root).length}; items ${hit.node.items.size} uses ${hit.node.uses.length}`); }
  }
  process.exit(0);
}
engine.processFile(path);
console.error(`[DEBUG] done ${path} ${Date.now() - t}ms lookups ${lookups} edits ${engine.report.edits.size}`);
