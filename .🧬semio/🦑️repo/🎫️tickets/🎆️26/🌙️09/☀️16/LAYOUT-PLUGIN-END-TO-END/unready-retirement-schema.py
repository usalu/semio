from pathlib import Path
import json,copy
root=Path.cwd();ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
base=root/"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage"
p=base/"🧬️schema/📥️install.json";document=json.loads(p.read_text());d=document["$defs"]
assert len(d["SelectedRetirementSelection"]["anyOf"])==2
unready=copy.deepcopy(d["SelectedRetirementSelection"]["anyOf"][1]);unready["properties"]["kind"]={"const":"unready-generation"}
capsule=copy.deepcopy(d["SelectedRetirementSelection"]["anyOf"][0]);capsule["properties"]["kind"]={"const":"retained-unready"};capsule["properties"]["prefix"]["pattern"]=r"^\.p[0-9a-f]{8}$"
d["SelectedRetirementSelection"]["anyOf"].extend([unready,capsule])
snapshot=d["SelectedRetirementSnapshot"];snapshot["properties"]["kind"]["enum"].extend(["unready-generation","retained-unready"]);snapshot["properties"]["prefix"]["pattern"]=r"^(?:[a-f0-9]{20}|\.p[0-9a-f]{8})$"
relations=[]
for kind in ["inactive-generation","empty-generation","unready-generation","retained-unready"]:
    stamped=kind in ["inactive-generation","retained-unready"]
    relations.append({"properties":{"kind":{"const":kind},"prefix":{"type":"string","pattern":r"^\.p[0-9a-f]{8}$" if kind=="retained-unready" else "^[a-f0-9]{20}$"},"generation":{"type":"string","pattern":"^[a-f0-9]{64}$"} if stamped else {"type":"null"},"receiptFingerprint":{"type":"string","minLength":1,"maxLength":1024} if stamped else {"type":"null"}}})
snapshot["allOf"]=[{"anyOf":relations}]
role=copy.deepcopy(d["SelectedRetirementRole"]);role["required"].append("container");role["properties"]["container"]={"enum":["unready-generation","retained-unready"]};d["UnreadyRetirementRole"]=role
d["UnreadyRetirementProtocol"]={"const":{"version":1,"containers":["unready-generation","retained-unready"],"capsuleRoles":["i","p","r"],"capsulePayload":"p","capsuleReceiptLast":["i","r"],"readyMustBeAbsent":True,"historicalMetadataAuthority":False,"freshInactivity":"actual-native-bindings-all-source-all-pins-and-reference-exclusion","wholeAllocationRetired":False}}
p.write_text(json.dumps(document,ensure_ascii=False,indent=2)+"\n")
rows=[]
def add(id,container,parts,kind,accepted,members=1):rows.append({"id":id,"container":container,"parts":parts,"kind":kind,"members":members,"accepted":accepted})
for container in ["unready-generation","retained-unready"]:
    pre=["p"] if container=="retained-unready" else []
    add(container+"-package",container,pre+["package.json"],"file",True)
    add(container+"-modules",container,pre+["node_modules"],"directory",True)
    add(container+"-module-leaf",container,pre+["node_modules","a","index.js"],"file",True)
    add(container+"-borrowed-slot",container,pre+["w","0","node_modules"],"link",True)
    add(container+"-ready",container,pre+["bindings.json"],"file",False)
    add(container+"-outside-slot",container,pre+["w","1","package.json"],"file",False)
    add(container+"-unknown",container,pre+["unknown"],"file",False)
    add(container+"-escape",container,pre+["node_modules",".."],"directory",False)
add("capsule-intent","retained-unready",["i"],"file",True)
add("capsule-receipt","retained-unready",["r"],"file",True)
add("capsule-payload","retained-unready",["p"],"directory",True)
add("capsule-payload-link","retained-unready",["p"],"link",False)
add("capsule-intent-child","retained-unready",["i","data"],"file",False)
add("capsule-unknown","retained-unready",["other"],"directory",False)
corpus={"version":1,"roles":rows}
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","roles"],"properties":{"version":{"const":1},"roles":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","container","parts","kind","members","accepted"],"properties":{"id":{"type":"string"},"container":{"enum":["unready-generation","retained-unready"]},"parts":{"type":"array","minItems":1,"items":{"type":"string"}},"kind":{"enum":["file","directory","link"]},"members":{"type":"integer","minimum":0},"accepted":{"type":"boolean"}}}}}}
out=base/"🧹️retirement/🗃️unready/🧪️tests"
for path,value in [(out/"♻️roles.json",corpus),(out/"♻️roles.schema.json",schema)]:
    assert not path.exists();path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n")
code=r'''import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "./♻️roles.json" with {type:"json"};
import schema from "./♻️roles.schema.json" with {type:"json"};

test("actual never-ready payload roles match independent SQLite and refuse ready admission",async()=>{
 const api=await import("../🟦️.ts"),read=(api as Record<string,unknown>).readJavascriptDependencyUnreadyRetirementRoleV1;
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);expect(typeof read).toBe("function");if(typeof read!=="function")throw Error("Actual never-ready role reader required");
 const db=new Database(":memory:");try{for(const row of corpus.roles){const input={container:row.container,parts:row.parts,kind:row.kind,members:row.members};let accepted=false,result:unknown;try{result=read(input);accepted=true;}catch{}
 const prefix=row.container==="retained-unready"?1:0,payload=row.parts.slice(prefix),first=payload[0],count=payload.length;
 const oracle=db.query("select case when ?1='retained-unready' and ?2 in ('i','r') then ?3=1 and ?4='file' when ?1='retained-unready' and ?2<>'p' then 0 when ?1='retained-unready' and ?3=1 then ?4='directory' else case when ?5='package.json' or ?5='bun.lock' or ?5='bunfig.toml' then ?6=1 and ?4='file' when ?5='node_modules' then (?6=1 and ?4='directory') or (?6>1 and not exists(select1 from json_each(?7) where value in ('..','.'))) when ?5='w' then ?8>0 and (?6=1 and ?4='directory' or ?6=2 and json_extract(?7,'$[1]')='0' and ?4='directory' or ?6=3 and json_extract(?7,'$[1]')='0' and (json_extract(?7,'$[2]')='package.json' and ?4='file' or json_extract(?7,'$[2]')='node_modules' and ?4 in ('directory','link'))) else 0 end end accepted".replace("select1","select 1")).get(row.container,row.parts[0]!,row.parts.length,row.kind,first??"",count,JSON.stringify(payload),row.members) as {accepted:number};
 expect(accepted,row.id).toBe(row.accepted);expect(Boolean(oracle.accepted),row.id).toBe(row.accepted);if(accepted){expect(result).toEqual(input);expect(Object.isFrozen(result)).toBe(true);expect(Object.isFrozen((result as {parts:unknown}).parts)).toBe(true);}}}finally{db.close();}
});
'''
path=out/"♻️roles.ts";assert not path.exists();path.write_text(code)
print(json.dumps({"schemaFirst":True,"roleCases":len(rows),"functionImplemented":False}))
