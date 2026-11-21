import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

const root = new URL("../../../../../../../", import.meta.url);
const read = (path) => JSON.parse(readFileSync(new URL(path, root), "utf8"));
const standard = read("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🗺️overlay/🧬️schema/🔣️.json");
const scene = read("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧭️terrain-descriptor/🔣️.json");
const actual = read(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/🗑️generated/terrain-scene.json").terrain;
const ajv = new Ajv2020({ formats: { uri: { type: "string", validate: (uri) => URL.canParse(uri) } } });
ajv.addSchema(standard);
const validate = ajv.compile(scene);
assert.equal(validate(actual), true, JSON.stringify(validate.errors));
assert.equal(validate({ ...actual, projectOriginLat: 86 }), false, "polar terrain origin");
const outside = { id: "outside", name: "Outside", coordinates: [0, -86] };
assert.equal(validate({ ...actual, overlays: { ...actual.overlays, positions: [outside] } }), false, "polar overlay position");
console.log("[DEBUG] Rust terrain scene satisfies the shared 2020-12 overlay schema and rejects latitudes outside Web Mercator");
