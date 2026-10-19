import {testTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🗂️taxonomy/🟦️.ts";
import {inspectTestLayoutSources} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🗺️test-layout/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const taxonomy = testTaxonomy(root);
const O = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace";
const src = "// header\n\npackage support\n";
const variants: Record<string, string[]> = {
  R: [`${O}/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml`, `${O}/🧪️tests/🧰️support/🦀️.rs`, `${O}/🧪️tests/🧰️support/🔬️store-fixture/🦀️.rs`],
  G: [`${O}/🧪️tests/🧰️support/📦️packages/🐹️go/go.mod`, `${O}/🧪️tests/🧰️support/🐹️.go`],
  H: [`${O}/🧪️tests/🧰️support/🐹️.go`, `${O}/🧪️tests/🧰️support/🔬️store-fixture/🐹️.go`],
  A: [`${O}/🧪️tests/🧰️support/go.mod`, `${O}/🧪️tests/🧰️support/🐹️.go`, `${O}/🧪️tests/🧰️support/🛡️root-guard/🐹️.go`],
  B: [`${O}/🧪️tests/🧰️support/📦️packages/🐹️go/go.mod`, `${O}/🧪️tests/🧰️support/🐹️.go`, `${O}/🧪️tests/🧰️support/🛡️root-guard/🐹️.go`],
  C: [`${O}/🧪️tests/🧰️support/📦️packages/🐹️go/go.mod`, `${O}/🧪️tests/🧰️support/📦️packages/🐹️go/🐹️.go`, `${O}/🧪️tests/🧰️support/📦️packages/🐹️go/🛡️root-guard/🐹️.go`],
  D: [`${O}/🧪️tests/🧰️support/🐹️go/go.mod`, `${O}/🧪️tests/🧰️support/🐹️go/🐹️.go`, `${O}/🧪️tests/🧰️support/🐹️go/🛡️root-guard/🐹️.go`],
};
for (const [name, paths] of Object.entries(variants)) {
  const dirs = new Set<string>();
  for (const p of paths) { const parts = p.split("/"); for (let i = 1; i < parts.length; i++) dirs.add(parts.slice(0, i).join("/")); }
  const findings = inspectTestLayoutSources(taxonomy, paths.map((path) => ({path, source: /\\.(go|rs)$/.test(path) ? src : ""})), [...dirs]);
  console.log(name, findings.length, findings.map((f: any) => `${f.code} ${f.path}`).join(" | "));
}
