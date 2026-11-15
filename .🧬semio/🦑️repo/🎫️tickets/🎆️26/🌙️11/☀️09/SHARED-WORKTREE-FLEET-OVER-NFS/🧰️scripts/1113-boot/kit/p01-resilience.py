import sys;sys.path.insert(0,__import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from lib import *
R='🛡️resilience/🟦️.ts'
sub(R,'requireResilienceShapeV1(definition:"SealedCode"|"SelfCheck"|"Health"|"Failure"|"History"|"Fixture"','requireResilienceShapeV1(definition:"SealedCode"|"SelfCheck"|"Health"|"Failure"|"History"|"Beat"|"Fallback"|"BootStage"|"Fixture"')
sub(R,'  summary:(limit=8)=>[...kinds].map(([kind,value])=>({kind,count:value.count,lastMs:value.lastMs})).sort(','  summary:(limit=8)=>[...[...kinds].map(([kind,value])=>({kind,count:value.count,lastMs:value.lastMs})).filter(item=>item.kind!=="loop-blocked"),...blockedFailuresV1()].sort(')
append(R,files('resilience-append.ts'))
def schema(s):
    d=s['definitions']
    d['Beat']={"description":"beat.json: written every second by the watchdog thread of the daemon, from a thread that a blocked event loop cannot stop: the process, its supervisor, the phase, the code and the lag of the event loop in milliseconds.","type":"object","additionalProperties":False,"required":["base","lagMs","parent","phase","pid","source","startedMs","version","wallMs"],"properties":{"version":{"const":1},"pid":{"type":"integer","minimum":1},"parent":{"type":"integer","minimum":0},"startedMs":{"$ref":"#/definitions/Count"},"wallMs":{"$ref":"#/definitions/Count"},"phase":{"enum":["boot","open","run"]},"base":{"type":"string","minLength":1},"source":{"enum":["worktree","snapshot","store"]},"lagMs":{"$ref":"#/definitions/Count"}}}
    d['Fallback']={"description":"The guard of a machine that runs a fallback code: handed to the daemon in SEMIO_FLEET_CODE_FALLBACK and shown in the attestation.","type":"object","additionalProperties":False,"required":["guard","reason","source"],"properties":{"guard":{"const":"code-fallback"},"source":{"enum":["snapshot","store"]},"reason":{"type":"string","maxLength":512}}}
    d['BootStage']={"description":"stage.json: the signature of the copy of the boot stage outside the module.","type":"object","additionalProperties":False,"required":["bytes","sha256","version","wallMs"],"properties":{"version":{"const":1},"sha256":{"$ref":"#/definitions/Sha256"},"bytes":{"$ref":"#/definitions/Count"},"wallMs":{"$ref":"#/definitions/Count"}}}
    d['StoreCodeCase']={"type":"object","additionalProperties":False,"required":["expected","files","name"],"properties":{"name":{"type":"string","minLength":1},"files":{"type":"array","items":{"type":"object","additionalProperties":False,"required":["path","text"],"properties":{"path":{"$ref":"#/definitions/RelativePath"},"text":{"type":"string"}}}},"expected":{"type":"array","items":{"$ref":"#/definitions/SealedFile"}}}}
    f=d['Fixture']
    f['required']=sorted(f['required']+['store'])
    f['properties']['store']={"type":"array","items":{"$ref":"#/definitions/StoreCodeCase"}}
json_edit('🛡️resilience/🧬️schema/🔣️.json',schema)
def fixture(f):
    import hashlib
    files=[("m/a/🟦️.ts","import d from \"./data.json\" with {type:\"json\"};\nexport const a=d;\n"),("m/a/data.json","{\"n\":1}"),("m/b/📜️script.ts","#!/usr/bin/env repo\nexport const b=2;\nexport const text=\"héllo ✓\";\n")]
    f['store']=[{"name":"a module with an imported schema and an entry script","files":[{"path":p,"text":t} for p,t in files],"expected":[{"path":p,"bytes":len(t.encode()),"sha256":hashlib.sha256(t.encode()).hexdigest()} for p,t in sorted(files,key=lambda i:i[0].encode())]}]
json_edit('🛡️resilience/🧪️tests/🧫️fixtures/🔣️.json',fixture)
