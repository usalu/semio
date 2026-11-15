from author_12_lib import *
S=seed("kitchen-extractor-hood")
SB={s["id"]:s for s in S["body"]}
def canopy_parts(poly,t,zw,end,zin=None,pref=""):
    inn=inset(poly,t)
    shell=ext("canopy","canopy","stainless-steel",0.85,poly,"xy",-(zw-end),zw-end,holes=[inn])
    walls=[ext("end-l","canopy","stainless-steel",0.85,poly,"xy",-zw,-(zw-end)),ext("end-r","canopy","stainless-steel",0.85,poly,"xy",zw-end,zw)]
    return [shell]+walls
POLY=[(0,0.02),(0.5,0.02),(0.5,0.06),(0.4,0.2),(0,0.2)]
HOLES=lambda r,n=16:[[ [z+p[0],u+p[1]] for p in circle(r,n)] for (z,u) in [(-0.2,0.1),(-0.2,0.37),(0.2,0.1),(0.2,0.37)]]
def filt(id,y0,y1,r):
    return ext(id,"canopy","filter-mesh",0.5,[(-0.43,0.02),(0.43,0.02),(0.43,0.48),(-0.43,0.48)],"zx",y0,y1,holes=HOLES(r))
def lamps(id,r,h):
    return cyl(id,"canopy","lamp-diffuser",0.95,r,0.0,h,c=(0.1,-0.2),n=24,role="detail",repeat={"count":2,"translation":[0.27,0,0]})
def chim(id,bone,w,y0,y1,t,cx):
    return rtube(id,bone,"stainless-steel",0.85,w,w,t,y0,y1,c=(cx,0),r=0.002)
def collar(id,y0,y1,rb):
    c=ext(id,"chimney-top","stainless-steel",0.85,[(-0.1,0.04),(0.1,0.04),(0.1,0.24),(-0.1,0.24)],"zx",y0,y1,holes=[[(p[0],p[1]+0.14) for p in circle(rb,24)]])
    return c
def fix(L):
    for s in L:
        if s["id"] in("lamps",): s["repeat"]={"count":2,"translation":[0.27,0,0]}
    return L
def lamp_pair(id,h=0.012,r=0.029):
    a=cyl(id+"-l","canopy","lamp-diffuser",0.95,r,0.0,h,c=(0.1,-0.2),n=24,role="detail"); a["repeat"]={"count":2,"translation":[0.27,0,0]}
    b=cyl(id+"-r","canopy","lamp-diffuser",0.95,r,0.0,h,c=(0.1,0.2),n=24,role="detail"); b["repeat"]={"count":2,"translation":[0.27,0,0]}
    return [a,b]
def skinrect(id,bone,y1,y0,half,cx,region,**k):
    half=half+0.0012
    return th(loft(id,bone,"stainless-steel",0.85,[((cx,y1,0),half,half,12),((cx,y0,0),half,half,12)],caps=False),region,id=id,**k)
def outer_prism(poly,zw,id="th-canopy"):
    return th(sec(ext(id,"canopy","stainless-steel",0.85,poly,"xy",-zw,zw),["-u"]),"canopy",id=id)
EMIT=lambda t:th(sec(cyl("e","chimney-top","stainless-steel",0.85,0.075,0.365-t/2,0.365+t/2,c=(0.14,0),role="detail",n=24),["+v","-v"]),"chimney",tags=["extract"],id="th-extract")
def members50():
    return None
B50=[*canopy_parts(POLY,0.01,0.45,0.01),filt("filter",0.0,0.02,0.03),*lamp_pair("lamp",0.012,0.029),
 chim("chimney-lower","chimney",0.3,0.0,0.35,0.01,0.15),chim("chimney-upper","chimney-top",0.28,0.0,0.35,0.01,0.14),collar("collar",0.35,0.365,0.075)]
TH50=[outer_prism(POLY,0.45),ref("th-filter","filter","filter","mesh"),ref("th-lamp-l","lamp-l","lamps","diffuser"),ref("th-lamp-r","lamp-r","lamps","diffuser"),
 skinrect("th-chimney-lower","chimney",0.35,0.0,0.15,0.15,"chimney"),skinrect("th-chimney-upper","chimney-top",0.35,0.0,0.14,0.14,"chimney"),ref("th-collar","collar","chimney","tube"),EMIT(0.008)]
def ring(id,bone,w,y0,t):
    return st(rtube("j",bone,"stainless-steel",0.85,w,w,0.006,y0,y0+t,c=(w/2,0),r=0.002),bone,"joint",id=id)
def sup(id,y,anchor,sz=0.02,wz=0.3):
    return st(box("s","canopy","stainless-steel",0.3,(sz,sz,wz),(sz/2,y,0)),"canopy","support",id=id,anchor=anchor)
def mass(id,bone,r,c): return st(ell("m",bone,"stainless-steel",0.6,r,c,role="detail"),bone,"mass",id=id)
ST50=[sref("member-canopy-shell","canopy","canopy","member"),sref("member-canopy-end-l","end-l","canopy","member"),sref("member-canopy-end-r","end-r","canopy","member"),
 sref("member-chimney-lower","chimney-lower","chimney","member"),sref("member-chimney-upper","chimney-upper","chimney-top","member"),
 ring("joint-canopy-chimney","chimney",0.3,0.0,0.008),ring("joint-chimney-top","chimney-top",0.28,0.0,0.008),
 sup("support-wall-hook",0.19,"wall-hook"),sup("support-wall-rest",0.03,"wall-rest"),
 mass("mass-canopy","canopy",(0.12,0.06,0.2),(0.22,0.1,0)),mass("mass-chimney","chimney",(0.08,0.08,0.08),(0.15,0.17,0)),mass("mass-chimney-top","chimney-top",(0.08,0.08,0.08),(0.14,0.17,0))]
V50=vol(50,B50,TH50,ST50,
 ["Body: the canopy is now a real 10 mm sheet shell: a hollow trapezoid prism (the seed was a solid wedge) closed by two end walls, with the 20 mm filter plate under it carrying four lamp openings, the four 58 mm lamp discs in them, two chimney tubes (300 and 280 mm square, 10 mm walls, hollow) and the 15 mm duct collar plate with the 150 mm opening.",
  "Thermal: the canopy is its outer prism (the back against the wall is excluded as the mounting plane), the filter and lamps reference body solids, the chimneys are uncapped skins on the outer faces so that the inside of the hollow tubes does not count, the collar a reference; the extract emitter is an 8 mm disc of the 150 mm collar diameter whose faces are openings (the anchor y of the outlet was 0.915 above the tube origin and is corrected to 0.365).",
  "Structure: shell and end walls are the canopy member, both chimney tubes members, flange rings at the two fixed joints, the wall hook (pin, 0.3 m long) and the wall rest (roller) as 20 mm bars at the wall, and one mass each for motor, lower and upper chimney."],
 ["Körper: die Haube ist jetzt eine echte 10-mm-Blechschale: ein hohles Trapezprisma (der Seed war ein massiver Keil), von zwei Stirnwänden geschlossen, darunter die 20 mm starke Filterplatte mit vier Lampenöffnungen, die vier 58-mm-Lampenscheiben darin, zwei Kaminrohre (300 und 280 mm quadratisch, 10 mm Wand, hohl) und die 15 mm starke Kragenplatte mit der 150-mm-Öffnung.",
  "Thermik: die Haube ist ihr Außenprisma (die Rückseite an der Wand entfällt als Montageebene), Filter und Lampen verweisen auf Körperkörper, die Kamine sind offene Häute auf den Außenflächen, damit die Innenseite der hohlen Rohre nicht zählt, der Kragen ein Verweis; der Abluft-Emitter ist eine 8-mm-Scheibe mit dem 150-mm-Kragendurchmesser, deren Flächen Öffnungen sind (das y des Auslass-Ankers lag mit 0,915 über dem Rohrursprung und ist auf 0,365 korrigiert).",
  "Tragwerk: Schale und Stirnwände sind der Hauben-Stab, beide Kaminrohre Stäbe, Flanschringe an den beiden festen Gelenken, der Wandhaken (Festlager, 0,3 m lang) und die Wandanlage (Rollenlager) als 20-mm-Stäbe an der Wand und je eine Masse für Motor, unteren und oberen Kamin."])

# ---------------- 1:100
POLY100=[(0,0.025),(0.5,0.025),(0.5,0.06),(0.4,0.2),(0,0.2)]
def filt100(r=0.033): 
    return ext("filter","canopy","filter-mesh",0.5,[(-0.43,0.02),(0.43,0.02),(0.43,0.48),(-0.43,0.48)],"zx",0.0,0.025,holes=HOLES(r,12))
B100=[*canopy_parts(POLY100,0.015,0.45,0.015),filt100(),*lamp_pair("lamp",0.016,0.03),
 chim("chimney-lower","chimney",0.3,0.0,0.35,0.015,0.15),chim("chimney-upper","chimney-top",0.27,0.0,0.35,0.015,0.135),
 ext("collar","chimney-top","stainless-steel",0.85,[(-0.105,0.03),(0.105,0.03),(0.105,0.24),(-0.105,0.24)],"zx",0.35,0.37,holes=[[(p[0],p[1]+0.135) for p in circle(0.075,16)]])]
for s_ in B100:
    if s_["id"] in("lamp-l","lamp-r"): s_["repeat"]={"count":2,"translation":[0.27,0,0]}
TH100=[outer_prism(POLY100,0.45),ref("th-filter","filter","filter"),ref("th-lamp-l","lamp-l","lamps"),ref("th-lamp-r","lamp-r","lamps"),
 skinrect("th-chimney-lower","chimney",0.35,0.0,0.15,0.15,"chimney"),skinrect("th-chimney-upper","chimney-top",0.35,0.0,0.135,0.135,"chimney"),ref("th-collar","collar","chimney"),
 th(sec(cyl("e","chimney-top","stainless-steel",0.85,0.075,0.3575,0.3725,c=(0.135,0),role="detail",n=16),["+v","-v"]),"chimney",tags=["extract"],id="th-extract")]
ST100=[sref("member-canopy-shell","canopy","canopy","member"),sref("member-canopy-end-l","end-l","canopy","member"),sref("member-canopy-end-r","end-r","canopy","member"),
 sref("member-chimney-lower","chimney-lower","chimney","member"),sref("member-chimney-upper","chimney-upper","chimney-top","member"),
 ring("joint-canopy-chimney","chimney",0.3,0.0,0.016),
 sup("support-wall-hook",0.19,"wall-hook",0.02),sup("support-wall-rest",0.03,"wall-rest",0.02),
 mass("mass-canopy","canopy",(0.12,0.06,0.2),(0.22,0.1,0)),mass("mass-chimney","chimney",(0.08,0.08,0.08),(0.15,0.17,0)),mass("mass-chimney-top","chimney-top",(0.08,0.08,0.08),(0.135,0.17,0))]
V100=vol(100,B100,TH100,ST100,
 ["1:100 keeps the hollow canopy (15 mm shell and end walls), the 25 mm perforated filter, four 60 mm lamp discs 16 mm deep, the two chimney tubes with 15 mm walls (the upper one 270 mm, as drawn) and the 20 mm collar plate. Smaller than 15 mm: plate lip, 20 mm flange of the lamp bezels, joint ring of the upper tube; filter holes are 12-gons.",
  "Thermal: the outer prism of the canopy, filter and lamps as references, chimney skins 1.2 mm outside the tube faces and the 15 mm extract disc; the exposed chimney surface leaves the back on the wall out.",
  "Structure: shell, end walls and both tubes are the members, one flange ring remains (the upper joint is merged into the tube), hook and rest as 20 mm bars, three masses."],
 ["1:100 behält die hohle Haube (15-mm-Schale und Stirnwände), den 25 mm starken gelochten Filter, vier 60-mm-Lampenscheiben von 16 mm Tiefe, die beiden Kaminrohre mit 15 mm Wand (das obere 270 mm, wie gezeichnet) und die 20 mm starke Kragenplatte. Kleiner als 15 mm: Plattenlippe, Lampenrand, Gelenkring des oberen Rohres; die Filterlöcher sind 12-Ecke.",
  "Thermik: das Außenprisma der Haube, Filter und Lampen als Verweise, Kaminhäute 1,2 mm außerhalb der Rohrflächen und die 15-mm-Abluftscheibe; die freie Kaminfläche lässt die Rückseite an der Wand weg.",
  "Tragwerk: Schale, Stirnwände und beide Rohre sind die Stäbe, ein Flanschring bleibt (das obere Gelenk geht im Rohr auf), Haken und Anlage als 20-mm-Stäbe, drei Massen."])
# ---------------- 1:200
POLY200=[(0,0),(0.5,0),(0.5,0.06),(0.4,0.2),(0,0.2)]
B200=[*canopy_parts(POLY200,0.03,0.45,0.03),
 chim("chimney-lower","chimney",0.3,0.0,0.35,0.03,0.15),chim("chimney-upper","chimney-top",0.3,0.0,0.37,0.03,0.15)]
TH200=[outer_prism(POLY200,0.45),
 th(ext("f","canopy","filter-mesh",0.5,[(-0.43,0.0),(0.43,0.0),(0.43,0.46),(-0.43,0.46)],"zx",-0.0012,0.03),"filter",id="th-filter"),
 th(cyl("l","canopy","lamp-diffuser",0.95,0.03,-0.0012,0.0305,c=(0.1,-0.2),n=12,role="detail"),"lamps",id="th-lamp-l"),
 skinrect("th-chimney-lower","chimney",0.35,0.0,0.15,0.15,"chimney"),skinrect("th-chimney-upper","chimney-top",0.37,0.0,0.15,0.15,"chimney"),
 th(sec(cyl("e","chimney-top","stainless-steel",0.85,0.075,0.355,0.385,c=(0.15,0),role="detail",n=12),["+v","-v"]),"chimney",tags=["extract"],id="th-extract")]
ST200=[sref("member-canopy-shell","canopy","canopy","member"),sref("member-canopy-end-l","end-l","canopy","member"),sref("member-canopy-end-r","end-r","canopy","member"),
 sref("member-chimney-lower","chimney-lower","chimney","member"),sref("member-chimney-upper","chimney-upper","chimney-top","member"),
 sup("support-wall-hook",0.19,"wall-hook",0.031),sup("support-wall-rest",0.04,"wall-rest",0.031),
 mass("mass-canopy","canopy",(0.12,0.06,0.2),(0.22,0.1,0)),mass("mass-chimney","chimney",(0.08,0.08,0.08),(0.15,0.17,0)),mass("mass-chimney-top","chimney-top",(0.08,0.08,0.08),(0.15,0.17,0))]
V200=vol(200,B200,TH200,ST200,
 ["1:200 models canopy and chimney as three hollow masses of 30 mm sheet: the canopy (filter plate and lamps merged into its floor), the 300 mm lower tube and the 300 x 370 mm upper tube (collar merged). No joint rings.",
  "Thermal: canopy prism, one filter plate and one lamp disc under the canopy floor, the two chimney skins and the 30 mm extract disc.",
  "Structure: the same five members as the bodies, 31 mm hook and rest, three masses."],
 ["1:200 modelliert Haube und Kamin als drei hohle Massen aus 30-mm-Blech: die Haube (Filterplatte und Lampen gehen in ihrem Boden auf), das untere 300-mm-Rohr und das obere 300 x 370 mm Rohr (Kragen aufgegangen). Keine Gelenkringe.",
  "Thermik: Haubenprisma, eine Filterplatte und eine Lampenscheibe unter dem Hauben-Boden, die beiden Kaminhäute und die 30-mm-Abluftscheibe.",
  "Tragwerk: dieselben fünf Stäbe wie die Körper, 31-mm-Haken und -Anlage, drei Massen."])
write("kitchen-extractor-hood",{"50":V50,"100":V100,"200":V200,"500":unavail(500)})
