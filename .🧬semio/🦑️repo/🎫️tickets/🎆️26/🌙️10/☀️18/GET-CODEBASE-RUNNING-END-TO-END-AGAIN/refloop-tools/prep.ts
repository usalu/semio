/** ⏱️ Times the graph verifier's analysis preparation in a fresh process and lists whether the declared-import provider is registered after it. */
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const t=performance.now();
const producer=await import(L+"/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
console.log("[DEBUG] prepared ms",Math.round(performance.now()-t),"rss",Math.round(process.memoryUsage().rss/1048576));
