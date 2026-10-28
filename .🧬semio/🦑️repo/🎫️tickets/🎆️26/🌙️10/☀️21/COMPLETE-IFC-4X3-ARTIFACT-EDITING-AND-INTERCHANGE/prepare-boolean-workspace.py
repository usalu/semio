"""🧮️ Author neutral live-value ownership programs before implementation."""
from pathlib import Path
import json

ticket=Path(__file__).parent
root=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean"
tests=root/"🧪️tests"
rows=[
 {"id":"retained-siblings-output","limit":192,"steps":[["reserve","left",60,60,60],["reserve","right",60,120,120],["reserve","output",72,192,192],["release","left",0,132,192],["release","right",0,72,192],["reserve","parent",0,72,192],["merge","parent","output",72,192],["release","parent",0,0,192]]},
 {"id":"split-and-transfer","limit":60,"steps":[["reserve","host",60,60,60],["split","host",24,"child",60,60],["merge","host","child",60,60],["release","host",0,0,60]]},
 {"id":"grow-before-backing-copy","limit":15,"steps":[["reserve","first",6,6,6],["reserve","second",6,12,12],["grow","first",3,15,15],["release","first",0,6,15],["release","second",0,0,15]]},
 {"id":"one-over-is-atomic","limit":119,"steps":[["reserve","left",60,60,60],["reserve","right",59,119,119],["denyGrow","right",1,119,119],["release","left",0,59,119],["release","right",0,0,119]]},
 {"id":"exact-safe-integer-frontier","limit":9007199254740991,"steps":[["reserve","large",9007199254740988,9007199254740988,9007199254740988],["reserve","tail",3,9007199254740991,9007199254740991],["denyGrow","tail",1,9007199254740991,9007199254740991],["release","large",0,3,9007199254740991],["release","tail",0,0,9007199254740991]]},
 {"id":"empty-buffer-retains-no-values","limit":1,"steps":[["reserve","empty",0,0,0],["grow","empty",1,1,1],["release","empty",0,0,1]]}
]
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":6,"maxItems":6,"items":{"type":"object","additionalProperties":False,"required":["id","limit","steps"],"properties":{"id":{"type":"string","minLength":1},"limit":{"type":"integer","minimum":1,"maximum":9007199254740991},"steps":{"type":"array","minItems":1,"items":{"oneOf":[]}}}}}}}
def fixed(items):return {"type":"array","items":items,"additionalItems":False,"minItems":len(items),"maxItems":len(items)}
name={"type":"string","minLength":1}
value={"type":"integer","minimum":0,"maximum":9007199254740991}
schema["properties"]["cases"]["items"]["properties"]["steps"]["items"]["oneOf"]=[fixed([{"enum":["reserve","grow","denyGrow","release"]},name,value,value,value]),fixed([{"const":"split"},name,value,name,value,value]),fixed([{"const":"merge"},name,name,value,value])]
for name,value in [("workspace-cases.json",{"version":1,"cases":rows}),("workspace.schema.json",schema)]:
 path=tests/name
 assert not path.exists(),str(path)
 path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
(ticket/"boolean-workspace-contract.md").write_text("# Logical Boolean Workspace Ownership\n\nSchema-first six neutral programs specify integer live capacity, peak, retained siblings/output overlap, split/merge ownership without duplicated grants, exact safe-integer frontiers, atomic one-over refusal and zero-value builders. An actual SQLite table/constraint oracle will independently reproduce every integer state.\n\nThe adjacent owned workspace is logical capacity permission only. A reservation is not a completed mesh certificate, source authority, native wallet or physical allocation receipt. Runtime use must reserve before actual backing birth/growth, keep input copies and retained siblings live until last use, and transfer ownership without double charging. Coordinate triples, index triples and retained normal triples belong to this ceiling; BSP metadata remains governed by original work/polygon/depth limits. No catch-and-continue fallback is allowed.\n\nPrivate identity leases reject forged, stale, duplicate and cross-workspace use. Growth refuses by subtraction before overflow or capacity excess. First failure latches, while valid entered release/close can still retire state. Close keeps peak and prevents future grants. There are no caller-supplied spent counters or refund methods. Actual kernel and IFC leaf integration are separate required work and not proven by ledger-only unit tests.\n",encoding="utf8")
print("[DEBUG] six neutral workspace programs and closed schema prepared")
