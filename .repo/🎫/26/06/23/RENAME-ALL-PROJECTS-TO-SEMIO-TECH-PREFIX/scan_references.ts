import { readFileSync } from "fs";
import { execSync } from "child_process";

const mappings = JSON.parse(readFileSync(".repo/🎫/26/06/23/RENAME-ALL-PROJECTS-TO-SEMIO-TECH-PREFIX/mappings.json", "utf8"));
const oldNames = Object.keys(mappings);

// Get tracked files from git
const files = execSync("git ls-files", { encoding: "utf8" })
  .split("\n")
  .map(f => f.trim())
  .filter(f => f && !f.includes("mappings.json") && !f.includes("scan_references.ts") && !f.includes("find_and_map.ts"));

const filesWithReferences: Record<string, string[]> = {};

for (const file of files) {
  try {
    const content = readFileSync(file, "utf8");
    const matched: string[] = [];
    for (const oldName of oldNames) {
      if (content.includes(oldName)) {
        matched.push(oldName);
      }
    }
    if (matched.length > 0) {
      filesWithReferences[file] = matched;
    }
  } catch (e) {
    // skip binary/unreadable files
  }
}

console.log(JSON.stringify(filesWithReferences, null, 2));
