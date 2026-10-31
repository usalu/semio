from common import *
A1=load_v1("person-with-umbrella")['asset']
s=0.4202/0.4447
pose0=next(p for p in A1['poses'] if p['id']=="umbrella-up-still")
W0=fk(A1,pose0['bones'],pose_root(A1,pose0))
V=lambda L:[(x/s,l/s,h/s) for x,l,h in L]
COAT_ST=V([(-0.1564,-0.1173,0.1153),(-0.0586,-0.1173,0.1153),(0.0391,-0.128,0.1173),(0.1368,-0.1339,0.1231),(0.2345,-0.1398,0.131),(0.3127,-0.1456,0.1388),(0.3909,-0.127,0.1368),(0.43,-0.0938,0.1095),(0.4495,-0.0684,0.0762)])
HAIR_V=next(sh for sh in A1['geometry']['elevation'] if sh['id']=='hair')['d']
P=dict(s=s,W=W0,torso_st=COAT_ST,
 torso=dict(region="coat",tone=0.62),pelvis=dict(region="leg-clothing",tone=0.25),
 sleeve=dict(region="coat",tone=0.62,x1=None),arm_skin="arm-skin",fsleeve=dict(region="coat",tone=0.62),hand="hand-skin",
 thigh_base=dict(region="leg-clothing",tone=0.2),thigh_g=dict(region="coat",tone=0.62,x1=0.3127/s,grow=0.032),leg_skin="leg-skin",shank_g=dict(region="leg-clothing",tone=0.2),sock=None,
 shoe=dict(region="shoes",tone=0.12),head="head-skin",hair=dict(region="hair",tone=0.3),hair_d={100:HAIR_V,200:HAIR_V},
 reg_fig=["head-skin","hair","coat","leg-clothing","torso-clothing"],reg_uarm=["coat"],reg_farm=["coat","arm-clothing","hand-skin"],reg_thigh=["coat","leg-clothing"],reg_shank=["leg-clothing"],
 thigh500=[(0.0,-0.112,0.105),(0.10,-0.14,0.13),(0.20,-0.125,0.12),(0.3127/s,-0.12,0.115),(0.3127/s,-0.088,0.086),(0.32,-0.068,0.070),(0.4447,-0.054,0.056)])
# canopy / umbrella shapes (rib frame x along the rib, hub at 0)
CF="M 0.0 0.0 C 0.16 0.115 0.42 0.135 0.56 0.01 C 0.42 0.04 0.18 0.03 0.0 0.0 Z"
CB="M 0.0 0.0 C 0.16 -0.115 0.42 -0.135 0.56 -0.01 C 0.42 -0.04 0.18 -0.03 0.0 0.0 Z"
HANDLE=next(sh for sh in A1['geometry']['elevation'] if sh['id']=='handle')['d']
def umbrella_shapes(den):
    o=[shp("canopy-front","rib-front","body",6.0,True,0.3,P_(CF),region="canopy",hatch="gores" if den==50 else None),
       shp("canopy-back","rib-back","body",6.0,True,0.3,P_(CB),region="canopy",hatch="gores" if den==50 else None)]
    if den==500:
        o.append(shp("shaft","umbrella","body",6.1,False,0.2,L_([(-0.1,0),(0.86,0)]),region="umbrella-frame"))
    if den==200:
        o.append(shp("shaft","umbrella","body",6.1,False,0.2,L_([(0.0,0),(0.86,0)]),region="umbrella-frame",penMm=0.13))
        o.append(shp("handle","umbrella","body",6.15,False,0.18,P_("M 0.0 0.0 L -0.09 0.0 C -0.15 0.0 -0.16 -0.07 -0.115 -0.07"),region="umbrella-frame",penMm=0.13))
        o.append(shp("rib-line-front","rib-front","detail",6.05,False,0.1,L_([(0,0),(0.56,0)]),region="umbrella-frame"))
        o.append(shp("rib-line-back","rib-back","detail",6.05,False,0.1,L_([(0,0),(0.56,0)]),region="umbrella-frame"))
    if den in (100,50):
        o+=[shp("shaft","umbrella","body",6.1,True,0.2,C_(-0.12,0,0.86,0,0.0075,0.0075),region="umbrella-frame"),
            shp("handle","umbrella","body",6.15,True,0.18,P_(HANDLE),region="umbrella-frame"),
            shp("ferrule","umbrella","detail",6.2,True,0.1,C_(0.82,0,0.88,0,0.012,0.004),region="umbrella-frame"),
            shp("hub","umbrella","detail",6.16,False,0.3,E_(0.78,0,0.025,0.025),region="umbrella-frame"),
            shp("rib-line-front","rib-front","detail",6.05,False,0.1,L_([(0,0),(0.56,0)]),region="umbrella-frame"),
            shp("rib-line-back","rib-back","detail",6.05,False,0.1,L_([(0,0),(0.56,0)]),region="umbrella-frame"),
            shp("rim-tip-front","rib-front","detail",6.06,True,0.1,E_(0.56,0.004,0.011,0.011),region="umbrella-frame"),
            shp("rim-tip-back","rib-back","detail",6.06,True,0.1,E_(0.56,-0.004,0.011,0.011),region="umbrella-frame")]
        o.append(shp("panel-seam-front","rib-front","texture" if den==50 else "detail",6.02,False,0.2,L_([(0.1,0.07),(0.3,0.1),(0.5,0.04)]),region="canopy"))
        o.append(shp("panel-seam-back","rib-back","texture" if den==50 else "detail",6.02,False,0.2,L_([(0.1,-0.07),(0.3,-0.1),(0.5,-0.04)]),region="canopy"))
    if den==50:
        o.append(shp("furl-strap","rib-front","detail",6.07,False,0.5,L_([(0.3,-0.03),(0.3,0.1)]),region="canopy"))
    return o
def person_details(den):
    o=[]
    if den in(100,200):
        o.append(shp("shirt-collar","head","body",4.7,True,0.85,P_(spath([(-0.02,-0.052),(0.03,-0.058),(0.05,-0.04),(0.05,0.045),(0.03,0.058),(-0.02,0.05)],s)),region="torso-clothing"))
        for sd in "lr":
            o.append(shp(f"cuff-{sd}",f"forearm-{sd}","detail",6.03 if sd=='r' else 1.43,False,0.9,L_([(0.2394,0.0274),(0.2394,-0.0254)]),region="arm-clothing"))
        o.append(shp("coat-hood","head","body",4.5,True,0.55,P_(next(x for x in A1['geometry']['elevation'] if x['id']=='coat-hood')['d']),region="coat"))
    if den==100:
        o.append(shp("coat-hem-r","thigh-r","detail",2.45,False,0.3,L_([(0.3127,0.1222),(0.3127,-0.1319)]),region="coat"))
        o.append(shp("coat-belt","torso","detail",3.3,False,0.3,L_([(0.1173,-0.127),(0.1173,0.1222)]),region="coat"))
    return o

N=copy.deepcopy(A1); N.pop('geometry')
N['dimensions'].update({'heightM':2.09,'widthM':1.05})
N['summary']=T("A person of 1.72 m in a knee-length raincoat holding an open umbrella in rain and gusty wind at 8 degrees Celsius: the canopy ribs open or furl, a wind load acts on the canopy and the person leans into the gust; four individually drawn scales.",
 "Eine 1,72 m große Person im knielangen Regenmantel mit aufgespanntem Schirm bei Regen und Böen und 8 Grad Celsius: Die Schirmrippen öffnen oder schließen sich, eine Windlast wirkt auf den Schirm und die Person lehnt sich in die Böe; vier einzeln gezeichnete Maßstäbe.")
N['provenance']['references']+=refs("gagge-1971","gupta-2010","hoeppe-1981","astrand-rodahl-2003","mifflin-1990","incropera-2011")
N['provenance']['author']="Semio staffage content worker CV-H3"
N['provenance']['notes']=T("Converted to semio.staffage/v2: the v1 outlines are raw material of the 1:50 drawing; the canopy, ribs and shaft are redrawn per scale on the same rig; the coat is modelled as shirt, trapped air and a water-repellent shell.",
 "Auf semio.staffage/v2 umgestellt: Die v1-Umrisse sind Rohmaterial der 1:50-Zeichnung; Schirmdach, Rippen und Schaft sind je Maßstab auf demselben Rig neu gezeichnet; der Mantel ist als Hemd, eingeschlossene Luft und wasserabweisende Hülle modelliert.")
N['anchors']=anchors_head(N['anchors'],0.2639)
regs=[]
regs.append(region("head-skin","Face and neck skin","Haut von Gesicht und Hals","skin","skin",0.082,0.15,"sphere",0.45,[],15000,"head",0.05,0.045,ref="dubois-1916",source="literature"))
regs.append(region("hair","Hair","Haar","hair","hair",0.073,0.15,"sphere",0.5,[layer("hair-pack","Hair and bun","Haar und Haarknoten","hair","hair",res=0.06,thick=0.01,cap=250,ref="iso-9920-2007")],4000,"head",0.01,0.04,ref="iso-9920-2007"))
regs.append(region("coat","Knee-length raincoat over a shirt","Knielanger Regenmantel über einem Hemd","clothing","rain-shell",1.47,0.3,"cylinder",0.45,
  [layer("shirt-under","Long-sleeve shirt (0.25 clo)","Langarmhemd (0,25 clo)","clothing","cotton-clothing",clo=0.25,thick=0.001,cap=200,ref="iso-9920-2007"),
   airgap("coat-air","Air inside the loose coat","Luft im weiten Mantel",0.04,"rain-shell",thick=0.012),
   layer("coat-shell","Water-repellent polyurethane-coated shell (0.45 clo)","Wasserabweisende, PU-beschichtete Hülle (0,45 clo)","shell","rain-shell",clo=0.45,thick=0.0006,cap=250,vap=30,ref="iso-9920-2007")],20000,"trunk",0.69,0.61,ref="iso-9920-2007"))
regs.append(region("torso-clothing","Shirt collar at the neck","Hemdkragen am Hals","clothing","cotton-clothing",0.037,0.08,"cylinder",0.5,
  [layer("collar","Shirt collar (0.25 clo)","Hemdkragen (0,25 clo)","clothing","cotton-clothing",clo=0.25,thick=0.001,cap=200,ref="iso-9920-2007")],12000,"trunk",0.0,0.005,ref="iso-9920-2007"))
regs.append(region("arm-clothing","Shirt cuffs under the coat sleeves","Hemdmanschetten unter den Mantelärmeln","clothing","cotton-clothing",0.018,0.07,"cylinder",0.5,
  [layer("cuff","Shirt cuff (0.25 clo)","Hemdmanschette (0,25 clo)","clothing","cotton-clothing",clo=0.25,thick=0.001,cap=200,ref="iso-9920-2007")],14000,"arm",0.0,0.01,ref="iso-9920-2007"))
regs.append(region("hand-skin","Hands (bare, cold)","Hände (nackt, kalt)","skin","skin",0.091,0.07,"cylinder",0.4,[],10000,"hand",0.03,0.05,ref="dubois-1916",source="literature"))
regs.append(region("leg-clothing","Trousers","Hose","clothing","cotton-clothing",0.378,0.12,"cylinder",0.35,
  [airgap("trouser-air","Air inside the trousers","Luft in der Hose",0.02,"cotton-clothing"),layer("trousers","Trousers (0.24 clo)","Hose (0,24 clo)","clothing","cotton-clothing",clo=0.24,thick=0.0012,cap=260,ref="iso-9920-2007")],18000,"leg",0.2,0.18,ref="iso-9920-2007"))
regs.append(region("shoes","Leather shoes with socks","Lederschuhe mit Socken","clothing","leather-shoe",0.128,0.12,"horizontal-plate-up",0.2,
  [layer("sock-layer","Wool sock (0.1 clo)","Wollsocke (0,1 clo)","clothing","cotton-clothing",clo=0.1,thick=0.003,cap=300,ref="iso-9920-2007"),airgap("shoe-air","Air in the shoe","Luft im Schuh",0.03,"leather-shoe",thick=0.008),layer("shoe-upper","Leather upper","Lederschaft","shell","leather-shoe",res=0.025,thick=0.003,cap=1200,ref="iso-9920-2007")],7000,"foot",0.02,0.06,ref="iso-9920-2007"))
regs.append(region("canopy","Wet umbrella canopy","Nasses Schirmdach","surface","umbrella-fabric",0.87,0.5,"horizontal-plate-up",0.7,[],600,model="passive",ref="incropera-2011"))
regs.append(region("umbrella-frame","Umbrella shaft, ribs and handle","Schirmschaft, -rippen und -griff","surface","umbrella-metal",0.08,0.05,"cylinder",0.5,[],2500,model="passive"))
pat=[breath_pattern("rest","Standing in the rain, nose breathing","Stehen im Regen, Nasenatmung",15,0.55,0.38,0.04,0.45,0.13,"nose","gupta-2010"),
     breath_pattern("walk","Walking against the wind","Gehen gegen den Wind",20,1.0,0.40,0.02,0.50,0.08,"nose","astrand-rodahl-2003"),
     breath_pattern("brace","Bracing the umbrella in a gust","Schirm in der Böe halten",24,1.1,0.42,0.0,0.52,0.06,"nose-and-mouth","astrand-rodahl-2003")]
N['thermal']={"environment":human_env(8,8,0.95,4,0,1.0,sky=5,ground=8),"baseMet":1.3,"body":physiology(70.46,172),"respiration":respiration(pat,31,0.95),"operating":[],"regions":regs,
 "emitters":breath_emitters(0.2639,s)+[plume_emitter("coat",0.4,("Warm air rising off the coat, blown away by the wind","Vom Wind verwehte Warmluft über dem Mantel"))],
 "assumptions":[
  T("DuBois skin area for 1.72 m and 70.46 kg. Clothing is a long-sleeve shirt (0.25 clo), the trapped air of a loose coat and a water-repellent shell (0.45 clo); ISO 9920 values with 1 clo = 0.155 m2K/W. The shell is vapour-tight (vapour resistance 30 m2 Pa/W, authored estimate: poor breathability).","DuBois-Hautfläche für 1,72 m und 70,46 kg. Die Kleidung besteht aus Langarmhemd (0,25 clo), der eingeschlossenen Luft eines weiten Mantels und einer wasserabweisenden Hülle (0,45 clo); ISO-9920-Werte mit 1 clo = 0,155 m2K/W. Die Hülle ist dampfdicht (Dampfwiderstand 30 m2 Pa/W, Autorenschätzung: schlechte Atmungsaktivität)."),
  T("Rain at 8 degrees Celsius, 95 % humidity, 4 m/s wind, overcast; wet coat and canopy surfaces (wettedness 0.1 to 0.45) cool by evaporation. Metabolic rate 1.2 to 2.2 met, Compendium and ISO 8996 classes 1 and 2.","Regen bei 8 Grad Celsius, 95 % Feuchte, 4 m/s Wind, bedeckt; nasse Mantel- und Schirmoberflächen (Benetzung 0,1 bis 0,45) kühlen durch Verdunstung. Stoffwechsel 1,2 bis 2,2 met, Compendium und ISO 8996 Klassen 1 und 2."),
  T("Breathing is nasal in the cold; only the strongest gust (24 breaths per minute, 1.1 L) adds the mouth. Exhaled air 31 degrees Celsius after Hoeppe (lower in cold air). All values are authored estimates guided by the cited sources.","Die Atmung erfolgt in der Kälte über die Nase; erst in der stärksten Böe (24 Atemzüge pro Minute, 1,1 L) kommt der Mund hinzu. Ausatemluft 31 Grad Celsius nach Hoeppe (in kalter Luft niedriger). Alle Werte sind Autorenschätzungen nach den genannten Quellen."),
  T("The canopy is a thin passive surface (0.87 m2, 600 J/(m2K) with its water film) and exchanges heat only with the wind and the wet pavement beneath; the wind load on it is the structural load of the poses and clip track.","Das Schirmdach ist eine dünne passive Fläche (0,87 m2, 600 J/(m2K) mit Wasserfilm) und tauscht Wärme nur mit Wind und nassem Pflaster darunter aus; die Windlast darauf ist die Tragwerkslast der Posen und der Clip-Spur.")]}
RG=regs; EM=["plume"]
spec={"umbrella-up-still":(1.3,"rest",0.2,0.5,0.5),"umbrella-tilted-to-wind":(1.9,"walk",0.4,0.5,0.3),"umbrella-furled":(1.2,"rest",0.1,0.5,0.6),"gust-light":(1.6,"walk",0.3,0.5,0.4),"gust-heavy":(2.2,"brace",0.45,0.5,0.2)}
for p in N['poses']:
    met,br,wet,fr,pl=spec[p['id']]
    reg={"coat":{"wettedness":wet},"canopy":{"wettedness":min(1.0,wet*2) if p['id']!="umbrella-furled" else 0.3},"leg-clothing":{"wettedness":round(wet/3,2)},"shoes":{"wettedness":round(wet/2,2),"contact":{"fraction":0.5,"tempC":8}}}
    bind_pose(p,RG,EM,met,br,reg=reg,extra_emit={"plume":pl})
    # canopy is only exposed to air when the umbrella is open; furled it is folded
    if p['id']=="umbrella-furled": p['thermal']['regions']['canopy']['exposedFraction']=0.35
for c in N['clips']:
    c['events']=[e for e in c['events'] if e['kind'] not in('inhale','exhale')]
    c['tracks']=[t for t in c['tracks'] if t['target']!="emitter:breath"]
    if c['id']=="rain-gust-sway":
        c['tracks']+=[{"target":"breathing","ease":"step","keys":[{"t":0,"v":0},{"t":2.7,"v":1},{"t":4.5,"v":2},{"t":6.2,"v":1},{"t":8,"v":0}]},
                      {"target":"emitter:plume","ease":"smooth","keys":[{"t":0,"v":0.5},{"t":4.8,"v":0.2},{"t":8,"v":0.5}]}]
        c['events'].append({"t":4.5,"kind":"thermal","title":T("Heavy gust: effort adds mouth breathing","Starke Böe: Anstrengung bringt Mundatmung hinzu")})
    if c['id']=="close-umbrella":
        c['tracks']+=[{"target":"breathing","ease":"step","keys":[{"t":0,"v":1},{"t":2.4,"v":0}]},
                      {"target":"region:canopy:exposedFraction","ease":"smooth","keys":[{"t":2.4,"v":1},{"t":4,"v":0.35}]}]
    c['events'].sort(key=lambda e:e['t'])
N['presentation']['defaultScale']=100; N['presentation']['viewpoints']=["elevation"]; N['completion']="authored"
# representations
def mk(den):
    if den==500: body=merged500(P,pose0['bones'],None)
    else: body=build_body(den,P)
    return body+person_details(den)+umbrella_shapes(den)
R={500:mk(500),200:mk(200),100:mk(100)}
drop={"torso","pelvis"}|{f"{p}-{sd}" for p in("upper-arm","forearm","thigh","shank","coat-skirt") for sd in "lr"}|{"coat-hood","coat-collar"}
keep50=[sh for sh in A1['geometry']['elevation'] if sh['id'] not in drop and sh['bone'] not in("umbrella","rib-front","rib-back")]
kit=[sh for sh in build_body(100,P) if sh['id'] in {"torso","pelvis"}|{f"{p}-{sd}" for p in("upper-arm","forearm","thigh","thigh-skin","shank") for sd in "lr"}]
v50=[{k:v for k,v in sh.items() if k!='lod'} for sh in keep50]+kit+umbrella_shapes(50)
v50.append(shp("coat-hood","head","body",4.5,True,0.55,P_(next(x for x in A1['geometry']['elevation'] if x['id']=='coat-hood')['d']),region="coat"))
v50.append(shp("coat-collar","head","body",4.6,True,0.58,P_(next(x for x in A1['geometry']['elevation'] if x['id']=='coat-collar')['d']),region="coat"))
R[50]=v50
# v1 hair strands and body-only details whose regions changed: nothing else to remap
HATCH=[{"id":"gores","title":T("Canopy gores","Schirmbahnen"),"pattern":"lines","angleDeg":75,"spacingMm":0.6,"penMm":0.1}]
N['representations']={
 "500":rep(500,R[500],DEC(
  silhouette=("A dark standing figure of thirteen articulated overlapping forms with a flat open umbrella above: the canopy is two thin lenses meeting at the hub, held by a single shaft line down to the hand.","Eine dunkle stehende Figur aus dreizehn gelenkig überlappenden Formen mit einem flach geöffneten Schirm darüber: Das Dach besteht aus zwei schlanken Linsen, die sich an der Nabe treffen, und wird von einer einzigen Schaftlinie bis zur Hand gehalten."),
  retained=("The 2.09 m height to the ferrule, the 1.02 m canopy span, the knee-length coat shape with its slightly flared skirt and the raised near arm that holds the shaft; both canopy halves stay bound to their rib bones so the umbrella still furls and tilts.","Die Höhe von 2,09 m bis zur Zwinge, die Dachspannweite von 1,02 m, die knielange Mantelform mit leicht ausgestelltem Schoß und der erhobene nahe Arm am Schaft; beide Dachhälften bleiben an ihre Rippenknochen gebunden, der Schirm lässt sich also weiter schließen und neigen."),
  merged=("Head, hood, coat body and pelvis are one path; sleeve and cuff are one forearm path with the hand; the trouser leg stays a shank form and the shoe its own form. Hub, ribs and rim tips vanish inside the canopy lenses.","Kopf, Kapuze, Mantelkörper und Becken sind ein Pfad; Ärmel und Manschette bilden mit der Hand einen Unterarmpfad; das Hosenbein bleibt eine Unterschenkelform und der Schuh eine eigene Form. Nabe, Rippen und Randspitzen verschwinden in den Dachlinsen."),
  omitted=("Handle crook (10 cm), ferrule, rib struts under the canopy, panel seams, face, hair bun, coat belt, hem line and rain spots.","Griffkrücke (10 cm), Zwinge, Rippenstreben unter dem Dach, Bahnennähte, Gesicht, Haarknoten, Mantelgürtel, Saumlinie und Regentropfen."),
  thinElements=("The 1.5 cm shaft is drawn as a single 0.15 mm line (7.5 cm real, a full pen width), the only thickening in the drawing; ribs are dropped because the canopy lens is only 0.2 mm thick.","Der 1,5 cm dicke Schaft ist als einzelne 0,15-mm-Linie gezeichnet (7,5 cm real, eine volle Federbreite), die einzige Verdickung der Zeichnung; Rippen entfallen, weil die Dachlinse nur 0,2 mm dick ist."),
  hatching=("None; canopy and figure are solid.","Keine; Dach und Figur sind flächig."),
  styles=("Solid black figure (tone 0), canopy slightly lighter (0.3) so that it reads as a separate roof; outline 0.15 mm.","Flächig schwarze Figur (Ton 0), Dach etwas heller (0,3), damit es als eigenes Dach liest; Kontur 0,15 mm."),
  overlays=("Canopy and frame keep their own regions, so a wet canopy shows its cooler surface above the figure; the figure forms mix coat, face and trousers by area.","Dach und Gestänge behalten eigene Regionen, ein nasses Dach zeigt also seine kühlere Oberfläche über der Figur; die Figurformen mischen Mantel, Gesicht und Hose nach Fläche."))),
 "200":rep(200,R[200],DEC(
  silhouette=("Coat as a long single mass with the hooded head above it, trousered legs below, one arm holding the shaft; the open canopy is two lenses with the rib struts visible underneath and a hooked handle.","Mantel als lange geschlossene Masse mit dem Kopf samt Kapuze darüber, Hosenbeine darunter, ein Arm am Schaft; das geöffnete Dach besteht aus zwei Linsen mit sichtbaren Rippenstreben darunter und einem Hakengriff."),
  retained=("Coat, coat skirt, trousers, shoes, hood, shirt collar and cuffs as separate masses; shaft, two rib struts, handle crook and both canopy lenses.","Mantel, Mantelschoß, Hose, Schuhe, Kapuze, Hemdkragen und Manschetten als getrennte Massen; Schaft, zwei Rippenstreben, Griffkrücke und beide Dachlinsen."),
  merged=("Hand and forearm form one sleeve segment; head and hair stay two masses but share their outline at the nape.","Hand und Unterarm bilden ein Ärmelsegment; Kopf und Haar bleiben zwei Massen, teilen aber die Kontur im Nacken."),
  omitted=("Hub, rim tips, ferrule, panel seams, coat belt and seam, rain spots, face and fingers.","Nabe, Randspitzen, Zwinge, Bahnennähte, Mantelgürtel und -naht, Regentropfen, Gesicht und Finger."),
  thinElements=("Shaft and handle are 0.13 mm strokes (2.6 cm real) instead of a 1.5 cm capsule; the rib struts are 0.13 mm lines because only two of the eight real ribs are visible in side view.","Schaft und Griff sind 0,13-mm-Striche (2,6 cm real) statt einer 1,5-cm-Kapsel; die Rippenstreben sind 0,13-mm-Linien, weil in der Seitenansicht nur zwei der acht echten Rippen sichtbar sind."),
  hatching=("No hatching; the dark canopy tone carries the wet fabric.","Keine Schraffur; der dunkle Dachton trägt den nassen Stoff."),
  styles=("Coat mid-grey, trousers dark, shoes near-black, canopy dark with 0.18 mm outline; detail lines 0.13 mm.","Mantel mittelgrau, Hose dunkel, Schuhe fast schwarz, Dach dunkel mit 0,18-mm-Kontur; Detaillinien 0,13 mm."),
  overlays=("Coat, trousers, shoes, face, hair, canopy and frame are separate shapes; shirt collar and cuffs appear as small forms so their regions are drawn too.","Mantel, Hose, Schuhe, Gesicht, Haar, Dach und Gestänge sind getrennte Formen; Hemdkragen und Manschetten erscheinen als kleine Formen, damit auch ihre Regionen gezeichnet werden."))),
 "100":rep(100,R[100],DEC(
  silhouette=("Coat with belt line and hem, hood down at the nape, sleeves with cuffs, mitten hand with thumb around the shaft, trousers and shoes; umbrella with canopy lenses, shaft, hub, crook handle, ferrule, rib struts and rim tips.","Mantel mit Gürtellinie und Saum, Kapuze im Nacken, Ärmel mit Manschetten, Fäustlingshand mit Daumen am Schaft, Hose und Schuhe; Schirm mit Dachlinsen, Schaft, Nabe, Krückgriff, Zwinge, Rippenstreben und Randspitzen."),
  retained=("Every construction part of the umbrella: shaft capsule, hub circle, ferrule, handle, two visible rib struts, rim tips and one seam per canopy half; coat belt and hem.","Jedes Konstruktionsteil des Schirms: Schaftkapsel, Nabenkreis, Zwinge, Griff, zwei sichtbare Rippenstreben, Randspitzen und eine Naht je Dachhälfte; Mantelgürtel und Saum."),
  merged=("Neck and head, coat torso and pelvis share outlines; the far hand is a plain mitten.","Hals und Kopf, Mantelrumpf und Becken teilen Konturen; die ferne Hand ist ein schlichter Fäustling."),
  omitted=("Eye, ear, mouth, hair strands, fingers, laces, soles, rain spots, furling strap.","Auge, Ohr, Mund, Haarsträhnen, Finger, Schnürung, Sohlen, Regentropfen, Raffband."),
  thinElements=("Shaft stays a 1.5 cm capsule (0.15 mm); ferrule and rim tips are small detail forms; panel seams are 0.13 mm lines.","Der Schaft bleibt eine 1,5-cm-Kapsel (0,15 mm); Zwinge und Randspitzen sind kleine Detailformen; Bahnennähte sind 0,13-mm-Linien."),
  hatching=("None: the canopy lens is only 1 mm thick on paper, so hatching would fall below the minimum spacing.","Keine: Die Dachlinse ist auf dem Papier nur 1 mm dick, Schraffur läge unter dem Mindestabstand."),
  styles=("0.2 mm outlines, 0.13 mm details and seams; coat mid-grey, canopy dark, trousers and shoes dark.","0,2-mm-Konturen, 0,13-mm-Details und -Nähte; Mantel mittelgrau, Dach dunkel, Hose und Schuhe dunkel."),
  overlays=("Canopy panels carry the canopy region, ribs and shaft the frame region; sleeves and skirt are separate coat forms over the trouser forms, so the layered clothing reads in thermal overlays.","Dachbahnen tragen die Dachregion, Rippen und Schaft die Gestängeregion; Ärmel und Schoß sind eigene Mantelformen über den Hosenformen, die Kleidungsschichten lesen sich also in Thermik-Überlagerungen."))),
 "50":rep(50,R[50],DEC(
  silhouette=("Full drawing: coat with hood, collar, belt, front seam, hem and rain spots over trousers with hem lines, sleeves with cuffs, hand with fingers and thumb, shoes with soles; umbrella with gored canopy, strap, ribs, hub, handle and ferrule.","Volle Zeichnung: Mantel mit Kapuze, Kragen, Gürtel, Frontnaht, Saum und Regentropfen über der Hose mit Saumlinien, Ärmel mit Manschetten, Hand mit Fingern und Daumen, Schuhe mit Sohlen; Schirm mit Bahnen, Raffband, Rippen, Nabe, Griff und Zwinge."),
  retained=("All v1 construction and clothing detail; the smooth-capped limb bodies replace the v1 square-ended limbs so knees and elbows stay closed in every pose.","Alle v1-Konstruktions- und Kleidungsdetails; die glatten, gerundeten Gliedmaßen ersetzen die v1-Glieder mit eckigen Enden, damit Knie und Ellbogen in jeder Pose geschlossen bleiben."),
  merged=("Coat skirt and trouser leg overlap with one hem line; hand and forearm are separate shapes.","Mantelschoß und Hosenbein überlappen mit einer Saumlinie; Hand und Unterarm sind getrennte Formen."),
  omitted=("Individual ribs beyond the two visible, stitching, fabric folds, hair strands beyond one stroke.","Einzelne Rippen über die zwei sichtbaren hinaus, Stiche, Stofffalten, Haarsträhnen über einen Strich hinaus."),
  thinElements=("Shaft 1.5 cm capsule (0.3 mm), rib struts 0.18 mm lines, panel seams and rain spots 0.13 mm.","Schaft 1,5-cm-Kapsel (0,3 mm), Rippenstreben 0,18-mm-Linien, Bahnennähte und Regentropfen 0,13 mm."),
  hatching=("Parallel hatching along the canopy gores (0.6 mm spacing, 0.1 mm pen, 75 degrees) for the water-proof fabric.","Parallelschraffur entlang der Schirmbahnen (0,6 mm Abstand, 0,1-mm-Feder, 75 Grad) für den wasserdichten Stoff."),
  styles=("Silhouettes 0.25 mm, seams and hems 0.18 mm, rain spots 0.13 mm; coat mid-grey, canopy dark with hatching.","Silhouetten 0,25 mm, Nähte und Säume 0,18 mm, Regentropfen 0,13 mm; Mantel mittelgrau, Dach dunkel mit Schraffur."),
  overlays=("Every region is a shape: coat parts, collar, cuffs, trousers, shoes, face, hair, hands, canopy and frame.","Jede Region ist eine Form: Mantelteile, Kragen, Manschetten, Hose, Schuhe, Gesicht, Haar, Hände, Dach und Gestänge.")),hatches=HATCH)}
N['symbols']=[]
N['exports']=A1['exports']
if __name__=="__main__":
    finish(N,out_path("person-with-umbrella")); print("written")
