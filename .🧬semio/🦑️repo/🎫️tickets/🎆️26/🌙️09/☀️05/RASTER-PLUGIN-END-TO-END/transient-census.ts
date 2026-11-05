import{readdirSync,readFileSync}from"node:fs";
let count=0;let missing=0;
function walk(p:string){for(const e of readdirSync(p,{withFileTypes:true})){if(e.name==="node_modules"||e.name==="target"||e.name.startsWith("."))continue;const q=p+"/"+e.name;if(e.isDirectory())walk(q);else if(e.name.endsWith(".rs")){const s=readFileSync(q,"utf8");if(/impl[^\n]*WindowTransientOwner for/.test(s)){count++;if(!s.includes("fn owners_birth_demands")){missing++;console.log(JSON.stringify({missing:q}));}}}}}
walk("✏️s");
console.log(JSON.stringify({count,missing}));
