import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { artifact, repo, subsets } from "./r3-f1-paths.ts";

const priv = join(import.meta.dir, "🗑️generated", "i-openings", "priv");
const find = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((n) => n.endsWith(suffix))!);
const sRoot = join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))!);
const pkgDir = find(find(artifact, "packages"), "rust");
const standards = dirname(subsets);
const slash = (p: string) => p.replaceAll("\\", "/");
const rebase = (text: string, from: string) => text.replace(/path = "([^"]+)"/g, (_m, p: string) => `path = "${slash(resolve(from, p))}"`);

rmSync(priv, { recursive: true, force: true });
mkdirSync(join(priv, "x", "y"), { recursive: true });

let root = readFileSync(join(sRoot, "Cargo.toml"), "utf8");
root = root.replace(/members = \[[\s\S]*?\n\]/, 'members = ["x/y"]').replace(/exclude = \[[\s\S]*?\n\]/, "");
root = rebase(root, sRoot);
writeFileSync(join(priv, "Cargo.toml"), root);
copyFileSync(join(sRoot, "Cargo.lock"), join(priv, "Cargo.lock"));

let pkg = readFileSync(join(pkgDir, "Cargo.toml"), "utf8");
pkg = pkg.replace(/^workspace = ".*"\n/m, "").replace(/^semio-s-artifact-stdio-ifc = .*\n/m, "");
pkg = rebase(pkg, pkgDir).replace(/\[lib\]\npath = "[^"]+"/, `[lib]\npath = "${slash(join(artifact, readdirSync(artifact).find((n) => n.endsWith(".rs"))!))}"`);
writeFileSync(join(priv, "x", "y", "Cargo.toml"), pkg);

symlinkSync(standards, join(priv, basename(standards)), "junction");
console.log("private workspace ready", slash(priv), existsSync(join(priv, basename(standards))), slash(sRoot), slash(pkgDir));
