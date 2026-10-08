#!/usr/bin/env bun
/**
 * 🧪️ Private build workspace of `m-multi-data` (input of the verification runs, not a deliverable). The shared workspace cannot build the
 * bim crate while peers' in-progress work breaks it (stdio crates, other leaves' tests), so this script writes a private workspace that
 * mounts the schema tree of the artifact only: the `stdio-ifc` dependency is dropped, `io`, editor and viewer are not mounted, and every
 * test module of a peer is left out. Only the tests of the leaves listed in `MINE` (and the shared leaf kit) are compiled.
 */
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { artifact, repo, subsets } from "./r3-f1-paths.ts";

const MINE = ["move-elements", "rotate-elements", "place-elements", "delete-elements", "rename-element", "set-element-property", "remove-element-property", "set-element-classification", "remove-element-classification", "delete-site", "delete-building", "delete-storey", "delete-wall"];
const priv = join(import.meta.dir, "🗑️generated", "m-multi-data", "priv");
const find = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((n) => n.endsWith(suffix))!);
const sRoot = join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))!);
const pkgDir = find(find(artifact, "packages"), "rust");
const standards = dirname(subsets);
const slash = (p: string) => p.replaceAll("\\", "/");
const rebase = (text: string, from: string) => text.replace(/path = "([^"]+)"/g, (_m, p: string) => `path = "${slash(resolve(from, p))}"`);

rmSync(priv, { recursive: true, force: true });
mkdirSync(join(priv, "x", "y"), { recursive: true });
mkdirSync(join(priv, "m"), { recursive: true });
symlinkSync(standards, join(priv, basename(standards)), "junction");

let root = readFileSync(join(sRoot, "Cargo.toml"), "utf8");
root = root.replace(/members = \[[\s\S]*?\n\]/, 'members = ["x/y"]').replace(/exclude = \[[\s\S]*?\n\]/, "");
writeFileSync(join(priv, "Cargo.toml"), rebase(root, sRoot));
copyFileSync(join(sRoot, "Cargo.lock"), join(priv, "Cargo.lock"));

const lib = slash(join(priv, "m", "lib.rs"));
let pkg = readFileSync(join(pkgDir, "Cargo.toml"), "utf8");
pkg = pkg.replace(/^workspace = ".*"\n/m, "").replace(/^semio-s-artifact-stdio-ifc = .*\n/m, "");
pkg = rebase(pkg, pkgDir).replace(/\[lib\]\npath = "[^"]+"/, () => `[lib]\npath = "${lib}"`);
writeFileSync(join(priv, "x", "y", "Cargo.toml"), pkg);

const absolute = `${slash(dirname(standards))}/${basename(standards)}/`;
const locate = (p: string, dir: string) => (p.startsWith(basename(standards) + "/") ? absolute + p.slice(basename(standards).length + 1) : slash(resolve(dir, p)));
const absolutise = (text: string, dir: string) => text.replace(/#\[path = "([^"]+)"\]/g, (_m, p: string) => `#[path = "${locate(p, dir)}"]`).replace(/include_(str|bytes)!\("([^"]+)"\)/g, (_m, kind: string, p: string) => `include_${kind}!("${locate(p, dir)}")`);

const real = readFileSync(join(artifact, readdirSync(artifact).find((n) => n.endsWith(".rs"))!), "utf8").replaceAll("\r\n", "\n").split("\n");
const head = real.findIndex((l) => l.includes("#region 🔖️ArtifactKind"));
const from = real.findIndex((l) => l.startsWith("pub mod standards {")) - 1;
const to = real.findIndex((l) => l.startsWith("                pub mod io {")) - 2;
const tail = ["            }", "        }", "    }", "}", "pub mod mutations {", "    pub use crate::standards::v1::subsets::any::schema::mutations::*;", "}", ""];
const ioFile = (() => {
  const subsetsDir = join(subsets, readdirSync(subsets).find((n) => n.endsWith("subsets"))!);
  const anyDir = join(subsetsDir, readdirSync(subsetsDir).find((n) => n.endsWith("any"))!);
  return join(anyDir, readdirSync(anyDir).find((n) => n.endsWith("io"))!, "🦀️.rs");
})();
const ioCopy = join(priv, "m", "io.rs");
writeFileSync(
  ioCopy,
  absolutise(
    readFileSync(ioFile, "utf8")
      .replaceAll("\r\n", "\n")
      .replace(/#\[path = "[^"]*export[^"]*"\]\npub mod export;\n/, "")
      .replace(/#\[path = "[^"]*import[^"]*"\]\npub mod import;\n/, "")
      .replace(/#\[cfg\(test\)\]\n#\[path = "[^"]+"\]\nmod tests;\n?/, "")
      .replace(/\/\/#region 🔖️IoDeclaration[\s\S]*?\/\/#endregion 🔖️IoDeclaration\n?/, ""),
    dirname(ioFile),
  ),
);
const ioBlock = ['                #[path = "."]', "                pub mod io {", `                    #[path = "${slash(ioCopy)}"]`, "                    mod component;", "                    pub use component::*;", "                }"];
const body = [...real.slice(0, head), ...real.slice(from, to + 1), ...ioBlock, ...tail];

const keep: string[] = [];
for (let index = 0; index < body.length; index += 1) {
  const line = body[index];
  if (line.trim() === "#[cfg(test)]" && body[index + 1]?.includes("#[path =") && body[index + 2]?.trim().startsWith("mod tests_")) {
    const owner = body[index + 1];
    const wanted = MINE.some((kind) => owner.includes(`${kind}/`));
    if (!wanted) {
      index += 2;
      continue;
    }
  }
  keep.push(line);
}

let counter = 0;
const rewritten = keep.join("\n").replace(/#\[path = "([^"]+)"\]\n(\s*)(pub )?mod component;/g, (whole, p: string) => {
  const path = p.startsWith(basename(standards) + "/") ? absolute + p.slice(basename(standards).length + 1) : p;
  const source = readFileSync(path, "utf8").replaceAll("\r\n", "\n");
  const stripped = source.replace(/#\[cfg\(test\)\]\n#\[path = "[^"]+"\]\nmod tests;\n?/g, "");
  if (stripped === source || path.includes("/🧬️mutations/")) return whole.replace(/#\[path = "([^"]+)"\]/, `#[path = "${slash(path)}"]`);
  counter += 1;
  const copy = join(priv, "m", `component-${counter}.rs`);
  writeFileSync(copy, absolutise(stripped, dirname(path)));
  return whole.replace(/#\[path = "([^"]+)"\]/, `#[path = "${slash(copy)}"]`);
});
writeFileSync(join(priv, "m", "lib.rs"), absolutise(rewritten, join(priv, "m")));
console.log("private workspace ready", head, from, to, `${counter} component copies`, existsSync(lib));
