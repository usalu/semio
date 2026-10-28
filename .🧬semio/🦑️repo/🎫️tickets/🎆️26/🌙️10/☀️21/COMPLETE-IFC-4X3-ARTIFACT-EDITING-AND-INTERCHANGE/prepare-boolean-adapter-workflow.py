"""🌳️ Author explicit frame and official-source contracts without selecting geometry by fixture ID."""
from pathlib import Path
import json

ticket=Path(__file__).parent
tests=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests"
reflected={"origin":[0,0,0],"originRotation":0,"target":[10,20,30],"axes":[[-1,0,0],[0,1,0],[0,0,1]],"scale":2,"secondary":[3,.5]}
outer={"origin":[1,-2,3],"originRotation":1.5707963267948966,"target":[-1,4,-2],"axes":[[0,1,0],[1,0,0],[0,0,1]],"scale":.5,"secondary":[None,None]}
positive={"origin":[0,0,0],"originRotation":0,"target":[0,0,0],"axes":[None,None,None],"scale":2,"secondary":[None,None]}
rows=[]
for name,operation,root,prefix,layers,volume in[("reflected-difference","DIFFERENCE","direct",None,[reflected],96),("reflected-union","UNION","csg",None,[reflected],420),("reflected-intersection","INTERSECTION","direct",None,[reflected],96),("millimetre-reflection","DIFFERENCE","csg","MILLI",[reflected],96e-9),("nested-double-reflection","DIFFERENCE","csg",None,[reflected,outer],12),("positive-uniform","DIFFERENCE","direct",None,[positive],256)]:rows.append({"id":name,"operation":operation,"root":root,"cut":[[2,-1,-1],[5,5,5]],"unitPrefix":prefix,"mapping":layers,"expected":"supported","volume":volume})
value={"scope":"ifc-owned-boolean-workflow","mapped":rows,"official":{"file":"bath-csg-solid.ifc","sha256":"4592f3029faa12625494ada734fcfe46e4fc91a33d001198e9e3144721870b95","product":225,"root":208,"profile":202,"radiusIndex":5,"radiusMillimetres":200,"editedRadiusMillimetres":100,"chords":72,"volume":.5481470113103606,"bounds":[[0,0,0],[2,.8,.8]]}}
vector={"type":"array","minItems":3,"maxItems":3,"items":{"type":"number"}}
layer={"type":"object","additionalProperties":False,"required":["origin","originRotation","target","axes","scale","secondary"],"properties":{"origin":vector,"originRotation":{"type":"number"},"target":vector,"axes":{"type":"array","minItems":3,"maxItems":3,"items":{"anyOf":[vector,{"type":"null"}]}},"scale":{"type":["number","null"]},"secondary":{"type":"array","minItems":2,"maxItems":2,"items":{"type":["number","null"]}}}}
row={"type":"object","additionalProperties":False,"required":list(rows[0]),"properties":{"id":{"type":"string"},"operation":{"enum":["DIFFERENCE","UNION","INTERSECTION"]},"root":{"enum":["direct","csg"]},"cut":{"type":"array","minItems":2,"maxItems":2,"items":vector},"unitPrefix":{"enum":[None,"MILLI"]},"mapping":{"type":"array","minItems":1,"maxItems":2,"items":layer},"expected":{"const":"supported"},"volume":{"type":"number","exclusiveMinimum":0}}}
schema={"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","additionalProperties":False,"required":list(value),"properties":{"scope":{"const":value["scope"]},"mapped":{"type":"array","minItems":6,"maxItems":6,"items":row},"official":{"const":value["official"]}}}
for name,data in[("boolean-workflow-cases.json",value),("boolean-workflow.schema.json",schema)]: (tests/name).write_text(json.dumps(data,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] six mapped original-source programs and pinned official contract authored")
