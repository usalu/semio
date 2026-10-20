import { existsSync } from "node:fs";
import { dirname } from "node:path";
import { buildTree, CrateTree, M, T, V } from "../../../rs-sweep-tools/tree.ts";
import { Package, loadPackage } from "../../../rs-sweep-tools/cargo.ts";
import { Def, lookup, setTreeBuilder, libTree } from "../../../rs-sweep-tools/resolve.ts";
import { Verdict } from "./transform.ts";

export const ROOT = process.env.GRAN_ROOT ?? "/Users/ueli/Documents/semio";
export const FACADE_DIR = `${ROOT}/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust`;

const trees = new Map<string, CrateTree>();

/** 🌳 Builds (once) the module tree of a package's lib target. */
export function treeOf(pkg: Package): CrateTree | null {
  const t = pkg.targets.find((x) => x.kind === "lib");
  if (!t || !t.path || !existsSync(t.path)) return null;
  const key = `${pkg.dir}|${t.path}`;
  let tr = trees.get(key);
  if (!tr) {
    tr = buildTree(pkg, t, pkg.name);
    trees.set(key, tr);
  }
  return tr;
}
setTreeBuilder(treeOf);

export type Owners = { byLib: Map<string, Package>; facade: Package; verify: (lib: string, segs: string[]) => Verdict };

/** 🧭 Loads the owning crates behind the facade and returns an outside-the-crate path verifier. */
export function loadOwners(libs: string[]): Owners {
  const facade = loadPackage(FACADE_DIR)!;
  const byLib = new Map<string, Package>();
  for (const d of facade.deps) {
    if (!d.path) continue;
    const p = loadPackage(d.path);
    if (p?.libName && libs.includes(p.libName)) byLib.set(p.libName, p);
  }
  for (const l of libs) if (!byLib.has(l)) throw new Error(`owner crate ${l} is not a dependency of the facade manifest`);
  const outside = treeOf(facade)!.root;
  const memo = new Map<string, Verdict>();
  const check = (lib: string, segs: string[], env: { test: boolean; wasm: boolean }): Verdict => {
    const tree = treeOf(byLib.get(lib)!);
    if (!tree) return "missing";
    let cur: Def[] = [{ kind: "crate", ns: T, mod: tree.root }];
    let unknown = false;
    for (let i = 0; i < segs.length; i++) {
      const mods = cur.filter((d) => d.mod);
      if (!mods.length) return i > 0 ? "ok" : "missing";
      const want = i === segs.length - 1 ? T | V | M : T | V;
      const next: Def[] = [];
      for (const d of mods) {
        const r = lookup(d.mod!, segs[i], want, outside, { env, home: tree });
        next.push(...r.defs);
        if (r.err?.kind === "unknown") unknown = true;
      }
      if (!next.length) return unknown ? "unknown" : "missing";
      cur = next;
    }
    return "ok";
  };
  const verify = (lib: string, segs: string[]): Verdict => {
    const key = `${lib}::${segs.join("::")}`;
    const hit = memo.get(key);
    if (hit) return hit;
    let v: Verdict = "missing";
    for (const env of [{ test: false, wasm: false }, { test: true, wasm: false }, { test: false, wasm: true }]) {
      const r = check(lib, segs, env);
      if (r === "ok") {
        v = "ok";
        break;
      }
      if (r === "unknown") v = "unknown";
    }
    memo.set(key, v);
    return v;
  };
  return { byLib, facade, verify };
}
export { dirname };
