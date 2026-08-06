import { readFileSync, writeFileSync } from "fs";

const cargoPath = readFileSync("/tmp/gis-cargo.txt", "utf8").trim();
let cargo = readFileSync(cargoPath, "utf8");
cargo = cargo.replace(
  /framework_surface_tiled_map = \{ path = "([^"]+)", package = "semio-framework-surface", default-features = false \}/,
  'framework_surface = { path = "$1", package = "semio-framework-surface", default-features = false }',
);
if (!cargo.includes("framework_surface = {")) {
  console.error("cargo dep replace failed");
  console.log(cargo.match(/.*surface.*/g));
  process.exit(1);
}
writeFileSync(cargoPath, cargo);
console.log("cargo dep renamed to framework_surface");

const glue = readFileSync("/tmp/gis-glue.txt", "utf8").trim();
let s = readFileSync(glue, "utf8");
const old = `extern crate framework_surface_tiled_map as framework_surface;
pub use framework_surface::tiled_map as framework_surface_tiled_map;
pub use framework_surface::terrain as framework_surface_terrain;`;
const neu = `pub use framework_surface::tiled_map as framework_surface_tiled_map;
pub use framework_surface::terrain as framework_surface_terrain;`;
if (!s.includes(old) && !s.includes(neu)) {
  // try just replace whatever is there
  s = s.replace(/extern crate framework_surface_tiled_map as framework_surface;\n?/, "");
  if (!s.includes("pub use framework_surface::tiled_map as framework_surface_tiled_map")) {
    console.error("glue unexpected");
    process.exit(1);
  }
} else if (s.includes(old)) {
  s = s.replace(old, neu);
}
writeFileSync(glue, s);
console.log("glue uses crate framework_surface");
