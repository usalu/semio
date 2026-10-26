import { createHash } from "node:crypto";
import { mkdir, readdir, readFile, stat, writeFile } from "node:fs/promises";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const source = resolve("C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial");
const destination = resolve("C:/git/semio/🎓️teaching");
const excluded = new Set([".agents", "__pycache__", "media", "rendered", ".venv"]);
const hash = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const pending = [source];
const paths: string[] = [];
while (pending.length) {
    const directory = pending.pop()!;
    for (const entry of await readdir(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory()) {
            if (!excluded.has(entry.name)) pending.push(path);
        } else if (entry.isFile()) paths.push(path);
        else throw new Error(`Unsupported source entry: ${path}`);
    }
}
paths.sort();
if (paths.length !== 131) throw new Error(`Expected 131 canonical files; found ${paths.length}`);
const seen = new Set<string>();
const manifest: { path: string; target: string; bytes: Uint8Array; sha256: string; exists: boolean }[] = [];
for (const path of paths) {
    const name = relative(source, path);
    const target = join(destination, name);
    const key = name.toLowerCase();
    if (seen.has(key)) throw new Error(`Case-insensitive duplicate: ${name}`);
    seen.add(key);
    if (target.length > 256) throw new Error(`Destination path exceeds 256 characters: ${target}`);
    if (name.split(sep).some(part => part === "AGENTS.md")) throw new Error(`Instruction file is outside the transfer scope: ${name}`);
    const bytes = await readFile(path);
    const sha256 = hash(bytes);
    let exists = false;
    try {
        const info = await stat(target);
        if (!info.isFile() || hash(await readFile(target)) !== sha256) throw new Error(`Destination collision: ${target}`);
        exists = true;
    } catch (error) {
        if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    }
    manifest.push({ path: name.split(sep).join("/"), target, bytes, sha256, exists });
}
for (const entry of manifest) {
    if (!entry.exists) {
        await mkdir(dirname(entry.target), { recursive: true });
        await writeFile(entry.target, entry.bytes, { flag: "wx" });
    }
    if (hash(await readFile(entry.target)) !== entry.sha256) throw new Error(`Transferred bytes differ: ${entry.target}`);
}
const rows = manifest.map(entry => `| \`${entry.path}\` | ${entry.bytes.length} | \`${entry.sha256}\` |`).join("\n");
await writeFile(join(ticket, "transfer.md"), `# Teaching Video Transfer\n\nTransferred and verified ${manifest.length} canonical source files from \`${source}\` to \`${destination}\`. SHA-256 was calculated from each source snapshot and checked against the destination. Existing identical files were retained. Every differing destination collision is rejected before writing. The existing teaching root README and unrelated files remain in place.\n\nExcluded directory names: ${[...excluded].map(name => `\`${name}\``).join(", ")}.\n\n| Relative Path | Bytes | SHA-256 |\n| --- | ---: | --- |\n${rows}\n`);
console.log(`[DEBUG] Transferred and hash-verified ${manifest.length} canonical teaching files.`);
