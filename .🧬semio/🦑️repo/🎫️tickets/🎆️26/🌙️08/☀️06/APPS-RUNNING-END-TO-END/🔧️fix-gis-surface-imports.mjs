import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

function walk(dir, acc = []) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, acc);
    else if (name.endsWith(".rs")) acc.push(p);
  }
  return acc;
}

const gisRoot = (() => {
  // discover gis plugin root from glue path
  const glue = readFileSync("/tmp/gis-glue.txt", "utf8").trim();
  return join(glue, "..", "..", "..");
})();

const files = walk(gisRoot);
let changed = 0;
for (const file of files) {
  let s = readFileSync(file, "utf8");
  const orig = s;
  // Keep glue's pub use aliases for crate:: paths; rewrite bare uses to framework_surface::…
  if (file.endsWith("📦️glue.rs") && s.includes("pub use framework_surface::tiled_map")) {
    // Remove the aliases — call sites will use framework_surface::* directly
    s = s.replace(`pub use framework_surface::tiled_map as framework_surface_tiled_map;\npub use framework_surface::terrain as framework_surface_terrain;\n`, "");
  } else {
    s = s.replaceAll("framework_surface_tiled_map::", "framework_surface::tiled_map::");
    s = s.replaceAll("framework_surface_terrain::", "framework_surface::terrain::");
    // bare `use framework_surface_tiled_map::{...}` already handled by replaceAll on path prefix
    // also handle `use framework_surface_tiled_map;` if any
    s = s.replaceAll("use framework_surface_tiled_map;", "use framework_surface::tiled_map;");
    s = s.replaceAll("use framework_surface_terrain;", "use framework_surface::terrain;");
  }
  if (s !== orig) {
    writeFileSync(file, s);
    changed += 1;
    console.log("updated", file);
  }
}
console.log("changed files:", changed);
