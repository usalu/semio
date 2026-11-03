from lib2 import *
A="thermostatic-radiator-valve"
def build(sc,asset):
    el={s["id"]:s for s in asset["representations"][str(sc)]["geometry"]["elevation"]}
    F=[];P=[]
    body=to_world(asset,el["body"])
    body["id"]="body"
    F.append(body)
    if sc==50:
        F.append(ext(asset,sc,"collar",-0.052,0.019,id="head")); P.append(ext(asset,sc,"collar",-0.052,0.019,id="head"))
        F.append(ext(asset,sc,"knob",-0.076,-0.052)); P.append(ext(asset,sc,"knob",-0.076,-0.052))
        F.append(line("collar-groove","head",[(-0.036,0.012),(-0.036,0.068)],z=3.15,tone=0.4))
        P.append(line("collar-groove","head",[(-0.03,-0.036),(0.03,-0.036)],z=3.15,tone=0.4))
        dec={"silhouette":("Front view: the valve body on the axis, the 0.06 m high head lying to the left of it with its collar groove, and the 0.04 m knob at the right-hand end. Top view: head and knob as two rounded blocks reaching 0.07 m to the right of the valve axis; the body hides under the head.",
                           "Vorderansicht: der Ventilkörper auf der Achse, der 0,06 m hohe Kopf links daneben mit seiner Rillenfuge und der 0,04 m große Knopf am rechten Ende. Draufsicht: Kopf und Knopf als zwei abgerundete Blöcke, die 0,07 m rechts der Ventilachse reichen; der Körper verschwindet unter dem Kopf."),
             "omitted":("Front and top view omit the index mark and the pointer, which sit on the end face of head and knob.",
                        "Vorder- und Draufsicht lassen Indexmarke und Zeiger weg, die auf der Stirnfläche von Kopf und Knopf sitzen.")}
    else:
        F.append(pl("head","head",[(0.019,0.01),(0.019,0.07),(-0.052,0.07),(-0.052,0.06),(-0.071,0.06),(-0.071,0.02),(-0.052,0.02),(-0.052,0.01)],z=3.1,tone=0.8,regions=["head","knob"],material="abs-plastic"))
        P.append(pl("head","head",[(-0.03,0.019),(0.03,0.019),(0.03,-0.052),(0.02,-0.052),(0.02,-0.071),(-0.02,-0.071),(-0.02,-0.052),(-0.03,-0.052)],z=3.1,tone=0.8,regions=["head","knob"],material="abs-plastic"))
        dec={"silhouette":("Front view: the valve body and the stepped head-and-knob block to its right. Top view: the same block, 0.06 m wide, stepping to 0.04 m at the knob.",
                           "Vorderansicht: der Ventilkörper und der abgestufte Kopf-Knopf-Block rechts daneben. Draufsicht: derselbe Block, 0,06 m breit, am Knopf auf 0,04 m abgesetzt."),
             "omitted":("Front and top view omit collar groove, index mark and pointer.",
                        "Vorder- und Draufsicht lassen Rillenfuge, Indexmarke und Zeiger weg.")}
    return F,P,dec
if __name__=="__main__": apply2(A,build)
