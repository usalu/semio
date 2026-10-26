import { createHash } from "node:crypto";
import { execFile } from "node:child_process";
import { readdir, readFile, stat, writeFile } from "node:fs/promises";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
const runTool = promisify(execFile);
const ticket = dirname(fileURLToPath(import.meta.url));
const sourceRoot = "C:/git/semio.worktrees/🦢niloufar/⛳wip";
const source = join(sourceRoot, "tutorial");
const destinationRoot = "C:/git/semio";
const destination = join(destinationRoot, "🎓️teaching");
const excluded = new Set([".agents", "__pycache__", "media", "rendered", ".venv"]);
const canonical: string[] = [];
async function walk(directory: string) {
    for (const entry of await readdir(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory()) { if (!excluded.has(entry.name)) await walk(path); }
        else if (entry.isFile()) canonical.push(relative(source, path).split(sep).join("/"));
    }
}
await walk(source);
canonical.sort();
if (canonical.length !== 131) throw new Error("Canonical source count differs: " + canonical.length);
for (const path of canonical) if (!(await stat(join(destination, path))).isFile()) throw new Error("Missing transferred file: " + path);
const unchanged = canonical.filter(path => /(?:^|\/)scene_\d+\.py$/.test(path) || path === "intro/intro_scene.py" || path === "manim_visuals.py" || /\.pdf$/i.test(path));
const rows: string[] = [];
const sha256 = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
for (const path of unchanged) {
    const original = sha256(await readFile(join(source, path)));
    const copied = sha256(await readFile(join(destination, path)));
    if (original !== copied) throw new Error("Authored asset changed: " + path);
    rows.push("| " + path + " | " + original + " |");
}
const tracked = (await runTool("git", ["-C", sourceRoot, "ls-files", "-z"], { encoding: "utf8", maxBuffer: 32 * 1024 * 1024 })).stdout.split("\0").filter(Boolean);
const other = tracked.filter(path => !path.startsWith("tutorial/") && /(?:manim|(?:^|\/)(?:teaching|tutorial|lectures?|lessons?|videos?)(?:\/|[_.-])|(?:full|build)[_-].*video|intro_scene)/i.test(path));
let manimSources: string[] = [];
try {
    manimSources = (await runTool("git", ["-C", sourceRoot, "grep", "-I", "-l", "-E", "from manim import|import manim|manim\\.Scene", "--", "*.py", ":!tutorial/**"], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 })).stdout.trim().split("\n").filter(Boolean);
} catch (error) { if ((error as { code?: number }).code !== 1) throw error; }
const videoNamedPaths = tracked.filter(path => !path.startsWith("tutorial/") && /(?:scene_\d+\.py$|\.(?:mp4|mov|webm)$)/i.test(path));
const staged = (await runTool("git", ["-C", destinationRoot, "diff", "--cached", "--name-only", "-z", "--", "🎓️teaching"], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 })).stdout.split("\0").filter(Boolean);
const stagedGenerated = staged.filter(path => /(?:^|\/)(?:media|rendered|__pycache__|\.venv|🗑️generated)(?:\/|$)|\.(?:mp4|mp3|wav|pyc)$/i.test(path));
await writeFile(join(ticket, "🗑️generated/source-tracked-paths.json"), JSON.stringify({ trackedCount: tracked.length, candidatesOutsideTutorial: other, stagedTeachingCount: staged.length, stagedGenerated, manimSourcesOutsideTutorial: manimSources, videoNamedPathsOutsideTutorial: videoNamedPaths }, null, 2));
await writeFile(join(ticket, "source-integrity.md"), "# Authored Source Integrity\n\nAll " + canonical.length + " canonical tutorial files exist in the teaching destination. SHA-256 matches the source for every requested immutable authored scene/visual/PDF asset: " + unchanged.length + " files. The source chapter scene count is " + unchanged.filter(path => /(?:^|\/)scene_\d+\.py$/.test(path)).length + ". Source names are retained exactly.\n\nAgent configuration and generated directories (.agents, __pycache__, media, rendered, .venv) are excluded from the canonical transfer. At this audit snapshot, " + staged.length + " teaching paths are staged and " + stagedGenerated.length + " staged teaching paths match generated-media/cache patterns. Parent coordination owns the final index check.\n\n## Unchanged Authored Assets\n\n| Relative Path | SHA-256 |\n| --- | --- |\n" + rows.join("\n") + "\n\n## Tracked Source Scope\n\nRead-only git ls-files inspected " + tracked.length + " tracked source-worktree paths. " + other.length + " path names outside tutorial match teaching/video-related scope terms. They are listed below for inspection; a matching term alone does not establish a missing authored curriculum.\n\n" + other.map(path => "- " + path).join("\n") + "\n\nA content-name-only Git grep for Manim imports outside tutorial found " + manimSources.length + " tracked Python files.\n\n" + manimSources.map(path => "- " + path).join("\n") + "\n\nTracked scene_N.py or video-container filenames outside tutorial: " + videoNamedPaths.length + ".\n\n" + videoNamedPaths.map(path => "- " + path).join("\n") + "\n");
console.log("[DEBUG] Verified", canonical.length, "transferred files and", unchanged.length, "unchanged authored assets; scope candidates:", other.length);
