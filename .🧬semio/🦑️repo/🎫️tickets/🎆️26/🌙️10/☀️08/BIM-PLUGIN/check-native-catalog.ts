import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const registry = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
const { emitNativeCatalogV1 } = await import(pathToFileURL(resolve(registry,"📽️projection/🟦️.ts")).href);
const catalog = JSON.parse(readFileSync(resolve("✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🦀️catalog.json"), "utf8"));
assert.deepEqual(JSON.parse(emitNativeCatalogV1(catalog.plugins, catalog.playgrounds)), catalog);
const fixture = JSON.parse(readFileSync(resolve(registry,"📥️runtime/🧪️tests/🧫️fixtures/🔣️.json"),"utf8")).alternate;
fixture.plugins[1].activationEvents = fixture.plugins[0].activationEvents;
assert.throws(() => emitNativeCatalogV1(fixture.plugins,fixture.playgrounds), /activation ownership refused/);
console.log(catalog.plugins.length, catalog.playgrounds.length);
