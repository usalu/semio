import { readFileSync, writeFileSync } from "node:fs";
const [vocabulary, terminology] = process.argv.slice(2);
const edit = (file, swaps) => {
  let text = readFileSync(file, "utf8");
  for (const [from, to] of swaps) {
    if (!text.includes(from)) throw new Error("missing " + from.slice(0, 70));
    text = text.replace(from, to);
  }
  writeFileSync(file, text);
};
if (!readFileSync(vocabulary, "utf8").includes("sf_surface")) {
  edit(vocabulary, [
    ["        Usage => labels.sf_usage,\n", "        Usage => labels.sf_usage,\n        Surface => labels.sf_surface,\n"],
    ["        LayerMass => labels.sf_layer_mass,\n", "        LayerMass => labels.sf_layer_mass,\n        FinishArea => labels.sf_finish_area,\n"],
    ["        Space => labels.sc_space,\n", "        Space => labels.sc_space,\n        Finish => labels.sc_finish,\n"],
    ['        (ScheduleField::Leaves, "double") => labels.sv_double.as_str().to_string(),\n', '        (ScheduleField::Leaves, "double") => labels.sv_double.as_str().to_string(),\n        (ScheduleField::Surface, "floor") => labels.sv_floor.as_str().to_string(),\n        (ScheduleField::Surface, "wall") => labels.sv_wall.as_str().to_string(),\n        (ScheduleField::Surface, "ceiling") => labels.sv_ceiling.as_str().to_string(),\n'],
  ]);
  edit(terminology, [
    ['    sf_usage: "Usage", "Nutzung";\n', '    sf_usage: "Usage", "Nutzung";\n    sf_surface: "Surface", "Bauteilseite";\n'],
    ['    sf_layer_mass: "Layer mass", "Schichtmasse";\n', '    sf_layer_mass: "Layer mass", "Schichtmasse";\n    sf_finish_area: "Finish area", "Belagsfläche";\n'],
    ['    sc_space: "Rooms", "Räume";\n', '    sc_space: "Rooms", "Räume";\n    sc_finish: "Room finishes", "Raumbeläge";\n'],
    ['    sv_double: "Double leaf", "Zweiflügelig";\n', '    sv_double: "Double leaf", "Zweiflügelig";\n    sv_floor: "Floor", "Boden";\n    sv_wall: "Walls", "Wände";\n    sv_ceiling: "Ceiling", "Decke";\n'],
  ]);
}
