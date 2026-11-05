
/** 🪞️ Independent JSON projection check of the committed cancellation metadata. */
import assert from "node:assert/strict";
import fs from "node:fs";
import Ajv from "ajv";
const base = "🌎️hub/🧩️compositions/💠️lowpoly/";
const fixture = JSON.parse(fs.readFileSync(base+"🧪️tests/🔬️surface/cancellation.json","utf8"));
const schema = JSON.parse(fs.readFileSync(base+"🧪️tests/🔬️surface/cancellation.schema.json","utf8"));
const validate = new Ajv({strict:true}).compile(schema);
assert.ok(validate(fixture),JSON.stringify(validate.errors));
const descriptor = JSON.parse(fs.readFileSync(base+"🧩️component/🔣️.json","utf8"));
const app = descriptor.manifest.apps.find(app=>app.id===fixture.controller);
assert.ok(app);
for (const id of fixture.tools) {
 const actions = [...app.actions,...app.windowKinds.flatMap(window=>window.actions)].filter(action=>action.id===id);
 assert.ok(actions.length>0,id+" declared");
 assert.ok(actions.every(action=>action.semantics.execution.cancellable===fixture.cancellable),id+" cancellable");
}
console.log("committed descriptor cancellation projection passed");
