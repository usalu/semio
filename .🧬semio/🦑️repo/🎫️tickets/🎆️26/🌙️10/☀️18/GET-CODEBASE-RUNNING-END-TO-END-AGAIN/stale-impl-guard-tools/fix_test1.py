import sys,re
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
p=L+"🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts"
s=open(p,encoding="utf-8").read()
i=s.index('test("a queued exclusive request that waits while the stamp moves')
j=s.index('test("the publication gate of a producer')
new='''test("a queued exclusive request that waits while the stamp moves withdraws its ticket, and the one at the head of the queue is refused at its admission",async()=>{
 const base=scratch(),directory=join(base,"leases");mkdirSync(directory);
 const table={...protocol,stamps:[{id:"p.format",schema:"s.json",pointer:"/f/stamp"}],resources:[{prefix:"producer:",stamps:["p.format"]}]};
 writeAtomic(join(base,"s.json"),{f:{stamp:"A"}});
 const registry=createFormatStampsV1({base,table:table as never,environment:{SEMIO_LANGUAGE:"de"}});registry.declare("p.format",readJson(join(base,"s.json")));
 const holder=await unguarded(directory,"producer:wait"),unregister=registerFormatStampGuardV1(registry);
 let head:Promise<unknown>|undefined;
 try{
  head=lease(directory,"producer:wait").then(()=>undefined,error=>error);
  await new Promise(accept=>setTimeout(accept,300));
  writeAtomic(join(base,"s.json"),{f:{stamp:"B"}});
  const waiting=await lease(directory,"producer:wait").then(()=>undefined,error=>error);
  expect(waiting).toBeInstanceOf(StaleImplementationErrorV1);
  expect(waiting.message).toContain("Dieser Befehl führt eine veraltete Implementierung aus");
  holder.release();
  const admitted=await head;
  expect(admitted).toBeInstanceOf(StaleImplementationErrorV1);
 }finally{unregister();try{holder.release();}catch{}await head;}
 const free=await lease(directory,"producer:wait",undefined,0);free.release();
});

'''
s=s[:i]+new+s[j:]
s=s.replace('join(dirname(dirname(dirname(dirname(dirname(dirname(library)))))),"🧰️framework","🔨️modules","🏃️process","🔒️leases","🟦️.ts")','join(dirname(dirname(dirname(dirname(library)))),"🔨️modules","🏃️process","🔒️leases","🟦️.ts")')
s=s.replace('{environment:{SEMIO_LANGUAGE:"en"},outermost:true,argv:[process.execPath,child,base,directory,output]','{environment:{...process.env,SEMIO_LANGUAGE:"en"},outermost:true,argv:[process.execPath,child,base,directory,output]')
open(p,"w",encoding="utf-8").write(s)
