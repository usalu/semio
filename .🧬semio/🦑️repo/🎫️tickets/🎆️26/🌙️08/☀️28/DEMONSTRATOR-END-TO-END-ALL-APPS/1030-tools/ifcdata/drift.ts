import {readFileSync} from "node:fs";
const IFC="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc";
const {extractExpressCatalog}=await import(IFC+"/📚️catalog/🛠️extract/🟦️.ts");
const o=IFC+"/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/";
const c=JSON.parse(readFileSync(o+"🔣️.json","utf8")),x=extractExpressCatalog(readFileSync(o+"IFC4X3_ADD2.exp","utf8"),JSON.parse(readFileSync(o+"source.json","utf8")));
const diff=(a:any,b:any,path:string,out:string[])=>{if(JSON.stringify(a)===JSON.stringify(b))return;if(a&&b&&typeof a==="object"&&typeof b==="object"&&!Array.isArray(a)){for(const k of new Set([...Object.keys(a),...Object.keys(b)]))diff(a[k],b[k],path+"."+k,out);}else if(Array.isArray(a)&&Array.isArray(b)&&a.length===b.length){a.forEach((v,i)=>diff(v,b[i],path+"["+i+"]",out));}else out.push(path+" :: "+JSON.stringify(a)?.slice(0,160)+" || "+JSON.stringify(b)?.slice(0,160));};
for(const k of ["schema","version","source","entities","types","functions","rules"]){const out:string[]=[];diff((c as any)[k],(x as any)[k],k,out);console.log(k,out.length);out.slice(0,Number(process.env.N??4)).forEach(l=>console.log("  ",l));}
const show=(a:string,b:string)=>{let i=0;while(a[i]===b[i])i++;console.log(JSON.stringify(a.slice(Math.max(0,i-60),i+120)));console.log(JSON.stringify(b.slice(Math.max(0,i-60),i+120)));};
if(process.env.SHOW){for(const k of ["IFCCONSTRAINTSPARAMBSPLINE","IFCGETBASISSURFACE","IFCMAKEARRAYOFARRAY"])show((c as any).functions[k],(x as any).functions[k]);show((c as any).entities.IFCMIRROREDPROFILEDEF.derived[0].expression,(x as any).entities.IFCMIRROREDPROFILEDEF.derived[0].expression);}
