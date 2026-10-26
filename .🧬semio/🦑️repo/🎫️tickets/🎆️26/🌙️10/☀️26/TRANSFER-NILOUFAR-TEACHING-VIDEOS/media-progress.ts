import { readdir, stat } from "node:fs/promises";
import { join } from "node:path";

for (const root of process.argv.slice(2)) {
    const pending = [root];
    const files: { path: string; bytes: number; modified: number }[] = [];
    while (pending.length) {
        const directory = pending.pop()!;
        for (const entry of await readdir(directory, { withFileTypes: true })) {
            const path = join(directory, entry.name);
            if (entry.isDirectory()) pending.push(path);
            else if (entry.name.endsWith(".mp4")) {
                const info = await stat(path);
                files.push({ path, bytes: info.size, modified: info.mtimeMs });
            }
        }
    }
    files.sort((left, right) => right.modified - left.modified);
    console.log("[DEBUG] " + JSON.stringify({ root, count: files.length, latest: files.slice(0, 3) }));
}
