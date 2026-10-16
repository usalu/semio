// 🔎️ Read-only: lists the external crates a lone crate root names but cannot resolve (the dependencies it needs), lib and test.
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const V = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts");
const {rustcUnresolvedCrates} = await import(V);
for (const root of process.argv.slice(2)) {
  for (const flags of [[], ["--test"]] as string[][]) {
    const res = rustcUnresolvedCrates(join(ROOT, root), "probe", flags);
    console.log(JSON.stringify({root: root.split("/").slice(-3).join("/"), flags: flags.join(" ") || "lib", unresolved: res.unresolved, exit: res.exit, other: res.otherErrors.slice(0, 6)}));
  }
}
