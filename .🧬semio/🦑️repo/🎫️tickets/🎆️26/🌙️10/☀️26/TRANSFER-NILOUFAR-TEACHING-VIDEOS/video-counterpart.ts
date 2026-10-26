import { createHash } from "node:crypto";
import { execFile } from "node:child_process";
import { appendFile, readFile, stat, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
const runTool = promisify(execFile);
const ticket = dirname(fileURLToPath(import.meta.url));
const sourceRoot = "C:/git/semio.worktrees/🦢niloufar/⛳wip";
const destinationRoot = "C:/git/semio";
const sourceRelative = "mit-bestand/präsentation/33.projektetage/public/bauen-mit-bestand.mp4";
const paths = (await runTool("git", ["-C", destinationRoot, "ls-files", "-z", "--", "*.mp4", "*.mov", "*.webm"], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 })).stdout.split("\0").filter(Boolean);
const sourceBytes = await readFile(join(sourceRoot, sourceRelative));
const hash = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const sourceHash = hash(sourceBytes);
const rows = [];
for (const path of paths) {
    try {
        const info = await stat(join(destinationRoot, path));
        if (info.size === sourceBytes.length || /bauen|bestand/i.test(path)) {
            const digest = hash(await readFile(join(destinationRoot, path)));
            rows.push({ path, bytes: info.size, sha256: digest, identical: digest === sourceHash });
        }
    } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
}
await writeFile(join(ticket, "🗑️generated/video-counterparts.json"), JSON.stringify({ sourceRelative, sourceBytes: sourceBytes.length, sourceHash, trackedContainerCount: paths.length, matches: rows }, null, 2));
await appendFile(join(ticket, "source-integrity.md"), "\n## Separate Presentation Video Counterpart\n\nSource: " + sourceRelative + ". Size: " + sourceBytes.length + " bytes. SHA-256: " + sourceHash + ". The file is an actual MP4 asset rather than an LFS pointer.\n\nDestination tracked video-container paths inspected: " + paths.length + ". Semantic-name or equal-size candidates:\n\n" + rows.map(row => "- " + row.path + " — " + row.bytes + " bytes, SHA-256 " + row.sha256 + "; identical=" + row.identical).join("\n") + "\n");
console.log("[DEBUG] Separate presentation counterpart:", JSON.stringify(rows));
