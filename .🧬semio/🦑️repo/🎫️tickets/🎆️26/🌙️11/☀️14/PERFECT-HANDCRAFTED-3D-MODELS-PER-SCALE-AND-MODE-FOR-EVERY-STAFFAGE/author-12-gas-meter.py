from author_12_lib import *
S=seed("gas-meter")
B=[json.loads(json.dumps(s)) for s in S["body"]]
setrole(B,"texture","faces","bead","counter","digits-black","digits-red","dial","ticks","plate","plate-line-1","plate-line-2","needle","pivot")
for s in B:
    if s["id"]=="index-frame": s["from"]=-0.0855
BODY50=B
nd=get(B,"needle"); nd["from"]=-0.0865; nd["to"]=-0.086
TH50=[ref("th-lower-shell","lower-shell","shell","shell"),ref("th-upper-shell","upper-shell","shell","shell"),ref("th-seam","seam","shell","shell"),
 ref("th-plate","plate","shell","shell"),
 ref("th-index-frame","index-frame","index","head"),ref("th-index-base","index-base","index","head"),ref("th-counter","counter","index","head"),ref("th-dial","dial","index","head"),ref("th-needle","needle","index","head"),
 ref("th-stubs","stubs","pipes","tube"),ref("th-unions","unions","pipes","tube"),ref("th-faces","faces","pipes","tube")]
def pipe_tubes(r,rin,y0,y1,id,cnt=2):
    return st(tube("p","body","steel-pipe",0.4,r,rin,y0,y1,c=(-0.055,0.0075)),"body","member",id=id,repeat={"count":2,"translation":[0.11,0,0]})
ST50=[st(rtube("m","body","die-cast-aluminium",0.7,0.24,0.155,0.008,-0.11,0.11,c=(0,0.0075)),"body","member",id="member-shell"),
 st(box("m","body","die-cast-aluminium",0.7,(0.22,0.2,0.008),(0,0,0.0765)),"body","member",id="member-backplate"),
 pipe_tubes(0.0215,0.0155,0.11,0.154,"member-pipes"),
 st(cyl("m","pointer","abs-plastic",0.3,0.0045,-0.0865,-0.06,c=(0,0),axis="z"),"pointer","member",id="member-pointer-shaft"),
 st(ell("j","pointer","abs-plastic",0.3,(0.008,0.008,0.008),(0,0,-0.07),role="detail"),"pointer","joint",id="joint-pointer"),
 st(box("s","body","steel-pipe",0.3,(0.02,0.02,0.09),(0,0.1,0.04)),"body","support",id="support-wall-bracket-top",anchor="wall-bracket-top"),
 st(box("s","body","steel-pipe",0.3,(0.02,0.02,0.09),(0,-0.1,0.04)),"body","support",id="support-wall-bracket-bottom",anchor="wall-bracket-bottom"),
 st(ell("m","body","die-cast-aluminium",0.7,(0.08,0.08,0.05),(0,0,0.01),role="detail"),"body","mass",id="mass-body")]
V50=vol(50,BODY50,TH50,ST50,
 ["Body: the seed is kept and repaired: a two-part die-cast shell with a 12 mm seam, a crimp bead, the 15 mm index head with its counter, eight digit cells, a 56 mm dial with four ticks, the needle with its pivot, the name plate and two pipe stubs with unions. Anything below 7.5 mm (digits, counter, dial, ticks, plate, bead, pipe faces, needle, pivot) is a declared texture sheet; the index frame is 7.5 mm deep.",
  "Thermal: shell, index head and pipes reference the body solids; exposed surface of the shell is 0.165 m2 against the declared 0.26 (listed in area-review.md, beyond factor 1.5), index head 0.029 and pipes 0.017 (areaM2 corrected at the root from 0.02 and 0.014).",
  "Structure: the shell is a hollow 8 mm cast-aluminium tube with a back plate, the inlet and outlet pipes are thin-walled tubes, the pointer has a 9 mm shaft and a joint, the upper bracket (pin) and the lower bracket (roller) are 20 mm hooks between the wall and the casing, one mass ellipsoid carries the 1.95 kg."],
 ["Körper: der Seed bleibt und ist repariert: eine zweiteilige Druckgussschale mit 12-mm-Naht, einem Bördelwulst, dem 15 mm tiefen Zählwerkskopf mit Zählwerk, acht Ziffernzellen, einer 56-mm-Scheibe mit vier Strichen, der Nadel mit Zapfen, dem Typenschild und zwei Rohrstutzen mit Verschraubungen. Alles unter 7,5 mm (Ziffern, Zählwerk, Scheibe, Striche, Schild, Wulst, Rohrstirnen, Nadel, Zapfen) ist als Texturblatt erklärt; der Zählwerkrahmen ist 7,5 mm tief.",
  "Thermik: Schale, Zählwerkskopf und Rohre verweisen auf Körperkörper; die freie Fläche der Schale beträgt 0,165 m2 gegen die erklärten 0,26 (in area-review.md aufgeführt, jenseits Faktor 1,5), des Zählwerkskopfs 0,029 und der Rohre 0,017 (areaM2 an der Wurzel von 0,02 und 0,014 korrigiert).",
  "Tragwerk: die Schale ist ein hohles 8-mm-Aluminiumgussrohr mit Rückplatte, Ein- und Austrittsrohr sind dünnwandige Rohre, der Zeiger hat eine 9-mm-Welle und ein Gelenk, der obere Halter (Festlager) und der untere Halter (Rollenlager) sind 20-mm-Haken zwischen Wand und Gehäuse, ein Masseellipsoid trägt die 1,95 kg."])
# 1:100
HOUSE=box("housing","body","die-cast-aluminium",0.7,(0.24,0.22,0.155),(0,0,0.0075),r=0.012)
IDX=box("index","body","abs-plastic",0.5,(0.19,0.098,0.015),(0,0.049,-0.0775),r=0.004)
def studs(id,r,y0,y1): 
    return ext(id,"body","steel-pipe",0.4,circle(r,24),"zx",y0,y1,c=(0,0),repeat={"count":2,"translation":[0.11,0,0]})
def cylx(id,r,y0,y1,xc,zc,**kw):
    d=cyl(id,"body","steel-pipe",0.4,r,y0,y1,c=(xc,zc),**kw); return d
B100=[HOUSE,IDX,
 box("counter","body","abs-plastic",0.1,(0.151,0.029,0.002),(0,0.0725,-0.0785),role="texture"),
 rev("dial","body","abs-plastic",0.9,[(0,0.028),(0.002,0.028)],axis=(0,0,1),origin=(0,0.018,-0.0875),role="texture"),
 ext("needle","pointer","abs-plastic",0.1,[(-0.006,-0.006),(0.006,-0.006),(0.0025,0.026),(-0.0025,0.026)],"xy",-0.0885,-0.0865,role="texture"),
 box("plate","body","abs-plastic",0.8,(0.14,0.045,0.002),(0,-0.0625,-0.071),role="texture"),
 cylx("stubs",0.014,0.11,0.154,-0.055,0.0075,repeat={"count":2,"translation":[0.11,0,0]}),
 cylx("unions",0.0225,0.108,0.148,-0.055,0.0075,repeat={"count":2,"translation":[0.11,0,0]})]
for s in B100:
    if s["id"] in("stubs","unions"): s["repeat"]={"count":2,"translation":[0.11,0,0]}
TH100=[ref("th-housing","housing","shell"),ref("th-index","index","index"),ref("th-counter","counter","index"),ref("th-dial","dial","index"),ref("th-needle","needle","index"),ref("th-stubs","stubs","pipes"),ref("th-unions","unions","pipes")]
ST100=[st(rtube("m","body","die-cast-aluminium",0.7,0.24,0.155,0.015,-0.11,0.11,c=(0,0.0075)),"body","member",id="member-shell"),
 st(box("m","body","die-cast-aluminium",0.7,(0.22,0.2,0.015),(0,0,0.073)),"body","member",id="member-backplate"),
 st(tube("p","body","steel-pipe",0.4,0.0135,0.0075,0.11,0.154,c=(-0.055,0.0075)),"body","member",id="member-pipes",repeat={"count":2,"translation":[0.11,0,0]}),
 st(cyl("m","pointer","abs-plastic",0.3,0.0078,-0.0865,-0.06,c=(0,0),axis="z"),"pointer","member",id="member-pointer-shaft"),
 st(box("s","body","steel-pipe",0.3,(0.02,0.02,0.09),(0,0.1,0.04)),"body","support",id="support-wall-bracket-top",anchor="wall-bracket-top"),
 st(box("s","body","steel-pipe",0.3,(0.02,0.02,0.09),(0,-0.1,0.04)),"body","support",id="support-wall-bracket-bottom",anchor="wall-bracket-bottom"),
 st(ell("m","body","die-cast-aluminium",0.7,(0.08,0.08,0.05),(0,0,0.01),role="detail"),"body","mass",id="mass-body")]
V100=vol(100,B100,TH100,ST100,
 ["1:100 draws the meter as seven solids: one rounded housing box that takes the two shells, seam and bead, the 15 mm index head, counter, dial, needle and name plate as thin sheets, and two 28 mm stubs with 40 mm unions. Digits, ticks, pivot and pipe faces vanish below 15 mm.",
  "Thermal: housing, index head with counter, dial and needle, stubs and unions, all referencing body solids of this scale.",
  "Structure: hollow 15 mm shell tube with back plate, thin-walled pipes, the 15.6 mm pointer shaft, the two 20 mm brackets and one mass; the pointer joint is merged into the shaft."],
 ["1:100 zeichnet den Zähler aus sieben Körpern: ein gerundeter Gehäusekasten, der beide Schalen, Naht und Wulst aufnimmt, der 15 mm tiefe Zählwerkskopf, Zählwerk, Scheibe, Nadel und Typenschild als dünne Blätter und zwei 28-mm-Stutzen mit 40-mm-Verschraubungen. Ziffern, Striche, Zapfen und Rohrstirnen verschwinden unter 15 mm.",
  "Thermik: Gehäuse, Zählwerkskopf mit Zählwerk, Scheibe und Nadel, Stutzen und Verschraubungen, alle als Verweise auf Körperkörper dieses Maßstabs.",
  "Tragwerk: hohles 15-mm-Schalenrohr mit Rückplatte, dünnwandige Rohre, die 15,6-mm-Zeigerwelle, die beiden 20-mm-Halter und eine Masse; das Zeigergelenk geht in die Welle auf."])
# 1:200
B200=[box("housing","body","die-cast-aluminium",0.7,(0.24,0.22,0.17),(0,0,0),r=0.012),
 box("counter","body","abs-plastic",0.1,(0.15,0.04,0.003),(0,0.074,-0.0865),role="texture"),
 rev("dial","body","abs-plastic",0.9,[(0,0.03),(0.003,0.03)],axis=(0,0,1),origin=(0,0.018,-0.0885),role="texture"),
 ext("needle","pointer","abs-plastic",0.1,[(-0.007,-0.006),(0.007,-0.006),(0.003,0.028),(-0.003,0.028)],"xy",-0.0905,-0.0875,role="texture"),
 cyl("stubs","body","steel-pipe",0.4,0.02,0.1,0.16,c=(-0.055,0.0075),repeat={"count":2,"translation":[0.11,0,0]})]
TH200=[ref("th-housing","housing","shell"),
 th(box("i","body","abs-plastic",0.5,(0.19,0.098,0.003),(0,0.049,-0.0865),role="texture"),"index",id="th-index"),ref("th-stubs","stubs","pipes")]
ST200=[st(rtube("m","body","die-cast-aluminium",0.7,0.24,0.17,0.03,-0.11,0.11,c=(0,0)),"body","member",id="member-shell"),
 st(box("m","body","die-cast-aluminium",0.7,(0.2,0.2,0.03),(0,0,0.07)),"body","member",id="member-backplate"),
 st(cyl("p","body","steel-pipe",0.4,0.0195,0.0,0.16,c=(-0.055,0.0075)),"body","member",id="member-pipe-stubs",repeat={"count":2,"translation":[0.11,0,0]}),
 st(cyl("m","pointer","abs-plastic",0.3,0.0155,-0.09,-0.05,c=(0,0),axis="z"),"pointer","member",id="member-pointer-shaft"),
 st(box("s","body","steel-pipe",0.3,(0.031,0.031,0.09),(0,0.1,0.04)),"body","support",id="support-wall-bracket-top",anchor="wall-bracket-top"),
 st(box("s","body","steel-pipe",0.3,(0.031,0.031,0.09),(0,-0.1,0.04)),"body","support",id="support-wall-bracket-bottom",anchor="wall-bracket-bottom"),
 st(ell("m","body","die-cast-aluminium",0.7,(0.08,0.08,0.05),(0,0,0.01),role="detail"),"body","mass",id="mass-body")]
V200=vol(200,B200,TH200,ST200,
 ["1:200 is housing, counter plate, dial, needle and the two pipe stubs: the 17 cm deep rounded box carries the index head merged into its front, the stubs are 40 mm pins.",
  "Thermal: housing, the index sheet in front and the stubs.",
  "Structure: 30 mm shell tube with back plate, one 45 mm pipe member through both stubs, the 31 mm pointer shaft, two 30 mm brackets and one mass."],
 ["1:200 besteht aus Gehäuse, Zählwerksplatte, Scheibe, Nadel und den zwei Rohrstutzen: der 17 cm tiefe gerundete Kasten trägt den Zählwerkskopf in seiner Front aufgegangen, die Stutzen sind 40-mm-Zapfen.",
  "Thermik: Gehäuse, das Zählwerksblatt davor und die Stutzen.",
  "Tragwerk: 30-mm-Schalenrohr mit Rückplatte, ein 45-mm-Rohrstab durch beide Stutzen, die 31-mm-Zeigerwelle, zwei 30-mm-Halter und eine Masse."])
write("gas-meter",{"50":V50,"100":V100,"200":V200,"500":unavail(500)})
