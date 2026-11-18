#!/usr/bin/env python3
import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
C=L+"⚡️caching/🚀️bootstrap/🛠️tools/"
LIVE={"formula":C+"📦️sdk/🎟️capacity/🧮️formula/🟦️.ts","policy":C+"📦️sdk/🎟️capacity/📋️policy.json","schema":C+"📦️sdk/🎟️capacity/🧬️schema.json","texts":L+"🏗️build/📖️texts/🔣️.json","headroom":C+"🧹️retention/📏️headroom/🟦️.ts","tools":C+"🟦️.ts","sealed":C+"📦️sdk/🎟️capacity/🧾️sealed-plan/🟦️.ts"}
def sub(t,old,new):
    assert t.count(old)==1,(old[:70],t.count(old))
    return t.replace(old,new)
FORMULA='''import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{refuseBuildV1}from"../../../../../../🏗️build/📖️texts/🟦️.ts";
import schema from"../🧬️schema.json"with{type:"json"};
import policy from"../📋️policy.json"with{type:"json"};
import snapshotPolicy from"../../../🔏️authorizers/📦️snapshot/🔣️policy.json"with{type:"json"};
import derivativeSchema from"../../📥️source/🔗️derivative/🧬️schema/🔣️.json"with{type:"json"};
const clonePublicationBytes=2*snapshotPolicy.maximumSnapshotBytes+2*snapshotPolicy.maximumOutputBytes+policy.maximumInputBytes;
function derivativePublicationBytes(inputBytes:number):number{return policy.derivativeSnapshotCopies*snapshotPolicy.maximumSnapshotBytes+policy.derivativeProgramCopies*snapshotPolicy.maximumOutputBytes+inputBytes+derivativeSchema.definitions.Policy.const.maximumIndexBytes+policy.maximumDerivativePatchBytes+policy.maximumDerivativeReceiptBytes;}
function refusal(derivative:string,need:number,ceiling:number,free:number|undefined){return refuseBuildV1("refused","sdk.derivative-capacity",{sdk:"nx",derivative,need:String(need),ceiling:String(ceiling),free:free===undefined?"?":String(free)});}
function headroom(derivative:string,value:unknown,publicationBytes:number,free:number|undefined):number{if(validateJsonSchemaSubset(schema.definitions.Material,value,schema).length)throw Error("Closed actual SDK clone material required");const row=value as{entries:number;copyAllocatedBytes:number},bytes=row.copyAllocatedBytes+(row.entries+policy.publicationEntries)*policy.metadataBytesPerEntry+publicationBytes;if(!Number.isSafeInteger(bytes)||bytes>policy.maximumHeadroomBytes)throw refusal(derivative,bytes,policy.maximumHeadroomBytes,free);return bytes;}
/** 🧮️ Charges copied file extents, namespace metadata and the declared artefact sizes of the compiler publication; a need above the declared headroom is the typed refusal `sdk.derivative-capacity` with the free bytes of the tool store. */
export function nxSdkCloneHeadroomBytesV1(value:unknown,freeBytes?:number):number{
 return headroom("clone",value,clonePublicationBytes,freeBytes);
}
/** 🔗️ Funds the exact bounded derivative writers, including index sealing and patch replacement; the publication ceiling is derived from the same declared artefact sizes, so no artefact policy can outgrow it. */
export function nxSdkDerivativeHeadroomBytesV1(value:unknown,freeBytes?:number):number{if(validateJsonSchemaSubset(schema.definitions.DerivativeMaterial,value,schema).length)throw Error("Closed actual SDK derivative material required");const row=value as{version:1;entries:number;copyAllocatedBytes:number;inputBytes:number},publicationBytes=derivativePublicationBytes(row.inputBytes),ceiling=derivativePublicationBytes(policy.maximumInputBytes);if(!Number.isSafeInteger(publicationBytes)||publicationBytes>ceiling)throw refusal("derivative",publicationBytes,ceiling,freeBytes);return headroom("derivative",{version:row.version,entries:row.entries,copyAllocatedBytes:row.copyAllocatedBytes},publicationBytes,freeBytes);}
'''
TEXT_ANCHOR='    "graph.plan-not-closed": {\n'
TEXT_NEW='''    "sdk.derivative-capacity": {
      "en": "Not enough storage to publish the {derivative} of the {sdk} SDK: it needs {need} bytes, the declared ceiling is {ceiling} bytes and {free} bytes are free on the tool store. Free disk space or reduce the SDK; nothing was written",
      "de": "Zu wenig Speicher, um das {derivative} des {sdk}-SDK zu veröffentlichen: es braucht {need} Bytes, die deklarierte Obergrenze ist {ceiling} Bytes und im Werkzeugspeicher sind {free} Bytes frei. Speicherplatz freigeben oder das SDK verkleinern; es wurde nichts geschrieben"
    },
'''
def edit(k,t):
    if k=="sealed": return open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"sealed-plan.ts"),encoding="utf8").read()
    if k=="formula": return FORMULA
    if k=="policy": return sub(t,'  "maximumPublicationBytes": 134217728,\n','')
    if k=="schema": return sub(t,'        "maximumPublicationBytes": 134217728,\n','')
    if k=="texts": return sub(t,TEXT_ANCHOR,TEXT_NEW+TEXT_ANCHOR)
    if k=="headroom":
        t=sub(t,'import schema from"./🧬️schema/🔣️.json"with{type:"json"};','import{refuseBuildV1}from"../../../../../🏗️build/📖️texts/🟦️.ts";\nimport schema from"./🧬️schema/🔣️.json"with{type:"json"};')
        return sub(t,'throw Error("Released managed tooling capacity required before SDK publication: "+JSON.stringify({availableBytes:facts.availableBytes,requiredBytes:facts.requiredBytes}));','throw refuseBuildV1("refused","sdk.derivative-capacity",{sdk:"nx",derivative:"publication",need:String(facts.requiredBytes),ceiling:String(facts.availableBytes),free:String(facts.availableBytes)});')
    if k=="tools":
        t=sub(t,'import snapshotPolicy from"./🔏️authorizers/📦️snapshot/🔣️policy.json" with { type: "json" };','import snapshotPolicy from"./🔏️authorizers/📦️snapshot/🔣️policy.json" with { type: "json" };\nimport{refuseBuildV1}from"../../../🏗️build/📖️texts/🟦️.ts";')
        t=sub(t,'throw new Error("SDK derivative compiler publication byte ceiling exceeded");','throw refuseBuildV1("refused","sdk.derivative-capacity",{sdk:"nx",derivative:"compiler",need:String(Math.max(captured.byteLength,sealed.bytes.byteLength)),ceiling:String(captured.byteLength>snapshotPolicy.maximumSnapshotBytes?snapshotPolicy.maximumSnapshotBytes:snapshotPolicy.maximumOutputBytes),free:"?"});')
        t=sub(t,'import{refuseBuildV1}from"../../../🏗️build/📖️texts/🟦️.ts";','import{refuseBuildV1}from"../../../🏗️build/📖️texts/🟦️.ts";\nimport{sealedSdkIndexV1,sdkIndexCopyBytesV1}from"./📦️sdk/🎟️capacity/🧾️sealed-plan/🟦️.ts";')
        t=sub(t,',index=readNxSdkSourceIndexV1(JSON.parse(bytes.toString()));if(index.snapshot!==captured.sdkDigest||JSON.stringify(index)+"\\n"!==bytes.toString())throw new Error("Exact planned sealed SDK index required");',',index=sealedSdkIndexV1(bytes,captured.sdkDigest);')
        t=sub(t,'let copyAllocatedBytes=0,ownedBytes=0,count=0;for(const row of index.entries){checkResourceLeaseControl(control);if(row.kind==="file"&&nxSdkDerivativeDispositionV1({path:row.path,kind:row.kind})==="copy"){ownedBytes+=row.bytes!;copyAllocatedBytes+=Math.ceil(row.bytes!/unit)*unit;}if(++count%128===0){await control.publish({resource:"nx:storage-planning",mode:"shared",elapsedMs:0});await control.yieldContinuation();checkResourceLeaseControl(control);}}if(ownedBytes>','const{copyAllocatedBytes,ownedBytes}=await sdkIndexCopyBytesV1(index,unit,()=>checkResourceLeaseControl(control),async()=>{await control.publish({resource:"nx:storage-planning",mode:"shared",elapsedMs:0});await control.yieldContinuation();});if(ownedBytes>')
        return sub(t,'minimumAvailableBytes=nxSdkDerivativeHeadroomBytesV1(material);','minimumAvailableBytes=nxSdkDerivativeHeadroomBytesV1(material,Number(facts.bavail*facts.bsize));')
    raise SystemExit(k)
def paths(root):
    return {k:(LIVE[k] if root=="live" else os.path.join(root,k+(".ts" if k in("formula","headroom","tools","sealed") else ".json"))) for k in LIVE}
if __name__=="__main__":
    root=sys.argv[1];P=paths(root)
    for k,p in P.items():
        t=open(p,encoding="utf8").read();n=edit(k,t);open(p,"w",encoding="utf8").write(n)
        print(k,hashlib.sha256(n.encode()).hexdigest()[:12])
