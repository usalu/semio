from pathlib import Path
import json
ticket=Path(__file__).parent
root=ticket.parents[6]
boolean=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean"
policy={"$schema":"http://json-schema.org/draft-07/schema#","title":"Cooperative Solid Execution Policy","description":"One captured Boolean evaluation shares original cumulative work, topology and depth grants. Each source-sized traversal turn is observable before atomic publication; async consumers yield to the host at most yieldEvery turns apart. Original observer and host-yield exceptions propagate by identity.","type":"object","additionalProperties":False,"properties":{"yieldEvery":{"type":"integer","minimum":1,"maximum":9007199254740991,"default":256}},"default":{"yieldEvery":256}}
(boolean/"execution.schema.json").write_text(json.dumps(policy,ensure_ascii=False,indent=2)+"\n",encoding="utf8")

