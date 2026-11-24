const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
for(const p of ["/🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts","/🌐️remote-cache/🏠️hosted/🟦️.ts","/🌐️remote-cache/⌨️command/🟦️.ts","/🌐️remote-cache/💾️store/🟦️.ts","/🌐️remote-cache/🖥️server/🟦️.ts"]){const m=await import(B+p);console.log("ok",p,Object.keys(m).length);}
const policy=(await import(B+"/🔣️policy.json",{with:{type:"json"}})).default.remoteCache?.server??undefined;
console.log(JSON.stringify(policy?.graph));
