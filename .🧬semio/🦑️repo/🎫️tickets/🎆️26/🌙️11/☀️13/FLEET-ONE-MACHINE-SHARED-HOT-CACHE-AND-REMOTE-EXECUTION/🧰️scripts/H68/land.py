import sys,os
R=sys.argv[1]; S=sys.argv[2]
def rw(p,f):
    s=open(p,encoding='utf8').read(); n=f(s); assert n!=s,p
    t=p+".h68tmp"; open(t,'w',encoding='utf8').write(n); os.replace(t,p)
def sub(s,old,new):
    assert s.count(old)==1,(old[:60],s.count(old)); return s.replace(old,new)
block=open(S+"/block.ts",encoding='utf8').read()
rw(R+"/🧾️provenance/🟦️.ts",lambda s:sub(s,"/** 🕰️ The output names that record when",block+"/** 🕰️ The output names that record when"))
def server(s):
    s=sub(s,"provenanceDifferingV1,provenanceFingerprintV1,type ConsumptionV1","provenanceDifferingV1,provenanceFingerprintV1,provenanceStreamsEquivalentV1,type ConsumptionV1")
    return sub(s,"reader=blobs(address.namespace);\n","reader=blobs(address.namespace);\n       if(await provenanceStreamsEquivalentV1(differing,keptRows,refusedRows,hash=>reader.read(hash))){options.provenance.advised(\"stream-rendering\");return text(200,\"Stored\");}\n")
rw(R+"/🖥️server/🟦️.ts",server)
def prov_test(s):
    s=sub(s,"provenanceFingerprintV1} from","provenanceFingerprintV1,provenanceStreamIdentityV1,provenanceStreamsEquivalentV1} from")
    s=sub(s,"differing ${fixture.differing.length}`","differing ${fixture.differing.length}\\nok streams ${fixture.streams.length}`")
    return s.rstrip("\n")+"\n"+open(S+"/test-provenance.ts",encoding='utf8').read()
rw(R+"/🧾️provenance/🧪️tests/🟦️.ts",prov_test)
rw(R+"/🖥️server/🧪️tests/🔐️access/🟦️.ts",lambda s:sub(s,'test("consumption: the status names',open(S+"/test-server.ts",encoding='utf8').read()+'\ntest("consumption: the status names'))
rw(R+"/🧾️provenance/🧪️tests/🐍️.py",lambda s:s.rstrip("\n")+"\n"+open(S+"/oracle.py",encoding='utf8').read())
os.replace(S+"/fixture.json",R+"/🧾️provenance/🧪️tests/🧫️fixtures/🔣️.json")
print("landed")
