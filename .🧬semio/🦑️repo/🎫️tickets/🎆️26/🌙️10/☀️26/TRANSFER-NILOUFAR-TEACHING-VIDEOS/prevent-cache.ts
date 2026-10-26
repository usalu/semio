import { lstat, realpath, rm, writeFile } from "node:fs/promises";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const generated = await realpath(join(ticket, "🗑️generated"));
const customization = join(generated, "v/Lib/site-packages/sitecustomize.py");
await writeFile(customization, "import sys\nsys.dont_write_bytecode = True\n");
await writeFile(join(generated, "v/Lib/site-packages/0-no-bytecode.pth"), "import sys; sys.dont_write_bytecode = True\n");
for (const suffix of ["pycache", "videos/pycache"]) {
    const target = resolve(generated, suffix);
    const inside = relative(generated, target);
    if (!inside || inside.startsWith(`..${sep}`) || inside === ".." || resolve(generated, inside) !== target) throw new Error(`Unsafe cache target: ${target}`);
    try {
        const info = await lstat(target);
        if (!info.isDirectory() || info.isSymbolicLink() || await realpath(target) !== target) throw new Error(`Cache target is not a regular directory: ${target}`);
        await rm(target, { recursive: true, force: true, maxRetries: 3, retryDelay: 50 });
        console.log(`[DEBUG] Removed generated Python cache: ${target}`);
    } catch (error) {
        if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    }
}
