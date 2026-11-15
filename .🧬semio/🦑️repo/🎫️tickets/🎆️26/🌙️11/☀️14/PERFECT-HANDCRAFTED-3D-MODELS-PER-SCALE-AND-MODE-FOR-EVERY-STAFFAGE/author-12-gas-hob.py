from author_12_lib import *
S=seed("gas-hob")
B=[json.loads(json.dumps(s)) for s in S["body"]]
B.append(rev("flame-seat","flame","flame",0.95,[(0,0.033),(0.0075,0.033),(0.0075,0.04),(0,0.04),(0,0.033)],origin=(0,0,0.03),role="detail"))
setrole(B,"texture","vent-rise","vent-fall","knob-pointer")
for s in B:
    if s["id"]=="burner-ring": s["profile"]=[[0,0.052],[0.0075,0.046]]
SEC=lambda d,faces: d.__setitem__("sectionFaces",faces) or d
def plate_th(id,thick=0.01):
    return SEC(th(get(B,"plate") if thick==0.01 else None,"plate",id=id),["-v","+u","-u","+w","-w"])
TH50=[SEC(th(get(B,"plate"),"plate",id="th-plate"),["-v","+u","-u","+w","-w"]),
 SEC(th(get(B,"knob"),"knob",id="th-knob"),["+w"]),ref("th-knob-pointer","knob-pointer","knob"),
 SEC(th(get(B,"burner-ring"),"burner",id="th-burner-ring"),["-v"]),ref("th-burner-cap","burner-cap","burner"),
 SEC(th(get(B,"flame-ring"),"flame",id="th-flame-ring"),["-v"]),ref("th-flame-centre","flame-centre","flame"),
 ref("th-trivet-rail","trivet-rail","trivet"),ref("th-trivet-legs-front","trivet-legs-front","trivet"),ref("th-trivet-legs-back","trivet-legs-back","trivet"),
 SEC(th(cyl("p","flame-tip","flame",0.95,0.05,0.0138,0.0218,c=(0,0.03),role="detail"),"flame",tags=["flame-plume"],id="th-flame-plume"),["+v","-v"])]
HOUS=tube
def rtube(id,bone,mat,tone,w,d,t,y0,y1,c=(0,0)):
    o=[(p[0]+c[0],p[1]+c[1]) for p in rrect(w,d,0.002,2)]
    i=[(p[0]+c[0],p[1]+c[1]) for p in rrect(w-2*t,d-2*t,0.001,2)]
    return ext(id,bone,mat,tone,[(q[1],q[0]) for q in o],"zx",y0,y1,holes=[[(q[1],q[0]) for q in i]])
ST50=[st(rtube("m","hob","steel-enamel",0.4,0.28,0.5,0.008,0.0,0.031),"hob","member",id="member-housing"),
 st(ell("m","hob","steel-enamel",0.4,(0.001,0.001,0.001)),"hob","mass",id="x"),
]
ST50=[st(rtube("m","hob","steel-enamel",0.4,0.28,0.5,0.008,0.0,0.031),"hob","member",id="member-housing"),
 sref("member-plate","plate","hob","member"),
 sref("member-burner","burner-ring","burner","member"),
 sref("member-trivet-rail","trivet-rail","trivet","member"),sref("member-trivet-legs-front","trivet-legs-front","trivet","member"),sref("member-trivet-legs-back","trivet-legs-back","trivet","member"),
 st(cyl("m","knob","brass",0.45,0.0045,-0.259,-0.2,c=(0,0),axis="z"),"knob","member",id="member-knob-spindle"),
 st(cyl("m","flame","brass",0.45,0.0075,0.0,0.02,c=(0,0.03)),"flame","member",id="member-flame-injector"),
 sref("member-flame-ring","flame-ring","flame-body","member"),sref("member-flame-centre","flame-centre","flame-tip","member"),
 st(ell("j","knob","brass",0.45,(0.008,0.008,0.008),(0,0,-0.255)),"knob","joint",id="joint-knob"),
 st(ell("j","flame","brass",0.45,(0.0075,0.0075,0.0075),(0,0.005,0.03),role="detail"),"flame","joint",id="joint-flame"),
 st(ell("j","flame-body","brass",0.45,(0.0075,0.0075,0.0075),(0,0.0075,0.03),role="detail"),"flame-body","joint",id="joint-flame-body"),
 st(ell("j","flame-tip","brass",0.45,(0.0075,0.0075,0.0075),(0,0.0075,0.03),role="detail"),"flame-tip","joint",id="joint-flame-tip"),
 st(box("s","hob","steel-enamel",0.3,(0.28,0.008,0.5),(0,0.004,0)),"hob","support",id="support-base-contact"),
 st(ell("m","hob","steel-enamel",0.4,(0.08,0.012,0.15),(0,0.02,0),role="detail"),"hob","mass",id="mass-hob"),
 st(ell("m","trivet","cast-iron",0.3,(0.09,0.005,0.03),(0,0.043,0.03),role="detail"),"trivet","mass",id="mass-trivet")]
V50=vol(50,B,TH50,ST50,
 ["Body: the seed is kept, it already follows the drawings: a 31 mm sheet housing under a 10 mm enamel plate with rounded corners, the 12-tongue flame crown and the central flame as lofted cones, the two-part burner, the cast-iron pan support as a frame with four legs and the knob on the front face. Repairs: the burner ring is 7.5 mm high (was 7.0) and the vent strokes and knob pointer are declared texture sheets.",
  "Thermal: the plate region is the exposed top face of the enamel plate (all other faces are section faces), the knob its front and side, the burner ring with its cap, the flame crown and centre and the pan support as bodies; the flame-plume emitter is a 7.5 mm disc of the 0.1 m plume diameter over the flame tip whose two faces are openings, not surfaces.",
  "Structure: every rig bone is a member: the housing as a hollow sheet tube and the plate slab, the burner ring, the pan-support frame and legs, the knob spindle, the gas injector and the two flame bodies; joints at the knob and flame slides, a 0.28 x 0.5 m contact pad on the table, and mass ellipsoids for the housing and the pan support."],
 ["Körper: der Seed bleibt, er folgt schon den Zeichnungen: ein 31 mm hohes Blechgehäuse unter einer 10 mm starken Emailplatte mit gerundeten Ecken, der Flammenkranz mit 12 Zungen und die Mittelflamme als geloftete Kegel, der zweiteilige Brenner, der gusseiserne Topfträger als Rahmen mit vier Beinen und der Knebel an der Frontseite. Reparaturen: der Brennerring ist 7,5 mm hoch (vorher 7,0) und die Lüftungsstriche und der Knebelzeiger sind als Texturblätter erklärt.",
  "Thermik: die Region Platte ist die freiliegende Oberseite der Emailplatte (alle anderen Flächen sind Schnittflächen), der Knebel seine Front und Mantelfläche, Brennerring mit Kappe, Flammenkranz und Mittelflamme und der Topfträger als Körper; der Emitter Flammenfahne ist eine 7,5-mm-Scheibe mit dem 0,1 m breiten Fahnendurchmesser über der Flammenspitze, deren beide Flächen Öffnungen und keine Oberflächen sind.",
  "Tragwerk: jeder Rig-Knochen ist ein Stab: das Gehäuse als hohles Blechrohr und die Plattenscheibe, der Brennerring, Rahmen und Beine des Topfträgers, die Knebelspindel, die Gasdüse und die beiden Flammenkörper; Gelenke an Knebel und Flammenschiebern, ein 0,28 x 0,5 m großes Kontaktpolster auf dem Tisch und Masseellipsoide für Gehäuse und Topfträger."])
# ---- 1:100
CROWN=[(0,0.044),(0.008,0.042),(0.016,0.034),(0.024,0.022),(0.030,0.011),(0.034,0.001)]
B100=[rev("flame-seat","flame","flame",0.95,[(0,0.033),(0.015,0.033),(0.015,0.04),(0,0.04),(0,0.033)],origin=(0,0,0.03),role="detail"),box("plate","hob","steel-enamel",0.4,(0.3,0.04,0.52),(0,0.02,0)),
 rev("burner","burner","brass",0.5,[(0,0.052),(0.0155,0.0465),(0.018,0.02)],origin=(0,0,0.03)),
 rev("flame-crown","flame-body","flame",0.95,CROWN,origin=(0,0,0.03),role="detail"),
 ext("trivet-rail","trivet","cast-iron",0.3,[(-0.037,-0.136),(0.097,-0.136),(0.097,0.136),(-0.037,0.136)],"zx",0.034,0.05,holes=[[(-0.0295,-0.1285),(0.0895,-0.1285),(0.0895,0.1285),(-0.0295,0.1285)]]),
 box("trivet-legs-front","trivet","cast-iron",0.3,(0.016,0.04,0.016),(-0.12,0.02,-0.029),repeat={"count":2,"translation":[0.24,0,0]}),
 box("trivet-legs-back","trivet","cast-iron",0.3,(0.016,0.04,0.016),(-0.12,0.02,0.089),repeat={"count":2,"translation":[0.24,0,0]})]
TH100=[SEC(th(get(B100,"plate"),"plate",id="th-plate"),["-v","+u","-u","+w","-w"]),
 SEC(th(cyl("k","knob","bakelite",0.3,0.0095,-0.2735,-0.2575,axis="z",c=(0,0),n=12),"knob",id="th-knob"),["+w"]),
 SEC(th(get(B100,"burner"),"burner",id="th-burner"),["-v"]),SEC(th(get(B100,"flame-crown"),"flame",id="th-flame-crown"),["-v"]),
 ref("th-trivet-rail","trivet-rail","trivet"),ref("th-trivet-legs-front","trivet-legs-front","trivet"),ref("th-trivet-legs-back","trivet-legs-back","trivet"),
 SEC(th(cyl("p","flame-tip","flame",0.95,0.05,0.0103,0.0257,c=(0,0.03),role="detail",n=16),"flame",tags=["flame-plume"],id="th-flame-plume"),["+v","-v"])]
ST100=[st(rtube("m","hob","steel-enamel",0.4,0.28,0.5,0.015,0.0,0.031),"hob","member",id="member-housing"),
 sref("member-plate","plate","hob","member"),
 sref("member-burner","burner","burner","member"),sref("member-trivet-rail","trivet-rail","trivet","member"),sref("member-trivet-legs-front","trivet-legs-front","trivet","member"),sref("member-trivet-legs-back","trivet-legs-back","trivet","member"),
 st(cyl("m","knob","brass",0.45,0.0078,-0.259,-0.2,c=(0,0),axis="z"),"knob","member",id="member-knob-spindle"),
 st(cyl("m","flame","brass",0.45,0.0075,0.0,0.02,c=(0,0.03)),"flame","member",id="member-flame-injector"),
 sref("member-flame-ring","flame-crown","flame-body","member"),
 st(cyl("m","flame-tip","brass",0.45,0.0075,0.0,0.015,c=(0,0.03)),"flame-tip","member",id="member-flame-tip"),
 st(ell("j","knob","brass",0.45,(0.0075,0.0075,0.0075),(0,0,-0.255)),"knob","joint",id="joint-knob"),
 st(box("s","hob","steel-enamel",0.3,(0.28,0.015,0.5),(0,0.0075,0)),"hob","support",id="support-base-contact"),
 st(ell("m","hob","steel-enamel",0.4,(0.08,0.015,0.15),(0,0.02,0),role="detail"),"hob","mass",id="mass-hob"),
 st(ell("m","trivet","cast-iron",0.3,(0.09,0.0075,0.03),(0,0.043,0.03),role="detail"),"trivet","mass",id="mass-trivet")]
V100=vol(100,B100,TH100,ST100,
 ["1:100 draws the hob as six solids: one 40 mm enamel block for housing and plate, one revolve for ring and cap of the burner, one bell-shaped revolve for the whole flame crown, the pan-support frame with 15 mm bars and four 16 mm legs. The knob, vent strokes, central flame and plate lip are below 15 mm or not drawn and merge into the block.",
  "Thermal: re-derived at this scale: plate top face, a knob stub half sunk into the front, burner and flame crown as bodies without their buried bottoms, the pan support, the 15 mm plume disc.",
  "Structure: housing tube with 15 mm walls and plate slab, burner, pan-support frame and legs, a 15 mm knob spindle, injector and flame-tip stems, one joint, the contact pad and two masses; the flame joints are merged into the stems."],
 ["1:100 zeichnet das Kochfeld aus sechs Körpern: ein 40 mm hoher Emailleblock für Gehäuse und Platte, ein Rotationskörper für Ring und Kappe des Brenners, ein glockenförmiger Rotationskörper für den ganzen Flammenkranz, der Topfträgerrahmen mit 15-mm-Stegen und vier 16-mm-Beine. Knebel, Lüftungsstriche, Mittelflamme und Plattenkante sind kleiner als 15 mm oder nicht gezeichnet und gehen im Block auf.",
  "Thermik: für diesen Maßstab neu abgeleitet: Plattenoberseite, ein zur Hälfte in die Front versenkter Knebelstumpf, Brenner und Flammenkranz als Körper ohne ihre verdeckten Böden, der Topfträger, die 15-mm-Fahnenscheibe.",
  "Tragwerk: Gehäuserohr mit 15 mm Wand und Plattenscheibe, Brenner, Rahmen und Beine des Topfträgers, eine 15-mm-Knebelspindel, Düsen- und Flammenspitzenschaft, ein Gelenk, das Kontaktpolster und zwei Massen; die Flammengelenke gehen in die Schäfte auf."])
# ---- 1:200
B200=[rev("flame-seat","flame","flame",0.95,[(0,0.026),(0.03,0.026),(0.03,0.04),(0,0.04),(0,0.026)],origin=(0,0,0.03),role="detail"),box("plate","hob","steel-enamel",0.4,(0.3,0.04,0.52),(0,0.02,0)),
 rev("burner","burner","brass",0.5,[(0,0.056),(0.018,0.05),(0.03,0.03)],origin=(0,0,0.03)),
 rev("flame-crown","flame-body","flame",0.95,[(0,0.052),(0.012,0.05),(0.026,0.036),(0.036,0.016),(0.041,0.002)],origin=(0,0,0.03),role="detail"),
 ext("trivet","trivet","cast-iron",0.3,[(-0.039,-0.138),(0.099,-0.138),(0.099,0.138),(-0.039,0.138)],"zx",0.004,0.048,holes=[[(-0.029,-0.128),(0.089,-0.128),(0.089,0.128),(-0.029,0.128)]])]
TH200=[SEC(th(get(B200,"plate"),"plate",id="th-plate"),["-v","+u","-u","+w","-w"]),
 SEC(th(cyl("k","knob","bakelite",0.3,0.0159,-0.2665,-0.2365,axis="z",c=(0,0),n=16),"knob",id="th-knob"),["+w"]),
 SEC(th(get(B200,"burner"),"burner",id="th-burner"),["-v"]),SEC(th(rev("fc","flame-body","flame",0.95,[(0,0.045),(0.035,0.002)],origin=(0,0,0.03),role="detail"),"flame",id="th-flame-crown"),["-v"]),
 th(ext("r","trivet","cast-iron",0.3,[(-0.039,-0.138),(0.099,-0.138),(0.099,0.138),(-0.039,0.138)],"zx",0.018,0.048,holes=[[(-0.029,-0.128),(0.089,-0.128),(0.089,0.128),(-0.029,0.128)]]),"trivet",id="th-trivet"),
 SEC(th(cyl("p","flame-tip","flame",0.95,0.05,0.003,0.033,c=(0,0.03),role="detail"),"flame",tags=["flame-plume"],id="th-flame-plume"),["+v","-v"])]
ST200=[st(rtube("m","hob","steel-enamel",0.4,0.28,0.5,0.03,0.0,0.031),"hob","member",id="member-housing"),
 sref("member-plate","plate","hob","member"),sref("member-burner","burner","burner","member"),sref("member-trivet","trivet","trivet","member"),
 st(cyl("m","knob","brass",0.45,0.0155,-0.259,-0.2,c=(0,0),axis="z"),"knob","member",id="member-knob-spindle"),
 st(cyl("m","flame","brass",0.45,0.015,0.0,0.03,c=(0,0.03)),"flame","member",id="member-flame-injector"),
 sref("member-flame-body","flame-crown","flame-body","member"),
 st(cyl("m","flame-tip","brass",0.45,0.015,0.0,0.03,c=(0,0.03)),"flame-tip","member",id="member-flame-tip"),
 st(box("s","hob","steel-enamel",0.3,(0.28,0.03,0.5),(0,0.015,0)),"hob","support",id="support-base-contact"),
 st(ell("m","hob","steel-enamel",0.4,(0.08,0.02,0.15),(0,0.02,0),role="detail"),"hob","mass",id="mass-hob")]
V200=vol(200,B200,TH200,ST200,
 ["1:200 is four solids: the 40 mm plate block, a burner dome, a flame crown and the pan support merged into one 44 mm high cage frame whose walls stand where legs and rail were. Knob, vents and lip are below 30 mm.",
  "Thermal: plate top face, a 30 mm knob stub on the front, burner and crown, the cage frame and the 30 mm plume disc.",
  "Structure: the housing tube with 30 mm walls, plate, burner, cage frame, knob spindle, injector and flame-tip stems of 30 mm, the contact pad and one mass; joints are merged."],
 ["1:200 sind vier Körper: der 40 mm hohe Plattenblock, eine Brennerkuppel, ein Flammenkranz und der Topfträger, zu einem 44 mm hohen Käfigrahmen zusammengefasst, dessen Wände dort stehen, wo Beine und Rahmen waren. Knebel, Schlitze und Kante liegen unter 30 mm.",
  "Thermik: Plattenoberseite, ein 30-mm-Knebelstumpf an der Front, Brenner und Kranz, der Käfigrahmen und die 30-mm-Fahnenscheibe.",
  "Tragwerk: das Gehäuserohr mit 30 mm Wand, Platte, Brenner, Käfigrahmen, Knebelspindel, Düsen- und Flammenspitzenschaft von 30 mm, das Kontaktpolster und eine Masse; Gelenke sind zusammengefasst."])
write("gas-hob",{"50":V50,"100":V100,"200":V200,"500":unavail(500)})
