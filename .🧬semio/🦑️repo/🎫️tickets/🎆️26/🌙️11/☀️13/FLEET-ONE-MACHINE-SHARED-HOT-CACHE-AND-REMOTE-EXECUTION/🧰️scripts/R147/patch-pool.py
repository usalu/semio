import re,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R147/"
t=open(S+"base/pool.ts",encoding="utf8").read()
def sub(old,new,count=1):
    global t
    assert t.count(old)==count,(old[:60],t.count(old))
    t=t.replace(old,new)
sub('waitingMegabytes?:number;oldestWaitMilliseconds?:number}>}>;','waitingMegabytes?:number;oldestWaitMilliseconds?:number;ask?:OwnerPoolAskV1}>}>;')
sub('/** 🏠️ What the host says about itself for the sizing of the pool:','/** 🙋️ What the machine\'s own read claims from the one budget through the capacity governor: the megabytes its first waiting owner needs (`alone`: the owner is one the cost table says needs the whole machine, claimed from the first request until the owner has ended).\n * @see ../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks/🟦️.ts */\nexport type OwnerPoolAskV1=Readonly<{megabytes:number;alone:boolean}>;\n\n/** 🏠️ What the host says about itself for the sizing of the pool:')
# new sizing body
start=t.index('export function ownerPoolSizingV1')
end=t.index('/** 📌️ The target of the pool without its numbers')
body=r'''export function ownerPoolSizingV1(input:Readonly<{host:OwnerPoolHostV1;children:number;childMegabytes:number}>,policy:typeof P=P):OwnerPoolTargetV1&Readonly<{limit:OwnerPoolLimitV1}>{
 const {host}=input,floor=clamp(Math.floor(host.totalMegabytes*policy.floorShareBasisPoints/10000),policy.floorMinimumMegabytes,policy.floorMaximumMegabytes),foreground=host.foreground,need=input.childMegabytes,ask=foreground.ask;
 const hold=foreground.waiters>0&&foreground.waitingMegabytes===undefined?foreground.unrealizedMegabytes:Math.min(foreground.unrealizedMegabytes,Math.floor(host.totalMegabytes*policy.ownHoldShareBasisPoints/10000)),roomNow=host.availableMegabytes-floor-hold,roomDrained=roomNow+input.children*need;
 const blocked=(megabytes:number)=>megabytes>roomNow&&megabytes<=roomDrained,counted=(megabytes:number)=>megabytes<=roomDrained?megabytes:0;
 const held=hold+counted(foreground.waitingMegabytes??0)+counted(ask?.megabytes??0),spare=host.availableMegabytes-floor-held;
 const reserved=foreground.waiters>0?Math.ceil(host.cores*policy.foregroundCoreShareBasisPoints/10000):0,booked=Math.max(0,host.cores-foreground.cpu-reserved),idle=host.idleCores??null,waited=foreground.oldestWaitMilliseconds??0,swap=host.swapOutMegabytesPerSecond??0;
 const limit=(kind:OwnerPoolLimitV1["limit"],children:number):OwnerPoolLimitV1=>({limit:kind,needMegabytes:need,freeMegabytes:Math.max(0,spare-Math.max(0,children-input.children)*need),heldMegabytes:held,floorMegabytes:floor,children:input.children,cores:host.cores,bookedCores:Math.min(host.cores,foreground.cpu),idleCores:idle,swapOutMegabytesPerSecond:swap,waitedMilliseconds:waited});
 const swapShare=host.swapOutMegabytesPerSecond===null||swap<policy.swapActiveMegabytesPerSecond?1:Math.max(0,(policy.swapOutMegabytesPerSecond-swap)/(policy.swapOutMegabytesPerSecond-policy.swapActiveMegabytesPerSecond)),swapCap=Math.floor(input.children*swapShare);
 if(host.pressure==="critical"||swapCap<1&&swapShare<1)return{children:0,reason:"pressure",limit:limit("pressure",0)};
 if(ask?.alone===true)return{children:0,reason:"waiting",limit:limit("waiting",0)};
 const coresHeld=idle===null||idle<1&&idle+input.children>=1,memoryHeld=foreground.waitingMegabytes===undefined||blocked(foreground.waitingMegabytes);
 if(foreground.waiters>0&&waited>=policy.yieldAfterMilliseconds&&(memoryHeld||coresHeld))return{children:0,reason:"waiting",limit:limit("waiting",0)};
 const minimum=Math.max(1,Math.ceil(host.cores*policy.minimumShareBasisPoints/10000)),byCores=idle!==null?(foreground.waiters>0?input.children+Math.floor(Math.max(0,idle-reserved)):host.cores):foreground.waiters>0?booked:Math.max(booked,minimum);
 const external=host.load===null?0:Math.max(0,host.load-input.children),byLoad=Math.max(minimum,idle===null?Math.floor(host.cores*policy.loadCeilingBasisPointsPerCore/10000-external):input.children+Math.floor(idle));
 const byMemory=input.children+Math.floor(spare/Math.max(1,need));
 let children=Math.min(byCores,byLoad,byMemory,policy.maximumChildren,...swapShare<1?[swapCap]:[]);
 if(host.pressure==="warn")children=Math.min(children,Math.max(1,input.children));
 const wanted=Math.min(host.cores,policy.maximumChildren),kind:OwnerPoolLimitV1["limit"]=byMemory<=Math.min(byCores,byLoad)?"memory":byLoad<byCores?"load":"cores",ownWork=foreground.cpu>0||foreground.waiters>0||ask!==undefined;
 if(children<1)return ownWork&&byMemory<1?{children:0,reason:"nothing-fits",limit:limit(kind,0)}:{children:1,reason:byMemory<1?"nothing-fits":"yielding",limit:limit(kind,1)};
 if(children<wanted)return{children,reason:kind==="memory"?"nothing-fits":"yielding",limit:limit(kind,children)};
 return{children,reason:null,limit:limit("none",children)};
}

'''
t=t[:start]+body+t[end:]
# docstring above function: replace first sentences about yield window
old=t[t.index('/** 🎯️ How many children this machine runs now:'):start]
new='''/** 🎯️ How many children this machine runs now, from one budget of memory and cores that the machine's own read and the pool share (the read has the first claim, the pool gets what it leaves): none while the read's owner is one that needs the whole machine (`ask.alone`, from its first request until the owner ended); none while a request of the machine's own work has waited `yieldAfterMilliseconds` in the gate and the children are what holds it up (its memory fits only once they are gone, or no core is idle although they would free one: a wait the children did not cause is no reason to stop them, and there is no time after which they come back); otherwise every core and every free megabyte that the system floor, the part of its own work's untouched reservations, what its waiting requests ask for and the first claim of its read (only when draining the children would make it fit) leave, so a claim that does not fit drains exactly as many children as it needs, the children that run are never more than the spare memory allows (one child is kept only where the machine's own work is absent or memory is not what limits it), and swap shrinks the pool before it stops anything else: from `swapActiveMegabytesPerSecond` to the `swapOutMegabytesPerSecond` at which the gate stops the farm the children are cut in proportion (the same number, a law test pins both). Cores: the machine's, less what its own grants book, less a share while its own requests wait; the idle cores the host measured (`idleCores`, the children's own load not counted) grow the pool by as many children, and where nothing was measured the load that does not come from the children is held against the load ceiling (`loadCeilingBasisPointsPerCore` per core); at most `maximumChildren`; memory: the available megabytes less the floor and the held part, divided by the size of one child, added to the children that already hold theirs. A pure function.
 * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */
'''
t=t.replace(old,new)
open(S+"stage/pool.ts","w",encoding="utf8").write(t)
# schema
s=open(S+"base/schema.json",encoding="utf8").read()
old='"yieldUntilMilliseconds": 600000,'
assert s.count(old)==1
s=s.replace(old,'"swapActiveMegabytesPerSecond": 1,\n        "swapOutMegabytesPerSecond": 64,')
open(S+"stage/schema.json","w",encoding="utf8").write(s)
