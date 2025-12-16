#!/usr/bin/env npx tsx
import { readFileSync, readdirSync, writeFileSync } from "fs";
import { join } from "path";
import { MetabolismKit } from "../assets/index";
import { exportKit, importKit } from "../js/js/semio";


const INCLUDE_FOLDERS = ["representations", "icons", "images"];


function collectFiles(dir: string, basePath: string = ""): Map<string, Blob> {
    const files = new Map<string, Blob>();
    const entries = readdirSync(dir, { withFileTypes: true });

    for (const entry of entries) {
        const fullPath = join(dir, entry.name);
        const relativePath = basePath ? `${basePath}/${entry.name}` : entry.name;

        if (entry.isDirectory()) {
            // Skip .semio, .git, and other non-included directories at root level
            if (entry.name === ".semio" || entry.name === ".git") continue;
            // At root level, only process included folders
            if (!basePath && !INCLUDE_FOLDERS.includes(entry.name)) continue;
            // Recursively collect files from subdirectories
            const subFiles = collectFiles(fullPath, relativePath);
            Array.from(subFiles.entries()).forEach(([path, blob]) => {
                files.set(path, blob);
            });
        } else {
            // Skip files at root level (only include files in subfolders)
            if (!basePath) continue;
            // Read file and create blob
            const buffer = readFileSync(fullPath);
            const blob = new Blob([buffer]);
            files.set(relativePath, blob);
        }
    }

    return files;
}

async function main() {
    console.log("Regenerating metabolism.zip...");

    const kit = MetabolismKit;

    // Collect files from examples/metabolism directory
    const metabolismDir = join(__dirname, "..", "examples", "metabolism");
    const files = collectFiles(metabolismDir);

    console.log(`Found ${files.size} files to include:`);
    Array.from(files.keys()).slice(0, 10).forEach((path) => {
        console.log(`  - ${path}`);
    });
    if (files.size > 10) {
        console.log(`  ... and ${files.size - 10} more`);
    }

    // Log info about the kit
    const tambourBefore = kit.types?.find(t => t.name === "Tambour");
    console.log("Tambour models in source:", tambourBefore?.models?.length ?? 0);

    const zipBlob = await exportKit(kit, files);
    const buffer = Buffer.from(await zipBlob.arrayBuffer());

    const outputPath = join(__dirname, "..", "assets", "semio", "metabolism.zip");
    writeFileSync(outputPath, buffer);

    console.log("Exported to:", outputPath);
    console.log("Size:", (buffer.length / 1024).toFixed(2), "KB");

    // Verify models in the exported kit
    const { kit: imported, files: importedFiles } = await importKit(buffer);
    const tambourAfter = imported.types?.find(t => t.name === "Tambour");
    console.log("Tambour models after import:", tambourAfter?.models?.length ?? 0);
    console.log("Files in imported zip:", importedFiles.size);
}

main().catch(console.error);
