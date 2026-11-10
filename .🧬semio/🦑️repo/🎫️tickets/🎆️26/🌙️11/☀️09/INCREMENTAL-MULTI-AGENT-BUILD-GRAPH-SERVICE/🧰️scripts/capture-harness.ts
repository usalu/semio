import{captureNxAuthorizerSourcesV1}from"/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const entry=process.argv[2]!;
try{const r=await captureNxAuthorizerSourcesV1("/home/deck/semio",entry,new AbortController().signal,{installed:true});console.log("ok",r.manifest.sources.length);}
catch(e:any){console.log("FAIL",e.message?.slice(0,300),JSON.stringify(e.lineage));}
