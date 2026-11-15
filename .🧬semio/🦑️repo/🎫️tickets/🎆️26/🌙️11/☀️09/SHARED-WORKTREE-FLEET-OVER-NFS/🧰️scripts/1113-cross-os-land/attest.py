from lib import *
P="🩺️attest/🟦️.ts"
def code(t):
    s=read(src(P))
    t=sub(t,'/** 🩺️ What one worker states about itself in `origins/<origin>/attest.json`: identity, clocks, replication cursors, the order-independent digest of its shared files, the git state, the working-tree view and the digest of its own code. */',between(s,"/** 🛡️ What a worker measures","/** 🩺️ What one worker states")+'/** 🩺️ What one worker states about itself in `origins/<origin>/attest.json`: identity, clocks, replication cursors, the order-independent digest of its shared files, the git state, the working-tree view, the guards of that view and the digest of its own code. */')
    t=sub(t,'view:FleetViewV1|null;latency:LatencyReportV1;queue:','view:FleetViewV1|null;guards:FleetGuardsV1|null;latency:LatencyReportV1;queue:')
    t=sub(t,'/** 🔎️ The fields over which settled workers must agree. */\nexport type FleetFieldV1="manifest"|"head.commit"|"indexTree"|"view.digest";',between(s,"/** 🔎️ The fields over which settled workers","/** ⚖️ The verdict of one worker"))
    t=sub(t,'differing:readonly FleetDifferenceV1[]}>;','differing:readonly FleetDifferenceV1[];warnings:readonly FleetWarningV1[]}>;')
    old=between(t,"const valueOf=","\n/** ⚖️ The pure fleet verdict.")
    new=between(s,"const valueOf=","\n/** ⚖️ The pure fleet verdict.")
    new=new.replace('if(field==="view.digest")return attestation.view?.digest??null;','if(field==="view.digest")return attestation.view?.stale===true?null:attestation.view?.digest??null;')
    t=t.replace(old,new,1)
    t=sub(t,'const voters=field==="view.digest"?live.filter(origin=>valueOf(input.attestations[origin]!,field)!==null):live,counts','const voters=field==="manifest"||field==="head.commit"||field==="indexTree"?live:live.filter(origin=>valueOf(input.attestations[origin]!,field)!==null),counts')
    t=sub(t,'freshSeconds:freshSecondsV1,rows:finished,differing};','freshSeconds:freshSecondsV1,rows:finished,differing,warnings:capabilities(live,input.attestations)};')
    t=sub(t,'Origins with sync on that are fresh and settled must agree on `manifest`, `head.commit`, `indexTree` and `view.digest` (an origin whose `view` is `null` has no git and is exempt from the last)','Origins with sync on that are fresh and settled must agree on `manifest`, `head.commit`, `indexTree`, `view.digest` and the divergence-producing `guards.*` fields; differences of capability (git version, volume behaviour, paths over the budget, links without link support) are `warnings` and never change a verdict (an origin whose `view` is `null` has no git and is exempt from the view, one whose `guards` are `null` from every `guards.*` field, as is a `null` `modeMismatch`)')
    t=sub(t,'onView?:(view:FleetViewV1)=>void}>;','onView?:(view:FleetViewV1)=>void;guards?:(signal?:AbortSignal)=>Promise<FleetGuardsV1|null>;guardsMilliseconds?:number;guardsMinimumMilliseconds?:number}>;')
    t=sub(t,'viewQuiet=options.viewDebounceMilliseconds??5000;','viewQuiet=options.viewDebounceMilliseconds??5000,guardsEvery=options.guardsMilliseconds??300_000,guardsMinimum=options.guardsMinimumMilliseconds??10_000;')
    t=sub(t,' let viewStale=false,viewState:',' let guardsState:FleetGuardsV1|null|undefined,guardsBusy:Promise<void>|undefined,guardsAt=-Infinity,guardsDigest:string|undefined;\n let viewStale=false,viewState:')
    t=sub(t,' const askGit=(signal?:AbortSignal)',''' const askGuards=(signal?:AbortSignal):Promise<void>=>{
  guardsAt=options.now();guardsDigest=viewState?.digest;
  return guardsBusy??=(options.guards===undefined?Promise.resolve(null):options.guards(signal)).then(state=>{if(signal?.aborted!==true)guardsState=state??guardsState??null;},()=>{guardsState??=null;}).finally(()=>{guardsBusy=undefined;});
 };
 const askGit=(signal?:AbortSignal)''')
    t=sub(t,'if(gitState!==undefined&&!gitState.present){viewState=null;viewStale=false;}','if(gitState!==undefined&&!gitState.present){viewState=null;viewStale=false;guardsState=null;}')
    t=sub(t,'void askView(signal);\n   }','void askView(signal);\n   }\n   if(gitState?.present===true&&guardsBusy===undefined&&(guardsState===undefined||now-guardsAt>=guardsEvery||viewState!==undefined&&viewState!==null&&viewState.digest!==guardsDigest&&!viewStale&&now-guardsAt>=guardsMinimum))void askGuards(signal);')
    t=sub(t,'gitState?.head,viewState?.digest,viewStale]);','gitState?.head,viewState?.digest,viewStale,guardsState]);')
    t=sub(t,'latency:options.latency?.()??silentLatencyV1(),queue:{incoming:snapshot.incoming}});','guards:guardsState??null,latency:options.latency?.()??silentLatencyV1(),queue:{incoming:snapshot.incoming}});')
    t=sub(t,' Git is asked at most once per heartbeat, in the background.',' The guards (see `FleetGuardsV1`) are measured in the background after the git state is known, every `guardsMilliseconds` (5 minutes) and, at most every `guardsMinimumMilliseconds` (10 s), after the view digest changed; the attestation states `null` until the first measurement arrives and never waits for it. Git is asked at most once per heartbeat, in the background.') if ' Git is asked at most once per heartbeat, in the background.' in t else t
    return t
edit(P,code)
def schema(t):
    s=read(src("🩺️attest/🧬️schema/🔣️.json"))
    t=sub(t,'  "GitHead": {',between(s,'  "Volume": {','  "GitHead": {')+'  "GitHead": {')
    t=sub(t,'"required": ["code", "cursors", "files", "groupId", "head", "hlc", "indexTree", "latency", "manifest",','"required": ["code", "cursors", "files", "groupId", "guards", "head", "hlc", "indexTree", "latency", "manifest",')
    t=sub(t,'''    "latency": {"$ref": "#/definitions/Latency"},
    "queue": {''','''    "guards": {"anyOf": [{"type": "null"}, {"$ref": "#/definitions/Guards"}]},
    "latency": {"$ref": "#/definitions/Latency"},
    "queue": {''')
    t=sub(t,'  "Field": {"enum": ["manifest", "head.commit", "indexTree", "view.digest"]},',between(s,'  "Field": {','  "Verdict":'))
    t=sub(t,'''   "required": ["differing", "freshSeconds", "ok", "rows", "version"],''','''   "required": ["differing", "freshSeconds", "ok", "rows", "version", "warnings"],''')
    t=sub(t,'''    "differing": {"type": "array", "items": {"$ref": "#/definitions/Difference"}}
   }''','''    "differing": {"type": "array", "items": {"$ref": "#/definitions/Difference"}},
    "warnings": {"type": "array", "items": {"$ref": "#/definitions/Warning"}}
   }''')
    return t
edit("🩺️attest/🧬️schema/🔣️.json",schema)
