import { readFileSync, writeFileSync } from "fs";
import { execSync } from "child_process";

const mappings = JSON.parse(readFileSync(".repo/🎫/26/06/23/RENAME-ALL-PROJECTS-TO-SEMIO-TECH-PREFIX/mappings.json", "utf8"));

// Sort old names by length descending to prevent partial matches
const oldNames = Object.keys(mappings).sort((a, b) => b.length - a.length);

const files = execSync("git ls-files", { encoding: "utf8" })
  .split("\n")
  .map((f) => f.trim())
  .filter((f) => {
    return f && !f.includes("mappings.json") && !f.includes("scan_references.json") && !f.includes("references.json") && !f.includes("find_and_map.ts") && !f.includes("scan_references.ts") && !f.includes("dry_run_rename.ts");
  });

const diffs: Record<string, string> = {};

for (const file of files) {
  // Only process source, config, and documentation files
  const ext = file.split(".").pop();
  if (!ext || !["ts", "tsx", "js", "jsx", "mjs", "cjs", "json", "md", "toml", "yaml", "yml"].includes(ext)) {
    continue;
  }

  try {
    const originalContent = readFileSync(file, "utf8");
    let content = originalContent;

    // 1. Replace scoped package names
    for (const oldName of oldNames) {
      if (oldName === "repo") continue; // Handle "repo" separately to avoid destroying text

      // We want to replace the exact package name.
      // Since scoped package names start with '@' and are unique, we can safely replace them.
      // We also replace references in imports, nx tasks, etc.
      // Let's replace all occurrences.
      content = content.replaceAll(oldName, mappings[oldName].newName);
    }

    // 2. Handle "repo" package renaming specifically
    if (file === "repo/client/vscode/package.json") {
      content = content.replace('"name": "repo"', '"name": "@semio-tech/repo-vscode"');
    }

    // In launch.json, replace command lines containing "repo" project name
    if (file === ".vscode/launch.json") {
      content = content.replaceAll("nx dev repo", "nx dev @semio-tech/repo-vscode");
      content = content.replaceAll("run repo:build", "run @semio-tech/repo-vscode:build");
    }

    // In script.ts, replace "repo:build"
    if (file === "script.ts") {
      content = content.replaceAll("repo:build", "@semio-tech/repo-vscode:build");
      content = content.replaceAll("repo:build-vsix", "@semio-tech/repo-vscode:build-vsix");
      content = content.replaceAll('"repo-vscode": "repo:build-vsix"', '"repo-vscode": "@semio-tech/repo-vscode:build-vsix"');
    }

    if (content !== originalContent) {
      diffs[file] = content;
    }
  } catch (e) {
    // skip binary or unreadable
  }
}

// Write the diff summary to a file for analysis
let diffOutput = "";
for (const [file, content] of Object.entries(diffs)) {
  diffOutput += `File: ${file}\n`;
  // Simple representation of changes (first 100 chars or just count of matches)
  diffOutput += `  Changed. Length: ${content.length}\n\n`;
}

console.log(`Found changes in ${Object.keys(diffs).length} files.`);
writeFileSync(".repo/🎫/26/06/23/RENAME-ALL-PROJECTS-TO-SEMIO-TECH-PREFIX/dry_run_diff_summary.txt", diffOutput);

// Also write a script to apply the changes when approved
const applyScript = `import { readFileSync, writeFileSync } from "fs";
const diffs = ${JSON.stringify(diffs, null, 2)};
for (const [file, content] of Object.entries(diffs)) {
  writeFileSync(file, content, "utf8");
  console.log("Updated", file);
}
`;
writeFileSync(".repo/🎫/26/06/23/RENAME-ALL-PROJECTS-TO-SEMIO-TECH-PREFIX/apply_changes.ts", applyScript);
console.log("Created apply_changes.ts in the ticket folder.");
