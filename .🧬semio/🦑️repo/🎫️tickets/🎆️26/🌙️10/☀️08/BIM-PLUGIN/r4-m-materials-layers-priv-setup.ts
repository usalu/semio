#!/usr/bin/env bun
/**
 * 🧰️ Seeds the private cargo workspace `🗑️generated/m-materials-layers/priv` for `r4-m-materials-layers-isolated.ts`: a copy of the `✏️s`
 * root manifest reduced to one member (set by the isolated script), with every relative `path` rebased to an absolute one, plus the
 * artifact package manifest without `workspace` and without the `semio-s-artifact-stdio-ifc` dependency (so the build does not drag in
 * unrelated crates that are broken at HEAD). Run once, then `bun r4-m-materials-layers-isolated.ts`.
 */
import { copyFileSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { artifact, repo } from "./r3-f1-paths.ts";

const priv = join(import.meta.dir, "🗑️generated", "m-materials-layers", "priv");
const find = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix))!);
const sRoot = join(repo, readdirSync(repo).find((name) => name.startsWith("✏") && name.endsWith("s"))!);
const pkgDir = find(find(artifact, "packages"), "rust");
const slash = (path: string) => path.replaceAll("\\", "/");
const rebase = (text: string, from: string) => text.replace(/path = "([^"]+)"/g, (_match, path: string) => `path = "${slash(resolve(from, path))}"`);

rmSync(priv, { recursive: true, force: true });
mkdirSync(join(priv, "x", "y"), { recursive: true });

const root = readFileSync(join(sRoot, "Cargo.toml"), "utf8").replace(/members = \[[\s\S]*?\n\]/, 'members = ["x/y"]').replace(/exclude = \[[\s\S]*?\n\]/, "");
writeFileSync(join(priv, "Cargo.toml"), rebase(root, sRoot));
copyFileSync(join(sRoot, "Cargo.lock"), join(priv, "Cargo.lock"));

const manifest = readFileSync(join(pkgDir, "Cargo.toml"), "utf8").replace(/^workspace = ".*"\n/m, "").replace(/^semio-s-artifact-stdio-ifc = .*\n/m, "");
writeFileSync(join(priv, "x", "y", "Cargo.toml"), rebase(manifest, pkgDir));
console.log("private workspace seeded", slash(priv));
