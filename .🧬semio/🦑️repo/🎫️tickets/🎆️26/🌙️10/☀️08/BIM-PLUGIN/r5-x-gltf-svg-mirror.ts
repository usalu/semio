#!/usr/bin/env bun
/** 🪞️ Builds a private mirror workspace of the BIM model crate whose lib tests contain only the glTF and SVG export tests (the fleet edits the peers' test modules concurrently, so the shared crate rarely compiles its whole test target). `bun r5-x-gltf-svg-mirror.ts` rebuilds the mirror; run the export tests with `cargo test --manifest-path <mirror>/priv/Cargo.toml -p semio-s-artifact-bim-model --lib -- export::gltf export::svg` through the gate. */
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { artifact, em, repo, subset, subsets } from "./r3-f1-paths.ts";

const here = join(import.meta.dir, "🗑️generated", "x-gltf-svg-mirror");
const mirror = join(here, "mirror");
const priv = join(here, "priv");
const find = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((n) => n.endsWith(suffix))!);
const sRoot = join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))!);
const pkgDir = find(find(artifact, "packages"), "rust");
const slash = (p: string) => p.replaceAll("\\", "/");
const rebase = (text: string, from: string) => text.replace(/path = "([^"]+)"/g, (_m, p: string) => `path = "${slash(resolve(from, p))}"`);
const modelDir = join(mirror, basename(artifact));
const standards = (root: string) => join(root, basename(dirname(subsets)));

const copyTree = (from: string, to: string) => {
  mkdirSync(to, { recursive: true });
  for (const name of readdirSync(from)) {
    const source = join(from, name);
    if (statSync(source).isDirectory()) copyTree(source, join(to, name));
    else copyFileSync(source, join(to, name));
  }
};
const walk = (dir: string): string[] => readdirSync(dir).flatMap((name) => {
  const path = join(dir, name);
  return statSync(path).isDirectory() ? walk(path) : [path];
});

const KEEP = [em(0x1f9ca) + "gltf", em(0x1f3a8) + "svg"];
const kept = (file: string) => KEEP.some((name) => slash(file).includes(`/${name}/`));
const TEST_MOUNT = /#\[cfg\(test\)\]\s*(?:#\[path\s*=\s*"[^"]*"\]\s*)?(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+\w+\s*;/g;

if (process.argv[2] === "back") {
  const mirrorAny = join(modelDir, subset.slice(artifact.length + 1));
  const assetDir = readdirSync(subset).find((n) => n.endsWith("assets"))!;
  for (const example of readdirSync(join(mirrorAny, assetDir))) {
    const from = join(mirrorAny, assetDir, example, em(0x1f5e3) + ".dsl.semio");
    if (existsSync(from) && example !== em(0x1f3ac) + "demo") copyFileSync(from, join(subset, assetDir, example, em(0x1f5e3) + ".dsl.semio"));
  }
  console.log("blessed texts copied back");
  process.exit(0);
}

rmSync(mirror, { recursive: true, force: true });
rmSync(priv, { recursive: true, force: true });
copyTree(artifact, modelDir);
let stripped = 0;
for (const file of walk(modelDir).filter((p) => p.endsWith(".rs") && !kept(p))) {
  const text = readFileSync(file, "utf8");
  const next = text.replace(TEST_MOUNT, (match) => (stripped++, ""));
  if (next !== text) writeFileSync(file, next);
}
const rootFile = join(modelDir, readdirSync(modelDir).find((n) => n.endsWith(".rs"))!);
const rootText = readFileSync(rootFile, "utf8");
writeFileSync(rootFile, rootText.replace(TEST_MOUNT, (match) => (stripped++, "")));

const MUTATION = em(0x1f9ec) + "mutations";
let owners = 0;
for (const file of walk(modelDir).filter((p) => basename(p) === em(0x1f523) + ".json" && existsSync(join(dirname(p), em(0x1f9a0) + "mutation")) && basename(dirname(dirname(p))) === MUTATION)) {
  const leaf = dirname(file);
  const owner = slash(leaf.slice(repo.length + 1));
  writeFileSync(file, readFileSync(file, "utf8").replace(/"owner": "[^"]*"/, `"owner": "${owner}"`));
  owners++;
}
mkdirSync(join(priv, "x", "y"), { recursive: true });
let root = readFileSync(join(sRoot, "Cargo.toml"), "utf8");
root = root.replace(/members = \[[\s\S]*?\n\]/, 'members = ["x/y"]').replace(/exclude = \[[\s\S]*?\n\]/, "");
writeFileSync(join(priv, "Cargo.toml"), rebase(root, sRoot));
copyFileSync(join(sRoot, "Cargo.lock"), join(priv, "Cargo.lock"));
let pkg = readFileSync(join(pkgDir, "Cargo.toml"), "utf8");
pkg = pkg.replace(/^workspace = ".*"\r?\n/m, "");
pkg = rebase(pkg, pkgDir).replace(/\[lib\]\r?\npath = "[^"]+"/, `[lib]\npath = "${slash(rootFile)}"`);
writeFileSync(join(priv, "x", "y", "Cargo.toml"), pkg);
symlinkSync(standards(modelDir), join(priv, basename(dirname(subsets))), "junction");
console.log(`mirror ready (${stripped} peer test mounts stripped, ${owners} leaf owners rebased):`, slash(priv));
