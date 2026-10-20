#!/usr/bin/env python3
"""Generates staged/sb-schema.json: the closed schema of the sandbox protocol, its corpus, the debt ledger and the guard report rows. usage: gen_schema.py"""
import json,os
HERE=os.path.dirname(os.path.abspath(__file__))
S={"type":"string","minLength":1}
def obj(props,req=None,extra=False): return {"type":"object","additionalProperties":extra,"required":req if req is not None else list(props),"properties":props}
msg=obj({"en":S,"de":S})
rule={"type":"object","additionalProperties":False,"required":["id","severity","languages","scope","message"],"properties":{
 "id":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},"severity":{"enum":["error","warn"]},
 "languages":{"type":"array","minItems":1,"uniqueItems":True,"items":{"enum":["ts","go","rs","py","sh"]}},
 "scope":{"enum":["line","file","owner"]},"flags":{"enum":["i"]},"pattern":S,
 "also":{"type":"array","items":S},"exempt":{"type":"array","items":S},"all":{"type":"array","minItems":1,"items":S},"none":{"type":"array","items":S},"message":msg}}
env=obj({k:{"type":"string","pattern":"^SEMIO_TEST_[A-Z_]+$"} for k in ["sandbox","artifacts","images","cache","guard","report","checkout"]})
lang_map=lambda: {"type":"object","additionalProperties":S,"required":["ts","go","rs","py","sh"],"properties":{k:S for k in ["ts","go","rs","py","sh"]}}
proto=obj({
 "version":{"const":1},"owner":{"type":"string","pattern":"^repository:[a-z][a-z0-9-]*$"},"environment":env,
 "layout":obj({"parent":{"type":"string","pattern":"^[a-z0-9]{1,8}$"},"owner":S,"directories":{"type":"array","minItems":1,"items":S},"maximumRootLength":{"type":"integer","minimum":40,"maximum":120},"heartbeatMilliseconds":{"type":"integer","minimum":1000},"staleHeartbeatMilliseconds":{"type":"integer","minimum":60000},"graceMilliseconds":{"type":"integer","minimum":1000},"reclaimIntervalMilliseconds":{"type":"integer","minimum":1000},"stamp":S}),
 "ports":obj({"minimum":{"type":"integer","minimum":1024,"maximum":65535},"maximum":{"type":"integer","minimum":1024,"maximum":65535},"attempts":{"type":"integer","minimum":1,"maximum":1000},"directory":S}),
 "guard":obj({"modes":{"type":"array","items":{"enum":["off","report","enforce"]}},"defaultMode":{"enum":["off","report","enforce"]},
  "writeTargets":{"type":"object","additionalProperties":{"type":"array","minItems":1,"items":{"type":"integer","minimum":0,"maximum":3}}},
  "openFunctions":{"type":"array","items":S},"kernelPrograms":{"type":"array","items":S},"killPrograms":{"type":"array","items":S},"kernelVerbs":{"type":"array","items":S},
  "protectedCheckoutPrefixes":{"type":"array","items":S},"allowedCheckoutPrefixes":{"type":"array","items":S},"scratchRoots":{"type":"array","items":S}}),
 "scan":obj({"testSegment":S,"skipSegments":{"type":"array","items":S},"extensions":{"type":"object","additionalProperties":{"type":"array","items":S},"required":["ts","go","rs","py","sh"],"properties":{k:{"type":"array","items":S} for k in ["ts","go","rs","py","sh"]}},"maximumBytes":{"type":"integer","minimum":1024},"suite":lang_map(),"commentLine":lang_map()}),
 "rules":{"type":"array","minItems":1,"items":rule}})
ledger=obj({"version":{"const":1},"entries":{"type":"array","items":obj({"file":S,"rule":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},"count":{"type":"integer","minimum":1},"reason":S})}})
loc=obj({"rule":{"type":"string"},"line":{"type":"integer","minimum":1}})
ctx=obj({"cwd":S,"platform":{"enum":["linux","darwin","win32"]},"checkout":S,"allowed":{"type":"array","items":S},"sandboxRoots":{"type":"array","items":S},"protectedPrefixes":{"type":"array","items":S},"allowedCheckoutPrefixes":{"type":"array","items":S}})
nul=lambda enum:{"enum":enum+[None]}
corpus=obj({"version":{"const":1},
 "scans":{"type":"array","items":obj({"id":S,"path":S,"language":{"enum":["ts","go","rs","py","sh"]},"text":{"type":"string"},"expected":{"type":"array","items":loc}})},
 "owners":{"type":"array","items":obj({"id":S,"sources":{"type":"array","items":obj({"language":{"enum":["ts","go","rs","py","sh"]},"text":{"type":"string"}})},"expected":{"type":"array","items":obj({"rule":S})}})},
 "paths":{"type":"array","items":obj({"path":S,"test":{"type":"boolean"},"declared":{"type":"array","items":S}},["path","test"])},
 "writes":{"type":"array","items":obj({"id":S,"context":ctx,"function":S,"target":S,"expected":nul(["checkout-write","protected-write"])})},
 "spawns":{"type":"array","items":obj({"id":S,"context":ctx,"command":S,"args":{"type":"array","items":{"type":"string"}},"expected":nul(["kernel-image","process-kill"])})},
 "listens":{"type":"array","items":obj({"id":S,"context":ctx,"port":{"type":"integer"},"leased":{"type":"array","items":{"type":"integer"}},"expected":nul(["fixed-port"])})}})
report=obj({"time":{"type":"integer"},"pid":{"type":"integer"},"kind":{"enum":["checkout-write","protected-write","kernel-image","process-kill","fixed-port"]},"function":S,"target":{"type":"string"},"frame":{"type":"string"}})
schema={"$schema":"http://json-schema.org/draft-07/schema#","$id":"urn:semio:framework:process:testing:sandbox","title":"Per-run test sandbox protocol","description":"Closed declaration of the per-run test sandbox: environment names, directory layout, port lease range, runtime guard policy and the static concurrency-law rules, plus the language-agnostic corpus, the debt ledger and the guard report row it is verified with.",
 "definitions":{"Protocol":proto,"Rule":rule,"Corpus":corpus,"Ledger":ledger,"GuardReportRow":report}}
json.dump(schema,open(HERE+"/staged/sb-schema.json","w",encoding="utf8"),ensure_ascii=False,indent=1); open(HERE+"/staged/sb-schema.json","a").write("\n")
print("schema ok")
