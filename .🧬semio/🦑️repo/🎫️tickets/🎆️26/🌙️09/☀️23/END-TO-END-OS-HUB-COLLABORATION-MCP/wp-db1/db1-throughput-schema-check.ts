/** 🧬️ DB1 one-off: validates the throughput fixture (and two mutated copies) against `ThroughputV1` with the repo's AJV. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const engine = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine");
const doc = JSON.parse(readFileSync(join(engine, "🧬️schema/🔣️.json"), "utf8"));
const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(doc).getSchema(`${doc.$id}#/$defs/ThroughputV1`)!;
const fixture = JSON.parse(readFileSync(join(engine, "🧫️fixtures/⏱️throughput/🔣️.json"), "utf8"));
console.log("fixture valid:", validate(fixture), JSON.stringify(validate.errors));
const legacy = { ...fixture, bounds: { ...fixture.bounds, welcomeStormToSoloRatioMax: 8 } };
console.log("unknown bound rejected:", !validate(legacy));
const loose = { ...fixture, bounds: { ...fixture.bounds, stormToSerialRatioMax: 1.5 } };
console.log("storm/serial > 1 rejected:", !validate(loose));
