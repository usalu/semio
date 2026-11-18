import {createHash} from "node:crypto";import {readFileSync,writeFileSync} from "node:fs";import {join} from "node:path";
const lib="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs";
const m=await import(lib);const root="/Users/ueli/Documents/semio";
const out={};
for(const family of ["generic","native"]){out[family+".inputs"]=m.nxBuildEngineInputsV1(family);}
out.external=m.nxBuildEngineExternalInputsV1();
const closure=m.nxBuildEngineClosureV1();out.closure=closure;
const rows={};for(const f of closure.files){if(f.endsWith(".json"))continue;rows[f]=m.nxBuildEngineReferencesV1(readFileSync(join(root,f),"utf8"));}
out.rows=rows;
for(const family of ["generic","native"])out[family+".digest"]=m.nxBuildEngineDigestV1(family);
writeFileSync(process.argv[2],JSON.stringify(out,null,1));
const h=x=>createHash("sha256").update(JSON.stringify(x)).digest("hex");
console.log("inputs",h([out["generic.inputs"],out["native.inputs"],out.external]),"rows",h(out.rows),"files",closure.files.length,"packages",closure.packages.join(","),"digests",out["generic.digest"].slice(0,12),out["native.digest"].slice(0,12));
