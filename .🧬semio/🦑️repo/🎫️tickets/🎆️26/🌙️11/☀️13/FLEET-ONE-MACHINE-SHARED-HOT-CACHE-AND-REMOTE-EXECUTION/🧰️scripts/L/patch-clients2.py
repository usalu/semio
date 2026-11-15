L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
C=L+"⚡️caching/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path.split('/')[-2],s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)
edit(C+'🚀️bootstrap/🛰️fleet-fetch/🟦️.ts',[('import {resolveRemoteCacheV1,fleetStoreRootV1} from','import {remoteCacheFetchV1,resolveRemoteCacheV1,fleetStoreRootV1} from')])
U=C+'🦀️cargo/🗃️unit-cache/🛰️fleet/'
s=open(U+'🟦️.ts',encoding='utf8').read()
print([l[:160] for l in s.split("\n") if l.startswith("import")])
