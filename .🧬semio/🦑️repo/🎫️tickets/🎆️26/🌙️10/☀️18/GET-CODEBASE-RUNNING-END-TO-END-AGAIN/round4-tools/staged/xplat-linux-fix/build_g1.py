import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import span, emit, live
HERE = os.path.dirname(os.path.abspath(__file__))
NB = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
D = NB + "/📁️directory/🟦️.ts"
edits = []
# 1 imports and exports
edits.append((D, 'import type{NativeWindowsHostPortV1}from"./🪟️host/🟦️.ts";\nexport{nativeDirectoryProtocolV1,observeNativeDirectoryIdentityV1,readNativeDirectoryIdentityV1};',
 'import type{NativeWindowsHostPortV1}from"./🪟️host/🟦️.ts";\nimport{measureNativeDirectoryUsageV1,measureNativeDirectoryUsageAsyncV1,nativeDirectoryEntryBoundV1,nativeDirectoryUsageV1,createNativeDirectoryUsageV1,type NativeDirectoryUsageV1}from"./📏️usage/🟦️.ts";\nexport{nativeDirectoryProtocolV1,observeNativeDirectoryIdentityV1,readNativeDirectoryIdentityV1,measureNativeDirectoryUsageV1,measureNativeDirectoryUsageAsyncV1,nativeDirectoryEntryBoundV1,nativeDirectoryUsageV1,createNativeDirectoryUsageV1};\nexport type{NativeDirectoryUsageV1};'))
# 2 remove the old synchronous measure (moved to the usage module)
old_measure = span(D, "/** 🧮️ Measures physical bytes below a root without following links or counting a hard link twice. */", "/** ⚖️ Bounds each payload request")
edits.append((D, old_measure, ""))
# 3 meter
old_meter = span(D, "/** 📏️ Charges every admission against a running estimate", "/** 📜️ Serializes the immutable partition anchor")
new_meter = '''/** 📏️ Charges every admission against a running estimate of the real tree and re-measures the tree periodically, on a commit, and before any refusal; the measurements are shared with every other reader of the tree, never overlap and never block the event loop. */
export type NativeDirectoryMeterV1=Readonly<{admit(request:unknown,signal?:AbortSignal):Promise<void>}>;
export function createNativeDirectoryMeterV1(input:Readonly<{root:string;filesystemBytes:number;maximumPhysicalBytes:number;measure?:(root:string,signal?:AbortSignal)=>number|Promise<number>;usage?:NativeDirectoryUsageV1;now?:()=>number}>):NativeDirectoryMeterV1{
 if(validateJsonSchemaSubset(schema.definitions.MeterLimits,{filesystemBytes:input.filesystemBytes,maximumPhysicalBytes:input.maximumPhysicalBytes},schema).length)throw Error("Closed directory meter limits required");
 const limits=protocol.meter,now=input.now??Date.now,usage=input.measure?undefined:input.usage??nativeDirectoryUsageV1(input.root,input.maximumPhysicalBytes),measure=(signal?:AbortSignal):Promise<number>=>usage?usage.fresh(signal):Promise.resolve(input.measure!(input.root,signal));
 let measured:number|undefined,measuredAt=0,admitted=0;
 const refresh=async(signal?:AbortSignal)=>{const startedAt=now(),before=admitted,bytes=await measure(signal);measured=bytes;measuredAt=startedAt;admitted=Math.max(0,admitted-before);return bytes;};
 const stale=()=>measured===undefined||now()-measuredAt>=limits.intervalMilliseconds;
 const gate=(used:number,requestBytes:number)=>admitNativeDirectoryCapacityV1({filesystemBytes:input.filesystemBytes,maximumPhysicalBytes:input.maximumPhysicalBytes,reservedBytes:0,usedBytes:used,requestBytes});
 return Object.freeze({admit:async(value:unknown,signal?:AbortSignal)=>{
  if(validateJsonSchemaSubset(schema.definitions.MeterRequest,value,schema).length)throw Error("Closed directory meter request required");
  const request=value as Readonly<{mode:"reserve"|"commit"|"recheck";requestBytes:number}>;
  if(request.mode==="commit"){gate(await refresh(signal),0);return;}
  if(request.mode==="recheck"){if(stale())await refresh(signal);gate(measured!+admitted,0);return;}
  if(measured===undefined||admitted+request.requestBytes>=limits.intervalBytes||stale())await refresh(signal);
  if(measured!+admitted+request.requestBytes>input.maximumPhysicalBytes)await refresh(signal);
  gate(measured!+admitted,request.requestBytes);admitted+=request.requestBytes;
 }});
}

'''
edits.append((D, old_meter, new_meter))
# schema usage block
S = NB + "/📁️directory/🧬️schema/🔣️.json"
edits.append((S, '        "meter": {\n          "intervalBytes": 16777216,\n          "intervalMilliseconds": 2000\n        },',
 '        "meter": {\n          "intervalBytes": 16777216,\n          "intervalMilliseconds": 2000\n        },\n        "usage": {\n          "blockBytes": 4096,\n          "entryFactor": 2,\n          "concurrency": 16,\n          "batch": 8,\n          "recentMilliseconds": 5000,\n          "synchronousMaximumAgeMilliseconds": 60000\n        },'))
# fixed port: asynchronous admission with the operation's cancellation
F = NB + "/🧱️fixed/🟦️.ts"
edits.append((F, 'await recheckPhysical(actual);meter.admit(request);await recheckPhysical(actual);', 'await recheckPhysical(actual);await meter.admit(request,actual.signal);await recheckPhysical(actual);'))
T = NB + "/📁️directory/🧪️tests/🟦️.ts"
M = 'createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:33554432,maximumPhysicalBytes:34603008,measure:()=>0})'
tests = [
 ('test("the meter charges every admission against the cap: many small writes, one large write and growth after commit are all refused, an honest writer never is",()=>{', 'test("the meter charges every admission against the cap: many small writes, one large write and growth after commit are all refused, an honest writer never is",async()=>{'),
 ('const attempt=(mode:"reserve"|"commit"|"recheck",requestBytes:number)=>{counts[mode]++;try{meter.admit({mode,requestBytes});return true;}', 'const attempt=async(mode:"reserve"|"commit"|"recheck",requestBytes:number)=>{counts[mode]++;try{await meter.admit({mode,requestBytes});return true;}'),
 ('if(step.reserve>0&&!attempt("reserve",step.reserve))break run;', 'if(step.reserve>0&&!await attempt("reserve",step.reserve))break run;'),
 ('if(step.commit&&!attempt("commit",0))break run;', 'if(step.commit&&!await attempt("commit",0))break run;'),
 ('if(!attempt("recheck",0))break run;', 'if(!await attempt("recheck",0))break run;'),
 ('for(const value of fixture.meterMalformed)expect(()=>' + M + '.admit(value),JSON.stringify(value)).toThrow("Closed directory meter request");', 'for(const value of fixture.meterMalformed)await expect(' + M + '.admit(value),JSON.stringify(value)).rejects.toThrow("Closed directory meter request");'),
 ('test("the meter over a real tree refuses many small files and one large file that bypassed the old sixteen MiB gate",()=>{', 'test("the meter over a real tree refuses many small files and one large file that bypassed the old sixteen MiB gate",async()=>{'),
 ('try{meter.admit({mode:"reserve",requestBytes:524288});writeFileSync', 'try{await meter.admit({mode:"reserve",requestBytes:524288});writeFileSync'),
 ('second.admit({mode:"reserve",requestBytes:1048576});writeFileSync', 'await second.admit({mode:"reserve",requestBytes:1048576});writeFileSync'),
 ('expect(()=>second.admit({mode:"commit",requestBytes:0})).toThrow(protocol.refusals.capacity);', 'await expect(second.admit({mode:"commit",requestBytes:0})).rejects.toThrow(protocol.refusals.capacity);'),
 ('test("directory meter separates metadata, fresh entitlement and recovered committed content",()=>{', 'test("directory meter separates metadata, fresh entitlement and recovered committed content",async()=>{'),
 ('try{meter.admit({mode:step.mode,requestBytes:request});}catch(error){expect((error as Error).message,row.id)', 'try{await meter.admit({mode:step.mode,requestBytes:request});}catch(error){expect((error as Error).message,row.id)'),
]
for old, new in tests:
    edits.append((T, old, new))
edits.append((NB + "/🧱️fixed/🧪️tests/🟦️.ts", 'expect(fixed).toContain("meter.admit(request)");', 'expect(fixed).toContain("await meter.admit(request,actual.signal)");'))
new = [NB + "/📁️directory/📏️usage/🟦️.ts", NB + "/📁️directory/📏️usage/🧪️tests/🟦️.ts", NB + "/📁️directory/📏️usage/🧪️tests/🧫️fixtures/🔣️.json"]
emit("g1", "directory capacity meter: async single-flight measurement, envelope-derived entry bound, shared usage cell", [], edits, new, os.path.join(HERE, "groups"))
print("g1 written", len(edits), "edits")
