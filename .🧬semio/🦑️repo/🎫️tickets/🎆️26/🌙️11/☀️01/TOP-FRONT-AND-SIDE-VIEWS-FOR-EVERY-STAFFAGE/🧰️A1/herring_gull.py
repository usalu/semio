from kit import *
ID = 'herring-gull'
NM = {'n': 'near', 'f': 'far'}


def front(k, T):
    k.ell('body', 'trunk', 0, 0, .08, .097, 'body-plumage', 3)
    if T == 50:
        k.ell('mantle', 'trunk', 0, .045, .06, .045, 'body-plumage', 3.1)
    k.col('tail', 'tail', 0, 0, .045, .035, 'tail-plumage', 2, y0=k.o('tail')[1] + .045, y1=k.t('tail')[1] - .04)
    n1o, n1t, n2o, n2t = k.o('neck-1')[1], k.t('neck-1')[1], k.o('neck-2')[1], k.t('neck-2')[1]
    k.col('neck-1', 'neck-1', 0, 0, .052, .048, 'neck-plumage', 3.2, y0=n1o - .06, y1=n1t)
    k.col('neck-2', 'neck-2', 0, 0, .048, .042, 'neck-plumage', 3.3, y0=n2o, y1=n2t)
    k.ell('head', 'head', 0, .171, .045, .045, 'head-plumage', 4)
    k.ell('bill', 'head', 0, .158, .016, .02, 'bill', 4.1)
    for sd, sg in (('n', -1), ('f', 1)):
        nme = NM[sd]
        zw = 4 if sd == 'n' else 3.9
        if T <= 100:
            k.col(f'{nme}-arm', f'arm-{sd}', sg * .08, sg * .08, .017, .017, 'wing-plumage', zw, y0=k.o(f'arm-{sd}')[1] + .054, y1=k.t(f'arm-{sd}')[1] - .054)
            k.col(f'{nme}-hand', f'hand-{sd}', sg * .08, sg * .08, .015, .01, 'wing-plumage', zw + .1, y0=k.o(f'hand-{sd}')[1] + .042, y1=k.t(f'hand-{sd}')[1] - .042)
        else:
            k.col(f'{nme}-arm', f'arm-{sd}', sg * .08, sg * .08, .017, .017, 'wing-plumage', zw, y0=k.o(f'arm-{sd}')[1] + .054, y1=k.t(f'arm-{sd}')[1] - .054)
            k.col(f'{nme}-hand', f'hand-{sd}', sg * .08, sg * .08, .015, .01, 'wing-plumage', zw + .1, y0=k.o(f'hand-{sd}')[1] + .042, y1=k.t(f'hand-{sd}')[1] - .042)
        lz = 4.5 if sd == 'n' else 1.2
        if T == 50:
            k.col(f'{nme}-thigh', f'thigh-{sd}', sg * .045, sg * .042, .03, .022, 'body-plumage', 3.6 if sd == 'n' else 1.1)
            k.col(f'{nme}-shank', f'shank-{sd}', sg * .042, sg * .04, .011, .009, 'legs-skin', lz)
            k.col(f'{nme}-tarsus', f'tarsus-{sd}', sg * .04, sg * .04, .009, .008, 'legs-skin', lz)
            k.poly(f'{nme}-foot', f'foot-{sd}', [(sg * .04 - .034, -.248), (sg * .04 + .034, -.248), (sg * .04 + .03, -.258), (sg * .04 - .03, -.258)], 'web-feet', lz)
        else:
            k.col(f'{nme}-leg', f'shank-{sd}', sg * .042, sg * .04, .012, .011, ['legs-skin', 'web-feet'], lz, y0=k.o(f'shank-{sd}')[1], y1=-.258)


def plan(k, T):
    k.cap('body', 'trunk', 0, 0, .075, .085, 'body-plumage', 3, x0=.04, x1=.27)
    if T == 50:
        k.cap('mantle', 'trunk', 0, 0, .06, .06, 'body-plumage', 3.1, x0=.08, x1=.24)
    k.cap('tail', 'tail', 0, 0, .05, .065, 'tail-plumage', 2, dx1=-.0)
    k.cap('neck-1', 'neck-1', 0, 0, .042, .038, 'neck-plumage', 3.2)
    k.cap('neck-2', 'neck-2', 0, 0, .038, .036, 'neck-plumage', 3.3)
    hx = k.o('head')[0]
    k.cap('head', 'head', 0, 0, .042, .036, 'head-plumage', 4, x0=hx, x1=hx + .031)
    k.cap('bill', 'head', 0, 0, .016, .008, 'bill', 4.1, x0=hx + .045, x1=hx + .117)
    for sd, sg in (('n', -1), ('f', 1)):
        nme = NM[sd]
        zw = 4 if sd == 'n' else 3.9
        k.cap(f'{nme}-arm', f'arm-{sd}', sg * .068, sg * .068, .03, .028, 'wing-plumage', zw)
        k.cap(f'{nme}-hand', f'hand-{sd}', sg * .068, sg * .055, .028, .012, 'wing-plumage', zw + .1, dx1=-.03)
        if T == 50:
            k.cap(f'{nme}-thigh', f'thigh-{sd}', sg * .042, sg * .042, .03, .026, 'body-plumage', 1.1)
            k.cap(f'{nme}-shank', f'shank-{sd}', sg * .04, sg * .04, .012, .01, 'legs-skin', 1.2)
            k.cap(f'{nme}-tarsus', f'tarsus-{sd}', sg * .04, sg * .04, .01, .009, 'legs-skin', 1.2)
            k.cap(f'{nme}-foot', f'foot-{sd}', sg * .04, sg * .04, .02, .03, 'web-feet', 1.2, dx1=.0)
        else:
            k.cap(f'{nme}-leg', f'shank-{sd}', sg * .04, sg * .04, .014, .03, ['legs-skin', 'web-feet'], 1.2, x0=k.o(f'thigh-{sd}')[0], x1=k.t(f'foot-{sd}')[0])


TEXT = {
    50: ("the rest pose carries the folded wing, so the body is an ellipse and a capsule at the 0.19 m folded width, the folded wings lie as separate arm and hand columns (front) or capsules (plan) that stretch with their bones when the wing is swept up or back, the neck is two followTip columns, the legs are thigh, shank, tarsus and a flat web foot, and a mantle patch, a bill and a tail fan are added. The rig swings the wing only in the side plane, so the spread wingspan of a soaring gull is not drawn: the lateral position stays that of the folded wing.",
         "Die Ruhepose trägt den gefalteten Flügel, daher sind Rumpf eine Ellipse und eine Kapsel in der gefalteten Breite von 0,19 m, die gefalteten Flügel liegen als getrennte Arm- und Handsäulen (Vorderansicht) oder -kapseln (Grundriss), die sich mit ihren Knochen strecken, wenn der Flügel nach oben oder hinten schwingt, der Hals besteht aus zwei followTip-Säulen, die Beine aus Oberschenkel, Unterschenkel, Lauf und flachem Schwimmfuß, dazu kommen ein Mantelfleck, ein Schnabel und ein Schwanzfächer. Das Rig schwenkt den Flügel nur in der Seitenebene, daher wird die gespreizte Spannweite eines segelnden Vogels nicht gezeichnet: die seitliche Lage bleibt die des gefalteten Flügels."),
    100: ("the same bone-bound masses without the mantle: body, tail, two neck columns, head with bill, arm and hand of each wing as separate shapes and one stroke-like leg piece per side.",
          "dieselben knochengebundenen Massen ohne Mantelfleck: Rumpf, Schwanz, zwei Halssäulen, Kopf mit Schnabel, Arm und Hand jedes Flügels als getrennte Formen und je Seite ein strichartiges Beinstück."),
    200: ("main masses only: body, tail, two neck pieces, head with bill, arm and hand per wing and one leg piece per side, all bound to the same bones as the side view; the wings stay at the folded lateral position.",
          "nur die Hauptmassen: Rumpf, Schwanz, zwei Halsstücke, Kopf mit Schnabel, Arm und Hand je Flügel und ein Beinstück je Seite, alle an dieselben Knochen gebunden wie in der Seitenansicht; die Flügel bleiben in der gefalteten seitlichen Lage."),
}


def build(doc):
    a = doc['asset']
    res, txt = {}, {}
    for sc in (50, 100, 200):
        if a['representations'][str(sc)]['status'] != 'authored':
            continue
        kf = K(a, sc); front(kf, sc)
        kp = K(a, sc); plan(kp, sc)
        res[sc] = {'front': kf.shapes, 'plan': kp.shapes}
        txt[sc] = TEXT[sc]
    return finish(doc, res, txt)
