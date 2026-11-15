import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{appendFileSync,copyFileSync,existsSync,linkSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
import Ajv from"ajv";
import schema from"../../../../🧬️schema/🔣️.json";
import mountSchema from"../../../📋️contract/📂️mount/🧬️schema/🔣️.json";
import relocationCorpus from"./🔣️.json";
import*as boot from"../../🟦️.ts";

test("a verified move on the volume re-binds the original image and nothing else, with an exact SQLite oracle",()=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original test artifact directory required");mkdirSync(base,{recursive:true});
 const ajv=new Ajv({strict:true}).addSchema(mountSchema).addSchema(schema),validCase=ajv.compile({$ref:schema.$id+"#/definitions/AncestryRelocationCase"}),validReceipt=ajv.compile({$ref:schema.$id+"#/definitions/ImageReceipt"}),validResult=ajv.compile({$ref:schema.$id+"#/definitions/AncestryRebinding"}),database=new Database(":memory:"),chain=(directory:string):string[]=>{const rows:string[]=[];for(let path=dirname(directory);;path=dirname(path)){rows.push(String(lstatSync(path,{bigint:true}).ino));if(dirname(path)===path)break;}return rows;},equal=(left:unknown,right:unknown):boolean=>Boolean((database.query("SELECT json_array_length(:left)=json_array_length(:right) AND NOT EXISTS(SELECT 1 FROM json_each(:left) AS l LEFT JOIN json_each(:right) AS r ON l.key=r.key WHERE l.value IS NOT r.value) AS same").get({":left":JSON.stringify(left),":right":JSON.stringify(right)})as{same:number}).same);
 try{for(const row of relocationCorpus){expect(Boolean(validCase(row)),row.name).toBe(true);
  const area=realpathSync(mkdtempSync(join(resolve(base),"rebind-"))),oldWork=join(area,"old-home","work"),newWork=join(area,"new-home","sub","work"),inner=join(".cache","b","d"),before=join(oldWork,inner),after=join(newWork,inner);
  try{mkdirSync(join(before,"m"),{recursive:true});mkdirSync(join(area,"elsewhere","work"),{recursive:true});writeFileSync(join(before,"fixed.dmg"),Buffer.alloc(4096,7));
   const backing=lstatSync(join(before,"fixed.dmg")),mount=lstatSync(join(before,"m")),authored={version:1,mountDirectory:{dev:String(mount.dev),ino:String(mount.ino),mode:mount.mode},bootSession:"1E4A0CBE-3710-481F-9614-F8F1FCBD61D0",dev:backing.dev,ino:backing.ino,size:backing.size,ancestry:[...boot.readNativeImageAncestryV1(before)],filesystemBytes:33554432,maximumPhysicalBytes:34603008};
   expect(validReceipt(authored),row.name).toBe(true);writeFileSync(join(before,"image.json"),JSON.stringify(authored)+"\n",{mode:0o600});
   let directory=after,movedFrom=oldWork,movedTo=newWork;
   if(row.fault==="already-current"){directory=before;movedFrom=newWork;movedTo=oldWork;}
   else if(row.anchor==="worktree"){mkdirSync(dirname(newWork),{recursive:true});renameSync(oldWork,newWork);}
   else{mkdirSync(dirname(after),{recursive:true});renameSync(before,after);movedFrom=before;movedTo=after;}
   const bound=()=>boot.requireNativeImageAncestryBindingV1({published:authored.ancestry,current:boot.readNativeImageAncestryV1(directory)});
   if(row.fault==="already-current")bound();else expect(bound,row.name).toThrow("Original Native image ancestry changed");
   const image=join(directory,"fixed.dmg"),receipt=join(directory,"image.json"),root=join(directory,"m");
   if(row.fault==="copied-image"){copyFileSync(image,image+".copy");renameSync(image+".copy",image);}
   if(row.fault==="linked-image")linkSync(image,join(area,"second-name"));
   if(row.fault==="resized-image")appendFileSync(image,"x");
   if(row.fault==="foreign-ancestry")writeFileSync(receipt,JSON.stringify({...authored,ancestry:authored.ancestry.map((ino,index)=>index===authored.ancestry.length-1?ino:String(BigInt(ino)+1n))})+"\n");
   if(row.fault==="other-origin")movedFrom=join(area,"elsewhere","work");
   if(row.fault==="aliased-origin")symlinkSync(newWork,oldWork,process.platform==="win32"?"junction":"dir");
   if(row.fault==="attached-root")writeFileSync(join(root,"mounted"),"");
   if(row.fault==="replaced-root"){rmSync(root,{recursive:true});mkdirSync(join(directory,"other"));mkdirSync(root);}
   if(row.fault==="pending-publication")mkdirSync(join(directory,schema.definitions.ImageProtocol.const.publicationDirectory));
   if(row.fault==="malformed-receipt"){const{bootSession:_,...partial}=authored;writeFileSync(receipt,JSON.stringify(partial)+"\n");}
   const bytes=readFileSync(receipt),call=()=>boot.rebindNativeImageAncestryV1({directory,movedFrom,movedTo});
   if(!row.accepted){expect(call,row.name).toThrow(row.failure!);expect(readFileSync(receipt).equals(bytes),row.name).toBe(true);expect(readdirSync(directory).some(name=>name.endsWith(schema.definitions.AncestryRebindProtocol.const.temporarySuffix)),row.name).toBe(false);continue;}
   const result=call(),current=chain(directory),written=JSON.parse(readFileSync(receipt,"utf8"));
   expect(validResult(result),row.name).toBe(true);expect(result.status,row.name).toBe(row.fault==="already-current"?"current":"rebound");expect(validReceipt(written),row.name).toBe(true);
   expect(equal(written.ancestry,current),row.name).toBe(true);expect(equal(result.published,authored.ancestry),row.name).toBe(true);expect(equal(result.current,current),row.name).toBe(true);expect(equal(authored.ancestry,current),row.name).toBe(row.fault==="already-current");
   expect({...written,ancestry:authored.ancestry},row.name).toEqual(authored);expect(lstatSync(receipt).mode&0o777,row.name).toBe(0o600);expect(lstatSync(image).ino,row.name).toBe(backing.ino);expect(readdirSync(directory).sort(),row.name).toEqual(["fixed.dmg","image.json","m"]);
   boot.requireNativeImageAncestryBindingV1({published:written.ancestry,current:boot.readNativeImageAncestryV1(directory)});
   const again=call();expect(again.status,row.name).toBe("current");expect(readFileSync(receipt,"utf8"),row.name).toBe(JSON.stringify(written)+"\n");
   if(row.fault==="none"){copyFileSync(image,image+".copy");renameSync(image+".copy",image);expect(call,row.name).toThrow("Original Native image backing required");}
  }finally{rmSync(area,{recursive:true,force:true});}}}
 finally{database.close();}
});

test("an inspected image binds again only through a relocation hint of its own move",()=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original test artifact directory required");mkdirSync(base,{recursive:true});
 const ajv=new Ajv({strict:true}).addSchema(mountSchema).addSchema(schema),validHint=ajv.compile({$ref:schema.$id+"#/definitions/AncestryRelocationHint"}),name=schema.definitions.AncestryRebindProtocol.const.hintName;
 for(const layout of[join(".cache","b","d"),join(".cache","b","n","m","shard","native")])for(const mode of["hint","no-hint","copied","foreign-hint","malformed-hint"]as const){
  const area=realpathSync(mkdtempSync(join(resolve(base),"admit-"))),oldWork=join(area,"old","work"),newWork=join(area,"new","deeper","work"),before=join(oldWork,layout),directory=join(newWork,layout),receipt=join(directory,"image.json"),label=layout+" "+mode;
  try{mkdirSync(join(before,"m"),{recursive:true});mkdirSync(join(area,"elsewhere"),{recursive:true});writeFileSync(join(before,"fixed.dmg"),Buffer.alloc(4096,7));
   const backing=lstatSync(join(before,"fixed.dmg")),mount=lstatSync(join(before,"m")),authored={version:1,mountDirectory:{dev:String(mount.dev),ino:String(mount.ino),mode:mount.mode},bootSession:"1E4A0CBE-3710-481F-9614-F8F1FCBD61D0",dev:backing.dev,ino:backing.ino,size:backing.size,ancestry:[...boot.readNativeImageAncestryV1(before)],filesystemBytes:33554432,maximumPhysicalBytes:34603008};
   writeFileSync(join(before,"image.json"),JSON.stringify(authored)+"\n",{mode:0o600});boot.admitNativeImageRelocationV1(before);expect(readFileSync(join(before,"image.json"),"utf8"),label).toBe(JSON.stringify(authored)+"\n");
   mkdirSync(dirname(newWork),{recursive:true});renameSync(oldWork,newWork);
   const home=join(newWork,".cache","b"),bound=()=>boot.requireNativeImageAncestryBindingV1({published:JSON.parse(readFileSync(receipt,"utf8")).ancestry,current:boot.readNativeImageAncestryV1(directory)});
   if(mode==="hint"||mode==="copied"){boot.recordNativeImageRelocationV1({directory:home,movedFrom:oldWork,movedTo:newWork});expect(validHint(JSON.parse(readFileSync(join(home,name),"utf8"))),label).toBe(true);expect(readdirSync(home).filter(entry=>entry.startsWith(name)),label).toEqual([name]);}
   if(mode==="foreign-hint")boot.recordNativeImageRelocationV1({directory:home,movedFrom:join(area,"elsewhere"),movedTo:newWork});
   if(mode==="malformed-hint")writeFileSync(join(home,name),JSON.stringify({version:1,movedFrom:oldWork})+"\n");
   if(mode==="copied"){copyFileSync(join(directory,"fixed.dmg"),join(directory,"copy"));renameSync(join(directory,"copy"),join(directory,"fixed.dmg"));}
   boot.admitNativeImageRelocationV1(directory);
   if(mode==="hint"){bound();expect({...JSON.parse(readFileSync(receipt,"utf8")),ancestry:authored.ancestry},label).toEqual(authored);boot.admitNativeImageRelocationV1(directory);bound();}
   else{expect(bound,label).toThrow("Original Native image ancestry changed");expect(readFileSync(receipt,"utf8"),label).toBe(JSON.stringify(authored)+"\n");}
   expect(()=>boot.recordNativeImageRelocationV1({directory:home,movedFrom:newWork,movedTo:newWork}),label).toThrow("Exact Native ancestry relocation scope required");
   expect(()=>boot.recordNativeImageRelocationV1({directory:area,movedFrom:oldWork,movedTo:newWork}),label).toThrow("Exact Native ancestry relocation scope required");
  }finally{rmSync(area,{recursive:true,force:true});}}
});
