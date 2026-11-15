import sys;sys.path.insert(0,__import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from lib import *
A='🩺️attest/🟦️.ts'
sub(A,'pending?:Readonly<{code:string;ok:false;reason:string}>;failures?:readonly FailureSummaryV1[]}>;','pending?:Readonly<{code:string;ok:false;reason:string}>;failures?:readonly FailureSummaryV1[];fallback?:FleetFallbackV1}>;')
sub(A,'/** 🔎️ The fields over which settled workers must agree:','/** 🪂️ The guard of a worker that runs a fallback code instead of the code of its worktree: `source` is the sealed `snapshot` of this machine or a snapshot that a healthy peer published to the `store`, `reason` why the worktree code was not used. */\nexport type FleetFallbackV1=Readonly<{guard:"code-fallback";source:"snapshot"|"store";reason:string}>;\n\n/** 🔎️ The fields over which settled workers must agree:')
sub(A,'kind:"git-version"|"volume"|"over-budget"|"symlinks"','kind:"git-version"|"volume"|"over-budget"|"symlinks"|"code-fallback"')
sub(A,'  if(!guards.volume.symlinks&&guards.links>0)warnings.push({originId:origin,kind:"symlinks",detail:String(guards.links)});\n }\n return warnings;','  if(!guards.volume.symlinks&&guards.links>0)warnings.push({originId:origin,kind:"symlinks",detail:String(guards.links)});\n }\n for(const origin of origins){const fallback=attestations[origin]!.fallback;if(fallback!==undefined)warnings.push({originId:origin,kind:"code-fallback",detail:`${fallback.source}: ${fallback.reason}`.slice(0,200)});}\n return warnings;')
sub(A,'refused?:(digest:string,reason:string)=>void;','refused?:(digest:string,reason:string)=>void;admit?:(digest:string,signal?:AbortSignal)=>Promise<Readonly<{ok:true}>|Readonly<{ok:false;reason:string}>>;fallback?:FleetFallbackV1;')
sub(A,'let started="",bootAt=0,reported=false,adopt:{digest:string;since:number}|undefined,','let started="",bootAt=0,reported=false,adopt:{digest:string;since:number}|undefined,admission:{digest:string;ok:boolean;reason:string;at:number}|undefined,admitting:Promise<void>|undefined,')
sub(A,'''  pending:()=>verdict?.ok===false?{code:verdict.digest,ok:false,reason:verdict.reason}:null,''','''  pending:shown,''')
sub(A,''' return {
  pending:shown,''',''' const shown=():Readonly<{code:string;ok:false;reason:string}>|null=>verdict?.ok===false?{code:verdict.digest,ok:false,reason:verdict.reason}:adopt!==undefined&&admission?.digest===adopt.digest&&!admission.ok?{code:admission.digest,ok:false,reason:`waiting: ${admission.reason}`}:null;
 return {
  pending:shown,''')
sub(A,'    if(digest===started){adopt=undefined;verdict=undefined;}\n    else if(adopt?.digest!==digest){\n     adopt={digest,since:now};verdict=undefined;','    if(digest===started){adopt=undefined;verdict=undefined;admission=undefined;}\n    else if(adopt?.digest!==digest){\n     adopt={digest,since:now};verdict=undefined;admission=undefined;')
sub(A,'''    if(verdict===undefined&&checking===undefined){
     checking=(options.check===undefined?''','''    if(verdict===undefined&&checking===undefined&&options.admit!==undefined&&admitting===undefined&&(admission?.digest!==digest||!admission.ok&&now-admission.at>=heartbeat)){
     admitting=Promise.resolve().then(()=>options.admit!(digest,signal)).then(result=>{if(adopt?.digest===digest)admission={digest,ok:result.ok,reason:result.ok?"":result.reason,at:options.now()};},error=>{if(adopt?.digest===digest)admission={digest,ok:false,reason:error instanceof Error?error.message:String(error),at:options.now()};}).finally(()=>{admitting=undefined;});
    }
    if(verdict===undefined&&checking===undefined&&(options.admit===undefined||admission?.digest===digest&&admission.ok)){
     checking=(options.check===undefined?''')
sub(A,'const refusal=verdict?.ok===false?{code:verdict.digest,ok:false as const,reason:verdict.reason.slice(0,512)}:undefined,failures=','const shownNow=shown(),refusal=shownNow===null?undefined:{code:shownNow.code,ok:false as const,reason:shownNow.reason.slice(0,512)},failures=')
sub(A,'const next=JSON.stringify([snapshot.sync,','const next=JSON.stringify([options.fallback,snapshot.sync,')
sub(A,'...refusal===undefined?{}:{pending:refusal},...failures.length===0?{}:{failures}});','...refusal===undefined?{}:{pending:refusal},...failures.length===0?{}:{failures},...options.fallback===undefined?{}:{fallback:options.fallback}});')
def schema(path,old,new):
    sub(path,old,new)
S='🩺️attest/🧬️schema/🔣️.json'
sub(S,'"kind": {"enum": ["git-version", "volume", "over-budget", "symlinks"]},','"kind": {"enum": ["git-version", "volume", "over-budget", "symlinks", "code-fallback"]},')
sub(S,'''      "properties": {"kind": {"type": "string", "pattern": "^[a-z][a-z0-9-]{0,39}$"}, "count": {"$ref": "#/definitions/Count"}, "lastMs": {"$ref": "#/definitions/Count"}}
     }
    }
   }
  },
  "Retirement": {''','''      "properties": {"kind": {"type": "string", "pattern": "^[a-z][a-z0-9-]{0,39}$"}, "count": {"$ref": "#/definitions/Count"}, "lastMs": {"$ref": "#/definitions/Count"}}
     }
    },
    "fallback": {
     "type": "object",
     "additionalProperties": false,
     "required": ["guard", "reason", "source"],
     "properties": {"guard": {"const": "code-fallback"}, "source": {"enum": ["snapshot", "store"]}, "reason": {"type": "string", "maxLength": 512}}
    }
   }
  },
  "Retirement": {''')
T='🔁️replica/📖️texts/🔣️.json'
sub(T,'''    "fleet.ok": {''','''    "fleet.warning.code-fallback": {
      "en": "  warning: {origin} runs a fallback code ({detail}) instead of the code of its worktree",
      "de": "  Warnung: {origin} führt einen Ersatz-Code aus ({detail}) statt des Codes seines Arbeitsverzeichnisses"
    },
    "fleet.ok": {''')
sub(A,'    if(digest===started){adopt=undefined;verdict=undefined;admission=undefined;}','    if(digest===started&&options.fallback===undefined){adopt=undefined;verdict=undefined;admission=undefined;}')
sub(A,'now-bootAt>=(options.minimumUptimeMilliseconds??30_000)&&','now-bootAt>=Math.max(options.minimumUptimeMilliseconds??30_000,digest===started?options.returnMilliseconds??300_000:0)&&')
sub(A,'admit?:(digest:string,signal?:AbortSignal)=>Promise<Readonly<{ok:true}>|Readonly<{ok:false;reason:string}>>;fallback?:FleetFallbackV1;','admit?:(digest:string,signal?:AbortSignal)=>Promise<Readonly<{ok:true}>|Readonly<{ok:false;reason:string}>>;fallback?:FleetFallbackV1;returnMilliseconds?:number;')
