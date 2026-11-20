import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd();
const { readRustSourceClosureV1 } = await import(pathToFileURL(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const targets = process.argv.slice(2);
const cfg = process.platform === "win32" ? ["windows"] : ["unix"];
if (targets[0] === "--test") {
    targets.shift();
    cfg.push("test");
}

async function read(file: string) {
    try {
        return await readFile(file);
    } catch (error) {
        if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
        throw error;
    }
}

for (const target of targets) {
    const exact = target.endsWith(".rs");
    const entry = resolve(root, target, ...(exact ? [] : ["🦀️.rs"]));
    const manifestDirectory = resolve(root, exact ? dirname(target) : target, "📦️packages/🦀️rust");
    const includes = await readRustSourceClosureV1([entry], root, cfg, read, { manifestDirectory, generated: () => undefined });
    const activeLines = includes.reduce((total: number, file: { activeLines: number }) => total + file.activeLines, 0);
    const bytes = includes.reduce((total: number, file: { bytes: number }) => total + file.bytes, 0);
    console.log("[DEBUG] Rust source admission", { target, files: includes.length, activeLines, bytes });
}
