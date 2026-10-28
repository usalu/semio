import sys,re
T=sys.argv[1]
p=T+"/make_e.py"
s=open(p,encoding="utf-8").read()
old_start=s.index('"""/** ⚓️ The boot instant of this boot')
old_end=s.index('/** 🛑️ Why a grant offered by a replaced command cannot be taken over. */""")')
new='''"""/** ⚓️ The boot instant of this boot, written once beside the grants: `bootAtMs` (the wall clock minus the uptime at the first reader, to the second) and the uptime of that reader. */
export type HostBootAnchorV1=Readonly<{bootAtMs:number;uptimeMs:number}>;

/** 🕰️ Decides the anchor of the boot a process runs in. The instant a grant carries must not follow the wall clock (a step of the clock, by the user, a time service or a resumed virtual machine, moves `now - uptime` by the step wherever the uptime is monotonic, and made every running grant look like a grant of another boot): an anchor taken at an uptime that is not later than ours belongs to this boot and stays as it is; none, or one taken later than now (the uptime ran backwards, so the machine restarted since), is replaced by one taken from the wall clock and the uptime. A restart that is quicker than a heartbeat of the grants is also refused by their age. Pure and deterministic. */
export function hostBootAnchorV1(anchor:HostBootAnchorV1|undefined,wallMs:number,uptimeMs:number):HostBootAnchorV1{
 return anchor!==undefined&&uptimeMs>=anchor.uptimeMs?anchor:Object.freeze({bootAtMs:Math.round((wallMs-uptimeMs)/1000)*1000,uptimeMs});
}

'''
s=s[:old_start]+new+s[old_end:]
s=s.replace("Number.isSafeInteger(row?.monotonicMs)?{bootAtMs:row!.bootAtMs!,monotonicMs:row!.monotonicMs!}:undefined","Number.isSafeInteger(row?.uptimeMs)?{bootAtMs:row!.bootAtMs!,uptimeMs:row!.uptimeMs!}:undefined")
s=s.replace("export function hostBootAtV1(directory:string,nowMs:number,uptimeMs:number=uptime()*1000,monotonicMs:number=Math.floor(Number(process.hrtime.bigint()/1000000n))):number{","export function hostBootAtV1(directory:string,nowMs:number,uptimeMs:number=Math.floor(uptime()*1000)):number{")
s=s.replace("anchor=hostBootAnchorV1(stored,nowMs,uptimeMs,monotonicMs);","anchor=hostBootAnchorV1(stored,nowMs,uptimeMs);")
open(p,"w",encoding="utf-8").write(s)
p=T+"/files/boot-anchor.ts"
s=open(p,encoding="utf-8").read()
s=s.replace("(the monotonic clock has run backwards since the anchor was taken)","(the uptime has run backwards since the anchor was taken)")
s=s.replace("hostBootAnchorV1(undefined,1_700_000_000_000,600_000,600_000)","hostBootAnchorV1(undefined,1_700_000_000_000,600_000)")
s=s.replace("monotonicMs:600_000","uptimeMs:600_000")
s=s.replace("hostBootAnchorV1(first,wall,uptime,1_600_000)","hostBootAnchorV1(first,wall,uptime)")
s=s.replace("hostBootAnchorV1(first,1_700_000_600_000,30_000,30_000)","hostBootAnchorV1(first,1_700_000_600_000,30_000)")
s=s.replace("monotonicMs:30_000","uptimeMs:30_000")
s=s.replace("a monotonic clock that ran backwards makes a new one","an uptime that ran backwards makes a new one")
s=s.replace("hostBootAnchorV1(undefined,1_700_000_000_000,600_000,600_000)","hostBootAnchorV1(undefined,1_700_000_000_000,600_000)")
s=s.replace("hostBootAnchorV1(anchor,1_700_000_000_000+5*HOUR,700_000,700_000)","hostBootAnchorV1(anchor,1_700_000_000_000+5*HOUR,700_000)")
s=s.replace("{bootAtMs:number;monotonicMs:number}","{bootAtMs:number;uptimeMs:number}")
s=s.replace("monotonicMs:stored.monotonicMs+10*HOUR","uptimeMs:stored.uptimeMs+10*HOUR")
s=s.replace(".monotonicMs).toBeLessThan(stored.monotonicMs+HOUR)",".uptimeMs).toBeLessThan(stored.uptimeMs+HOUR)")
open(p,"w",encoding="utf-8").write(s)
