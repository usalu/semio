from jogger import *
from common import *
N=copy.deepcopy(A1)
for k in ('geometry',): N.pop(k)
N['dimensions'].update({'heightM':1.77})
N['summary']=T("A 1.82 m athletic jogger in running shirt, short shorts and running shoes at 2.6 m/s: two flight phases per stride, heavy mouth breathing, wet skin and a stretching sequence afterwards; four individually drawn scales from 1:500 to 1:50.",
 "Ein 1,82 m großer, sportlicher Jogger in Laufshirt, kurzer Hose und Laufschuhen bei 2,6 m/s: zwei Flugphasen pro Schritt­zyklus, kräftige Mundatmung, feuchte Haut und anschließende Dehnübungen; vier einzeln gezeichnete Maßstäbe von 1:500 bis 1:50.")
N['provenance']['references']+= refs("gagge-1971","gupta-2010","hoeppe-1981","astrand-rodahl-2003","bramble-carrier-1983","mifflin-1990","incropera-2011")
N['provenance']['author']="Semio staffage content worker CV-H3"
N['provenance']['notes']=T("Converted to semio.staffage/v2: the v1 outlines are the raw material of the 1:50 drawing; the 1:100, 1:200 and 1:500 drawings are new composed forms bound to the same rig. Thermal, respiration and clothing layers are authored estimates guided by the cited sources.",
 "Auf semio.staffage/v2 umgestellt: Die v1-Umrisse sind Rohmaterial der 1:50-Zeichnung; die Zeichnungen 1:100, 1:200 und 1:500 sind neu komponierte Formen auf demselben Rig. Thermik, Atmung und Kleidungsschichten sind Autorenschätzungen nach den genannten Quellen.")
# ---------------- materials
M={m['id']:m for m in N['materials']}
M['leather-shoe']['title']=T("Running shoe (mesh and foam)","Laufschuh (Mesh und Schaum)")
M['cotton-clothing']['title']=T("Cotton sock","Baumwollsocke")
N['materials']=list(M.values())
# ---------------- anchors
N['anchors']=anchors_head(N['anchors'],0.2792)
# ---------------- thermal
gA=1.924
regs=[]
regs.append(region("head-skin","Face and neck skin","Haut von Gesicht und Hals","skin","skin",0.096,0.15,"sphere",0.55,[],15000,"head",0.05,0.05,ref="dubois-1916",source="literature"))
regs.append(region("hair","Hair and headband","Haar und Stirnband","hair","hair",0.077,0.15,"sphere",0.6,[layer("hair-pack","Hair and headband","Haar und Stirnband","hair","hair",res=0.05,thick=0.008,cap=250,ref="iso-9920-2007")],4000,"head",0.01,0.04,ref="iso-9920-2007"))
regs.append(region("torso-clothing","Running shirt","Laufshirt","clothing","nylon-clothing",0.783,0.3,"cylinder",0.5,
  [airgap("torso-air","Air between skin and shirt","Luft zwischen Haut und Shirt",0.025,"nylon-clothing"),layer("shirt","Polyester running shirt (0.09 clo)","Polyester-Laufshirt (0,09 clo)","clothing","nylon-clothing",clo=0.09,thick=0.0008,cap=160,ref="iso-9920-2007")],20000,"trunk",0.4,0.37,ref="iso-9920-2007"))
regs.append(region("arm-clothing","Short sleeves and armband","Kurze Ärmel und Armband","clothing","nylon-clothing",0.059,0.08,"cylinder",0.5,
  [airgap("sleeve-air","Air under the sleeve","Luft unter dem Ärmel",0.02,"nylon-clothing"),layer("sleeve","Polyester sleeve (0.09 clo)","Polyester-Ärmel (0,09 clo)","clothing","nylon-clothing",clo=0.09,thick=0.0008,cap=160,ref="iso-9920-2007")],16000,"arm",0.04,0.028,ref="iso-9920-2007"))
regs.append(region("arm-skin","Bare arms","Nackte Arme","skin","skin",0.196,0.08,"cylinder",0.5,[],14000,"arm",0.09,0.102,ref="dubois-1916",source="literature"))
regs.append(region("hand-skin","Hands","Hände","skin","skin",0.096,0.07,"cylinder",0.4,[],10000,"hand",0.03,0.05,ref="dubois-1916",source="literature"))
regs.append(region("leg-clothing","Running shorts","Laufshorts","clothing","nylon-clothing",0.182,0.15,"cylinder",0.4,
  [airgap("shorts-air","Air under the shorts","Luft unter der Hose",0.02,"nylon-clothing"),layer("shorts","Polyester running shorts (0.05 clo)","Polyester-Laufshorts (0,05 clo)","clothing","nylon-clothing",clo=0.05,thick=0.0007,cap=140,ref="iso-9920-2007")],18000,"leg",0.12,0.086,ref="iso-9920-2007"))
regs.append(region("leg-skin","Bare legs and ankle socks","Nackte Beine und Knöchelsocken","skin","skin",0.412,0.12,"cylinder",0.4,[],16000,"leg",0.24,0.214,ref="dubois-1916",source="literature"))
regs.append(region("shoes","Running shoes over socks","Laufschuhe über Socken","clothing","leather-shoe",0.135,0.12,"horizontal-plate-up",0.25,
  [layer("sock-layer","Cotton sock (0.04 clo)","Baumwollsocke (0,04 clo)","clothing","cotton-clothing",clo=0.04,thick=0.002,cap=300,ref="iso-9920-2007"),airgap("shoe-air","Air in the shoe","Luft im Schuh",0.04,"leather-shoe",thick=0.01),layer("shoe-upper","Mesh and foam upper","Obermaterial aus Mesh und Schaum","shell","leather-shoe",res=0.03,thick=0.008,cap=900,ref="iso-9920-2007")],6000,"foot",0.02,0.06,ref="iso-9920-2007"))
pat=[breath_pattern("run","Running, mouth breathing","Laufen, Mundatmung",41.67,1.4,0.46,0.0,0.54,0.0,"mouth","bramble-carrier-1983"),
     breath_pattern("cooldown","Recovery after the run","Erholung nach dem Lauf",26,1.1,0.42,0.02,0.5,0.06,"nose-and-mouth","astrand-rodahl-2003"),
     breath_pattern("rest","Calm standing breathing","Ruhige Atmung im Stand",16,0.7,0.40,0.04,0.46,0.10,"nose","gupta-2010")]
N['thermal']={"environment":human_env(20,20,0.5,0.2,0,1.0,sky=12,ground=22),"baseMet":1.5,
 "body":physiology(72,182),"respiration":respiration(pat,34,0.95),"operating":[],"regions":regs,
 "emitters":breath_emitters(0.2792,1)+[plume_emitter("torso-clothing")],
 "assumptions":[
  T("DuBois skin area for 1.82 m and 72 kg; bare arms, legs and a 0.09 clo polyester shirt with 0.05 clo shorts (ISO 9920 values, 1 clo = 0.155 m2K/W) give the large heat loss of runners. Layer resistances include the enclosed air.","DuBois-Hautfläche für 1,82 m und 72 kg; nackte Arme und Beine, ein Polyestershirt (0,09 clo) und Shorts (0,05 clo) nach ISO 9920 (1 clo = 0,155 m2K/W) ergeben den hohen Wärmeverlust beim Laufen. Die Schichtwiderstände enthalten die eingeschlossene Luft."),
  T("Metabolic rate 8 met at 2.6 m/s (Compendium: running 5 mph 8.3 met, jogging 7 met), 2 to 4 met while stretching; skin wettedness 0.3 while running. Skin blood flow limits 6.3 to 90 L/(m2 h), set points 36.8/34.1 degrees Celsius after the Gagge two-node model.","Stoffwechsel 8 met bei 2,6 m/s (Compendium: Laufen 5 mph 8,3 met, Joggen 7 met), 2 bis 4 met beim Dehnen; Hautbenetzung 0,3 beim Laufen. Hautdurchblutung 6,3 bis 90 L/(m2 h), Sollwerte 36,8/34,1 Grad Celsius nach dem Zwei-Knoten-Modell von Gagge."),
  T("Running breathes through the mouth at 41.7 breaths per minute with 1.4 L tidal volume (about 58 L/min, consistent with 2 L/min oxygen uptake at 8 met), phase-locked 4:1 to the step rate of 2.8 Hz (one breath per two strides). Exhaled air 34 degrees Celsius, 95 % humidity. All respiration numbers are authored estimates guided by the cited sources.","Beim Laufen wird durch den Mund geatmet: 41,7 Atemzüge pro Minute mit 1,4 L Atemzugvolumen (etwa 58 L/min, passend zu 2 L/min Sauerstoffaufnahme bei 8 met), 4:1 an die Schrittfrequenz von 2,8 Hz gekoppelt (ein Atemzug je zwei Doppelschritte). Ausatemluft 34 Grad Celsius, 95 % Feuchte. Alle Atemwerte sind Autorenschätzungen nach den genannten Quellen."),
  T("Contact conduction through the shoe sole uses 26 % of the shoe area per planted foot at ground 22 degrees Celsius; flight phases have no contact.","Die Wärmeleitung durch die Sohle nutzt 26 % der Schuhfläche je aufgesetztem Fuß bei 22 Grad Celsius Boden; Flugphasen haben keinen Kontakt.")]}
# ---------------- poses
RG=regs; EM=["plume"]
wet_run={"head-skin":{"wettedness":0.3},"arm-skin":{"wettedness":0.3},"leg-skin":{"wettedness":0.3},"hand-skin":{"wettedness":0.25},"torso-clothing":{"wettedness":0.2}}
wet_str={"head-skin":{"wettedness":0.2},"arm-skin":{"wettedness":0.2},"leg-skin":{"wettedness":0.2},"hand-skin":{"wettedness":0.15},"torso-clothing":{"wettedness":0.15}}
def sole(frac): return {"shoes":{"contact":{"fraction":frac,"tempC":22}}}
def mix(a,b):
    r={k:dict(v) for k,v in a.items()}
    for k,v in b.items(): r.setdefault(k,{}).update(v)
    return r
contact_frac={"initial-contact-l":0.20,"loading-l":0.26,"midstance-l":0.26,"propulsion-l":0.12,"toe-off-l":0.06,"flight-l":None,
              "initial-contact-r":0.20,"loading-r":0.26,"midstance-r":0.26,"propulsion-r":0.12,"toe-off-r":0.06,"flight-r":None}
for p in N['poses']:
    pid=p['id']
    if pid in contact_frac:
        reg=mix(wet_run,sole(contact_frac[pid]) if contact_frac[pid] else {})
        bind_pose(p,RG,EM,8,"run",reg=reg,extra_emit={"plume":0.35})
    elif pid=="stretch-stand": bind_pose(p,RG,EM,2.6,"cooldown",reg=mix(wet_str,sole(0.52)),extra_emit={"plume":1.0})
    elif pid=="stretch-quad-r": bind_pose(p,RG,EM,2.2,"cooldown",reg=mix(wet_str,sole(0.26)),extra_emit={"plume":0.9})
    elif pid=="stretch-hamstring": bind_pose(p,RG,EM,2.0,"rest",reg=mix(wet_str,sole(0.52)),extra_emit={"plume":0.8})
    else: raise SystemExit(pid)
# ---------------- clips
for c in N['clips']:
    c['events']=[e for e in c['events'] if e['kind'] not in('inhale','exhale')]
    c['tracks']=[t for t in c['tracks'] if t['target']!="emitter:breath"]
    if c['id']=="run-cycle":
        c['events'].append({"t":0.0,"kind":"thermal","title":T("One breath per two strides: exhalation starts at left foot strike (4:1 coupling)","Ein Atemzug je zwei Schritte: Ausatmen beginnt beim linken Fußaufsatz (4:1-Kopplung)")})
        c['events'].sort(key=lambda e:e['t'])
    if c['id']=="stretch":
        c['tracks']+= [{"target":"breathing","ease":"step","keys":[{"t":0,"v":1},{"t":3,"v":1},{"t":8,"v":2}]},
                       {"target":"emitter:plume","ease":"smooth","keys":[{"t":0,"v":0.6},{"t":3,"v":0.9},{"t":8,"v":1.0}]}]
        c['events'].append({"t":8,"kind":"thermal","title":T("Breathing settles to the calm nasal pattern","Atmung beruhigt sich zum ruhigen Nasenmuster")})
        c['events'].sort(key=lambda e:e['t'])
N['presentation']['defaultScale']=100
N['presentation']['viewpoints']=["elevation"]
N['completion']="authored"
# ---------------- representations
R=reps()
R[50].append(shp('hair-strands','head','texture',5.5,False,0.4,L_([(0.2,0.088),(0.262,0.05),(0.283,-0.004)]),region='hair'))
R[50].append(shp('hair-strands-2','head','texture',5.5,False,0.4,L_([(0.15,0.075),(0.2,0.04),(0.225,-0.02)]),region='hair'))
sh100=R[100]
sh100.append(shp("headband","head","detail",5.5,False,0.9,L_([(0.212,0.0983),(0.244,0.0397),(0.2503,-0.0298),(0.2337,-0.0864)]),region="hair"))
sh100.append(shp("armband","upper-arm-l","body",1.42,True,0.25,P_(profile(sub(UARM,0.0724,0.1758,0.006),1.0)),region="arm-clothing"))
R[50].append(shp('armband','upper-arm-l','body',1.42,True,0.25,P_(profile(sub(UARM,0.0724,0.1758,0.006),1.0)),region='arm-clothing'))
N['representations']={
 "500":rep(500,R[500],DEC(
  silhouette=("One solid dark runner silhouette in thirteen articulated overlapping forms so that the run cycle still animates: head, trunk and pelvis as one path on the trunk bone, and per side an upper arm, a forearm with fist, a thigh, a shank, a shoe and a toe wedge.","Eine geschlossene dunkle Läufersilhouette aus dreizehn gelenkig verbundenen, überlappenden Formen, damit der Laufzyklus weiter animiert werden kann: Kopf, Rumpf und Becken als ein Pfad am Rumpfknochen, je Seite Oberarm, Unterarm mit Faust, Oberschenkel, Unterschenkel, Schuh und Zehenkeil."),
  retained=("Stride pose with both flight-capable legs, forward trunk, bent arms and the nose-forward head line; the 1.77 m height of the forward-leaning default pose (standing height 1.82 m) and its 1.04 m stride width are reproduced.","Laufhaltung mit beiden flugfähigen Beinen, Rumpf, gebeugten Armen und vorgeschobenem Kopf; Höhe 1,77 m der vorgeneigten Standardpose (Standhöhe 1,82 m) und ihre Schrittweite von 1,04 m werden wiedergegeben."),
  merged=("Neck, head and hair, shirt, shorts and pelvis form one body; hand and fist merge into the forearm; sock and shin form one lower-leg form. Parts closer than the minimum gap (0.1 m real, 0.2 mm paper) are drawn as one.","Hals, Kopf und Haar, Shirt, Hose und Becken bilden einen Körper; Hand und Faust verschmelzen mit dem Unterarm; Socke und Schienbein bilden eine Unterschenkelform. Teile mit weniger als dem Mindestabstand (0,1 m real, 0,2 mm Papier) werden als eins gezeichnet."),
  omitted=("Face, ear, fingers, thumb, headband, armband, sock edge, shoe sole, seams and the far-hand outline are omitted; joints are hidden by overlap instead of lines.","Gesicht, Ohr, Finger, Daumen, Stirnband, Armbinde, Sockenkante, Schuhsohle, Nähte und der Umriss der fernen Hand entfallen; Gelenke werden durch Überlappung statt durch Linien verdeckt."),
  thinElements=("Fingers and the headband are far below the pen width (0.15 mm = 7.5 cm) and are dropped; the 7 cm wrist is not thickened and stays inside the merged arm.","Finger und Stirnband liegen weit unter der Federbreite (0,15 mm = 7,5 cm) und entfallen; das 7 cm breite Handgelenk wird nicht verdickt und bleibt im verschmolzenen Arm."),
  hatching=("No hatching: the figure is solid.","Keine Schraffur: Die Figur ist flächig gefüllt."),
  styles=("Dark solid fill (tone 0) with a 0.15 mm outline for every part, so overlapped joints disappear; line weight is the only hierarchy.","Dunkle Vollfläche (Ton 0) mit 0,15 mm Kontur für jedes Teil, überlappende Gelenke verschwinden; die Strichstärke ist die einzige Hierarchie."),
  overlays=("Thermal overlays colour each merged form with the area-weighted value of its regions: the trunk form mixes face, hair, shirt and shorts, the upper-arm forms mix sleeve and bare arm, the forearm forms bare arm and hand, the leg forms bare leg, the shoe forms their own region.","Thermik-Überlagerungen färben jede verschmolzene Form mit dem flächengewichteten Wert ihrer Regionen: Die Rumpfform mischt Gesicht, Haar, Shirt und Hose, die Oberarmformen Ärmel und nackten Arm, die Unterarmformen nackten Arm und Hand, die Beinformen nacktes Bein, die Schuhformen ihre eigene Region."))),
 "200":rep(200,R[200],DEC(
  silhouette=("Head with hair cap, shirt trunk, shorts, one arm of two segments with the hand as a tapered tip, one leg of thigh and shank per side, and a shoe of two parts so the toe follows the foot in toe-off.","Kopf mit Haarkappe, Shirtrumpf, Hose, je ein Arm aus zwei Segmenten mit der Hand als spitz zulaufendem Ende, je ein Bein aus Ober- und Unterschenkel und ein zweiteiliger Schuh, damit die Zehe dem Abdruck folgt."),
  retained=("The separate masses head, hair, shirt, shorts, sleeve, bare arm, bare leg and shoe with their different tones, and rounded knee and elbow caps that keep joints closed during the run.","Die getrennten Massen Kopf, Haar, Shirt, Hose, Ärmel, nackter Arm, nacktes Bein und Schuh mit unterschiedlichen Tönen sowie gerundete Knie- und Ellbogenenden, die die Gelenke beim Laufen geschlossen halten."),
  merged=("Hand and forearm form one tapered arm segment; neck and head are one outline; sock and shoe are one colour with the shoe.","Hand und Unterarm bilden ein spitz zulaufendes Armsegment; Hals und Kopf sind eine Kontur; Socke und Schuh haben die gleiche Farbe wie der Schuh."),
  omitted=("Thumb, fingers, face details, headband, armband, sock, laces, sole line and seams (below 0.13 mm paper or without meaning at 1:200).","Daumen, Finger, Gesichtsdetails, Stirnband, Armbinde, Socke, Schnürung, Sohlenlinie und Nähte (unter 0,13 mm Papier bzw. ohne Aussage bei 1:200)."),
  thinElements=("The 4 cm wide wrist and the 3 cm hand tip are drawn as a continuous taper of the arm; nothing is thickened.","Das 4 cm breite Handgelenk und die 3 cm breite Handspitze sind als durchgehende Verjüngung des Arms gezeichnet; nichts wird verdickt."),
  hatching=("No hatching; tone alone separates light shirt, dark shorts and skin.","Keine Schraffur; allein der Ton trennt helles Shirt, dunkle Hose und Haut."),
  styles=("0.18 mm body pen for outlines, light shirt and skin, dark shorts and hair as solids, white shoes.","0,18 mm Körperfeder für Konturen, helles Shirt und Haut, dunkle Hose und Haar als Vollflächen, weiße Schuhe."),
  overlays=("Each mass carries its own region: shirt, sleeve, bare arm, hand, shorts, bare leg, shoe, face and hair, so overlays show their separate values; the sleeve is a separate form on the upper arm.","Jede Masse trägt ihre eigene Region: Shirt, Ärmel, nackter Arm, Hand, Hose, nacktes Bein, Schuh, Gesicht und Haar, so zeigen Überlagerungen ihre getrennten Werte; der Ärmel ist eine eigene Form auf dem Oberarm."))),
 "100":rep(100,sh100,DEC(
  silhouette=("Articulated clothing outlines: shirt with short sleeves, shorts over the thigh, bare lower leg with ankle sock, two-part running shoe, head with face profile and hair cap, hand as a mitten with thumb.","Gegliederte Kleidungsumrisse: Shirt mit kurzen Ärmeln, Shorts über dem Oberschenkel, nacktes Bein mit Knöchelsocke, zweiteiliger Laufschuh, Kopf mit Gesichtsprofil und Haarkappe, Hand als Fäustling mit Daumen."),
  retained=("Sleeve and bare arm, shorts hem on the thigh, sock edge, shoe heel and toe, nose, chin and hair mass, headband and armband.","Ärmel und nackter Arm, Hosensaum am Oberschenkel, Sockenkante, Schuhferse und -spitze, Nase, Kinn und Haarmasse, Stirnband und Armbinde."),
  merged=("Neck and head share one outline; the left and right hand details of the 1:50 drawing are reduced to one outline each.","Hals und Kopf teilen eine Kontur; die Handdetails der 1:50-Zeichnung sind auf je eine Kontur reduziert."),
  omitted=("Eye, brow, ear, mouth line, fingers, laces, soles, shirt number and stripe, hair strands.","Auge, Braue, Ohr, Mundlinie, Finger, Schnürung, Sohlen, Startnummer und Streifen, Haarsträhnen."),
  thinElements=("The headband (3 cm) is one 0.13 mm detail line; fingers are merged into the mitten, the thumb stays a separate form (2.5 cm wide = 0.25 mm).","Das Stirnband (3 cm) ist eine 0,13-mm-Detaillinie; die Finger sind im Fäustling verschmolzen, der Daumen bleibt eigene Form (2,5 cm breit = 0,25 mm)."),
  hatching=("None: tone fills carry the material; hatching would be below the minimum spacing on 0.2 m wide parts.","Keine: Tonflächen tragen das Material; Schraffur läge bei 0,2 m breiten Teilen unter dem Mindestabstand."),
  styles=("0.2 mm outlines, 0.13 mm detail lines; light shirt, skin and white shoes, dark shorts, hair and armband.","0,2-mm-Konturen, 0,13-mm-Detaillinien; helles Shirt, Haut und weiße Schuhe, dunkle Hose, Haar und Armbinde."),
  overlays=("Regions map one-to-one onto the shapes: sleeve and bare arm are separate, the sock carries the leg-skin region, the hand carries hand-skin and the shoes their own region.","Regionen entsprechen den Formen eins zu eins: Ärmel und nackter Arm sind getrennt, die Socke trägt die Region Beinhaut, die Hand die Handhaut und die Schuhe ihre eigene Region."))),
 "50":rep(50,R[50],DEC(
  silhouette=("Full articulation from the v1 drawing: face profile with eye, brow, ear and mouth, hair strands under the headband, shirt with collar, hem, stripe and number, hands with three finger lines and thumb, shorts with stripe, socks, shoes with sole line and laces.","Volle Gliederung aus der v1-Zeichnung: Gesichtsprofil mit Auge, Braue, Ohr und Mund, Haarsträhnen unter dem Stirnband, Shirt mit Kragen, Saum, Streifen und Startnummer, Hände mit drei Fingerlinien und Daumen, Shorts mit Streifen, Socken, Schuhe mit Sohlenlinie und Schnürung."),
  retained=("All v1 shapes: 0.25 mm silhouettes, 0.18 mm seams and hems, 0.13 mm hair strands, laces and shirt stripe.","Alle v1-Formen: 0,25-mm-Silhouetten, 0,18-mm-Nähte und -Säume, 0,13-mm-Haarsträhnen, Schnürung und Shirtstreifen."),
  merged=("The thigh skin and the shorts overlap with a hem line instead of separate outlines.","Oberschenkelhaut und Shorts überlappen mit einer Saumlinie statt mit getrennten Konturen."),
  omitted=("Nostril, eyelashes, individual fingers and fabric folds (under 0.1 mm paper).","Nasenloch, Wimpern, einzelne Finger und Stofffalten (unter 0,1 mm Papier)."),
  thinElements=("Finger lines are 0.13 mm detail strokes at 6 mm spacing; laces are three-point strokes; the sleeve hem is a 0.18 mm line.","Fingerlinien sind 0,13-mm-Striche im Abstand von 6 mm; die Schnürung sind Striche aus drei Punkten; der Ärmelsaum ist eine 0,18-mm-Linie."),
  hatching=("None; the line hierarchy and tones are enough at this scale.","Keine; Linienhierarchie und Töne genügen bei diesem Maßstab."),
  styles=("Silhouettes 0.25 mm, seams 0.18 mm, texture 0.13 mm; light shirt and shoes, dark shorts and hair.","Silhouetten 0,25 mm, Nähte 0,18 mm, Textur 0,13 mm; helles Shirt und Schuhe, dunkle Hose und Haar."),
  overlays=("Every region is its own shape or a detail line on it, including shirt, sleeve, armband, bare arm, hand, shorts, bare leg with sock, shoe, face and hair.","Jede Region ist eine eigene Form oder eine Detaillinie darauf, einschließlich Shirt, Ärmel, Armbinde, nacktem Arm, Hand, Shorts, nacktem Bein mit Socke, Schuh, Gesicht und Haar.")))}
N['symbols']=[{"id":"runner-direction-mark","title":T("Runner position and direction mark","Läufer-Positions- und Richtungsmarke"),"description":T("A small circle with a direction tick marking where a runner is and where they are heading, for overview plans with many people.","Ein kleiner Kreis mit Richtungsstrich, der Standort und Laufrichtung eines Läufers in Übersichtsplänen mit vielen Personen markiert."),
 "semantics":"true-position","units":"paper-mm","denominators":[500,200],"pens":PENS(0.18,None,None,None,0.2),"hatches":[],
 "geometry":{"elevation":[shp("mark-dot","pelvis","body",1,True,0.1,E_(0,0,0.5,0.5)),shp("mark-tick","pelvis","body",1.1,False,0.1,L_([(0.5,0),(1.6,0)]))]},
 "dimensionalNote":T("Only the insertion point is true (the runner's hip position); circle and tick are fixed paper sizes of 1 mm and 1.1 mm and carry no dimension.","Nur der Einfügepunkt ist maßgebend (Hüftposition des Läufers); Kreis und Strich haben feste Papiergrößen von 1 mm und 1,1 mm und tragen keine Maße.")}]
N['exports']=A1['exports']
if __name__=="__main__":
    out={k:N[k] for k in N}
    finish(N,out_path("jogger"))
    print("written")
