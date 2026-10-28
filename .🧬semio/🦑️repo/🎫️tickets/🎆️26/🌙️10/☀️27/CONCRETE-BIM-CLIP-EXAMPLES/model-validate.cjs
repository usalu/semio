/** 🧪 Ticket script: validate the model JSON against its schema with AJV (strict). */
const fs = require("node:fs");
const path = require("node:path");
const Ajv = require("ajv/dist/2020").default;
const clip = path.resolve(__dirname, "../../../../../../../🎓️teaching/🏛️architecture/🏗️bim/🎬️clip");
const read = file => JSON.parse(fs.readFileSync(path.join(clip, file), "utf8"));
const validate = new Ajv({strict: true, allErrors: true}).compile(read("🧱️model/🧬️schema/🔣️.schema.json"));
const model = read("🧱️model/🔣️.json");
console.log("[DEBUG] valid:", validate(model));
if (validate.errors) console.log("[DEBUG]", JSON.stringify(validate.errors.slice(0, 12), null, 1));
const mutate = (name, edit) => {
  const copy = structuredClone(model);
  edit(copy);
  console.log("[DEBUG] mutation", name, "rejected:", !validate(copy));
};
mutate("bad guid length", m => { m.elements[0].guid = m.elements[0].guid.slice(1); });
mutate("unknown class", m => { m.elements[0].class = "IfcFoo"; });
mutate("missing basis", m => { delete m.elements[25].attributes.thickness.basis; });
mutate("negative thickness", m => { m.elements[25].attributes.thickness.value = -0.1; });
mutate("missing german", m => { delete m.building.name.de; });
mutate("extra key", m => { m.elements[0].extra = 1; });
