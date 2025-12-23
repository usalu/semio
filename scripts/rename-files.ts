// #region Header

// scripts/rename-files.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

import { readdirSync, renameSync, statSync } from "fs";
import { join, relative, dirname, basename } from "path";
import { createInterface } from "readline";

// ANSI color codes
const RED = "\x1B[31m";
const GREEN = "\x1B[32m";
const STRIKETHROUGH = "\x1B[9m";
const RESET = "\x1B[0m";
const INVERSE = "\x1B[7m";
const DIM = "\x1B[2m";

interface FileMatch {
  path: string;
  newPath: string;
  selected: boolean;
  oldFileName: string;
  newFileName: string;
  dirPath: string;
}

function findFiles(pattern: string, replacement: string, rootDir: string, ignoreDirs: string[]): FileMatch[] {
  const matches: FileMatch[] = [];
  const regex = new RegExp(pattern, "g");

  function walk(dir: string): void {
    const files = readdirSync(dir);
    for (const file of files) {
      const filePath = join(dir, file);
      const stat = statSync(filePath);
      if (stat.isDirectory()) {
        if (!ignoreDirs.some((ignored) => file === ignored || filePath.includes(ignored))) {
          walk(filePath);
        }
      } else {
        // Reset regex lastIndex for each file
        regex.lastIndex = 0;
        if (regex.test(file)) {
          regex.lastIndex = 0;
          const newFileName = file.replace(regex, replacement);
          const newPath = join(dirname(filePath), newFileName);
          matches.push({
            path: filePath,
            newPath,
            selected: true,
            oldFileName: file,
            newFileName,
            dirPath: relative(rootDir, dirname(filePath)),
          });
        }
      }
    }
  }

  walk(rootDir);
  return matches;
}

function clearScreen(): void {
  process.stdout.write("\x1B[2J\x1B[0f");
}

function formatRename(oldName: string, newName: string, pattern: string, replacement: string): string {
  // Find what changed between old and new name
  const regex = new RegExp(pattern, "g");
  let result = "";
  let lastIndex = 0;

  // Reset and find all matches
  regex.lastIndex = 0;
  let match;
  while ((match = regex.exec(oldName)) !== null) {
    // Add unchanged part before match
    result += oldName.slice(lastIndex, match.index);
    // Add old part as red strikethrough, new part as green
    result += `${RED}${STRIKETHROUGH}${match[0]}${RESET}${GREEN}${replacement}${RESET}`;
    lastIndex = match.index + match[0].length;
  }
  // Add remaining unchanged part
  result += oldName.slice(lastIndex);

  return result;
}

function renderList(matches: FileMatch[], cursor: number, pattern: string, replacement: string, pageSize: number = 20): void {
  clearScreen();
  console.log("\x1B[1mRename Files Interactive\x1B[0m");
  console.log("─".repeat(80));
  console.log("Controls: ↑/↓ navigate | Space toggle | a select all | n select none | Enter confirm | q quit");
  console.log("─".repeat(80));

  const selectedCount = matches.filter((m) => m.selected).length;
  console.log(`Selected: ${selectedCount}/${matches.length} files\n`);

  // Calculate page window
  const halfPage = Math.floor(pageSize / 2);
  let startIdx = Math.max(0, cursor - halfPage);
  let endIdx = Math.min(matches.length, startIdx + pageSize);
  if (endIdx - startIdx < pageSize) {
    startIdx = Math.max(0, endIdx - pageSize);
  }

  if (startIdx > 0) {
    console.log(`${DIM}  ... ${startIdx} more above${RESET}`);
  }

  for (let i = startIdx; i < endIdx; i++) {
    const match = matches[i];
    const isCursor = i === cursor;
    const checkbox = match.selected ? "✓" : " ";
    const checkboxColor = match.selected ? GREEN : DIM;

    const formattedName = formatRename(match.oldFileName, match.newFileName, pattern, replacement);
    const dirPrefix = match.dirPath ? `${DIM}${match.dirPath}/${RESET}` : "";

    const line = `${checkboxColor}[${checkbox}]${RESET} ${dirPrefix}${formattedName}`;

    if (isCursor) {
      console.log(`${INVERSE}→${RESET} ${line}`);
    } else {
      console.log(`  ${line}`);
    }
  }

  if (endIdx < matches.length) {
    console.log(`${DIM}  ... ${matches.length - endIdx} more below${RESET}`);
  }

  console.log();
  console.log("─".repeat(80));
}

async function interactiveSelect(matches: FileMatch[], pattern: string, replacement: string): Promise<FileMatch[]> {
  return new Promise((resolve) => {
    if (matches.length === 0) {
      console.log("No files found matching the pattern.");
      resolve([]);
      return;
    }

    let cursor = 0;

    const rl = createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    if (process.stdin.isTTY) {
      process.stdin.setRawMode(true);
    }
    process.stdin.resume();

    renderList(matches, cursor, pattern, replacement);

    process.stdin.on("data", (key: Buffer) => {
      const keyStr = key.toString();

      if (keyStr === "\u0003" || keyStr === "q" || keyStr === "Q") {
        // Ctrl+C or q
        if (process.stdin.isTTY) {
          process.stdin.setRawMode(false);
        }
        rl.close();
        console.log("\nCancelled.");
        process.exit(0);
      } else if (keyStr === "\u001B[A") {
        // Up arrow
        cursor = Math.max(0, cursor - 1);
        renderList(matches, cursor, pattern, replacement);
      } else if (keyStr === "\u001B[B") {
        // Down arrow
        cursor = Math.min(matches.length - 1, cursor + 1);
        renderList(matches, cursor, pattern, replacement);
      } else if (keyStr === " ") {
        // Space - toggle selection
        matches[cursor].selected = !matches[cursor].selected;
        renderList(matches, cursor, pattern, replacement);
      } else if (keyStr === "\r" || keyStr === "\n") {
        // Enter - confirm
        if (process.stdin.isTTY) {
          process.stdin.setRawMode(false);
        }
        rl.close();
        resolve(matches.filter((m) => m.selected));
      } else if (keyStr === "a" || keyStr === "A") {
        // Select all
        matches.forEach((m) => (m.selected = true));
        renderList(matches, cursor, pattern, replacement);
      } else if (keyStr === "n" || keyStr === "N") {
        // Select none
        matches.forEach((m) => (m.selected = false));
        renderList(matches, cursor, pattern, replacement);
      }
    });
  });
}

function renameFiles(files: FileMatch[]): void {
  console.log("\nRenaming files...\n");
  for (const file of files) {
    try {
      renameSync(file.path, file.newPath);
      console.log(`✓ ${file.path} → ${file.newPath}`);
    } catch (error) {
      console.error(`✗ Failed to rename ${file.path}: ${error}`);
    }
  }
  console.log(`\nRenamed ${files.length} files.`);
}

async function main(): Promise<void> {
  const args = process.argv.slice(2);

  if (args.length < 2 || args.includes("--help") || args.includes("-h")) {
    console.log(`
Usage: npx tsx scripts/rename-files.ts <pattern> <replacement> [rootDir] [--ignore dir1,dir2]

Arguments:
  pattern      - Regex pattern to search for in file names
  replacement  - String to replace matches with
  rootDir      - Root directory to search (default: current directory)
  --ignore     - Comma-separated list of directories to ignore

Examples:
  npx tsx scripts/rename-files.ts "port" "connector" .
  npx tsx scripts/rename-files.ts "port" "connector" ./src --ignore node_modules,dist

  # Use lookbehind/lookahead to exclude certain matches:
  # Match "port" but not in "export", "import", "report", "portion", "portal", "portable"
  npx tsx scripts/rename-files.ts "(?<![xXpPeEmMsSoOwWrRnN-])port(?!ion)(?!al)(?!able)" "connector" .

Controls:
  ↑/↓    - Navigate through files
  Space  - Toggle file selection
  a      - Select all
  n      - Select none
  Enter  - Confirm and rename selected files
  q      - Quit without renaming

Display:
  ${RED}${STRIKETHROUGH}old${RESET}${GREEN}new${RESET} - Red strikethrough shows removed text, green shows new text
`);
    process.exit(0);
  }

  const pattern = args[0];
  const replacement = args[1];
  const rootDir = args[2] && !args[2].startsWith("--") ? args[2] : ".";

  let ignoreDirs = ["node_modules", ".git", "dist", "build", "__pycache__", ".venv", "bin", "obj"];
  const ignoreIndex = args.indexOf("--ignore");
  if (ignoreIndex !== -1 && args[ignoreIndex + 1]) {
    ignoreDirs = args[ignoreIndex + 1].split(",");
  }

  // Validate regex pattern
  try {
    new RegExp(pattern);
  } catch (e) {
    console.error(`Invalid regex pattern: ${pattern}`);
    console.error((e as Error).message);
    process.exit(1);
  }

  console.log(`Searching for files matching "${pattern}" to replace with "${replacement}"...`);
  console.log(`Root directory: ${rootDir}`);
  console.log(`Ignoring: ${ignoreDirs.join(", ")}\n`);

  const matches = findFiles(pattern, replacement, rootDir, ignoreDirs);

  if (matches.length === 0) {
    console.log("No files found matching the pattern.");
    process.exit(0);
  }

  console.log(`Found ${matches.length} files.\n`);

  const selectedFiles = await interactiveSelect(matches, pattern, replacement);

  if (selectedFiles.length === 0) {
    console.log("\nNo files selected. Exiting.");
    process.exit(0);
  }

  renameFiles(selectedFiles);
}

main().catch(console.error);
