from pathlib import Path
import json
ticket=Path(__file__).parent
base=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests"
rows=[{"operation":op,"mode":mode,"observer":observer,"maxWork":1,"expected":"boolean-work-limit","calls":2,"receiver":"undefined"} for op in ["difference","union","intersection","halfspace"] for mode in ["sync","steps","async"] for observer in ["tick","progress"]]
props={"operation":{"enum":["difference","union","intersection","halfspace"]},"mode":{"enum":["sync","steps","async"]},"observer":{"enum":["tick","progress"]},"maxWork":{"const":1},"expected":{"const":"boolean-work-limit"},"calls":{"const":2},"receiver":{"const":"undefined"}}
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":24,"maxItems":24,"items":{"type":"object","additionalProperties":False,"required":list(props),"properties":props}}}}
for name,data in [("control-receiver.json",{"version":1,"cases":rows}),("control-receiver.schema.json",schema)]:
 (base/name).write_text(json.dumps(data,ensure_ascii=False,indent=2)+"\n",encoding="utf8")

