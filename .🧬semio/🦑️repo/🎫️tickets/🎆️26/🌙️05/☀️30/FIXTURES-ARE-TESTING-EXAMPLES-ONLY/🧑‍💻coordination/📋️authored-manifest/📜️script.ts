import { mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const ticket = dirname(dirname(import.meta.dir));
const output = join(ticket, "🗑️generated", "coordinator");
const files = new Set<string>();
const rejected = new Set<string>();
const inputs: { report: string; paths: number }[] = [];
const accepted = (value: unknown): value is string => typeof value === "string" && !value.endsWith("/") && (/^(?:🧰️framework\/|✏️s\/|🌎️hub\/|♻️mit-bestand\/|🧪️tests\/|🧫️fixtures\/|\.🧬semio\/|\.claude\/|\.vscode\/|undefined\/)/u.test(value) || ["Cargo.toml", "Cargo.lock", "package.json", "project.json", "nx.json", "bun.lock", "📜️script.ts", "🔒️dependencies.json"].includes(value));
const add = (value: unknown) => { if (accepted(value) && !value.includes("/🗑️generated/")) files.add(value); else if (typeof value === "string" && !value.includes("/🗑️generated/")) rejected.add(value); };
for (const entry of readdirSync(ticket).filter(path => path.endsWith(".md") && !path.includes("authored-manifest"))) {
  const source = readFileSync(join(ticket, entry), "utf8");
  let added = 0;
  for (const block of source.matchAll(/\x60\x60\x60json\s*\n([\s\S]*?)\n\x60\x60\x60/g)) {
    let value: unknown;
    try { value = JSON.parse(block[1]!); } catch { continue; }
    const heading = source.slice(0, block.index).match(/^#{1,3} .+$/gm)?.at(-1) ?? "";
    const before = files.size;
    if (/authored.*paths?|authored.*files?|exact paths|input and report inventory|additional.*paths|changed.*paths|file manifest/i.test(heading) && Array.isArray(value)) value.forEach(add);
    if (Array.isArray(value)) for (const row of value) {
      if (row && typeof row === "object" && (("old" in row && "new" in row) || ("oldPath" in row && "newPath" in row))) {
        add(row.old ?? row.oldPath);
        add(row.new ?? row.newPath);
      }
    }
    added += files.size - before;
  }
  inputs.push({ report: entry, paths: added });
}
function retained(directory: string): void {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.name === "🗑️generated") continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) retained(path);
    else if (entry.isFile()) add(relative(root, path).replaceAll("\\", "/"));
  }
}
retained(ticket);
const manifest = [...files].sort();
const status = manifest.map(path => {
  try { return { path, state: statSync(join(root, path)).isFile() ? "present" : "directory" }; }
  catch { return { path, state: "removed" }; }
});
mkdirSync(output, { recursive: true });
writeFileSync(join(output, "authored-manifest-candidate.json"), JSON.stringify({ inputs, files: manifest, status, rejected: [...rejected] }, null, 2) + "\n");
console.log("[DEBUG] " + JSON.stringify({ reports: inputs.length, paths: manifest.length, present: status.filter(row => row.state === "present").length, removed: status.filter(row => row.state === "removed").length, directories: status.filter(row => row.state === "directory"), rejected: [...rejected] }));
