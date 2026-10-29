/** 📥 Pinned test-only IFC engine setup through portable Node APIs; no package scripts execute. */
import {readFile,writeFile,mkdir} from "node:fs/promises";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {gunzipSync} from "node:zlib";

const version="0.0.78";
const integrity="sha512-LGKMNYFVNyz9vVERN0iz+zvPQJqT7MEGBxslzcuuhwDDOA116LmJLKkg8sZ5SEvQJSc5NRVMWgpRNkDmuoQoJg==";

/** 🧰 Recreate the native reader from its authenticated official npm archive. */
export async function setupNativeIfcEngine(directory) {
  await mkdir(directory,{recursive:true});
  const archivePath=join(directory,"package.tgz");
  let archive;
  try {archive=await readFile(archivePath);} catch {
    const response=await fetch("https://registry.npmjs.org/web-ifc/-/web-ifc-"+version+".tgz");
    if(!response.ok) throw new Error("web-ifc archive HTTP "+response.status);
    archive=Buffer.from(await response.arrayBuffer());
    await writeFile(archivePath,archive);
  }
  if("sha512-"+createHash("sha512").update(archive).digest("base64")!==integrity) throw new Error("web-ifc package integrity mismatch");
  const tar=gunzipSync(archive),selected=new Set(["package/web-ifc-api-node.js","package/web-ifc-node.wasm","package/package.json"]);
  for(let offset=0;offset+512<=tar.length;) {
    const name=tar.subarray(offset,offset+100).toString().replace(/\0.*$/s,"");
    if(!name) break;
    const size=parseInt(tar.subarray(offset+124,offset+136).toString().replace(/\0.*$/s,"").trim(),8)||0;
    if(selected.has(name)) {
      const target=join(directory,name.substring(8));
      if(target.length>256) throw new Error("IFC engine path exceeds256 characters");
      await writeFile(target,tar.subarray(offset+512,offset+512+size));
      selected.delete(name);
    }
    offset+=512+Math.ceil(size/512)*512;
  }
  if(selected.size) throw new Error("IFC engine archive lacks expected files");
  return {modulePath:join(directory,"web-ifc-api-node.js"),wasmDirectory:directory,version,integrity};
}
