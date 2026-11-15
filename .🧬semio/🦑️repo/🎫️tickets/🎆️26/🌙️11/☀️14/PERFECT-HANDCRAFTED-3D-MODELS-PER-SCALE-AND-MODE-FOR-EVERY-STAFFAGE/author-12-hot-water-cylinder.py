from author_12_lib import *
S=seed("hot-water-cylinder")
B=[json.loads(json.dumps(s)) for s in S["body"]]
for s_ in B:
    if s_["id"] in("feet-left","feet-right"): s_["transform"]["translation"][2]=-0.12; s_["repeat"]["translation"]=[0,0,0.24]
setrole(B,"texture","dial","tick-cold","tick-mid","tick-hot","needle","label","zip")
DOME=[(0,0.3),(0.01736,0.29544),(0.0342,0.28191),(0.05,0.25981),(0.06428,0.22981),(0.0766,0.19284),(0.0866,0.15),(0.09397,0.10261),(0.09848,0.05209),(0.1,0)]
def zone_skin(id,bone,mat="foam-jacket",y0=0.0,y1=0.33,**k):
    return skin(id,bone,mat,0.9,[(y1,0.3),(y0,0.3)],**k)
def dome_skin(id,**k):
    return skin(id,"cap","cap-plastic",0.2,[(a,r) for (a,r) in DOME][::-1][::-1],**k)
PLUME=lambda t,sc:th(sec(cyl("p","cap","cap-plastic",0.2,0.2,0.1-t/2,0.1+t/2,c=(0,0),role="detail",n=16),["+v","-v"]),"top-cap",tags=["top-plume"],id="th-top-plume")
TH50=[ref("th-zone-4","zone-4","zone-4","jacket"),ref("th-zone-3","zone-3","zone-3","jacket"),ref("th-zone-2","zone-2","zone-2","jacket"),ref("th-zone-1","zone-1","zone-1","jacket"),
 ref("th-cap","cap","top-cap","shell"),ref("th-hot-out","hot-out","connections","tube"),ref("th-stubs-left","stubs-left","connections","tube"),ref("th-nuts-left","nuts-left","connections","tube"),
 ref("th-stubs-left-low","stubs-left-low","connections","tube"),ref("th-nuts-left-low","nuts-left-low","connections","tube"),ref("th-stubs-right","stubs-right","connections","tube"),ref("th-nuts-right","nuts-right","connections","tube"),
 PLUME(0.008,1)]
def pad(id,x,t,w=0.07): return st(box("s","base","powder-steel",0.3,(w,t,w),(x,t/2,0)),"base","support",id=id,anchor="foot-left" if x<0 else "foot-right")
def tank(id,bone,wall,h=0.33): return st(tube("m",bone,"powder-steel",0.6,0.24,0.24-wall,0.0,h,c=(0,0),n=48),bone,"member",id=id)
def mass(id,bone,r,c): return st(ell("m",bone,"powder-steel",0.6,r,c,role="detail"),bone,"mass",id=id)
def pipes50():
    out=[]
    for i,(x,y,sx) in enumerate([(-0.33,1.1977,1),(-0.33,0.7852,1),(-0.33,0.1893,1),(0.3,0.9227,-1),(0.3,0.5102,-1)]):
        pass
    return out
def stubmem(id,x0,y,r,rin,length,sign=1):
    return st(tube("p","connections","brass",0.45,r,rin,0.0,length,c=(y,0),axis="x"),"connections","member",id=id)
def shift(d,x):
    d["transform"]={"translation":[x,0,0]}; return d
STR50=[tank("member-zone-4","zone-4",0.008),tank("member-zone-3","zone-3",0.008),tank("member-zone-2","zone-2",0.008),tank("member-zone-1","zone-1",0.008),
 st(rev("m","cap","powder-steel",0.6,[(a,r*0.8) for (a,r) in DOME]),"cap","member",id="member-cap-dome"),
 shift(stubmem("member-pipe-left-high",0,1.1977,0.0108,0.0075,0.09),-0.33),shift(stubmem("member-pipe-left-mid",0,0.7852,0.0108,0.0075,0.09),-0.33),shift(stubmem("member-pipe-left-low",0,0.1893,0.0108,0.0075,0.09),-0.33),
 shift(stubmem("member-pipe-right-high",0,0.9227,0.0108,0.0075,0.09),0.24),shift(stubmem("member-pipe-right-low",0,0.5102,0.0108,0.0075,0.09),0.24),
 st(tube("p","connections","brass",0.45,0.015,0.011,1.43,1.478,c=(0.2039,0)),"connections","member",id="member-hot-out"),
 pad("support-foot-left",-0.215,0.008),pad("support-foot-right",0.215,0.008),
 mass("mass-zone-4","zone-4",(0.2,0.1,0.2),(0,0.165,0)),mass("mass-zone-3","zone-3",(0.2,0.1,0.2),(0,0.165,0)),mass("mass-zone-2","zone-2",(0.2,0.1,0.2),(0,0.165,0)),mass("mass-zone-1","zone-1",(0.2,0.1,0.2),(0,0.165,0)),
 st(ell("j","needle","brass",0.45,(0.008,0.008,0.008),(0,0,-0.29),role="detail"),"needle","joint",id="joint-needle")]
for i,(zb,y) in enumerate([("zone-4",0.0),("zone-3",0.0),("zone-2",0.0),("zone-1",0.0),("cap",0.0)]):
    STR50.append(st(tube("j",zb,"powder-steel",0.6,0.245,0.232,y,y+0.008,c=(0,0),n=48),zb,"joint",id="joint-"+zb+"-girth"))

V50=vol(50,B,TH50,STR50,
 ["Body: the seed cylinder kept and repaired. Four 330 mm foam-jacket zones with a 300 mm radius under a dome cap (ten-point arc 100 mm high), a base ring, two foot bars of 70 mm, five stubs with hexagonal nuts and the hot outlet. Dial, ticks, needle, wrap-around label and zip are below 7.5 mm and are declared texture sheets.",
  "Thermal: every zone references its jacket revolve (0.62 m2 of lateral surface, the buried ends do not count), the dome its 0.31 m2, the connections their stubs, nuts and hot outlet; the plume emitter is an 8 mm disc of 0.4 m diameter on the dome apex whose faces are openings.",
  "Structure: every zone is a hollow steel tank tube of 480 mm diameter and 8 mm wall inside the foam, the dome a 0.8 scaled steel head, five 24 mm brass pipes through the jacket and the hot-out pipe, girth-weld rings at the five zone joints, the needle pivot, two 70 mm contact pads at the feet and one mass ellipsoid per zone."],
 ["Körper: der Seed-Zylinder bleibt und ist repariert. Vier 330 mm hohe Schaumzonen mit 300 mm Radius unter einer Kuppelkappe (Zehn-Punkte-Bogen, 100 mm hoch), ein Sockelring, zwei 70-mm-Fußbalken, fünf Stutzen mit Sechskantmuttern und der Warmwasserabgang. Skala, Striche, Nadel, umlaufendes Etikett und Reißverschluss sind dünner als 7,5 mm und als Texturblätter erklärt.",
  "Thermik: jede Zone verweist auf ihren Mantel (0,62 m2 Mantelfläche, die verdeckten Enden zählen nicht), die Kuppel auf ihre 0,31 m2, die Anschlüsse auf Stutzen, Muttern und Warmwasserabgang; der Fahnen-Emitter ist eine 8-mm-Scheibe von 0,4 m Durchmesser auf dem Kuppelscheitel, deren Flächen Öffnungen sind.",
  "Tragwerk: jede Zone ist ein hohles Stahlbehälterrohr von 480 mm Durchmesser und 8 mm Wand im Schaum, die Kuppel ein auf 0,8 verkleinerter Stahlboden, fünf 24-mm-Messingrohre durch den Mantel und das Warmwasserrohr, Rundnahtringe an den fünf Zonenfugen, der Nadelzapfen, zwei 70-mm-Auflagerpolster an den Füßen und ein Masseellipsoid je Zone."])
# ---- 1:100
def zone(bone,n=48): return rev(bone,bone,"foam-jacket",0.9,[(0,0.3),(0.33,0.3)])
B100=[rev("feet","base","powder-steel",0.3,[(0,0.25),(0.06,0.25)]),
 zone("zone-4"),zone("zone-3"),zone("zone-2"),zone("zone-1"),
 rev("cap","cap","cap-plastic",0.2,[(a,r) for (a,r) in DOME[::3]]),
 rev("hot-out","connections","brass",0.45,[(0,0.015),(0.048,0.015)],origin=(0.2039,1.43,0)),
 rev("stubs-left","connections","brass",0.45,[(0,0.015),(0.03,0.015)],axis=(1,0,0),origin=(-0.33,1.1977,0)),
 rev("stubs-left-low","connections","brass",0.45,[(0,0.015),(0.03,0.015)],axis=(1,0,0),origin=(-0.33,0.1893,0)),
 rev("stubs-right","connections","brass",0.45,[(0,0.015),(0.03,0.015)],axis=(1,0,0),origin=(0.3,0.9227,0)),
 rev("dial","zone-2","abs",0.9,[(0,0.05),(0.003,0.05)],axis=(0,0,1),origin=(0,0.165,-0.3007),role="texture")]
for s_ in B100:
    if s_["id"]=="stubs-left": s_["repeat"]={"count":2,"translation":[0,-0.4125,0]}
    if s_["id"]=="stubs-right": s_["repeat"]={"count":2,"translation":[0,-0.4125,0]}
    if s_["id"]=="dial": s_["material"]="cap-plastic"
TH100=[ref("th-zone-4","zone-4","zone-4"),ref("th-zone-3","zone-3","zone-3"),ref("th-zone-2","zone-2","zone-2"),ref("th-zone-1","zone-1","zone-1"),ref("th-cap","cap","top-cap"),
 ref("th-hot-out","hot-out","connections"),ref("th-stubs-left","stubs-left","connections"),ref("th-stubs-left-low","stubs-left-low","connections"),ref("th-stubs-right","stubs-right","connections"),PLUME(0.0155,1)]
STR100=[tank("member-zone-4","zone-4",0.015),tank("member-zone-3","zone-3",0.015),tank("member-zone-2","zone-2",0.015),tank("member-zone-1","zone-1",0.015),
 st(rev("m","cap","powder-steel",0.6,[(a,r*0.8) for (a,r) in DOME[::3]]),"cap","member",id="member-cap-dome"),
 shift(stubmem("member-pipe-left-high",0,1.1977,0.0108,0.0075,0.09),-0.33),shift(stubmem("member-pipe-left-mid",0,0.7852,0.0108,0.0075,0.09),-0.33),shift(stubmem("member-pipe-left-low",0,0.1893,0.0108,0.0075,0.09),-0.33),
 shift(stubmem("member-pipe-right-high",0,0.9227,0.0108,0.0075,0.09),0.24),shift(stubmem("member-pipe-right-low",0,0.5102,0.0108,0.0075,0.09),0.24),
 st(tube("p","connections","brass",0.45,0.015,0.0075,1.43,1.478,c=(0.2039,0)),"connections","member",id="member-hot-out"),
 pad("support-foot-left",-0.215,0.0155),pad("support-foot-right",0.215,0.0155),
 mass("mass-zone-4","zone-4",(0.2,0.1,0.2),(0,0.165,0)),mass("mass-zone-3","zone-3",(0.2,0.1,0.2),(0,0.165,0)),mass("mass-zone-2","zone-2",(0.2,0.1,0.2),(0,0.165,0)),mass("mass-zone-1","zone-1",(0.2,0.1,0.2),(0,0.165,0))]
V100=vol(100,B100,TH100,STR100,
 ["1:100 draws the cylinder as eleven solids: one 500 mm drum for the three feet and base ring, four 330 mm zones, a dome of four profile points, the hot outlet, three stub groups without nuts and the dial as one sheet. Ticks, needle, label, zip and nuts vanish below 15 mm.",
  "Thermal: the zone cylinders, the dome, the stubs and the hot outlet reference body solids of this scale; the plume disc is 15 mm thick.",
  "Structure: four 15 mm tank tubes, the dome head, five 24 mm pipes, the hot-out pipe, two 70 mm pads and one mass per zone; the girth joints are merged into the tank tubes."],
 ["1:100 zeichnet den Speicher aus elf Körpern: eine 500-mm-Trommel für die drei Füße und den Sockelring, vier 330-mm-Zonen, eine Kuppel aus vier Profilpunkten, der Warmwasserabgang, drei Stutzengruppen ohne Muttern und die Skala als ein Blatt. Striche, Nadel, Etikett, Reißverschluss und Muttern verschwinden unter 15 mm.",
  "Thermik: die Zonenzylinder, die Kuppel, die Stutzen und der Warmwasserabgang verweisen auf Körperkörper dieses Maßstabs; die Fahnenscheibe ist 15 mm dick.",
  "Tragwerk: vier 15-mm-Behälterrohre, der Kuppelboden, fünf 24-mm-Rohre, das Warmwasserrohr, zwei 70-mm-Polster und eine Masse je Zone; die Rundnähte gehen in die Behälterrohre auf."])
# ---- 1:200
B200=[rev("feet","base","powder-steel",0.3,[(0,0.25),(0.06,0.25)]),
 rev("zone-4","zone-4","foam-jacket",0.9,[(0,0.3),(0.66,0.3)]),
 rev("zone-2","zone-2","foam-jacket",0.9,[(0,0.3),(0.66,0.3)]),
 rev("cap","cap","cap-plastic",0.2,[(a,r) for (a,r) in DOME[::4]]+[DOME[-1]]),
 rev("hot-out","connections","brass",0.45,[(0,0.025),(0.05,0.025)],origin=(0.2028,1.43,0)),
 rev("side-l","connections","brass",0.45,[(0,0.025),(0.04,0.025)],axis=(1,0,0),origin=(-0.33,1.1988,0)),
 rev("side-r","connections","brass",0.45,[(0,0.025),(0.04,0.025)],axis=(1,0,0),origin=(0.29,0.9238,0))]
TH200=[skin("th-zone-4","zone-4","foam-jacket",0.9,[(0.33,0.3),(0,0.3)],region="zone-4"),skin("th-zone-3","zone-4","foam-jacket",0.9,[(0.66,0.3),(0.33,0.3)],region="zone-3"),
 skin("th-zone-2","zone-2","foam-jacket",0.9,[(0.33,0.3),(0,0.3)],region="zone-2"),skin("th-zone-1","zone-2","foam-jacket",0.9,[(0.66,0.3),(0.33,0.3)],region="zone-1"),
 ref("th-cap","cap","top-cap"),ref("th-hot-out","hot-out","connections"),ref("th-side-l","side-l","connections"),ref("th-side-r","side-r","connections"),PLUME(0.03,1)]
STR200=[tank("member-zone-4","zone-4",0.03,0.33),st(tube("m","zone-3","powder-steel",0.6,0.24,0.21,0.0,0.33,c=(0,0),n=48),"zone-3","member",id="member-zone-3"),
 st(tube("m","zone-2","powder-steel",0.6,0.24,0.21,0.0,0.33,c=(0,0),n=48),"zone-2","member",id="member-zone-2"),st(tube("m","zone-1","powder-steel",0.6,0.24,0.21,0.0,0.33,c=(0,0),n=48),"zone-1","member",id="member-zone-1"),
 st(rev("m","cap","powder-steel",0.6,[(a,r*0.8) for (a,r) in DOME[::4]]+[(DOME[-1][0],DOME[-1][1]*0.8)]),"cap","member",id="member-cap-dome"),
 st(cyl("p","connections","brass",0.45,0.0155,-0.33,-0.24,c=(1.1988,0),axis="x"),"connections","member",id="member-pipe-left-high"),
 st(cyl("p","connections","brass",0.45,0.0155,0.24,0.33,c=(0.9238,0),axis="x"),"connections","member",id="member-pipe-right-high"),
 st(cyl("p","connections","brass",0.45,0.0155,1.43,1.48,c=(0.2028,0)),"connections","member",id="member-hot-out"),
 pad("support-foot-left",-0.215,0.031),pad("support-foot-right",0.215,0.031),
 mass("mass-lower","zone-4",(0.2,0.2,0.2),(0,0.33,0)),mass("mass-upper","zone-2",(0.2,0.2,0.2),(0,0.33,0))]
V200=vol(200,B200,TH200,STR200,
 ["1:200 merges the four zones into a lower and an upper 660 mm cylinder as its drawings do; the zone-3 and zone-1 bones keep no body solid. Feet and base ring are one 60 mm drum, the dome has three profile points, the connections are 50 mm pins (hot out above, one stub on each side). Dial, label and nuts are gone.",
  "Thermal: the four zone regions are four uncapped 330 mm skins of the two cylinders, the dome and the pins are body references, the plume disc 30 mm.",
  "Structure: four 30 mm tank tubes (the merged cylinders are still four members), the dome head, three 31 mm pipes, two 70 mm pads and two lumped masses."],
 ["1:200 fasst die vier Zonen zu einem unteren und einem oberen 660-mm-Zylinder zusammen, wie es die Zeichnungen tun; die Knochen Zone 3 und Zone 1 behalten keinen Körper. Füße und Sockelring sind eine 60-mm-Trommel, die Kuppel hat drei Profilpunkte, die Anschlüsse sind 50-mm-Zapfen (Warmwasser oben, je ein Stutzen seitlich). Skala, Etikett und Muttern entfallen.",
  "Thermik: die vier Zonenregionen sind vier offene 330-mm-Häute der beiden Zylinder, Kuppel und Zapfen sind Körperverweise, die Fahnenscheibe 30 mm.",
  "Tragwerk: vier 30-mm-Behälterrohre (die zusammengefassten Zylinder bleiben vier Stäbe), der Kuppelboden, drei 31-mm-Rohre, zwei 70-mm-Polster und zwei zusammengefasste Massen."])
# ---- 1:500
B500=[rev("feet","base","powder-steel",0.3,[(0,0.25),(0.12,0.25)]),
 rev("zone-4","zone-4","foam-jacket",0.9,[(0,0.3),(0.66,0.3)]),rev("zone-2","zone-2","foam-jacket",0.9,[(0,0.3),(0.66,0.3)]),
 rev("cap","cap","cap-plastic",0.2,[DOME[0],DOME[5],DOME[9]]),
 rev("hot-out","connections","brass",0.45,[(0,0.055),(0.11,0.055)],origin=(0.2028,1.37,0)),
 rev("side-l","connections","brass",0.45,[(0,0.055),(0.11,0.055)],axis=(1,0,0),origin=(-0.33,1.1988,0)),
 rev("side-r","connections","brass",0.45,[(0,0.055),(0.11,0.055)],axis=(1,0,0),origin=(0.22,0.9238,0))]
patch=lambda id,x,y,sgn: th(sec(cyl("c","connections","brass",0.45,0.055,x,x+0.002,c=(y,0),axis="x",role="texture",n=12),["+u"] if sgn>0 else ["-u"]),"connections",id=id)
TH500=[skin("th-zone-4","zone-4","foam-jacket",0.9,[(0.33,0.3),(0,0.3)],region="zone-4"),skin("th-zone-3","zone-4","foam-jacket",0.9,[(0.66,0.3),(0.33,0.3)],region="zone-3"),
 skin("th-zone-2","zone-2","foam-jacket",0.9,[(0.33,0.3),(0,0.3)],region="zone-2"),skin("th-zone-1","zone-2","foam-jacket",0.9,[(0.66,0.3),(0.33,0.3)],region="zone-1"),
 ref("th-cap","cap","top-cap"),
 th(sec(cyl("c","connections","brass",0.45,0.055,1.478,1.48,c=(0.2028,0),role="texture",n=12),["-v"]),"connections",id="th-hot-out-patch"),
 patch("th-side-l-patch",-0.332,1.1988,1),patch("th-side-r-patch",0.33,0.9238,-1),PLUME(0.0755,1)]
STR500=[tank("member-zone-4","zone-4",0.075,0.33),st(tube("m","zone-3","powder-steel",0.6,0.24,0.165,0.0,0.33,c=(0,0),n=48),"zone-3","member",id="member-zone-3"),
 st(tube("m","zone-2","powder-steel",0.6,0.24,0.165,0.0,0.33,c=(0,0),n=48),"zone-2","member",id="member-zone-2"),st(tube("m","zone-1","powder-steel",0.6,0.24,0.165,0.0,0.33,c=(0,0),n=48),"zone-1","member",id="member-zone-1"),
 st(rev("m","cap","powder-steel",0.6,[(a,r*0.8) for (a,r) in [DOME[0],DOME[5],DOME[9]]]),"cap","member",id="member-cap-dome"),
 st(cyl("p","connections","brass",0.45,0.0378,-0.33,-0.24,c=(1.1988,0),axis="x"),"connections","member",id="member-pipe-left-high"),
 st(cyl("p","connections","brass",0.45,0.0378,0.22,0.33,c=(0.9238,0),axis="x"),"connections","member",id="member-pipe-right-high"),
 st(cyl("p","connections","brass",0.45,0.0378,1.37,1.48,c=(0.2028,0)),"connections","member",id="member-hot-out"),
 pad("support-foot-left",-0.215,0.076,0.08),pad("support-foot-right",0.215,0.076,0.08),
 mass("mass-lower","zone-4",(0.2,0.2,0.2),(0,0.33,0)),mass("mass-upper","zone-2",(0.2,0.2,0.2),(0,0.33,0))]
V500=vol(500,B500,TH500,STR500,
 ["1:500 is seven solids: a 120 mm foot drum, the lower and the upper 660 mm cylinder, a three-segment dome and three 110 mm pins; zone-3 and zone-1 are merged into the cylinders. Everything is at least 75 mm.",
  "Thermal: the four zone regions are uncapped 330 mm skins, the dome a body reference, the connections one-sided end patches (the lateral faces of a 110 mm pin would be nine times the region area), the plume a 75 mm disc.",
  "Structure: four 75 mm tank walls, the dome, three 76 mm pipe bars, two 76 mm pads and two lumped masses."],
 ["1:500 besteht aus sieben Körpern: eine 120-mm-Fußtrommel, der untere und der obere 660-mm-Zylinder, eine Kuppel aus drei Segmenten und drei 110-mm-Zapfen; Zone 3 und Zone 1 gehen in den Zylindern auf. Alles ist mindestens 75 mm.",
  "Thermik: die vier Zonenregionen sind offene 330-mm-Häute, die Kuppel ein Körperverweis, die Anschlüsse einseitige Stirnflächen (die Mantelflächen eines 110-mm-Zapfens wären das Neunfache der Regionsfläche), die Fahne eine 75-mm-Scheibe.",
  "Tragwerk: vier 75-mm-Behälterwände, die Kuppel, drei 76-mm-Rohrstäbe, zwei 76-mm-Polster und zwei zusammengefasste Massen."])
write("hot-water-cylinder",{"50":V50,"100":V100,"200":V200,"500":V500})
