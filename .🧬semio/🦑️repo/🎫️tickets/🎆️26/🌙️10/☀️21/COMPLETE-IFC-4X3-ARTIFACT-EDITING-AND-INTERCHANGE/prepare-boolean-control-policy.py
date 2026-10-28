from pathlib import Path
import json
ticket=Path(__file__).parent
base=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests"
invalid=[None,0,-1,1.5,"256",False,{},[],9007199254740992]
rows=[{"field":field,"mode":mode,"value":value,"expected":"boolean-invalid-yield-policy"if field=="yieldEvery"else"boolean-invalid-limits"} for field in ["maxWork","maxPolygons","maxDepth","yieldEvery"] for mode in (["async"]if field=="yieldEvery"else["sync","steps","async"]) for value in invalid]
props={"field":{"enum":["maxWork","maxPolygons","maxDepth","yieldEvery"]},"mode":{"enum":["sync","steps","async"]},"value":{},"expected":{"enum":["boolean-invalid-limits","boolean-invalid-yield-policy"]}}
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":90,"maxItems":90,"items":{"type":"object","additionalProperties":False,"required":list(props),"properties":props}}}}
for name,data in [("control-policy.json",{"version":1,"cases":rows}),("control-policy.schema.json",schema)]:
 (base/name).write_text(json.dumps(data,ensure_ascii=False,indent=2)+"\n",encoding="utf8")

