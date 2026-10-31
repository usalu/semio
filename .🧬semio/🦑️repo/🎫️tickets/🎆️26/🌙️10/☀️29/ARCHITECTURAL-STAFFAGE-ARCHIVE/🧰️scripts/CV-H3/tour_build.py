from common import *
A1=load_v1("tour-guide")['asset']
s=0.4276/0.4447
pose0=next(p for p in A1['poses'] if p['id']=="standing-pointing")
W0=fk(A1,pose0['bones'],pose_root(A1,pose0))
G=lambda i:next(x for x in A1['geometry']['elevation'] if x['id']==i)
P=dict(s=s,W=W0,torso=dict(region="torso-clothing",tone=0.42),pelvis=dict(region="leg-clothing",tone=0.24),
 sleeve=dict(region="arm-clothing",tone=0.42,x1=None),arm_skin="arm-skin",fsleeve=dict(region="arm-clothing",tone=0.42),hand="hand-skin",
 thigh_g=dict(region="leg-clothing",tone=0.24,x1=None),leg_skin="leg-skin",shank_g=dict(region="leg-clothing",tone=0.24),sock=None,
 shoe=dict(region="shoes",tone=0.2),head="head-skin",hair=dict(region="hair",tone=0.2),hair_d={100:G('cap')['d'],200:G('cap')['d']},
 reg_fig=["head-skin","hair","torso-clothing","leg-clothing"],reg_uarm=["arm-clothing"],reg_farm=["arm-clothing","arm-skin","hand-skin"],reg_thigh=["leg-clothing"],reg_shank=["leg-clothing"])
def extras(den):
    o=[]
    if den in(100,200):
        o.append(shp("wrist-skin-r","forearm-r","detail",6.1,False,0.8,L_([(0.2486,0.0266),(0.2486,-0.0246)]),region="arm-skin"))
        o.append(shp("badge","torso","body",3.55,True,0.95,R_(0.215,-0.14,0.085,0.056,0.004),region="torso-clothing"))
    if den==100:
        for i in('jacket-hem','jacket-zip','lanyard'): o.append(shp(i,"torso","detail",3.3 if i!='lanyard' else 3.5,False,0.2,L_(G(i)['points']),region="torso-clothing"))
        o.append(shp("cap-seam","head","detail",5.75,False,0.6,L_(G('cap-seam')['points']),region="hair"))
    # accessories per scale
    fl=G('flag')['d']
    if den==500:
        o+=[shp("pole","flag-pole","body",6.3,False,0.2,L_([(-0.3,0),(1.05,0)]),region="pole-metal"),
            shp("flag","flag","body",6.2,True,0.3,P_(fl),region="flag-cloth"),
            shp("pointer","pointer","body",6.4,False,0.15,L_([(0,0),(0.55,0)]),region="pointer-metal")]
    elif den==200:
        o+=[shp("pole","flag-pole","body",6.3,False,0.2,L_([(-0.3,0),(1.05,0)]),region="pole-metal",penMm=0.13),
            shp("flag","flag","body",6.2,True,0.78,P_(fl),region="flag-cloth"),
            shp("pointer","pointer","body",6.4,False,0.15,L_([(0,0),(0.55,0)]),region="pointer-metal",penMm=0.13)]
    elif den==100:
        o+=[shp("pole","flag-pole","body",6.3,True,0.2,C_(-0.3,0,1.05,0,0.009,0.009),region="pole-metal"),
            shp("pole-finial","flag-pole","detail",6.35,True,0.8,E_(1.06,0,0.016,0.016),region="pole-metal"),
            shp("flag","flag","body",6.2,True,0.78,P_(fl),region="flag-cloth"),
            shp("flag-emblem","flag","detail",6.25,False,0.2,E_(0.22,0.14,0.07,0.07),region="flag-cloth"),
            shp("pointer","pointer","body",6.4,True,0.15,C_(0,0,0.55,0,0.005,0.003),region="pointer-metal"),
            shp("pointer-tip","pointer","detail",6.45,True,0.35,C_(0.5,0,0.55,0,0.0065,0.004),region="pointer-metal")]
    return o
N=copy.deepcopy(A1); N.pop('geometry')
N['summary']=T("A 1.75 m tour guide in a softshell jacket over a shirt, cap and lanyard who holds a flag pole, points with a telescopic pointer, gestures to the group and waves; speech breathing through the mouth (also loud, projecting speech), sun and wind on the jacket and flag; four individually drawn scales.",
 "Eine 1,75 m große Stadtführerin bzw. ein Stadtführer in Softshelljacke über einem Hemd, mit Mütze und Umhängeband, die oder der eine Fahnenstange hält, mit einem Teleskopzeiger zeigt, zur Gruppe gestikuliert und winkt; Sprechatmung durch den Mund (auch laut, mit projizierter Stimme), Sonne und Wind auf Jacke und Fahne; vier einzeln gezeichnete Maßstäbe.")
N['provenance']['references']+=refs("gagge-1971","gupta-2010","hoeppe-1981","winkworth-1995","mifflin-1990","incropera-2011")
N['provenance']['author']="Semio staffage content worker CV-H3"
N['provenance']['notes']=T("Converted to semio.staffage/v2: the v1 outlines are raw material of the 1:50 drawing; 1:100, 1:200 and 1:500 are new composed forms; the jacket is modelled as shirt, trapped air and a windproof softshell.",
 "Auf semio.staffage/v2 umgestellt: Die v1-Umrisse sind Rohmaterial der 1:50-Zeichnung; 1:100, 1:200 und 1:500 sind neu komponierte Formen; die Jacke ist als Hemd, eingeschlossene Luft und winddichtes Softshell modelliert.")
N['materials'].append({"id":"softshell","title":T("Softshell jacket fabric","Softshell-Jackenstoff"),"emissivity":0.92,"solarAbsorptivity":0.6,"source":"authored-estimate"})
N['anchors']=anchors_head(N['anchors'],0.2685)
regs=[]
regs.append(region("head-skin","Face and neck skin","Haut von Gesicht und Hals","skin","skin",0.085,0.15,"sphere",0.55,[],15000,"head",0.05,0.045,ref="dubois-1916",source="literature"))
regs.append(region("hair","Hair under the cap","Haar unter der Mütze","hair","cap-fabric",0.068,0.15,"sphere",0.6,[layer("hair-under","Hair","Haar","hair","hair",res=0.04,thick=0.008,cap=250,ref="iso-9920-2007"),layer("cap","Cotton cap (0.05 clo)","Baumwollmütze (0,05 clo)","clothing","cap-fabric",clo=0.05,thick=0.0015,cap=250,ref="iso-9920-2007")],4000,"head",0.01,0.04,ref="iso-9920-2007"))
regs.append(region("torso-clothing","Softshell jacket over a shirt","Softshelljacke über einem Hemd","clothing","softshell",0.91,0.3,"cylinder",0.45,
  [layer("shirt-under","Long-sleeve shirt (0.25 clo)","Langarmhemd (0,25 clo)","clothing","cotton-clothing",clo=0.25,thick=0.001,cap=200,ref="iso-9920-2007"),airgap("jacket-air","Air between shirt and jacket","Luft zwischen Hemd und Jacke",0.03,"softshell"),layer("jacket","Windproof softshell jacket (0.3 clo)","Winddichte Softshelljacke (0,3 clo)","clothing","softshell",clo=0.3,thick=0.002,cap=350,vap=12,ref="iso-9920-2007")],21000,"trunk",0.40,0.375,ref="iso-9920-2007"))
regs.append(region("arm-clothing","Jacket sleeves over shirt sleeves","Jackenärmel über Hemdärmeln","clothing","softshell",0.316,0.08,"cylinder",0.45,
  [layer("shirt-sleeve","Shirt sleeve (0.25 clo)","Hemdärmel (0,25 clo)","clothing","cotton-clothing",clo=0.25,thick=0.001,cap=200,ref="iso-9920-2007"),airgap("sleeve-air","Air in the sleeve","Luft im Ärmel",0.025,"softshell"),layer("jacket-sleeve","Softshell sleeve (0.3 clo)","Softshell-Ärmel (0,3 clo)","clothing","softshell",clo=0.3,thick=0.002,cap=350,vap=12,ref="iso-9920-2007")],17000,"arm",0.12,0.128,ref="iso-9920-2007"))
regs.append(region("arm-skin","Wrists","Handgelenke","skin","skin",0.004,0.05,"cylinder",0.45,[],10000,"arm",0.0,0.002,ref="dubois-1916",source="literature"))
regs.append(region("hand-skin","Hands","Hände","skin","skin",0.097,0.07,"cylinder",0.45,[],10000,"hand",0.03,0.05,ref="dubois-1916",source="literature"))
regs.append(region("leg-clothing","Chino trousers","Chinohose","clothing","cotton-clothing",0.67,0.14,"cylinder",0.35,
  [airgap("trouser-air","Air inside the trousers","Luft in der Hose",0.02,"cotton-clothing"),layer("trousers","Cotton trousers (0.24 clo)","Baumwollhose (0,24 clo)","clothing","cotton-clothing",clo=0.24,thick=0.0012,cap=260,ref="iso-9920-2007")],18000,"leg",0.37,0.30,ref="iso-9920-2007"))
regs.append(region("shoes","Leather walking shoes with socks","Lederwanderschuhe mit Socken","clothing","leather-shoe",0.135,0.12,"horizontal-plate-up",0.25,
  [layer("sock-layer","Cotton sock (0.05 clo)","Baumwollsocke (0,05 clo)","clothing","cotton-clothing",clo=0.05,thick=0.003,cap=300,ref="iso-9920-2007"),airgap("shoe-air","Air in the shoe","Luft im Schuh",0.03,"leather-shoe",thick=0.008),layer("shoe-upper","Leather upper","Lederschaft","shell","leather-shoe",res=0.03,thick=0.003,cap=1200,ref="iso-9920-2007")],7000,"foot",0.02,0.06,ref="iso-9920-2007"))
regs.append(region("pointer-metal","Telescopic aluminium pointer","Teleskopzeiger aus Aluminium","surface","pointer-aluminium",0.01,0.02,"cylinder",0.5,[],2500,model="passive"))
regs.append(region("pole-metal","Aluminium flag pole","Fahnenstange aus Aluminium","surface","pointer-aluminium",0.076,0.02,"cylinder",0.5,[],3000,model="passive"))
regs.append(region("flag-cloth","Flag cloth","Fahnentuch","surface","flag-textile",0.26,0.3,"vertical-plate",0.5,[],500,model="passive"))
pat=[breath_pattern("rest","Calm standing breathing","Ruhige Atmung im Stand",14,0.5,0.38,0.04,0.45,0.13,"nose","gupta-2010"),
     breath_pattern("talk","Speaking to the group","Sprechen zur Gruppe",11,0.9,0.10,0.02,0.80,0.08,"mouth","winkworth-1995"),
     breath_pattern("talk-loud","Loud, projected speech","Laute, getragene Sprache",10,1.3,0.12,0.0,0.83,0.05,"mouth","winkworth-1995")]
N['thermal']={"environment":human_env(16,17,0.55,1.5,250,0.35,sky=8,ground=20),"baseMet":1.3,"body":physiology(78.59,175),"respiration":respiration(pat,32.5,0.95),"operating":[],"regions":regs,
 "emitters":breath_emitters(0.2685,s)+[plume_emitter("torso-clothing",0.38)],
 "assumptions":[
  T("DuBois skin area for 1.75 m and 78.59 kg; layered clothing of long-sleeve shirt (0.25 clo), trapped air and a windproof softshell (0.3 clo) on the trunk and arms, cotton trousers (0.24 clo), after ISO 9920 with 1 clo = 0.155 m2K/W.","DuBois-Hautfläche für 1,75 m und 78,59 kg; Lagenkleidung aus Langarmhemd (0,25 clo), eingeschlossener Luft und winddichter Softshell (0,3 clo) an Rumpf und Armen, Baumwollhose (0,24 clo), nach ISO 9920 mit 1 clo = 0,155 m2K/W."),
  T("Standing guide at 1.3 to 1.8 met (ISO 8996 class 1 with active arms), 16 degrees Celsius, 55 % humidity, 1.5 m/s wind and 250 W/m2 direct sun: sunlit areas receive the full irradiance, shaded areas the diffuse share; flag, pole and pointer are exposed fully.","Stehender Führer mit 1,3 bis 1,8 met (ISO 8996 Klasse 1 mit bewegten Armen), 16 Grad Celsius, 55 % Feuchte, 1,5 m/s Wind und 250 W/m2 direkter Sonne: Besonnte Flächen erhalten die volle Einstrahlung, beschattete den diffusen Anteil; Fahne, Stange und Zeiger sind voll exponiert."),
  T("Speech breathing is oral: 11 breaths per minute with 0.9 L and one second of inspiration per cycle (long, controlled expiration over phrases of several seconds, after Winkworth et al.); loud, projected speech uses 10 breaths per minute with 1.3 L. Exhaled air 32.5 degrees Celsius at 95 % humidity (Hoeppe: lower in cool air). The numbers are authored estimates guided by the cited sources.","Die Sprechatmung erfolgt über den Mund: 11 Atemzüge pro Minute mit 0,9 L und kurzer Einatmung je Zyklus (lange, kontrollierte Ausatmung über Phrasen von mehreren Sekunden, nach Winkworth et al.); laute, getragene Sprache nutzt 10 Atemzüge pro Minute mit 1,3 L. Ausatemluft 32,5 Grad Celsius bei 95 % Feuchte (Hoeppe: in kühler Luft niedriger). Die Zahlen sind Autorenschätzungen nach den genannten Quellen."),
  T("Flag cloth, pole and pointer are passive surfaces without internal heat; the aluminium parts heat strongly in sun because of their low emissivity and thin section.","Fahnentuch, Stange und Zeiger sind passive Flächen ohne innere Wärme; die Aluminiumteile erwärmen sich in der Sonne wegen ihres niedrigen Emissionsgrads und des dünnen Querschnitts stark.")]}
RG=regs; EM=["plume"]
def reg_sun(sun_body=0.5,flag=1.0):
    r={"head-skin":{"sunlitFraction":sun_body},"hair":{"sunlitFraction":sun_body},"torso-clothing":{"sunlitFraction":sun_body},"arm-clothing":{"sunlitFraction":sun_body},"arm-skin":{"sunlitFraction":sun_body},"hand-skin":{"sunlitFraction":sun_body},
       "leg-clothing":{"sunlitFraction":sun_body},"shoes":{"sunlitFraction":0.6,"contact":{"fraction":0.5,"tempC":20}},"pointer-metal":{"sunlitFraction":flag},"pole-metal":{"sunlitFraction":flag},"flag-cloth":{"sunlitFraction":flag}}
    return r
spec={"standing-pointing":(1.5,"talk",0.5,0.8),"gesturing-to-group":(1.4,"talk",0.5,0.8),"holding-flag":(1.3,"rest",0.5,0.6),"waving":(1.7,"talk-loud",0.5,0.9),
      "wave-swing-a":(1.7,"talk-loud",0.5,0.9),"wave-swing-b":(1.7,"talk-loud",0.5,0.9),"turning-to-group":(1.5,"talk",0.4,0.8)}
for p in N['poses']:
    met,br,sun,pl=spec[p['id']]
    reg=reg_sun(sun)
    if p['id']=="turning-to-group":
        for k in("head-skin","hair","torso-clothing","arm-clothing","hand-skin","leg-clothing"): reg[k]={"sunlitFraction":0.3}
    bind_pose(p,RG,EM,met,br,reg=reg,extra_emit={"plume":pl})
for c in N['clips']:
    c['events']=[e for e in c['events'] if e['kind'] not in('inhale','exhale')]
    c['tracks']=[t for t in c['tracks'] if t['target']!="emitter:breath"]
    if c['id']=="gesture-point":
        c['tracks']+=[{"target":"breathing","ease":"step","keys":[{"t":0,"v":0},{"t":0.8,"v":1},{"t":6.4,"v":0}]}]
        c['events'].append({"t":0.8,"kind":"thermal","title":T("Short mouth inspiration before speaking, then a long controlled expiration while pointing","Kurze Mundeinatmung vor dem Sprechen, dann lange kontrollierte Ausatmung beim Zeigen")})
    if c['id']=="turn-and-wave":
        c['tracks']+=[{"target":"breathing","ease":"step","keys":[{"t":0,"v":0},{"t":1.2,"v":1},{"t":2.2,"v":2},{"t":5.2,"v":0}]},{"target":"region:torso-clothing:sunlitFraction","ease":"smooth","keys":[{"t":0,"v":0.5},{"t":1.2,"v":0.3},{"t":2.2,"v":0.5}]}]
        c['events'].append({"t":2.2,"kind":"thermal","title":T("Calling to the group: loud, projected speech","Ruf zur Gruppe: laute, getragene Sprache")})
    c['events'].sort(key=lambda e:e['t'])
N['presentation']['defaultScale']=100; N['presentation']['viewpoints']=["elevation"]; N['completion']="authored"
def mk(den):
    if den==500:
        b=merged500(P,pose0['bones'],None)
        return b+extras(500)
    return build_body(den,P)+extras(den)
R={500:mk(500),200:mk(200),100:mk(100)}
drop={"torso","pelvis"}|{f"{p}-{sd}" for p in("upper-arm","forearm","thigh","shank") for sd in "lr"}
v=[{k:q for k,q in sh.items() if k!='lod'} for sh in A1['geometry']['elevation'] if sh['id'] not in drop]
for sh in v:
    if sh['id'] in('pole','pole-finial'): sh['region']='pole-metal'; sh['material']='pointer-aluminium'
kit=[sh for sh in build_body(100,P) if sh['id'] in {"torso","pelvis"}|{f"{p}-{sd}" for p in("upper-arm","forearm","thigh","shank") for sd in "lr"}]
R[50]=v+kit
N['representations']={
 "500":rep(500,R[500],DEC(
  silhouette=("A solid dark standing figure of thirteen articulated overlapping forms with a cap, the raised near arm with pointer line and the tall flag pole line with a small light flag; the pole defines the overall 2.28 m height.","Eine flächig dunkle stehende Figur aus dreizehn gelenkig überlappenden Formen mit Mütze, dem erhobenen nahen Arm mit Zeigerlinie und der hohen Fahnenstangenlinie mit kleiner heller Fahne; die Stange bestimmt die Gesamthöhe von 2,28 m."),
  retained=("The cap, the pointing arm raised 40 degrees, the pole at its real length and the flag lens (0.46 by 0.28 m) so the guide reads as a flag-bearing guide even at 3.6 mm height.","Die Mütze, der um 40 Grad erhobene Zeigearm, die Stange in echter Länge und die Fahnenlinse (0,46 mal 0,28 m), damit die Führung auch bei 3,6 mm Höhe als Fahnenträger lesbar ist."),
  merged=("Head, cap, jacket, hip and trousers form one body; sleeve, wrist and hand one forearm; trouser leg and shoe remain separate shank and shoe forms.","Kopf, Mütze, Jacke, Hüfte und Hose bilden einen Körper; Ärmel, Handgelenk und Hand einen Unterarm; Hosenbein und Schuh bleiben getrennte Unterschenkel- und Schuhformen."),
  omitted=("Cap peak (7 cm), face, badge, lanyard, zip, hem, emblem, finial, pointer tip and fingers.","Mützenschirm (7 cm), Gesicht, Namensschild, Umhängeband, Reißverschluss, Saum, Emblem, Stangenknauf, Zeigerspitze und Finger."),
  thinElements=("Pole (1.8 cm) and pointer (1 cm) are single 0.2 mm and 0.15 mm lines, i.e. 10 cm and 7.5 cm real: the only thickened parts, kept because without them the flag and the gesture vanish.","Stange (1,8 cm) und Zeiger (1 cm) sind einzelne 0,2-mm- und 0,15-mm-Linien, also 10 cm und 7,5 cm real: die einzigen verdickten Teile, beibehalten, weil sonst Fahne und Geste verschwinden."),
  hatching=("None.","Keine."),
  styles=("Solid black body (tone 0), flag light (0.3) against the dark pole; 0.15 mm outline.","Flächig schwarzer Körper (Ton 0), Fahne hell (0,3) gegen die dunkle Stange; 0,15-mm-Kontur."),
  overlays=("Body forms mix jacket, face, cap and trousers; the pole, pointer and flag keep their own passive regions, so the sun-heated aluminium and cloth show above the figure.","Körperformen mischen Jacke, Gesicht, Mütze und Hose; Stange, Zeiger und Fahne behalten eigene passive Regionen, das sonnenerwärmte Aluminium und Tuch zeigen sich also über der Figur."))),
 "200":rep(200,R[200],DEC(
  silhouette=("Cap-wearing head with the peak, jacket trunk with the small badge, long sleeves, trousers and shoes; the left hand holds a flag pole drawn as one line with a flag shape, the right arm holds the pointer line.","Kopf mit Mütze und Schirm, Jackenrumpf mit kleinem Namensschild, lange Ärmel, Hose und Schuhe; die linke Hand hält eine als Linie gezeichnete Fahnenstange mit Fahnenform, der rechte Arm den Zeigerstrich."),
  retained=("Jacket, sleeves, trousers, shoes, cap with peak, badge, wrist line, flag, pole and pointer, each as its own mass or line.","Jacke, Ärmel, Hose, Schuhe, Mütze mit Schirm, Namensschild, Handgelenkslinie, Fahne, Stange und Zeiger, jeweils als eigene Masse oder Linie."),
  merged=("Hand and forearm are one sleeve segment; cap and hair are one dark mass.","Hand und Unterarm sind ein Ärmelsegment; Mütze und Haar sind eine dunkle Masse."),
  omitted=("Lanyard, zip, hem, cap seam, emblem, stripe, finial, pointer tip, face details and fingers.","Umhängeband, Reißverschluss, Saum, Mützennaht, Emblem, Streifen, Stangenknauf, Zeigerspitze, Gesichtsdetails und Finger."),
  thinElements=("Pole and pointer are 0.13 mm strokes (2.6 cm real) instead of 1.8 cm and 1 cm capsules; badge (8.5 cm) remains a small filled form.","Stange und Zeiger sind 0,13-mm-Striche (2,6 cm real) statt Kapseln von 1,8 cm und 1 cm; das Namensschild (8,5 cm) bleibt eine kleine gefüllte Form."),
  hatching=("None.","Keine."),
  styles=("0.18 mm outlines, 0.13 mm lines; jacket mid-dark (0.42), trousers dark (0.24), flag light (0.78).","0,18-mm-Konturen, 0,13-mm-Linien; Jacke mitteldunkel (0,42), Hose dunkel (0,24), Fahne hell (0,78)."),
  overlays=("Jacket, sleeve, trousers, shoe, face, cap, hand, wrist, pole, pointer and flag are separate shapes so that layered clothing zones, sun-lit and shaded sides and the wind-cooled flag are distinct.","Jacke, Ärmel, Hose, Schuh, Gesicht, Mütze, Hand, Handgelenk, Stange, Zeiger und Fahne sind getrennte Formen, damit Bekleidungszonen, besonnte und beschattete Seiten und die windgekühlte Fahne unterscheidbar sind."))),
 "100":rep(100,R[100],DEC(
  silhouette=("Jacket with hem, zip, lanyard and badge, cap with peak and seam, long sleeves with wrist line, mitten hands with thumb, trousers and shoes; flag with emblem on a capsule pole with finial and a telescopic pointer with tip.","Jacke mit Saum, Reißverschluss, Umhängeband und Namensschild, Mütze mit Schirm und Naht, lange Ärmel mit Handgelenkslinie, Fäustlingshände mit Daumen, Hose und Schuhe; Fahne mit Emblem an einer Kapselstange mit Knauf und Teleskopzeiger mit Spitze."),
  retained=("Every accessory construction part: pole capsule, finial circle, flag lens and emblem, pointer capsule and tip.","Jedes Konstruktionsteil des Zubehörs: Stangenkapsel, Knaufkreis, Fahnenlinse und Emblem, Zeigerkapsel und -spitze."),
  merged=("Neck and head share an outline; the flag stripe of the 1:50 drawing is merged into the emblem circle.","Hals und Kopf teilen eine Kontur; der Fahnenstreifen der 1:50-Zeichnung geht im Emblemkreis auf."),
  omitted=("Eye, ear, mouth, hair strands, fingers, laces, soles, flag stripe.","Auge, Ohr, Mund, Haarsträhnen, Finger, Schnürung, Sohlen, Fahnenstreifen."),
  thinElements=("Pole (0.18 mm) and pointer (0.1 mm) stay true-width capsules; the pointer is thinner than the pen, so its outline is its width.","Stange (0,18 mm) und Zeiger (0,1 mm) bleiben Kapseln in echter Breite; der Zeiger ist dünner als die Feder, seine Kontur ist also seine Breite."),
  hatching=("None.","Keine."),
  styles=("0.2 mm outlines, 0.13 mm details and seams; jacket mid-dark, trousers and shoes dark, flag light.","0,2-mm-Konturen, 0,13-mm-Details und -Nähte; Jacke mitteldunkel, Hose und Schuhe dunkel, Fahne hell."),
  overlays=("Jacket and sleeve forms carry the layered clothing regions, the head its face and cap regions, accessories their own passive regions.","Jacken- und Ärmelformen tragen die Lagenkleidungsregionen, der Kopf seine Gesichts- und Mützenregion, das Zubehör eigene passive Regionen."))),
 "50":rep(50,R[50],DEC(
  silhouette=("Full drawing with face profile, cap seam, jacket hem, zip, lanyard and badge, finger lines, laces, soles; flag with emblem and stripe, pole with finial, pointer with tip.","Volle Zeichnung mit Gesichtsprofil, Mützennaht, Jackensaum, Reißverschluss, Umhängeband und Namensschild, Fingerlinien, Schnürung, Sohlen; Fahne mit Emblem und Streifen, Stange mit Knauf, Zeiger mit Spitze."),
  retained=("All v1 detail; the smooth, round-capped limb bodies replace the square-ended v1 limbs.","Alle v1-Details; die glatten, gerundeten Gliedmaßen ersetzen die v1-Glieder mit eckigen Enden."),
  merged=("Sleeve and forearm are one shape; badge sits on the jacket outline.","Ärmel und Unterarm sind eine Form; das Namensschild sitzt auf der Jackenkontur."),
  omitted=("Fabric folds, pointer segments, flag hem stitches.","Stofffalten, Zeigersegmente, Fahnensaumnähte."),
  thinElements=("Pointer capsule 1 cm (0.2 mm), pole 1.8 cm (0.36 mm), stripes and finger lines 0.13 mm.","Zeigerkapsel 1 cm (0,2 mm), Stange 1,8 cm (0,36 mm), Streifen und Fingerlinien 0,13 mm."),
  hatching=("None.","Keine."),
  styles=("0.25 mm silhouettes, 0.18 mm seams, 0.13 mm textures; jacket mid-dark, flag light.","0,25-mm-Silhouetten, 0,18-mm-Nähte, 0,13-mm-Texturen; Jacke mitteldunkel, Fahne hell."),
  overlays=("Each region is its own shape: jacket, sleeves, trousers, shoes, face, cap, hands, wrist, pole, pointer and flag.","Jede Region ist eine eigene Form: Jacke, Ärmel, Hose, Schuhe, Gesicht, Mütze, Hände, Handgelenk, Stange, Zeiger und Fahne.")))}
N['symbols']=[]
N['exports']=A1['exports']
if __name__=="__main__":
    finish(N,out_path("tour-guide")); print("written")
