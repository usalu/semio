/** 🧭️ Rule R2 over Cargo: counts the layout plugin's manifest edges (direct path and workspace-inherited) that resolve into another plugin, before (a git revision) and after (the working tree). Read-only. */
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { posix } from "node:path";

const root = "/Users/ueli/Documents/semio";
const plugin = "✏️s/🔌️plugins/📏️layout/";
const revision = process.argv[2] ?? "HEAD";

const git = (args: string[]) => spawnSync("git", ["-c", "core.quotepath=false", ...args], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout;
const tracked = git(["ls-files", "--", `${plugin}**/Cargo.toml`, `${plugin}Cargo.toml`]).split("\n").filter(Boolean);
const untracked = git(["ls-files", "--others", "--exclude-standard", "--", `${plugin}**/Cargo.toml`]).split("\n").filter(Boolean);
const afterFiles = [...new Set([...tracked, ...untracked])].filter((f) => !f.includes("node_modules"));
const beforeFiles = git(["ls-tree", "-r", "--name-only", revision, "--", plugin]).split("\n").filter((f) => f.endsWith("Cargo.toml") && !f.includes("node_modules"));

type Edge = { from: string; dep: string; to: string };
function edges(files: string[], read: (file: string) => string | undefined): Edge[] {
  const parsed = new Map<string, any>();
  for (const f of files) {
    const text = read(f);
    if (text === undefined) continue;
    try { parsed.set(f, Bun.TOML.parse(text)); } catch { /* unreadable manifest */ }
  }
  const workspaceFor = (file: string) => {
    let dir = posix.dirname(file);
    while (dir !== ".") {
      const candidate = `${dir}/Cargo.toml`;
      const toml = parsed.get(candidate);
      if (toml?.workspace) return { file: candidate, toml };
      dir = posix.dirname(dir);
    }
    return undefined;
  };
  const out: Edge[] = [];
  for (const [file, toml] of parsed) {
    const base = posix.dirname(file);
    for (const section of ["dependencies", "dev-dependencies", "build-dependencies"]) {
      for (const [name, spec] of Object.entries<any>(toml[section] ?? {})) {
        let path: string | undefined, from = base;
        if (typeof spec === "object" && spec.path) path = spec.path;
        else if (typeof spec === "object" && spec.workspace === true) {
          const ws = workspaceFor(file);
          const entry = ws?.toml.workspace?.dependencies?.[name];
          if (entry?.path) { path = entry.path; from = posix.dirname(ws!.file); }
        }
        if (!path) continue;
        const target = posix.normalize(posix.join(from, path));
        if (target.startsWith("✏️s/🔌️plugins/") && !target.startsWith(plugin)) out.push({ from: file, dep: name, to: target });
      }
    }
  }
  return out;
}

const before = edges(beforeFiles, (f) => git(["show", `${revision}:${f}`]) || undefined);
const after = edges(afterFiles, (f) => { try { return readFileSync(`${root}/${f}`, "utf8"); } catch { return undefined; } });
const group = (list: Edge[]) => {
  const byPlugin: Record<string, number> = {};
  for (const e of list) { const p = e.to.split("/")[2]; byPlugin[p] = (byPlugin[p] ?? 0) + 1; }
  return byPlugin;
};
console.log(`revision=${revision} manifests_before=${beforeFiles.length} manifests_after=${afterFiles.length}`);
console.log(`R2 cargo edges layout -> other plugin: before=${before.length} after=${after.length}`);
console.log(`before by plugin ${JSON.stringify(group(before))}`);
console.log(`after by plugin ${JSON.stringify(group(after))}`);
for (const e of after) console.log(`AFTER ${e.from} -> ${e.dep} -> ${e.to}`);
const forms = after.filter((e) => e.to.includes("📋️forms"));
console.log(`remaining non-stdio edges: ${after.filter((e) => !e.to.includes("🗄️stdio")).length}`);
void forms;
