from author_12_lib import *
S=seed("laptop")
SB={s["id"]:s for s in S["body"]}
BASE=lambda t,y0,r=0.003:box("base","base","alu-anodised",0.6,(0.218,t,0.313),(0,y0+t/2,0),r=r)
LID=lambda t,r=0.0022:box("lid-shell","lid","alu-anodised",0.55,(0.218,t,0.313),(0.104,t/2,0),r=r)
SEC=lambda d,f:sec(d,f)
def deck(t,y0,id="th-deck"):
    return th(sec(BASE(t,y0),["-v","+u","-u","+w","-w"]),"deck",id=id)
def underside(t,y0):
    return th(sec(BASE(t,y0),["+v","+u","-u","+w","-w"]),"underside",id="th-underside")
PLUME=lambda t:th(sec(cyl("p","base","alu-anodised",0.6,0.1,0.0135,0.0135+t,c=(0.01,0),role="detail",n=24),["+v","-v"]),"deck",tags=["deck-plume"],id="th-deck-plume")
PAD=lambda t:st(box("s","base","rubber-foot",0.1,(0.19,t,0.28),(0,t/2,0),role="ground"),"base","support",id="support-feet",anchor="feet-contact")
def mass(id,bone,r,c): return st(ell("m",bone,"alu-anodised",0.6,r,c,role="detail"),bone,"mass",id=id)
B50=[json.loads(json.dumps(SB["foot-l"])),json.loads(json.dumps(SB["foot-r"])),BASE(0.0105,0.002),json.loads(json.dumps(SB["hinge-shoulder"])),json.loads(json.dumps(SB["keys"])),json.loads(json.dumps(SB["trackpad"])),LID(0.0075),json.loads(json.dumps(SB["display"])),json.loads(json.dumps(SB["hinge"]))]
setrole(B50,"ground","foot-l","foot-r"); setrole(B50,"texture","keys","trackpad")
TH50=[deck(0.0105,0.002),ref("th-keys","keys","deck","deck-shell"),ref("th-trackpad","trackpad","deck","deck-shell"),ref("th-hinge-shoulder","hinge-shoulder","deck","deck-shell"),underside(0.0105,0.002),ref("th-lid-shell","lid-shell","lid","lid-shell"),ref("th-display","display","lid","lid-shell"),ref("th-hinge","hinge","lid","lid-shell"),PLUME(0.008)]
ST50=[sref("member-base","base","base","member"),sref("member-lid","lid-shell","lid","member"),sref("joint-hinge","hinge","lid","joint"),PAD(0.002),
 mass("mass-base","base",(0.08,0.004,0.12),(0,0.0072,0)),mass("mass-lid","lid",(0.08,0.004,0.12),(0.106,0.00375,0))]
V50=vol(50,B50,TH50,ST50,
 ["Body: the seed is repaired to 1:50: the two stacked base boxes (4.6 and 7.1 mm, both thinner than 7.5 mm) are one 10.5 mm aluminium chassis slab, the lid is a 7.5 mm slab (drawn 6.5 mm), the hinge shoulder, the display glass, the hinge barrel, the four rubber feet as ground sheets and the key rows and the track pad as texture sheets.",
  "Thermal: the deck is the top face of the chassis (all other faces are section faces), the underside its bottom face, the lid its slab with display and hinge; the deck-plume emitter is an 8 mm disc of 0.2 m diameter over the keyboard whose faces are openings. The underside face lies within the contact tolerance of the desk plane and cannot count as exposed area (author request).",
  "Structure: chassis and lid slab are the two members, the hinge barrel the joint, the 190 x 280 mm foot pad the support on the desk, and one mass ellipsoid per half carries 0.82 and 0.48 kg."],
 ["Körper: der Seed ist auf 1:50 repariert: die zwei gestapelten Basis-Kästen (4,6 und 7,1 mm, beide dünner als 7,5 mm) sind eine 10,5 mm dicke Aluminiumchassisplatte, der Deckel ist eine 7,5-mm-Platte (gezeichnet 6,5 mm), dazu Scharnierschulter, Displayglas, Scharniertrommel, vier Gummifüße als Bodenblätter sowie Tastenreihen und Touchpad als Texturblätter.",
  "Thermik: das Deck ist die Oberseite des Chassis (alle anderen Flächen sind Schnittflächen), die Unterseite seine Unterfläche, der Deckel seine Platte mit Display und Scharnier; der Emitter Deckfahne ist eine 8-mm-Scheibe von 0,2 m Durchmesser über der Tastatur, deren Flächen Öffnungen sind. Die Unterseite liegt innerhalb der Kontakttoleranz der Tischebene und kann nicht als freie Fläche zählen (Autorenanfrage).",
  "Tragwerk: Chassis und Deckelplatte sind die zwei Stäbe, die Scharniertrommel das Gelenk, das 190 x 280 mm große Fußpolster das Auflager auf dem Tisch, und ein Masseellipsoid je Hälfte trägt 0,82 und 0,48 kg."])
B100=[BASE(0.0125,0.0,0.002),LID(0.0065,0.002)]
TH100=[deck(0.0125,0.0),underside(0.0125,0.0),ref("th-lid-shell","lid-shell","lid"),PLUME(0.008)]
ST100=[sref("member-base","base","base","member"),sref("member-lid","lid-shell","lid","member"),PAD(0.0125),
 mass("mass-base","base",(0.08,0.005,0.12),(0,0.00625,0)),mass("mass-lid","lid",(0.08,0.0034,0.12),(0.106,0.00325,0))]
V100=vol(100,B100,TH100,ST100,
 ["1:100 draws the laptop as two slabs: the 12.5 mm chassis (the two base boxes, feet, hinge shoulder, keys and track pad merged) and the 6.5 mm lid slab (display and hinge merged). Both are thinner than 15 mm exactly as their 2D drawings are, so the thinnest drawn extent sets the feature.",
  "Thermal: deck top, underside bottom, lid slab and the 8 mm plume disc, derived anew from this scale's two slabs.",
  "Structure: the two slabs are the members, a 190 x 280 mm foot pad and two masses; the hinge joint is merged into the lid."],
 ["1:100 zeichnet das Notebook als zwei Platten: das 12,5 mm dicke Chassis (beide Basis-Kästen, Füße, Scharnierschulter, Tasten und Touchpad zusammengefasst) und die 6,5 mm dicke Deckelplatte (Display und Scharnier aufgegangen). Beide sind dünner als 15 mm, genau wie ihre 2D-Zeichnungen; die dünnste gezeichnete Ausdehnung bestimmt die Strukturgröße.",
  "Thermik: Deckoberseite, Unterseite, Deckelplatte und die 8-mm-Fahnenscheibe, neu aus den zwei Platten dieses Maßstabs abgeleitet.",
  "Tragwerk: die beiden Platten sind die Stäbe, ein 190 x 280 mm großes Fußpolster und zwei Massen; das Scharniergelenk geht im Deckel auf."])
B200=[box("base","base","alu-anodised",0.6,(0.218,0.0125,0.313),(0,0.00625,0)),box("lid","lid","alu-anodised",0.55,(0.219,0.0095,0.319),(0.1065,0.00175,0))]
TH200=[deck(0.0125,0.0),underside(0.0125,0.0),th(sec(box("l","lid","alu-anodised",0.55,(0.219,0.0095,0.319),(0.1065,0.00175,0)),[]),"lid",id="th-lid"),PLUME(0.0126)]
TH200[2].pop("sectionFaces",None)
ST200=[sref("member-base","base","base","member"),sref("member-lid","lid","lid","member"),PAD(0.0125),
 mass("mass-base","base",(0.08,0.0065,0.12),(0,0.00625,0)),mass("mass-lid","lid",(0.08,0.005,0.12),(0.106,0.00175,0))]
V200=vol(200,B200,TH200,ST200,
 ["Minimal: 1:200 is two plates: chassis 218 x 12.5 x 313 mm and lid 219 x 9.5 x 319 mm (drawn with its 3 mm overhang), nothing else; the corner radii of 1:100 are dropped.",
  "Thermal: deck top, underside, lid slab and the plume disc.",
  "Structure: the two plates as members, the foot pad and two masses."],
 ["Minimal: 1:200 sind zwei Platten: Chassis 218 x 12,5 x 313 mm und Deckel 219 x 9,5 x 319 mm (mit gezeichneten 3 mm Überstand), sonst nichts; die Eckenradien von 1:100 entfallen.",
  "Thermik: Deckoberseite, Unterseite, Deckelplatte und die Fahnenscheibe.",
  "Tragwerk: die zwei Platten als Stäbe, das Fußpolster und zwei Massen."])
write("laptop",{"50":V50,"100":V100,"200":V200,"500":unavail(500)})
