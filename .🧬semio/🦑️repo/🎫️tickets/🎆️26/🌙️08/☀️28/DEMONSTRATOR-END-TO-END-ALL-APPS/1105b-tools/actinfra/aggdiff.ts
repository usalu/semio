import {readFileSync} from "node:fs";
import {canonicalTestAggregatePolicyV1 as p,deriveCanonicalTestAggregateV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🧮️aggregate/🟦️.ts";
const proj=JSON.parse(readFileSync("/Users/ueli/Documents/semio/"+p.root+"/📋️project.json","utf8"));
const e=deriveCanonicalTestAggregateV1({name:p.owner,root:p.root,targets:proj.targets}),a=proj.targets[p.target];
for(const k of new Set([...Object.keys(e),...Object.keys(a)])){if(JSON.stringify(e[k])!==JSON.stringify(a[k])){if(Array.isArray(e[k])){console.log(k,"missing",e[k].filter((x:string)=>!a[k].includes(x)),"extra",a[k].filter((x:string)=>!e[k].includes(x)),"order",JSON.stringify(e[k].filter((x:string)=>a[k].includes(x)))===JSON.stringify(a[k].filter((x:string)=>e[k].includes(x))))}else console.log(k,JSON.stringify(e[k]),JSON.stringify(a[k]))}}
const d=e.dependsOn as string[],c=a.dependsOn as string[];let i=0;while(d[i]===c[i])i++;console.log("first diff",i,d.slice(i-1,i+4),c.slice(i-1,i+4));
const k=d.indexOf("test-build-preflight");console.log("PLACE",d[k-1],"|",d[k+1]);
