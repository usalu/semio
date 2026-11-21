import assert from "node:assert/strict";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";

const fixture = await Bun.file(process.argv[2]).json();
const { worldCameraSetCameraDispatchArgs } = await import(pathToFileURL(resolve(process.argv[3])).href);
const coordinate = { type: "array", minItems: 3, maxItems: 3, items: { type: "number" } };
const validate = new Ajv().compile({
  type: "object",
  additionalProperties: false,
  required: ["windowId", "camera"],
  properties: {
    windowId: { type: "string", minLength: 1 },
    camera: {
      type: "object",
      additionalProperties: false,
      required: ["position", "target"],
      properties: { position: coordinate, target: coordinate, up: coordinate, zoom: { type: "number" } },
    },
  },
});
const rendererArgs = worldCameraSetCameraDispatchArgs(fixture.args.windowId, fixture.args.camera);
assert.deepEqual(rendererArgs, fixture.args);
assert.equal(validate(rendererArgs), true);
assert.equal(validate({ ...rendererArgs, camera: fixture.malformedCamera }), false);
assert.equal(validate({ windowId: fixture.args.windowId, ...fixture.args.camera }), false);
console.log("[DEBUG] Ajv accepts the actual nested renderer camera command and refuses malformed and flat poses");
