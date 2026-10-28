from make_lib import *

# ---- E: the boot instant of a grant is anchored per boot, not derived from the wall clock
H = P + "/🏗️build/🌡️host-capacity"
sch, dec, gate, hand, con = H + "/🧬️schema/🔣️.json", H + "/📏️decision/🟦️.ts", H + "/🎫️gate/🟦️.ts", H + "/🎫️gate/🤝️handover/🟦️.ts", H + "/🎫️gate/📋️contract/🟦️.ts"
edit(sch, '        "bootToleranceMilliseconds": 120000,\n', '        "bootToleranceMilliseconds": 120000,\n        "bootFile": "boot.json",\n')
edit(dec, "/** 🛑️ Why a grant offered by a replaced command cannot be taken over. */",
"""/** ⚓️ The boot instant of this boot, written once beside the grants: `bootAtMs` (the wall clock minus the uptime at the first reader, to the second) and the uptime of that reader. */
export type HostBootAnchorV1=Readonly<{bootAtMs:number;uptimeMs:number}>;

/** 🕰️ Decides the anchor of the boot a process runs in. The instant a grant carries must not follow the wall clock (a step of the clock, by the user, a time service or a resumed virtual machine, moves `now - uptime` by the step wherever the uptime is monotonic, and made every running grant look like a grant of another boot): an anchor taken at an uptime that is not later than ours belongs to this boot and stays as it is; none, or one taken later than now (the uptime ran backwards, so the machine restarted since), is replaced by one taken from the wall clock and the uptime. A restart that is quicker than a heartbeat of the grants is also refused by their age. Pure and deterministic. */
export function hostBootAnchorV1(anchor:HostBootAnchorV1|undefined,wallMs:number,uptimeMs:number):HostBootAnchorV1{
 return anchor!==undefined&&uptimeMs>=anchor.uptimeMs?anchor:Object.freeze({bootAtMs:Math.round((wallMs-uptimeMs)/1000)*1000,uptimeMs});
}

/** 🛑️ Why a grant offered by a replaced command cannot be taken over. */""")
edit(gate, "const bootAt=(now:number)=>Math.round((now-uptime()*1000)/1000)*1000;\n",
"""const bootAnchors=new Map<string,HostBootAnchorV1>();
const readBootAnchor=(value:unknown):HostBootAnchorV1|undefined=>{const row=value as Partial<HostBootAnchorV1>|undefined;return Number.isSafeInteger(row?.bootAtMs)&&Number.isSafeInteger(row?.uptimeMs)?{bootAtMs:row!.bootAtMs!,uptimeMs:row!.uptimeMs!}:undefined;};
/** ⚓️ The boot instant of the grants in `directory`: read from the anchor of the boot (see `hostBootAnchorV1`), written when this is the first process of the boot, and kept for the life of this process, so the wall clock is read once and never moves it. */
export function hostBootAtV1(directory:string,nowMs:number,uptimeMs:number=Math.floor(uptime()*1000)):number{
 let anchor=bootAnchors.get(directory);
 if(anchor===undefined){
  const path=join(directory,protocol.bootFile),stored=readBootAnchor(readJson(path));
  anchor=hostBootAnchorV1(stored,nowMs,uptimeMs);
  if(anchor!==stored)try{writeJson(path,anchor);}catch{}
  bootAnchors.set(directory,anchor);
 }
 return anchor.bootAtMs;
}
""")
edit(gate, " const rows:HostGrantV1[]=[],boot=bootAt(now);", " const rows:HostGrantV1[]=[],boot=hostBootAtV1(directory,now);")
edit(gate, "pidAlive:isHostCapacityV1(\"Grant\",inherited)&&alive((inherited as HostGrantV1).pid),nowMs:at,bootAtMs:bootAt(at),", "pidAlive:isHostCapacityV1(\"Grant\",inherited)&&alive((inherited as HostGrantV1).pid),nowMs:at,bootAtMs:hostBootAtV1(directory,at),")
edit(gate, "observedMegabytes:null,pid,bootAtMs:bootAt(at),", "observedMegabytes:null,pid,bootAtMs:hostBootAtV1(directory,at),")
edit(gate, "readJson,writeJson,remove,alive,bootAt,modified,delay,", "readJson,writeJson,remove,alive,bootAt:hostBootAtV1,modified,delay,")
edit(hand, "pidAlive:alive(grant.pid),nowMs:at,bootAtMs:bootAt(at),", "pidAlive:alive(grant.pid),nowMs:at,bootAtMs:bootAt(directory,at),")
edit(hand, '("Grant",{...rest,pid,bootAtMs:bootAt(at)});', '("Grant",{...rest,pid,bootAtMs:bootAt(directory,at)});')
edit(con, "bootAt:(now:number)=>number;", "bootAt:(directory:string,now:number)=>number;")
newfile(H + "/🧪️tests/⚓️boot-anchor/🟦️.ts", "boot-anchor.ts")
edit(gate, "import {decideHostHandoverV1,", "import {decideHostHandoverV1,hostBootAnchorV1,type HostBootAnchorV1,")
