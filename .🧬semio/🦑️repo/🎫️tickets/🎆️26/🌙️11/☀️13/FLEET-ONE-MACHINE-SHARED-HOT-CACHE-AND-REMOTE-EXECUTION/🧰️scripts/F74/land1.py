import os,sys,re,time
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","F66"))
from lib import TICKET,LIB,locks,release
got,held=locks({"caching-cargo","laws"},"F74")
if held:print("held",held);release(got);sys.exit(3)
def rw(path,fn):
    s=open(path,encoding="utf8").read();n=fn(s);assert n!=s,path
    t=path+".f74tmp";open(t,"w",encoding="utf8").write(n);os.replace(t,path)
U=LIB+"/⚡️caching/🦀️cargo/📦️units"
def once(s,a,b):
    assert s.count(a)==1,(a[:60],s.count(a));return s.replace(a,b)
def units(s):
    s=once(s,'import {captureRustLibraryDescriptorV1} from "../📥️descriptor/🟦️.ts";\n','')
    i=s.index("/** 🧫️ Gives the private repository the io-base64");j=s.index('test("closed crate units agree with Ajv')
    s=s[:i]+s[j:]
    i=s.index('test("actual io-base64 Cargo package');j=s.index('test("full owned data capture')
    s=s[:i]+s[j:]
    a='await expect(captureOwnedCommandV1("cargo",["build","--manifest-path",manifest,"--unit-graph","-Zunstable-options"],root,{control,env:environment})).rejects.toThrow();'
    b='expect((await captureOwnedCommandResultV1("cargo",["build","--manifest-path",manifest,"--unit-graph","-Zunstable-options"],root,{control,env:environment,output:{maximumBytes:1048576,maximumLines:65536}})).status).not.toBe(0);'
    return once(s,a,b)
rw(U+"/🧪️tests/🟦️.ts",units)
rw(U+"/🧪️tests/📐️profile/🟦️.ts",lambda s:once(once(s,'const descriptor=await inspect("📥️descriptor/🟦️.ts"),compiler','const compiler'),'expect(descriptor.calls).toContain("captureCargoCompilationGraphV1");expect(descriptor.calls).toContain("withRepositoryCompilerStoreV1");expect(descriptor.text).not.toContain("options.arguments");',''))
rw(U+"/🧭️context/🧪️tests/🟦️.ts",lambda s:once(s,'for(const path of["../../📥️descriptor/🟦️.ts","../../📥️descriptor/🕸️graph/🟦️.ts"])','for(const path of["../../📥️descriptor/🕸️graph/🟦️.ts"])'))
rw(LIB+"/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json",lambda s:re.sub(r'  "[^"\n]*📦️units/📥️descriptor/🟦️\.ts": 7,\n','',s,count=1))
os.remove(U+"/📥️descriptor/🟦️.ts")
release(got);print("landed")
