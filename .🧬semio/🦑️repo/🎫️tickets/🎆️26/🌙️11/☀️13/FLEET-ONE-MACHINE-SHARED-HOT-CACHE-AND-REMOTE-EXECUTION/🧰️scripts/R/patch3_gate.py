import sys
P="/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts"
t=open(P,encoding='utf8').read()
if 'memoryNeverFits' in t: sys.exit("already patched")
def sub(old,new):
    global t
    assert t.count(old)==1,(old[:60],t.count(old))
    t=t.replace(old,new)
sub('''     if(mine.verdict==="admit"){
      if(!holdsLock(directory,pid))continue;''','''     if(mine.verdict==="wait"&&(mine.reason==="never"||mine.reason==="starved")){
      const total=host.totalMegabytes,floor=evaluation.floorMegabytes,minimum=request.minimumMegabytes??0;
      throw hostCapacityErrorV1(mine.reason==="never"?"memoryNeverFits":"memoryStarved",{operation:request.operation,minimum,total,floor,capacity:Math.floor(Math.max(0,total-floor)*policy.memory.budgetBasisPoints/10000),needed:minimum+floor,spare:Math.max(0,evaluation.headroomMegabytes),seconds:Math.round((at-arrivedAtMs)/1000)},environment);
     }
     if(mine.verdict==="admit"){
      if(!holdsLock(directory,pid))continue;''')
sub('''      else if(mine.reason==="reduced")write(''','''      else if(mine.reason==="shrunk")write(formatHostCapacityV1(language,"shrunk",{operation:request.operation,reserved:mine.reservedMegabytes,wanted:request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,spare:Math.max(0,evaluation.headroomMegabytes),floor:evaluation.floorMegabytes,minimum:request.minimumMegabytes??0}));
      else if(mine.reason==="reduced")write(''')
open(P,'w',encoding='utf8').write(t);print("ok")
