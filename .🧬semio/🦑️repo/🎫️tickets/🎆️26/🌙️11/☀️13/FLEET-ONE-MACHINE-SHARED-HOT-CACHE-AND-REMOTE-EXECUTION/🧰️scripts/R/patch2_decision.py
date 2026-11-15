import sys
P="/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts"
t=open(P,encoding='utf8').read()
if 'shrinkable' in t: sys.exit("already patched")
def sub(old,new,count=1):
    global t
    assert t.count(old)==count,(old[:60],t.count(old))
    t=t.replace(old,new)
sub('elastic?:boolean;label?:string}>;\n/** ⏳️','elastic?:boolean;minimumMegabytes?:number;label?:string}>;\n/** ⏳️')
sub('|"ramp"|"quiet-limit";\n/** ⚖️','|"ramp"|"quiet-limit"|"shrunk"|"never"|"starved";\n/** ⚖️')
sub('reason:Exclude<HostReasonV1,"fits"|"reduced"|"override">','reason:Exclude<HostReasonV1,"fits"|"reduced"|"override"|"shrunk">')
sub('"waiterUpgrade"|"reduced"|"reducedGrows"','"waiterUpgrade"|"reduced"|"shrunk"|"reducedGrows"')
sub('`elastic` marks a supplementary request','`minimumMegabytes` is the least memory a request that can run smaller still starts with: when its full reservation does not fit but the free memory covers this minimum it is admitted with what is free (`shrunk`), when this minimum can never fit the machine it is refused at once (`never`), and when nothing of ours runs and not even the minimum is free after `load.overrideAfterMilliseconds` it is refused (`starved`), because waiting for memory that no run of ours holds cannot help; `elastic` marks a supplementary request')
sub('''  const memoryFits=fits(request.minimumJobs),first=''','''  const capacityLimit=Math.floor(cap*policy.memory.budgetBasisPoints/10000);
  if(request.minimumMegabytes!==undefined&&request.minimumMegabytes>capacityLimit&&replaced===undefined&&!elastic&&!quiet){refuse("never");continue;}
  const memoryFits=fits(request.minimumJobs),shrinkable=request.minimumMegabytes!==undefined&&replaced===undefined&&!elastic&&!quiet&&request.minimumMegabytes<need(request.minimumJobs),room=Math.min(freeObserved,freeBudget),shrunk=shrinkable&&!memoryFits&&room>=request.minimumMegabytes!?Math.min(need(request.minimumJobs),room):0,first=''')
sub('''  if(jobs===0&&pause===undefined&&memoryFits&&(avoids''','''  if(jobs===0&&pause===undefined&&(memoryFits||shrunk>0)&&(avoids''')
sub('''  if(jobs>0){admit(jobs,jobs<request.preferredJobs?"reduced":"fits");continue;}
''','''  if(jobs>0){if(shrunk>0&&jobs===request.minimumJobs)admit(jobs,"shrunk",shrunk);else admit(jobs,jobs<request.preferredJobs?"reduced":"fits");continue;}
  if(shrunk>0&&active.length===0&&admitted===0&&waited>=policy.load.overrideAfterMilliseconds){admit(request.minimumJobs,"shrunk",shrunk);continue;}
  if(shrinkable&&!memoryFits&&shrunk===0&&active.length===0&&waited>=policy.load.overrideAfterMilliseconds){refuse("starved");continue;}
''')
open(P,'w',encoding='utf8').write(t);print("ok")
