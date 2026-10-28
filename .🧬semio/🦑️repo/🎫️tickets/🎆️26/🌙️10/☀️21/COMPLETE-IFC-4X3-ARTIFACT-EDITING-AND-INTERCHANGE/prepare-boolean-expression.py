"""🌳️ Hand-author nested/DAG/cycle IFC expression obligations before the adapter."""
from pathlib import Path
import json

ticket=Path(__file__).parent
tests=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests"
def block(key,origin,size):return {"key":key,"kind":"block","origin":origin,"size":size}
def operation(key,operator,first,second):return {"key":key,"kind":"boolean","operator":operator,"first":first,"second":second}
host=block("host",[0,0,0],[4,4,4])
cut=block("cut",[2,0,0],[4,4,4])
first=operation("first","DIFFERENCE","host","cut")
rows=[
 {"id":"nested-two-cuts","root":"final","rootForm":"csg","nodes":[host,cut,first,block("secondCut",[0,0,0],[1,4,4]),operation("final","DIFFERENCE","first","secondCut")],"expected":"mesh","volume":16,"physical":"valid"},
 {"id":"repeated-dag-union","root":"final","rootForm":"direct","nodes":[host,cut,first,operation("final","UNION","first","first")],"expected":"mesh","volume":32,"physical":"valid"},
 {"id":"disconnected-then-intersection","root":"final","rootForm":"csg","nodes":[host,block("far",[6,0,0],[4,4,4]),operation("joined","UNION","host","far"),block("window",[0,0,0],[8,4,4]),operation("final","INTERSECTION","joined","window")],"expected":"mesh","volume":96,"physical":"valid"},
 {"id":"empty-child-union","root":"final","rootForm":"direct","nodes":[host,operation("empty","DIFFERENCE","host","host"),operation("final","UNION","empty","host")],"expected":"mesh","volume":64,"physical":"valid"},
 {"id":"shared-subtree-full-subtraction","root":"final","rootForm":"csg","nodes":[host,cut,first,operation("final","DIFFERENCE","first","first")],"expected":"empty","volume":0,"physical":"valid"},
 {"id":"active-path-cycle","root":"cycle","rootForm":"csg","nodes":[host,operation("cycle","DIFFERENCE","cycle","host")],"expected":"unsupported","volume":None,"physical":"cyclic-not-sdk-evaluated"},
 {"id":"dangling-operand","root":"final","rootForm":"direct","nodes":[host,operation("final","DIFFERENCE","host","missing")],"expected":"unsupported","volume":None,"physical":"malformed"}
]
point={"type":"array","minItems":3,"maxItems":3,"items":{"type":"number"}}
key={"type":"string","minLength":1}
def node(required,properties):return {"type":"object","additionalProperties":False,"required":required,"properties":properties}
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":7,"maxItems":7,"items":node(["id","root","rootForm","nodes","expected","volume","physical"],{"id":key,"root":key,"rootForm":{"enum":["csg","direct"]},"nodes":{"type":"array","minItems":2,"maxItems":8,"items":{"oneOf":[node(["key","kind","origin","size"],{"key":key,"kind":{"const":"block"},"origin":point,"size":{**point,"items":{"type":"number","exclusiveMinimum":0}}}),node(["key","kind","operator","first","second"],{"key":key,"kind":{"const":"boolean"},"operator":{"enum":["DIFFERENCE","UNION","INTERSECTION"]},"first":key,"second":key})]}},"expected":{"enum":["mesh","empty","unsupported"]},"volume":{"anyOf":[{"type":"null"},{"type":"number","minimum":0}]},"physical":{"enum":["valid","cyclic-not-sdk-evaluated","malformed"]}})}}}
for name,value in [("boolean-expression-cases.json",{"version":1,"cases":rows}),("boolean-expression.schema.json",schema)]:
 path=tests/name
 assert not path.exists()
 path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] seven actual expression contracts prepared")
