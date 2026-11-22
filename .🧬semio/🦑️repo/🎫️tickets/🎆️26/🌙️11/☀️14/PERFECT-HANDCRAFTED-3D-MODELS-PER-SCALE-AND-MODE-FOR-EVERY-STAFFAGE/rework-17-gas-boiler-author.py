"""🔥 Author 17, wall-hung condensing gas boiler: the inside of the casing rebuilt by hand as the real appliance (coiled-tube heat exchanger in its shell, cylinder burner, premix fan with volute, motor and venturi, pump with volute and motor, diverter valve, plate heat exchanger, gas valve, hydroblocks, siphon, vessel and the pipework that joins them), and the heat exchanger carried on a saddle that clears the vessel. Frame of the casing bone: u = width (left of the face is -u), v = height, w = depth (front face -0.155, wall 0.175)."""
import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rework-17-kit.py')).read())

SLUG = 'gas-boiler'
V = load(SLUG)
C = 'casing'
W = [0, 0, 1]
U = [1, 0, 0]
UP = [0, 1, 0]
HX = [0.0, 0.56, 0.0]
HIDDEN = {'heat-exchanger', 'fan', 'expansion-vessel', 'hydraulics'}


def clear_hidden(L):
    L.v['thermal'] = [s for s in L.v['thermal'] if s['region'] not in HIDDEN]


def volute(id, x0, x1, cy, cw, r0, r1, **kw):
    """🐚 Fan scroll about the u axis: radius r0 at the tongue (front) growing to r1 at the bottom, tangential outlet to the front."""
    pts = []
    for k in range(10):
        phi = math.radians(-90 + 30 * k)
        r = r0 + (r1 - r0) * k / 9
        pts.append([cy + r * math.cos(phi), cw + r * math.sin(phi)])
    pts += [[cy - r1, cw - r1 - 0.001], [cy - r1 + 0.026, cw - r1 - 0.001], [cy - r1 + 0.026, cw - r0 + 0.002]]
    return ext(id, C, 'die-cast-aluminium', 'yz', pts, x0, x1, tone=0.6, **kw)


# ---------- 1:50 (7.5 mm) ----------
L = Lists(V, 50)
burner = cyl('burner-tube', C, 'burner-mesh', W, [0.0, 0.531, 0.0], -0.142, -0.03, 0.01, tone=0.45)
L.put('body', burner, after='chamber')
L.drop('body', 'burner-tube')
L.put('body', burner, after='chamber')
clear_hidden(L)
hx = dict(region='heat-exchanger', layer='wall')
fan = dict(region='fan', layer='housing')
ves = dict(region='expansion-vessel', layer='shell')
hy = dict(region='hydraulics', layer='body')
S = 'stainless-steel'
B = 'brass'
L.put('thermal',
      tube('th-hx-shell', C, S, W, HX, -0.137, 0.043, 0.113, 0.115, tone=0.5, **hx),
      tube('th-hx-door', C, S, W, HX, -0.147, -0.137, 0.0365, 0.115, tone=0.45, **hx),
      cyl('th-hx-back', C, S, W, HX, 0.033, 0.043, 0.113, tone=0.45, **hx),
      torus('th-hx-coil', C, S, W, HX, -0.12, 0.092, 0.0105, n=12, tone=0.6, repeat={'count': 6, 'translation': [0, 0, 0.026]}, **hx),
      cyl('th-hx-collector', C, S, UP, [0.0, 0.0, 0.04], 0.665, 0.77, 0.04, tone=0.5, **hx),
      cyl('th-hx-sump', C, S, UP, [0.0, 0.0, 0.032], 0.4, 0.45, 0.008, tone=0.5, **hx),
      volute('th-fan-volute', -0.195, -0.15, 0.6, -0.05, 0.04, 0.054, **fan),
      cyl('th-fan-motor', C, 'die-cast-aluminium', U, [0.0, 0.6, -0.05], -0.217, -0.195, 0.032, tone=0.55, **fan),
      rev('th-fan-venturi', C, 'die-cast-aluminium', U, [0.0, 0.6, -0.05], [[-0.15, 0], [-0.15, 0.022], [-0.138, 0.014], [-0.125, 0.02], [-0.125, 0]], tone=0.6, **fan),
      cap('th-fan-duct', C, 'die-cast-aluminium', [-0.172, 0.559, -0.1], [-0.095, 0.545, -0.14], 0.012, tone=0.6, **fan),
      rev('th-vessel', C, 'painted-steel', W, [0.0, 0.32, 0.0], [[0.075, 0], [0.077, 0.08], [0.087, 0.122], [0.105, 0.137], [0.116, 0.137], [0.12, 0.14], [0.124, 0.137], [0.135, 0.137], [0.153, 0.122], [0.163, 0.08], [0.165, 0]], tone=0.35, **ves),
      cyl('th-vessel-valve', C, 'painted-steel', UP, [0.0, 0.0, 0.12], 0.168, 0.185, 0.008, tone=0.35, **ves),
      box('th-block-flow', C, B, [0.17, 0.03, 0.05], [-0.105, 0.075, 0.07], r=0.003, tone=0.4, **hy),
      box('th-block-return', C, B, [0.15, 0.03, 0.05], [0.115, 0.075, 0.07], r=0.003, tone=0.4, **hy),
      rev('th-pump-volute', C, B, W, [0.125, 0.155, 0.0], [[0.045, 0], [0.045, 0.042], [0.052, 0.05], [0.088, 0.05], [0.095, 0.042], [0.095, 0]], tone=0.4, **hy),
      cyl('th-pump-neck', C, B, UP, [0.125, 0.0, 0.07], 0.088, 0.115, 0.014, tone=0.4, **hy),
      rev('th-pump-motor', C, B, W, [0.125, 0.155, 0.0], [[-0.04, 0], [-0.04, 0.034], [-0.034, 0.04], [0.037, 0.04], [0.037, 0.047], [0.045, 0.047], [0.045, 0]], tone=0.45, **hy),
      box('th-pump-terminal', C, 'polycarbonate', [0.05, 0.022, 0.045], [0.125, 0.205, -0.01], r=0.003, tone=0.3, **hy),
      cyl('th-diverter', C, B, UP, [-0.16, 0.0, 0.07], 0.088, 0.16, 0.017, tone=0.4, **hy),
      box('th-diverter-motor', C, 'polycarbonate', [0.04, 0.045, 0.04], [-0.16, 0.125, 0.035], r=0.004, tone=0.3, **hy),
      box('th-plate-hx', C, S, [0.075, 0.17, 0.04], [-0.06, 0.175, 0.05], r=0.003, tone=0.45, **hy),
      box('th-gas-valve', C, B, [0.06, 0.06, 0.05], [0.03, 0.16, 0.0], r=0.004, tone=0.4, **hy),
      cyl('th-gas-solenoids', C, 'polycarbonate', UP, [0.015, 0.0, 0.0], 0.188, 0.225, 0.013, tone=0.3, repeat={'count': 2, 'translation': [0.03, 0, 0]}, **hy),
      cap('th-gas-inlet', C, B, [0.0, 0.055, 0.07], [0.0, 0.105, 0.07], 0.0075, tone=0.4, **hy),
      cap('th-gas-inlet-bend', C, B, [0.0, 0.105, 0.07], [0.02, 0.135, 0.02], 0.0075, tone=0.4, **hy),
      cap('th-gas-pipe-low', C, B, [0.0, 0.165, -0.005], [-0.19, 0.165, -0.02], 0.0075, tone=0.4, **hy),
      cap('th-gas-pipe-riser', C, B, [-0.19, 0.165, -0.02], [-0.19, 0.5, -0.02], 0.0075, tone=0.4, **hy),
      cap('th-gas-pipe-top', C, B, [-0.19, 0.5, -0.02], [-0.137, 0.5, -0.05], 0.0075, tone=0.4, **hy),
      cap('th-gas-pipe-venturi', C, B, [-0.137, 0.5, -0.05], [-0.137, 0.585, -0.05], 0.0075, tone=0.4, **hy),
      cap('th-flow-pipe-out', C, B, [-0.105, 0.52, 0.0], [-0.16, 0.5, 0.03], 0.009, tone=0.4, **hy),
      cap('th-flow-pipe', C, B, [-0.16, 0.5, 0.03], [-0.16, 0.2, 0.03], 0.009, tone=0.4, **hy),
      cap('th-flow-pipe-in', C, B, [-0.16, 0.2, 0.03], [-0.16, 0.155, 0.065], 0.009, tone=0.4, **hy),
      cap('th-return-pipe-out', C, B, [0.15, 0.195, 0.07], [0.15, 0.24, 0.055], 0.009, tone=0.4, **hy),
      cap('th-return-pipe', C, B, [0.15, 0.24, 0.055], [0.15, 0.53, 0.055], 0.009, tone=0.4, **hy),
      cap('th-return-pipe-in', C, B, [0.15, 0.53, 0.055], [0.108, 0.53, 0.02], 0.009, tone=0.4, **hy),
      cyl('th-siphon', C, 'polycarbonate', UP, [0.18, 0.0, -0.09], 0.1, 0.2, 0.02, tone=0.4, **hy),
      cap('th-condensate-hose', C, 'polycarbonate', [0.0, 0.4, 0.032], [0.18, 0.2, -0.09], 0.006, tone=0.4, **hy),
      cap('th-vessel-hose', C, B, [0.0, 0.17, 0.12], [0.08, 0.092, 0.08], 0.005, tone=0.4, **hy),
      )
L.drop('structure', 'member-hx-brackets-low', 'member-manifold')
G = 'galvanised-steel'
saddle = [[-0.125, 0.425]] + [[0.125, 0.425], [0.125, 0.445], [0.06, 0.462]] + arc(0.0, 0.56, 0.116, -58, -122, 6)[1:-1] + [[-0.06, 0.462], [-0.125, 0.445]]
L.put('structure',
      ext('member-hx-saddle', C, G, 'xy', saddle, -0.01, 0.02, tone=0.7, member=C, part='member'),
      ext('member-hx-arms', C, G, 'xy', [[0.095, 0.425], [0.125, 0.425], [0.125, 0.445], [0.098, 0.445], [0.098, 0.428], [0.095, 0.428]][::-1], 0.02, 0.1735, tone=0.7, member=C, part='member'),
      ext('member-hx-arms-left', C, G, 'xy', [[-0.095, 0.425], [-0.125, 0.425], [-0.125, 0.445], [-0.098, 0.445], [-0.098, 0.428], [-0.095, 0.428]], 0.02, 0.1735, tone=0.7, member=C, part='member'),
      box('member-block-flow', C, B, [0.17, 0.03, 0.05], [-0.105, 0.075, 0.07], r=0.003, tone=0.45, member=C, part='member'),
      box('member-block-return', C, B, [0.15, 0.03, 0.05], [0.115, 0.075, 0.07], r=0.003, tone=0.45, member=C, part='member'),
      after='member-chassis-bottom')
L.decide('Thermal:',
         'Thermal: the casing sheet is the closed hood with chassis, fascia and bezel; behind it every hidden part is the real component in its place. Heat exchanger: a 2 mm stainless shell of 230 mm with the burner door, the insulated back plate, six turns of 21 mm coil tube at 184 mm coil diameter, the flue collector up to the collar and the condensate outlet. Premix fan on the left: a scroll that grows from 40 to 54 mm with its tangential outlet and the duct to the burner door, the flat motor outside and the gas-air venturi inside. Lower half: the 280 mm expansion vessel with its crimped seam at the back, the wet-rotor pump (volute, motor can, terminal box) on the return hydroblock, the diverter valve with its actuator on the flow hydroblock, the plate heat exchanger, the gas valve with two solenoids and its line through the venturi, flow and return pipes between heat exchanger and hydroblocks, the siphon with the condensate hose and the vessel hose. The burner is a 20 mm perforated cylinder on the chamber axis with the flames on its crown.',
         'Thermik: das Gehäuseblech ist die geschlossene Haube mit Chassis, Blende und Schauglasrahmen; dahinter ist jedes verdeckte Teil das wirkliche Bauteil an seinem Platz. Wärmeübertrager: ein 2 mm starker Edelstahlmantel von 230 mm mit Brennertür, gedämmter Rückwand, sechs Windungen Rohrschlange von 21 mm bei 184 mm Wickeldurchmesser, dem Abgassammler bis zum Stutzen und dem Kondensatablauf. Vormischgebläse links: eine Spirale, die von 40 auf 54 mm wächst, mit tangentialem Auslass und dem Kanal zur Brennertür, außen der flache Motor, innen das Gas-Luft-Venturi. Untere Hälfte: das Ausdehnungsgefäß von 280 mm mit gebördelter Naht an der Rückwand, die Nassläuferpumpe (Spiralgehäuse, Spaltrohrmotor, Klemmenkasten) auf dem Rücklauf-Hydroblock, das Umschaltventil mit Stellmotor auf dem Vorlauf-Hydroblock, der Plattenwärmeübertrager, die Gasarmatur mit zwei Magnetspulen und ihrer Leitung zum Venturi, Vor- und Rücklaufrohr zwischen Wärmeübertrager und Hydroblöcken, der Siphon mit Kondensatschlauch und der Schlauch des Gefäßes. Der Brenner ist ein gelochter Zylinder von 20 mm auf der Kammerachse mit den Flammen auf seinem Scheitel.')
L.decide('Structure:',
         'Structure: the load path of the real boiler, not its cladding: the galvanised back sheet with its folded edges hangs on the wall rail and leans on its lower edge; the heat exchanger hangs on the upper pair of angle brackets and sits in a saddle whose two arms run back to the sheet outside the expansion vessel; the flue collector and the collar stand on the heat exchanger, the two hydroblocks with the five stubs hang on the manifold angle of the bottom edge. The centre of mass is the magenta ball.',
         'Tragwerk: der Lastweg des wirklichen Kessels, nicht seine Verkleidung: das verzinkte Rückblech mit gekanteten Rändern hängt an der Wandschiene und lehnt an seiner Unterkante; der Wärmeübertrager hängt am oberen Paar Winkelkonsolen und liegt in einem Sattel, dessen zwei Arme außerhalb des Ausdehnungsgefäßes zum Blech zurücklaufen; Abgassammler und Stutzen stehen auf dem Wärmeübertrager, die zwei Hydroblöcke mit den fünf Stutzen hängen am Verteilerwinkel der Unterkante. Der Schwerpunkt ist die magentafarbene Kugel.')

# ---------- 1:100 (15 mm) ----------
L = Lists(V, 100)
L.drop('body', 'burner-tube')
L.put('body', cyl('burner-tube', C, 'burner-mesh', W, [0.0, 0.531, 0.0], -0.14, -0.035, 0.01, tone=0.45), after='chamber')
clear_hidden(L)
L.put('thermal',
      rev('th-hx-shell', C, S, W, HX, [[-0.147, 0.0365], [-0.147, 0.115], [0.043, 0.115], [0.043, 0.0], [0.035, 0.0], [0.035, 0.112], [-0.139, 0.112], [-0.139, 0.0365], [-0.147, 0.0365]], tone=0.5, **hx),
      torus('th-hx-coil', C, S, W, HX, -0.115, 0.09, 0.012, n=8, tone=0.6, repeat={'count': 4, 'translation': [0, 0, 0.04]}, **hx),
      cyl('th-hx-collector', C, S, UP, [0.0, 0.0, 0.04], 0.665, 0.77, 0.04, tone=0.5, **hx),
      volute('th-fan-volute', -0.195, -0.15, 0.6, -0.05, 0.041, 0.054, **fan),
      cyl('th-fan-motor', C, 'die-cast-aluminium', U, [0.0, 0.6, -0.05], -0.217, -0.195, 0.032, tone=0.55, **fan),
      cyl('th-fan-venturi', C, 'die-cast-aluminium', U, [0.0, 0.6, -0.05], -0.15, -0.125, 0.02, tone=0.6, **fan),
      rev('th-vessel', C, 'painted-steel', W, [0.0, 0.32, 0.0], [[0.075, 0], [0.078, 0.09], [0.092, 0.128], [0.12, 0.14], [0.148, 0.128], [0.162, 0.09], [0.165, 0]], tone=0.35, **ves),
      box('th-block-flow', C, B, [0.17, 0.03, 0.05], [-0.105, 0.075, 0.07], tone=0.4, **hy),
      box('th-block-return', C, B, [0.15, 0.03, 0.05], [0.115, 0.075, 0.07], tone=0.4, **hy),
      cyl('th-pump-volute', C, B, W, [0.125, 0.155, 0.0], 0.045, 0.095, 0.05, tone=0.4, **hy),
      cyl('th-pump-motor', C, B, W, [0.125, 0.155, 0.0], -0.04, 0.045, 0.04, tone=0.45, **hy),
      cyl('th-diverter', C, B, UP, [-0.16, 0.0, 0.07], 0.088, 0.16, 0.017, tone=0.4, **hy),
      box('th-plate-hx', C, S, [0.075, 0.17, 0.04], [-0.06, 0.175, 0.05], tone=0.45, **hy),
      box('th-gas-valve', C, B, [0.06, 0.09, 0.05], [0.03, 0.175, 0.0], tone=0.4, **hy),
      cap('th-gas-pipe-riser', C, B, [-0.19, 0.165, -0.02], [-0.19, 0.5, -0.02], 0.0075, tone=0.4, **hy),
      cap('th-flow-pipe', C, B, [-0.16, 0.5, 0.03], [-0.16, 0.16, 0.03], 0.009, tone=0.4, **hy),
      cap('th-return-pipe', C, B, [0.15, 0.2, 0.055], [0.15, 0.53, 0.055], 0.009, tone=0.4, **hy),
      cyl('th-siphon', C, 'polycarbonate', UP, [0.18, 0.0, -0.09], 0.1, 0.2, 0.02, tone=0.4, **hy),
      )
L.drop('structure', 'member-hx-brackets')
L.put('structure',
      box('member-hx-saddle', C, G, [0.25, 0.02, 0.03], [0.0, 0.435, 0.005], tone=0.7, member=C, part='member'),
      box('member-hx-arms', C, G, [0.03, 0.02, 0.154], [0.11, 0.435, 0.097], tone=0.7, member=C, part='member', repeat={'count': 2, 'translation': [-0.22, 0, 0]}),
      after='member-chassis')
L.decide('Thermal:',
         'Thermal: authored for the 15 mm feature. The closed hood with the fascia (casing), display and knob (controls), sight glass, burner cylinder with flames, collar and stubs as at 1:50 in coarser parts; inside, the heat exchanger is one shell with door and back plate and four fat coil turns of 24 mm, the fan keeps scroll, motor and venturi, the vessel is a plain lens, pump (volute and motor), diverter valve, plate heat exchanger, gas valve, two hydroblocks, siphon and the three risers (gas, flow, return) of 15 to 18 mm stay; hoses and bends are below the feature.',
         'Thermik: für das Merkmal von 15 mm gezeichnet. Die geschlossene Haube mit Blende (Gehäuse), Display und Knopf (Bedienung), Schauglas, Brennerzylinder mit Flammen, Stutzen und Anschlüsse wie bei 1:50 in gröberen Teilen; innen ist der Wärmeübertrager ein Mantel mit Tür und Rückwand und vier dicken Rohrwindungen von 24 mm, das Gebläse behält Spirale, Motor und Venturi, das Gefäß ist eine schlichte Linse, Pumpe (Spiralgehäuse und Motor), Umschaltventil, Plattenwärmeübertrager, Gasarmatur, zwei Hydroblöcke, Siphon und die drei Steigleitungen (Gas, Vorlauf, Rücklauf) von 15 bis 18 mm bleiben; Schläuche und Bögen liegen unter dem Merkmal.')
L.decide('Structure:',
         'Structure: chassis channel with its two edge angles, the heat exchanger on a 20 mm saddle bar whose two arms run back to the sheet outside the vessel, flue collector, collar, connection bar and the two wall supports; the centre of mass as a ball.',
         'Tragwerk: Chassis-U mit seinen zwei Randwinkeln, der Wärmeübertrager auf einem Sattelstab von 20 mm, dessen zwei Arme außerhalb des Gefäßes zum Blech zurücklaufen, Abgassammler, Stutzen, Anschlussleiste und die zwei Wandauflager; der Schwerpunkt als Kugel.')

# ---------- 1:200 (30 mm) ----------
L = Lists(V, 200)
clear_hidden(L)
L.put('thermal',
      rev('th-hx', C, S, W, HX, [[-0.14, 0.0365], [-0.14, 0.115], [0.043, 0.115], [0.043, 0.0], [0.01, 0.0], [0.01, 0.0365]], tone=0.5, **hx),
      cyl('th-hx-collector', C, S, UP, [0.0, 0.0, 0.04], 0.665, 0.77, 0.04, tone=0.5, **hx),
      cyl('th-fan', C, 'die-cast-aluminium', U, [0.0, 0.6, -0.05], -0.215, -0.15, 0.052, tone=0.6, **fan),
      rev('th-vessel', C, 'painted-steel', W, [0.0, 0.32, 0.0], [[0.075, 0], [0.08, 0.1], [0.12, 0.14], [0.16, 0.1], [0.165, 0]], tone=0.3, **ves),
      box('th-hydroblock', C, B, [0.36, 0.03, 0.05], [0.0, 0.075, 0.07], tone=0.36, **hy),
      cyl('th-pump', C, B, W, [0.125, 0.155, 0.0], -0.04, 0.095, 0.045, tone=0.4, **hy),
      box('th-plate-hx', C, S, [0.075, 0.17, 0.04], [-0.06, 0.175, 0.05], tone=0.45, **hy),
      box('th-gas-valve', C, B, [0.06, 0.09, 0.05], [0.03, 0.165, 0.0], tone=0.4, **hy),
      )
L.drop('structure', 'member-hx-brackets')
L.put('structure',
      box('member-hx-shelf', C, G, [0.25, 0.03, 0.045], [0.0, 0.43, 0.02], tone=0.7, member=C, part='member'),
      box('member-hx-arms', C, G, [0.03, 0.03, 0.131], [0.11, 0.43, 0.108], tone=0.7, member=C, part='member', repeat={'count': 2, 'translation': [-0.22, 0, 0]}),
      after='member-chassis')
L.decide('Thermal:',
         'Thermal: the hood (casing), the control band (controls), a flush window disc, the burner block, collar and connection bar; inside, for the 30 mm feature, the heat exchanger is one drum with the burner bore and the flue collector on it, the fan one 104 mm drum, the vessel a lens at the back, and the hydraulics are the pump as one drum, the plate heat exchanger, the gas valve block and one hydroblock bar.',
         'Thermik: die Haube (Gehäuse), das Bedienband (Bedienung), eine bündige Schauglasscheibe, der Brennerblock, Stutzen und Anschlussleiste; innen ist für das Merkmal von 30 mm der Wärmeübertrager eine Trommel mit Brennerbohrung und dem Abgassammler darauf, das Gebläse eine Trommel von 104 mm, das Gefäß eine Linse an der Rückwand, und die Hydraulik sind die Pumpe als eine Trommel, der Plattenwärmeübertrager, der Block der Gasarmatur und ein Hydroblock-Stab.')
L.decide('Structure:',
         'Structure: the chassis as a channel 30 mm deep at the wall, the heat exchanger drum on a 30 mm shelf with two arms back to the sheet outside the vessel, the flue collector, the collar, the connection bar and the two wall supports, with the centre of mass as a ball.',
         'Tragwerk: das Chassis als 30 mm tiefes U an der Wand, die Wärmeübertrager-Trommel auf einem Bord von 30 mm mit zwei Armen außerhalb des Gefäßes zurück zum Blech, Abgassammler, Stutzen, Anschlussleiste und die zwei Wandauflager, der Schwerpunkt als Kugel.')

# ---------- 1:500 (75 mm) ----------
L = Lists(V, 500)
clear_hidden(L)
L.put('thermal',
      cyl('th-hx', C, S, W, HX, -0.14, 0.043, 0.115, tone=0.5, **hx),
      cyl('th-fan', C, 'die-cast-aluminium', U, [0.0, 0.6, -0.05], -0.215, -0.14, 0.05, tone=0.6, **fan),
      rev('th-vessel', C, 'painted-steel', W, [0.0, 0.32, 0.0], [[0.075, 0], [0.085, 0.11], [0.12, 0.14], [0.155, 0.11], [0.165, 0]], tone=0.3, **ves),
      cyl('th-pump', C, B, W, [0.125, 0.155, 0.0], -0.04, 0.095, 0.05, tone=0.4, **hy),
      box('th-hydroblock', C, B, [0.25, 0.2, 0.075], [-0.07, 0.16, 0.05], tone=0.36, **hy),
      )
L.drop('structure', 'member-hx-brackets')
L.put('structure',
      box('member-hx-shelf', C, G, [0.25, 0.03, 0.17], [0.0, 0.43, 0.088], tone=0.7, member=C, part='member'),
      after='member-chassis')
L.decide('Thermal:',
         'Thermal: hood, control strip, the window as a flush disc, one burner block, collar and connection bar; inside, the heat exchanger drum, the fan drum of 100 mm, the vessel lens, the pump drum and one block for plate heat exchanger, valves and hydroblock: the four hidden regions stay separate bodies in their real places at the 75 mm feature.',
         'Thermik: Haube, Bedienstreifen, das Schauglas als bündige Scheibe, ein Brennerblock, Stutzen und Anschlussleiste; innen die Trommel des Wärmeübertragers, die Gebläsetrommel von 100 mm, die Linse des Gefäßes, die Pumpentrommel und ein Block für Plattenwärmeübertrager, Ventile und Hydroblock: die vier verdeckten Regionen bleiben beim Merkmal von 75 mm getrennte Körper an ihrem wirklichen Platz.')
L.decide('Structure:',
         'Structure: the real load path, generalised: the galvanised chassis channel at the wall, one shelf that carries the heat exchanger drum, the collar on it, the connection bar and the two wall supports; the centre of mass as a ball.',
         'Tragwerk: der wirkliche Lastweg, verallgemeinert: das verzinkte Chassis-U an der Wand, ein Bord, das die Wärmeübertrager-Trommel trägt, darauf der Stutzen, die Anschlussleiste und die zwei Wandauflager; der Schwerpunkt als Kugel.')

save(SLUG, V)
print('saved', {s: tuple(len(V[s][l]) for l in ('body', 'thermal', 'structure')) for s in V})

# ---------- catalog root: areas and hidden notes of the rebuilt regions ----------
set_area(SLUG, {'heat-exchanger': 0.65, 'fan': 0.05, 'expansion-vessel': 0.16, 'hydraulics': 0.32, 'burner': 0.012})
patch_catalog(SLUG, [
    ('Its area is the envelope of the housing (0.35 m2); the wetted coil area that takes the 20 kW into the heating water is not modelled, the region is passive at room air.',
     'Its area is the surface of the shell, the door, the back plate, the flue collector and the six coil turns as modelled (0.65 m2); the heat the coil takes into the heating water is not modelled, the region is passive at room air.'),
    ('Seine Fläche ist die Hülle des Gehäuses (0,35 m2); die benetzte Rohrfläche, die die 20 kW ans Heizwasser abgibt, ist nicht modelliert, die Region ist passiv bei Raumluft.',
     'Seine Fläche ist die Oberfläche von Mantel, Tür, Rückwand, Abgassammler und den sechs Rohrwindungen wie modelliert (0,65 m2); die Wärme, die die Rohrschlange ans Heizwasser abgibt, ist nicht modelliert, die Region ist passiv bei Raumluft.'),
    ('The premix fan of 90 mm diameter sits beside the heat exchanger on the left and blows the gas-air mixture through the burner door',
     'The premix fan with its scroll of 80 to 108 mm, the flat motor and the gas-air venturi sits beside the heat exchanger on the left and blows the mixture through the burner door'),
    ('Das Vormischgebläse von 90 mm Durchmesser sitzt links neben dem Wärmeübertrager und bläst das Gas-Luft-Gemisch durch die Brennertür',
     'Das Vormischgebläse mit seiner Spirale von 80 bis 108 mm, dem flachen Motor und dem Gas-Luft-Venturi sitzt links neben dem Wärmeübertrager und bläst das Gemisch durch die Brennertür'),
    ('The flat round expansion vessel of 0.32 m diameter and 80 mm depth stands between the heat exchanger and the back sheet',
     'The flat round expansion vessel of 0.28 m diameter and 90 mm depth stands at the back sheet below the heat exchanger'),
    ('Das flache runde Ausdehnungsgefäß von 0,32 m Durchmesser und 80 mm Tiefe steht zwischen Wärmeübertrager und Rückwand',
     'Das flache runde Ausdehnungsgefäß von 0,28 m Durchmesser und 90 mm Tiefe steht an der Rückwand unter dem Wärmeübertrager'),
    ('Pump, gas valve, domestic hot-water plate heat exchanger, the connection manifold, the condensate siphon and the flow and return pipes from the heat exchanger fill the lower part of the casing behind the control panel',
     'Pump, diverter valve, gas valve with its line to the fan, domestic hot-water plate heat exchanger, the two hydroblocks, the condensate siphon with its hose and the flow and return pipes from the heat exchanger fill the lower part of the casing behind the control panel'),
    ('Pumpe, Gasarmatur, Trinkwasser-Plattenwärmeübertrager, der Anschlussverteiler, der Kondensatsiphon und Vor- und Rücklaufrohre vom Wärmeübertrager füllen den unteren Gehäuseteil hinter dem Bedienfeld',
     'Pumpe, Umschaltventil, Gasarmatur mit ihrer Leitung zum Gebläse, Trinkwasser-Plattenwärmeübertrager, die zwei Hydroblöcke, der Kondensatsiphon mit Schlauch und Vor- und Rücklaufrohre vom Wärmeübertrager füllen den unteren Gehäuseteil hinter dem Bedienfeld'),
])
