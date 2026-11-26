import{memoryUsage}from"node:process";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar";
const m=()=>(memoryUsage.rss()/1048576).toFixed(1);
// import ready WITHOUT its metrics import being counted: measure ready closure, then product metrics module itself (already part of it)
const before=memoryUsage.rss();
await import(B+"/📈️metrics/🟦️.ts");console.log("metrics product module alone",((memoryUsage.rss()-before)/1048576).toFixed(1),"MB");
const b2=memoryUsage.rss();
await import(B+"/🚦️ready/🟦️.ts");console.log("then the rest of ready adds",((memoryUsage.rss()-b2)/1048576).toFixed(1),"MB");
