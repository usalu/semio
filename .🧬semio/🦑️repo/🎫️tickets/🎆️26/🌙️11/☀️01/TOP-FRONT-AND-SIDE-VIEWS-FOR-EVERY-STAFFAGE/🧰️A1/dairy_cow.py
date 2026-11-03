from legs import *
ID = 'dairy-cow'

FORE = [seg('scapula-f{s}', .2, .2, .11, .11, 'coat-forelimbs', 'scapula'), seg('humerus-f{s}', .2, .19, .11, .09, 'coat-forelimbs', 'humerus'),
        seg('forearm-f{s}', .19, .18, .08, .06, 'coat-forelimbs', 'forearm'), seg('cannon-f{s}', .18, .18, .05, .047, 'lower-legs', 'fore-cannon'),
        seg('hoof-f{s}', .18, .18, .06, .07, 'hooves', 'fore-hoof')]
HIND = [seg('thigh-h{s}', .22, .22, .13, .11, 'coat-hindlimbs', 'thigh'), seg('shank-h{s}', .22, .2, .09, .06, 'coat-hindlimbs', 'gaskin'),
        seg('metatarsus-h{s}', .2, .2, .05, .047, 'lower-legs', 'hind-cannon'), seg('hoof-h{s}', .2, .2, .06, .07, 'hooves', 'hind-hoof')]
GF = {200: [(1, [0, 1], 'fore-upper'), (2, [2], 'forearm'), (3, [3, 4], 'fore-lower')], 500: [(1, [0, 1], 'fore-upper'), (2, [2, 3, 4], 'fore-leg')]}
GH = {200: [(0, [0], 'thigh'), (1, [1], 'gaskin'), (2, [2, 3], 'hind-lower')], 500: [(0, [0], 'thigh'), (1, [1, 2, 3], 'hind-leg')]}


def front(k, T):
    for sd, sg in (('n', -1), ('f', 1)):
        leg_front(k, T, sd, sg, FORE, GF, 1.5)
        leg_front(k, T, sd, sg, HIND, GH, 1.0)
    trunk = ['coat-trunk', 'coat-patches', 'flank-rumen'] if T >= 200 else 'coat-trunk'
    k.ell('hindbody', 'lumbar', 0, -.067, .33, .42, trunk, 3)
    k.ell('chest', 'thorax', 0, -.065, .35, .435, trunk if T >= 200 else 'coat-trunk', 3.2)
    oy = k.o('neck')[1]
    ty = k.t('neck')[1]
    if T <= 100:
        k.ell('patch-rump', 'lumbar', -.1, .05, .15, .2, 'coat-patches', 3.4)
        k.ell('patch-chest', 'thorax', .12, .1, .17, .17, 'coat-patches', 3.4)
        k.ell('rumen-flank', 'lumbar', .3, -.02, .04, .18, 'flank-rumen', 3.5)
    if T <= 100:
        k.col('udder', 'udder', 0, 0, .12, .09, 'udder', 2.9, y0=-.4, y1=-.63)
        k.col('teat-near', 'udder', -.06, -.06, .016, .014, 'udder', 2.95, y0=-.6, y1=-.67)
        k.col('teat-far', 'udder', .06, .06, .016, .014, 'udder', 2.95, y0=-.6, y1=-.67)
    else:
        k.col('udder', 'udder', 0, 0, .14, .12, 'udder', 2.9, y0=-.4, y1=-.68)
    if T == 500:
        k.ell('neck', 'neck', 0, .105, .17, .283, 'coat-head-neck', 3.3)
    else:
        k.col('neck', 'neck', 0, 0, .19, .15, 'coat-head-neck', 3.3, y0=oy + .26, y1=ty - .38)
    if T <= 100:
        k.col('head', 'head', 0, 0, .11, .07, 'coat-head-neck', 4, y0=.27, dy1=-.066, cap0=.1)
        k.col('muzzle', 'head', 0, 0, .08, .075, 'muzzle', 4.2, y0=-.2, dy1=-.066)
        for sd, sg in (('n', -1), ('f', 1)):
            k.poly(f'{NM[sd]}-ear', f'ear-{sd}', [(sg * .1, .2), (sg * .3, .22), (sg * .31, .19), (sg * .1, .165)], 'ears', 4.5 if sd == 'n' else 4.4)
            k.ell(f'{NM[sd]}-eye', 'head', sg * .1, .15, .015, .011, 'eyes', 5, role='detail' if T == 50 else 'body')
        if T == 50:
            k.col('blaze', 'head', 0, 0, .03, .02, 'coat-head-neck', 4.1, y0=.2, dy1=-.14)
            for sg in (-1, 1):
                k.ell(f'nostril-{"n" if sg < 0 else "f"}', 'head', sg * .035, -.27, .012, .009, None, 5, role='detail', filled=False)
    elif T == 200:
        k.col('head', 'head', 0, 0, .11, .07, ['coat-head-neck', 'muzzle', 'eyes'], 4, y0=.27, dy1=-.066)
        for sd, sg in (('n', -1), ('f', 1)):
            k.poly(f'{NM[sd]}-ear', f'ear-{sd}', [(sg * .1, .2), (sg * .3, .22), (sg * .31, .19), (sg * .1, .165)], 'ears', 4.5)
    else:
        k.col('head', 'head', 0, 0, .11, .07, ['coat-head-neck', 'ears', 'muzzle', 'eyes'], 4, y0=.27, dy1=-.066)
        k.poly('ear-span', 'ear-n', [(-.31, .2), (.31, .2)], 'ears', 3.9, closed=False, filled=False)
    if T <= 100:
        k.col('tail-1', 'tail-1', 0, 0, .04, .035, 'coat-tail', 2.8)
        k.col('tail-2', 'tail-2', 0, 0, .035, .03, 'coat-tail', 2.8)
        k.col('tail-3', 'tail-3', 0, 0, .03, .028, 'coat-tail', 2.8)
        k.col('tail-switch', 'tail-3', 0, 0, .06, .07, 'coat-tail', 2.85, y0=-.6, y1=-.78)
    elif T == 200:
        k.col('tail-1', 'tail-1', 0, 0, .04, .035, 'coat-tail', 2.8)
        k.col('tail-2', 'tail-2', 0, 0, .035, .03, 'coat-tail', 2.8)
        k.col('tail-3', 'tail-3', 0, 0, .05, .065, 'coat-tail', 2.85, y1=-.78)
    else:
        k.col('tail', 'tail-2', 0, 0, .04, .04, 'coat-tail', 2.8, y0=.27, y1=-.78)


def plan(k, T):
    for sd, sg in (('n', -1), ('f', 1)):
        leg_plan(k, T, sd, sg, FORE, GF, 1.5)
        leg_plan(k, T, sd, sg, HIND, GH, 1.0)
    trunk = ['coat-trunk', 'coat-patches', 'flank-rumen'] if T >= 200 else 'coat-trunk'
    k.cap('hindbody', 'lumbar', 0, 0, .2, .31, trunk, 3, x0=-.148, x1=.5)
    k.cap('chest', 'thorax', 0, 0, .35, .3, trunk, 3.2, x0=.5, x1=1.1)
    k.cap('udder', 'udder', 0, 0, .12, .12, 'udder', 1.2, x0=.1, x1=.28)
    if T <= 100:
        k.cap('patch-rump', 'lumbar', 0, 0, .15, .17, 'coat-patches', 3.4, x0=.05, x1=.4)
        k.cap('patch-barrel', 'thorax', .06, .06, .17, .17, 'coat-patches', 3.4, x0=.8, x1=1.05)
        k.cap('rumen-flank', 'lumbar', .26, .26, .06, .06, 'flank-rumen', 3.5, x0=.3, x1=.6)
    k.cap('neck', 'neck', 0, 0, .18, .13, 'coat-head-neck', 3.3)
    if T <= 100:
        k.cap('head', 'head', 0, 0, .11, .07, 'coat-head-neck', 4, dx1=-.03)
        k.cap('muzzle', 'head', 0, 0, .08, .075, 'muzzle', 4.2, x0=k.t('head')[0] - .06, dx1=-.03)
        for sd, sg in (('n', -1), ('f', 1)):
            k.cap(f'{NM[sd]}-ear', f'ear-{sd}', sg * .1, sg * .28, .045, .04, 'ears', 3.9, dx1=-.03)
        if T == 50:
            for sg in (-1, 1):
                k.cap(f'eye-{"n" if sg < 0 else "f"}', 'head', sg * .1, sg * .1, .013, .013, 'eyes', 5, x0=k.o('head')[0] + .06, x1=k.o('head')[0] + .08)
            k.cap('blaze', 'head', 0, 0, .025, .018, 'coat-head-neck', 4.1, x0=k.o('head')[0] + .02, x1=k.t('head')[0] - .06)
    elif T == 200:
        k.cap('head', 'head', 0, 0, .11, .07, ['coat-head-neck', 'muzzle', 'eyes'], 4, dx1=-.03)
        for sd, sg in (('n', -1), ('f', 1)):
            k.cap(f'{NM[sd]}-ear', f'ear-{sd}', sg * .1, sg * .28, .045, .04, 'ears', 3.9, dx1=-.03)
    else:
        k.cap('head', 'head', 0, 0, .11, .07, ['coat-head-neck', 'ears', 'muzzle', 'eyes'], 4, dx1=-.03)
        k.poly('ear-span', 'ear-n', [(k.o('ear-n')[0], -.28), (k.o('ear-n')[0] - .03, .28)], 'ears', 3.9, closed=False, filled=False)
    if T <= 100:
        k.cap('tail-1', 'tail-1', 0, 0, .04, .035, 'coat-tail', 2.8)
        k.cap('tail-2', 'tail-2', 0, 0, .035, .03, 'coat-tail', 2.8)
        k.cap('tail-3', 'tail-3', 0, 0, .03, .028, 'coat-tail', 2.8)
        k.cap('tail-switch', 'tail-3', 0, 0, .05, .06, 'coat-tail', 2.85, x0=k.t('tail-3')[0], x1=k.t('tail-3')[0] - .03)
    elif T == 200:
        k.cap('tail-1', 'tail-1', 0, 0, .04, .035, 'coat-tail', 2.8)
        k.cap('tail-2', 'tail-2', 0, 0, .035, .03, 'coat-tail', 2.8)
        k.cap('tail-3', 'tail-3', 0, 0, .04, .05, 'coat-tail', 2.85, dx1=-.03)
    else:
        k.cap('tail', 'tail-2', 0, 0, .04, .04, 'coat-tail', 2.8, x0=k.o('tail-1')[0], x1=k.t('tail-3')[0] - .03)


TEXT = {
    50: ("the barrel and the hindquarters are ellipses and capsules at the true 0.70 m width, the udder with two teats, the black patches and the rumen flank are laid on the coat, the neck is a followTip column that bends down with the head, every leg segment is its own column (front) or capsule (plan) so strides and kneeling stretch correctly, the ears stand out sideways, and eyes, blaze and nostrils are added; from above the barrel hides the legs and the udder.",
         "Rumpf und Hinterhand sind Ellipsen und Kapseln in der wahren Breite von 0,70 m, das Euter mit zwei Zitzen, die schwarzen Flecken und die Pansenflanke liegen auf dem Fell, der Hals ist eine followTip-Säule, die mit dem Kopf nach unten abknickt, jedes Beinglied ist eine eigene Säule (Vorderansicht) oder Kapsel (Grundriss), sodass Schritte und Niederknien sich richtig strecken, die Ohren stehen seitlich ab, dazu kommen Augen, Blesse und Nüstern; von oben verdecken Rumpf die Beine und das Euter."),
    100: ("the same bone-bound masses without detail lines: barrel and hindquarters, udder with two teats, coat patches, a neck column, head with muzzle, eyes and two ears, and every leg segment as its own followTip column or capsule.",
          "dieselben knochengebundenen Massen ohne Detaillinien: Rumpf und Hinterhand, Euter mit zwei Zitzen, Fellflecken, eine Halssäule, der Kopf mit Maul, Augen und zwei Ohren und jedes Beinglied als eigene followTip-Säule oder -Kapsel."),
    200: ("main masses only: hindquarters and chest with the patches merged into their regions, the udder, one neck mass, one head mass with two ears, three tail pieces, and each leg in three bone-bound pieces (upper arm or thigh, forearm or gaskin, and a lower piece with cannon and hoof).",
          "nur die Hauptmassen: Hinterhand und Brust mit in die Regionen eingeschmolzenen Flecken, das Euter, eine Halsmasse, eine Kopfmasse mit zwei Ohren, drei Schwanzstücke und jedes Bein in drei knochengebundenen Stücken (Oberarm oder Oberschenkel, Unterarm oder Unterschenkel und ein unteres Stück mit Röhrbein und Klaue)."),
    500: ("a composed silhouette: hindquarters and chest, udder, neck, one head mass whose ears are one bar across the head, one tail piece, and each leg as a thick upper piece and one lower piece bound to the same bones as the side view.",
          "eine zusammengesetzte Silhouette: Hinterhand und Brust, Euter, Hals, eine Kopfmasse, deren Ohren ein Balken quer über dem Kopf sind, ein Schwanzstück und jedes Bein als kräftiges oberes und ein unteres Stück, an dieselben Knochen gebunden wie in der Seitenansicht."),
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
