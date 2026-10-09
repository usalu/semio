import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2];
let text = readFileSync(file, "utf8");
const swap = (from, to) => { if (!text.includes(from)) throw new Error("missing " + from.slice(0, 70)); text = text.replace(from, to); };
if (!text.includes("FinishArea")) {
  swap("ALL: [ScheduleField; 33]", "ALL: [ScheduleField; 35]");
  swap("        Self::Usage,\n        Self::Swing,", "        Self::Usage,\n        Self::Surface,\n        Self::Swing,");
  swap("        Self::LayerMass,\n    ];", "        Self::LayerMass,\n        Self::FinishArea,\n    ];");
  swap('            Self::Usage => "usage",\n', '            Self::Usage => "usage",\n            Self::Surface => "surface",\n');
  swap('            Self::LayerMass => "layer_mass",\n', '            Self::LayerMass => "layer_mass",\n            Self::FinishArea => "finish_area",\n');
  swap("Self::Number | Self::Usage | Self::Swing | Self::Leaves)", "Self::Number | Self::Usage | Self::Surface | Self::Swing | Self::Leaves)");
  swap('Self::GrossArea | Self::NetArea | Self::SurfaceArea | Self::LayerArea => "m²"', 'Self::GrossArea | Self::NetArea | Self::SurfaceArea | Self::LayerArea | Self::FinishArea => "m²"');
  swap("            Self::Number | Self::Usage => category == Space,\n", "            Self::Number | Self::Usage => matches!(category, Space | Finish),\n            Self::Surface | Self::FinishArea => category == Finish,\n");
  swap("pub const ALL: [ScheduleCategory; 13] = [Self::Wall, Self::CurtainWall, Self::Slab, Self::Roof, Self::Column, Self::Beam, Self::Window, Self::Door, Self::Void, Self::Stair, Self::Railing, Self::Space, Self::Material];", "pub const ALL: [ScheduleCategory; 14] = [Self::Wall, Self::CurtainWall, Self::Slab, Self::Roof, Self::Column, Self::Beam, Self::Window, Self::Door, Self::Void, Self::Stair, Self::Railing, Self::Space, Self::Finish, Self::Material];");
  swap('            Self::Space => "space",\n', '            Self::Space => "space",\n            Self::Finish => "finish",\n');
  swap("`material` for the material take-off.", "`finish` for the room finishes, `material` for the material take-off.");
  swap("    /// 🧱️ Whether the rows of the category are layers or material runs of elements of any kind.", "    /// 🎨️ Whether the rows of the category are the finished surfaces (floor, walls, ceiling) of rooms.\n    pub const fn is_finish(self) -> bool {\n        matches!(self, Self::Finish)\n    }\n\n    /// 🧱️ Whether the rows of the category are layers or material runs of elements of any kind.");
  swap('pub const PRESETS: [&str; 5] = ["door", "window", "room", "wall", "material"];', 'pub const PRESETS: [&str; 6] = ["door", "window", "room", "finish", "wall", "material"];');
  swap("`room` the spaces grouped by storey with area and volume totals,", "`room` the spaces grouped by storey with area and volume totals, `finish` the finished surfaces of the rooms collapsed per material with the finish areas summed,");
  swap('        "wall" => schedule(', '        "finish" => schedule(ScheduleCategory::Finish, vec![column(Material, false), column(Surface, false), column(FinishArea, true)], vec![ascending(Material), ascending(Surface)], Vec::new(), vec![group(Material)], false),\n        "wall" => schedule(');
}
writeFileSync(file, text);
