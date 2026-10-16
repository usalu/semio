// 🧭️ Offline emulation of the build graph's "Canonical native dependency owner required" admission: every path dependency root reachable from a producer must be a registered owner root.
import {existsSync, readFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const STDIO = join(ROOT, "✏️s/🔌️plugins/🗄️stdio");
const owners = JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as {name: string; root: string}[];
const roots = new Set(owners.map(row => resolve(ROOT, row.root)));
const workspaceDeps = (Bun.TOML.parse(readFileSync(join(STDIO, "Cargo.toml"), "utf8")) as any).workspace.dependencies ?? {};
const start = [join(STDIO, "📇️registry/🧬️contract/📦️packages/🦀️rust")];
for (const dir of Bun.spawnSync(["/usr/bin/find", join(STDIO, "📇️registry/🧬️contract"), "-path", "*/📦️packages/🦀️rust/Cargo.toml"]).stdout.toString().trim().split("\n")) start.push(dirname(dir));
const seen = new Set<string>(), missing = new Set<string>();
const visit = (dir: string) => {
  if (seen.has(dir)) return;
  seen.add(dir);
  if (!roots.has(dir)) missing.add(dir.slice(ROOT.length + 1));
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) return;
  const toml = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  for (const table of [toml.dependencies, toml["dev-dependencies"], toml["build-dependencies"]]) for (const [key, spec] of Object.entries(table ?? {}) as [string, any][]) {
    const declared = spec?.workspace ? workspaceDeps[key] : spec;
    if (declared?.path) visit(resolve(spec?.workspace ? STDIO : dir, declared.path));
  }
};
for (const dir of new Set(start)) visit(dir);
console.log(JSON.stringify({visited: seen.size, ownerRootsMissing: [...missing]}));
