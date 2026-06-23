import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const glob = new Bun.Glob("**/package.json");
const mappings: Record<string, { path: string, newName: string }> = {};

for (const file of glob.scanSync({ cwd: ".", onlyFiles: true })) {
  if (
    file.includes("node_modules") ||
    file.includes(".nx") ||
    file.includes(".venv") ||
    file.includes("dist") ||
    file.includes("temp")
  ) {
    continue;
  }
  if (file === "package.json") continue; // skip root package.json
  
  try {
    const content = JSON.parse(readFileSync(file, "utf8"));
    const oldName = content.name;
    if (!oldName) continue;
    
    let newName = oldName;
    if (oldName === "repo" && file.includes("vscode")) {
      newName = "@semio-tech/repo-vscode";
    } else if (oldName.startsWith("@")) {
      // e.g. @compose/play -> @semio-tech/compose-sketchpad-play
      if (oldName === "@compose/play") {
        newName = "@semio-tech/compose-sketchpad-play";
      } else if (oldName === "@compose/docs") {
        newName = "@semio-tech/compose-sketchpad-docs";
      } else if (oldName === "@compose/sketchpad") {
        newName = "@semio-tech/compose-sketchpad";
      } else {
        // General rule: convert @scope/name/subname -> @semio-tech/scope-name-subname
        const withoutAt = oldName.slice(1);
        const parts = withoutAt.split("/");
        newName = `@semio-tech/${parts.join("-")}`;
      }
    }
    
    mappings[oldName] = { path: file, newName };
  } catch (e) {
    console.error("Error reading", file, e);
  }
}

console.log(JSON.stringify(mappings, null, 2));
