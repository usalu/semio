"""🔲️ Author domain-neutral indexed convex boundary cases before paired math implementations."""
from pathlib import Path
import math,json

ticket=Path(__file__).parent
base=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🔲️convex/🧪️tests"
base.mkdir(parents=True,exist_ok=True)
cases=[]
def add(id,kind,radius,chords,width=None,height=None,valid=True):
 source={"kind":kind,"radius":radius,"chords":chords}
 if width is not None:source.update(width=width,height=height)
 if not valid:size=area=None
 elif kind=="circle":size=chords;area=chords*radius*radius*math.sin(2*math.pi/chords)/2
 else:size=chords+(2 if radius<width/2 else 0)+(2 if radius<height/2 else 0);area=width*height-4*radius*radius+chords*radius*radius*math.sin(2*math.pi/chords)/2
 cases.append({"id":id,"boundary":source,"expectedSize":size,"expectedArea":area})
add("circle-cardinals","circle",1,4)
add("circle-default","circle",1,72)
add("rounded-default","rounded-rectangle",.2,72,1.8,.6)
add("capsule-horizontal","rounded-rectangle",.5,72,2,1)
add("capsule-vertical","rounded-rectangle",.5,72,1,2)
add("rounded-circle-limit","rounded-rectangle",1,72,2,2)
add("rounded-coarse","rounded-rectangle",.2,4,1.8,.6)
add("rounded-absolute-reference","rounded-rectangle",.2,316,1.8,.6)
add("radius-zero","rounded-rectangle",0,72,2,2,False)
add("radius-over-half","rounded-rectangle",2,72,2,2,False)
add("nonquadrant-chords","circle",1,5,valid=False)
add("negative-width","rounded-rectangle",1,72,-2,2,False)
number={"type":"number"};integer={"type":"integer"}
def boundary(kind,extra):
 props={"kind":{"const":kind},"radius":number,"chords":integer,**extra}
 return {"type":"object","additionalProperties":False,"required":list(props),"properties":props}
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":12,"maxItems":12,"items":{"type":"object","additionalProperties":False,"required":["id","boundary","expectedSize","expectedArea"],"properties":{"id":{"type":"string","minLength":1},"boundary":{"oneOf":[boundary("circle",{}),boundary("rounded-rectangle",{"width":number,"height":number})]},"expectedSize":{"anyOf":[{"type":"null"},{"type":"integer","minimum":4}]},"expectedArea":{"anyOf":[{"type":"null"},{"type":"number","minimum":0}]}}}}}}
for name,body in [("cases.json",{"version":1,"cases":cases}),("cases.schema.json",schema)]:
 path=base/name
 assert not path.exists()
 path.write_text(json.dumps(body,indent=2)+"\n",encoding="utf8")
print("[DEBUG] convex neutral12 cases/schema authored")
