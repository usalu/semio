import assert from "node:assert/strict";
import { resolve } from "node:path";

const module = "🧰️framework/🔨️modules/🏃️process";
const contract = "📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
const [publicSurface, implementation] = await Promise.all([
  import(resolve(module, "📦️packages/🟦️typescript", contract)),
  import(resolve(module, contract)),
]);
assert.equal(typeof implementation.nativeImageFreshSpaceV1, "function");
assert.equal(publicSurface.nativeImageFreshSpaceV1, implementation.nativeImageFreshSpaceV1);
console.log("The Process package exposes its native fresh space function");
