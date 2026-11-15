from author_12_lib import *
S50=seed("pendant-lamp")
B50=[json.loads(json.dumps(s)) for s in S50["body"]]
for s in B50:
    if s["id"]=="cord": s["profile"]=[[-0.521,0.00375],[0,0.00375]]
def dome(n): return [(y,r) for (r,y) in bez((0.017,0),(0.06,0),(0.11,-0.03),(0.13,-0.133),n)]
def dome200(n): return [(0.03,0.025)]+[(y,r) for (r,y) in bez((0.025,0),(0.06,0),(0.11,-0.03),(0.133,-0.1385),n)]
GL=[(0,0),(0.0006,0.0053),(0.0025,0.0106),(0.0054,0.0156),(0.0093,0.0203),(0.0140,0.0243),(0.0194,0.0273),(0.0255,0.0293),(0.032,0.03),(0.0414,0.0288),(0.0494,0.0259),(0.0586,0.0209),(0.0700,0.0158),(0.078,0.0138)]
def shell(id,bone,mat,tone,wall,n,out=None):
    out=out or dome(n); inn=[(y,max(r-wall,0.002)) for (y,r) in out]
    return rev(id,bone,mat,tone,out+list(reversed(inn)))
def jnt(id,bone,y,r): return st(ell("j",bone,"brass-lacquered",0.5,(r,r,r),(0,y,0)),bone,"joint",id=id)
def mass(id,bone,mat,tone,radii,c): return st(ell("m",bone,mat,tone,radii,c,role="detail"),bone,"mass",id=id)
def members(cr,stem,hook,massr,joints,extra):
    out=[st(cyl("m","cord","textile-cord",0.15,cr,-0.521,0),"cord","member",id="member-cord"),
     sref("member-shade","shade","shade","member"),
     st(cyl("m","bulb","brass-lacquered",0.45,stem,0.105,0.225),"bulb","member",id="member-bulb-stem")]
    out+=joints
    out.append(st(cap("m","rose","brass-lacquered",0.3,(0,0,-hook*3),(0,0,hook*3),hook),"rose","support",id="support-ceiling-hook"))
    out.append(mass("mass-shade","shade","enamel-white",0.4,massr,(0,-0.06,0)))
    return out+extra
plume=lambda f,r:ext("th-shade-plume","shade","enamel-white",0.4,circle(r,24),"zx",0.0,f,role="detail",region="shade",tags=["shade-plume"])
roseskin=lambda y0,y1,disc=True:skin("th-rose","rose","rose-polymer",0.7,[(y1,0.05),(y0,0.05)]+([(y0+0.0001,0.0005)] if disc else []),region="rose")
COLLAR=[(0,0.01),(-0.006,0.0105),(-0.0075,0.0155),(-0.03,0.0155)]
th50=[skin("th-rose","rose","rose-polymer",0.7,[(-0.012,0.05),(-0.024,0.05),(-0.0239,0.0005)],region="rose",layer="rose-shell"),ref("th-cord","cord","cord","sheath"),
 skin("th-collar","holder","brass-lacquered",0.45,COLLAR,region="lampholder",layer="brass-shell"),
 skin("th-bulb-cap","bulb","brass-lacquered",0.5,[(0.105,0.0135),(0.078,0.0135)],region="lampholder",layer="brass-shell"),
 ref("th-bulb-glass","bulb-glass","bulb-glass","envelope"),
 skin("th-shade","shade","enamel-white",0.4,dome(14),region="shade",layer="shade-steel"),plume(0.0075,0.017)]
str50=members(0.00375,0.008,0.005,(0.045,0.03,0.045),
 [jnt("joint-cord","cord",0,0.005),jnt("joint-holder","holder",0,0.0075),jnt("joint-shade","shade",0,0.0075),jnt("joint-bulb","bulb",0,0.0075)],
 [sref("member-bulb-cap","bulb-cap","bulb","member"),mass("mass-rose","rose","rose-polymer",0.7,(0.03,0.01,0.03),(0,-0.012,0)),
  mass("mass-cord","cord","textile-cord",0.15,(0.0045,0.03,0.0045),(0,-0.26,0)),mass("mass-holder","holder","brass-lacquered",0.45,(0.01,0.01,0.01),(0,-0.015,0)),
  mass("mass-bulb","bulb","bulb-glass",0.9,(0.015,0.02,0.015),(0,0.05,0))])
V50=vol(50,B50,th50,str50,
 "Rose, cord, collar, shade, cap and glass bulb are single revolves about the vertical axis: the pendant is axially symmetric, so a round model is exact in all three views. The shade is a 2.5 mm enamelled sheet shell with a rolled rim, the cord is widened from 6.6 to the 7.5 mm smallest feature of 1:50, the bulb is the A60 envelope with a rounded shoulder and a neck into the E27 cap. Thermal: rose, cord and bulb glass reuse the body surface; collar and cap are open one-sided skins so the lampholder area is the 0.0042 m2 of the region (corrected from 0.005) and not both faces; the shade is a one-sided dome skin of 0.11 m2; the plume is a flat disc on the shade apex as wide as the shade because the 0.3 m far-field plume would leave the body box. Structure: the cord is the tension member, the shade dome and the socket stem down to the cap carry the bulb; joints sit at the four fixed rig joints, a pin at the ceiling hook, mass solids at the centres of mass.",
 "Rosette, Kabel, Hülse, Schirm, Sockel und Glaskolben sind einzelne Rotationskörper um die senkrechte Achse: die Pendelleuchte ist achsensymmetrisch, ein runder Körper stimmt daher in allen drei Ansichten. Der Schirm ist eine 2,5 mm dicke emaillierte Blechschale mit Wulstrand, das Kabel ist von 6,6 auf die kleinste Strukturgröße von 7,5 mm des Maßstabs 1:50 verbreitert, der Kolben ist die A60-Hülle mit gerundeter Schulter und Hals in den E27-Sockel. Thermik: Rosette, Kabel und Glaskolben nutzen die Körperfläche; Hülse und Sockel sind offene einseitige Häute, damit die Fassungsfläche die 0,0042 m2 der Region (korrigiert von 0,005) beträgt und nicht beide Seiten; der Schirm ist eine einseitige Kuppelhaut von 0,11 m2; die Fahne ist eine flache Scheibe auf dem Scheitel, so breit wie der Schirm, weil die 0,3 m breite Fernfahne den Körperkasten verlassen würde. Tragwerk: Das Kabel ist der Zugstab, die Schirmkuppel und der Fassungsschaft bis zum Sockel tragen die Lampe; Gelenke an den vier festen Rig-Gelenken, ein Festlager am Deckenhaken und Massekörper in den Schwerpunkten.")
B100=[rev("rose","rose","rose-polymer",0.7,[(-0.024,0.05),(-0.0087,0.0495),(-0.003,0.042),(0,0.032)]),
 cyl("cord","cord","textile-cord",0.15,0.004,-0.521,0),
 rev("collar","holder","brass-lacquered",0.45,[(0,0.0105),(-0.0075,0.0155),(-0.03,0.0155)]),
 shell("shade","shade","enamel-white",0.4,0.0035,10),
 rev("bulb-cap","bulb","brass-lacquered",0.5,[(0.078,0.0135),(0.105,0.0135)]),
 rev("bulb-glass","bulb","bulb-glass",0.9,GL[::2]+[GL[-1]])]
th100=[roseskin(-0.024,-0.007),ref("th-cord","cord","cord"),
 ref("th-collar","collar","lampholder"),ref("th-bulb-cap","bulb-cap","lampholder"),ref("th-bulb-glass","bulb-glass","bulb-glass"),
 skin("th-shade","shade","enamel-white",0.4,dome(8),region="shade"),plume(0.015,0.017)]
str100=members(0.004,0.0105,0.0075,(0.05,0.03,0.05),[jnt("joint-shade","shade",0,0.0105),jnt("joint-bulb","bulb",0,0.0105)],
 [mass("mass-cord","cord","textile-cord",0.15,(0.009,0.03,0.009),(0,-0.26,0))])
V100=vol(100,B100,th100,str100,
 "1:100 redraws the pendant against its own shapes: the rose is a four-point revolve, the cord is a slender part of 8 mm (drawn 9 mm, exempt from the 15 mm feature; the thermal cord is the body cord, so 8 mm keeps its surface inside 30 % of 0.0108 m2), collar and cap stay separate brass parts, the shade is a 6 mm shell on a 10-segment dome and the bulb a seven-point glass profile. The rose, cord-holder and cord joints are merged into the members; the thermal surfaces are re-cut from coarser skins and the plume disc is 15 mm thick. Structure keeps the cord as tension member, the dome and the socket stem, joints at shade and bulb, a 15 mm pin and the masses of shade and cord.",
 "1:100 zeichnet die Pendelleuchte gegen ihre eigenen Formen neu: die Rosette ist ein Rotationskörper aus vier Punkten, das Kabel ist ein schlankes Teil von 8 mm (gezeichnet 9 mm, von der Strukturgröße 15 mm ausgenommen; das Thermikkabel ist das Körperkabel, 8 mm hält seine Fläche innerhalb 30 % von 0,0108 m2), Hülse und Sockel bleiben getrennte Messingteile, der Schirm ist eine 6-mm-Schale auf einer Kuppel mit 10 Segmenten und der Kolben ein Glasprofil aus sieben Punkten. Die Gelenke an Rosette, Kabel und Hülse sind in die Stäbe aufgegangen; die Thermikflächen sind aus gröberen Häuten neu geschnitten und die Fahnenscheibe ist 15 mm dick. Das Tragwerk behält das Kabel als Zugstab, die Kuppel und den Fassungsschaft, Gelenke an Schirm und Kolben, ein 15-mm-Festlager und die Massen von Schirm und Kabel.")
B200=[cyl("rose","rose","rose-polymer",0.7,0.05,-0.03,0),cyl("cord","cord","textile-cord",0.15,0.0043,-0.521,0),
 shell("shade","shade","enamel-white",0.4,0.0035,6,out=dome200(6)),
 rev("bulb","bulb","bulb-glass",0.9,[GL[0],GL[3],GL[6],GL[8],GL[10],GL[12],(0.08,0.016)])]
th200=[roseskin(-0.0306,0.0,True),ref("th-cord","cord","cord"),ref("th-bulb","bulb","bulb-glass"),
 skin("th-shade","shade","enamel-white",0.4,dome200(6),region="shade"),
 skin("th-collar","shade","brass-lacquered",0.45,[(0.03,0.0255),(0.0,0.0255)],region="lampholder"),
 plume(0.03,0.0155)]
str200=[st(cyl("m","cord","textile-cord",0.15,0.0043,-0.521,0),"cord","member",id="member-cord"),
 sref("member-shade","shade","shade","member"),
 st(cyl("m","bulb","brass-lacquered",0.45,0.0155,0.08,0.225),"bulb","member",id="member-bulb-stem"),
 st(cap("m","rose","brass-lacquered",0.3,(0,0,-0.045),(0,0,0.045),0.0155),"rose","support",id="support-ceiling-hook"),
 mass("mass-shade","shade","enamel-white",0.4,(0.055,0.035,0.055),(0,-0.06,0))]
V200=vol(200,B200,th200,str200,
 "1:200 merges the pendant into four masses, as its own shapes do: a 30 mm thick disc rose, a slender cord of 8.6 mm (drawn 14 mm, exempt from the 30 mm feature; kept thin so that the cord surface stays inside 30 % of 0.0108 m2), one 3.5 mm shell dome that takes the collar as a closed neck, and one bulb of six glass points with the cap inside the dome. Thermal: rose, cord and bulb reuse body solids; shade is a six-point skin; the lampholder is two 31 and 29 mm skins at the collar and the cap; the plume disc is 30 mm thick. Structure: cord (8.6 mm) and socket stem (31 mm) as members, the dome as the shade member, a 31 mm pin and one mass; joints are merged into the members.",
 "1:200 fasst die Pendelleuchte wie ihre eigenen Formen zu vier Massen zusammen: eine 30 mm hohe Scheibe als Rosette, ein schlankes Kabel von 8,6 mm (gezeichnet 14 mm, von der Strukturgröße 30 mm ausgenommen; dünn gehalten, damit die Kabelfläche innerhalb 30 % von 0,0108 m2 bleibt), eine 3,5-mm-Kuppelschale, die die Hülse als geschlossenen Hals aufnimmt und ein Kolben aus sechs Glaspunkten, dessen Sockel in der Kuppel liegt. Thermik: Rosette, Kabel und Kolben nutzen Körper; der Schirm ist eine Haut aus sechs Punkten; die Fassung besteht aus zwei Häuten von 31 und 29 mm an Hülse und Sockel; die Fahnenscheibe ist 30 mm dick. Tragwerk: Kabel (8,6 mm) und Fassungsschaft (31 mm) als Stäbe, die Kuppel als Schirmstab, ein 31-mm-Festlager und eine Masse; Gelenke sind in die Stäbe aufgegangen.")
write("pendant-lamp",{"50":V50,"100":V100,"200":V200,"500":unavail(500)})
