TL="⚡️caching/🚀️bootstrap/🛠️tools/"
def plan(p,L):
    t=L+TL+"📦️sdk/📊️material/🧪️tests/🟦️.ts"
    s=p.load(t)
    a=s.index('test("SDK material reads full valid authorizer digest manifests within their own byte and aggregate bounds"')
    b=s.index('\n});\n',a)+len('\n});\n')
    p.sub(t,s[a:b],'')
    p.sub(t,'''[".semio-authorizer-source.json",Buffer.from(JSON.stringify({version:1,mode:"release",entry:"entry.ts",resolutions:[],sources:[{path:"entry.ts",sha256:"a".repeat(64)}]}))],[".semio-authorizer-compiler.json",Buffer.from(JSON.stringify({version:1,mode:"release",bun:"bun@1.3.14",resolutions:[],sources:[{path:"entry.ts",sha256:"a".repeat(64)}]}))],''','')
    p.sub(t,'update("nx-tooling-install-v4\\0")','update("nx-tooling-install-v5\\0")')
    def fixture(d):del d["authorizerManifests"]
    p.json(L+TL+"📦️sdk/📊️material/🧪️tests/🔣️.json",fixture)
    p.sub(L+"⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts",'join(dirname(dirname(sdkRequire.resolve("nx/package.json"))),"../.semio-authorizer-source.json")','join(process.env.SEMIO_NX_AUTHORIZER_ROOT!,".semio-authorizer-source.json")')
