// ⚖️ Verdicts of the static resolver for paths below the ui aggregate alias: aggregate-owned, producer-owned (with the owner's public path), module, unresolved or external.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {join} from "node:path";
import {Resolver, lineAt, loadCrate} from "./rtd-world.ts";

export const ROOT = "/Users/ueli/Documents/semio";
export const UI = join(ROOT, "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust");
export const ui = loadCrate(UI)!;
export const R = new Resolver();

export type Verdict = {status: "producer" | "aggregate" | "module" | "unresolved" | "external"; segs: string[]; consumed?: number; ownerPkg?: string; ownerDir?: string; ownerPath?: string[]; detail?: string; gates?: string[]; defFile?: string; defLine?: number; kind?: string};
const cache = new Map<string, Verdict>();
export const verdictOf = (segs: string[]): Verdict => {
  const key = segs.join("::");
  const hit = cache.get(key);
  if (hit) return hit;
  let v: Verdict;
  const {ent} = R.resolveFrom(ui, segs);
  if (!ent) v = {status: "unresolved", segs};
  else if (ent.kind === "external") v = {status: "external", segs, detail: ent.name};
  else if (ent.kind === "item") {
    const d = ent.def;
    const consumed = segs.length - ent.rest.length;
    if (d.mod.crate.pkg === "semio-framework-ui") v = {status: "aggregate", segs, consumed, kind: d.kind, defFile: d.file.replace(ROOT + "/", ""), defLine: lineAt(d), ownerPkg: d.mod.crate.pkg};
    else {
      const path = R.publicPath(d);
      v = path ? {status: "producer", gates: R.gatesOf(d, path), segs, consumed, ownerPkg: d.mod.crate.pkg, ownerDir: d.mod.crate.dir, ownerPath: path, kind: d.kind, defFile: d.file.replace(ROOT + "/", ""), defLine: lineAt(d)} : {status: "unresolved", segs, detail: `no public path for ${d.mod.crate.pkg}::${d.name}`};
    }
  } else {
    const m = ent.mod;
    v = m.crate.pkg === "semio-framework-ui" ? {status: "aggregate", segs, consumed: segs.length, kind: "mod", ownerPkg: m.crate.pkg} : {status: "module", segs, consumed: segs.length, ownerPkg: m.crate.pkg, ownerDir: m.crate.dir, ownerPath: m.path, kind: "mod"};
  }
  cache.set(key, v);
  return v;
};

