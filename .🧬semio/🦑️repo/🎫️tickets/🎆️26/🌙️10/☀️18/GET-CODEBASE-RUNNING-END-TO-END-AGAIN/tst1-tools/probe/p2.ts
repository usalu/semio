const d="/Users/ueli/Documents/semio/node_modules/.bun/jsonc-parser@3.3.1/node_modules/jsonc-parser";
try{console.log(Bun.resolveSync("jsonc-parser", d))}catch(e){console.log("fail1",String(e))}
try{console.log(Bun.resolveSync(d, "/"))}catch(e){console.log("fail2",String(e))}
try{console.log(Bun.resolveSync("jsonc-parser", d+"/..") )}catch(e){console.log("fail3",String(e))}
try{console.log(Bun.resolveSync("jsonc-parser", d+"/.."+"/..") )}catch(e){console.log("fail4",String(e))}
