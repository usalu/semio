import Ajv from "ajv";
const roots = [
  "🧰️framework/🔨️modules/⏳️async/🛑️cancel/♻️retirement",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/📦️lease",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure",
];
for (const root of roots) {
  const schema = await Bun.file(`${root}/🧬️schema/🔣️.json`).json();
  const fixture = await Bun.file(`${root}/🧪️tests/🧫️fixtures/🔣️.json`).json();
  const validate = new Ajv({strict:true}).compile(schema);
  if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));
}
console.log(`custody schemas: ${roots.length} passed`);
