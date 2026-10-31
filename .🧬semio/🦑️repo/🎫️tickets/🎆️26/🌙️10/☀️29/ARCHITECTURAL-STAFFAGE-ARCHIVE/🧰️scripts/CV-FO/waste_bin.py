"""waste-bin v2: pedestal bin with hinged lid, four scales, passive sun/shade model."""
import sys
from lib import *
from common import *

path, A = load('objects', 'waste-bin')
rig = Rig(A['rig']['bones'])
BIN, LID, LIN, RIB = 'bin-hdpe', 'lid-hdpe', 'liner-film', 'bin-ribs'
KB, KL, KN, KR = (dict(region=BIN, material='hdpe-plastic'), dict(region=LID, material='hdpe-plastic'), dict(region=LIN, material='ldpe-film'), dict(region=RIB, material='hdpe-plastic'))
WL = lw(rig, 'lid')
LIDP = [('M', 0, 0), ('L', .52, 0), ('L', .52, .006), ('C', .52, .03, .45, .04, .40, .038), ('L', .12, .038), ('C', .05, .04, 0, .03, 0, .006), ('Z',)]
HANDLE = [('M', .38, .038), ('C', .39, .058, .47, .058, .48, .038), ('Z',)]
SHELL = [('M', -.2, .025), ('L', .2, .025), ('C', .215, .3, .235, .7, .248, .955), ('L', -.248, .955), ('C', -.235, .7, -.215, .3, -.2, .025), ('Z',)]


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


def hw(y):
    return 0.2 + 0.048 * (y - 0.025) / 0.93


def rep50():
    sh = Shapes(rig)
    sh.items = v1_shapes(A, skip=('hinge-pin', 'liner-fold', 'lid-handle-recess'))
    for s in sh.items:
        i = s['id']
        if i in ('shell', 'rim', 'lid', 'lid-skirt', 'pedestal'):
            s['penMm'] = 0.35
        if i.startswith('band') or i in ('shell-texture', 'rim-inner', 'lid-skirt-line'):
            s['penMm'] = 0.13
    sh.ids = {s['id'] for s in sh.items}
    return rep(50, sh, dict(
        silhouette=T('Side elevation 10.2 x 20.6 mm: a pedestal on two feet, a flaring HDPE shell with three moulded bands, a rim with a hanging liner collar, a hinge bracket and a lid with skirt and handle.',
                     'Seitenansicht 10,2 x 20,6 mm: ein Sockel auf zwei Füßen, eine sich öffnende HDPE-Schale mit drei angeformten Bändern, ein Rand mit überhängendem Sackkragen, ein Scharnierbock und ein Deckel mit Schürze und Griff.'),
        retained=T('Pedestal with feet, shell, three bands each with a bead line, shell texture strokes, rim with inner line, liner collar and ring, hinge bracket and barrel, lid with skirt, skirt line and handle.', 'Sockel mit Füßen, Schale, drei Bänder mit je einer Wulstlinie, Texturstriche der Schale, Rand mit Innenlinie, Sackkragen und Ring, Scharnierbock und -rolle, Deckel mit Schürze, Schürzenlinie und Griff.'),
        merged=T('Nothing merged.', 'Nichts verschmolzen.'),
        omitted=T('Hinge pin (0.2 mm dot), liner fold marks and the handle recess (0.1 mm).', 'Scharnierstift (0,2-mm-Punkt), Sackfalten und die Griffmulde (0,1 mm).'),
        thinElements=T('Bands, beads, rim line and skirt line are single 0.13 mm strokes (6 mm real); the 0.012 m liner ring is a 0.24 mm solid; the hinge barrel (0.028 m) a 0.56 mm ring.', 'Bänder, Wulstlinien, Randlinie und Schürzenlinie sind einzelne 0,13-mm-Striche (6 mm real); der 0,012 m starke Sackring ist eine 0,24 mm breite Fläche; die Scharnierrolle (0,028 m) ein 0,56-mm-Ring.'),
        hatching=T('None; the shell texture is three 0.13 mm curved strokes (no uniform spacing is needed for a moulded surface).', 'Keine; die Schalentextur besteht aus drei gebogenen 0,13-mm-Strichen (für eine Formfläche ist kein gleichmäßiger Abstand nötig).'),
        styles=T('Shell 0.42, rim 0.5, lid 0.55 and skirt 0.45, liner 0.5; body pen 0.35, detail 0.13-0.25.', 'Schale 0,42, Rand 0,5, Deckel 0,55 und Schürze 0,45, Sack 0,5; Körperstift 0,35, Detail 0,13-0,25.'),
        overlays=T('Shell, pedestal, rim, bracket = bin-hdpe; bands and ring = bin-ribs; collar = liner-film; lid, skirt, handle, barrel = lid-hdpe; the opening lid carries its own sun/shade state.', 'Schale, Sockel, Rand, Bock = bin-hdpe; Bänder und Ring = bin-ribs; Kragen = liner-film; Deckel, Schürze, Griff, Rolle = lid-hdpe; der aufgehende Deckel trägt seinen eigenen Sonne/Schatten-Zustand.')))


def rep100():
    sh = Shapes(rig)
    sh.rect('pedestal', 'body', -.2, 0, .2, .025, z=1, tone=0.3, **KB)
    sh.poly('shell', 'body', [(-.2, .025), (.2, .025), (.248, .94), (-.248, .94)], z=3, tone=0.42, **KB)
    for i, y in enumerate([.15, .45, .72]):
        sh.line(f'band-{i}', 'body', [(-hw(y), y), (hw(y), y)], z=3.5, **KR)
    sh.rect('rim', 'body', -.255, .94, .255, .98, r=.008, z=4, tone=0.5, **KB)
    sh.line('liner-collar', 'body', [(-.24, .925), (.24, .925)], z=4.2, tone=0.5, **KN)
    sh.rect('hinge-bracket', 'body', -.268, .925, -.242, .99, z=4.5, tone=0.3, **KB)
    sh.path('lid', 'lid', mapc(WL, [('M', 0, -.028), ('L', .52, -.028), ('L', .52, .006), ('C', .52, .03, .45, .04, .40, .038), ('C', .39, .058, .47, .058, .48, .038), ('L', .12, .038), ('C', .05, .04, 0, .03, 0, .006), ('Z',)]), z=5, tone=0.55, **KL)
    return rep(100, sh, dict(
        silhouette=T('Side elevation 5.2 x 10.3 mm: a pedestal slab, a flaring shell with three band lines, a rim, a liner collar line, hinge bracket and a lid with skirt and handle bump.', 'Seitenansicht 5,2 x 10,3 mm: eine Sockelplatte, eine sich öffnende Schale mit drei Bandlinien, ein Rand, eine Sackkragenlinie, Scharnierbock und ein Deckel mit Schürze und Griffwölbung.'),
        retained=T('Pedestal, shell, bands, rim, liner collar as a line, bracket, lid.', 'Sockel, Schale, Bänder, Rand, Sackkragen als Linie, Bock, Deckel.'),
        merged=T('Lid top and skirt become one lid silhouette (0.66 mm); pedestal feet merge into the pedestal slab; band beads merge into the band lines.', 'Deckeloberseite und Schürze werden eine Deckelsilhouette (0,66 mm); Sockelfüße gehen in der Sockelplatte auf; Wulstlinien gehen in den Bandlinien auf.'),
        omitted=T('Shell texture, rim inner line, liner ring, barrel, skirt line, handle recess, feet.', 'Schalentextur, Randinnenlinie, Sackring, Rolle, Schürzenlinie, Griffmulde, Füße.'),
        thinElements=T('The 0.025 m pedestal is a 0.25 mm solid; band and collar lines are 0.18 mm strokes; the 0.012 m liner collar is only a stroke.', 'Der 0,025 m starke Sockel ist eine 0,25 mm dicke Fläche; Band- und Kragenlinien sind 0,18-mm-Striche; der 0,012 m starke Sackkragen ist nur ein Strich.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Shell 0.42, rim 0.5, lid 0.55; body pen 0.35, detail 0.18.', 'Schale 0,42, Rand 0,5, Deckel 0,55; Körperstift 0,35, Detail 0,18.'),
        overlays=T('Shell, rim, pedestal, bracket = bin-hdpe; band lines = bin-ribs; collar line = liner-film; lid = lid-hdpe.', 'Schale, Rand, Sockel, Bock = bin-hdpe; Bandlinien = bin-ribs; Kragenlinie = liner-film; Deckel = lid-hdpe.')))


def rep200():
    sh = Shapes(rig)
    sh.poly('body-silhouette', 'body', [(-.2, 0), (.2, 0), (.2, .025), (.248, .94), (-.248, .94), (-.2, .025)], z=3, tone=0.42, **KB)
    for i, y in enumerate([.15, .45, .72]):
        sh.line(f'band-{i}', 'body', [(-hw(y), y), (hw(y), y)], z=3.5, **KR)
    sh.rect('rim', 'body', -.255, .94, .255, .98, z=4, tone=0.5, **KB)
    sh.line('liner-collar', 'body', [(-.2, .925), (.2, .925)], z=4.2, tone=0.5, **KN)
    sh.path('lid', 'lid', mapc(WL, [('M', 0, -.02), ('L', .52, -.02), ('L', .52, .006), ('C', .52, .03, .45, .04, .40, .038), ('L', .12, .038), ('C', .05, .04, 0, .03, 0, .006), ('Z',)]), z=5, tone=0.55, **KL)
    return rep(200, sh, dict(
        silhouette=T('Side elevation 2.6 x 5.2 mm: one flared body block on a pedestal, a rim, a lid slab, three band lines and a collar line.', 'Seitenansicht 2,6 x 5,2 mm: ein sich öffnender Korpusblock auf Sockel, ein Rand, eine Deckelplatte, drei Bandlinien und eine Kragenlinie.'),
        retained=T('Overall size, flared shell, rim, lid, bands.', 'Gesamtgröße, aufgeweitete Schale, Rand, Deckel, Bänder.'),
        merged=T('Pedestal and shell are one outline (the 0.025 m pedestal step is 0.13 mm); skirt, handle and lid top become one lid slab; hinge bracket and barrel merge into the rim.', 'Sockel und Schale sind eine Kontur (die 0,025 m hohe Sockelstufe ist 0,13 mm); Schürze, Griff und Deckeloberseite werden eine Deckelplatte; Scharnierbock und -rolle gehen im Rand auf.'),
        omitted=T('Handle bump, hinge, feet, texture, inner lines, beads.', 'Griffwölbung, Scharnier, Füße, Textur, Innenlinien, Wülste.'),
        thinElements=T('Bands and the collar are single 0.13 mm strokes; the 0.04 m rim is a 0.2 mm solid with outline; the 0.04 m lid slab a 0.2 mm solid.', 'Bänder und Kragen sind einzelne 0,13-mm-Striche; der 0,04 m starke Rand ist eine 0,2 mm Fläche mit Kontur; die 0,04 m starke Deckelplatte eine 0,2 mm Fläche.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Body 0.42, rim 0.5, lid 0.55; body pen 0.25, detail 0.13.', 'Korpus 0,42, Rand 0,5, Deckel 0,55; Körperstift 0,25, Detail 0,13.'),
        overlays=T('Body, rim = bin-hdpe; band lines = bin-ribs; collar line = liner-film; lid = lid-hdpe.', 'Korpus, Rand = bin-hdpe; Bandlinien = bin-ribs; Kragenlinie = liner-film; Deckel = lid-hdpe.')))


def rep500():
    sh = Shapes(rig)
    sh.poly('body', 'body', [(-.2, 0), (.2, 0), (.2, .025), (.255, .94), (.255, .98), (-.255, .98), (-.255, .94), (-.2, .025)], z=3, tone=0.42, regions=[BIN, RIB, LIN], material='hdpe-plastic')
    sh.path('lid', 'lid', mapc(WL, [('M', 0, 0), ('L', .52, 0), ('L', .52, .04), ('L', 0, .04), ('Z',)]), z=5, tone=0.55, **KL)
    return rep(500, sh, dict(
        silhouette=T('Composed bin sign 1.0 x 2.1 mm: one flared body with rim and a separate lid slab on top.', 'Zusammengesetztes Behälterzeichen 1,0 x 2,1 mm: ein aufgeweiteter Korpus mit Rand und eine getrennte Deckelplatte obenauf.'),
        retained=T('Height, width, flare of the shell, the lid as a separate slab (it is the part that moves).', 'Höhe, Breite, Aufweitung der Schale, der Deckel als eigene Platte (er ist das bewegte Teil).'),
        merged=T('Pedestal, shell, bands, rim, hinge bracket and liner collar become one body shape listing bin-hdpe, bin-ribs and liner-film; skirt and handle merge into the lid slab.', 'Sockel, Schale, Bänder, Rand, Scharnierbock und Sackkragen werden eine Korpusform mit bin-hdpe, bin-ribs und liner-film; Schürze und Griff gehen in der Deckelplatte auf.'),
        omitted=T('Bands, feet, handle, hinge, texture, collar - all below 0.05 m.', 'Bänder, Füße, Griff, Scharnier, Textur, Kragen - alle unter 0,05 m.'),
        thinElements=T('The 0.025 m pedestal step is merged into the outline; the lid (0.04 m = 0.08 mm) is shown as a 0.04 m slab whose outline pen (0.18 mm) makes it 0.26 mm; no hairlines.', 'Die 0,025 m hohe Sockelstufe ist in der Kontur aufgegangen; der Deckel (0,04 m = 0,08 mm) ist eine 0,04 m dicke Platte, deren Konturstift (0,18 mm) sie auf 0,26 mm bringt; keine Haarlinien.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Body 0.42, lid 0.55; one 0.18 mm outline.', 'Korpus 0,42, Deckel 0,55; eine 0,18-mm-Kontur.'),
        overlays=T('The body is a merged form (bin-hdpe, bin-ribs, liner-film, painted with the area-weighted value and labelled merged); the lid is lid-hdpe.', 'Der Korpus ist eine verschmolzene Form (bin-hdpe, bin-ribs, liner-film, mit dem flächengewichteten Wert gefärbt und als verschmolzen gekennzeichnet); der Deckel ist lid-hdpe.')))


def thermal():
    hd = MAT['hdpe']
    c3 = effective_cap(hd['rho'], hd['cp'], hd['k'], 0.003)
    c4 = effective_cap(hd['rho'], hd['cp'], hd['k'], 0.004)
    cf = effective_cap(920, 2300, 0.33, 0.00003)
    L = lambda t, c, i='hdpe-wall': layer(i, T(f'HDPE wall {int(t * 1000)} mm', f'HDPE-Wand {int(t * 1000)} mm'), 'polymer', 'hdpe-plastic', t, hd['k'], c, reference='iso-10456-2007')
    A['thermal'] = {'environment': {'airTempC': 20, 'meanRadiantTempC': 25, 'relativeHumidity': 0.5, 'windMS': 1.0, 'windDirectionDeg': 180, 'solarWM2': 600, 'diffuseFraction': 0.15, 'pressurePa': 101325}, 'operating': [], 'emitters': [],
                    'regions': [
        region(BIN, T('HDPE shell and pedestal', 'HDPE-Schale und Sockel'), 'surface', 'passive', 'hdpe-plastic', 1.4, 0.5, 'cylinder', 0.5, [L(0.003, c3)], c3, reference='iso-10456-2007'),
        region(LID, T('HDPE lid', 'HDPE-Deckel'), 'surface', 'passive', 'hdpe-plastic', 0.26, 0.5, 'horizontal-plate-up', 0.9, [L(0.004, c4)], c4, reference='iso-10456-2007'),
        region(LIN, T('Liner bag collar', 'Sackkragen'), 'surface', 'passive', 'ldpe-film', 0.08, 0.5, 'vertical-plate', 0.5, [layer('film', T('LDPE film 30 micrometres', 'LDPE-Folie 30 Mikrometer'), 'polymer', 'ldpe-film', 0.00003, 0.33, cf, source='authored-estimate')], cf),
        region(RIB, T('Moulded reinforcing bands', 'Angeformte Verstärkungsbänder'), 'surface', 'passive', 'hdpe-plastic', 0.16, 0.5, 'cylinder', 0.5, [L(0.005, effective_cap(hd['rho'], hd['cp'], hd['k'], 0.005))], effective_cap(hd['rho'], hd['cp'], hd['k'], 0.005), reference='iso-10456-2007')],
                    'assumptions': [
        T('Passive HDPE surfaces in sun: absorbed radiation, long-wave exchange, convection with heat capacity; no emitters, no authored temperatures. The waste inside is not modelled thermally.', 'Passive HDPE-Flächen in der Sonne: absorbierte Strahlung, Langwellenaustausch, Konvektion mit Wärmekapazität; keine Emitter, keine festgelegten Temperaturen. Der Abfall im Inneren wird thermisch nicht modelliert.'),
        T('A vertical cylindrical shell is lit on its sun-facing half (sunlit fraction 0.5); the horizontal lid is lit completely; the liner collar hangs over the rim and is half in the shade of the lid. Fractions are scenario data.', 'Eine senkrechte zylindrische Schale ist auf ihrer sonnenzugewandten Hälfte beleuchtet (besonnter Anteil 0,5); der waagrechte Deckel vollständig; der Sackkragen hängt über dem Rand und liegt halb im Schatten des Deckels. Die Anteile sind Szenariodaten.'),
        T('Thin walls (3-5 mm HDPE) respond within minutes (heat capacity rho*cp*thickness, 5-7 kJ/m2K); values after ISO 10456; film 30 micrometres is an authored estimate.', 'Dünne Wände (3-5 mm HDPE) reagieren in Minuten (Wärmekapazität rho*cp*Dicke, 5-7 kJ/m2K); Werte nach ISO 10456; die 30-Mikrometer-Folie ist ein Schätzwert.')]}


def bind(sun_shell=0.5, sun_lid=1.0, sun_liner=0.3):
    return binding({BIN: (1.0, sun_shell), LID: (1.0, sun_lid), LIN: (0.8, sun_liner), RIB: (1.0, sun_shell)})


def finish():
    A['category'] = 'objects'
    A['dimensions'] = {'heightM': 1.03, 'widthM': 0.53, 'depthM': 0.5, 'massKg': 5.7}
    A['provenance']['author'] = 'Semio staffage content (CV-FO)'
    A['provenance']['references'] = [REF_ASHBY, REF_EN1991, REF_ISO10456, REF_INCROPERA]
    A['materials'] = [
        {'id': 'hdpe-plastic', 'title': T('HDPE', 'HDPE'), 'emissivity': 0.92, 'solarAbsorptivity': 0.9, 'densityKgM3': 980, 'conductivityWmK': 0.5, 'specificHeatJKgK': 1800, 'source': 'literature', 'reference': 'iso-10456-2007'},
        {'id': 'ldpe-film', 'title': T('LDPE film', 'LDPE-Folie'), 'emissivity': 0.9, 'solarAbsorptivity': 0.4, 'densityKgM3': 920, 'conductivityWmK': 0.33, 'specificHeatJKgK': 2300, 'source': 'authored-estimate'}]
    A['representations'] = {'500': rep500(), '200': rep200(), '100': rep100(), '50': rep50()}
    A['symbols'] = []
    A.pop('geometry', None)
    thermal()
    for p in A['poses']:
        p['thermal'] = bind(0.5, 0.0 if False else 1.0, 0.3) if p['id'] != 'lid-open' else bind(0.5, 0.8, 1.0)
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
        render(out['representations'][k], rig, int(k), f'bin-{k}-print', dpi=600, zoom=1 if k in ('100', '50') else 5)
