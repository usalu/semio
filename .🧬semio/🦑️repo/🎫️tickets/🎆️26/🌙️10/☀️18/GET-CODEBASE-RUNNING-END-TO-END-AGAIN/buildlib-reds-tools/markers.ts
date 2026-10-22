import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedImportsV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import selection from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🧭️selection/🧬️schema/🔣️.json";
import{existsSync}from"node:fs";
const root="/Users/ueli/Documents/semio",command="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/";
const markers=new Map<string,string[]>();
for(const path of Object.values(selection.properties.kernels.const as Record<string,string>)){
 const source=await captureNxAuthorizerSourcesV1(root,command+path,new AbortController().signal,{installed:true});
 for(const[file,map]of nxAuthorizerCapturedImportsV1(root,source))for(const[spec,target]of Object.entries(map))if(target.startsWith("installed:")){const list=markers.get(target)??[];list.push(file+" <- "+spec);markers.set(target,list);}
}
for(const[m,list]of markers)console.log(m,existsSync(root+"/node_modules/"+m.slice(10,m.lastIndexOf("@"))+"/package.json"),list.length,list[0]);
