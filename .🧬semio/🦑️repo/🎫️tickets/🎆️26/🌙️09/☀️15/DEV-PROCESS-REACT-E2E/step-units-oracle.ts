import assert from "node:assert/strict";
import { parsePart21 } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🟦️.ts";
import fixture from "./browser-import-fixture.json";

const source = await Bun.file(fixture.step.source).text();
const padded = source.padEnd(fixture.maximumRawFileBytes, " ");
assert.equal(Buffer.byteLength(padded), fixture.maximumRawFileBytes);
const model = parsePart21(padded);
const declarations = model.instances.flatMap((instance) => instance.entities
  .filter((entity) => entity.name === "LENGTH_UNIT")
  .map(() => instance.entities.find((entity) => entity.name === "SI_UNIT")?.args));
assert.deepEqual(declarations, [[{ kind: "enum", value: "MILLI" }, { kind: "enum", value: "METRE" }]]);
console.log("[DEBUG] Independent Part21 parser accepts the padded AP214 box and resolves its millimetre declaration");
