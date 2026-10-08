/**
 * 🏷️ Renames one file or folder of the BIM plugin (one emoji per name, unique among siblings) and rewrites every reference to it:
 * `bun r6-z-mutations-rename.ts <existing absolute path> <new name> [--dry]`. References are the old name qualified by its parent
 * folder name (`parent/old`, `parent\old`) anywhere in the plugin and the ticket inputs, plus bare `old` inside the parent subtree
 * (relative `#[path]`, `include_str!`, sibling imports). With `--refs` it only lists the files that mention the old name.
 */
import { readdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, normalize } from "node:path";
import { plugin, repo } from "./r3-f1-paths.ts";

const SKIP = new Set(["node_modules", "target", "dist", ".git", "🗑️generated", ".venv"]);
const TEXT = /\.(rs|ts|tsx|js|mjs|json|md|py|toml|feature|semio|graphql|proto|ebnf|g4|yml|yaml|txt|svg|gltf|semio)$/;
const ticket = join(import.meta.dir);

const scripts = (files: string[]) => files.filter((file) => /\.(ts|py|mjs)$/.test(file));

function walk(dir: string, into: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    if (SKIP.has(name)) continue;
    const path = join(dir, name);
    const stat = statSync(path);
    if (stat.isDirectory()) walk(path, into);
    else if (TEXT.test(name) && stat.size < 4_000_000) into.push(path);
  }
  return into;
}

export function references(oldName: string): string[] {
  return [...walk(plugin), ...scripts(walk(ticket))].filter((file) => readFileSync(file, "utf8").includes(oldName));
}

if (import.meta.main) {
  const [given, newName] = process.argv.slice(2).filter((arg) => !arg.startsWith("--"));
  const target = normalize(given);
  const dry = process.argv.includes("--dry");
  if (process.argv.includes("--refs")) {
    for (const file of references(basename(target))) console.log(file.slice(repo.length + 1));
    process.exit(0);
  }
  const parent = dirname(target);
  const oldName = basename(target);
  const parentName = basename(parent);
  const grandName = basename(dirname(parent));
  const inParent = (file: string) => file.startsWith(parent + "\\") || file.startsWith(parent + "/");
  let changed = 0;
  const files = [...walk(plugin), ...scripts(walk(ticket))];
  if (!dry) renameSync(target, join(parent, newName));
  for (const file of files) {
    const path = !dry && inParent(file) && file.startsWith(target) ? join(parent, newName, file.slice(target.length + 1)) : file;
    let text: string;
    try {
      text = readFileSync(path, "utf8");
    } catch {
      continue;
    }
    if (!text.includes(oldName)) continue;
    const crlf = text.includes("\r\n");
    let next = text;
    for (const separator of ["/", "\\", "\\\\"]) {
      const qualifier = `${grandName}${separator}${parentName}${separator}`;
      next = next.replaceAll(qualifier + oldName, qualifier + newName);
    }
    if (inParent(file)) next = next.replaceAll(oldName, newName);
    if (next !== text) {
      changed++;
      console.log(`${dry ? "would update" : "updated"} ${path.slice(repo.length + 1)}`);
      if (!dry) writeFileSync(path, crlf ? next.replaceAll("\r\n", "\n").replaceAll("\n", "\r\n") : next);
    }
  }
  console.log(`renamed ${oldName} -> ${newName}; ${changed} files rewritten`);
}
