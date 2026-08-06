import { readFileSync, writeFileSync } from "fs";

const glue = readFileSync("/tmp/gis-glue.txt", "utf8").trim();
let s = readFileSync(glue, "utf8");
const old = `extern crate framework_surface_tiled_map as framework_surface_terrain;`;
const neu = `extern crate framework_surface_tiled_map as framework_surface;
pub use framework_surface::tiled_map as framework_surface_tiled_map;
pub use framework_surface::terrain as framework_surface_terrain;`;
if (!s.includes(old)) {
  if (s.includes("pub use framework_surface::tiled_map as framework_surface_tiled_map")) {
    console.log("glue aliases already fixed");
  } else {
    console.error("extern alias not found");
    process.exit(1);
  }
} else {
  s = s.replace(old, neu);
  writeFileSync(glue, s);
  console.log("glue aliases fixed");
}

const gis2d = readFileSync("/tmp/gis2d-path.txt", "utf8").trim();
let g = readFileSync(gis2d, "utf8");
const oldMenu = `    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, GisMapDocument>,
        cfg: &ConfigView<'_, Gis2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        gis2d_context_menu_items(registry, request.surface.as_ref(), &cfg.projection.selected_ids)
    }`;
const neuMenu = `    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, GisMapDocument>,
        cfg: &ConfigView<'_, Gis2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        gis2d_context_menu_items(registry, request.surface.as_ref(), &cfg.projection.selected_ids)
    }`;
if (!g.includes(oldMenu)) {
  console.error("context_menu block not found");
  const i = g.indexOf("fn context_menu");
  console.log(JSON.stringify(g.slice(i, i + 450)));
  process.exit(1);
}
g = g.replace(oldMenu, neuMenu);
writeFileSync(gis2d, g);
console.log("context_menu signature fixed");
