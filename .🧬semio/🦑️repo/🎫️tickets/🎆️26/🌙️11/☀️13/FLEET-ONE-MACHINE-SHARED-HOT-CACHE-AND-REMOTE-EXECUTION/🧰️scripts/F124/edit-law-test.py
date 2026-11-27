import os
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
s+='''
test("a scratch repository with the old dedupe files fails the law on the walker and on the command, the fixed files pass, and the oracle agrees",()=>{
 const policy=readImageLeaseLawPolicyV1(policyDocument),directory=mkdtempSync(join(testScratchRootV1("image-lease-law"),"repo-"));
 try{
  const source=(folder:string,text:string)=>{const path=join(directory,"🧰️framework","dedupe",folder,"🟦️.ts");mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);};
  const walker=(reader:boolean)=>"type Request=Readonly<{base:string;"+(reader?"reader:Reader;":"")+"minimumBytes:number}>;\\n"
   +"async function* files(base:string"+(reader?",reader:Reader":"")+"){\\n const stack=[base];\\n while(stack.length){\\n  const directory=stack.pop()!;\\n  const children="+(reader?"await reader.run(()=>readdir(directory))":"await readdir(directory)")+";\\n  for(const child of children){const path=join(directory,child);"+(reader?"yield await reader.run(()=>lstat(path));":"yield await lstat(path);")+"}\\n }\\n}\\n"
   +"export async function dedupe(request:DedupeRequestV1){\\n const found=[];\\n for await(const file of files(request.base"+(reader?",request.reader":"")+"))found.push(file.path);\\n for(const path of found)"+(reader?"await request.reader.run(()=>unlink(path));":"await unlink(path);")+"\\n}\\n";
  const command=(reader:boolean)=>"export async function run(context:Context,budgets:CachePartitionBudgetsV1,leaseDirectory:string){\\n const partition=budgets.tools,base=join(context.workspace,partition.directory,\\"m\\");\\n"
   +(reader?" const reader=createNativeImageSharedReaderV1({leaseDirectory,imageDirectory:join(context.workspace,partition.directory),signal});\\n try{\\n  await reader.run(()=>stat(base));\\n  await dedupeTreeV1({base,reader},ports);\\n }finally{await reader.close();}\\n":" await stat(base);\\n await dedupeTreeV1({base},ports);\\n")+"}\\n";
  source("old-walker",walker(false));source("old-command",command(false));source("fixed-walker",walker(true));source("fixed-command",command(true));
  const verdict=judgeImageLeaseRepositoryV1(directory,policy),shown=verdict.sites.map(site=>site.path.split("/").slice(-2)[0]+":"+site.opener);
  expect(verdict.unrouted.map(path=>path.split("/").slice(-2)[0])).toEqual(["old-command","old-walker"]);
  expect(shown.filter(row=>row.startsWith("old-command"))).toEqual(["old-command:stat","old-command:dedupeTreeV1"]);
  expect(shown.filter(row=>row.startsWith("old-walker"))).toEqual(["old-walker:readdir","old-walker:lstat","old-walker:unlink"]);
  expect(shown.filter(row=>row.startsWith("fixed")),"the fixed files pass").toEqual([]);
  expect(verdict.named.length).toBe(4);
  expect(shape(verdict)).toEqual(ask(["repository",directory,allowlist]) as OracleVerdict);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
'''
tmp=p+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,p);print("law test ok")
