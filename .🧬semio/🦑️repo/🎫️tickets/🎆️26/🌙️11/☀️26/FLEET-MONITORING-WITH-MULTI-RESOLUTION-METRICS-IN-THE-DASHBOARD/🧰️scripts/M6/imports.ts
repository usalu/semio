import{memoryUsage}from"node:process";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const F="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build";
const which=process.argv[2]!;
const map:Record<string,string>={
 machine:F+"/🏠️machine-state/🟦️.ts",gate:F+"/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts",host:F+"/📈️metrics/🛰️collector/🔌️host/🟦️.ts",
 unit:B+"/../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts",ledger:B+"/📈️counter-ledger/🟦️.ts",cas:B+"/🗄️cas/🗃️store/🟦️.ts",nx:B+"/../⚡️caching/🚀️bootstrap/📊️nx-cache/🟦️.ts",offload:B+"/🌐️remote-execution/🚀️offload/📊️counters/🟦️.ts",collector:B+"/🗄️cas/🧹️collector/🟦️.ts",shared:B+"/🌐️remote-execution/🛰️sidecar/🧰️shared/🟦️.ts"};
const before=memoryUsage.rss();await import(map[which]!);console.log(which,((memoryUsage.rss()-before)/1048576).toFixed(1),"MB");
