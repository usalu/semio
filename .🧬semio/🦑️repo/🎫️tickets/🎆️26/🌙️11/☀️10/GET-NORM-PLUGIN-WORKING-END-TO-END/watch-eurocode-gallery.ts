#!/usr/bin/env repo
import { mkdir, readdir } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const [lane, rustc] = process.argv.slice(2);
if (!lane || !rustc) throw new Error("Expected the grouped compiler lane and Rust compiler path");

const ticket = dirname(fileURLToPath(import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const output = join(ticket, "🗑️generated", "gallery-wB34");
const build = join(root, ".cache/b/r/m/u/l", lane, "wasm32-wasip2/prototype/build");
const packages = [
  "semio-plugin",
  "semio-ui-runtime",
  "semio-s-plugin-norm-app-surface-visual-chrome",
  "semio-s-artifact-norm-en1990-model",
  "semio-s-artifact-norm-en1990-inferences",
  "semio-s-artifact-norm-en1990-scene-diagrams",
];

async function available(name: string): Promise<boolean> {
  const found = await readdir(join(build, name)).catch(() => []);
  for (const hash of found) {
    const files = await readdir(join(build, name, hash, "out")).catch(() => []);
    if (files.some(file => file.endsWith(".rlib") && files.includes(file.replace(/\.rlib$/u, ".rmeta")))) return true;
  }
  return false;
}

const assets = join(root, "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
const examples = Array.from(new Bun.Glob("**/*.dsl.semio").scanSync({ cwd: assets, absolute: true })).sort();
if (examples.length === 0) throw new Error("No authored EN 1990 examples");
const jcoPackage = join(root, "node_modules/@bytecodealliance/jco");
const jcoBin = (await Bun.file(join(jcoPackage, "package.json")).json()).bin.jco;
if (!jcoBin || !await Bun.file(join(jcoPackage, jcoBin)).exists()) throw new Error("The installed JCO command is unavailable");
await mkdir(output, { recursive: true });

let previous = "";
for (;;) {
  const ready = await Promise.all(packages.map(available));
  const missing = packages.filter((_, index) => !ready[index]).join(", ");
  if (missing.length === 0) break;
  if (missing !== previous) {
    console.log(`[DEBUG] ${lane} waiting for ${missing}`);
    previous = missing;
  }
  await Bun.sleep(15000);
}

async function run(command: string[]): Promise<void> {
  const result = Bun.spawn(command, { cwd: root, stdout: "inherit", stderr: "inherit" });
  const code = await result.exited;
  if (code !== 0) process.exit(code);
}

await run(["repo", "capacity", "run", "--memory=700", "--", process.platform === "win32" ? "python" : "python3", join(ticket, "link-eurocode-windows.py"), "en1990", "--gallery-only", "--lane", lane, "--rustc", rustc, "--output-dir", output]);
await run(["repo", "capacity", "run", "--memory=350", "--", "node", join(jcoPackage, jcoBin), "run", join(output, "gallery-en1990.wasm"), "--jco-dir", join(output, "jco"), "--every", ...examples]);
