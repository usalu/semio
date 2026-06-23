import * as fs from "fs";
import * as path from "path";

const EXCLUDE_DIRS = new Set([
  ".git",
  "node_modules",
  ".repo",
  "dist",
  "target",
  "storybook-static",
  ".repo-cache",
  ".venv"
]);

const BUNDLES = [
  "client", "dev", "server", "site", "fixture", "asset",
  "algorithms", "antlr", "assets", "desktop", "docs", "engine", "examples", "gh", "go",
  "graphql", "js", "jsonschema", "liveblocks", "net", "openapi", "peg", "play", "py",
  "rb", "rdf", "reports", "rs", "sites", "sketchpad", "sqlite", "studio", "ui", "vscode"
];

function walk(dir: string, callback: (file: string) => void) {
  const files = fs.readdirSync(dir);
  for (const file of files) {
    const fullPath = path.join(dir, file);
    const stat = fs.statSync(fullPath);
    if (stat.isDirectory()) {
      if (EXCLUDE_DIRS.has(file)) continue;
      walk(fullPath, callback);
    } else {
      callback(fullPath);
    }
  }
}

function runTechRename() {
  console.log("Starting technology rename for missed files...");

  let modifiedCount = 0;

  walk(".", (file) => {
    // Ignore binary/image files
    if (
      file.endsWith(".png") ||
      file.endsWith(".ico") ||
      file.endsWith(".zip") ||
      file.endsWith(".jpg") ||
      file.endsWith(".3dm") ||
      file.endsWith(".gh") ||
      file.endsWith(".wasm")
    ) {
      return;
    }

    try {
      const content = fs.readFileSync(file, "utf8");
      let newContent = content;

      // 1. Replace `@compose/` package scopes
      newContent = newContent.replace(/@compose\//g, "@compose/");

      // 2. Replace bundle paths `compose/<bundle>` and `compose\<bundle>`
      for (const bundle of BUNDLES) {
        const slashRegex = new RegExp(`compose/${bundle}`, "g");
        newContent = newContent.replace(slashRegex, `compose/${bundle}`);

        const backslashRegex = new RegExp(`compose\\\\${bundle}`, "g");
        newContent = newContent.replace(backslashRegex, `compose\\${bundle}`);
      }

      // 3. Replace identifiers
      newContent = newContent.replace(/compose-sketchpad/g, "compose-sketchpad");
      newContent = newContent.replace(/compose-react/g, "compose-react");
      newContent = newContent.replace(/compose-ui/g, "compose-ui");
      newContent = newContent.replace(/compose\.sketchpad/g, "compose.sketchpad");

      if (newContent !== content) {
        console.log(`Renaming tech references in: ${file}`);
        fs.writeFileSync(file, newContent, "utf8");
        modifiedCount++;
      }
    } catch (e) {
      // Ignore read errors
    }
  });

  console.log(`Tech rename completed! Modified ${modifiedCount} files.`);
}

runTechRename();
