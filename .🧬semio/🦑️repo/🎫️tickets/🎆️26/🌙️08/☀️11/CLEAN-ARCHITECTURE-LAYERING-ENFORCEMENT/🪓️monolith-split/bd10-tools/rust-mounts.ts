// 🧰 bd10: mount walker for `#[path]`-mounted Rust crates (module path of every mounted file and the mount item that reaches it).
import {existsSync, readFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {parseRustSourceTree, type RustModuleNode} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";

export type MountSite = {readonly file: string; readonly node: RustModuleNode; readonly target: string; readonly modPath: readonly string[]; readonly test: boolean};

export function walkCrate(rootAbs: string): {sites: MountSite[]; paths: Map<string, readonly string[]>} {
  const sites: MountSite[] = [];
  const paths = new Map<string, readonly string[]>();
  const seen = new Set<string>();
  const visit = (file: string, modPath: readonly string[], test: boolean): void => {
    const key = file + "\0" + modPath.join("::");
    if (seen.has(key) || !existsSync(file)) return;
    seen.add(key);
    if (!paths.has(file)) paths.set(file, modPath);
    const tree = parseRustSourceTree(readFileSync(file, "utf8"));
    const body = (nodes: readonly RustModuleNode[], dir: string, trail: readonly string[], inTest: boolean): void => {
      for (const node of nodes) {
        const here = [...trail, node.name];
        const nodeTest = inTest || node.conditionalTest;
        if (node.inline) {
          const next = node.pathAttribute !== null ? resolve(dir, node.pathAttribute) : join(dir, node.name);
          body(node.children, next, here, nodeTest);
        } else if (node.pathAttribute !== null) {
          const target = resolve(dir, node.pathAttribute);
          sites.push({file, node, target, modPath: here, test: nodeTest});
          visit(target, here, nodeTest);
        }
      }
    };
    body(tree.modules, dirname(file), modPath, test);
  };
  visit(rootAbs, [], false);
  return {sites, paths};
}
