import sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:80],s.count(old))
    s=s.replace(old,new)
rep('watch:{digest:string;since:number}|undefined,verdict','adopt:{digest:string;since:number}|undefined,verdict')
rep('''    if(digest===started){watch=undefined;verdict=undefined;}
    else if(watch?.digest!==digest){watch={digest,since:now};verdict=undefined;}
   }
   if(watch!==undefined&&now-watch.since>=stable){
    const digest=watch.digest,refuse=(reason:string):void=>{if(watch?.digest===digest){''','''    if(digest===started){adopt=undefined;verdict=undefined;}
    else if(adopt?.digest!==digest){adopt={digest,since:now};verdict=undefined;}
   }
   if(adopt!==undefined&&now-adopt.since>=stable){
    const digest=adopt.digest,refuse=(reason:string):void=>{if(adopt?.digest===digest){''')
rep('      if(watch?.digest!==digest)return;\n      if(result.ok)','      if(adopt?.digest!==digest)return;\n      if(result.ok)')
rep('at=watch.since+stable+restartJitterMillisecondsV1','at=adopt.since+stable+restartJitterMillisecondsV1')
rep('const next=JSON.stringify([snapshot.sync,snapshot.files,snapshot.manifest,snapshot.cursors,snapshot.incoming,code,gitState?.commit','const refusal=verdict?.ok===false?{code:verdict.digest,ok:false as const,reason:verdict.reason.slice(0,512)}:undefined,failures=options.failures?.()??[];\n   const next=JSON.stringify([snapshot.sync,snapshot.files,snapshot.manifest,snapshot.cursors,snapshot.incoming,started,refusal,failures,gitState?.commit')
old=s[s.index("   await options.prepare?.();"):s.index("   signature=next;wrote=now;")]
body=old.replace("   await options.prepare?.();\n","").replace("code,cursors:snapshot.cursors","code:started,cursors:snapshot.cursors")
assert "code:started" in body
body=body.replace("queue:{incoming:snapshot.incoming}});","queue:{incoming:snapshot.incoming},...refusal===undefined?{}:{pending:refusal},...failures.length===0?{}:{failures}});")
assert "pending:refusal" in body
new='''   const write=async():Promise<void>=>{
    await options.prepare?.();
'''+body.replace("\n   ","\n    ").replace("   if(viewState","    if(viewState",1).replace("   const commit","    const commit",1).replace("   await writeAttestationV1","    await writeAttestationV1",1)+'''   };
   if(options.store===undefined)await write();
   else{try{await options.store(write);}catch(error){if(error instanceof StoreStalledV1)return "idle";throw error;}}
'''
s=s.replace(old,new)
rep('failures?:()=>readonly FailureSummaryV1[];git:','failures?:()=>readonly FailureSummaryV1[];store?:<T>(work:()=>Promise<T>)=>Promise<T>;git:')
rep('import {importClosureV1,moduleSourcesV1,restartJitterMillisecondsV1,type FailureSummaryV1}','import {importClosureV1,moduleSourcesV1,restartJitterMillisecondsV1,StoreStalledV1,type FailureSummaryV1}')
open(p,"w",encoding="utf8").write(s)
