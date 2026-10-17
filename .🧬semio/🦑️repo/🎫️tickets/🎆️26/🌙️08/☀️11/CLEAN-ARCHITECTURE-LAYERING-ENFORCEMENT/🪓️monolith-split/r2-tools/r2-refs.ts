import { readFileSync } from "node:fs";
const m = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const f = process.argv[2]!;
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { fixturesToRuntime } = await import(`${L}/🧪️test-closure/🟦️.ts`);
const text = fixturesToRuntime(readFileSync(f, "utf8"), f, "/Users/ueli/Documents/semio")!;
const { inspectRustCompileReferences } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts").catch(() => ({} as any));
console.log(Object.keys(m).join(","), typeof inspectRustCompileReferences);
if (inspectRustCompileReferences) console.log(JSON.stringify([...inspectRustCompileReferences(text)]).slice(0, 600));
