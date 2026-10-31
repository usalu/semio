import {join} from "node:path";
import {pathToFileURL} from "node:url";
const api=await import(pathToFileURL(join(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts")).href);
for(const extension of ["ts","tsx","jsx"]){
 const source="import {value} from './value';\nexport const view = "+(extension==="ts"?"value;":"<section>{value}</section>;");
 let actual:unknown;
 try{actual=api.closureLiteralSitesV1("/fixture."+extension,source);}catch(e){actual={name:e.name,message:e.message};}
 const oracle=new Bun.Transpiler({loader:extension as "ts"|"tsx"|"jsx",target:"node"}).scan(source).imports;
 console.log(JSON.stringify({extension,actual,oracle}));
}
