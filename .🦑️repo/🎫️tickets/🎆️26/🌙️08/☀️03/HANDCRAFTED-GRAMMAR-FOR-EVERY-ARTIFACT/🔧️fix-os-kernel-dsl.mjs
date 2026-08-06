import { readFileSync, existsSync, readdirSync, writeFileSync } from "fs";
import { join } from "path";

const fw = readdirSync(".").find((x) => x.includes("framework"));
const products = join(fw, "🛍️products");
const os = readdirSync(products).find((x) => x.includes("os"));
const pkgDir = join(products, os, "📦️packages", "🦀️rust");
const cargoPath = join(pkgDir, "Cargo.toml");
const cargo = readFileSync(cargoPath, "utf8");
console.log("cargo", cargoPath);
console.log("has dsl_grammar", cargo.includes("dsl_grammar"));
console.log(
  cargo
    .split("\n")
    .filter((l) => /dsl|grammar|^name/.test(l))
    .slice(0, 50)
    .join("\n")
);

const gluePath = join(pkgDir, "📦️glue.rs");
if (existsSync(gluePath)) {
  const g = readFileSync(gluePath, "utf8");
  console.log(
    "glue lines",
    g
      .split("\n")
      .filter((l) => /dsl|grammar/.test(l))
      .slice(0, 40)
      .join("\n")
  );
}

// If os-kernel compiles component.rs with pub use dsl_grammar but lacks dep, add it.
const grammarPath = join(
  products,
  os,
  "🔨️modules",
  "🗣️dsl",
  "📖️grammar",
  "⚡️implementations",
  "🦀️rust"
);
console.log("grammarPath exists", existsSync(grammarPath), grammarPath);

if (!cargo.includes("dsl_grammar")) {
  const depLine =
    'dsl_grammar = { path = "../../🔨️modules/🗣️dsl/📖️grammar/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-dsl-grammar" }';
  let next = cargo;
  if (cargo.includes("dsl_schema")) {
    next = cargo.replace(
      /dsl_schema = \{[^}]+\}/,
      (m) => m + "\n" + depLine
    );
  } else {
    next = cargo.replace(
      /\[dependencies\]/,
      "[dependencies]\n" + depLine
    );
  }
  writeFileSync(cargoPath, next);
  console.log("added dsl_grammar dep to os-kernel");
} else {
  console.log("dep already present");
}
