import re,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124/"
def rd(p):return open(p,encoding="utf8").read()
def wr(p,s):open(p,"w",encoding="utf8").write(s)
def sub(s,a,b,count=1):
    assert s.count(a)==count,(a[:60],s.count(a))
    return s.replace(a,b)

s=rd(S+"base/owner-pool.ts")
s=sub(s,'export type OwnerPoolTargetV1=Readonly<{children:number;reason:"pressure"|"yielding"|"nothing-fits"|"waiting"|null}>;',
'''export type OwnerPoolTargetV1=Readonly<{children:number;reason:"pressure"|"yielding"|"nothing-fits"|"waiting"|null;limit?:OwnerPoolLimitV1}>;
/** 🔢️ The numbers behind a target that is below the cores: which rule binds (`memory`, `cores`, `load`, `pressure`, `waiting`, `none`), what one more child needs, what is free for it above the floor, what the machine's own work holds back, the floor, and the cores (all, booked by the machine's own work, idle by measurement, `null` when not measured). */
export type OwnerPoolLimitV1=Readonly<{limit:"memory"|"cores"|"load"|"pressure"|"waiting"|"none";needMegabytes:number;freeMegabytes:number;heldMegabytes:number;floorMegabytes:number;children:number;cores:number;bookedCores:number;idleCores:number|null;swapOutMegabytesPerSecond:number;waitedMilliseconds:number}>;''')
a=s.index("/** 🎯️ How many children this machine runs now")
b=s.index("/** 🌡️ The idle cores between two samples")
old=s[a:b]
doc=old[:old.index("export function ownerPoolTargetV1")]
doc=doc.replace("every core and every free megabyte that its own work and the system floor leave,","every core and every free megabyte that the system floor and the part of its own work's untouched reservations leave (the reservations of its own work are held back up to `ownHoldShareBasisPoints` of the memory while nothing of it waits, all of them once it waits: a request that cannot be admitted makes the pool yield within `yieldAfterMilliseconds` anyway, so holding more in advance only idles the machine; the memory its own work really holds is already out of the available memory),")
assert "ownHoldShareBasisPoints" in doc
new=doc+'''export function ownerPoolSizingV1(input:Readonly<{host:OwnerPoolHostV1;children:number;childMegabytes:number}>,policy:typeof P=P):OwnerPoolTargetV1&Readonly<{limit:OwnerPoolLimitV1}>{
 const {host}=input,floor=clamp(Math.floor(host.totalMegabytes*policy.floorShareBasisPoints/10000),policy.floorMinimumMegabytes,policy.floorMaximumMegabytes),foreground=host.foreground,need=input.childMegabytes;
 const held=foreground.waiters>0?foreground.unrealizedMegabytes:Math.min(foreground.unrealizedMegabytes,Math.floor(host.totalMegabytes*policy.ownHoldShareBasisPoints/10000)),spare=host.availableMegabytes-floor-held;
 const reserved=foreground.waiters>0?Math.ceil(host.cores*policy.foregroundCoreShareBasisPoints/10000):0,booked=Math.max(0,host.cores-foreground.cpu-reserved),idle=host.idleCores??null,waited=foreground.oldestWaitMilliseconds??0,swap=host.swapOutMegabytesPerSecond??0;
 const limit=(kind:OwnerPoolLimitV1["limit"],children:number):OwnerPoolLimitV1=>({limit:kind,needMegabytes:need,freeMegabytes:Math.max(0,spare-Math.max(0,children-input.children)*need),heldMegabytes:held,floorMegabytes:floor,children:input.children,cores:host.cores,bookedCores:Math.min(host.cores,foreground.cpu),idleCores:idle,swapOutMegabytesPerSecond:swap,waitedMilliseconds:waited});
 if(host.pressure==="critical"||host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>64)return{children:0,reason:"pressure",limit:limit("pressure",0)};
 if(foreground.waiters>0&&waited>=policy.yieldAfterMilliseconds&&waited<policy.yieldUntilMilliseconds)return{children:0,reason:"waiting",limit:limit("waiting",0)};
 const minimum=Math.max(1,Math.ceil(host.cores*policy.minimumShareBasisPoints/10000)),byCores=foreground.waiters>0?booked:idle===null?Math.max(booked,minimum):host.cores;
 const external=host.load===null?0:Math.max(0,host.load-input.children),byLoad=Math.max(minimum,idle===null?Math.floor(host.cores*policy.loadCeilingBasisPointsPerCore/10000-external):input.children+Math.floor(idle));
 const byMemory=input.children+Math.floor(spare/Math.max(1,need));
 let children=Math.min(byCores,byLoad,byMemory,policy.maximumChildren);
 if(host.pressure==="warn")children=Math.min(children,Math.max(1,input.children));
 const wanted=Math.min(host.cores,policy.maximumChildren),kind:OwnerPoolLimitV1["limit"]=byMemory<=Math.min(byCores,byLoad)?"memory":byLoad<byCores?"load":"cores";
 if(children<1)return{children:1,reason:byMemory<1?"nothing-fits":"yielding",limit:limit(kind,1)};
 if(children<wanted)return{children,reason:kind==="memory"?"nothing-fits":"yielding",limit:limit(kind,children)};
 return{children,reason:null,limit:limit("none",children)};
}

/** 🎯️ The target of the pool without its numbers: how many children and why fewer than the cores.
 * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */
export function ownerPoolTargetV1(input:Readonly<{host:OwnerPoolHostV1;children:number;childMegabytes:number}>,policy:typeof P=P):OwnerPoolTargetV1{
 const {children,reason}=ownerPoolSizingV1(input,policy);
 return{children,reason};
}

'''
s=s[:a]+new+s[b:]
# measured footprint
oldf=s[s.index("/** 📏️ What one more child is expected to hold"):s.index("/** ♻️ Whether a child is replaced")]
newf='''/** 📏️ What one more child is expected to hold: the `peakPercentile` of the newest `peakSamples` resident sizes the children reported after an owner together with the resident sizes the children hold right now (`live`: a child that waits for an owner or loads its program is a real child too, so a pool of many idle children is not priced as if every one ran the heaviest owner), nearest rank, plus the margin, so a single giant owner does not price every light one and the price does not jump with the last five; the declared default while nobody reported. These are the children's own measurements, never a share of a parent's growth.
 * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */
export function ownerPoolChildMegabytesV1(samples:readonly number[],live:readonly number[]=[],policy:typeof P=P):number{
 const recent=[...samples.filter(value=>Number.isFinite(value)&&value>0).slice(-policy.peakSamples),...live.filter(value=>Number.isFinite(value)&&value>0)].sort((a,b)=>a-b);
 return recent.length===0?policy.childMegabytesDefault:Math.ceil(recent[Math.max(0,Math.ceil(recent.length*policy.peakPercentile/100)-1)]!*(100+policy.peakMarginPercent)/100);
}

'''
s=s.replace(oldf,newf)
wr(S+"stage/owner-pool.ts",s)
