"""⛏️ Excavator: access hardware at 1:50 and 1:100, and the undercarriage and bucket modelled anew for 1:100, 1:200 and 1:500 (they were the 1:50 parts carried over: 14 mm shoe bands, 18 mm grousers, 12 mm wear strips, stepped adjuster profiles at every scale)."""
exec(open('rework-10-lib.py').read())
s = Source('excavator')
U, B, PS, TS, WS = 'undercarriage', 'bucket', 'paint-steel', 'track-steel', 'wear-steel'
SPROCKET, IDLER = [-1.825, 0.47], [1.825, 0.45]
BOTH = {"repeat": {"count": 2, "translation": [0, 0, -2.38]}}

for scale, r in ((50, 0.015), (100, 0.016)):
    s.put(scale, [
        box('track-step', U, PS, [0.32, 0.03, 0.15], [0.75, 0.47, 1.415], tone=0.35, repeat={"count": 2, "translation": [-1.5, 0, 0]}),
        box('track-step-right', U, PS, [0.32, 0.03, 0.15], [0.75, 0.47, -1.415], tone=0.35, repeat={"count": 2, "translation": [-1.5, 0, 0]}),
    ], region='tracks')
    s.put(scale, [
        tube('cab-grab-door', 'upper', PS, [[0.5, 0.74, 1.36], [0.5, 0.74, 1.425], [0.5, 1.56, 1.425], [0.5, 1.56, 1.36]], r),
        tube('cab-grab-front', 'upper', PS, [[1.405, 0.5, 1.36], [1.405, 0.5, 1.425], [1.405, 0.98, 1.425], [1.405, 0.98, 1.36]], r),
        box('cab-light', 'upper', PS, [0.08, 0.09, 0.15], [1.2, 1.945, 0.62], tone=0.25, cornerRadius=0.015, repeat={"count": 2, "translation": [0, 0, 0.55]}),
    ], region='house')

s.get(50, 'stack')['profile'] = [[0, 0.052], [0, 0.06], [0.5, 0.06], [0.5, 0.052], [0, 0.052]]
s.drop(100, ['bucket-wear-1', 'bucket-wear-2'])
shoes100 = s.get(100, 'track-shoes')
shoes100['profile'] = belt(SPROCKET, IDLER, 0.374, 0.026, 1.42)
shoes100['holes'] = [belt(SPROCKET, IDLER, 0.354, 0.046, 1.42)]

OUT = ['track-rail-outer', 'track-rail-inner', 'grouser-bottom-l', 'grouser-bottom-r', 'sprocket-teeth-l', 'sprocket-teeth-r', 'roller-shaft-l', 'roller-shaft-r', 'bucket-wear-1', 'bucket-wear-2', 'bucket-shell', 'bucket-cheek-right', 'bucket-cheek-left']
SHELL = [[1.305, 0.145], [0.9, 0.497], [0.49, 0.703], [0.39, 0.72], [-0.05, 0.583], [-0.13, 0.505], [-0.188, 0.26], [-0.132, -0.062], [-0.092, -0.094], [0.06, -0.11]]

CUTTER = [[1.2207, -0.0313], [1.2468, -0.0118]]


def bucket(scale, t, lip):
    path = SHELL if lip else [[1.333, 0.12]] + SHELL[1:]
    return [
        ext('bucket-shell', B, WS, thick(path, t, side=1), 'xy', -0.68 + t, 0.52 - t, tone=0.4),
        ext('bucket-cheek-right', B, WS, path + CUTTER, 'xy', -0.68, -0.68 + t, tone=0.42),
        ext('bucket-cheek-left', B, WS, path + CUTTER, 'xy', 0.52 - t, 0.52, tone=0.42),
    ]


s.drop(200, OUT + ['idler-cross-pin', 'track-adjuster-barrel', 'track-adjuster-rod'])
s.get(200, 'track-shoes').update(profile=belt(SPROCKET, IDLER, 0.4, 0, 1.42), holes=[belt(SPROCKET, IDLER, 0.36, 0.04, 1.42)])
s.put(200, [
    ext('track-links', U, TS, belt(SPROCKET, IDLER, 0.362, 0.038, 1.42), 'xy', 1.1, 1.28, tone=0.25, role='detail', holes=[belt(SPROCKET, IDLER, 0.275, 0.125, 1.42)], **BOTH),
    cyl('roller-l', U, TS, [-1.42, 0.215, 1.09], [0, 0, 1], 0.09, 0.2, tone=0.3, repeat={"count": 8, "translation": [0.4057, 0, 0]}),
    cyl('roller-r', U, TS, [-1.42, 0.215, -1.29], [0, 0, 1], 0.09, 0.2, tone=0.3, repeat={"count": 8, "translation": [0.4057, 0, 0]}),
    cyl('idler-cross-pin', U, PS, [1.825, 0.45, 1.04], [0, 0, 1], 0.035, 0.3, **BOTH),
    cyl('track-adjuster-barrel', U, PS, [0.995, 0.45, 1.19], [1, 0, 0], 0.07, 0.24, **BOTH),
    cyl('track-adjuster-rod', U, 'chrome', [1.235, 0.45, 1.19], [1, 0, 0], 0.035, 0.13, tone=0.7, **BOTH),
], region='tracks')
s.put(200, bucket(200, 0.03, True), region='attachment')

s.drop(500, OUT + ['idler-cross-pin', 'track-adjuster-barrel', 'track-adjuster-rod', 'sprocket-cap', 'carrier-post-l', 'carrier-post-r', 'idler-yoke-outer', 'idler-yoke-inner', 'bucket-lip', 'bucket-link-pin', 'bucket-ear-right', 'bucket-ear-left', 'idler', 'sprocket'])
s.get(500, 'track-shoes').update(profile=belt(SPROCKET, IDLER, 0.4, 0, 1.42, n=7), holes=[belt(SPROCKET, IDLER, 0.275, 0.125, 1.42, n=7)])
YOKE = [[1.34, 0.37], [1.86, 0.37], [1.91, 0.45], [1.86, 0.53], [1.34, 0.53]]
EAR = [[-0.12, 0.62], [-0.218, 0.2], [-0.13, -0.07], [-0.084, -0.1], [0.12, -0.1], [0.12, 0.05], [-0.02, 0.12], [-0.02, 0.6]]
s.put(500, [
    cyl('sprocket', U, TS, [-1.825, 0.47, 1.145], [0, 0, 1], 0.29, 0.09, tone=0.3, **BOTH),
    cyl('idler', U, TS, [1.825, 0.45, 1.095], [0, 0, 1], 0.28, 0.19, tone=0.3, **BOTH),
    cyl('roller-l', U, TS, [-1.42, 0.215, 1.09], [0, 0, 1], 0.09, 0.2, tone=0.3, repeat={"count": 8, "translation": [0.4057, 0, 0]}),
    cyl('roller-r', U, TS, [-1.42, 0.215, -1.29], [0, 0, 1], 0.09, 0.2, tone=0.3, repeat={"count": 8, "translation": [0.4057, 0, 0]}),
    ext('track-frame', U, PS, [[0.215, 1.04], [0.215, 1.34], [0.52, 1.34], [0.62, 1.19], [0.52, 1.04]], 'yz', -1.7, 1.5, role='detail', **BOTH),
    ext('idler-yoke-outer', U, PS, YOKE, 'xy', 1.285, 1.36, role='detail', **BOTH),
    ext('idler-yoke-inner', U, PS, YOKE, 'xy', 1.02, 1.095, role='detail', **BOTH),
], region='tracks')
s.put(500, bucket(500, 0.075, False) + [
    ext('bucket-ear-right', B, WS, EAR, 'xy', -0.31, -0.235, tone=0.38),
    ext('bucket-ear-left', B, WS, EAR, 'xy', 0.075, 0.15, tone=0.38),
], region='attachment')

s.decide(50, "Rework 10: two step plates on each track frame, the door and front grab rails and the two roof work lights were missing and are added as what they are.", "Überarbeitung 10: je zwei Trittbleche an den Kettenrahmen, die Haltestangen an Tür und Front sowie die beiden Dacharbeitsscheinwerfer fehlten und sind als das ergänzt, was sie sind.")
s.decide(100, "Rework 10: the shoe band is 20 mm thick for the 15 mm feature, the 12 mm bucket wear strips are left to 1:50; steps, grab rails and roof lights as at 1:50.", "Überarbeitung 10: das Bodenplattenband ist für das 15-mm-Merkmal 20 mm dick, die 12-mm-Verschleißleisten des Löffels bleiben 1:50 vorbehalten; Tritte, Haltestangen und Dachscheinwerfer wie bei 1:50.")
s.decide(200, "Rework 10: track and bucket modelled anew for the 30 mm feature instead of the 1:50 parts: one 40 mm shoe band with the grousers in it on an 85 mm link chain, a plain sprocket disc, plain 180 mm roller treads, a plain pin, barrel and rod for the adjuster, and a bucket of 30 mm shell and cheeks.", "Überarbeitung 10: Kette und Löffel für das 30-mm-Merkmal neu modelliert statt der 1:50-Teile: ein 40-mm-Bodenplattenband einschließlich Stollen auf einer 85-mm-Gliederkette, glatte Antriebsradscheibe, glatte Laufrollen, Spanner aus glattem Bolzen, Rohr und Stange, Löffel mit 30 mm Schale und Wangen.")
s.decide(500, "Rework 10: track and bucket modelled anew for the 75 mm feature: one 125 mm band (shoe and chain), plain sprocket and idler discs, plain rollers, a closed box track frame with two 75 mm idler arms, and a bucket of 75 mm shell, cheeks and ears with its five teeth.", "Überarbeitung 10: Kette und Löffel für das 75-mm-Merkmal neu modelliert: ein 125-mm-Band (Platte und Kette), glatte Antriebs- und Leitradscheiben, glatte Laufrollen, geschlossener Kastenrahmen mit zwei 75-mm-Leitradarmen, Löffel mit 75 mm Schale, Wangen und Laschen und seinen fünf Zähnen.")
s.save()
