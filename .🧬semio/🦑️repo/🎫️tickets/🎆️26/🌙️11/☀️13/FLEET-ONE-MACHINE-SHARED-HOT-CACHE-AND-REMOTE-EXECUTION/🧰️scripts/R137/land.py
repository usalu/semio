import os,sys,hashlib,shutil,json
ROOT='/Users/ueli/Documents/semio'
SC='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R137'
L=ROOT+'/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'
READY=L+'/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready'
EX=L+'/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange'
HOST=L+'/🏗️build/🕸️graph/🕰️host/🟦️.ts'
def sub(text,old,new):
    assert text.count(old)==1,(old[:80],text.count(old))
    return text.replace(old,new)
edits={}
s=open(READY+'/🧬️schema/🔣️.json').read()
s=sub(s,'{"name": "workspace", "kind": "repo", "arguments": ["run", "workspace:setup"], "timeoutMilliseconds": 14400000, "inputs": [], "bootstrapInput": false}','{"name": "workspace", "kind": "repo", "arguments": ["run-many", "--projects=@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp", "--targets=build"], "timeoutMilliseconds": 3600000, "inputs": [], "bootstrapInput": false}')
s=sub(s,'    "Texts": {','''    "Probes": {
      "const": {
        "version": 1,
        "target": "build",
        "projects": [
          {"name": "@semio-tech/norm-din4108", "kind": "typescript"},
          {"name": "@semio-tech/norm-din4108-rs", "kind": "rust"},
          {"name": "repo-mcp", "kind": "go"}
        ]
      }
    },
    "Texts": {''')
edits[READY+'/🧬️schema/🔣️.json']=s
t=open(READY+'/🟦️.ts').read()
a='/** 🧬️ The schema of `ready.json`. */'
t=sub(t,a,'/** 🎯️ The declared probes of the `workspace` stage: one target of a handful of projects of different kinds, so readiness reads their closure and never the whole workspace. */\nexport const READY_PROBES_V1=schema.definitions.Probes.const as Readonly<{version:1;target:string;projects:readonly Readonly<{name:string;kind:string}>[]}>;\n'+a)
edits[READY+'/🟦️.ts']=t
t=open(EX+'/🟦️.ts').read()
t=sub(t,'implementation:string;stale?:number;snapshot?:string}>;','implementation:string;stale?:number;snapshot?:string;scope?:string}>;')
t=sub(t,'export function generationExchangeKeyV1(identity:string,platform:string,origin:string):string{return sha(Buffer.from(JSON.stringify([policy.domain,identity,platform,origin])));}',
 'export function generationExchangeKeyV1(identity:string,platform:string,origin:string,scope:string=policy.allScope):string{return sha(Buffer.from(JSON.stringify(scope===policy.allScope?[policy.domain,identity,platform,origin]:[policy.domain,identity,platform,origin,scope])));}\n\n/** 🔭️ Whether a head describes the scope a reader wants: a head of one slice is never the generation of another slice or of the whole workspace, and the whole workspace is the scope of a head that names none. */\nexport const generationScopeMatchesV1=(note:Readonly<{scope?:string}>,scope:string|undefined):boolean=>(note.scope??policy.allScope)===(scope??policy.allScope);')
t=sub(t,'origin?:string;implementation?:string;stale?:number;serving?:boolean;closures?:ClosurePortV1}>;','origin?:string;implementation?:string;stale?:number;serving?:boolean;closures?:ClosurePortV1;scope?:string}>;')
t=sub(t,'...shipped?{snapshot:shipped.instance}:{}};','...shipped?{snapshot:shipped.instance}:{},...location.scope===undefined||location.scope===policy.allScope?{}:{scope:location.scope}};')
t=sub(t,'if(!note||note.identity!==location.identity||note.platform!==location.platform)return false;','if(!note||note.identity!==location.identity||note.platform!==location.platform||!generationScopeMatchesV1(note,location.scope))return false;')
t=sub(t,'if(!note||note.identity!==location.identity||note.platform!==location.platform)return{status:"foreign"};','if(!note||note.identity!==location.identity||note.platform!==location.platform||!generationScopeMatchesV1(note,location.scope))return{status:"foreign"};')
t=sub(t,'serving?:boolean;closures?:ClosurePortV1}>):GenerationExchangeV1|undefined{','serving?:boolean;closures?:ClosurePortV1;scope?:string}>):GenerationExchangeV1|undefined{')
t=sub(t,'heads=(identity:string)=>({key:generationExchangeKeyV1(identity,platform,origins.self),keys:origins.all.map(origin=>generationExchangeKeyV1(identity,platform,origin)),origin:origins.self});','scope=input.scope??policy.allScope,heads=(identity:string)=>({key:generationExchangeKeyV1(identity,platform,origins.self,scope),keys:origins.all.map(origin=>generationExchangeKeyV1(identity,platform,origin,scope)),origin:origins.self,scope});')
edits[EX+'/🟦️.ts']=t
s=open(EX+'/🧬️schema/🔣️.json').read()
s=sub(s,'"rankCandidates": 3','"rankCandidates": 3,\n        "allScope": "all"')
s=sub(s,'        "snapshot": {\n','        "scope": {\n          "type": "string",\n          "pattern": "^[a-f0-9]{64}$",\n          "description": "The digest of the scope (selection and registry) of a slice generation; absent for the generation of the whole workspace. A reader takes only heads of its own scope."\n        },\n        "snapshot": {\n')
edits[EX+'/🧬️schema/🔣️.json']=s
h=open(HOST).read()
h=sub(h,'registry:BuildOwnerRegistryV1,compact=false){','registry:BuildOwnerRegistryV1,compact=false,scope?:string){')
h=sub(h,'remoteGenerationExchangeV1({environment:process.env,workspace:root,identity:async()=>buildGraphGenerationIdentityV1(registry),','remoteGenerationExchangeV1({environment:process.env,workspace:root,identity:async()=>buildGraphGenerationIdentityV1(registry),...scope===undefined?{}:{scope},')
edits[HOST]=h
news={READY+'/🧪️tests/🎯️probes/🟦️.ts':SC+'/stage/probes.test.ts',EX+'/🧪️tests/🔭️scope/🟦️.ts':SC+'/stage/scope.test.ts'}
sha=lambda b:hashlib.sha256(b).hexdigest()[:12]
if '--dry' in sys.argv: print('dry ok',len(edits)); sys.exit(0)
bk=SC+'/backup';os.makedirs(bk,exist_ok=True);log={}
for p,c in edits.items():
    old=open(p,'rb').read();open(bk+'/'+sha(p.encode()),'wb').write(old)
    open(p,'w').write(c);log[p]={'before':sha(old),'after':sha(c.encode())}
for p,src in news.items():
    os.makedirs(os.path.dirname(p),exist_ok=True)
    c=open(src).read()
    if 'probes' in src: pass
    open(p,'w').write(c);log[p]={'before':None,'after':sha(c.encode())}
json.dump(log,open(SC+'/landed.json','w'),ensure_ascii=False,indent=1)
print(json.dumps({k[-60:]:v for k,v in log.items()},ensure_ascii=False,indent=1))
