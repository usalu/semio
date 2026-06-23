import * as fs from "fs";
import * as path from "path";

const EXCLUDE_DIRS = new Set([
  ".git",
  "node_modules",
  "target",
  ".nx",
  "dist",
  "temp",
  "storybook-static",
  "bin"
]);

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

function runRenameAndBrand() {
  console.log("Starting global content rename and branding correction...");
  let modifiedCount = 0;

  walk(".", (file) => {
    // Ignore binary/image/model files
    if (
      file.endsWith(".png") ||
      file.endsWith(".ico") ||
      file.endsWith(".zip") ||
      file.endsWith(".jpg") ||
      file.endsWith(".3dm") ||
      file.endsWith(".gh") ||
      file.endsWith(".wasm") ||
      file.endsWith(".glb")
    ) {
      return;
    }

    try {
      const content = fs.readFileSync(file, "utf8");
      let newContent = content;

      // 1. Global replacements of technology name
      newContent = newContent
        .replace(/compose/g, "compose")
        .replace(/Compose/g, "Compose")
        .replace(/COMPOSE/g, "COMPOSE");

      // 2. Restore repository and Go module roots
      newContent = newContent.replace(/github\.com\/usalu\/compose/g, "github.com/usalu/semio");
      newContent = newContent.replace(/github\.com\/usalu\/Compose/g, "github.com/usalu/semio");

      // 3. Restore email addresses
      newContent = newContent.replace(/ueli@compose-tech\.com/g, "ueli@semio-tech.com");
      newContent = newContent.replace(/compose-tech\.com/g, "semio-tech.com");

      // 4. Restore playground title prefix
      newContent = newContent.replace(/semio ·/g, "semio ·");
      newContent = newContent.replace(/Semio ·/g, "Semio ·");

      // 5. Restore devcontainer paths and workspace folders
      newContent = newContent.replace(/\/workspaces\/compose\/compose/g, "/workspaces/semio/compose");
      newContent = newContent.replace(/\/workspaces\/compose/g, "/workspaces/semio");

      // 6. Restore specific docker image repository
      newContent = newContent.replace(/image: compose\/compose-hub/g, "image: semio/compose-hub");

      // 7. Restore COMPOSE environment variables
      newContent = newContent.replace(/SEMIO_GITKRAKEN/g, "SEMIO_GITKRAKEN");
      newContent = newContent.replace(/SEMIO_F3D/g, "SEMIO_F3D");
      newContent = newContent.replace(/SEMIO_POST_ATTACH/g, "SEMIO_POST_ATTACH");

      if (newContent !== content) {
        console.log(`Updating contents in: ${file}`);
        fs.writeFileSync(file, newContent, "utf8");
        modifiedCount++;
      }
    } catch (e) {
      // Ignore read errors
    }
  });

  console.log(`Global content rename and branding completed! Modified ${modifiedCount} files.`);
}

runRenameAndBrand();
