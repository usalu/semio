import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";

const require = createRequire(resolve("package.json"));
const { SaxesParser } = require("saxes");
const fixture = JSON.parse(await readFile(new URL("./idml-package-records.json", import.meta.url), "utf8"));
const packaging = "http://ns.adobe.com/AdobeInDesign/idml/1.0/packaging";
for (const test of fixture.cases) {
  const records = [];
  const parser = new SaxesParser({ xmlns: true });
  let depth = 0;
  let wrapped = false;
  parser.on("opentag", tag => {
    if (depth === 0) wrapped = tag.local === test.shape && tag.uri === packaging;
    if (tag.local === test.shape && tag.uri === "" && (depth === 0 || (wrapped && depth === 1))) {
      records.push(tag.attributes.Self?.value ?? "");
    }
    depth++;
  });
  parser.on("closetag", () => { depth--; });
  parser.write(test.xml).close();
  if (test.error) {
    assert.equal(test.error, "layout.idml.missing-identity");
    assert.ok(records.some(id => id.length === 0), test.xml);
  } else {
    assert.deepEqual(records, test.identities, test.xml);
  }
}
console.log("[DEBUG]", fixture.cases.length, "namespace-aware XML package cases");
