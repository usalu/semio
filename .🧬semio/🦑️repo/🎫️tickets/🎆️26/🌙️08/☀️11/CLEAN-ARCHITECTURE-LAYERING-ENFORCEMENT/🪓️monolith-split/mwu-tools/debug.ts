import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join, posix } from "node:path";
import { Universe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🟦️.ts";
const REPO = "/Users/ueli/Documents/semio";
const [dirPrefix, file, ...rest] = process.argv.slice(2);
const git = (...args: string[]): string => execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: REPO, encoding: "utf8", maxBuffer: 1 << 29 });
const u = new Universe(REPO, new Map());
for (const manifest of git("ls-files", "-z", "--", `${dirPrefix}/**/Cargo.toml`).split("\0").filter(Boolean)) {
  const abs = join(REPO, manifest); if (!existsSync(abs)) continue;
  const parsed = Bun.TOML.parse(readFileSync(abs, "utf8")) as any; const name = parsed.package?.name; if (!name) continue;
  const dir = posix.dirname(manifest); const root = posix.normalize(posix.join(dir, parsed.lib?.path ?? "src/lib.rs"));
  if (existsSync(join(REPO, root))) u.addCrate(name, dir, root);
}
for (const c of u.crates.values()) u.root(c.ident);
const info = u.files.get(file!)!;
const node = info.nodes[0]!;
console.log("node crate", node.crate.name, "uses", node.uses.length);
for (const path of rest) {
  const segs = path.split("::");
  const r = u.resolvePath(node, segs);
  if (r) for (const h of r.hits) console.log("  node", h.node.crate.ident, h.node.name, h.node.file.slice(-40), "items", [...h.node.items.keys()].slice(0,8), "children", [...h.node.children.keys()].slice(0,8), "uses", h.node.uses.length, "inline", h.node.inline);
  console.log(path, "=>", r ? JSON.stringify({ consumed: r.consumed, hits: r.hits.map((h) => ({ kind: h.kind, crate: h.node.crate.ident, name: h.node.name, rep: h.rep })) }) : null);
}
