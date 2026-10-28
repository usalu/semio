"""🧪️ Author actual public producer limits before wiring the permission owner."""
from pathlib import Path
import json

ticket=Path(__file__).parent
tests=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests"
box=[[0,0,0],[4,4,4]]
rows=[{"id":"empty","kind":"difference","left":None,"right":None,"volume":0},{"id":"copy","kind":"difference","left":box,"right":None,"volume":64},{"id":"disjoint-empty","kind":"intersection","left":box,"right":[[6,0,0],[10,4,4]],"volume":0},{"id":"overlapping-cut","kind":"difference","left":box,"right":[[2,0,0],[6,4,4]],"volume":32},{"id":"disconnected-union","kind":"union","left":box,"right":[[6,0,0],[10,4,4]],"volume":128},{"id":"plane-cut","kind":"halfspace","left":box,"right":None,"volume":32}]
point={"type":"array","minItems":3,"maxItems":3,"items":{"type":"number"}}
bound={"anyOf":[{"type":"null"},{"type":"array","minItems":2,"maxItems":2,"items":point}]}
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":6,"maxItems":6,"items":{"type":"object","additionalProperties":False,"required":["id","kind","left","right","volume"],"properties":{"id":{"type":"string","minLength":1},"kind":{"enum":["difference","intersection","union","halfspace"]},"left":bound,"right":bound,"volume":{"type":"number","minimum":0}}}}}}
for name,value in [("workspace-kernel-cases.json",{"version":1,"cases":rows}),("workspace-kernel.schema.json",schema)]:
 path=tests/name
 assert not path.exists()
 path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] six public workspace producer programs prepared before implementation")
