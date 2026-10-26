import { readdir, writeFile } from "node:fs/promises";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const generated = join(ticket, "🗑️generated");
const pending = [generated];
const paths: string[] = [];
while (pending.length) {
    const directory = pending.pop()!;
    for (const entry of await readdir(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        paths.push(path);
        if (entry.isDirectory()) pending.push(path);
    }
}
const long = paths.filter(path => path.length > 256).sort((left, right) => right.length - left.length);
const groups: Record<string, number> = {};
for (const path of long) {
    const key = relative(generated, path).split(/[\\/]/)[0]!;
    groups[key] = (groups[key] ?? 0) + 1;
}
const longest = paths.sort((left, right) => right.length - left.length)[0];
await writeFile(join(ticket, "path-audit.md"), `# Generated Path Audit\n\nAudited ${paths.length} existing generated entries. ${long.length} exceed 256 UTF-16 code units. Longest path: ${longest?.length ?? 0}.\n\nCounts by generated child: \`${JSON.stringify(groups)}\`.\n\n${long.slice(0, 20).map(path => `- ${path.length}: \`${path}\``).join("\n")}\n`);
console.log("[DEBUG] " + JSON.stringify({ entries: paths.length, overlong: long.length, longest: longest?.length, groups, longestPaths: long.slice(0, 3) }));
