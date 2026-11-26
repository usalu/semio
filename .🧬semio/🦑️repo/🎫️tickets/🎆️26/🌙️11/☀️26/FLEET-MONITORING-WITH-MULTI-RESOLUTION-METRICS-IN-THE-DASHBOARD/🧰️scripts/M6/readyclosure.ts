import{memoryUsage}from"node:process";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar";
const m=()=>(memoryUsage.rss()/1048576).toFixed(1);
console.log("bare",m());
await import(B+"/🚦️ready/🟦️.ts");console.log("after ready module (it already imports the metrics starter)",m());
const t=Date.now();const keys=Object.keys(require.cache??{}).length;
console.log("modules",keys);
