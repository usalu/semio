import { execSync } from "child_process";
import * as fs from "fs";
import * as path from "path";

const EXCLUDE_DIRS = new Set([".git", "node_modules", "target", ".nx", "dist", "temp", "storybook-static"]);

function walk(dir: string, callback: (file: string) => void) {
  const files = fs.readdirSync(dir);
  for (const file of files) {
    const fullPath = path.join(dir, file);
    const stat = fs.statSync(fullPath);
    if (stat.isDirectory()) {
      if (EXCLUDE_DIRS.has(file)) continue;
      walk(fullPath, callback);
      callback(fullPath);
    } else {
      callback(fullPath);
    }
  }
}

const matches: string[] = [];
walk(".", (file) => {
  const base = path.basename(file);
  if (base.toLowerCase().includes("compose")) {
    matches.push(file);
  }
});

// Sort by depth descending so we rename files/subfolders before their parent folders
matches.sort((a, b) => b.split(path.sep).length - a.split(path.sep).length);

console.log(`Found ${matches.length} matches to rename.`);

for (const match of matches) {
  if (!fs.existsSync(match)) {
    console.log(`Skipping non-existent: ${match}`);
    continue;
  }
  const dir = path.dirname(match);
  const base = path.basename(match);
  let newBase = base
    .replace(/compose/g, "compose")
    .replace(/Compose/g, "Compose")
    .replace(/COMPOSE/g, "COMPOSE");

  if (newBase === base) continue;

  const newPath = path.join(dir, newBase);
  console.log(`Renaming: ${match} -> ${newPath}`);
  try {
    execSync(`git mv "${match}" "${newPath}"`);
  } catch (e) {
    console.log(`git mv failed, trying fs.renameSync: ${e}`);
    try {
      fs.renameSync(match, newPath);
    } catch (fsErr) {
      console.error(`fs.renameSync also failed: ${fsErr}`);
    }
  }
}
