import json,copy
from body import *
from thermal import *
PENS_ALL={500:PENS(0.15,None,None,None,0.2),200:PENS(0.18,0.13,None,None,0.25),100:PENS(0.2,0.13,0.13,0.13,0.3),50:PENS(0.25,0.18,0.13,0.13,0.35)}
VALID={500:(350,1000),200:(150,350),100:(75,150),50:(20,75)}
def rep(den,shapes,decisions,hatches=None,plan=None,pens=None):
    lo,hi=VALID[den]
    g={"elevation":shapes}
    return {"status":"authored","denominator":den,"validRange":{"minDenominator":lo,"maxDenominator":hi},"pens":pens or PENS_ALL[den],"hatches":hatches or [],"geometry":g,"decisions":decisions}
def finish(N,path):
    doc={"$schema":"../../../🧬️schema/🔣️.json#/$defs/AssetDocument","version":"semio.staffage/v2","asset":N}
    open(path,"w").write(json.dumps(doc,ensure_ascii=False,indent=2)+"\n")
def human_env(air,mrt,rh,wind,sun,diffuse,sky=None,ground=None,winddir=180):
    e={"airTempC":air,"meanRadiantTempC":mrt}
    if sky is not None: e["skyTempC"]=sky
    if ground is not None: e["groundTempC"]=ground
    e.update({"relativeHumidity":rh,"windMS":wind,"windDirectionDeg":winddir,"solarWM2":sun,"diffuseFraction":diffuse,"pressurePa":101325})
    return e
def new_anchor(id,bone,x,y,en,de,kind="emitter"): return {"id":id,"bone":bone,"x":x,"y":y,"kind":kind,"title":T(en,de)}
def physiology(mass,h_cm,ref="gagge-1971"):
    return {"species":"human","massKg":mass,"skinAreaM2":round(dubois(mass,h_cm),3),"basalMetabolismW":round(mifflin(mass,h_cm),1),"metWM2":58.15,"coreSetPointC":36.8,"skinSetPointC":34.1,"sweating":True,"panting":False,"maxSkinBloodFlowLM2H":90,"minSkinBloodFlowLM2H":6.3,"source":"authored-estimate","reference":ref}
def respiration(patterns,temp,rh,ref="gupta-2010",tref="hoeppe-1981"):
    return {"noseEmitter":"nose-breath","mouthEmitter":"mouth-breath","exhaledTempC":temp,"exhaledRelativeHumidity":rh,"patterns":patterns,"motion":[{"bone":"torso","angleDeg":1.2,"dy":0.004},{"bone":"pelvis","angleDeg":0.5,"dy":0.003}],"source":"authored-estimate","reference":tref}
def breath_emitters(head_len,sc):
    return [
     {"id":"nose-breath","kind":"breath","title":T("Nostril jet","Nasenstrahl"),"anchor":"nostrils","directionDeg":-150,"diameterM":0.0095,"source":"authored-estimate","reference":"gupta-2010"},
     {"id":"mouth-breath","kind":"breath","title":T("Mouth jet","Mundstrahl"),"anchor":"mouth","directionDeg":-100,"diameterM":0.0124,"source":"authored-estimate","reference":"gupta-2010"}]
def plume_emitter(region,dia=0.35,title=("Rising warm air above the body","Aufsteigende Warmluft über dem Körper")):
    return {"id":"plume","kind":"plume","title":T(*title),"anchor":"crown","directionDeg":0,"diameterM":dia,"region":region,"source":"authored-estimate","reference":"incropera-2011"}
def anchors_head(A,head_len):
    """nostril and crown anchors on the head bone (head frame: x up, face towards -y)."""
    out=list(A)
    out.append(new_anchor("nostrils","head",round(0.53*head_len,4),round(-0.37*head_len,4),"Nostrils","Nasenlöcher"))
    out.append(new_anchor("crown","head",round(1.03*head_len,4),0.0,"Crown of the head (plume source)","Scheitel (Fahnenquelle)"))
    return out
