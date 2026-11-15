from author_12_lib import *
S=seed("mvhr-unit")
SB={s["id"]:s for s in S["body"]}
cp=lambda i:json.loads(json.dumps(SB[i]))
def sup(id,y,anchor,sx=0.03):
    return st(box("s","casing","painted-steel",0.3,(sx,0.03,0.15),(0,y,0.075)),"casing","support",id=id,anchor=anchor)
def mass(id,bone,r,c): return st(ell("m",bone,"painted-steel",0.6,r,c,role="detail"),bone,"mass",id=id)
def spig_cyl(id,r,h,t=None,solid=True):
    out=[]
    for k,(x0,z0) in enumerate([(-0.225,0.07),(-0.075,-0.07)]):
        d=cyl(id+"-"+"ab"[k],"spigots","galvanised-steel",0.7,r,0.0,h,c=(x0,z0),n=24); d["repeat"]={"count":2,"translation":[0.3,0,0]}
        out.append(d)
    return out
def front_prism(y0,y1,rc,hole,w=0.6,r=0.025):
    prof=[(p[0],p[1]) for p in rrect(w,w,r,3,(0,0.3))]
    return ext("c","casing","painted-steel",0.88,prof,"xy",-0.15,0.15,holes=[[(p[0],p[1]+0.3) for p in circle(hole,32)]])
def th_casing(hole,r=0.025):
    d=front_prism(0,0,0,hole,r=r); d["sectionFaces"]=["+w"]; return th(d,"casing",id="th-casing")
def th_window(r,t,z0):
    d=rev("w","casing","polycarbonate",0.8,[(z0,r),(z0-t,r)],axis=(0,0,1),origin=(0,0.3,0),role="glazing"); d["sectionFaces"]=["+w"]; return th(d,"window",id="th-window")
def emit(id,x,z,t,ytop):
    d=cyl("e","spigots","galvanised-steel",0.7,0.049,ytop-t,ytop,c=(x,z),role="detail",n=16); d["sectionFaces"]=["+v","-v"]
    return th(d,"ducts",tags=[id],id="th-"+id)
B50=[cp("casing-wall"),cp("casing"),cp("window-ring"),cp("window"),cp("hub"),cp("blade"),cp("led"),cp("handle"),cp("spigot-a"),cp("spigot-b"),cp("bead-a"),cp("bead-b")]
setrole(B50,"texture","window-ring","led")
h=get(B50,"handle"); h["lateralScale"]=0.5; h["r0"]=0.012; h["r1"]=0.012
B50=[h if s["id"]=="handle" else s for s in B50]
TH50=[th_casing(0.128),th_window(0.112,0.002,-0.1495),ref("th-hub","hub","impeller","blades"),ref("th-blade","blade","impeller","blades"),
 *[th(d,"ducts",id="th-"+d["id"]) for d in spig_cyl("duct",0.049,0.062)],ref("th-bead-a","bead-a","ducts","tube"),ref("th-bead-b","bead-b","ducts","tube"),
 emit("supply-air",0.075,0.07,0.008,0.0655),emit("extract-air",-0.225,0.07,0.008,0.0655)]
ST50=[st(ext("m","casing","painted-steel",0.88,[(q[0],q[1]) for q in rrect(0.6,0.6,0.025,3,(0,0.3))],"xy",-0.12,0.142,holes=[[(q[0],q[1]) for q in rrect(0.584,0.584,0.017,3,(0,0.3))]]),"casing","member",id="member-shell"),
 st(box("m","casing","painted-steel",0.88,(0.58,0.58,0.008),(0,0.3,0.146)),"casing","member",id="member-back"),sref("member-front","casing-wall","casing","member"),
 sref("member-spigot-a","spigot-a","spigots","member"),sref("member-spigot-b","spigot-b","spigots","member"),
 sref("member-hub","hub","impeller","member"),sref("member-blades","blade","impeller","member"),
 st(cyl("j","impeller","pp-impeller",0.4,0.01,-0.12,0.0,c=(0,0),axis="z",role="detail"),"impeller","joint",id="joint-impeller-shaft"),
 st(tube("j","spigots","galvanised-steel",0.7,0.052,0.045,0.0,0.008,c=(-0.225,0.07)),"spigots","joint",id="joint-spigots-a-flange",repeat={"count":2,"translation":[0.3,0,0]}),
 st(tube("j","spigots","galvanised-steel",0.7,0.052,0.045,0.0,0.008,c=(-0.075,-0.07)),"spigots","joint",id="joint-spigots-b-flange",repeat={"count":2,"translation":[0.3,0,0]}),
 sup("support-wall-hook",0.57,"wall-hook"),sup("support-wall-rest",0.03,"wall-rest"),mass("mass-casing","casing",(0.2,0.2,0.1),(0,0.3,0))]
V50=vol(50,B50,TH50,ST50,
 ["Body: the seed is kept: a 270 mm rounded sheet casing with a 30 mm front cover that has the 224 mm window opening, the 256 mm window ring, the glass, the nine-blade impeller with its hub (blades 18 mm deep, each a 17-point curved profile), the LED and the grip, and four hollow spigots with beads. The grip is now 12 mm deep (was a 6 mm ribbon) and the window ring and the LED are declared texture sheets; the casing is a closed sheet volume, its interior is not modelled.",
  "Thermal: the casing is its outer prism with the window opening (the back against the wall is a section face), the window the front face of the glass, the impeller its hub and nine blades, the ducts four closed 98 mm cylinders under the spigots (lateral area and spigot beads give the 0.12 m2 exposed; areaM2 corrected at the root from 0.08, factor 1.5); the two air emitters are 8 mm discs of 98 mm on the spigot tops whose faces are openings.",
  "Structure: the casing is a sheet shell (8 mm tube, top, bottom, back, front cover), the four spigots are the duct members, hub and blades the impeller member, a shaft as joint, two flange rings per spigot row, hook and rest as 30 mm bars from the wall to the casing mid-plane and one mass."],
 ["Körper: der Seed bleibt: ein 270 mm tiefes gerundetes Blechgehäuse mit 30 mm starker Frontabdeckung, die die 224-mm-Fensteröffnung hat, der 256-mm-Fensterring, das Glas, das Laufrad mit neun Schaufeln samt Nabe (Schaufeln 18 mm tief, je ein gekrümmtes Profil aus 17 Punkten), die LED und die Griffmulde, dazu vier hohle Stutzen mit Wülsten. Die Griffmulde ist jetzt 12 mm tief (vorher ein 6-mm-Band), Fensterring und LED sind als Texturblätter erklärt; das Gehäuse ist ein geschlossenes Blechvolumen, sein Inneres ist nicht modelliert.",
  "Thermik: das Gehäuse ist sein Außenprisma mit der Fensteröffnung (die Rückseite an der Wand ist eine Schnittfläche), das Fenster die Vorderfläche des Glases, das Laufrad seine Nabe und neun Schaufeln, die Kanäle vier geschlossene 98-mm-Zylinder unter den Stutzen (Mantelfläche und Stutzenwülste ergeben die freien 0,12 m2; areaM2 an der Wurzel von 0,08 korrigiert, Faktor 1,5); die zwei Luft-Emitter sind 8-mm-Scheiben von 98 mm auf den Stutzenenden, deren Flächen Öffnungen sind.",
  "Tragwerk: das Gehäuse ist eine Blechschale (8-mm-Rohr, Deckel, Boden, Rückwand, Frontabdeckung), die vier Stutzen sind die Kanalstäbe, Nabe und Schaufeln der Laufradstab, eine Welle das Gelenk, zwei Flanschringe je Stutzenreihe, Haken und Anlage als 30-mm-Stäbe von der Wand bis zur Gehäusemittelebene und eine Masse."])
# ---- 1:100
BL=lambda n,t:ext("blade","impeller","pp-impeller",0.4,[(0.048,0.012),(0.07,0.0),(0.095,-0.03),(0.108,-0.07),(0.07,-0.06),(0.048,-0.014)],"xy",-0.141,-0.141+t,repeat={"count":n,"rotationAxis":[0,0,1],"rotationRad":6.2832/n,"origin":[0,0,0]})
def spig_tube(id,r,rin,h):
    out=[]
    for k,(x0,z0) in enumerate([(-0.225,0.07),(-0.075,-0.07)]):
        d=tube(id+"-"+"ab"[k],"spigots","galvanised-steel",0.7,r,rin,0.0,h,c=(x0,z0),n=24); d["repeat"]={"count":2,"translation":[0.3,0,0]}; out.append(d)
    return out
B100=[box("casing","casing","painted-steel",0.88,(0.6,0.6,0.27),(0,0.3,0.015),r=0.004),ext("casing-wall","casing","painted-steel",0.88,[(p[0],p[1]) for p in rrect(0.6,0.6,0.004,2,(0,0.3))],"xy",-0.15,-0.12,holes=[[(p[0],p[1]+0.3) for p in circle(0.118,32)]]),
 rev("window","casing","polycarbonate",0.8,[(-0.1495,0.118),(-0.1515,0.118)],axis=(0,0,1),origin=(0,0.3,0),role="glazing"),
 rev("hub","impeller","pp-impeller",0.4,[(-0.142,0.048),(-0.126,0.048)],axis=(0,0,1)),BL(7,0.018),*spig_tube("spigot",0.049,0.041,0.062)]
TH100=[th_casing(0.118,0.004),th_window(0.118,0.002,-0.1495),ref("th-hub","hub","impeller"),ref("th-blade","blade","impeller"),
 *[th(d,"ducts",id="th-"+d["id"]) for d in spig_cyl("duct",0.049,0.062)],emit("supply-air",0.075,0.07,0.015,0.0695),emit("extract-air",-0.225,0.07,0.015,0.0695)]
ST100=[st(ext("m","casing","painted-steel",0.88,[(q[0],q[1]) for q in rrect(0.6,0.6,0.004,3,(0,0.3))],"xy",-0.12,0.135,holes=[[(q[0],q[1]) for q in rrect(0.57,0.57,0.001,3,(0,0.3))]]),"casing","member",id="member-shell"),
 st(box("m","casing","painted-steel",0.88,(0.58,0.58,0.015),(0,0.3,0.1425)),"casing","member",id="member-back"),
 sref("member-spigot-a","spigot-a","spigots","member"),sref("member-spigot-b","spigot-b","spigots","member"),
 sref("member-hub","hub","impeller","member"),sref("member-blades","blade","impeller","member"),
 st(cyl("j","impeller","pp-impeller",0.4,0.0155,-0.12,0.0,c=(0,0),axis="z",role="detail"),"impeller","joint",id="joint-impeller-shaft"),
 sup("support-wall-hook",0.57,"wall-hook",0.031),sup("support-wall-rest",0.03,"wall-rest",0.031),mass("mass-casing","casing",(0.2,0.2,0.1),(0,0.3,0))]
V100=vol(100,B100,TH100,ST100,
 ["1:100 draws the unit as nine solids: one rounded 600 x 600 x 300 mm casing (front cover, ring, LED and grip merged), the window glass, the hub with a seven-blade impeller (the drawing has seven blades, each a six-point plate), and four hollow spigots with their beads merged.",
  "Thermal: casing prism with the 236 mm opening, glass face, hub and blades, four closed cylinders for the duct surface and the two 15 mm emitter discs.",
  "Structure: 15 mm casing shell with top, bottom and back plate, the four spigots, hub and blades, the impeller shaft, hook and rest and one mass; the spigot flanges are merged into the tubes."],
 ["1:100 zeichnet das Gerät aus neun Körpern: ein gerundetes Gehäuse 600 x 600 x 300 mm (Frontabdeckung, Ring, LED und Griffmulde aufgegangen), das Fensterglas, die Nabe mit siebenschaufligem Laufrad (die Zeichnung hat sieben Schaufeln, je eine Sechspunktplatte) und vier hohle Stutzen, deren Wülste aufgegangen sind.",
  "Thermik: Gehäuseprisma mit der 236-mm-Öffnung, Glasfläche, Nabe und Schaufeln, vier geschlossene Zylinder für die Kanalfläche und die zwei 15-mm-Emitterscheiben.",
  "Tragwerk: 15-mm-Gehäuseschale mit Deckel, Boden und Rückwand, die vier Stutzen, Nabe und Schaufeln, die Laufradwelle, Haken und Anlage und eine Masse; die Stutzenflansche gehen in den Rohren auf."])
# ---- 1:200
BL4=ext("blade","impeller","pp-impeller",0.4,[(0.056,0.02),(0.09,0.0),(0.112,-0.045),(0.1,-0.095),(0.056,-0.02)],"xy",-0.1415,-0.111,repeat={"count":4,"rotationAxis":[0,0,1],"rotationRad":1.5708,"origin":[0,0,0]})
def spig_solid(id,r,h):
    return spig_cyl(id,r,h)
B200=[box("casing","casing","painted-steel",0.88,(0.6,0.6,0.27),(0,0.3,0.015)),ext("casing-wall","casing","painted-steel",0.88,[(p[0],p[1]) for p in rrect(0.6,0.6,0.004,1,(0,0.3))],"xy",-0.15,-0.12,holes=[[(p[0],p[1]+0.3) for p in circle(0.125,16)]]),
 rev("window","casing","polycarbonate",0.8,[(-0.1495,0.125),(-0.1515,0.125)],axis=(0,0,1),origin=(0,0.3,0),role="glazing"),
 rev("hub","impeller","pp-impeller",0.4,[(-0.1415,0.056),(-0.111,0.056)],axis=(0,0,1)),BL4,*spig_cyl("spigot",0.045,0.065)]
TH200=[th_casing(0.125,0.005),th_window(0.125,0.002,-0.1495),ref("th-hub","hub","impeller"),ref("th-blade","blade","impeller"),
 *[th(d,"ducts",id="th-"+d["id"]) for d in spig_cyl("duct",0.049,0.065)],emit("supply-air",0.075,0.07,0.0305,0.0945),emit("extract-air",-0.225,0.07,0.0305,0.0945)]
for t_ in TH200[-2:]:
    pass
ST200=[st(ext("m","casing","painted-steel",0.88,[(p[0],p[1]) for p in rrect(0.6,0.6,0.005,3,(0,0.3))],"xy",-0.12,0.1195,holes=[[(p[0],p[1]) for p in rrect(0.539,0.539,0.001,3,(0,0.3))]]),"casing","member",id="member-shell"),
 st(box("m","casing","painted-steel",0.88,(0.58,0.58,0.0305),(0,0.3,0.1348)),"casing","member",id="member-back"),
 sref("member-spigot-a","spigot-a","spigots","member"),sref("member-spigot-b","spigot-b","spigots","member"),
 sref("member-hub","hub","impeller","member"),sref("member-blades","blade","impeller","member"),
 sup("support-wall-hook",0.57,"wall-hook",0.031),sup("support-wall-rest",0.03,"wall-rest",0.031),mass("mass-casing","casing",(0.2,0.2,0.1),(0,0.3,0))]
V200=vol(200,B200,TH200,ST200,
 ["1:200 keeps the casing, the window glass, a four-blade impeller with its hub and the four spigots as solid 90 mm cylinders; casing details, ring and beads are gone.",
  "Thermal: casing prism with the 250 mm opening, glass face, hub and blades, the four duct cylinders and the two 30 mm emitter discs.",
  "Structure: 30 mm shell tube and back plate (top and bottom merged into the shell tube), spigots, hub and blades, hook and rest and one mass."],
 ["1:200 behält das Gehäuse, das Fensterglas, ein vierschaufliges Laufrad mit Nabe und die vier Stutzen als massive 90-mm-Zylinder; Gehäusedetails, Ring und Wülste entfallen.",
  "Thermik: Gehäuseprisma mit der 250-mm-Öffnung, Glasfläche, Nabe und Schaufeln, die vier Kanalzylinder und die zwei 30-mm-Emitterscheiben.",
  "Tragwerk: 30-mm-Schalenrohr und Rückwand (Deckel und Boden im Schalenrohr aufgegangen), Stutzen, Nabe und Schaufeln, Haken und Anlage und eine Masse."])
write("mvhr-unit",{"50":V50,"100":V100,"200":V200,"500":unavail(500)})
