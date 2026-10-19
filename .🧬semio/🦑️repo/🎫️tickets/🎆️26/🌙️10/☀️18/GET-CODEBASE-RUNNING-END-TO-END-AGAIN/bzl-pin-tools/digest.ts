import{readFileSync}from"node:fs";
import{createHash}from"node:crypto";
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📋️contract/🔣️.json";
const corpus=JSON.parse(readFileSync(B,"utf8"));
function normalized(value:any,prefix:string):any{if(Array.isArray(value))return value.map(r=>normalized(r,prefix));if(value&&typeof value==="object")return Object.fromEntries(Object.entries(value).filter(([k])=>k!=="$schema"&&k!=="$id").sort(([a],[b])=>a<b?-1:a>b?1:0).map(([k,r])=>[k,k==="$ref"&&typeof r==="string"&&r.startsWith(prefix)?"#/"+r.slice(prefix.length):normalized(r,prefix)]));return value;}
for(const row of corpus.contracts){const c=JSON.parse(readFileSync(row.component,"utf8"));const v=c.definitions?.[row.definition];const e=JSON.stringify(normalized(v,"#/definitions/"+row.definition+"/"));const d=e===undefined?undefined:createHash("sha256").update(e).digest("hex");console.log(row.definition,d===row.sha256?"ok":"CHANGED "+d);}
