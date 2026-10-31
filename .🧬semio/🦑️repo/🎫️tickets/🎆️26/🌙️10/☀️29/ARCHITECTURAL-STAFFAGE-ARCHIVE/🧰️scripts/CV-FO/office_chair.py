"""office-swivel-chair v2: side elevation of a five-star swivel chair; four scales; indoor passive thermal model."""
import sys
from lib import *
from common import *

path, A = load('furniture', 'office-swivel-chair')
rig = Rig(A['rig']['bones'])
PL, FA, ST, RU = 'chair-plastic', 'chair-fabric', 'chair-steel', 'castor-rubber'
KP, KF, KS, KR = (dict(region=PL, material='pa-plastic'), dict(region=FA, material='fabric-upholstery'), dict(region=ST, material='steel-painted'), dict(region=RU, material='rubber'))
WB, WS = lw(rig, 'backrest'), lw(rig, 'seat')
SEAT_Y = 0.445


def mapc(W, cmds):
    out = []
    for c in cmds:
        if c[0] == 'Z':
            out.append(c)
            continue
        v = []
        for i in range(1, len(c), 2):
            v += list(W(c[i], c[i + 1]))
        out.append((c[0], *v))
    return out


SHELL = [('M', .07, -.012), ('C', .20, -.046, .36, -.030, .60, -.014), ('C', .635, -.010, .65, .010, .62, .028), ('C', .40, .038, .22, .044, .08, .040), ('C', .06, .036, .06, 0, .07, -.012), ('Z',)]
CUSHION = [('M', -.20, -.016), ('C', -.23, -.014, -.24, .004, -.23, .02), ('C', -.22, .036, -.21, .04, -.19, .04), ('L', .17, .04), ('C', .21, .04, .232, .03, .232, .01), ('C', .232, -.008, .22, -.022, .19, -.022), ('L', -.18, -.022), ('Z',)]
STAR = [(-.243, .022), (-.243, .047), (-.04, .067), (.04, .067), (.30, .047), (.30, .022), (.28, .019), (.055, .027), (.055, .012), (.03, .005), (-.03, .005), (-.055, .012), (-.055, .027), (-.223, .019)]


def seat_parts(sh, fine):
    sh.path('seat-cushion', 'seat', mapc(WS, CUSHION), z=4, tone=0.62, **KF)
    sh.rect('seat-mechanism', 'seat', -.1, SEAT_Y - .062, .1, SEAT_Y - .02, r=.006 if fine else 0, z=3.9, tone=0.22, **KS)


def rep50():
    sh = Shapes(rig)
    skip = ('arm-side-b', 'arm-side-b-rib', 'arm-side-b-yoke', 'arm-side-b-stem', 'arm-back-b', 'arm-back-b-rib', 'arm-back-b-yoke', 'arm-back-b-stem')
    sk = list(skip) + [f'castor-{n}-{p}' for n in ('side-b', 'back-b') for p in ('wheel', 'hub', 'tread', 'axle')] + [f'castor-{n}-{p}' for n in ('front', 'side-a', 'back-a') for p in ('tread', 'axle')]
    sk += ['column-ring-0', 'column-ring-1', 'column-ring-2', 'column-ring-3', 'seat-lever-knob']
    sh.items = v1_shapes(A, skip=sk)
    for s in sh.items:
        i = s['id']
        if i.endswith('-wheel'):
            s['penMm'] = 0.3
        if i.endswith('-hub'):
            s.update(rx=0.008, ry=0.008, tone=0.2, filled=True)
        if i in ('seat-cushion', 'backrest-shell'):
            s['penMm'] = 0.35
        if i.startswith('arm-') and i.count('-') == 2 and not i.endswith(('rib', 'yoke', 'stem')):
            s['penMm'] = 0.35
        if i in ('column', 'backrest-strut', 'armrest-pad', 'armrest-post', 'seat-mechanism', 'column-sleeve', 'column-cone'):
            s['penMm'] = 0.35
    sh.ids = {s['id'] for s in sh.items}
    return rep(50, sh, dict(
        silhouette=T('Side elevation 11.9 x 22.0 mm: a five-star base projected into three visible arms on castors, a gas-lift column with sleeve and rod, an upholstered seat on its mechanism, a curved backrest on a strut and one armrest.',
                     'Seitenansicht 11,9 x 22,0 mm: ein Fünfstern-Fuß, auf drei sichtbare Arme mit Rollen projiziert, eine Gasfedersäule mit Hülse und Stange, ein gepolsterter Sitz auf seinem Mechanismus, eine gebogene Lehne auf Strebe und eine Armlehne.'),
        retained=T('Hub, arm ribs, castor yokes and stems, tyre and hub of each castor, sleeve with collar, rod with groove and cone, cushion with shell line, piping and fold, mechanism with lever, armrest post, pad and line, backrest strut, shell, pad edge, rim and texture.',
                   'Nabe, Armrippen, Rollengabeln und -schäfte, Reifen und Nabe jeder Rolle, Hülse mit Kragen, Stange mit Nut und Konus, Polster mit Schalenlinie, Biese und Falte, Mechanismus mit Hebel, Armlehnenpfosten, -pad und -linie, Lehnenstrebe, -schale, Polsterkante, Rand und Textur.'),
        merged=T('Nothing merged; the two far arms and castors coincide in this elevation with the near ones and are hidden, so no shape is drawn for them.', 'Nichts verschmolzen; die zwei fernen Arme und Rollen fallen in dieser Ansicht mit den nahen zusammen und sind verdeckt, für sie wird keine Form gezeichnet.'),
        omitted=T('Far arms and castors, castor tread ring and axle (0.4 and 0.16 mm, below the minimum feature), the three lower column rings (0.13 mm lines closer than 0.3 mm), the lever knob, the second armrest.', 'Ferne Arme und Rollen, Profilring und Achse der Rollen (0,4 und 0,16 mm, unter dem Mindestmaß), die drei unteren Säulenringe (0,13-mm-Linien enger als 0,3 mm), der Hebelknopf, die zweite Armlehne.'),
        thinElements=T('Arm ribs, yoke stems and the armrest post (0.012-0.025 m) are 0.25 mm detail strokes or narrow solids; the castor hub is a 0.32 mm dot; the rod (0.025 m) is a 0.5 mm capsule.', 'Armrippen, Gabelschäfte und Armlehnenpfosten (0,012-0,025 m) sind 0,25-mm-Detailstriche oder schmale Flächen; die Rollennabe ist ein 0,32-mm-Punkt; die Stange (0,025 m) ist eine 0,5 mm breite Kapsel.'),
        hatching=T('None; the fabric fold and backrest texture are single 0.13 mm strokes.', 'Keine; Stofffalte und Lehnentextur sind einzelne 0,13-mm-Striche.'),
        styles=T('Fabric and shell light-to-mid (0.5-0.62), plastic base 0.3, steel parts darker (0.15-0.25), tyres 0.2; wheel 0.3 mm, solids 0.35 mm.', 'Stoff und Schale hell bis mittel (0,5-0,62), Kunststofffuß 0,3, Stahlteile dunkler (0,15-0,25), Reifen 0,2; Rad 0,3 mm, Flächen 0,35 mm.'),
        overlays=T('Plastic base, arms, shell and armrest = chair-plastic; cushion and pad = chair-fabric; column, mechanism and stems = chair-steel; tyres and hubs = castor-rubber.', 'Kunststofffuß, Arme, Schale und Armlehne = chair-plastic; Polster und Pad = chair-fabric; Säule, Mechanismus und Schäfte = chair-steel; Reifen und Naben = castor-rubber.')))


def common_chair(sh, fine, rod_pen=None):
    sh.poly('base-star', 'base', STAR, z=2.5, tone=0.3, **KP)
    for n, x in (('front', .30), ('side-a', .093), ('back-a', -.243)):
        sh.ell(f'castor-{n}', f'castor-{n}', x, .025, .025, z=1, tone=0.2, **KR)
    sh.rect('column-sleeve', 'column-sleeve', -.028, .025, .028, .325, r=.004, z=3.4, tone=0.25, **KS)


def rep100():
    sh = Shapes(rig)
    common_chair(sh, True)
    sh.path('column-rod', 'column', [('M', -.0125, .325), ('L', .0125, .325), ('L', .0125, .40), ('L', .032, .46), ('L', -.032, .46), ('L', -.0125, .40), ('Z',)], z=3, tone=0.7, **KS)
    seat_parts(sh, True)
    sh.path('backrest-shell', 'backrest', mapc(WB, SHELL), z=4, tone=0.5, **KP)
    a, b = WB(0, .012), WB(.24, .026)
    sh.line('backrest-strut', 'backrest', [a, b], role='body', z=3.8, tone=0.22, **KS)
    sh.line('armrest-post', 'seat', [(-.035, SEAT_Y + .04), (-.035, SEAT_Y + .2)], role='detail', z=3, **KP)
    sh.cap('armrest-pad', 'seat', -.19, SEAT_Y + .215, .115, SEAT_Y + .215, .014, .018, z=3.1, tone=0.4, **KP)
    return rep(100, sh, dict(
        silhouette=T('Side elevation 5.9 x 11.0 mm: one base silhouette with three castor dots, a gas-lift column with a visible rod, an upholstered seat, a curved backrest on a stroke strut, an armrest.',
                     'Seitenansicht 5,9 x 11,0 mm: eine Fußsilhouette mit drei Rollenpunkten, eine Gasfedersäule mit sichtbarer Stange, ein gepolsterter Sitz, eine gebogene Lehne auf einem Strebenstrich, eine Armlehne.'),
        retained=T('Hub, front arm and near back arm as one star silhouette, three castor tyres, sleeve, rod with cone, cushion, mechanism, backrest shell, strut, armrest post and pad.', 'Nabe, vorderer Arm und naher hinterer Arm als eine Sternsilhouette, drei Rollenreifen, Hülse, Stange mit Konus, Polster, Mechanismus, Lehnenschale, Strebe, Armlehnenpfosten und -pad.'),
        merged=T('Hub, three arms and their yokes become one star silhouette; arms and castors are three separate shapes of the 0.5 mm wheels; the sleeve rings, collar, groove and cone details merge into plain rod and sleeve; the near side arm lies inside the star.',
                 'Nabe, drei Arme und ihre Gabeln werden eine Sternsilhouette; Arme und Rollen sind drei getrennte 0,5-mm-Räder; Ringe, Kragen, Nut und Konusdetails der Hülse gehen in schlichter Stange und Hülse auf; der nahe Seitenarm liegt im Stern.'),
        omitted=T('Far arms and castors, hub ring, castor hubs, yoke stems, shell line, piping, fold, lever, rim, pad edge, texture.', 'Ferne Arme und Rollen, Nabenring, Rollennaben, Gabelschäfte, Schalenlinie, Biese, Falte, Hebel, Rand, Polsterkante, Textur.'),
        thinElements=T('The 0.025 m strut is a single 0.35 mm stroke; the armrest post (0.02 m) a single 0.18 mm stroke; the 0.03 m arm tips (0.3 mm) stay inside the star silhouette; wheels are 0.5 mm discs, not rings.', 'Die 0,025 m starke Strebe ist ein einzelner 0,35-mm-Strich; der Armlehnenpfosten (0,02 m) ein einzelner 0,18-mm-Strich; die 0,03 m dicken Armspitzen (0,3 mm) bleiben in der Sternsilhouette; Räder sind 0,5-mm-Scheiben, keine Ringe.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Base 0.3, tyres 0.2, steel 0.25-0.7, fabric 0.62, shell 0.5, pad 0.4; body pen 0.35, detail 0.18.', 'Fuß 0,3, Reifen 0,2, Stahl 0,25-0,7, Stoff 0,62, Schale 0,5, Pad 0,4; Körperstift 0,35, Detail 0,18.'),
        overlays=T('Star and armrest = chair-plastic, cushion = chair-fabric, sleeve, rod, mechanism and strut = chair-steel, wheels = castor-rubber; the far castors are not drawn.', 'Stern und Armlehne = chair-plastic, Polster = chair-fabric, Hülse, Stange, Mechanismus und Strebe = chair-steel, Räder = castor-rubber; die fernen Rollen werden nicht gezeichnet.')))


def rep200():
    sh = Shapes(rig)
    stubs = [(-.243, .0), (.093, .0), (.30, .0)]
    sh.poly('base-star', 'base', [(-.218, .03), (-.055, .027), (-.03, .005), (.03, .005), (.055, .027), (.275, .03), (.30, .047), (.04, .067), (-.04, .067), (-.243, .047)], z=2.5, tone=0.3, **KP)
    for n, x in (('front', .30), ('side-a', .093), ('back-a', -.243)):
        sh.ell(f'castor-{n}', f'castor-{n}', x, .025, .025, z=1, tone=0.2, **KR)
    sh.rect('column-sleeve', 'column-sleeve', -.03, .06, .03, .325, z=3.4, tone=0.25, **KS)
    sh.line('column-rod', 'column', [(0, .31), (0, .45)], role='body', z=3, tone=0.7, **KS)
    sh.path('seat-cushion', 'seat', mapc(WS, [('M', -.23, .0), ('L', -.23, .035), ('L', .232, .035), ('L', .232, .0), ('Z',)]), z=4, tone=0.62, **KF)
    sh.path('backrest-shell', 'backrest', mapc(WB, [('M', .07, -.03), ('C', .2, -.046, .4, -.035, .62, -.014), ('L', .63, .03), ('C', .4, .04, .22, .044, .08, .040), ('Z',)]), z=4, tone=0.5, **KP)
    sh.line('backrest-strut', 'backrest', [WB(0, .012), WB(.1, .016)], role='body', z=3.8, tone=0.22, **KS)
    sh.line('armrest', 'seat', [(-.035, SEAT_Y + .04), (-.035, SEAT_Y + .215), (.1, SEAT_Y + .215)], role='detail', z=3.1, tone=0.4, **KP)
    return rep(200, sh, dict(
        silhouette=T('Side elevation 3.0 x 5.5 mm: a base with three castor feet, a hairline-thin column, a seat band, a curved backrest shell and a single armrest stroke.', 'Seitenansicht 3,0 x 5,5 mm: ein Fuß mit drei Rollenfüßen, eine feine Säule, ein Sitzband, eine gebogene Lehnenschale und ein einzelner Armlehnenstrich.'),
        retained=T('Overall size, the dome of the base with its three castor positions, column, seat height, backrest curve, armrest height.', 'Gesamtgröße, die Wölbung des Fußes mit seinen drei Rollenpositionen, Säule, Sitzhöhe, Lehnenkrümmung, Armlehnenhöhe.'),
        merged=T('Hub and arms form one closed base shape; the three castors are 0.25 mm discs below it; cushion, mechanism and shell line become one seat band; the strut is only a short stub at the backrest foot.', 'Nabe und Arme bilden eine geschlossene Fußform; die drei Rollen sind 0,25-mm-Scheiben darunter; Polster, Mechanismus und Schalenlinie werden ein Sitzband; die Strebe ist nur ein kurzer Stummel am Lehnenfuß.'),
        omitted=T('Wheel hubs and tyres as such, yokes, rod cone, collar, rings, mechanism box, lever, piping, texture, pad.', 'Rollennaben und Reifen als solche, Gabeln, Stangenkonus, Kragen, Ringe, Mechanismuskasten, Hebel, Biese, Textur, Pad.'),
        thinElements=T('The 0.056 m sleeve is a 0.3 mm solid and the 0.025 m rod a single 0.25 mm stroke; castors (0.05 m = 0.25 mm) are plain discs without hub; the armrest (0.03 m) is one 0.13 mm line.', 'Die 0,056 m starke Hülse ist eine 0,3 mm breite Fläche und die 0,025 m starke Stange ein einzelner 0,25-mm-Strich; Rollen (0,05 m = 0,25 mm) sind schlichte Scheiben ohne Nabe; die Armlehne (0,03 m) ist eine 0,13-mm-Linie.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Base 0.3, seat 0.62, shell 0.5, sleeve 0.25, strokes black; body pen 0.25, detail 0.13.', 'Fuß 0,3, Sitz 0,62, Schale 0,5, Hülse 0,25, Striche schwarz; Körperstift 0,25, Detail 0,13.'),
        overlays=T('Base and shell = chair-plastic, seat = chair-fabric, column = chair-steel, discs = castor-rubber.', 'Fuß und Schale = chair-plastic, Sitz = chair-fabric, Säule = chair-steel, Scheiben = castor-rubber.')))


def rep500():
    sh = Shapes(rig)
    sh.rect('base-bar', 'base', -.243, .02, .30, .05, z=2.5, tone=0.3, **KP)
    for n, x in (('front', .30), ('side-a', .093), ('back-a', -.243)):
        sh.ell(f'castor-{n}', f'castor-{n}', x, .028, .028, z=1, tone=0.2, **KR)
    sh.line('column', 'column', [(0, .03), (0, .445)], role='body', z=3, tone=0.4, **KS)
    sh.rect('seat-band', 'seat', -.24, SEAT_Y - .055, .232, SEAT_Y + .045, z=4, tone=0.62, **KF)
    sh.line('backrest', 'backrest', [WB(.06, 0), WB(.635, 0)], role='body', z=4, tone=0.5, **KP)
    return rep(500, sh, dict(
        silhouette=T('Composed chair sign 1.2 x 2.2 mm: a base bar spanning the star, a column line, a seat bar and a backrest line rising to the full 1.08 m.', 'Zusammengesetztes Stuhlzeichen 1,2 x 2,2 mm: ein Fußbalken über die Sternbreite, eine Säulenlinie, ein Sitzbalken und eine bis zur vollen Höhe 1,08 m reichende Lehnenlinie.'),
        retained=T('Overall width of the star (0.59 m), seat height, seat width, backrest height and its position behind the column.', 'Gesamtbreite des Sterns (0,59 m), Sitzhöhe, Sitzbreite, Lehnenhöhe und ihre Lage hinter der Säule.'),
        merged=T('Hub and arms become one base bar with three castor dots; cushion, mechanism and armrests are absorbed into the seat bar; strut and shell become one backrest line.', 'Nabe und Arme werden ein Fußbalken mit drei Rollenpunkten; Polster, Mechanismus und Armlehnen gehen im Sitzbalken auf; Strebe und Schale werden eine Lehnenlinie.'),
        omitted=T('Castor tyre and hub detail, armrests, rod/sleeve difference, all detail; the sign has no armrest although the chair has one.', 'Reifen- und Nabendetail der Rollen, Armlehnen, Unterschied Stange/Hülse, alle Details; das Zeichen hat keine Armlehne, obwohl der Stuhl eine hat.'),
        thinElements=T('The seat bar is thickened from 0.062 to 0.1 m (0.2 mm minimum feature, within the pen tolerance); the 0.03 m arms, the 0.056 m sleeve and the 0.05 m shell are single 0.18 mm strokes (90 mm real line weight); they are not thickened further and not drawn as outlines.', 'Der Sitzbalken wird von 0,062 auf 0,1 m verstärkt (0,2 mm Mindestmaß, innerhalb der Stifttoleranz); die 0,03 m dicken Arme, die 0,056 m starke Hülse und die 0,05 m starke Schale sind einzelne 0,18-mm-Striche (90 mm reale Linienstärke); sie werden nicht weiter verstärkt und nicht als Konturen gezeichnet.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Seat bar mid tone, all strokes black at 0.18 mm; ground 0.25 mm.', 'Sitzbalken mittlerer Ton, alle Striche schwarz in 0,18 mm; Boden 0,25 mm.'),
        overlays=T('Base bar and backrest = chair-plastic, seat bar = chair-fabric, column = chair-steel (line colours); the castor dots are castor-rubber.', 'Fußbalken und Lehne = chair-plastic, Sitzbalken = chair-fabric, Säule = chair-steel (Linienfarben); die Rollenpunkte sind castor-rubber.')))


def thermal():
    pa, ru, st = MAT['pa'], MAT['rubber'], MAT['steel']
    c_pa, c_st, c_ru = effective_cap(pa['rho'], pa['cp'], pa['k'], 0.004), effective_cap(st['rho'], st['cp'], st['k'], 0.003), effective_cap(ru['rho'], ru['cp'], ru['k'], 0.008)
    foam = dict(rho=40, cp=1400, k=0.04)
    c_foam = effective_cap(foam['rho'], foam['cp'], foam['k'], 0.04)
    regions = [
        region(PL, T('Plastic base, arms and backrest shell', 'Kunststofffuß, Arme und Rückenschale'), 'surface', 'passive', 'pa-plastic', 0.35, 0.3, 'vertical-plate', 0.4,
               [layer('pa-shell', T('Glass-fibre polyamide shell 4 mm', 'Polyamid-Schale 4 mm'), 'polymer', 'pa-plastic', 0.004, pa['k'], c_pa, reference='iso-10456-2007')], c_pa, reference='iso-10456-2007'),
        region(FA, T('Upholstered seat and backrest pad', 'Gepolsterter Sitz und Lehnenpolster'), 'surface', 'passive', 'fabric-upholstery', 0.4, 0.4, 'horizontal-plate-up', 0.4,
               [layer('fabric', T('Woven fabric', 'Webstoff'), 'polymer', 'fabric-upholstery', 0.001, 0.05, 150, source='authored-estimate'), layer('foam', T('Flexible PU foam 40 mm', 'PU-Weichschaum 40 mm'), 'insulation', 'fabric-upholstery', 0.04, foam['k'], c_foam, source='authored-estimate')], c_foam + 150),
        region(ST, T('Steel gas-lift column and mechanism', 'Stahl-Gasfedersäule und Mechanik'), 'surface', 'passive', 'steel-painted', 0.1, 0.044, 'cylinder', 0.4,
               [layer('tube', T('Steel tube 3 mm', 'Stahlrohr 3 mm'), 'metal', 'steel-painted', 0.003, st['k'], c_st, reference='iso-10456-2007')], c_st, reference='iso-10456-2007'),
        region(RU, T('Castor tyres', 'Rollenreifen'), 'tyre', 'passive', 'rubber', 0.02, 0.05, 'cylinder', 0.3,
               [layer('tyre', T('Soft rubber tread 8 mm', 'Weiche Gummilauffläche 8 mm'), 'polymer', 'rubber', 0.008, ru['k'], c_ru, reference='iso-10456-2007')], c_ru, reference='iso-10456-2007')]
    A['thermal'] = {'environment': {'airTempC': 22, 'meanRadiantTempC': 22, 'relativeHumidity': 0.5, 'windMS': 0.1, 'windDirectionDeg': 180, 'solarWM2': 0, 'diffuseFraction': 1.0, 'pressurePa': 101325},
                    'operating': [], 'regions': regions, 'emitters': [], 'assumptions': [
        T('Indoor reference case: no sun, 22 degC air and radiant temperature, still air; passive surfaces with layers and heat capacity, no emitters, no authored temperatures.', 'Innenraum-Referenzfall: keine Sonne, 22 degC Luft- und Strahlungstemperatur, ruhige Luft; passive Flächen mit Schichten und Wärmekapazität, keine Emitter, keine festgelegten Temperaturen.'),
        T('The seat is the only region that meets a person: 40 mm flexible PU foam (conductivity 0.04 W/mK, authored estimate) under woven fabric gives a high resistance and a small heat capacity, so an occupied seat warms its surface within minutes (not modelled in these poses, which are empty).', 'Der Sitz ist die einzige Region mit Körperkontakt: 40 mm PU-Weichschaum (Leitfähigkeit 0,04 W/mK, Schätzwert) unter Webstoff ergibt hohen Widerstand und kleine Wärmekapazität, sodass ein besetzter Sitz seine Oberfläche binnen Minuten erwärmt (in diesen leeren Posen nicht abgebildet).'),
        T('Heat capacities use rho*cp*min(thickness, sqrt(alpha*3600 s)); conductivity, cp and density of polyamide, steel and rubber after ISO 10456.', 'Wärmekapazitäten nach rho*cp*min(Dicke, sqrt(alpha*3600 s)); Leitfähigkeit, cp und Dichte von Polyamid, Stahl und Gummi nach ISO 10456.'),
        T('The five castors are summarised in one region of 0.02 m2; conduction into the floor through five 15 x 10 mm contacts is neglected.', 'Die fünf Rollen sind in einer Region von 0,02 m2 zusammengefasst; die Leitung in den Boden über fünf Kontakte von 15 x 10 mm wird vernachlässigt.')]}


def bind():
    return binding({PL: (1.0, 0.0), FA: (1.0, 0.0), ST: (0.9, 0.0), RU: (0.8, 0.0)})


def finish():
    A['category'] = 'furniture'
    A['provenance']['author'] = 'Semio staffage content (CV-FO)'
    A['provenance']['references'] = [REF_ASHBY, REF_EN1991, REF_ISO10456, REF_INCROPERA]
    A['dimensions'] = {'heightM': 1.08, 'widthM': 0.6, 'depthM': 0.6, 'massKg': 15.4}
    A['materials'] = [
        {'id': 'pa-plastic', 'title': T('Glass-fibre polyamide', 'Glasfaserverstärktes Polyamid'), 'emissivity': 0.9, 'solarAbsorptivity': 0.6, 'densityKgM3': 1150, 'conductivityWmK': 0.25, 'specificHeatJKgK': 1600, 'source': 'literature', 'reference': 'iso-10456-2007'},
        {'id': 'steel-painted', 'title': T('Painted steel', 'Lackierter Stahl'), 'emissivity': 0.9, 'solarAbsorptivity': 0.6, 'densityKgM3': 7850, 'conductivityWmK': 50, 'specificHeatJKgK': 450, 'source': 'authored-estimate', 'reference': 'iso-10456-2007'},
        {'id': 'fabric-upholstery', 'title': T('Upholstery fabric on PU foam', 'Polsterstoff auf PU-Schaum'), 'emissivity': 0.9, 'solarAbsorptivity': 0.6, 'densityKgM3': 40, 'conductivityWmK': 0.04, 'specificHeatJKgK': 1400, 'source': 'authored-estimate'},
        {'id': 'rubber', 'title': T('Soft castor rubber', 'Weiches Rollengummi'), 'emissivity': 0.9, 'solarAbsorptivity': 0.85, 'densityKgM3': 1150, 'conductivityWmK': 0.25, 'specificHeatJKgK': 1000, 'source': 'literature', 'reference': 'iso-10456-2007'}]
    A['representations'] = {'500': rep500(), '200': rep200(), '100': rep100(), '50': rep50()}
    A['symbols'] = []
    A.pop('geometry', None)
    thermal()
    for p in A['poses']:
        p['thermal'] = bind()
    A['structure']['assumptions'] = A['structure']['assumptions'][:1] + [T('The swivel about the vertical axis is not visible in this elevation (it is shown by motion events only); the two far arms and castors coincide with the near ones.', 'Die Drehung um die Hochachse ist in dieser Ansicht nicht sichtbar (nur durch Bewegungsereignisse dargestellt); die zwei fernen Arme und Rollen fallen mit den nahen zusammen.')]
    A['presentation']['defaultScale'] = 50
    A['completion'] = 'authored'
    return sorted_keys(A)


if __name__ == '__main__':
    out = finish()
    for k in ('500', '200', '100', '50'):
        print(k, [round(v, 4) for v in bbox_world(rig, out['representations'][k]['geometry']['elevation'])])
    if '--write' in sys.argv:
        save(path, out)
    for k in ('500', '200', '100', '50'):
        render(out['representations'][k], rig, int(k), f'office-{k}-print', dpi=600)
        render(out['representations'][k], rig, int(k), f'office-{k}-zoom', dpi=600, zoom=8 if k == '500' else 5 if k == '200' else 3)
