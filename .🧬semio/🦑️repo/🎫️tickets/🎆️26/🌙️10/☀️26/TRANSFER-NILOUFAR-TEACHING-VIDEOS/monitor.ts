import { open, stat } from "node:fs/promises";
for (const path of process.argv.slice(2)) {
    const info = await stat(path);
    const file = await open(path, "r");
    try {
        const bytes = Buffer.alloc(Math.min(info.size, 4000));
        await file.read(bytes, 0, bytes.length, Math.max(0, info.size - bytes.length));
        console.log(`[DEBUG] ${path}\n${bytes.toString("utf8")}`);
    } finally { await file.close(); }
}
