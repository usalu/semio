from pathlib import Path
import json,hashlib
ticket=Path(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END")
base=Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🗃️unready")
encode=lambda value:json.dumps(value,ensure_ascii=False,separators=(",",":"))
digest=lambda value:hashlib.sha256(value.encode()).hexdigest()
props={"version":{"const":1},"purpose":{"const":"retained-unready-dependency-generation-data"},"authority":{"const":False},"currentReady":{"const":False},"capsule":{"type":"string","pattern":r"^\.p[0-9a-f]{8}$"},"generation":{"type":"string","pattern":"^[0-9a-f]{64}$"},"scope":{"type":"string","minLength":1,"maxLength":256},"originalDirectory":{"type":"string","minLength":1,"maxLength":256},"recipeSha256":{"type":"string","pattern":"^[0-9a-f]{64}$"},"intentSha256":{"type":"string","pattern":"^[0-9a-f]{64}$"},"receiptSha256":{"type":"string","pattern":"^[0-9a-f]{64}$"}}
schema={"$schema":"http://json-schema.org/draft-07/schema#","title":"RetainedUnreadyDependencyGenerationDataV1","type":"object","additionalProperties":False,"required":list(props),"properties":props}
cases=[]
for mode in ["posix","windows","foreign-intent-hash","foreign-capsule","foreign-generation","foreign-root","unknown-intent-field","delete-payload","adopt-payload","claims-inactive","foreign-identity","noncanonical-intent","absent-source-false","file-payload","malformed-receipt"]:
    windows=mode=="windows"; workspace=r"C:\w" if windows else "/w"; root=workspace+(r"\.cache\b\d\m" if windows else "/.cache/b/d/m"); sep="\\" if windows else "/"; generation="a"*64
    intent={"version":1,"purpose":"retain-current-unready-dependency-generation","scope":".","generation":generation,"directory":root+sep+"j"+sep+generation[:20],"rootStamp":encode(["17","2","16832","2","32","3","4"]),"parentStamp":encode(["17","3","16832","2","64","5","6"]),"rosterSha256":"b"*64,"sourceInputsSha256":"c"*64,"sourceEndpointsSha256":"d"*64,"recipeSha256":"e"*64,"materializationSha256":"f"*64,"fundingSha256":"0"*64,"readyAbsent":True,"directSourceReferences":0,"inactivityProved":False}
    if mode=="foreign-generation":intent["generation"]="b"*64
    if mode=="unknown-intent-field":intent["foreign"]=True
    if mode=="claims-inactive":intent["inactivityProved"]=True
    intent_text=encode(intent)
    if mode=="noncanonical-intent":intent_text=json.dumps(intent,ensure_ascii=False)
    capsule=".p"+digest(intent_text)[:8]
    receipt={"version":1,"purpose":"retain-current-ordinary-entry-without-binding","name":generation[:20],"capsule":capsule,"kind":"directory","identity":{"dev":"17","ino":"2","mode":"16832","size":"32","nlink":"2","mtimeNs":"3","ctimeNs":"5"},"intentSha256":digest(intent_text),"sourceAbsent":True,"deletePayload":False,"adoptPayload":False}
    if mode=="foreign-intent-hash":receipt["intentSha256"]="f"*64
    if mode=="foreign-capsule":capsule=".p12345678"
    if mode=="foreign-root":root="/foreign/m"
    if mode=="delete-payload":receipt["deletePayload"]=True
    if mode=="adopt-payload":receipt["adoptPayload"]=True
    if mode=="foreign-identity":receipt["identity"]["ino"]="9"
    if mode=="absent-source-false":receipt["sourceAbsent"]=False
    if mode=="file-payload":receipt["kind"]="file"
    receipt_text=encode(receipt)+"\n" if mode!="malformed-receipt" else "{"
    accepted=mode in ["posix","windows"]
    expected={"version":1,"purpose":"retained-unready-dependency-generation-data","authority":False,"currentReady":False,"capsule":capsule,"generation":generation,"scope":".","originalDirectory":intent["directory"],"recipeSha256":intent["recipeSha256"],"intentSha256":digest(intent_text),"receiptSha256":digest(receipt_text)} if accepted else None
    cases.append({"id":mode,"workspace":workspace,"root":root,"capsule":capsule,"intentText":intent_text,"receiptText":receipt_text,"accepted":accepted,"expected":expected})
corpus={"version":1,"cases":cases}
caseprops={"id":{"type":"string"},"workspace":{"type":"string"},"root":{"type":"string"},"capsule":{"type":"string"},"intentText":{"type":"string"},"receiptText":{"type":"string"},"accepted":{"type":"boolean"},"expected":{"anyOf":[{"type":"null"},schema]}}
corpus_schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":list(caseprops),"properties":caseprops}}}}
test=r'''import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import corpus from "./🔣️.json" with {type:"json"};
import corpusSchema from "./🧬️schema.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

test("retained unready capsule neutral fixtures carry closed non-authoritative output",()=>{
 const ajv=new Ajv({strict:true});expect(ajv.validate(corpusSchema,corpus)).toBe(true);
 for(const row of corpus.cases)if(row.accepted){expect(ajv.validate(schema,row.expected)).toBe(true);expect(row.expected?.intentSha256).toBe(createHash("sha256").update(row.intentText).digest("hex"));expect(row.expected?.receiptSha256).toBe(createHash("sha256").update(row.receiptText).digest("hex"));}
});

test("actual retained unready capsule reader matches independent SQLite projection and refuses hostile wire",async()=>{
 const api=await import("../🟦️.ts"),read=(api as Record<string,unknown>).readJavascriptDependencyRetainedUnreadyCapsuleDataV1;expect(typeof read).toBe("function");if(typeof read!=="function")throw Error("Actual retained unready capsule reader required");
 const db=new Database(":memory:");try{for(const row of corpus.cases){let accepted=false,result:unknown;try{result=read(row.workspace,row.root,row.capsule,row.intentText,row.receiptText);accepted=true;}catch{}expect(accepted,row.id).toBe(row.accepted);if(accepted){const projection=db.query("select json_object('version',1,'purpose','retained-unready-dependency-generation-data','authority',json('false'),'currentReady',json('false'),'capsule',?1,'generation',json_extract(?2,'$.generation'),'scope',json_extract(?2,'$.scope'),'originalDirectory',json_extract(?2,'$.directory'),'recipeSha256',json_extract(?2,'$.recipeSha256'),'intentSha256',?3,'receiptSha256',?4) text").get(row.capsule,row.intentText,createHash("sha256").update(row.intentText).digest("hex"),createHash("sha256").update(row.receiptText).digest("hex")) as {text:string};expect(result).toEqual(JSON.parse(projection.text));expect(result).toEqual(row.expected);expect(Object.isFrozen(result)).toBe(true);expect(new Ajv({strict:true}).validate(schema,result)).toBe(true);}}}finally{db.close();}
 let hooks=0;const hostile=new Proxy({},{get(){hooks++;throw Error("Getter");},ownKeys(){hooks++;throw Error("Keys");}}),base=corpus.cases[0]!;for(const index of [0,1,2,3,4]){const args:unknown[]=[base.workspace,base.root,base.capsule,base.intentText,base.receiptText];args[index]=hostile;expect(()=>read(...args)).toThrow();}expect(hooks).toBe(0);
});
'''
files={base/"🧬️schema/🔣️.json":json.dumps(schema,ensure_ascii=False,indent=2)+"\n",base/"🧪️tests/🔣️.json":json.dumps(corpus,ensure_ascii=False,indent=2)+"\n",base/"🧪️tests/🧬️schema.json":json.dumps(corpus_schema,ensure_ascii=False,indent=2)+"\n",base/"🧪️tests/🟦️.ts":test,base/"🟦️.ts":"export {};\n"}
for path in files:
    assert not path.exists(),str(path)
    assert len(str(path.resolve()).encode("utf-16-le"))//2<=256
for path,text in files.items():
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(text)
print(encode({"created":len(files),"cases":len(cases),"implementation":"not implemented","longestPath":max(len(str(p.resolve()).encode("utf-16-le"))//2 for p in files)}))
