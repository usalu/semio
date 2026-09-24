const publisher = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts");
const outputs = await publisher.bundleFlowBrowserModule(false);
const text: string = await outputs[0].text();
const needle = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏃️runtime/🟨️.js";
const at = text.indexOf(needle);
console.log(process.cwd().slice(-20), at, JSON.stringify(text.slice(Math.max(0, at - 120), at + needle.length + 40)));
