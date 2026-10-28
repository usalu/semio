from pathlib import Path
import json
ticket=Path(__file__).parent
base=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests"
cases=[
 {"id":"tunnel-half","host":[[0,0,0],[4,4,4]],"sourceTurns":7,"operations":[{"operation":"difference","right":[[1,1,-1],[3,3,5]],"volume":48},{"operation":"difference","right":[[2,0,0],[6,4,4]],"volume":24}]},
 {"id":"disconnected-retained","host":[[0,0,0],[4,4,4]],"sourceTurns":5,"operations":[{"operation":"union","right":[[5,0,0],[9,4,4]],"volume":128},{"operation":"intersection","right":[[-1,-1,-1],[10,5,5]],"volume":128}]},
 {"id":"identity-covering-empty","host":[[0,0,0],[4,4,4]],"sourceTurns":3,"operations":[{"operation":"intersection","right":[[0,0,0],[4,4,4]],"volume":64},{"operation":"difference","right":[[-1,-1,-1],[5,5,5]],"volume":0}]}
]
point={"type":"array","minItems":3,"maxItems":3,"items":{"type":"number"}}
bounds={"type":"array","minItems":2,"maxItems":2,"items":point}
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":3,"maxItems":3,"items":{"type":"object","additionalProperties":False,"required":["id","host","sourceTurns","operations"],"properties":{"id":{"type":"string","minLength":1},"host":bounds,"sourceTurns":{"type":"integer","minimum":1},"operations":{"type":"array","minItems":2,"maxItems":2,"items":{"type":"object","additionalProperties":False,"required":["operation","right","volume"],"properties":{"operation":{"enum":["difference","union","intersection"]},"right":bounds,"volume":{"type":"number","minimum":0}}}}}}}}}
for name,value in [("session-cases.json",{"version":1,"cases":cases}),("session.schema.json",schema)]:
 (base/name).write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")

