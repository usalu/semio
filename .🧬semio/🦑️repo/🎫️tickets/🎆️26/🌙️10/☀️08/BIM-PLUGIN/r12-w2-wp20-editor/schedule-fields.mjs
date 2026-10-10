import { readFileSync, writeFileSync, copyFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2];
const T = process.argv[3];
const S = join(root, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any");
const E = join(S, "✏️editor");

const edit = (file, swaps, marker) => {
  let text = readFileSync(file, "utf8");
  if (text.includes(marker)) return console.log("kept  ", file.slice(-48));
  const crlf = text.includes("\r\n");
  text = text.replaceAll("\r\n", "\n");
  for (const [from, to] of swaps) {
    if (!text.includes(from)) throw new Error(file.slice(-48) + " missing: " + from.slice(0, 80));
    text = text.replace(from, () => to);
  }
  const tmp = file + ".new.tmp";
  writeFileSync(tmp, crlf ? text.replaceAll("\n", "\r\n") : text);
  copyFileSync(tmp, file);
  console.log("edited", file.slice(-48));
};

edit(join(T, "r3-f1-gen-model.ts"), [['"LayerArea", "LayerVolume", "LayerMass", "FinishArea"] }', '"LayerArea", "LayerVolume", "LayerMass", "FinishArea", "UValue", "GValue"] }']], '"FinishArea", "UValue"');

edit(join(S, "🧬️schema/📸️snapshot/💠️values/🦀️.rs"), [["    LayerMass,\n    FinishArea,\n}", "    LayerMass,\n    FinishArea,\n    UValue,\n    GValue,\n}"]], "    UValue,\n    GValue,");
edit(join(S, "🧬️schema/🔗️.graphql"), [["  LayerMass\n  FinishArea\n}", "  LayerMass\n  FinishArea\n  UValue\n  GValue\n}"]], "  UValue\n");
edit(join(S, "🧬️schema/🔣️.json"), [['        "LayerMass",\n        "FinishArea"\n      ]', '        "LayerMass",\n        "FinishArea",\n        "UValue",\n        "GValue"\n      ]']], '"UValue"');
edit(join(S, "🧬️schema/🟦️.ts"), [['"LayerMass" | "FinishArea";', '"LayerMass" | "FinishArea" | "UValue" | "GValue";']], '"UValue"');

edit(join(S, "🧬️schema/📸️snapshot/📋️schedule-kit/🦀️.rs"), [
  ["ALL: [ScheduleField; 35]", "ALL: [ScheduleField; 37]"],
  ["        Self::LayerMass,\n        Self::FinishArea,\n    ];", "        Self::LayerMass,\n        Self::FinishArea,\n        Self::UValue,\n        Self::GValue,\n    ];"],
  ['            Self::FinishArea => "finish_area",\n', '            Self::FinishArea => "finish_area",\n            Self::UValue => "u_value",\n            Self::GValue => "g_value",\n'],
  ['            Self::Mass | Self::LayerMass => "kg",\n            _ => "",', '            Self::Mass | Self::LayerMass => "kg",\n            Self::UValue => "W/(m²·K)",\n            _ => "",'],
  ["            Self::Surface | Self::FinishArea => category == Finish,\n", "            Self::Surface | Self::FinishArea => category == Finish,\n            Self::UValue => matches!(category, Window | Door),\n            Self::GValue => category == Window,\n"],
  ['pub const PRESETS: [&str; 6] = ["door", "window", "room", "finish", "wall", "material"];', 'pub const PRESETS: [&str; 7] = ["door", "window", "room", "finish", "envelope", "wall", "material"];'],
  ["`finish` the floor,", "`envelope` every window with its type, size, area, U-value and g-value, `finish` the floor,"],
  ['        "wall" => schedule(', '        "envelope" => schedule(ScheduleCategory::Window, vec![column(Name, false), column(Type, false), column(Storey, false), column(Width, false), column(Height, false), column(GrossArea, true), column(UValue, false), column(GValue, false), column(Count, true)], vec![ascending(Storey), ascending(Name)], Vec::new(), Vec::new(), true),\n        "wall" => schedule('],
], "Self::UValue => \"u_value\"");

edit(join(S, "🧬️schema/💡️inferences/📋️schedules/🗂️rows/🦀️.rs"), [
  ["    pub panes: ScheduleCell,\n", "    pub panes: ScheduleCell,\n    pub u_value: ScheduleCell,\n    pub g_value: ScheduleCell,\n"],
  ["            panes: window.map_or(ScheduleCell::Empty, |kind| ScheduleCell::number(f64::from(kind.panes))),\n", "            panes: window.map_or(ScheduleCell::Empty, |kind| ScheduleCell::number(f64::from(kind.panes))),\n            u_value: window.map(|kind| kind.u_value).or_else(|| door.map(|(_, kind)| kind.u_value)).flatten().map_or(ScheduleCell::Empty, ScheduleCell::number),\n            g_value: window.and_then(|kind| kind.g_value).map_or(ScheduleCell::Empty, ScheduleCell::number),\n"],
  ["            ScheduleField::Panes => fact(|row| &row.panes),\n", "            ScheduleField::Panes => fact(|row| &row.panes),\n            ScheduleField::UValue => fact(|row| &row.u_value),\n            ScheduleField::GValue => fact(|row| &row.g_value),\n"],
], "ScheduleField::UValue");

edit(join(E, "🎭️modes/✏️edit/🪟️windows/🧮️schedule/🏷️vocabulary/🦀️.rs"), [["        FinishArea => labels.sf_finish_area,\n", "        FinishArea => labels.sf_finish_area,\n        UValue => labels.sf_u_value,\n        GValue => labels.sf_g_value,\n"]], "UValue => labels.sf_u_value");
edit(join(E, "🎭️modes/✏️edit/🪟️windows/🧮️schedule/🦀️.rs"), [['        "finish" => labels.sp_finish,\n', '        "finish" => labels.sp_finish,\n        "envelope" => labels.sp_envelope,\n']], '"envelope" => labels.sp_envelope');
edit(join(E, "🗣️terminology/🦀️.rs"), [
  ['    sf_finish_area: "Finish area", "Belagsfläche";\n', '    sf_finish_area: "Finish area", "Belagsfläche";\n    sf_u_value: "U-value", "U-Wert";\n    sf_g_value: "g-value", "g-Wert";\n'],
  ['    sp_finish: "Room finish schedule", "Raumbelagsliste";\n', '    sp_finish: "Room finish schedule", "Raumbelagsliste";\n    sp_envelope: "Envelope schedule", "Hüllenliste";\n'],
], "sf_u_value:");
