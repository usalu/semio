import os,re,sys
ROOT='/Users/ueli/Documents/semio'
B='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R137/stage/exchange'
G=ROOT+'/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange'
def sub(text,old,new,count=1):
    assert text.count(old)==count,(old[:70],text.count(old))
    return text.replace(old,new)
t=open(G+'/🟦️.ts').read()
t=sub(t,'implementation:string;stale?:number;snapshot?:string}>;','implementation:string;stale?:number;snapshot?:string;scope?:string}>;')
t=sub(t,'export function generationExchangeKeyV1(identity:string,platform:string,origin:string):string{return sha(Buffer.from(JSON.stringify([policy.domain,identity,platform,origin])));}',
 'export function generationExchangeKeyV1(identity:string,platform:string,origin:string,scope:string=policy.allScope):string{return sha(Buffer.from(JSON.stringify(scope===policy.allScope?[policy.domain,identity,platform,origin]:[policy.domain,identity,platform,origin,scope])));}\n\n/** 🔭️ Whether a head describes the scope a reader wants: a head of one slice is never the generation of another slice or of the whole workspace, and the whole workspace is the scope of a head that names none. */\nexport const generationScopeMatchesV1=(note:Readonly<{scope?:string}>,scope:string|undefined):boolean=>(note.scope??policy.allScope)===(scope??policy.allScope);')
t=sub(t,'origin?:string;implementation?:string;stale?:number;serving?:boolean;closures?:ClosurePortV1}>;','origin?:string;implementation?:string;stale?:number;serving?:boolean;closures?:ClosurePortV1;scope?:string}>;')
t=sub(t,'...shipped?{snapshot:shipped.instance}:{}};','...shipped?{snapshot:shipped.instance}:{},...location.scope===undefined||location.scope===policy.allScope?{}:{scope:location.scope}};')
t=sub(t,'if(!note||note.identity!==location.identity||note.platform!==location.platform)return false;','if(!note||note.identity!==location.identity||note.platform!==location.platform||!generationScopeMatchesV1(note,location.scope))return false;')
t=sub(t,'if(!note||note.identity!==location.identity||note.platform!==location.platform)return{status:"foreign"};','if(!note||note.identity!==location.identity||note.platform!==location.platform||!generationScopeMatchesV1(note,location.scope))return{status:"foreign"};')
t=sub(t,'serving?:boolean;closures?:ClosurePortV1}>):GenerationExchangeV1|undefined{','serving?:boolean;closures?:ClosurePortV1;scope?:string}>):GenerationExchangeV1|undefined{')
t=sub(t,'heads=(identity:string)=>({key:generationExchangeKeyV1(identity,platform,origins.self),keys:origins.all.map(origin=>generationExchangeKeyV1(identity,platform,origin)),origin:origins.self});','scope=input.scope??policy.allScope,heads=(identity:string)=>({key:generationExchangeKeyV1(identity,platform,origins.self,scope),keys:origins.all.map(origin=>generationExchangeKeyV1(identity,platform,origin,scope)),origin:origins.self,scope});')
def absolute(m):
    q=m.group(1);p=m.group(2)
    if p.startswith('./🧬️schema'): return m.group(0)
    return q+os.path.normpath(os.path.join(G,p))+q
t=re.sub(r'(["\'])(\.\.?/[^"\']+)\1',absolute,t)
s=open(G+'/🧬️schema/🔣️.json').read()
s=sub(s,'"rankCandidates": 3','"rankCandidates": 3,\n        "allScope": "all"')
s=sub(s,'        "snapshot": {\n','        "scope": {\n          "type": "string",\n          "pattern": "^[a-f0-9]{64}$",\n          "description": "The digest of the scope (selection and registry) of a slice generation; absent for the generation of the whole workspace. A reader takes only heads of its own scope."\n        },\n        "snapshot": {\n')
os.makedirs(B+'/🧬️schema',exist_ok=True);os.makedirs(B+'/🧪️tests',exist_ok=True)
open(B+'/🟦️.ts','w').write(t);open(B+'/🧬️schema/🔣️.json','w').write(s)
print('staged exchange')
