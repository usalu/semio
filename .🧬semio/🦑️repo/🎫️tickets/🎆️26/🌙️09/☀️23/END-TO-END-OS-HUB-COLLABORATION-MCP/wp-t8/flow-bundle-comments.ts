const publisher = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts");
const text: string = await (await publisher.bundleFlowBrowserModule(false))[0].text();
console.log(process.cwd().slice(-20), text.match(/\/\* [^*]*\.js \*\//gu), Bun.hash(text));
