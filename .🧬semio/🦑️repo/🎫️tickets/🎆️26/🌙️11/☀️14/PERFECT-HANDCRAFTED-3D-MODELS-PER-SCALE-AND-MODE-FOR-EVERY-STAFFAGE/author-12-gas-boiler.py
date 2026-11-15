from author_12_lib import *
S=seed("gas-boiler")
B=[json.loads(json.dumps(s)) for s in S["body"]]
setrole(B,"texture","door","slits","logo-1","logo-2","led","chamber","display","tick-off","tick-heat","tick-water","pointer","set-notch","gauge-needle")
setrole(B,"glazing","sight-glass","gauge")
b=get(B,"bezel"); b["from"]=-0.163; B=[b if s["id"]=="bezel" else s for s in B]
sg=get(B,"sight-glass"); sg["profile"]=[[0,0.04],[0.002,0.04]]; B=[sg if s["id"]=="sight-glass" else s for s in B]
sk=get(B,"set-knob"); sk["origin"]=[0.12,0.14,-0.173]; sk["profile"]=[[0,0.018],[0.008,0.018]]; B=[sk if s["id"]=="set-knob" else s for s in B]
bt=get(B,"burner-tube"); bt["size"]=[0.052,0.011,0.0075]; bt["transform"]["translation"]=[0,0.4355,-0.1375]; B=[bt if s["id"]=="burner-tube" else s for s in B]
fl=lambda id,bone,h,w,t:loft(id,bone,"burner-mesh",0.95,[((0,0,-0.1405),w,t/2,2),((0,h*0.34,-0.1405),w*0.85,t/2*0.95),((0,h*0.7,-0.1405),w*0.5,t/2*0.8),((0,h,-0.1405),w*0.06,t/2*0.3)],role="detail")
B=[fl("flame-tall","flame-tall",0.032,0.024,0.009) if s["id"]=="flame-tall" else fl("flame","flame",0.017,0.018,0.0075) if s["id"]=="flame" else s for s in B]
for s in B:
    if s["id"] in("flame","flame-tall"): s["material"]="burner-mesh"
for s in B:
    if s["id"]=="flame-tall": s["transform"]={"translation":[0,0,0.001]}
for s in B:
    if s["id"]=="flame": s["transform"]={"translation":[0,0,-0.002]}
CAV=lambda: [[p[0],p[1]] for p in rrect(0.11,0.11,0.005,2,(0,0.46))]
def casing_parts(zf):
    return [box("casing","casing","painted-steel",0.88,(0.44,0.62,0.205),(0,0.36,0.0725),r=0.016),
            ext("casing-chamber","casing","painted-steel",0.88,rrect(0.44,0.62,0.016,3,(0,0.36)),"xy",zf,-0.03,holes=[CAV()])]
def caseskin(id,y0,y1,ry,zc,rx=0.2235,exp=10):
    ry=ry+0.0035
    return loft(id,"casing","painted-steel",0.88,[((0,y1,zc),rx,ry,exp),((0,y0,zc),rx,ry,exp)],caps=False)
GL=lambda id,y,r,**k:skinz(id,"casing","sight-glass",0.9,[(0.1581,r),(0.158,0.0005)],c=(0,y,0),role="glazing",**k)
TUBES50=th(tube("t","connections","brass",0.45,0.011,0.0095,0.0,0.052,c=(-0.16,0.07)),"connections","tube",id="th-stubs",repeat={"count":5,"translation":[0.08,0,0]})
TH50=[th(caseskin("c",0.05,0.67,0.165,0.01),"casing","sheet",id="th-casing"),ref("th-bezel","bezel","casing","sheet"),
 GL("th-sight-glass",0.46,0.0365,region="sight-glass",layer="glass"),
 th(box("b","casing","burner-mesh",0.95,(0.052,0.011,0.045),(0,0.4355,-0.119),r=0.0014),"burner","mesh",id="th-burner-tube"),ref("th-flame-tall","flame-tall","burner","mesh"),ref("th-flame","flame","burner","mesh"),
 ref("th-knob","knob","controls","panel"),ref("th-set-knob","set-knob","controls","panel"),ref("th-gauge","gauge","controls","panel"),
 th(box("d","casing","polycarbonate",0.3,(0.11,0.06,0.0075),(-0.115,0.155,-0.1688),r=0.001),"controls","panel",id="th-display"),
 ref("th-collar-base","collar-base","flue-collar","pipe"),ref("th-collar-pipe","collar-pipe","flue-collar","pipe"),
 TUBES50,ref("th-nuts","nuts","connections","tube"),
 th(cyl("flue-gas","collar","flue-pipe",0.5,0.03,0.0425,0.0575,c=(0,0.04),role="detail"),"flue-collar",tags=["flue-gas"],id="th-flue-gas")]
B=[x for s_ in B for x in ([*casing_parts(-0.13)] if s_["id"]=="casing" else [s_])]
for s_ in B:
    if s_["id"]=="chamber": s_["origin"]=[0,0.46,-0.031]
def joint(id,bone,mem,solid): return st(solid,mem,"joint",id=id)
ST50=[
 st(ext("m","casing","painted-steel",0.88,rrect(0.44,0.62,0.016,3,(0,0.36)),"xy",-0.155,0.175,holes=[rrect(0.424,0.604,0.012,3,(0,0.36))]),"casing","member",id="member-casing-shell"),
 st(box("m","casing","painted-steel",0.88,(0.4,0.6,0.008),(0,0.36,0.167)),"casing","member",id="member-casing-backplate"),
 st(tube("m","collar","flue-pipe",0.5,0.04,0.034,0.0,0.05,c=(0,0.04)),"collar","member",id="member-collar-flue"),
 st(tube("m","connections","brass",0.45,0.011,0.007,0.0,0.052,c=(-0.16,0.07)),"connections","member",id="member-connections-pipe",**{}),
 st(ell("j","mode-knob","brass",0.45,(0.009,0.009,0.009),(0,0,-0.158)),"mode-knob","joint",id="joint-mode-knob"),
 st(cyl("j","collar","flue-pipe",0.5,0.055,0.0,0.01,c=(0,0.04)),"collar","joint",id="joint-collar-flange"),
 st(box("j","connections","brass",0.45,(0.36,0.008,0.03),(0,0.046,0.07)),"connections","joint",id="joint-connections-manifold"),
 st(ell("j","flame","burner-mesh",0.9,(0.0075,0.0075,0.0075),(0,0.0075,-0.1405),role="detail"),"flame","joint",id="joint-flame"),
 st(ell("j","flame-tall","burner-mesh",0.9,(0.0075,0.0075,0.0075),(0,0.0075,-0.1405),role="detail"),"flame-tall","joint",id="joint-flame-tall"),
 st(box("s","casing","painted-steel",0.3,(0.2,0.02,0.175),(0,0.64,0.0875)),"casing","support",id="support-wall-bracket"),
 st(ell("m","casing","painted-steel",0.5,(0.15,0.18,0.1),(0,0.4,0.03),role="detail"),"casing","mass",id="mass-casing"),
]
ST50[3]["repeat"]={"count":5,"translation":[0.08,0,0]}

V50=vol(50,B,TH50,ST50,
 ["Body: the seed solids repaired to the 1:50 smallest feature of 7.5 mm. The casing is a rear box plus a middle frame with a real 110 x 110 mm combustion chamber behind the window (so the burner and flames sit in a cavity), and carries a 25 mm front plate with a window hole, the door, logo, vent slits, LED, ticks, pointer, display and gauge needle are declared texture or glazing sheets (they are below 7.5 mm thick), the bezel is thickened to 8 mm, the set knob to 8 mm, the sight glass is a 4.0 cm window, the burner bar is 7.5 mm deep and the two flames are lofted teardrops with a real 7.5 to 9 mm depth instead of flat extrusions.",
  "Thermal (exposed surface; areaM2 corrected at the root for the sight glass 0.005 to 0.0042, the controls 0.015, flue collar 0.032 and connections 0.025 measured against the declared 0.03, 0.05 and 0.04 are listed in area-review.md): the casing is an open sheet skin 3.5 mm outside the four sides (0.90 m2, no back against the wall) because the door, panel and bezel sit on the front; the sight glass is a single 4 cm face; the burner is a real 45 mm deep bar plus the two flames; the display is a 7.5 mm plate; the connection stubs are thin-walled tubes (outer 22 mm, bore 19 mm) so that outside and bore surface add up to the 0.04 m2 of the region; the flue-gas emitter is a 7.5 mm disc of the 60 mm flue diameter straddling the top of the collar.",
  "Structure: the casing is a hollow sheet-steel tube plus an 8 mm back plate (the front cover is cladding and absent), the flue collar and the five connection stubs are thin-walled tubes, joints are the knob shaft, the collar flange, the connection manifold and the two flame slides, the support is the 0.2 m hanger arm from the wall bracket into the casing, and one mass ellipsoid carries the 27.5 kg of the casing at its centre of mass."],
 ["Körper: die Seed-Körper sind auf die kleinste Strukturgröße von 7,5 mm des Maßstabs 1:50 repariert. Das Gehäuse ist ein hinterer Kasten plus Mittelrahmen mit einer echten 110 x 110 mm großen Brennkammer hinter dem Fenster (Brenner und Flammen sitzen so in einem Hohlraum) und trägt eine 25 mm starke Frontplatte mit Fensterloch, Tür, Logo, Lüftungsschlitze, LED, Skalenstriche, Zeiger, Display und Manometernadel sind als Textur- oder Glasblätter erklärt (sie sind dünner als 7,5 mm), die Blende ist auf 8 mm verdickt, der Sollwertknopf auf 8 mm, das Schauglas ist ein 8-cm-Fenster, der Brennerbalken ist 7,5 mm tief und die beiden Flammen sind gelofte Tropfen mit echter Tiefe von 7,5 bis 9 mm statt flacher Extrusionen.",
  "Thermik (freie Fläche; areaM2 an der Wurzel korrigiert beim Schauglas 0,005 auf 0,0042, die gemessenen Bedienfeld 0,015, Abgasstutzen 0,032 und Anschlüsse 0,025 gegen die erklärten 0,03, 0,05 und 0,04 stehen in area-review.md): das Gehäuse ist eine offene Blechhaut 3,5 mm außerhalb der vier Seiten (0,90 m2, ohne die Rückseite an der Wand), weil Tür, Bedienfeld und Blende auf der Front sitzen; das Schauglas ist eine einzelne Fläche von 8 cm Durchmesser; der Brenner ist ein echter 45 mm tiefer Balken samt den zwei Flammen; das Display ist eine 7,5-mm-Platte; die Anschlussstutzen sind dünnwandige Rohre (außen 22 mm, Bohrung 19 mm), damit Außen- und Bohrungsfläche die 0,04 m2 der Region ergeben; der Abgas-Emitter ist eine 7,5-mm-Scheibe mit dem 60-mm-Abgasdurchmesser über dem Ende des Stutzens.",
  "Tragwerk: das Gehäuse ist ein hohles Blechrohr samt 8 mm starker Rückplatte (die Frontabdeckung ist Verkleidung und fehlt), Abgasstutzen und die fünf Anschlussstutzen sind dünnwandige Rohre, Gelenke sind Knopfwelle, Stutzenflansch, Anschlussverteiler und die beiden Flammenschieber, das Auflager ist der 0,2 m lange Halterarm von der Wandhalterung ins Gehäuse und ein Masseellipsoid trägt die 27,5 kg des Gehäuses in seinem Schwerpunkt."])
# 1:100
WIN=lambda: rev("sight-glass","casing","sight-glass",0.9,[(0,0.04),(0.002,0.04)],axis=(0,0,1),origin=(0,0.46,-0.157),role="glazing")
FPL=ext("front-plate","casing","painted-steel",0.88,rrect(0.44,0.62,0.016,3,(0,0.36)),"xy",-0.155,-0.13,holes=[circle(0.04,24)])
FPL["holes"]=[[[p[0],p[1]+0.46] for p in circle(0.04,24)]]
B100=[*casing_parts(-0.13),FPL,
 box("panel","casing","painted-steel",0.7,(0.41,0.145,0.015),(0,0.1375,-0.1575),r=0.004),
 WIN(),
 ell("flame-tall","flame-tall","burner-mesh",0.95,(0.0264,0.0191,0.0075),(0,0.0199,-0.1405),role="detail"),
 ell("flame","flame","burner-mesh",0.95,(0.018,0.0085,0.0075),(0,0.0085,-0.1385),role="detail"),
 rev("knob","mode-knob","polycarbonate",0.3,[(0,0.028),(0.015,0.028)],axis=(0,0,1),origin=(0,0,-0.18)),
 rev("collar","collar","flue-pipe",0.5,[(0,0.0585),(0.004,0.0625),(0.03,0.0625),(0.03,0.04),(0.05,0.04)],origin=(0,0,0.04)),
 rev("stubs","connections","brass",0.45,[(0,0.013),(0.052,0.013)],origin=(-0.16,0,0.07),repeat={"count":5,"translation":[0.08,0,0]})]
for s in B100:
    if s["id"]=="stubs": s["repeat"]={"count":5,"translation":[0.08,0,0]}
GL=lambda id,y,r,**k:skinz(id,"casing","sight-glass",0.9,[(0.1581,r),(0.158,0.0005)],c=(0,y,0),role="glazing",**k)
TH100=[th(caseskin("c",0.05,0.67,0.165,0.01),"casing",id="th-casing"),
 GL("th-sight-glass",0.46,0.0365,region="sight-glass"),
 th(box("b","casing","burner-mesh",0.95,(0.052,0.015,0.03),(0,0.4355,-0.1215)),"burner",id="th-burner-bar"),ref("th-flame-tall","flame-tall","burner"),ref("th-flame","flame","burner"),
 ref("th-knob","knob","controls"),
 th(box("d","casing","polycarbonate",0.3,(0.11,0.06,0.015),(-0.115,0.155,-0.1575),r=0.002),"controls",id="th-display"),
 ref("th-collar","collar","flue-collar"),
 th(tube("t","connections","brass",0.45,0.013,0.0115,0.0,0.052,c=(-0.16,0.07)),"connections",id="th-stubs",repeat={"count":5,"translation":[0.08,0,0]}),
 th(cyl("flue-gas","collar","flue-pipe",0.5,0.03,0.035,0.05,c=(0,0.04),role="detail"),"flue-collar",tags=["flue-gas"],id="th-flue-gas")]
ST100=[
 st(ext("m","casing","painted-steel",0.88,rrect(0.44,0.62,0.016,3,(0,0.36)),"xy",-0.155,0.175,holes=[rrect(0.41,0.59,0.012,3,(0,0.36))]),"casing","member",id="member-casing-shell"),
 st(box("m","casing","painted-steel",0.88,(0.4,0.6,0.015),(0,0.36,0.1675)),"casing","member",id="member-casing-backplate"),
 st(tube("m","collar","flue-pipe",0.5,0.04,0.03,0.0,0.05,c=(0,0.04)),"collar","member",id="member-collar-flue"),
 st(tube("m","connections","brass",0.45,0.013,0.008,0.0,0.052,c=(-0.16,0.07)),"connections","member",id="member-connections-pipe"),
 st(cyl("j","collar","flue-pipe",0.5,0.055,0.0,0.015,c=(0,0.04)),"collar","joint",id="joint-collar-flange"),
 st(ell("j","mode-knob","brass",0.45,(0.012,0.012,0.012),(0,0,-0.16)),"mode-knob","joint",id="joint-mode-knob"),
 st(box("s","casing","painted-steel",0.3,(0.2,0.02,0.175),(0,0.64,0.0875)),"casing","support",id="support-wall-bracket"),
 st(ell("m","casing","painted-steel",0.5,(0.15,0.18,0.1),(0,0.4,0.03),role="detail"),"casing","mass",id="mass-casing")]
ST100[3]["repeat"]={"count":5,"translation":[0.08,0,0]}
V100=vol(100,B100,TH100,ST100,
 ["1:100 draws the boiler as nine solids: the casing box with a front plate that keeps the 80 mm sight window, the control panel as one 15 mm slab, the window glass, the two flames, the mode knob, 15 mm deep instead of the drawn 10 mm (set knob, gauge, display, door, vents, nuts and logo are merged into the panel and the front), one stepped revolve for the flue collar and five 26 mm stubs. The flames are 15 mm deep ellipsoids that sit in the window recess.",
  "Thermal: casing skin, window face, a 15 mm burner bar with the two flames, the display slab as the polycarbonate surface, the collar, thin-walled connection tubes and the 15 mm flue-gas disc; thermal solids are re-derived at this scale, not copied from 1:50.",
  "Structure: sheet tube with 15 mm walls and back plate, flue and connection tubes, two joints (collar flange and knob shaft, the flame slides and the manifold are merged into the members), the 0.2 m hanger arm and one mass."],
 ["1:100 zeichnet den Kessel aus neun Körpern: der Gehäusekasten mit einer Frontplatte, die das 80-mm-Schaufenster behält, das Bedienfeld als eine 15-mm-Platte, das Fensterglas, die zwei Flammen, der Betriebsartenschalter, 15 statt der gezeichneten 10 mm tief (Sollwertknopf, Manometer, Display, Tür, Schlitze, Muttern und Logo gehen in Bedienfeld und Front auf), ein gestufter Rotationskörper für den Abgasstutzen und fünf 26-mm-Stutzen. Die Flammen sind 15 mm tiefe Ellipsoide in der Fensternische.",
  "Thermik: Gehäusehaut, Fensterfläche, ein 15-mm-Brennerbalken mit den beiden Flammen, die Displayplatte als Polycarbonatfläche, der Stutzen, dünnwandige Anschlussrohre und die 15-mm-Abgasscheibe; die Thermikkörper sind für diesen Maßstab neu abgeleitet und nicht aus 1:50 kopiert.",
  "Tragwerk: Blechrohr mit 15 mm Wand und Rückplatte, Abgas- und Anschlussrohre, zwei Gelenke (Stutzenflansch und Knopfwelle; Flammenschieber und Verteiler gehen in die Stäbe auf), der 0,2 m lange Halterarm und eine Masse."])
# 1:200
B200=[*casing_parts(-0.15),
 box("panel","casing","painted-steel",0.7,(0.4,0.145,0.031),(0,0.1375,-0.1655),r=0.004),
 cyl("collar","collar","flue-pipe",0.5,0.0625,0.0,0.05,c=(0,0.04)),
 box("stubs","connections","brass",0.45,(0.34,0.05,0.04),(0,0.025,0.07))]
TH200=[th(caseskin("c",0.05,0.67,0.1625,0.0125),"casing",id="th-casing"),
 GL("th-sight-glass",0.46,0.0365,region="sight-glass"),
 th(box("b","casing","burner-mesh",0.95,(0.06,0.03,0.045),(0,0.44,-0.1125)),"burner",id="th-burner-bar"),
 th(box("d","casing","polycarbonate",0.3,(0.125,0.07,0.031),(-0.115,0.155,-0.1805),r=0.002),"controls",id="th-display"),
 ref("th-collar","collar","flue-collar"),
 th(loft("t","connections","brass",0.45,[((0,0.035,0.07),0.1725,0.0225,10),((0,0.0,0.07),0.1725,0.0225,10)],caps=False),"connections",id="th-stubs"),
 th(cyl("flue-gas","collar","flue-pipe",0.5,0.03,0.03,0.06,c=(0,0.04),role="detail"),"flue-collar",tags=["flue-gas"],id="th-flue-gas")]
ST200=[
 st(ext("m","casing","painted-steel",0.88,rrect(0.44,0.62,0.016,3,(0,0.36)),"xy",-0.15,0.175,holes=[rrect(0.38,0.56,0.012,3,(0,0.36))]),"casing","member",id="member-casing-shell"),
 st(box("m","casing","painted-steel",0.88,(0.4,0.6,0.03),(0,0.36,0.16)),"casing","member",id="member-casing-backplate"),
 st(tube("m","collar","flue-pipe",0.5,0.0625,0.035,0.0,0.05,c=(0,0.04)),"collar","member",id="member-collar-flue"),
 sref("member-connections-manifold","stubs","connections","member"),
 st(box("s","casing","painted-steel",0.3,(0.2,0.03,0.175),(0,0.64,0.0875)),"casing","support",id="support-wall-bracket"),
 st(ell("m","casing","painted-steel",0.5,(0.15,0.18,0.1),(0,0.4,0.03),role="detail"),"casing","mass",id="mass-casing")]
V200=vol(200,B200,TH200,ST200,
 ["1:200 models the boiler as four masses, as its own shapes do: the casing box 325 mm deep, the control slab drawn 15 mm deep but modelled 31 mm to keep the 30 mm smallest feature, the flue collar as one 125 mm cylinder and the five stubs merged into one manifold block 340 x 50 x 40 mm. Knob, flames, window, door and nuts are below the feature and disappear into the casing.",
  "Thermal: casing skin, window face, a 60 x 30 x 45 mm burner block inside the casing, the control block as the polycarbonate surface, the collar, the manifold block and the flue-gas disc; every region keeps a solid but is a single abstraction of its surface.",
  "Structure: 30 mm sheet tube and back plate, the flue tube, the manifold block as the connection member, the hanger arm and one mass; all joints are merged into the members."],
 ["1:200 modelliert den Kessel wie seine eigenen Formen aus vier Massen: der Gehäusekasten 325 mm tief, die Bedienplatte mit gezeichneten 15 mm Tiefe, aber 31 mm modelliert wegen der kleinsten Strukturgröße von 30 mm, der Abgasstutzen als ein 125-mm-Zylinder und die fünf Stutzen zu einem Verteilerblock 340 x 50 x 40 mm zusammengefasst. Knopf, Flammen, Fenster, Tür und Muttern liegen unter der Strukturgröße und gehen im Gehäuse auf.",
  "Thermik: Gehäusehaut, Fensterfläche, ein Brennerblock 60 x 30 x 45 mm im Gehäuse, der Bedienblock als Polycarbonatfläche, der Stutzen, der Verteilerblock und die Abgasscheibe; jede Region behält einen Körper, ist aber eine einzelne Abstraktion ihrer Fläche.",
  "Tragwerk: 30-mm-Blechrohr und Rückplatte, das Abgasrohr, der Verteilerblock als Anschlussstab, der Halterarm und eine Masse; alle Gelenke gehen in die Stäbe auf."])
write("gas-boiler",{"50":V50,"100":V100,"200":V200,"500":unavail(500)})
