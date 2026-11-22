import sys,json,re
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/"
def sub(path,old,new,count=1):
    s=open(B+path,encoding="utf8").read()
    assert s.count(old)==count,(path,old[:60],s.count(old))
    open(B+path,"w",encoding="utf8").write(s.replace(old,new))
P="🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs"
sub(P,"planGuard=HOST_POLICY_V1.slicePlanGuard,","planGuard=HOST_POLICY_V1.slicePlanGuard,sameRootWaitMilliseconds=HOST_POLICY_V1.sliceSameRootWaitMilliseconds,")
sub(P,"""  const key=String(request.root),previous=tails.get(key)??Promise.resolve(),turn=holding(previous,signal,onProgress,()=>{const lead=leads.get(key);return lead===undefined?{key:"queued-root",used:active,budget:maximum,limit:0}:{key:"queued-leader",used:active,budget:maximum,limit:0,kind:lead.phase,done:lead.done,total:lead.total};}).then(async()=>{""",
"""  const key=String(request.root),previous=tails.get(key)??Promise.resolve(),waitStarted=performance.now(),remaining=()=>Math.max(0,Math.ceil((sameRootWaitMilliseconds-(performance.now()-waitStarted))/1000));let leaderEnded=false;void previous.then(()=>{leaderEnded=true;});
  const aheadOrBound=new Promise(resolve=>{const timer=setTimeout(resolve,sameRootWaitMilliseconds);timer.unref?.();void previous.then(()=>{clearTimeout(timer);resolve();});signal?.addEventListener("abort",()=>clearTimeout(timer),{once:true});});
  const turn=holding(aheadOrBound,signal,onProgress,()=>{const lead=leads.get(key);return lead===undefined?{key:"queued-root",used:active,budget:maximum,limit:remaining()}:{key:"queued-leader",used:active,budget:maximum,limit:remaining(),kind:lead.phase,done:lead.done,total:lead.total};}).then(async()=>{
   if(!leaderEnded&&!signal?.aborted){const lead=leads.get(key);note();onProgress?.(0,{phase:"queue",done:lead?.done??0,total:lead?.total??0,running:[],decision:{key:"alongside",used:active,budget:maximum,limit:Math.round(sameRootWaitMilliseconds/1000),kind:lead?.phase??"",done:lead?.done??0,total:lead?.total??0}},true);}""")
sub(P,"""if(detail!==undefined&&detail.phase!=="queue"&&detail.phase!=="reservation")leads.set(root,{phase:String(detail.phase),done:Number(detail.done)||0,total:Number(detail.total)||0,owner:mine});""","""if(detail!==undefined&&detail.phase!=="queue"&&detail.phase!=="reservation"){const ahead=leads.get(root);if(ahead===undefined||ahead.owner===mine)leads.set(root,{phase:String(detail.phase),done:Number(detail.done)||0,total:Number(detail.total)||0,owner:mine});}""")
S="🧬️schema/🔣️.json"
sub(S,'''    "sliceRestartSameCauseMaximum": 3,''','''    "sliceRestartSameCauseMaximum": 3,
    "sliceSameRootWaitMilliseconds": 30000,''')
EN_OLD="both share one fact store, so this one starts when that one has ended."
EN_NEW="both share one fact store, so this one starts when that one has ended, or after {limit} more s alongside it (it then takes what that one has already written)."
DE_OLD="beide teilen einen Faktenspeicher, daher beginnt diese, wenn jene beendet ist."
DE_NEW="beide teilen einen Faktenspeicher, daher beginnt diese, wenn jene beendet ist, oder nach {limit} weiteren s neben ihr (sie übernimmt dann, was jene schon geschrieben hat)."
sub(S,EN_OLD,EN_NEW,2)
sub(S,DE_OLD,DE_NEW,2)
sub(S,'''      "host-slice-owners-queued-pool": "Reading''','''      "host-slice-owners-alongside": "Reading the project graph slice ({seconds} s): the load ahead of this one on this checkout is still running after {limit} s ({kind} {done} of {total}), so this one now runs alongside it and takes the facts that one has already written instead of waiting for it.",
      "host-slice-owners-queued-pool": "Reading''')
sub(S,'''      "host-slice-owners-queued-pool": "Lese''','''      "host-slice-owners-alongside": "Lese den Projektgraph-Ausschnitt ({seconds} s): die vorausgehende Ladung dieses Checkouts läuft nach {limit} s noch ({kind} {done} von {total}), daher läuft diese jetzt neben ihr und übernimmt die Fakten, die jene schon geschrieben hat, statt auf sie zu warten.",
      "host-slice-owners-queued-pool": "Lese''')
