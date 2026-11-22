TL="⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/"
def walk(node,visit):
    if isinstance(node,dict):
        visit(node)
        for v in list(node.values()):walk(v,visit)
    elif isinstance(node,list):
        for v in node:walk(v,visit)
def plan(p,L):
    t=L+TL+"🧪️tests/🟦️.ts"
    src=p.load(t)
    a=src.index('registerIsolatedBuildTestV1(import.meta.path,"managed tooling crash recovery authenticates the complete current lease implementation closure"')
    b=src.index('registerIsolatedBuildTestV1(import.meta.path,"private toolkit cleanup retains arbitrary primary')
    p.sub(t,src[a:b],'')
    p.sub(t,'nxAuthorizerReceiptHoldsV1:()=>{calls++;if(mode==="receipt-throw")throw undefined;return mode!=="authorizer-receipt";}};','};')
    p.sub(t,'["install-identity","marker-version","marker-digest","authorizer-receipt"].includes(row.mode)?row.mode:"install-identity"','["install-identity","marker-version","marker-digest"].includes(row.mode)?row.mode:"install-identity"')
    p.sub(t,"SELECT ?1 IN ('install-identity','marker-version','marker-digest','authorizer-receipt') accepted","SELECT ?1 IN ('install-identity','marker-version','marker-digest') accepted")
    p.sub(t,'for(const fault of["install-identity","marker-version","marker-digest","authorizer-receipt","success","receipt-throw"])','for(const fault of["install-identity","marker-version","marker-digest","success"])')
    p.sub(t,'let delivery="normal",mode="install-identity",calls=0;','let delivery="normal",mode="install-identity";')
    p.sub(t,'mode=fault;delivery=output;calls=0;writes.length=0;','mode=fault;delivery=output;writes.length=0;')
    p.sub(t,'expect(calls).toBe(["authorizer-receipt","success","receipt-throw"].includes(fault)?1:0);','')
    p.sub(t,'expect(writes.length).toBe(Number(!["success","receipt-throw"].includes(fault)&&output==="normal"));if(fault==="receipt-throw")expect(failure).toBeUndefined();else if(failed)expect(failure).toBe(primary);','expect(writes.length).toBe(Number(fault!=="success"&&output==="normal"));if(failed)expect(failure).toBe(primary);')
    p.sub(t,"const bundle='export const authoredSource = true;',sourceManifest=JSON.stringify({version:1,mode:'release',entry:'fixture.ts',sources:[{path:'fixture.ts',sha256:hash('authored source')}],resolutions:[]}),material=JSON.stringify(","const material=JSON.stringify(")
    p.sub(t,"['bun.lock',Buffer.from('authored-generation-lock')],['.semio-authorizer-source.json',Buffer.from(sourceManifest)],['.semio-authorizer-compiler.json',Buffer.from('{}')],['.semio-sdk-material.json',Buffer.from(material)]])","['bun.lock',Buffer.from('authored-generation-lock')],['.semio-sdk-material.json',Buffer.from(material)]])")
    p.sub(t,",generation=path.join(root,'nx',generationId),authorizers={version:1,mode:'release',sourceSha256:hash(sourceManifest),entry:'semio-authorizers.mjs',outputs:[{path:'semio-authorizers.mjs',sha256:hash(bundle)}]};",",generation=path.join(root,'nx',generationId);")
    p.sub(t,"await fs.writeFile(path.join(generation,'semio-authorizers.mjs'),row.fault==='authorizer'?bundle+'changed':bundle);","")
    p.sub(t,"JSON.stringify({version:1,digest:generationId,authorizers})","JSON.stringify({version:1,digest:generationId})")
    p.sub(t,"IN ('none','escaped','identity','authorizer') zero","IN ('none','escaped','identity') zero")
    p.sub(t,'source=JSON.stringify(row.fault==="valid"?{version:1,mode:"release",entry:"entry.ts",sources:[{path:"entry.ts",sha256:hash("source")}],resolutions:[]}:{entry:"../../never-read",opaque:true}),output=row.fault==="valid"?Buffer.alloc(33_554_432,0x20):"export const neverImported=1;",binaryReceipt=','binaryReceipt=')
    p.sub(t,'["bun.lock",Buffer.from("closed-lock")],[".semio-authorizer-source.json",Buffer.from(source)],[".semio-authorizer-compiler.json",Buffer.from("{}")],[".semio-sdk-material.json"','["bun.lock",Buffer.from("closed-lock")],[".semio-sdk-material.json"')
    p.sub(t,'await fs.writeFile(path.join(generation,"semio-authorizers.mjs"),row.fault==="output-drift"?"changed":output);','')
    p.sub(t,'authorizers:{version:1,mode:"release",sourceSha256:row.fault==="marker-drift"?"0".repeat(64):hash(source),entry:"semio-authorizers.mjs",outputs:[{path:"semio-authorizers.mjs",sha256:hash(output)}]},binaries:','binaries:')
    p.sub(t,'if(row.fault==="source-drift")await fs.writeFile(path.join(generation,".semio-authorizer-source.json"),"changed");','')
    p.sub(t,'if(row.fault==="changed-metadata")await fs.writeFile(path.join(generation,".semio-authorizer-source.json"),"changed");','if(row.fault==="changed-metadata")await fs.writeFile(path.join(generation,"package.json"),"changed");')
    p.sub(t,"SELECT ? IN ('opaque','valid') accepted","SELECT ? IN ('valid') accepted")
    def corpus(d):
        del d["readerSourceCases"]
        d["generationAuthenticationDiagnostics"]=[r for r in d["generationAuthenticationDiagnostics"] if r["mode"]!="authorizer-receipt"]
        d["derivativeReads"]=[r for r in d["derivativeReads"] if r["fault"]!="authorizer"]
        rows=[r for r in d["generationIntegrityReads"] if r["fault"] not in("opaque","source-drift","output-drift","marker-drift")]
        for r in rows:
            if r["captured"]:r["strictAccepted"]=True
        d["generationIntegrityReads"]=rows
    p.json(L+TL+"🧪️tests/🔣️.json",corpus)
    def schema(d):
        defs=d["definitions"]
        del defs["ReaderSourceCase"]
        del defs["Protocol"]["const"]["readerEntry"]
        def visit(n):
            e=n.get("enum")
            if isinstance(e,list):
                if "authorizer-receipt" in e:n["enum"]=[x for x in e if x!="authorizer-receipt"]
                elif "authorizer" in e and "source-index" in e:n["enum"]=[x for x in e if x!="authorizer"]
            pt=n.get("pattern")
            if isinstance(pt,str) and "[.]semio-authorizer-source[.]json|" in pt:n["pattern"]=pt.replace("[.]semio-authorizer-source[.]json|[.]semio-authorizer-compiler[.]json|","")
        walk(d,visit)
    p.json(L+TL+"🧬️schema/🔣️.json",schema)
