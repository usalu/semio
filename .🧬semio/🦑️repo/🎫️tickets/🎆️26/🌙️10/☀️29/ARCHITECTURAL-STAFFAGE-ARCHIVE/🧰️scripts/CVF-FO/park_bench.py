"""park-bench v2: front elevation, four individually authored scales, passive sun/shade thermal model."""
import sys
from lib import *
from common import *

path, A = load('furniture', 'park-bench')
rig = Rig(A['rig']['bones'])
HATCH = [{'id': 'oak-grain', 'title': T('Oak grain', 'Eichenmaserung'), 'pattern': 'lines', 'angleDeg': 0, 'spacingMm': 0.6, 'penMm': 0.13}]
R_SEAT, R_BACK, R_STEEL = 'seat-timber', 'back-timber', 'steel-frame'
FRAMES = [('l', 'end-left', 0.12), ('r', 'end-right', 1.68)]


def frame50(sh):
    for p, b, c in FRAMES:
        kw = dict(region=R_STEEL, material='steel-painted')
        sh.path(f'{p}-foot', b, [('M', c - .062, 0), ('L', c + .062, 0), ('L', c + .062, .012), ('C', c + .062, .034, c + .028, .05, c + .024, .13), ('L', c - .024, .13),
                                 ('C', c - .028, .05, c - .062, .034, c - .062, .012), ('Z',)], z=2, tone=0.35, **kw)
        sh.path(f'{p}-leg', b, [('M', c - .024, .12), ('L', c + .024, .12), ('C', c + .022, .22, c + .02, .30, c + .026, .375), ('L', c - .026, .375),
                                ('C', c - .02, .30, c - .022, .22, c - .024, .12), ('Z',)], z=3, tone=0.4, **kw)
        sh.rect(f'{p}-brace', b, c - .03, .19, c + .03, .212, r=.004, role='detail', z=2.2, tone=0.5, **kw)
        sh.line(f'{p}-web-rim', b, [(c - .012, .15), (c - .011, .36)], z=3.2, **kw)
        sh.line(f'{p}-web-rim-b', b, [(c + .012, .15), (c + .011, .36)], z=3.2, **kw)
        sh.line(f'{p}-foot-bead', b, [(c - .062, .016), (c + .062, .016)], z=3.2, **kw)
        sh.rect(f'{p}-bearer', b, c - .04, .37, c + .04, .406, r=.004, z=2.5, tone=0.45, **kw)
        sh.cap(f'{p}-post', b, c, .40, c, .8285, .0155, .0135, z=3, tone=0.4, **kw)
        sh.ell(f'{p}-finial', b, c, .8325, .0185, .0175, z=3.1, tone=0.38, **kw)
        sh.rect(f'{p}-arm-post', b, c - .016, .40, c + .016, .60, role='detail', z=4.9, tone=0.45, **kw)
        sh.rect(f'{p}-armrest', b, c - .045, .60, c + .045, .634, r=.013, z=5, tone=0.42, **kw)
        sh.path(f'{p}-arm-scroll', b, [('M', c - .04, .612), ('C', c - .04, .585, c - .016, .58, c - .016, .55)], role='detail', z=5.1, filled=False, tone=0.3, **kw)
        for i, (bx, by) in enumerate([(c - .05, .008), (c + .05, .008), (c, .388)]):
            sh.ell(f'{p}-bolt-{i}', b, bx, by, .0045, role='detail', z=3.3, tone=0.15, **kw)


def timber50(sh):
    kw = dict(material='oak-timber')
    sh.rect('seat-front', 'seat', 0, .405, 1.8, .45, r=.008, z=4, tone=0.6, region=R_SEAT, hatch='oak-grain', penMm=0.35, **kw)
    sh.line('seat-chamfer', 'seat', [(.012, .441), (1.788, .441)], z=4.15, region=R_SEAT, **kw)
    for i, (y0, y1) in enumerate([(.52, .60), (.63, .71), (.74, .82)], 1):
        n = f'back-slat-{i}'
        sh.rect(n, 'backrest', 0, y0, 1.8, y1, r=.01, z=4, tone=0.55, region=R_BACK, hatch='oak-grain', penMm=0.35, **kw)
        sh.line(n + '-chamfer', 'backrest', [(.012, y1 - .008), (1.788, y1 - .008)], z=4.1, region=R_BACK, **kw)
        sh.line(n + '-chamfer-b', 'backrest', [(.012, y0 + .008), (1.788, y0 + .008)], z=4.1, region=R_BACK, **kw)
        for j, bx in enumerate([.105, .135, 1.665, 1.695]):
            sh.ell(f'{n}-bolt-{j}', 'backrest', bx, (y0 + y1) / 2, .0038, role='detail', z=4.3, tone=0.15, region=R_STEEL, material='steel-painted')


def rep50():
    sh = Shapes(rig)
    frame50(sh)
    timber50(sh)
    return rep(50, sh, dict(
        silhouette=T('Full front elevation: two cast-style end frames carrying a seat slat and three back slats that overhang the frames by 0.12 m; reads as a 0.85 x 1.80 m bench (17.0 x 36.0 mm).',
                     'Vollständige Vorderansicht: zwei Endgestelle in Gussoptik tragen Sitzlatte und drei Rückenlatten, die 0,12 m über die Gestelle auskragen; wirkt als Bank 0,85 x 1,80 m (17,0 x 36,0 mm).'),
        retained=T('Foot plate with bead, tapered leg with web rims, brace, bearer, back post with finial, arm post, scrolled armrest, bolts; each back slat separately with both chamfer lines and four carriage-bolt heads; front seat slat with chamfer.',
                   'Fußplatte mit Wulst, verjüngtes Bein mit Stegrändern, Strebe, Auflager, Rückenpfosten mit Knauf, Armlehnenpfosten, Armlehne mit Volute, Bolzen; jede Rückenlatte einzeln mit beiden Fasen und vier Schlossschrauben; vordere Sitzlatte mit Fase.'),
        merged=T('Nothing merged; the slat gaps (0.03 m = 0.6 mm) stay open because the slats are drawn with the lighter 0.35 mm pen.', 'Nichts verschmolzen; die Lattenfugen (0,03 m = 0,6 mm) bleiben offen, weil die Latten mit dem leichteren 0,35-mm-Stift gezeichnet sind.'),
        omitted=T('Seat slats behind the front slat and the seat rail (invisible in front view); a plan view would show them.', 'Die Sitzlatten hinter der vorderen Latte und die Sitzzarge (in der Vorderansicht unsichtbar); eine Draufsicht würde sie zeigen.'),
        thinElements=T('Web rims and bead are single detail strokes (0.25 mm = 0.0125 m); bolts are 0.9 mm discs; the arm scroll is one curved stroke.', 'Stegränder und Wulst sind einzelne Detailstriche (0,25 mm = 0,0125 m); Bolzen sind 0,9-mm-Scheiben; die Voluten sind je ein gebogener Strich.'),
        hatching=T('Oak grain as horizontal lines every 0.6 mm (0.03 m) on the four slats only (above the 0.5 mm hatch minimum); steel is plain tone.', 'Eichenmaserung als waagrechte Linien im Abstand 0,6 mm (0,03 m) nur auf den vier Latten (über dem Mindestabstand 0,5 mm); Stahl nur als Tonfläche.'),
        styles=T('Timber light (tone 0.55-0.6), steel darker (0.35-0.45); bolts near black; line weight 0.5 / 0.35 slats / 0.25 detail.', 'Holz hell (Ton 0,55-0,6), Stahl dunkler (0,35-0,45); Bolzen fast schwarz; Strichstärke 0,5 / 0,35 Latten / 0,25 Detail.'),
        overlays=T('Every shape carries one region (seat-timber, back-timber, steel-frame); sun/shade and surface temperature colour slats and frames separately, bolts follow the steel.', 'Jede Form trägt genau eine Region (seat-timber, back-timber, steel-frame); Sonne/Schatten und Oberflächentemperatur färben Latten und Gestelle getrennt, Bolzen folgen dem Stahl.')),
        hatches=HATCH)


def rep100():
    sh = Shapes(rig)
    for p, b, c in FRAMES:
        kw = dict(region=R_STEEL, material='steel-painted')
        sh.path(f'{p}-foot-leg', b, [('M', c - .062, 0), ('L', c + .062, 0), ('L', c + .062, .012), ('C', c + .062, .034, c + .03, .06, c + .026, .13), ('L', c + .026, .375), ('L', c - .026, .375),
                                     ('L', c - .026, .13), ('C', c - .03, .06, c - .062, .034, c - .062, .012), ('Z',)], z=2, tone=0.4, **kw)
        sh.rect(f'{p}-bearer', b, c - .04, .37, c + .04, .406, z=2.5, tone=0.45, **kw)
        sh.cap(f'{p}-post', b, c, .40, c, .8285, .015, .0135, z=3, tone=0.4, **kw)
        sh.ell(f'{p}-finial', b, c, .8325, .0185, .0175, z=3.1, tone=0.38, **kw)
        sh.rect(f'{p}-armrest', b, c - .045, .60, c + .045, .634, r=.012, z=5, tone=0.42, **kw)
        sh.line(f'{p}-arm-post', b, [(c - .016, .40), (c - .016, .60)], z=4.9, **kw)
        sh.line(f'{p}-arm-post-b', b, [(c + .016, .40), (c + .016, .60)], z=4.9, **kw)
        sh.line(f'{p}-brace', b, [(c - .026, .20), (c + .026, .20)], z=2.2, **kw)
    sh.rect('seat-front', 'seat', 0, .405, 1.8, .45, r=.008, z=4, tone=0.6, region=R_SEAT, material='oak-timber')
    sh.rect('back-band', 'backrest', 0, .52, 1.8, .82, r=.012, z=4, tone=0.55, region=R_BACK, material='oak-timber')
    for i, y in enumerate([.615, .725], 1):
        sh.line(f'back-joint-{i}', 'backrest', [(.012, y), (1.788, y)], z=4.1, region=R_BACK, material='oak-timber')
    for j, bx in enumerate([.12, 1.68]):
        for i, y in enumerate([.57, .67, .78]):
            sh.ell(f'bolt-{j}-{i}', 'backrest', bx, y, .0075, role='detail', z=4.3, tone=0.15, region=R_STEEL, material='steel-painted')
    return rep(100, sh, dict(
        silhouette=T('Front elevation 8.5 x 18.0 mm: a solid back band, a seat band beneath it with a clear 0.7 mm gap, two end frames with armrests and finials.',
                     'Vorderansicht 8,5 x 18,0 mm: ein geschlossenes Rückenband, darunter das Sitzband mit 0,7 mm Luft, zwei Endgestelle mit Armlehnen und Knäufen.'),
        retained=T('Frame foot-and-leg as one profile, bearer, post with finial, armrest, arm post as two lines, brace; two slat joints in the back band; six bolt discs; seat band with rounded ends.',
                   'Gestellfuß und Bein als ein Profil, Auflager, Pfosten mit Knauf, Armlehne, Armlehnenpfosten als zwei Linien, Strebe; zwei Lattenfugen im Rückenband; sechs Bolzenscheiben; Sitzband mit gerundeten Enden.'),
        merged=T('Three back slats become one band (slat gaps 0.03 m = 0.3 mm would close under the 0.35 mm outline); foot, leg and web rims become one profile.',
                 'Drei Rückenlatten werden zu einem Band (Lattenfugen 0,03 m = 0,3 mm würden unter der 0,35-mm-Kontur zulaufen); Fuß, Bein und Stegränder werden ein Profil.'),
        omitted=T('Foot bead, bearer and foot bolts, web rims, arm scroll, chamfers, slat grain, seat chamfer.', 'Fußwulst, Fußbolzen, Stegränder, Volute, Fasen, Lattenmaserung, Sitzfase.'),
        thinElements=T('The 0.032 m arm post is two 0.18 mm lines 0.32 mm apart (kept, not filled); the brace is a single stroke; the 0.048 m leg is drawn as a 0.52 mm solid.',
                       'Der 0,032 m starke Armlehnenpfosten sind zwei 0,18-mm-Linien im Abstand 0,32 mm (gehalten, nicht gefüllt); die Strebe ist ein Einzelstrich; das 0,048 m starke Bein ist ein 0,52 mm breites Vollprofil.'),
        hatching=T('None: oak grain would need lines every 0.3 mm, below the 0.5 mm hatch minimum.', 'Keine: die Maserung bräuchte Linien alle 0,3 mm, unter dem Hatch-Minimum von 0,5 mm.'),
        styles=T('Timber tone 0.55-0.6, steel 0.4; bolts dark; body pen 0.35, slat joints 0.18.', 'Holz Ton 0,55-0,6, Stahl 0,4; Bolzen dunkel; Körperstift 0,35, Lattenfugen 0,18.'),
        overlays=T('Back band stays one region (back-timber) with its joints inside; seat band is seat-timber; frames and bolts steel-frame; no merged regions.', 'Das Rückenband bleibt eine Region (back-timber) mit Fugen darin; das Sitzband ist seat-timber; Gestelle und Bolzen steel-frame; keine verschmolzenen Regionen.')))


def rep200():
    sh = Shapes(rig)
    for p, b, c in FRAMES:
        kw = dict(region=R_STEEL, material='steel-painted')
        sh.path(f'{p}-frame', b, [('M', c - .062, 0), ('L', c + .062, 0), ('L', c + .062, .012), ('C', c + .062, .04, c + .034, .06, c + .03, .13), ('L', c + .03, .37), ('L', c + .04, .37), ('L', c + .04, .406),
                                  ('L', c + .02, .406), ('L', c + .02, .60), ('L', c + .045, .60), ('L', c + .045, .634), ('L', c + .02, .634), ('L', c + .018, .83), ('L', c + .0185, .84), ('L', c + .012, .85), ('L', c - .012, .85), ('L', c - .0185, .84), ('L', c - .018, .83), ('L', c - .02, .634),
                                  ('L', c - .045, .634), ('L', c - .045, .60), ('L', c - .02, .60), ('L', c - .02, .406), ('L', c - .04, .406), ('L', c - .04, .37), ('L', c - .03, .37), ('L', c - .03, .13),
                                  ('C', c - .034, .06, c - .062, .04, c - .062, .012), ('Z',)], z=3, tone=0.4, **kw)
    sh.rect('seat', 'seat', 0, .405, 1.8, .45, z=4, tone=0.6, region=R_SEAT, material='oak-timber')
    sh.rect('back-band', 'backrest', 0, .52, 1.8, .82, z=4, tone=0.55, region=R_BACK, material='oak-timber')
    for i, y in enumerate([.615, .725], 1):
        sh.line(f'back-joint-{i}', 'backrest', [(.0, y), (1.8, y)], z=4.1, region=R_BACK, material='oak-timber')
    return rep(200, sh, dict(
        silhouette=T('Front elevation 4.3 x 9.0 mm: back band, seat band with 0.35 mm clear gap, two end-frame contours that rise above the back band to the finials.',
                     'Vorderansicht 4,3 x 9,0 mm: Rückenband, Sitzband mit 0,35 mm Luft, zwei Gestellkonturen, die über das Rückenband bis zu den Knäufen reichen.'),
        retained=T('Frame contour with foot flare, bearer step, armrest bulge and finial; back band with its two slat joints; seat band; ground line.', 'Gestellkontur mit Fußausstellung, Auflagerstufe, Armlehnenwulst und Knauf; Rückenband mit zwei Lattenfugen; Sitzband; Bodenlinie.'),
        merged=T('Each end frame (foot, leg, bearer, post, arm post, armrest, finial) is one closed contour; the three back slats are one band; the front seat slat is the seat band.', 'Jedes Endgestell (Fuß, Bein, Auflager, Pfosten, Armlehnenpfosten, Armlehne, Knauf) ist eine geschlossene Kontur; die drei Rückenlatten sind ein Band; die vordere Sitzlatte ist das Sitzband.'),
        omitted=T('Brace, web rims, foot bead, bolts, chamfers, scroll, grain.', 'Strebe, Stegränder, Fußwulst, Bolzen, Fasen, Volute, Maserung.'),
        thinElements=T('The 0.04 m post and arm post (0.2 mm) are not drawn as separate parts but as the narrow neck of the frame contour; the slat joints (0.03 m) are 0.13 mm detail lines instead of gaps.',
                       'Pfosten und Armlehnenpfosten (0,2 mm) sind keine eigenen Teile, sondern der schmale Hals der Gestellkontur; die Lattenfugen (0,03 m) sind 0,13-mm-Detaillinien statt Spalten.'),
        hatching=T('None at this scale.', 'Keine Schraffur in diesem Maßstab.'),
        styles=T('Timber tone 0.55-0.6, frames 0.4; body pen 0.25, joints 0.13.', 'Holz Ton 0,55-0,6, Gestelle 0,4; Körperstift 0,25, Fugen 0,13.'),
        overlays=T('Back band = back-timber, seat band = seat-timber, frames = steel-frame; bolts do not exist at this scale so the steel overlay colours only the frames.', 'Rückenband = back-timber, Sitzband = seat-timber, Gestelle = steel-frame; Bolzen gibt es in diesem Maßstab nicht, die Stahlfärbung betrifft nur die Gestelle.')))


def rep500():
    sh = Shapes(rig)
    for p, b, c in FRAMES:
        sh.poly(f'{p}-frame', b, [(c - .08, 0), (c + .08, 0), (c + .08, .85), (c - .08, .85)], z=2, tone=0.4, region=R_STEEL, material='steel-painted')
    sh.rect('seat-back-block', 'seat', 0, .405, 1.8, .82, z=4, tone=0.58, regions=[R_SEAT, R_BACK], material='oak-timber')
    return rep(500, sh, dict(
        silhouette=T('Composed silhouette 1.7 x 3.6 mm: one solid block (seat and back) on two legs; the open space under the seat (0.8 mm) stays clear.', 'Zusammengesetzte Silhouette 1,7 x 3,6 mm: ein Block (Sitz und Rückenlehne) auf zwei Beinen; der offene Raum unter der Sitzfläche (0,8 mm) bleibt frei.'),
        retained=T('Overall size, the block of seat and back, the two end frames reaching the finial height, the open leg space.', 'Gesamtgröße, der Block aus Sitz und Lehne, die zwei Endgestelle bis zur Knaufhöhe, der offene Beinraum.'),
        merged=T('Seat, 0.07 m air gap and three back slats become one block (the gap is 0.14 mm, under the 0.28 mm minimum); each end frame becomes one 0.16 m wide stem.', 'Sitz, 0,07 m Luft und drei Rückenlatten werden ein Block (die Fuge ist 0,14 mm, unter dem Minimum 0,28 mm); jedes Endgestell wird ein 0,16 m breiter Schaft.'),
        omitted=T('Armrests, foot flares, bolts, slat joints, bracing - all below 0.1 m.', 'Armlehnen, Fußverbreiterungen, Bolzen, Lattenfugen, Streben - alle unter 0,1 m.'),
        thinElements=T('The 0.05 m leg and the post are thickened to a 0.16 m stem (0.32 mm), which stays within the 0.09 m outline tolerance of the 0.18 mm pen on each side; no thin part is drawn as a hairline.', 'Das 0,05 m dünne Bein und der Pfosten werden zu einem 0,16 m breiten Schaft (0,32 mm) verstärkt, innerhalb der Konturtoleranz von 0,09 m des 0,18-mm-Stifts je Seite; kein dünnes Teil wird als Haarlinie gezeichnet.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Block mid tone (0.58), stems dark (0.4); one 0.18 mm outline for everything; ground 0.25 mm.', 'Block mittlerer Ton (0,58), Schäfte dunkel (0,4); eine 0,18-mm-Kontur für alles; Boden 0,25 mm.'),
        overlays=T('The block is a merged form listing seat-timber and back-timber (painted with their area-weighted value and labelled merged); the two stems are steel-frame.', 'Der Block ist eine verschmolzene Form mit seat-timber und back-timber (mit dem flächengewichteten Wert gefärbt und als verschmolzen gekennzeichnet); die zwei Schäfte sind steel-frame.')))


#region physical scenario
def thermal():
    oak, st = MAT['oak'], MAT['steel']
    t_slat, t_steel = 0.04, 0.01
    seat_cap, steel_cap = effective_cap(oak['rho'], oak['cp'], oak['k'], t_slat), effective_cap(st['rho'], st['cp'], st['k'], t_steel)
    lay_t = lambda tt: layer('oak-slat', T('Oak slat', 'Eichenlatte'), 'timber', 'oak-timber', tt, oak['k'], seat_cap, reference='iso-10456-2007')
    regions = [
        region(R_SEAT, T('Seat slat top', 'Sitzlatte oben'), 'surface', 'passive', 'oak-timber', 0.76, 0.42, 'horizontal-plate-up', 0.85, [lay_t(t_slat)], seat_cap, source='authored-estimate', reference='iso-10456-2007'),
        region(R_BACK, T('Backrest slats', 'Rückenlatten'), 'surface', 'passive', 'oak-timber', 0.54, 0.30, 'vertical-plate', 0.5, [lay_t(t_slat)], seat_cap, source='authored-estimate', reference='iso-10456-2007'),
        region(R_STEEL, T('Steel end frames', 'Stahl-Endgestelle'), 'surface', 'passive', 'steel-painted', 0.9, 0.05, 'vertical-plate', 0.5,
               [layer('cast-style-steel', T('Painted steel section', 'Lackierter Stahlquerschnitt'), 'metal', 'steel-painted', t_steel, st['k'], steel_cap, reference='iso-10456-2007')], steel_cap, source='authored-estimate', reference='iso-10456-2007'),
    ]
    A['thermal'] = {
        'environment': {'airTempC': 20, 'meanRadiantTempC': 20, 'relativeHumidity': 0.5, 'windMS': 1.0, 'windDirectionDeg': 180, 'solarWM2': 600, 'diffuseFraction': 0.15, 'pressurePa': 101325},
        'operating': [], 'regions': regions, 'emitters': [],
        'assumptions': [
            T('Passive timber and steel surfaces: absorbed sun, long-wave exchange, convection and conduction with heat capacity; no emitters and no authored temperatures.', 'Passive Holz- und Stahlflächen: absorbierte Sonne, Langwellenaustausch, Konvektion und Leitung mit Wärmekapazität; keine Emitter und keine festgelegten Temperaturen.'),
            T('Areal heat capacity is rho*cp*min(thickness, sqrt(alpha*3600 s)): the part of the section that takes part within one hour (oak 40 mm slat: 24 mm; steel section 10 mm: 10 mm). Conductivity, cp and density after ISO 10456.', 'Die flächenbezogene Wärmekapazität ist rho*cp*min(Dicke, sqrt(alpha*3600 s)): der Teil des Querschnitts, der binnen einer Stunde teilnimmt (Eiche 40 mm: 24 mm; Stahl 10 mm: 10 mm). Leitfähigkeit, cp und Dichte nach ISO 10456.'),
            T('Occupied poses cover part of the seat and back with a clothed body (contact surface at 30 degC, ISO 7730 clothing surface range) and shade it; the fractions are authored estimates for an adult (0.18 m2 on the seat, 0.06 m2 on the back).', 'Besetzte Posen bedecken einen Teil von Sitz und Lehne mit einem bekleideten Körper (Kontaktfläche 30 degC, Bekleidungsoberflächen nach ISO 7730) und beschatten ihn; die Anteile sind Schätzwerte für einen Erwachsenen (0,18 m2 am Sitz, 0,06 m2 an der Lehne).'),
            T('Heat flow into the ground through the four foot plates (0.1 percent of the area) is neglected.', 'Der Wärmestrom über die Fußplatten (0,1 Prozent der Fläche) in den Boden wird vernachlässigt.'),
            T('Reference case: 600 W/m2 sun, 20 degC air, 1 m/s breeze. Sunlit fractions are scenario data of each pose or clip.', 'Referenzfall: 600 W/m2 Sonne, 20 degC Luft, 1 m/s Wind. Besonnte Anteile sind Szenariodaten der jeweiligen Pose oder des Clips.')]}


def bind(sun_seat=1.0, sun_back=1.0, sun_steel=1.0, occ=0):
    c = {1: (0.24, 0.1), 2: (0.47, 0.2)}.get(occ)
    seat = {'exposedFraction': 1 - (c[0] if c else 0), 'sunlitFraction': sun_seat * (1 - (c[0] if c else 0))}
    back = {'exposedFraction': 1 - (c[1] if c else 0), 'sunlitFraction': sun_back * (1 - (c[1] if c else 0) * 3)}
    if c:
        seat['contact'] = {'fraction': c[0], 'tempC': 30}
        back['contact'] = {'fraction': c[1], 'tempC': 30}
    return binding({R_SEAT: seat, R_BACK: back, R_STEEL: (1.0, sun_steel)})


def finish():
    A['category'] = 'furniture'
    A['provenance']['author'] = 'Semio staffage content (CV-FO)'
    A['provenance']['references'] = [REF_WOOD, REF_EN1993, REF_EN1991, REF_ISO10456, REF_INCROPERA, REF_ISO7730]
    A['provenance']['notes'] = T('Front elevation along the bench length. Masses from nominal section x density; occupant masses are authored estimates (75 kg adult). Backrest drawn upright (tilt omitted). v2: four individually authored scales, layered passive thermal model.',
                                 'Vorderansicht entlang der Banklänge. Massen aus Nennquerschnitt x Dichte; Personenmassen sind Schätzungen (75 kg Erwachsene). Lehne aufrecht gezeichnet (Neigung weggelassen). v2: vier einzeln gezeichnete Maßstäbe, geschichtetes passives Thermikmodell.')
    A['materials'] = [
        {'id': 'oak-timber', 'title': T('Oak timber', 'Eichenholz'), 'emissivity': 0.9, 'solarAbsorptivity': 0.6, 'densityKgM3': 700, 'conductivityWmK': 0.18, 'specificHeatJKgK': 1600, 'elasticModulusPa': 11000000000, 'source': 'authored-estimate', 'reference': 'iso-10456-2007'},
        {'id': 'steel-painted', 'title': T('Painted steel', 'Lackierter Stahl'), 'emissivity': 0.9, 'solarAbsorptivity': 0.6, 'densityKgM3': 7850, 'conductivityWmK': 50, 'specificHeatJKgK': 450, 'elasticModulusPa': 210000000000, 'source': 'authored-estimate', 'reference': 'iso-10456-2007'}]
    A['representations'] = {'500': rep500(), '200': rep200(), '100': rep100(), '50': rep50()}
    A['symbols'] = []
    A.pop('geometry', None)
    thermal()
    sun = {'unloaded': bind(), 'occupied-one': bind(occ=1), 'occupied-two': bind(occ=2)}
    for p in A['poses']:
        p['thermal'] = sun[p['id']]
    for c in A['clips']:
        for t in c['tracks']:
            t['target'] = t['target'].replace(':exposedFraction', ':sunlitFraction')
    A['clips'][1]['description'] = T('Illustrative scenario: a tree canopy shades the bench and the sun returns; the sunlit fraction of every surface varies and the surfaces cool and warm with their heat capacity.', 'Beispielszenario: Eine Baumkrone beschattet die Bank, dann kehrt die Sonne zurück; der besonnte Anteil jeder Fläche variiert, die Flächen kühlen und erwärmen sich gemäß ihrer Wärmekapazität.')
    for c in A['clips']:
        if c['id'] == 'sun-exposure-cycle':
            c['tracks'] = [{'target': f'region:{r}:sunlitFraction', 'ease': 'smooth', 'keys': [{'t': 0, 'v': 0.3 if r != R_STEEL else 0.25}, {'t': 30, 'v': 1}, {'t': 60, 'v': 0.3 if r != R_STEEL else 0.25}]} for r in (R_SEAT, R_BACK, R_STEEL)]
    A['structure']['assumptions'] = A['structure']['assumptions'][:2] + [T('Slat gaps and chamfers are geometric detail of the 1:50 drawing only; the structural model is unchanged by the drawing scale.', 'Lattenfugen und Fasen sind nur Detail der Zeichnung 1:50; das Tragmodell hängt nicht vom Zeichnungsmaßstab ab.')]
    A['presentation']['defaultScale'] = 50
    A['completion'] = 'authored'
    order = ['id', 'category', 'title', 'summary', 'tags', 'provenance', 'dimensions', 'materials', 'rig', 'anchors', 'representations', 'symbols', 'poses', 'clips', 'thermal', 'structure', 'presentation', 'exports', 'completion']
    return {k: A[k] for k in order}


if __name__ == '__main__':
    out = finish()
    x0, y0, x1, y1 = bbox_world(rig, out['representations']['50']['geometry']['elevation'])
    print('bbox50', x0, y0, x1, y1)
    for k in ('500', '200', '100'):
        print(k, bbox_world(rig, out['representations'][k]['geometry']['elevation']))
    if '--write' in sys.argv:
        save(path, out)
    for k in ('500', '200', '100', '50'):
        render(out['representations'][k], rig, int(k), f'bench-{k}-print', dpi=600)
        render(out['representations'][k], rig, int(k), f'bench-{k}-zoom', dpi=600, zoom=6 if k in ('500', '200') else 3)
