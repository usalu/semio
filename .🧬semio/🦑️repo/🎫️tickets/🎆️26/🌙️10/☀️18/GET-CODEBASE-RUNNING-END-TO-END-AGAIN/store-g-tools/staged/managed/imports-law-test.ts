import{test,expect}from"bun:test";
import{existsSync,readdirSync,readFileSync}from"node:fs";
import{dirname,join,normalize,sep}from"node:path";
const storage=join(import.meta.dir,"../.."),packages=join(import.meta.dir,"../../../../../../📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage");
const spec=/(?:from|import)\s*["'](\.[^"']+)["']/gu;
test("every relative import of the storage module and of its public facades resolves to a file",()=>{
 const missing:string[]=[];
 for(const root of[storage,packages])for(const file of readdirSync(root,{recursive:true,encoding:"utf8"}).filter(path=>/\.ts$/u.test(path)&&!path.split(sep).some(part=>part==="node_modules"||part.includes("🧪️tests")))){
  const text=readFileSync(join(root,file),"utf8");
  for(const match of text.matchAll(spec)){const target=normalize(join(dirname(join(root,file)),match[1]!));if(!existsSync(target)&&!existsSync(target+".ts"))missing.push(file+" -> "+match[1]);}
 }
 expect(missing).toEqual([]);
});
