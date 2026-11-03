import json
p="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-hob/🔣️.json"
s=open(p,encoding="utf-8",newline="").read()
def shape(i,role,z,tone,kind,geo,region=None,regions=None,material="steel-enamel",filled=True):
    o={"id":i,"bone":"hob","role":role,"z":z,"filled":filled,"tone":tone,"kind":kind,**geo}
    if region: o["region"]=region
    if regions: o["regions"]=regions
    o["material"]=material
    return o
cy=0.03
def ell(r): return {"cx":0,"cy":cy,"rx":r,"ry":r}
frame=[[-0.135,cy-0.067],[0.135,cy-0.067],[0.135,cy+0.067],[-0.135,cy+0.067]]
p50=[
 shape("plan-plate","body",1,0.3,"rect",{"x":-0.15,"y":-0.26,"w":0.3,"h":0.52,"r":0.01},region="plate"),
 shape("plan-burner-ring","body",2,0.5,"ellipse",ell(0.052),region="burner",material="brass"),
 shape("plan-flame-ring","body",3,0.92,"ellipse",ell(0.04),region="flame",material="flame"),
 shape("plan-burner-cap","body",4,0.55,"ellipse",ell(0.02),region="burner",material="brass"),
 shape("plan-trivet-back","body",5,0.45,"rect",{"x":-0.135,"y":cy+0.055,"w":0.27,"h":0.012},region="trivet",material="cast-iron"),
 shape("plan-trivet-front","body",5,0.45,"rect",{"x":-0.135,"y":cy-0.067,"w":0.27,"h":0.012},region="trivet",material="cast-iron"),
 shape("plan-trivet-left","body",5,0.45,"rect",{"x":-0.135,"y":cy-0.067,"w":0.012,"h":0.134},region="trivet",material="cast-iron"),
 shape("plan-trivet-right","body",5,0.45,"rect",{"x":0.123,"y":cy-0.067,"w":0.012,"h":0.134},region="trivet",material="cast-iron"),
 shape("plan-knob","body",6,0.6,"ellipse",{"cx":0.105,"cy":-0.2,"rx":0.011,"ry":0.011},region="knob",material="bakelite"),
 shape("plan-knob-pointer","detail",7,0.1,"poly",{"points":[[0.105,-0.2],[0.105,-0.189]],"closed":False},region="knob",material="bakelite",filled=False),
]
p100=[
 shape("plan-plate","body",1,0.3,"rect",{"x":-0.15,"y":-0.26,"w":0.3,"h":0.52,"r":0.01},regions=["plate","knob"]),
 shape("plan-burner","body",2,0.5,"ellipse",ell(0.052),regions=["burner","flame"],material="brass"),
 shape("plan-trivet","body",3,0.45,"poly",{"points":frame,"closed":True},region="trivet",material="cast-iron",filled=False),
]
def block(shapes,ind):
    t=json.dumps(shapes,indent=2,ensure_ascii=False)
    lines=t.split("\n")
    return "\n".join(lines[0:1]+[ " "*ind+l for l in lines[1:]])
mark='\n          ]\n        },\n        "decisions"'
parts=s.split(mark); assert len(parts)==4,len(parts)
def ins(shapes): return '\n          ],\n          "plan": '+block(shapes,10)+'\n        },\n        "decisions"'
s=parts[0]+ins(p50)+parts[1]+ins(p100)+mark.join(parts[2:])
def sub(o,n):
    global s
    assert s.count(o)==1,o[:40]; s=s.replace(o,n)
sub('dem Brenner darauf, dem Flammenkranz darüber und dem Pfannenträger aus Gusseisen darüber hinweg."','dem Brenner darauf, dem Flammenkranz darüber und dem Pfannenträger aus Gusseisen darüber hinweg. Grundriss: die Platte von 0,30 m mal 0,52 m (15 mm mal 26 mm), der Brennerkopf mit Flammenring und Deckel, der rechteckige Pfannenträger-Rahmen und der Bedienknebel an der Vorderkante."'.replace('dem Brenner darauf, dem','dem Brenner darauf, dem')) if False else None
sub('and the cast-iron pan support spanning over it."','and the cast-iron pan support spanning over it. Plan: the 0.30 m by 0.52 m plate (15 mm by 26 mm), the burner head with its flame ring and cap, the rectangular pan-support frame and the control knob at the front edge."')
sub('dem Pfannenträger aus Gusseisen darüber hinweg."','dem Pfannenträger aus Gusseisen darüber hinweg. Grundriss: die Platte von 0,30 m mal 0,52 m (15 mm mal 26 mm), der Brennerkopf mit Flammenring und Deckel, der rechteckige Pfannenträger-Rahmen und der Bedienknebel an der Vorderkante."')
sub('a burner wedge with its flame crown and the pan support bridge above."','a burner wedge with its flame crown and the pan support bridge above. Plan: the 3 mm by 5.2 mm plate, one burner disc with the flame ring merged in and the pan-support frame as an outline."')
sub('und die Pfannenträger-Brücke darüber."','und die Pfannenträger-Brücke darüber. Grundriss: die Platte von 3 mm mal 5,2 mm, eine Brennerscheibe mit eingeschmolzenem Flammenring und der Pfannenträger-Rahmen als Umriss."')
sub('"viewpoints": [\n        "elevation"\n      ]','"viewpoints": [\n        "elevation",\n        "plan"\n      ]')
open(p,"w",encoding="utf-8",newline="").write(s); json.loads(s); print("ok")
