import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
const require = createRequire(import.meta.url), a = require("@iarna/toml"), b = require("smol-toml");
const root = path.resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
function read(relative) { const file = path.join(root, relative, "Cargo.toml"), source = fs.readFileSync(file, "utf8"), value = a.parse(source); assert.deepEqual(value, b.parse(source)); return {file, value}; }
const owner = read("🚪️io/📦️packages/🦀️rust"), dependent = read("🧩️composition/🚪️open/📦️packages/🦀️rust");
const canonical = owner.value.package.name;
assert.equal(canonical, "semio-os-store-text-io");
const bindings = Object.entries(dependent.value["dev-dependencies"]).filter(([key, value]) => value && typeof value === "object" && value.path && path.resolve(path.dirname(dependent.file), value.path) === path.dirname(owner.file));
assert.equal(bindings.length, 1);
assert.equal(bindings[0][1].package ?? bindings[0][0], canonical, "path binding must name its actual package");
assert.equal(bindings[0][0], canonical, "dependency key must use current canonical name");
console.log(JSON.stringify({parsers:2, canonical, binding:bindings[0][0], resolved:path.dirname(owner.file)}));
for (const file of ["🦀️.rs", "🧪️testing/🧭️prelude/🦀️.rs"]) assert(!fs.readFileSync(path.join(root, file), "utf8").includes("semio_framework_os_store_text_io"), file);
