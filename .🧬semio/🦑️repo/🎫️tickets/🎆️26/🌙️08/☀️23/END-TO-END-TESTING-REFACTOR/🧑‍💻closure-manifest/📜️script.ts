import { readFile, writeFile } from "node:fs/promises";
import { dirname, join, relative } from "node:path";

const root = process.env.SEMIO_LAYOUT_REPO_ROOT;
if (!root) throw new Error("SEMIO_LAYOUT_REPO_ROOT is required");
const ticket = dirname(import.meta.dir);
const report = join(ticket, "📓️test-layout-complete-authored-files-2026-09-08.md");
const slash = (path: string) => path.replaceAll("\\", "/");
const authored = new Set<string>([slash(relative(root, import.meta.path)), slash(relative(root, report)), slash(relative(root, join(ticket, "🎫️ticket.json")))]);
const sources = process.argv.slice(2);
if (!sources.length) throw new Error("Explicit source manifest names are required");
const counts: Array<{ path: string; entries: number }> = [];
for (const name of sources) {
  const path = join(ticket, name);
  const source = await readFile(path, "utf8");
  const arrays = [...source.matchAll(/```json\r?\n([\s\S]*?)\r?\n```/g)].map(match => JSON.parse(match[1]!)).filter(value => Array.isArray(value) && value.length > 0 && value.every(item => typeof item === "string")) as string[][];
  if (!arrays.length) throw new Error(`No flat authored-path array in ${name}`);
  let entries = 0;
  for (const files of arrays) for (const file of files) {
    if (file.startsWith("/") || file.split("/").some(part => part === ".." || part === "🗑️generated") || file.endsWith("/AGENTS.md")) throw new Error(`Invalid authored path in ${name}: ${file}`);
    authored.add(file);
    entries++;
  }
  authored.add(slash(relative(root, path)));
  counts.push({ path: slash(relative(root, path)), entries });
}
const files = [...authored].sort();
const compact = JSON.stringify(files);
const sha256 = new Bun.CryptoHasher("sha256").update(compact).digest("hex");
const bytes = Buffer.byteLength(compact);
const markdown = "# Complete Authored Test Layout Paths\n\nThis manifest combines the explicit executor and coordinator arrays. Removed paths are retained. Evidence-only mappings, generated outputs, and unrelated concurrent Git changes are excluded. It does not infer attribution from current Git status.\n\n" + counts.map(row => `- \`${row.path}\`: ${row.entries} entries before deduplication.`).join("\n") + `\n\nUnique paths: ${files.length}. Compact UTF-8 bytes: ${bytes}. SHA-256: \`${sha256}\`.\n\n\`\`\`json\n` + JSON.stringify(files, null, 2) + "\n```\n";
if (Buffer.byteLength(markdown) > 5 * 1024 * 1024) throw new Error("Manifest exceeds ticket retained-file limit");
await writeFile(report, markdown);
console.log(JSON.stringify({ files: files.length, bytes, sha256, report: slash(relative(root, report)) }));
