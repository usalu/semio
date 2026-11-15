import sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:90],s.count(old))
    s=s.replace(old,new)
rep('  touch:{valued:["--worktree","--lang"],plain:["--stdin","--hook"]}\n','  touch:{valued:["--worktree","--lang"],plain:["--stdin","--hook"]},\n  "self-check":{valued:["--lang"],plain:[]}\n')
rep(' say(`status.git.${report.git.reason}`,{tree:report.git.indexTree??"-"});',''' if(report.health!==null){
  const seconds=(since:number):number=>Math.max(0,Math.floor((Date.now()-since)/1000));
  if(report.health.store!=="ok")say("status.health.store",{state:report.health.store,seconds:seconds(report.health.storeSinceMs)});
  if(report.health.pending!==null)say("status.health.pending",{digest:report.health.pending.code.slice(0,12),reason:report.health.pending.reason});
  for(const failure of report.health.failures)say("status.health.failure",{kind:failure.kind,count:failure.count,seconds:seconds(failure.lastMs)});
 }
 say(`status.git.${report.git.reason}`,{tree:report.git.indexTree??"-"});''')
rep('''  const attested=await readAttestationV1(storeRoot,origin),age=attested===undefined?undefined:Math.max(0,Math.floor((now-attested.wallMs)/1000));''','''  const attested=await readAttestationV1(storeRoot,origin),age=attested===undefined?undefined:Math.max(0,Math.floor((now-attested.wallMs)/1000));
  if(!force&&attested===undefined&&await stat(attestPathV1(storeRoot,origin)).then(()=>true,()=>false)){say("error.retire.unreadable",{origin});return {exitCode:1};}''')
rep(' const resolved=await resolveOptions(parsed,context),{options,storeRoot}=resolved;',''' if(parsed.operation==="self-check"){
  const root=resolve(import.meta.dirname,"..",".."),state=await dryOpenStateV1(context.root),checked0=await selfCheckV1(root,signal),checked=state===null?checked0:{...checked0,ok:false,error:`local state cannot be read: ${state}`},report={...checked,digest:checked.ok?await codeDigestV1(root).catch(()=>null):null};
  context.write(JSON.stringify(report));
  if(report.ok)say("selfcheck.ok",{modules:report.modules,schemas:report.schemas,texts:report.texts});else say("selfcheck.failed",{error:report.error??"-"});
  return {exitCode:report.ok?0:1};
 }
 const resolved=await resolveOptions(parsed,context),{options,storeRoot}=resolved;''')
rep('''  await runV1(options,signal,event=>{
   if(event.event==="restarting")restarting=true;''','''  await runV1({...options,watchRoot:context.environment.SEMIO_FLEET_CODE_WATCH},signal,event=>{
   if(event.event==="restarting")restarting=true;
   if(event.event==="restart-refused")say("event.restart-refused",{digest:(event.reason??"").slice(0,12),reason:event.message??"-"});''')
rep('  if(error instanceof ReplicaBusyV1){say("error.busy",{pid:error.pid});return {exitCode:1};}','  if(error instanceof ReplicaBusyV1){say("error.busy",{pid:error.pid});return {exitCode:1};}\n  if(error instanceof OriginTakenV1){say("error.origin.taken",{origin:error.originId});return {exitCode:1};}')
rep('🎬️ Routes `seed|join|run|status|snapshot|relay|compact|verify|bootstrap-archive|bootstrap-settle|sync|fleet|rebase|repair|retire|touch`;','🎬️ Routes `seed|join|run|status|snapshot|relay|compact|verify|bootstrap-archive|bootstrap-settle|sync|fleet|rebase|repair|retire|touch|self-check`; `self-check` loads every module of the replica, compiles every embedded schema and validates every text catalogue without opening a store or a worktree state, prints one report line and exits 0 only when all is well (the daemon runs it in a fresh process on new code before it restarts);')
open(p,"w",encoding="utf8").write(s)

s=open(p,encoding="utf8").read()
assert "compactV1,homedOriginsV1," in s
s=s.replace("compactV1,homedOriginsV1,","compactV1,dryOpenStateV1,homedOriginsV1,",1)
open(p,"w",encoding="utf8").write(s)
