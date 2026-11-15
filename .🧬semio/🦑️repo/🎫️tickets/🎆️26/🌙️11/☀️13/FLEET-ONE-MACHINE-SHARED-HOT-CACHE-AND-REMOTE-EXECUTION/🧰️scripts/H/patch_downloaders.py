import os,sys,hashlib
V="️"
def e(cp,rest=""): return chr(cp)+V+rest
FW="/Users/Shared/semio/"+e(0x1F9F0,"framework")
LIB=FW+"/"+"/".join([e(0x1F6CD,"products"),e(0x1F991,"repo"),e(0x1F528,"modules"),e(0x1F4DA,"library")])
T=LIB+"/"+e(0x26A1,"caching")+"/"+e(0x1F680,"bootstrap")+"/"+e(0x1F6E0,"tools")
dry="--dry" in sys.argv
log=[]
def sub(path,old,new,count=1):
    s=open(path,encoding="utf8").read()
    if new in s and old not in s: print("already",path[-60:]); return
    n=s.count(old)
    if n!=count: raise SystemExit(f"anchor count {n} != {count} in {path}: {old[:90]!r}")
    before=hashlib.sha256(s.encode()).hexdigest()
    s2=s.replace(old,new)
    if not dry: open(path,"w",encoding="utf8").write(s2)
    log.append((path,before,hashlib.sha256(s2.encode()).hexdigest()))
    print("patched" if not dry else "ok(dry)",path[-70:])
FF_REL=lambda ups: "../"*ups+e(0x1F6F0,"fleet-fetch")+"/"+e(0x1F7E6,".ts")
imp=lambda ups,names="fleetFetchContextV1,pinnedFetchV1": f'import{{{names}}}from"{FF_REL(ups)}";\n'

# bun tool
p=T+"/"+e(0x1F7E6,"bun")+"/"+e(0x1F7E6,".ts")
sub(p,'import {linuxHostLibcV1} from "../../../../🎛️commands/🔧️setup/🟦️.ts";\n','import {linuxHostLibcV1} from "../../../../🎛️commands/🔧️setup/🟦️.ts";\n'+imp(2))
sub(p,'const response = await fetch(`${manifest.release}/${row.archive}`, { signal:AbortSignal.any([signal,deadline.signal]) });','const response = await pinnedFetchV1(fleetFetchContextV1(process.env, workspace), {url:`${manifest.release}/${row.archive}`, sha256:row.sha256, bytes:row.bytes}, { signal:AbortSignal.any([signal,deadline.signal]) });')
# binaryen
p=T+"/"+e(0x1F578,"wasm")+"/"+e(0x1F7E6,".ts")
s=open(p,encoding="utf8").read(); first=s.split("\n",1)[0]
sub(p,first+"\n",first+"\n"+imp(2))
sub(p,'const response = await fetch(`${manifest.release}/${row.archive}`, { signal });','const response = await pinnedFetchV1(fleetFetchContextV1(process.env, workspace), {url:`${manifest.release}/${row.archive}`, sha256:row.sha256, bytes:row.bytes}, { signal });')
# bazel
p=T+"/"+e(0x1F3D7,"bazel")+"/"+e(0x1F4DC,"script.ts")
s=open(p,encoding="utf8").read(); first=s.split("\n",1)[0]
sub(p,first+"\n",first+"\n"+imp(2))
sub(p,'const response=await fetch(`${manifest.release}/${row.file}`,{signal});','const response=await pinnedFetchV1(fleetFetchContextV1(process.env,workspace),{url:`${manifest.release}/${row.file}`,sha256:row.sha256,bytes:row.bytes},{signal});')
# native tools
p=T+"/"+e(0x1F9F0,"native")+"/"+e(0x1F7E6,".ts")
s=open(p,encoding="utf8").read(); first=s.split("\n",1)[0]
sub(p,"  fetch(url: string, signal: AbortSignal): Promise<Response>;","  fetch(url: string, signal: AbortSignal, pin?: Readonly<{sha256: string; bytes: number}>): Promise<Response>;")
sub(p,"const response = await port.fetch(nativeToolUrlV1(tool, host), signal);","const response = await port.fetch(nativeToolUrlV1(tool, host), signal, {sha256: row.sha256, bytes: row.bytes});")
p=T+"/"+e(0x1F9F0,"native")+"/"+e(0x1F50C,"port")+"/"+e(0x1F7E6,".ts")
s=open(p,encoding="utf8").read(); first=s.split("\n",1)[0]
sub(p,first+"\n",first+"\n"+imp(3))
sub(p,"fetch: (url: string, signal: AbortSignal) => fetch(url, {signal}),","fetch: (url: string, signal: AbortSignal, pin?: Readonly<{sha256: string; bytes: number}>) => pin ? pinnedFetchV1(fleetFetchContextV1(process.env, workspace), {url, ...pin}, {signal}) : fetch(url, {signal}),")
sub(p,'fetch: async (url: string, signal: AbortSignal) => { check(signal); if (authority.network.offline) throw Error("Offline Original native tool network refused"); return fetch(url, {signal}); },','fetch: async (url: string, signal: AbortSignal, pin?: Readonly<{sha256: string; bytes: number}>) => { check(signal); if (authority.network.offline) throw Error("Offline Original native tool network refused"); return pin ? pinnedFetchV1(fleetFetchContextV1(environment), {url, ...pin}, {signal}) : fetch(url, {signal}); },')
# rustup
R=T+"/"+e(0x1F980,"rustup")
SCH=R+"/"+e(0x1F9EC,"schema")+"/"+e(0x1F523,".json")
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"patch_tail.py"),encoding="utf8").read())
