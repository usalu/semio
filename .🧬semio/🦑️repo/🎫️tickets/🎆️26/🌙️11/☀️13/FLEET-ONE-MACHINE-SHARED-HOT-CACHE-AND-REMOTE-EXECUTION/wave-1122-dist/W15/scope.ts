/** 🔑️ W15: the scope key of the Ready workspace selection (selection kinds tried). */
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const a=await import(B+"🕸️graph/🚪️admission/🟦️.ts");
const P=["@semio-tech/norm-din4108","@semio-tech/norm-din4108-rs","repo-mcp"];
for(const s of [{kind:"tasks",tasks:P.map(project=>({project,target:"build"}))},{kind:"owners",projects:P,exclude:[],targets:["build"]},{kind:"owners",projects:P,exclude:[]}]){try{console.log(a.buildGraphScopeKeyV1(s as never),JSON.stringify(s).slice(0,80));}catch(e){console.log("bad",String(e).slice(0,80));}}
