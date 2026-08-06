import { readFileSync, writeFileSync, existsSync } from "fs";

// 1) norm Cargo.toml
const normCargo = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml";
let t = readFileSync(normCargo, "utf8");
const old = t.match(/^fem_core = \{[^}]+\}/m)?.[0];
const neu = 'fem = { path = "../../../🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }';
if (old) {
  t = t.replace(old, neu);
  // drop anticipating comments if present
  t = t.replace(/# before running their ULS checks\.[\s\S]*?as part of 🏗️fem's[^\n]*\n/g, "");
  writeFileSync(normCargo, t);
  console.log("norm Cargo.toml:", old, "=>", neu);
} else {
  console.log("norm: fem_core line not found; current fem lines:");
  for (const l of t.split("\n")) if (l.includes("fem")) console.log(" ", l);
}

// 2) norm engine use sites
for (const rel of [
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/⚙️engine/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/⚙️engine/🦀️component.rs",
]) {
  const p = "/Users/ueli/Documents/semio/" + rel;
  if (!existsSync(p)) { console.log("missing", rel); continue; }
  let s = readFileSync(p, "utf8");
  const before = s;
  s = s.replace(
    /use fem_core::\{BeamEb2, Dof, MemberUdl, Model, Node, Support\};/g,
    "use fem::core::elements2d::BeamEb2;\nuse fem::core::{Dof, MemberUdl, Model, Node, Support};"
  );
  s = s.replace(/fem_core::/g, "fem::core::");
  if (s !== before) {
    writeFileSync(p, s);
    console.log("patched", rel);
  } else console.log("no change", rel);
}

// 3) fixture-sweep
const sweep = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/Cargo.toml";
t = readFileSync(sweep, "utf8");
const before = t;
t = t.replace(/^fem2d = \{[^}]+\}/m, "").replace(/^fem3d = \{[^}]+\}/m, "");
if (!t.includes('package = "semio-s-plugin-fem"')) {
  // insert fem dep near where fem2d was — after cleanup blank lines
  const femLine = 'fem = { path = "../../../../../../../../✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }';
  // put after [dependencies]
  t = t.replace(/^\[dependencies\]\n/m, `[dependencies]\n${femLine}\n`);
}
t = t.replace(/\n{3,}/g, "\n\n");
writeFileSync(sweep, t);
console.log("sweep cargo changed", before !== t);

const sweepLib = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/📦️lib.rs";
if (existsSync(sweepLib)) {
  let s = readFileSync(sweepLib, "utf8");
  const b = s;
  s = s.replace(/use fem2d::Fem2dDocument;/g, "use fem::artifacts::fem2d::Fem2dDocument;");
  s = s.replace(/use fem3d::Fem3dDocument;/g, "use fem::artifacts::fem3d::Fem3dDocument;");
  if (s !== b) { writeFileSync(sweepLib, s); console.log("patched sweep lib"); }
  else console.log("sweep lib no change / already patched");
}
