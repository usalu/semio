// 🧪️ Probe of the static resolver: prints owner crate and public path for ui aggregate paths given on argv.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {Resolver, crateFor, defOf, lineAt, loadCrate} from "./rtd-world.ts";
const UI = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust";
const ui = loadCrate(UI)!;
const r = new Resolver();
for (const a of process.argv.slice(2)) {
  const segs = a.split("::");
  const {ent, via} = r.resolveFrom(ui, segs);
  const d = defOf(ent);
  console.log(a, "=>", ent?.kind, d ? `${d.mod.crate.pkg}::${d.mod.path.join("::")}::${d.name} [${d.kind}] ${d.file.split("/").slice(-3).join("/")}:${lineAt(d)} pub=${d.vis} path=${JSON.stringify(r.publicPath(d))}` : "", "via", via.join(","));
}
void crateFor;
