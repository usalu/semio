import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {isDeepStrictEqual} from "node:util";
import {renderRustCatalog} from "./old-render.ts";
const IFC="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc";
const {generateCatalog,packCatalogFile}=await import(IFC+"/📚️catalog/🛠️extract/📁️generate.ts");
const {decodeCatalogPack,encodeCatalogPayload}=await import(IFC+"/📚️catalog/🛠️extract/📦️pack.ts");
const owners={"2x3":["🔖️2x3/🪆️subsets/🧱️base","IFC2X3_TC1.exp"],"4":["4️⃣4/🪆️subsets/✳️any","IFC4.exp"],"4x3":["🔖️4x3/🪆️subsets/✳️any","IFC4X3_ADD2.exp"]} as const;
const write=process.argv[2]==="write";
for(const [id,[dir,exp]] of Object.entries(owners)){
 const owner=`${IFC}/🏅️standards/${dir}/🧬️schema/📚️catalog`,old=existsSync(owner+"/data.rs")?readFileSync(owner+"/data.rs","utf8"):null;
 const catalog=JSON.parse(readFileSync(owner+"/🔣️.json","utf8"));
 if(old!==null){const same=renderRustCatalog(catalog)===old;console.log(id,"old data.rs sha256",createHash("sha256").update(old).digest("hex"),"json->render==data.rs",same);if(!same)throw Error("oracle mismatch "+id);}
 const out=write?owner+"/catalog.ifcpack":"/tmp/ifcpack-"+id;
 await packCatalogFile({json:owner+"/🔣️.json",pack:out});
 const bytes=new Uint8Array(readFileSync(out)),decoded=decodeCatalogPack(bytes),{validation,...retained}=catalog;
 let extracted="skipped";
 try{await generateCatalog({source:owner+"/"+exp,manifest:owner+"/source.json",json:"/tmp/ifcjson-"+id,pack:"/tmp/ifcpack-x-"+id});extracted=String(readFileSync("/tmp/ifcjson-"+id,"utf8")===readFileSync(owner+"/🔣️.json","utf8"));}catch(e){extracted="error "+e}
 console.log(id,"pack bytes",bytes.length,"payload",encodeCatalogPayload(retained).length,"decoded==json",isDeepStrictEqual(JSON.parse(JSON.stringify(decoded)),JSON.parse(JSON.stringify(retained))),"extractor reproduces committed json:",extracted);
}
