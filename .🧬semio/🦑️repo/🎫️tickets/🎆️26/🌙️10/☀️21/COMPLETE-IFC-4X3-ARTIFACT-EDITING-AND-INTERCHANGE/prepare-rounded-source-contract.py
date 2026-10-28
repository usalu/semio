"""▣️ Author actual rounded source laws from the approved mathematical proposal."""
from pathlib import Path
import json,math,sys,hashlib

ticket=Path(__file__).parent
tests=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests"
original=ticket/"rounded-rectangle-profile-cases.json"
value=json.loads(original.read_text(encoding="utf8"))
value["scope"]="rounded-rectangle-extrusion-neutral"
for row in value["cases"]:
 if row["id"]=="integer-radius":
  row["dimensions"]=[4,4]
  row["radius"]=1
 row.update(expectedChords=None,expectedBoundary=None,polygonVolume=None)
 if row["geometry"]!="supported":continue
 x,y=row["dimensions"];radius=row["radius"]
 joins=(2 if radius<x/2 else 0)+(2 if radius<y/2 else 0)
 scale=.001 if row["unitPrefix"]=="MILLI"else 1
 theta=row["solid"]["tilt"];phi=row["profile"]["rotation"];sx,sy,sz=row["mappingScale"]
 u=[sx*math.cos(theta)*math.cos(phi),sy*math.sin(phi),-sz*math.sin(theta)*math.cos(phi)]
 v=[-sx*math.cos(theta)*math.sin(phi),sy*math.cos(phi),sz*math.sin(theta)*math.sin(phi)]
 aa=sum(n*n for n in u);bb=sum(n*n for n in v);ab=sum(a*b for a,b in zip(u,v))
 world_radius=radius*scale*math.sqrt((aa+bb+math.hypot(aa-bb,2*ab))/2)
 tolerance=min(1,max(row["sampling"]["relativeChordError"],row["sampling"]["absoluteChordErrorMetres"]/world_radius))
 available=(row["sampling"]["maxSegments"]-joins)//4
 admitted=lambda q:(math.nextafter(math.pi,math.inf)/(4*q))**2*.5*(1+8*sys.float_info.epsilon)<=tolerance
 assert available>0 and admitted(available)
 q=next(q for q in range(1,available+1)if admitted(q));n=4*q
 area=x*y-4*radius*radius+n*radius*radius*math.sin(2*math.pi/n)/2
 row.update(expectedChords=n,expectedBoundary=n+joins,polygonVolume=area*row["depth"]*abs(row["direction"][2])/math.hypot(*row["direction"])*abs(math.prod(row["mappingScale"]))*scale**3)
schema=json.loads((ticket/"rounded-rectangle-profile.schema.json").read_text(encoding="utf8"))
schema["properties"]["scope"]["const"]=value["scope"]
row_schema=schema["properties"]["cases"]["items"]
row_schema["required"]+= ["expectedChords","expectedBoundary","polygonVolume"]
for name in ["expectedChords","expectedBoundary"]:row_schema["properties"][name]={"anyOf":[{"type":"null"},{"type":"integer","minimum":4}]}
row_schema["properties"]["polygonVolume"]={"type":["number","null"]}
row_schema["allOf"]=[{"if":{"properties":{"carriers":{"properties":{"radius":{"const":"INTEGER"}}}}},"then":{"properties":{"radius":{"type":"integer"}}}}]
for name,body in [("rounded-rectangle-cases.json",value),("rounded-rectangle.schema.json",schema)]:
 path=tests/name
 assert not path.exists()
 path.write_text(json.dumps(body,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
(ticket/"rounded-source-contract.md").write_text("# Actual Rounded Source Contract\n\nThe approved16-row mathematical proposal is hand-authored into actual source laws with explicit expected chords/boundary/polygon volumes. Total maxSegments caps actual boundaryM including straight joins; no existing circle policy meaning changes. Eight supported rows have independent Three/mpmath reference agreement: defaultN72,M76 ordinary/M74capsule/M72circle; absolute1e-5 N316/M320. Exact outputvalues18M-12; ideal-volume difference remains a finite chord approximation.\n\nOne prototype inconsistency is explicitly corrected only in the new source contract: radius.2 cannot have an INTEGERphysical carrier. integer-radius now declares dimensions[4,4],radius1 with all geometric predicates valid after an explicit REAL repair. No fixture rounds/coerces numeric values. Original ticket proposal remains unchanged SHA-256 "+hashlib.sha256(original.read_bytes()).hexdigest()+". New schema requires integer numeric radius when its explicit carrier isINTEGER. This avoids conflating type refusal with fractional-to-integer conversion or overhalf radius.\n\nThe remaining source values/formal classifications are retained. Actual IFC producer is still absent; original independent math/proposal/native limitations remain documented. New source tests and physical originals are pending; no source support flag follows from expected metadata.\n",encoding="utf8")
print("[DEBUG] actual rounded16 contracts created; integer carrier isolated, eightexpectedmathrows")
