from common import *
A1=load_v1("leaning-person")['asset']
s=0.4202/0.4447
pose0=next(p for p in A1['poses'] if p['id']=="leaning-shoulder-wall")
W0=fk(A1,pose0['bones'],pose_root(A1,pose0))
G=lambda i:next(x for x in A1['geometry']['elevation'] if x['id']==i)
P=dict(s=s,W=W0,torso=dict(region="torso-clothing",tone=0.34),pelvis=dict(region="leg-clothing",tone=0.3),
 sleeve=dict(region="arm-clothing",tone=0.34,x1=None),arm_skin="arm-skin",fsleeve=dict(region="arm-clothing",tone=0.34),hand="hand-skin",
 thigh_g=dict(region="leg-clothing",tone=0.3,x1=None),leg_skin="leg-skin",shank_g=dict(region="leg-clothing",tone=0.3),sock=None,
 shoe=dict(region="shoes",tone=0.9),head="head-skin",hair=dict(region="hair",tone=0.3),hair_d={100:G('beanie')['d'],200:G('beanie')['d']},
 reg_fig=["head-skin","hair","torso-clothing","leg-clothing"],reg_uarm=["arm-clothing"],reg_farm=["arm-clothing","arm-skin","hand-skin"],reg_farm_r=["arm-clothing","arm-skin","hand-skin","phone-surface"],reg_thigh=["leg-clothing"],reg_shank=["leg-clothing"])
WALL=[shp("wall-surface","wall","ground",0.1,False,0.6,L_([(0,0),(0,1.9)]),region="wall-face",material="masonry")]
COURSES=shp("wall-courses","wall","ground",0.2,False,0.5,P_(G('wall-courses')['d']),region="wall-face",material="masonry",penMm=0.13)
def extras(den):
    o=[]
    if den in(100,200):
        o.append(shp("hoodie-hood","head","body",4.4,True,0.38,P_(G('hoodie-hood')['d']),region="torso-clothing"))
        o.append(shp("wrist-skin-r","forearm-r","detail",6.1,False,0.8,L_([(0.2443,0.0249),(0.2443,-0.023)]),region="arm-skin"))
        o.append(shp("phone","hand-r","body",6.3,True,0.12,R_(0.025,-0.045,0.125,0.032,0.008),region="phone-surface",material="phone-glass"))
    if den==100:
        o.append(shp("beanie-rim","head","detail",5.75,False,0.2,L_(G('beanie-rim')['points']),region="hair"))
        o.append(shp("hoodie-pocket","torso","detail",3.3,False,0.2,L_(G('hoodie-pocket')['points']),region="torso-clothing"))
        o.append(shp("hoodie-hem","torso","detail",3.3,False,0.2,L_(G('hoodie-hem')['points']),region="torso-clothing"))
        o.append(shp("phone-screen","hand-r","glazing",6.35,True,0.4,R_(0.032,-0.04,0.105,0.023,0.005),region="phone-surface",material="phone-glass"))
        o.append(COURSES)
    return o+copy.deepcopy(WALL)
N=copy.deepcopy(A1); N.pop('geometry')
N['dimensions'].update({'heightM':1.73,'widthM':0.53})
N['summary']=T("A 1.72 m person in hoodie, jeans and beanie leaning with the shoulder blades against a masonry wall while looking at or talking on a phone: wall reaction and friction at the feet, heat conduction into the wall, speech breathing and a push-off to free standing; four individually drawn scales.",
 "Eine 1,72 m große Person in Kapuzenpullover, Jeans und Mütze, die mit den Schulterblättern an einer gemauerten Wand lehnt und aufs Telefon schaut oder telefoniert: Wandreaktion und Reibung an den Füßen, Wärmeleitung in die Wand, Sprechatmung und Abstoßen zum freien Stehen; vier einzeln gezeichnete Maßstäbe.")
N['provenance']['references']+=refs("gagge-1971","gupta-2010","hoeppe-1981","winkworth-1995","mifflin-1990","incropera-2011")
N['provenance']['references'].append({"id":"din-4108-4","citation":"DIN 4108-4 (2020) Wärmeschutz und Energie-Einsparung in Gebäuden, Teil 4: Wärme- und feuchteschutztechnische Bemessungswerte (conductivity of brick masonry about 0.8 W/(m K), plaster about 0.87 W/(m K))."})
N['provenance']['author']="Semio staffage content worker CV-H3"
N['provenance']['notes']=T("Converted to semio.staffage/v2: the v1 outlines are raw material of the 1:50 drawing; 1:100, 1:200 and 1:500 are new composed forms. The wall stays a ground-role line and texture so the bounds are the person's.",
 "Auf semio.staffage/v2 umgestellt: Die v1-Umrisse sind Rohmaterial der 1:50-Zeichnung; 1:100, 1:200 und 1:500 sind neu komponierte Formen. Die Wand bleibt eine Linie und Textur der Rolle Boden, damit die Maße die der Person sind.")
N['anchors']=anchors_head(N['anchors'],0.2639)
regs=[]
regs.append(region("head-skin","Face skin","Gesichtshaut","skin","skin",0.07,0.15,"sphere",0.5,[],15000,"head",0.05,0.04,ref="dubois-1916",source="literature"))
regs.append(region("hair","Wool beanie over hair","Wollmütze über dem Haar","hair","wool-hat",0.071,0.15,"sphere",0.6,[layer("hair-under","Hair","Haar","hair","hair",res=0.04,thick=0.008,cap=250,ref="iso-9920-2007"),layer("beanie","Knitted wool beanie (0.2 clo)","Gestrickte Wollmütze (0,2 clo)","clothing","wool-hat",clo=0.2,thick=0.004,cap=500,ref="iso-9920-2007")],4500,"head",0.01,0.04,ref="iso-9920-2007"))
regs.append(region("torso-clothing","Cotton hoodie with hood","Baumwoll-Kapuzenpullover mit Kapuze","clothing","cotton-clothing",0.80,0.3,"cylinder",0.4,
  [airgap("torso-air","Air between skin and hoodie","Luft zwischen Haut und Pullover",0.03,"cotton-clothing"),layer("hoodie","Cotton fleece hoodie (0.35 clo)","Baumwoll-Fleece-Pullover (0,35 clo)","clothing","cotton-clothing",clo=0.35,thick=0.004,cap=450,ref="iso-9920-2007")],22000,"trunk",0.40,0.38,ref="iso-9920-2007"))
regs.append(region("arm-clothing","Hoodie sleeves","Pulloverärmel","clothing","cotton-clothing",0.27,0.08,"cylinder",0.45,
  [airgap("sleeve-air","Air under the sleeve","Luft unter dem Ärmel",0.025,"cotton-clothing"),layer("sleeve","Cotton fleece sleeve (0.35 clo)","Baumwoll-Fleece-Ärmel (0,35 clo)","clothing","cotton-clothing",clo=0.35,thick=0.004,cap=450,ref="iso-9920-2007")],18000,"arm",0.13,0.125,ref="iso-9920-2007"))
regs.append(region("arm-skin","Wrists","Handgelenke","skin","skin",0.009,0.05,"cylinder",0.4,[],10000,"arm",0.0,0.005,ref="dubois-1916",source="literature"))
regs.append(region("hand-skin","Hands","Hände","skin","skin",0.089,0.07,"cylinder",0.4,[],10000,"hand",0.03,0.05,ref="dubois-1916",source="literature"))
regs.append(region("leg-clothing","Jeans","Jeans","clothing","cotton-clothing",0.61,0.14,"cylinder",0.35,
  [airgap("jeans-air","Air inside the jeans","Luft in der Jeans",0.02,"cotton-clothing"),layer("denim","Denim trousers (0.25 clo)","Jeansstoff (0,25 clo)","clothing","cotton-clothing",clo=0.25,thick=0.0012,cap=700,ref="iso-9920-2007")],20000,"leg",0.36,0.30,ref="iso-9920-2007"))
regs.append(region("shoes","Sneakers with socks","Sneaker mit Socken","clothing","leather-shoe",0.125,0.12,"horizontal-plate-up",0.2,
  [layer("sock-layer","Cotton sock (0.04 clo)","Baumwollsocke (0,04 clo)","clothing","cotton-clothing",clo=0.04,thick=0.002,cap=300,ref="iso-9920-2007"),airgap("shoe-air","Air in the shoe","Luft im Schuh",0.035,"leather-shoe",thick=0.009),layer("shoe-upper","Canvas and rubber upper","Obermaterial aus Leinen und Gummi","shell","leather-shoe",res=0.03,thick=0.004,cap=1000,ref="iso-9920-2007")],6500,"foot",0.02,0.06,ref="iso-9920-2007"))
regs.append(region("phone-surface","Smartphone glass and case","Smartphone-Glas und Hülle","surface","phone-glass",0.02,0.15,"vertical-plate",0.4,[],900,model="operating",operating={"key":"phone","heatW":1.5},source="authored-estimate"))
regs.append(region("wall-face","Masonry wall face (heated building behind)","Mauerwerksfläche (beheiztes Gebäude dahinter)","wall","masonry",3.5,1.9,"vertical-plate",0.5,
  [layer("plaster","Lime plaster 15 mm","Kalkputz 15 mm","masonry","masonry",res=round(0.015/0.87,4),thick=0.015,cap=25000,vap=False,ref="din-4108-4",src="literature"),
   layer("brick","Brick masonry 365 mm","Ziegelmauerwerk 365 mm","masonry","masonry",res=round(0.365/0.8,4),thick=0.365,cap=120000,vap=False,ref="din-4108-4",src="literature")],150000,model="conditioned",conditioned={"interiorTempC":20,"interiorFilmM2KW":0.13},ref="din-4108-4",source="literature"))
pat=[breath_pattern("rest","Calm standing breathing","Ruhige Atmung im Stand",14,0.5,0.38,0.04,0.45,0.13,"nose","gupta-2010"),
     breath_pattern("phone-talk","Speaking on the phone","Sprechen am Telefon",12,0.75,0.10,0.02,0.78,0.10,"mouth","winkworth-1995"),
     breath_pattern("push","Pushing off the wall","Abstoßen von der Wand",18,0.9,0.42,0.0,0.52,0.06,"nose-and-mouth","gupta-2010")]
N['thermal']={"environment":human_env(18,17,0.5,0.3,0,1.0,sky=10,ground=15),"baseMet":1.1,"body":physiology(66,172),"respiration":respiration(pat,33,0.95),
 "operating":[{"id":"phone","title":T("Phone activity","Telefonaktivität"),"levels":[{"v":0,"title":T("Off","Aus")},{"v":0.3,"title":T("Screen on","Bildschirm an")},{"v":1,"title":T("In a call","Im Gespräch")}]}],"regions":regs,
 "emitters":breath_emitters(0.2639,s)+[plume_emitter("torso-clothing",0.35)],
 "assumptions":[
  T("DuBois skin area for 1.72 m and 66 kg; hoodie (0.35 clo), jeans (0.25 clo) and a wool beanie (0.2 clo) after ISO 9920 with 1 clo = 0.155 m2K/W; layer resistances include the enclosed air.","DuBois-Hautfläche für 1,72 m und 66 kg; Kapuzenpullover (0,35 clo), Jeans (0,25 clo) und Wollmütze (0,2 clo) nach ISO 9920 mit 1 clo = 0,155 m2K/W; die Schichtwiderstände enthalten die eingeschlossene Luft."),
  T("Contact conduction: the shoulder blades press on a 15 to 18 degree wall surface over about 8 % of the hoodie area (12 % with the seat), the shoes stand on 15 degree pavement over half of their area, the hand holds a phone warmed to about 27 degrees; the wall surface value of 17.5 degrees is an authored estimate for a masonry wall of a heated building in the shade.","Wärmeleitung: Die Schulterblätter drücken auf eine 15 bis 18 Grad kühle Wandfläche auf etwa 8 % der Pulloverfläche (12 % mit dem Gesäß), die Schuhe stehen auf 15 Grad kaltem Pflaster auf der Hälfte ihrer Fläche, die Hand hält ein auf etwa 27 Grad erwärmtes Telefon; der Wandoberflächenwert von 17,5 Grad ist eine Autorenschätzung für eine Mauer eines beheizten Gebäudes im Schatten."),
  T("The wall is a conditioned element: 15 mm plaster and 365 mm brick (U about 1.5 W/(m2K) with films) between 18 degree air and a 20 degree interior; its heat capacity is that of the first 0.1 m.","Die Wand ist ein konditioniertes Bauteil: 15 mm Putz und 365 mm Ziegel (U etwa 1,5 W/(m2K) mit Übergängen) zwischen 18 Grad warmer Luft und 20 Grad Innenraum; die Wärmekapazität entspricht den ersten 0,1 m."),
  T("Speech breathing (12 breaths per minute, 0.75 L, short mouth inspiration and 3.9 s expiration) follows the pattern described by Winkworth et al.; while listening the calm nasal pattern applies. Numbers are authored estimates.","Die Sprechatmung (12 Atemzüge pro Minute, 0,75 L, kurze Mundeinatmung und 3,9 s Ausatmung) folgt dem von Winkworth et al. beschriebenen Muster; beim Zuhören gilt das ruhige Nasenmuster. Die Zahlen sind Autorenschätzungen.")]}
RG=regs; EM=["plume"]
def sole(fr): return {"shoes":{"contact":{"fraction":fr,"tempC":15}}}
spec={"leaning-shoulder-wall":(1.1,"rest",0.3,{"torso-clothing":{"contact":{"fraction":0.08,"tempC":17.5}},"hand-skin":{"contact":{"fraction":0.12,"tempC":27}},**sole(0.5)},0.3,0.8),
 "leaning-arms-folded":(1.0,"rest",0.0,{"torso-clothing":{"contact":{"fraction":0.12,"tempC":17.5}},"arm-clothing":{"exposedFraction":0.6},"hand-skin":{"exposedFraction":0.5},**sole(0.5)},0.3,0.7),
 "phone-call":(1.2,"phone-talk",1.0,{"torso-clothing":{"contact":{"fraction":0.08,"tempC":17.5}},"hand-skin":{"contact":{"fraction":0.12,"tempC":29}},"head-skin":{"contact":{"fraction":0.03,"tempC":30}},**sole(0.5)},0.3,0.8),
 "pushing-off":(1.8,"push",0.3,{"hand-skin":{"contact":{"fraction":0.12,"tempC":27}},**sole(0.5)},0.3,1.0),
 "standing-away":(1.3,"rest",0.3,{"hand-skin":{"contact":{"fraction":0.12,"tempC":27}},**sole(0.5)},0.3,0.9)}
for p in N['poses']:
    met,br,ph,reg,_,pl=spec[p['id']]
    bind_pose(p,RG,EM,met,br,reg=reg,extra_emit={"plume":pl})
    p['thermal']['operating']={"phone":ph}
for c in N['clips']:
    c['events']=[e for e in c['events'] if e['kind'] not in('inhale','exhale')]
    c['tracks']=[t for t in c['tracks'] if t['target']!="emitter:breath"]
    if c['id']=="phone-call-idle":
        c['tracks']+=[{"target":"breathing","ease":"step","keys":[{"t":0,"v":0},{"t":3,"v":1},{"t":6,"v":0}]},{"target":"operating:phone","ease":"step","keys":[{"t":0,"v":0.3},{"t":3,"v":1},{"t":6,"v":0.3}]}]
        c['events'].append({"t":3,"kind":"thermal","title":T("Speech starts: mouth breathing with long expirations","Sprechen beginnt: Mundatmung mit langen Ausatmungen")})
    if c['id']=="push-off":
        c['tracks']+=[{"target":"breathing","ease":"step","keys":[{"t":0,"v":0},{"t":0.8,"v":2},{"t":2.4,"v":0}]}]
        c['events'].append({"t":1.4,"kind":"thermal","title":T("The back leaves the wall: wall contact conduction ends","Der Rücken löst sich von der Wand: die Wärmeleitung in die Wand endet")})
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
kit=[sh for sh in build_body(100,P) if sh['id'] in {"torso","pelvis"}|{f"{p}-{sd}" for p in("upper-arm","forearm","thigh","shank") for sd in "lr"}]
R[50]=v+kit
for sh in R[50]:
    if sh['id']=='wall-surface': sh['role']='ground'
    if sh['id']=='wall-courses': sh['role']='ground'; sh['penMm']=0.13
N['representations']={
 "500":rep(500,R[500],DEC(
  silhouette=("A solid dark figure of thirteen articulated overlapping forms leaning back against a single wall line: the backward-tilted trunk with head and beanie as one path, the near arm with the phone in the fist, legs with the sneakers.","Eine flächig dunkle Figur aus dreizehn gelenkig überlappenden Formen, die sich an eine einzelne Wandlinie zurücklehnt: der nach hinten geneigte Rumpf mit Kopf und Mütze als ein Pfad, der nahe Arm mit dem Telefon in der Faust, Beine mit den Sneakern."),
  retained=("The 1.72 m height, the backward lean of 11 degrees with the shoulder blades touching the wall line and the forward head; both legs stay separate forms so the wall-to-feet gap reads.","Die Höhe von 1,72 m, die Rückneigung von 11 Grad mit den Schulterblättern an der Wandlinie und der vorgeschobene Kopf; beide Beine bleiben getrennte Formen, damit der Abstand zwischen Wand und Füßen lesbar ist."),
  merged=("Head, beanie, hood, hoodie and pelvis form one path; sleeve, wrist, hand and phone are one forearm path; jeans stay thigh and shank forms, the sneaker has its own form and toe.","Kopf, Mütze, Kapuze, Pullover und Becken bilden einen Pfad; Ärmel, Handgelenk, Hand und Telefon einen Unterarmpfad; Jeans bleiben Oberschenkel- und Unterschenkelform, der Sneaker hat eine eigene Form samt Zehe."),
  omitted=("Face, ear, hood string, pocket, hem, jeans seam, phone screen (the phone is 7 cm wide and joins the fist), wall courses and all fingers.","Gesicht, Ohr, Kapuzenband, Tasche, Saum, Jeansnaht, Telefonbildschirm (das Telefon ist 7 cm breit und geht in der Faust auf), Mauerwerksfugen und alle Finger."),
  thinElements=("The wall is one 0.2 mm ground line at the real wall plane (the line width is 10 cm real) and not a hatched mass, so it does not enter the figure bounds.","Die Wand ist eine einzelne 0,2-mm-Bodenlinie in der echten Wandebene (die Linienbreite entspricht 10 cm real) und keine schraffierte Masse, damit sie nicht in die Figurenmaße eingeht."),
  hatching=("None.","Keine."),
  styles=("Solid black figure (tone 0) with 0.15 mm outline; the wall line at 0.2 mm is slightly heavier than the figure line.","Flächig schwarze Figur (Ton 0) mit 0,15-mm-Kontur; die Wandlinie mit 0,2 mm ist etwas kräftiger als die Figurenlinie."),
  overlays=("Merged forms mix their regions by area; the wall face keeps its own region on the wall line so the contact area on the wall stays identifiable.","Verschmolzene Formen mischen ihre Regionen nach Fläche; die Wandfläche behält ihre eigene Region auf der Wandlinie, damit die Kontaktfläche an der Wand erkennbar bleibt."))),
 "200":rep(200,R[200],DEC(
  silhouette=("Beanie-capped head with the hood bunched at the neck, boxy hoodie trunk, sleeves bent to the phone, jeans and two-part sneakers, drawn leaning on the wall line.","Kopf mit Mütze und der am Hals gebauschten Kapuze, kastiger Pulloverrumpf, zum Telefon gebeugte Ärmel, Jeans und zweiteilige Sneaker, an die Wandlinie gelehnt gezeichnet."),
  retained=("Hood, hoodie, sleeve, jeans, sneaker and beanie masses with tone steps (dark jeans, mid hoodie, white shoes), wrist skin line and the phone as one small dark rectangle.","Kapuze, Pullover, Ärmel, Jeans, Sneaker und Mützenmasse mit Tonstufen (dunkle Jeans, mittlerer Pullover, weiße Schuhe), Handgelenkslinie und das Telefon als kleines dunkles Rechteck."),
  merged=("Hand and forearm are one sleeve segment; beanie and hair are one dark mass.","Hand und Unterarm sind ein Ärmelsegment; Mütze und Haar sind eine dunkle Masse."),
  omitted=("Face details, fingers, pocket, hem, jeans seam, hood string, beanie rim, phone screen and wall courses.","Gesichtsdetails, Finger, Tasche, Saum, Jeansnaht, Kapuzenband, Mützenumschlag, Telefonbildschirm und Mauerwerksfugen."),
  thinElements=("The 6 cm wrist line is a 0.13 mm detail stroke; the wall stays a single line.","Die 6 cm breite Handgelenkslinie ist ein 0,13-mm-Detailstrich; die Wand bleibt eine einzelne Linie."),
  hatching=("None.","Keine."),
  styles=("0.18 mm outlines; wall line 0.25 mm; tones: hoodie 0.34, jeans 0.3, skin 0.8, shoes 0.9.","0,18-mm-Konturen; Wandlinie 0,25 mm; Töne: Pullover 0,34, Jeans 0,3, Haut 0,8, Schuhe 0,9."),
  overlays=("Hoodie, hood, sleeve, jeans, shoe, face, beanie, hand, wrist, phone and the wall face are separate shapes, so the wall contact patch and cooled shoulders can be coloured separately.","Pullover, Kapuze, Ärmel, Jeans, Schuh, Gesicht, Mütze, Hand, Handgelenk, Telefon und Wandfläche sind getrennte Formen, die Kontaktfläche zur Wand und die ausgekühlten Schultern lassen sich also getrennt einfärben."))),
 "100":rep(100,R[100],DEC(
  silhouette=("Hoodie with front pocket, hem and bunched hood, sleeves with a visible wrist, jeans, sneakers with toe cap, beanie with rim, hand with thumb holding a phone with a glass screen; the wall as a face line with brick courses.","Pullover mit Känguru-Tasche, Saum und gebauschter Kapuze, Ärmel mit sichtbarem Handgelenk, Jeans, Sneaker mit Zehenkappe, Mütze mit Umschlag, Hand mit Daumen am Telefon mit Glasbildschirm; die Wand als Flächenlinie mit Ziegelschichten."),
  retained=("Pocket and hem lines, beanie rim, wrist line, phone body and screen, wall courses every 0.3 m.","Taschen- und Saumlinien, Mützenumschlag, Handgelenkslinie, Telefonkörper und -bildschirm, Mauerwerksschichten alle 0,3 m."),
  merged=("Neck and head share an outline; the hood forms one shape behind the head.","Hals und Kopf teilen eine Kontur; die Kapuze bildet eine Form hinter dem Kopf."),
  omitted=("Eye, ear, mouth, hood string, jeans seam, laces, soles and individual brick joints.","Auge, Ohr, Mund, Kapuzenband, Jeansnaht, Schnürung, Sohlen und einzelne Ziegelfugen."),
  thinElements=("The phone (7 cm thick) is a 0.7 mm body with a 0.13 mm glazing screen; the courses are 0.13 mm ground-role ticks left of the wall plane.","Das Telefon (7 cm Breite) ist ein 0,7-mm-Körper mit 0,13-mm-Bildschirm; die Schichten sind 0,13-mm-Bodenrollenstriche links der Wandebene."),
  hatching=("None; the wall is a line, not a mass.","Keine; die Wand ist eine Linie, keine Masse."),
  styles=("0.2 mm outlines, 0.13 mm details, glazing 0.13 mm in light blue; wall line 0.3 mm.","0,2-mm-Konturen, 0,13-mm-Details, Verglasung 0,13 mm in Hellblau; Wandlinie 0,3 mm."),
  overlays=("Hoodie and hood share the hoodie region over the jeans region; the phone carries its own operating region, the wall its conditioned region.","Pullover und Kapuze teilen die Pulloverregion über der Jeansregion; das Telefon trägt seine eigene Betriebsregion, die Wand ihre konditionierte Region."))),
 "50":rep(50,R[50],DEC(
  silhouette=("Full drawing with hood string, pocket, hem, beanie rim, jeans seam, finger lines, laces, soles, the phone with screen and the wall face with courses.","Volle Zeichnung mit Kapuzenband, Tasche, Saum, Mützenumschlag, Jeansnaht, Fingerlinien, Schnürung, Sohlen, dem Telefon mit Bildschirm und der Wandfläche mit Schichten."),
  retained=("All v1 detail; the v1 square-ended limbs are replaced by smooth, round-capped limb bodies so the leaning pose has closed knees and elbows.","Alle v1-Details; die v1-Glieder mit eckigen Enden sind durch glatte, gerundete Gliedmaßen ersetzt, damit die Lehnposen geschlossene Knie und Ellbogen haben."),
  merged=("Beanie and hair are separate shapes; the sleeve and the forearm are one shape.","Mütze und Haar sind getrennte Formen; Ärmel und Unterarm sind eine Form."),
  omitted=("Hood lining, fabric folds, individual bricks.","Kapuzenfutter, Stofffalten, einzelne Ziegel."),
  thinElements=("Finger lines 0.13 mm, hood string 0.13 mm, courses 0.13 mm.","Fingerlinien 0,13 mm, Kapuzenband 0,13 mm, Schichten 0,13 mm."),
  hatching=("None.","Keine."),
  styles=("0.25 mm silhouettes, 0.18 mm seams, 0.13 mm textures; wall line 0.35 mm.","0,25-mm-Silhouetten, 0,18-mm-Nähte, 0,13-mm-Texturen; Wandlinie 0,35 mm."),
  overlays=("Every region has its own shape, including wrist, phone and the wall face; the wall is drawn as the contact plane for the shoulder-blade patch.","Jede Region hat eine eigene Form, auch Handgelenk, Telefon und Wandfläche; die Wand ist als Kontaktebene für die Schulterblattfläche gezeichnet.")))}
N['symbols']=[]
N['exports']=A1['exports']
if __name__=="__main__":
    finish(N,out_path("leaning-person")); print("written")
