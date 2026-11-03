from legs import *
ID = 'goat'

U, LL, HV = 'coat-upper-legs', 'lower-legs', 'hooves'
FORE = [seg('scapula-f{s}', .09, .09, .06, .06, U, 'shoulder'), seg('humerus-f{s}', .09, .09, .055, .045, U, 'humerus'),
        seg('forearm-f{s}', .09, .085, .04, .03, U, 'forearm'), seg('metacarpus-f{s}', .085, .085, .025, .022, LL, 'cannon-f'),
        seg('hoof-f{s}', .085, .085, .03, .036, HV, 'hoof-f')]
HIND = [seg('thigh-h{s}', .09, .09, .07, .055, U, 'thigh'), seg('shank-h{s}', .09, .09, .05, .035, U, 'gaskin'),
        seg('metatarsus-h{s}', .09, .09, .027, .024, LL, 'cannon-h'), seg('hoof-h{s}', .09, .09, .03, .036, HV, 'hoof-h')]
GF = {200: [(1, [0, 1], 'shoulder'), (2, [2], 'forearm'), (3, [3, 4], 'fore-lower')], 500: [(2, [0, 1, 2], 'leg-fore-upper'), (3, [3, 4], 'leg-fore-lower')]}
GH = {200: [(0, [0], 'thigh'), (1, [1], 'gaskin'), (2, [2, 3], 'hind-lower')], 500: [(1, [0, 1], 'leg-hind-upper'), (2, [2, 3], 'leg-hind-lower')]}


def front(k, T):
    for sd, sg in (('n', -1), ('f', 1)):
        leg_front(k, T, sd, sg, FORE, GF, 1.5, ground=-.003)
        leg_front(k, T, sd, sg, HIND, GH, 1.0, ground=-.003)
    trunk = ['coat-trunk', 'tail'] if T == 200 else (['coat-trunk', 'coat-upper-legs', 'tail'] if T == 500 else 'coat-trunk')
    chest = ['coat-trunk', 'coat-upper-legs'] if T == 500 else 'coat-trunk'
    k.ell('hindbody', 'lumbar', 0, -.045, .15, .2, trunk, 3)
    k.ell('chest', 'thorax', 0, -.055, .165, .21, chest, 3.2)
    oy, ty = k.o('neck')[1], k.t('neck')[1]
    k.col('neck', 'neck', 0, 0, .1, .075, 'coat-neck' if T < 500 else ['coat-neck'], 2.9, y0=oy + .204, y1=ty - .278)
    if T <= 100:
        k.col('head', 'head', 0, 0, .06, .04, 'face', 4, y0=.33, dy1=-.024, cap0=.06)
        for sd, sg in (('n', -1), ('f', 1)):
            k.poly(f'ear-{sd}', f'ear-{sd}', [(sg * .05, .27), (sg * .15, .29), (sg * .16, .265), (sg * .05, .25)], 'ears', 4.5 if sd == 'n' else 4.4)
            k.col(f'horn-{sd}', 'head', sg * .045, sg * .075, .014, .005, 'horns', 4.4 if sd == 'n' else 4.3, y0=.287, y1=.454, follow=False)
        if T == 50:
            for sg in (-1, 1):
                k.ell(f'eye-{"n" if sg < 0 else "f"}', 'head', sg * .045, .245, .01, .008, 'face', 4.6, role='detail')
            k.col('beard', 'jaw', 0, 0, .02, .012, 'face', 3.95, y0=.144, y1=.067)
        k.col('tail-root', 'tail-1', 0, 0, .025, .02, 'tail', 2.8)
        k.col('tail-tip', 'tail-2', 0, 0, .02, .012, 'tail', 2.8)
    elif T == 200:
        k.col('head', 'head', 0, 0, .06, .04, 'face', 4, y0=.33, dy1=-.024)
        k.poly('ear', 'ear-n', [(-.05, .27), (-.15, .29), (-.16, .265), (-.05, .25)], 'ears', 4.5)
        k.col('horn', 'head', -.045, -.075, .014, .005, 'horns', 4.4, y0=.287, y1=.454, follow=False)
        k.col('tail', 'tail-1', 0, 0, .025, .012, 'tail', 2.8, y1=.192)
    else:
        k.ell('head', 'head', 0, .215, .068, .115, ['face', 'ears'], 4)
        k.poly('ear-span', 'ear-n', [(-.16, .275), (.16, .275)], ['ears'], 3.9, closed=False, filled=False)
        k.poly('horn', 'head', [(-.045, .287), (-.075, .454)], ['horns'], 4.4, closed=False, filled=False)


def plan(k, T):
    for sd, sg in (('n', -1), ('f', 1)):
        leg_plan(k, T, sd, sg, FORE, GF, 1.5, toe=.02)
        leg_plan(k, T, sd, sg, HIND, GH, 1.0, toe=.02)
    trunk = ['coat-trunk', 'tail'] if T == 200 else (['coat-trunk', 'coat-upper-legs', 'tail'] if T == 500 else 'coat-trunk')
    chest = ['coat-trunk', 'coat-upper-legs'] if T == 500 else 'coat-trunk'
    k.cap('hindbody', 'lumbar', 0, 0, .12, .16, trunk, 3, x0=-.005, x1=.4)
    k.cap('chest', 'thorax', 0, 0, .17, .16, chest, 3.2, x0=.55, x1=.65)
    k.cap('neck', 'neck', 0, 0, .09, .07, 'coat-neck' if T < 500 else ['coat-neck'], 2.9)
    hx = k.o('head')[0]
    if T <= 100:
        k.cap('head', 'head', 0, 0, .06, .035, 'face', 4, dx1=-.015)
        for sd, sg in (('n', -1), ('f', 1)):
            k.cap(f'ear-{sd}', f'ear-{sd}', sg * .05, sg * .14, .028, .024, 'ears', 3.9, dx1=.02)
            k.cap(f'horn-{sd}', 'head', sg * .04, sg * .075, .014, .01, 'horns', 4.4, x0=hx - .0, x1=hx - .06)
        if T == 50:
            for sg in (-1, 1):
                k.cap(f'eye-{"n" if sg < 0 else "f"}', 'head', sg * .045, sg * .045, .009, .009, 'face', 4.6, x0=hx + .07, x1=hx + .075, role='detail')
            k.cap('beard', 'jaw', 0, 0, .02, .014, 'face', 3.95, x0=k.t('jaw')[0] - .05, x1=k.t('jaw')[0] - .02)
        k.cap('tail-root', 'tail-1', 0, 0, .025, .02, 'tail', 2.8)
        k.cap('tail-tip', 'tail-2', 0, 0, .02, .012, 'tail', 2.8)
    elif T == 200:
        k.cap('head', 'head', 0, 0, .06, .035, 'face', 4, dx1=-.015)
        k.cap('ear', 'ear-n', -.05, -.14, .028, .024, 'ears', 3.9, dx1=.02)
        k.cap('horn', 'head', -.04, -.075, .014, .01, 'horns', 4.4, x0=hx, x1=hx - .06)
        k.cap('tail', 'tail-1', 0, 0, .025, .012, 'tail', 2.8, x1=k.t('tail-2')[0])
    else:
        k.cap('head', 'head', 0, 0, .07, .058, ['face', 'ears'], 4, dx1=-.03)
        k.poly('ear-span', 'ear-n', [(hx + .07, -.14), (hx + .09, .14)], ['ears'], 3.9, closed=False, filled=False)
        k.poly('horn', 'head', [(hx, -.04), (hx - .06, -.075)], ['horns'], 4.4, closed=False, filled=False)


TEXT = {
    50: ("the body is two ellipses and capsules at the true 0.33 m width, the neck a followTip column that bends down when the goat browses or climbs, every leg segment is its own column (front) or capsule (plan) so walking and the ledge climb stretch correctly, the horns rise from the head, the ears stand out sideways, and a beard, eyes and a short tail are added; from above the trunk hides the legs.",
         "Der Körper besteht aus zwei Ellipsen und Kapseln in der wahren Breite von 0,33 m, der Hals ist eine followTip-Säule, die beim Äsen und Klettern nach unten abknickt, jedes Beinglied ist eine eigene Säule (Vorderansicht) oder Kapsel (Grundriss), sodass Gehen und das Klettern über die Kante sich richtig strecken, die Hörner steigen vom Kopf auf, die Ohren stehen seitlich ab, dazu kommen Bart, Augen und ein kurzer Schwanz; von oben verdeckt der Rumpf die Beine."),
    100: ("the same bone-bound masses without detail lines: trunk and chest, a neck column, head with two horns and two ears, two tail pieces and every leg segment as its own followTip column or capsule.",
          "dieselben knochengebundenen Massen ohne Detaillinien: Rumpf und Brust, eine Halssäule, der Kopf mit zwei Hörnern und zwei Ohren, zwei Schwanzstücke und jedes Beinglied als eigene followTip-Säule oder -Kapsel."),
    200: ("main masses only: hindquarters, chest, neck, head, one horn mass, one ear bar, one tail piece, and each leg in three bone-bound pieces (shoulder or thigh, forearm or gaskin, and a lower piece with cannon and hoof).",
          "nur die Hauptmassen: Hinterhand, Brust, Hals, Kopf, eine Hornmasse, ein Ohrbalken, ein Schwanzstück und jedes Bein in drei knochengebundenen Stücken (Schulter oder Oberschenkel, Unterarm oder Unterschenkel und ein unteres Stück mit Röhrbein und Klaue)."),
    500: ("a composed silhouette: hindquarters and chest, neck, one head mass with its ears as one bar, one horn mark, and each leg as an upper piece and a lower piece bound to the same bones as the side view.",
          "eine zusammengesetzte Silhouette: Hinterhand und Brust, Hals, eine Kopfmasse mit den Ohren als ein Balken, eine Hornmarke und jedes Bein als oberes und unteres Stück, an dieselben Knochen gebunden wie in der Seitenansicht."),
}


def build(doc):
    a = doc['asset']
    res, txt = {}, {}
    for sc in (50, 100, 200, 500):
        if a['representations'][str(sc)]['status'] != 'authored':
            continue
        kf = K(a, sc); front(kf, sc)
        kp = K(a, sc); plan(kp, sc)
        res[sc] = {'front': kf.shapes, 'plan': kp.shapes}
        txt[sc] = TEXT[sc]
    return finish(doc, res, txt)
