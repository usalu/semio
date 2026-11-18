import sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:70],s.count(old))
    s=s.replace(old,new)
rep('const rows=[],files=await fs.list(directory),sides','const rows=[],files=[...await fs.list(directory)],sides')
rep(' function handle(name,lock){',' function handle(name,lock,settings){')
rep('''    return admitted(signal,async()=>{
     if((await live(signal)).some(claim=>claim.token!==name&&claim.state==="held"&&claimsConflictV1(next,claim)))return{held:false,request};''','''    return admitted(signal,async()=>{
     if((await active(signal)).some(claim=>claim.token!==name&&claim.state==="held"&&claimsConflictV1(next,claim)))return{held:false,request};''')
rep('return admitted(signal,async()=>{if((await live(signal)).some(claim=>claim.token!==name))return false;await write({...held,structural:true});return true;});','return admitted(signal,async()=>{if((await active(signal)).some(claim=>claim.token!==name))return false;await write({...held,structural:true});return true;});')
rep('''   async release(){if(!own.has(name))return;own.delete(name);try{await fs.remove(pathOf(name));}finally{lock.release();}},
  });''','''   async settle(finished){
    const held=current();if(held.structural)return false;
    const done=new Set(finished),rest=held.owners.filter(owner=>!done.has(owner));
    if(rest.length===held.owners.length)return true;
    await write(readCustodyClaimV1({...held,owners:rest}));return true;
   },
   async progress(event){
    const at=now(),next=advanceProgressV1(side,event,at),phaseChanged=next.phase!==side.phase,moved=next.advancedMs!==side.advancedMs;
    side=next;
    if(!moved||!phaseChanged&&at-wrote<P.progressWriteMilliseconds)return;
    wrote=at;await writeSide(name,"progress",side).catch(()=>{});
   },
   async handover(){
    const ask=await readSide(name,"ask",readAskV1);
    if(ask)return{reason:ask.reason,ask};
    const mine=own.get(name)?.holder?.digest,found=await digestNow();
    if(!mine||!found||mine===found)return undefined;
    const claim=current();
    return(await live()).some(row=>row.token!==name&&row.state==="waiting"&&claimsConflictV1(claim,row))?{reason:"stale-code",current:found}:undefined;
   },
   handoverError(reason,{done=0,total=0}={}){
    const fields={seconds:Math.round((idleMillisecondsV1({progress:side},now())??0)/1000),done,total,digest:(own.get(name)?.holder?.digest??"").slice(0,12),current:(reason.current??settings.currentSeen??"").slice(0,12)},key=reason.reason==="stale-code"?"handoverStale":"handoverStalled";
    return Object.assign(Error(renderCustodyMessageV1(key,fields,"en")+"\\n"+renderCustodyMessageV1(key,fields,"de")),{code:"SEMIO_STALE_IMPLEMENTATION",handover:{reason:reason.reason,done,total}});
   },
   async release(){if(!own.has(name))return;own.delete(name);try{await fs.remove(pathOf(name));await removeSides(name);}finally{lock.release();}},
  });''')
rep('''  return Object.freeze({
   token:name,
   get owners(){return current().owners;},''','''  let side={phase:"start",done:0,total:0,advancedMs:now()},wrote=side.advancedMs;
  const digestNow=()=>settings.digest();
  return Object.freeze({
   token:name,
   get owners(){return current().owners;},''')
# hold
rep(' async function hold({owners=[],structural=false,signal,say=()=>{}}={}){',' async function hold({owners=[],structural=false,signal,say=()=>{},language="en",holder,currentDigest}={}){')
rep('  const mine={owners:ownersOf(owners),structural:structural===true},started=now();let claim,noticed=-Infinity;','''  rememberDoorV1(directory);
  const mine={owners:ownersOf(owners),structural:structural===true,...holder?{holder:readHolderV1(holder)}:{}},started=now(),told=new Set();let claim,noticed=-Infinity,seen,seenAt=-Infinity;
  const digestNow=async()=>{if(!currentDigest)return undefined;if(now()-seenAt<P.noticeMilliseconds&&seen!==undefined)return seen;try{seen=await currentDigest();}catch{}seenAt=now();return seen;};
  const asking=async blockers=>{
   const at=now(),current=await digestNow();
   for(const row of blockers){
    if(isDeposedV1(row,at)&&!told.has("d"+row.token)){told.add("d"+row.token);say("deposed",{pid:row.holder?.pid??0,seconds:Math.round((at-row.ask.atMs)/1000)});continue;}
    const reason=askReasonV1({claim:row,now:at,current});
    if(!reason)continue;
    await writeSide(row.token,"ask",{version:1,reason,atMs:at,by:{pid:process.pid,digest:mine.holder?.digest??""}}).catch(()=>{});
    if(reason==="stalled")say("askedStalled",{pid:row.holder?.pid??0,seconds:Math.round((idleMillisecondsV1(row,at)??0)/1000)});else say("askedStale",{pid:row.holder?.pid??0,digest:(row.holder?.digest??"").slice(0,12),current:(current??"").slice(0,12)});
   }
  };''')
rep('''     const others=(await live(signal)).filter(row=>row.token!==name);''','''     const at=now(),others=(await live(signal)).filter(row=>row.token!==name&&!isDeposedV1(row,at));''')
rep('''     claim??={version:1,token:name,ticket:others.reduce((top,row)=>Math.max(top,row.ticket),0)+1,...mine,state:"waiting",queuedMs:now()};''','''     claim??={version:1,token:name,ticket:others.reduce((top,row)=>Math.max(top,row.ticket),0)+1,...mine,state:"waiting",queuedMs:now()};''')
rep('    if(decision.admit)return handle(name,lock);','''    if(decision.admit){await writeSide(name,"progress",{phase:"start",done:0,total:0,advancedMs:now()}).catch(()=>{});return handle(name,lock,{digest:digestNow});}
    await asking(decision.blockers);''')
rep('''     say(claim.structural||first.structural||!sample.length?"blockedAll":"blocked",{peers:decision.blockers.length,seconds:Math.round((now()-started)/1000),sample:sample.join(", ")});''','''     say(claim.structural||first.structural||!sample.length?"blockedAll":"blocked",{peers:decision.blockers.length,seconds:Math.round((now()-started)/1000),sample:sample.join(", "),holder:holderSuffixV1(first,now(),language)});''')
rep('  }catch(error){own.delete(name);await fs.remove(pathOf(name)).catch(()=>{});lock.release();throw error;}','  }catch(error){own.delete(name);await fs.remove(pathOf(name)).catch(()=>{});await removeSides(name);lock.release();throw error;}')
rep(' return!(await live(signal)).some(claim=>claimsConflictV1(mine,claim));',' return!(await active(signal)).some(claim=>claimsConflictV1(mine,claim));')
open(p,"w",encoding="utf8").write(s)
