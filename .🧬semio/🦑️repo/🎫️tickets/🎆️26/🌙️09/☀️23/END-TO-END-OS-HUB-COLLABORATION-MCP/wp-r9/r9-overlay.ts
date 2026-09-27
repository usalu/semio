#!/usr/bin/env bun
/**
 * 🪞️ R9 window-3 scratch overlay: APFS-clones every tracked and untracked-unignored file plus the ignored `🤖️generated/`
 * trees into `.🧬semio/🌐hub/s13-r9-overlay/`, and symlinks every ignored `node_modules/` back to the real tree.
 * Resumable: existing destination files are kept. `--refresh <path…>` re-clones the named repo paths from the real tree.
 */
import { copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, rmSync, symlinkSync } from "node:fs";
import { dirname, join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const OVERLAY = join(ROOT, ".🧬semio/🌐hub/s13-r9-overlay");
const git = (...args: string[]) => Bun.spawnSync(["git", ...args], { cwd: ROOT, stdout: "pipe", stderr: "pipe" }).stdout.toString().split("\0").filter(Boolean);
const clone = (path: string, force = false): boolean => {
  const source = join(ROOT, path), destination = join(OVERLAY, path);
  let node;
  try { node = lstatSync(source); } catch { return false; }
  if (!force && existsSync(destination)) return false;
  mkdirSync(dirname(destination), { recursive: true });
  if (force) rmSync(destination, { recursive: true, force: true });
  if (node.isSymbolicLink()) symlinkSync(Bun.spawnSync(["readlink", source]).stdout.toString().trim(), destination);
  else if (node.isDirectory()) cpSync(source, destination, { recursive: true, verbatimSymlinks: true });
  else copyFileSync(source, destination);
  return true;
};
const args = process.argv.slice(2);
if (args[0] === "--refresh") {
  for (const path of args.slice(1)) console.log(`${clone(path, true) ? "refreshed" : "missing"} ${path}`);
  process.exit(0);
}
const started = performance.now();
const files = git("ls-files", "-z", "--cached", "--others", "--exclude-standard");
let cloned = 0;
for (const path of files) if (clone(path)) cloned += 1;
const ignored = git("ls-files", "-z", "--others", "--ignored", "--exclude-standard", "--directory").map((path) => path.replace(/\/$/u, ""));
let generated = 0, linked = 0;
for (const path of ignored) {
  const segments = path.split("/");
  if (segments.includes("node_modules") && segments.indexOf("node_modules") !== segments.length - 1) continue;
  if (segments.at(-1) === "node_modules") {
    const destination = join(OVERLAY, path);
    if (!existsSync(destination)) { mkdirSync(dirname(destination), { recursive: true }); symlinkSync(join(ROOT, path), destination); linked += 1; }
  } else if (segments.at(-1) === "🤖️generated" && !segments.some((segment) => segment.startsWith(".🧬semio") || segment === "target")) {
    if (clone(path)) generated += 1;
  }
}
console.log(JSON.stringify({ files: files.length, cloned, generated, linked, seconds: Math.round((performance.now() - started) / 1000) }));
