import { readFileSync, writeFileSync, copyFileSync } from "node:fs";
import { join } from "node:path";
const root = process.argv[2];
const file = join(root, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🏗️frame/🦀️.rs");
let text = readFileSync(file, "utf8");
if (text.includes('key: "frame_fraction"')) { console.log("kept"); process.exit(0); }
const crlf = text.includes("\r\n");
text = text.replaceAll("\r\n", "\n");
const row = (key) => `    FieldRow { key: "${key}", label: |labels| labels.field_${key}, input: InputKind::Number, read: |s, id| s.curtain_wall_types.get(id).map(|row| row.${key}.map_or_else(String::new, number)), write: Some(|_, id, value| partial::<crate::mutations::set_type_thermal_data::SetTypeThermalData, _>(id, "${key}", &Assigned::new(parse_optional_number(value)?)).map(ModelMutation::SetTypeThermalData)), choices: None },\n`;
const anchor = "write: Some(write_type_mullion_material), choices: Some(material_choices) },\n];";
if (!text.includes(anchor)) throw new Error("anchor missing");
text = text.replace(anchor, () => anchor.slice(0, -2) + ["u_value", "g_value", "frame_fraction"].map(row).join("") + "];");
const tmp = file + ".new.tmp";
writeFileSync(tmp, crlf ? text.replaceAll("\n", "\r\n") : text);
copyFileSync(tmp, file);
console.log("edited");
