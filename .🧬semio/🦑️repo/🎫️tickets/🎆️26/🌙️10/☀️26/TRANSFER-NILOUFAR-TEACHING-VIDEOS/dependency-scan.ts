import { readdir, readFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const roots = [resolve(".cache"), resolve(".🧬semio/🦑️repo/⚡️cache"), join(process.env.LOCALAPPDATA!, "Microsoft/WinGet/Packages"), join(process.env.LOCALAPPDATA!, "ms-playwright"), ...process.env.PATH!.split(";").filter(Boolean)];
const found = new Set<string>();
for (const root of roots) {
    const pending = [root];
    const recursive = roots.indexOf(root) < 4;
    while (pending.length) {
        const directory = pending.pop()!;
        try {
            for (const entry of await readdir(directory, { withFileTypes: true })) {
                const path = join(directory, entry.name);
                if (entry.isDirectory() && recursive) pending.push(path);
                else if (entry.isFile() && /(?:ffmpeg|ffprobe).*\.exe$/i.test(entry.name)) found.add(path);
            }
        } catch (error) {
            if (!["ENOENT", "ENOTDIR", "EACCES"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error;
        }
    }
}
console.log("[DEBUG] " + JSON.stringify({ executables: [...found].sort(), runInterface: await readFile(join(ticket, "run.ts"), "utf8").catch(() => "absent") }));
