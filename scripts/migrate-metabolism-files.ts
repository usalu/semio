import { readFileSync, writeFileSync } from "fs";
import { basename, dirname, extname, join } from "path";
import { guid } from "../js/js/semio";

const kitPath = join(__dirname, "..", "assets", "semio", "kit_metabolism.json");
const kit = JSON.parse(readFileSync(kitPath, "utf-8"));

// MIME type mapping based on file extension
const EXTENSION_TO_MIME: Record<string, string> = {
    // 3D model formats
    ".glb": "model/gltf-binary",
    ".gltf": "model/gltf+json",
    ".fbx": "model/vnd.autodesk.fbx",
    ".obj": "model/obj",
    ".3dm": "model/vnd.rhino3dm",
    ".3ds": "model/vnd.3ds",
    ".stl": "model/stl",
    ".ply": "model/ply",
    ".usdz": "model/vnd.usdz+zip",
    ".vrm": "model/gltf-binary",
    ".ifc": "model/ifc",
    ".3mf": "model/3mf",
    ".dae": "model/vnd.collada+xml",
    // Image formats
    ".png": "image/png",
    ".jpg": "image/jpeg",
    ".jpeg": "image/jpeg",
    ".gif": "image/gif",
    ".svg": "image/svg+xml",
    ".webp": "image/webp",
    ".bmp": "image/bmp",
    ".ico": "image/x-icon",
    // Document formats
    ".pdf": "application/pdf",
    ".json": "application/json",
    ".xml": "application/xml",
    ".html": "text/html",
    ".txt": "text/plain",
    ".md": "text/markdown",
};

function getMimeType(filename: string): string | undefined {
    const ext = extname(filename).toLowerCase();
    return EXTENSION_TO_MIME[ext];
}

function isRemoteUrl(path: string): boolean {
    return path.startsWith("http://") || path.startsWith("https://") || path.startsWith("ftp://");
}

// Create folders from unique folder paths
const folderPaths = new Set<string>();
const files = kit.files || [];

// Collect all unique folder paths from files
for (const file of files) {
    const filePath = file.name;
    if (!isRemoteUrl(filePath) && filePath.includes("/")) {
        const folderPath = dirname(filePath);
        // Add folder and all parent folders
        const parts = folderPath.split("/");
        for (let i = 1; i <= parts.length; i++) {
            folderPaths.add(parts.slice(0, i).join("/"));
        }
    }
}

// Create folder objects with GUIDs
const folderMap = new Map<string, { guid: string; name: string; parent?: { guid: string } }>();
const folders: Array<{ guid: string; name: string; parent?: { guid: string }; createdAt: string; updatedAt: string }> = [];

const now = new Date().toISOString();

// Sort folder paths to ensure parents are created before children
const sortedFolderPaths = Array.from(folderPaths).sort((a, b) => {
    const aDepth = a.split("/").length;
    const bDepth = b.split("/").length;
    return aDepth - bDepth;
});

for (const folderPath of sortedFolderPaths) {
    const parts = folderPath.split("/");
    const folderName = parts[parts.length - 1];
    const parentPath = parts.length > 1 ? parts.slice(0, -1).join("/") : undefined;

    const folder: { guid: string; name: string; parent?: { guid: string }; createdAt: string; updatedAt: string } = {
        guid: guid(),
        name: folderName,
        createdAt: now,
        updatedAt: now,
    };

    if (parentPath) {
        const parentFolder = folderMap.get(parentPath);
        if (parentFolder) {
            folder.parent = { guid: parentFolder.guid };
        }
    }

    folderMap.set(folderPath, folder);
    folders.push(folder);
}

// Migrate files
let filesUpdated = 0;
let remotesAdded = 0;
let mimesAdded = 0;

for (const file of files) {
    const originalName = file.name;

    if (isRemoteUrl(originalName)) {
        // Handle remote URLs
        // Keep the URL in remote field, derive a name
        file.remote = originalName;
        // Extract a reasonable name from the URL
        const urlParts = originalName.split("/");
        const lastPart = urlParts[urlParts.length - 1];
        // For Speckle URLs like https://app.speckle.systems/projects/e7de1a2f8f/models/e5267da44d
        // Use the model ID as name
        file.name = lastPart || urlParts[urlParts.length - 2] || "remote";
        remotesAdded++;
        filesUpdated++;
    } else if (originalName.includes("/")) {
        // Handle local file paths
        const fileName = basename(originalName);
        const folderPath = dirname(originalName);
        const folder = folderMap.get(folderPath);

        file.name = fileName;
        if (folder) {
            file.folder = { guid: folder.guid };
        }

        // Add MIME type
        const mime = getMimeType(fileName);
        if (mime) {
            file.mime = mime;
            mimesAdded++;
        }
        filesUpdated++;
    } else {
        // File is already just a name, add MIME type if possible
        const mime = getMimeType(originalName);
        if (mime) {
            file.mime = mime;
            mimesAdded++;
            filesUpdated++;
        }
    }
}

// Add folders to kit
kit.folders = folders;

// Write updated kit
writeFileSync(kitPath, JSON.stringify(kit, null, 2));

console.log(`Migration completed:`);
console.log(`  Created ${folders.length} folders`);
console.log(`  Updated ${filesUpdated} files`);
console.log(`  Added ${mimesAdded} MIME types`);
console.log(`  Converted ${remotesAdded} URLs to remote references`);
console.log(`\nFolders created:`);
for (const folder of folders) {
    const parentInfo = folder.parent ? ` (parent: ${folders.find(f => f.guid === folder.parent?.guid)?.name})` : "";
    console.log(`  ${folder.name}${parentInfo} - ${folder.guid}`);
}
