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
const orig = universe.member.bind(universe);
let calls = 0;
const counts = new Map<string, number>();
(universe as any).member = (node: any, name: string, from: any, stack?: Set<string>) => {
  calls += 1;
  const k = `${node.crate.ident}:${node.name}/${name}`;
  counts.set(k, (counts.get(k) ?? 0) + 1);
  if (calls % 200000 === 0) { console.error(`[DEBUG] member calls ${calls} depth ${stack?.size} rss ${(process.memoryUsage().rss / 1e9).toFixed(2)}G`); const top = [...counts].sort((a, b) => b[1] - a[1]).slice(0, 8); console.error(top.map(([a, b]) => `${b} ${a}`).join("\n")); }
  if (calls > 1500000) process.exit(4);
  return orig(node, name, from, stack);
};
const path = process.argv[2] ?? `${art}/🦀️.rs`;
const t = Date.now();
engine.processFile(path);
console.error(`[DEBUG] done ${path} ${Date.now() - t}ms calls ${calls} edits ${engine.report.edits.size}`);
