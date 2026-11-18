
/** 📏️ Contract: a ceiling on a collection that grows with the workspace never falls below the workspace again. Every such ceiling is a declared member of the workspace scale family, holds the declared workspace size times the safety factor, and a workspace-named ceiling that is neither a member nor exempt with a reason fails the scan. */
const scaleFamily=scaleDeclaration.definitions.Family.const;
const scaleLibrary=fileURLToPath(new URL("../../../../../../",import.meta.url));
const scaleRead=(file:string)=>JSON.parse(readFileSync(scaleLibrary+file,"utf8"));
const scalePointer=(file:string,pointer:string)=>pointer.split("/").filter(Boolean).reduce((node:any,key)=>node[key],scaleRead(file));
const scaleRequired=(expression:string)=>Math.ceil(expression.split("*").reduce((product,name)=>product*(scaleFamily.workspace as Record<string,number>)[name]!,1)*scaleFamily.safetyFactor);

test("every declared ceiling of a workspace-scaled collection holds the declared workspace size times the safety factor",()=>{
 const failures:string[]=[];
 for(const row of scaleFamily.members){const value=scalePointer(row.file,row.pointer)[row.key],required=scaleRequired(row.scale);if(!(Number.isInteger(value)&&value>=required))failures.push(row.file+" "+row.pointer+" "+row.key+"="+value+" < "+required+" ("+row.scale+" x "+scaleFamily.safetyFactor+")");}
 expect(failures,"ceilings below the workspace; raise them or derive them from the declared workspace").toEqual([]);
});

test("ceilings of the same documents are one value",()=>{
 for(const group of scaleFamily.equal){const values=group.map(index=>{const row=scaleFamily.members[index]!;return scalePointer(row.file,row.pointer)[row.key];});expect(new Set(values).size,JSON.stringify(values)).toBe(1);}
});

test("a workspace-named ceiling below the owners times the safety factor is a member or exempt with a reason in both languages",()=>{
 const bar=scaleRequired("owners"),declared=new Set<string>(),undeclared:string[]=[];
 for(const row of scaleFamily.members)declared.add(row.file+"#"+row.pointer+"#"+row.key);
 for(const row of scaleFamily.exempt){declared.add(row.file+"#"+row.pointer+"#"+row.key);expect(scalePointer(row.file,row.pointer)[row.key],row.file+" "+row.pointer).toBeLessThan(bar);expect(row.reason.en.length&&row.reason.de.length,row.pointer).toBeGreaterThan(0);}
 const numbers=/^(maximum(Owners|Projects|Documents|Closures|Nodes|Suspects)|entryMaximumDocuments|closureStoreMaximum\w+|outputSignatureMaximumEntries|memoMaximumEntries|digestCacheMaximumFiles|journalMaximumEvents)$/u,arrays=/^(projects|owners|documents|closures)$/u;
 const walk=(node:any,pointer:string[],file:string)=>{
  if(Array.isArray(node)){node.forEach((value,index)=>walk(value,[...pointer,String(index)],file));return;}
  if(!node||typeof node!=="object")return;
  for(const[key,value]of Object.entries<any>(node)){
   if(Number.isInteger(value)&&numbers.test(key)&&value>=64&&value<bar&&!declared.has(file+"#"+pointer.join("/")+"#"+key))undeclared.push(file+" "+pointer.join("/")+" "+key+"="+value);
   if(value&&typeof value==="object"&&!Array.isArray(value)&&arrays.test(key))for(const limit of["maxItems","maximum"])if(Number.isInteger(value[limit])&&value[limit]>=64&&value[limit]<bar&&!declared.has(file+"#"+[...pointer,key].join("/")+"#"+limit))undeclared.push(file+" "+[...pointer,key].join("/")+" "+limit+"="+value[limit]);
   walk(value,[...pointer,key],file);
  }
 };
 for(const sub of["🏗️build","⚡️caching"])for(const rel of readdirSync(scaleLibrary+sub,{recursive:true})as string[]){if(!rel.endsWith(".json")||rel.includes("🧪️tests")||rel.includes("🤖️generated")||rel.includes("node_modules")||rel.includes("fixtures"))continue;let value:unknown;try{value=scaleRead(sub+"/"+rel);}catch{continue;}walk(value,[],sub+"/"+rel);}
 expect(undeclared,"ceilings named for a workspace collection that sit below the owners times the safety factor; add them to the workspace scale family as members or exempt").toEqual([]);
});
