/**
 * 📍️ M5a anchor scan — for every plugin with an OPEN gesture route, finds the declaration site to
 * fix at: the `.rs` file whose builder chain declares that id, and the enum path spelling that
 * file's neighbouring `action_interactive_job` call already uses (so a new `action_audience` line
 * imports nothing new).
 *
 * Run: `bun .🧬semio/.../🐍️m5a-audience-anchor-scan.ts <plugin>=<id>,<id> ...`
 */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const registry = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"), "utf8")) as Array<{ pluginId: string; cratePath: string }>;
const ownerRootOf = new Map(registry.map((entry) => [entry.pluginId, entry.cratePath.split("/").slice(0, -2).join("/")]));

for (const argument of process.argv.slice(2)) {
  const [plugin, rawIds] = argument.split("=");
  const dir = ownerRootOf.get(plugin!);
  if (!dir) {
    console.log(`${plugin}: NOT IN REGISTRY`);
    continue;
  }
  const ids = (rawIds ?? "").split(",").filter(Boolean);
  const anchors = spawnSync("grep", ["-rn", "--include=*.rs", "-F", "-e", ".action_interactive_job(", join(repoRoot, dir)], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 }).stdout ?? "";
  const byFile = new Map<string, { line: number; text: string }>();
  for (const row of anchors.split("\n")) {
    const match = /^(.+?):(\d+):(.*)$/.exec(row);
    if (!match) continue;
    const file = match[1]!.slice(repoRoot.length + 1);
    if (!byFile.has(file)) byFile.set(file, { line: Number(match[2]), text: match[3]!.trim() });
  }
  console.log(`\n### ${plugin} — ${ids.join(", ")}`);
  for (const [file, anchor] of byFile) console.log(`  anchor  ${file}:${anchor.line}  ${anchor.text.slice(0, 140)}`);
  for (const id of ids) {
    const declarations = spawnSync("grep", ["-rn", "--include=*.rs", "-F", "-e", `"${id}"`, join(repoRoot, dir)], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 }).stdout ?? "";
    const rows = declarations
      .split("\n")
      .filter((row) => /ActionDefinition|action_with|mutation\(|view_action|\.action\(/.test(row))
      .slice(0, 3)
      .map((row) => `    ${row.slice(repoRoot.length + 1).slice(0, 190)}`);
    console.log(`  id ${id}:`);
    for (const row of rows) console.log(row);
  }
}
