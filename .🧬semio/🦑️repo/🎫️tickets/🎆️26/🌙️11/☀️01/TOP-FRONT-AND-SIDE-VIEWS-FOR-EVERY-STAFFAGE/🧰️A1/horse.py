from kit import *
ID = 'horse'
NM = {'n': 'near', 'f': 'far'}


def front(k, T):
    zf, zh = 1.5, 1.0
    for sd, sg in (('n', -1), ('f', 1)):
        L = lambda v: sg * v
        nme = NM[sd]
        if T <= 100:
            k.col(f'{nme}-scapula', f'scapula-f{sd}', L(.19), L(.19), .10, .10, 'coat-forelimbs', zf)
            k.col(f'{nme}-humerus', f'humerus-f{sd}', L(.19), L(.17), .10, .085, 'coat-forelimbs', zf)
            k.col(f'{nme}-radius', f'radius-f{sd}', L(.17), L(.155), .075, .055, 'coat-forelimbs', zf)
            k.col(f'{nme}-fore-cannon', f'cannon-f{sd}', L(.155), L(.155), .046, .042, 'coat-forelimbs', zf)
            k.col(f'{nme}-fore-pastern', f'pastern-f{sd}', L(.155), L(.155), .045, .05, 'coat-forelimbs', zf)
            k.col(f'{nme}-fore-hoof', f'hoof-f{sd}', L(.155), L(.155), .058, .07, 'hooves', zf, dy1=-.005)
            k.col(f'{nme}-thigh', f'thigh-h{sd}', L(.2), L(.2), .115, .095, 'coat-hindlimbs', zh)
            k.col(f'{nme}-gaskin', f'shank-h{sd}', L(.2), L(.17), .085, .055, 'coat-hindlimbs', zh)
            k.col(f'{nme}-hind-cannon', f'cannon-h{sd}', L(.17), L(.165), .048, .043, 'coat-hindlimbs', zh)
            k.col(f'{nme}-hind-pastern', f'pastern-h{sd}', L(.165), L(.165), .046, .05, 'coat-hindlimbs', zh)
            k.col(f'{nme}-hind-hoof', f'hoof-h{sd}', L(.165), L(.165), .058, .07, 'hooves', zh, dy1=-.005)
        elif T == 200:
            k.col(f'{nme}-fore-upper', f'humerus-f{sd}', L(.19), L(.17), .10, .085, 'coat-forelimbs', zf, y0=.3)
            k.col(f'{nme}-radius', f'radius-f{sd}', L(.17), L(.155), .075, .055, 'coat-forelimbs', zf)
            k.col(f'{nme}-fore-lower', f'cannon-f{sd}', L(.155), L(.155), .046, .06, ['coat-forelimbs', 'hooves'], zf, y1=-1.3188)
            k.col(f'{nme}-thigh', f'thigh-h{sd}', L(.2), L(.2), .115, .095, 'coat-hindlimbs', zh)
            k.col(f'{nme}-gaskin', f'shank-h{sd}', L(.2), L(.17), .085, .055, 'coat-hindlimbs', zh)
            k.col(f'{nme}-hind-lower', f'cannon-h{sd}', L(.17), L(.165), .048, .06, ['coat-hindlimbs', 'hooves'], zh, y1=-1.3188)
        else:
            k.col(f'{nme}-fore-upper', f'humerus-f{sd}', L(.19), L(.17), .10, .085, 'coat-forelimbs', zf, y0=.2)
            k.col(f'{nme}-forearm', f'radius-f{sd}', L(.17), L(.155), .07, .05, 'coat-forelimbs', zf)
            k.col(f'{nme}-fore-lower', f'cannon-f{sd}', L(.155), L(.155), .04, .045, ['coat-forelimbs', 'hooves'], zf, y1=-1.3188)
            k.col(f'{nme}-hind-upper', f'thigh-h{sd}', L(.2), L(.2), .115, .095, 'coat-hindlimbs', zh)
            k.col(f'{nme}-gaskin', f'shank-h{sd}', L(.2), L(.17), .08, .05, 'coat-hindlimbs', zh)
            k.col(f'{nme}-hind-lower', f'cannon-h{sd}', L(.17), L(.165), .04, .045, ['coat-hindlimbs', 'hooves'], zh, y1=-1.3188)
    k.ell('hindbody', 'lumbar', 0, -.1, .295, .375, 'coat-trunk', 3)
    k.ell('chest', 'thorax', 0, -.145, .295, .425, 'coat-trunk', 3.2)
    if T <= 100:
        for i, (wa, wb) in enumerate([(.05, .05), (.05, .048), (.048, .045), (.045, .04)], 1):
            k.col(f'tail-{i}', f'tail-{i}', 0, 0, wa, wb, 'tail', 2.8)
    elif T == 200:
        k.col('tail-1', 'tail-1', 0, 0, .05, .05, 'tail', 2.8, y1=k.t('tail-2')[1])
        k.col('tail-2', 'tail-3', 0, 0, .047, .04, 'tail', 2.8, y0=k.o('tail-3')[1], y1=k.t('tail-4')[1])
    else:
        k.col('tail-upper', 'tail-2', 0, 0, .05, .045, 'tail', 2.8, y0=.14, y1=k.t('tail-2')[1])
        k.col('tail-lower', 'tail-4', 0, 0, .042, .038, 'tail', 2.8, y0=k.o('tail-3')[1], y1=k.t('tail-4')[1])
    if T <= 100:
        k.col('neck-1', 'neck-1', 0, 0, .15, .13, 'coat-trunk', 3.3)
        k.col('neck-2', 'neck-2', 0, 0, .13, .11, 'coat-trunk', 3.3)
        k.col('neck-3', 'neck-3', 0, 0, .11, .1, 'coat-trunk', 3.3)
        if T == 50:
            k.col('mane-1', 'neck-1', 0, 0, .162, .142, 'mane', 3.2)
            k.col('mane-2', 'neck-2', 0, 0, .142, .122, 'mane', 3.2)
            k.col('mane-3', 'neck-3', 0, 0, .122, .112, 'mane', 3.2)
        k.col('head', 'head', 0, 0, .09, .058, 'coat-head', 4, y0=.785, cap0=.09)
        k.col('muzzle', 'head', 0, 0, .052, .05, 'muzzle-skin', 4.2, y0=.37, y1=.285)
        for sd, sg in (('n', -1), ('f', 1)):
            nme = NM[sd]
            k.col(f'{nme}-ear', f'ear-{sd}', sg * .075, sg * .062, .03, .006, 'ears', 4.5 if sd == 'n' else 4.4, y1=.925)
            k.ell(f'{nme}-eye', 'head', sg * .088, .5, .014, .011, 'eyes', 5, role='detail' if T == 50 else 'body')
        if T == 50:
            k.col('forelock', 'head', 0, 0, .03, .022, 'mane', 4.1, y0=.76, y1=.63)
            for sg in (-1, 1):
                k.ell(f'nostril-{"n" if sg < 0 else "f"}', 'head', sg * .026, .32, .008, .01, None, 5, role='detail', filled=False)
    elif T == 200:
        k.col('neck-1', 'neck-1', 0, 0, .15, .13, 'coat-trunk', 3.3)
        k.col('neck-2', 'neck-2', 0, 0, .13, .11, ['coat-trunk', 'mane'], 3.3, y1=.722)
        k.col('head', 'head', 0, 0, .09, .058, ['coat-head', 'muzzle-skin', 'eyes'], 4, y0=.785, y1=.285)
        for sd, sg in (('n', -1), ('f', 1)):
            k.col(f'{NM[sd]}-ear', f'ear-{sd}', sg * .075, sg * .062, .03, .006, 'ears', 4.5, y1=.925)
    else:
        k.col('neck-a', 'neck-1', 0, 0, .15, .13, 'coat-trunk', 3.3)
        k.col('neck-b', 'neck-2', 0, 0, .13, .1, ['coat-trunk', 'mane'], 3.3, y1=.722)
        k.col('head', 'head', 0, 0, .09, .058, ['coat-head', 'muzzle-skin', 'eyes'], 4, y0=.785, y1=.285)
        k.col('ear-n', 'ear-n', -.075, -.062, .03, .006, 'ears', 4.5, y1=.925)
        k.col('ear-f', 'ear-f', .075, .062, .03, .006, 'ears', 4.5, y1=.925)


def plan(k, T):
    zf, zh = 1.5, 1.0
    for sd, sg in (('n', -1), ('f', 1)):
        L = lambda v: sg * v
        nme = NM[sd]
        hoofx = k.t(f'hoof-f{sd}')[0] + .025
        hoofh = k.t(f'hoof-h{sd}')[0] + .025
        if T <= 100:
            k.cap(f'{nme}-scapula', f'scapula-f{sd}', L(.19), L(.19), .1, .1, 'coat-forelimbs', zf)
            k.cap(f'{nme}-humerus', f'humerus-f{sd}', L(.19), L(.17), .1, .085, 'coat-forelimbs', zf)
            k.cap(f'{nme}-radius', f'radius-f{sd}', L(.17), L(.155), .075, .055, 'coat-forelimbs', zf)
            k.cap(f'{nme}-fore-cannon', f'cannon-f{sd}', L(.155), L(.155), .046, .042, 'coat-forelimbs', zf)
            k.cap(f'{nme}-fore-pastern', f'pastern-f{sd}', L(.155), L(.155), .045, .05, 'coat-forelimbs', zf)
            k.cap(f'{nme}-fore-hoof', f'hoof-f{sd}', L(.155), L(.155), .058, .065, 'hooves', zf, dx1=.025)
            k.cap(f'{nme}-thigh', f'thigh-h{sd}', L(.2), L(.2), .115, .095, 'coat-hindlimbs', zh)
            k.cap(f'{nme}-gaskin', f'shank-h{sd}', L(.2), L(.17), .085, .055, 'coat-hindlimbs', zh)
            k.cap(f'{nme}-hind-cannon', f'cannon-h{sd}', L(.17), L(.165), .048, .043, 'coat-hindlimbs', zh)
            k.cap(f'{nme}-hind-pastern', f'pastern-h{sd}', L(.165), L(.165), .046, .05, 'coat-hindlimbs', zh)
            k.cap(f'{nme}-hind-hoof', f'hoof-h{sd}', L(.165), L(.165), .058, .065, 'hooves', zh, dx1=.025)
        elif T == 200:
            k.cap(f'{nme}-fore-upper', f'humerus-f{sd}', L(.19), L(.17), .1, .085, 'coat-forelimbs', zf, x0=k.o(f'scapula-f{sd}')[0])
            k.cap(f'{nme}-radius', f'radius-f{sd}', L(.17), L(.155), .075, .055, 'coat-forelimbs', zf)
            k.cap(f'{nme}-fore-lower', f'cannon-f{sd}', L(.155), L(.155), .046, .06, ['coat-forelimbs', 'hooves'], zf, x1=hoofx)
            k.cap(f'{nme}-thigh', f'thigh-h{sd}', L(.2), L(.2), .115, .095, 'coat-hindlimbs', zh)
            k.cap(f'{nme}-gaskin', f'shank-h{sd}', L(.2), L(.17), .085, .055, 'coat-hindlimbs', zh)
            k.cap(f'{nme}-hind-lower', f'cannon-h{sd}', L(.17), L(.165), .048, .06, ['coat-hindlimbs', 'hooves'], zh, x1=hoofh)
        else:
            k.cap(f'{nme}-fore-upper', f'humerus-f{sd}', L(.19), L(.17), .1, .085, 'coat-forelimbs', zf, x0=k.o(f'scapula-f{sd}')[0])
            k.cap(f'{nme}-forearm', f'radius-f{sd}', L(.17), L(.155), .07, .05, 'coat-forelimbs', zf)
            k.cap(f'{nme}-fore-lower', f'cannon-f{sd}', L(.155), L(.155), .04, .045, ['coat-forelimbs', 'hooves'], zf, x1=hoofx)
            k.cap(f'{nme}-hind-upper', f'thigh-h{sd}', L(.2), L(.2), .115, .095, 'coat-hindlimbs', zh)
            k.cap(f'{nme}-gaskin', f'shank-h{sd}', L(.2), L(.17), .08, .05, 'coat-hindlimbs', zh)
            k.cap(f'{nme}-hind-lower', f'cannon-h{sd}', L(.17), L(.165), .04, .045, ['coat-hindlimbs', 'hooves'], zh, x1=hoofh)
    k.cap('hindbody', 'pelvis', 0, 0, .19, .3, 'coat-trunk', 3, x0=-.3, x1=.55)
    k.cap('chest', 'thorax', 0, 0, .3, .26, 'coat-trunk', 3.2, x0=.45, x1=1.28)
    hx = k.o('head')[0]
    if T <= 100:
        for i, (ra, rb) in enumerate([(.055, .052), (.052, .05), (.05, .046), (.046, .04)], 1):
            k.cap(f'tail-{i}', f'tail-{i}', 0, 0, ra, rb, 'tail', 2.8, dx1=-.03 if i == 4 else 0)
        k.cap('neck-1', 'neck-1', 0, 0, .14, .12, 'coat-trunk', 3.3)
        k.cap('neck-2', 'neck-2', 0, 0, .12, .105, 'coat-trunk', 3.3)
        k.cap('neck-3', 'neck-3', 0, 0, .105, .095, 'coat-trunk', 3.3)
        if T == 50:
            k.cap('mane-1', 'neck-1', 0, 0, .026, .026, 'mane', 3.4)
            k.cap('mane-2', 'neck-2', 0, 0, .026, .026, 'mane', 3.4)
            k.cap('mane-3', 'neck-3', 0, 0, .026, .026, 'mane', 3.4)
            k.cap('forelock', 'head', 0, 0, .02, .016, 'mane', 4.1, x0=hx + .06, x1=hx + .2)
        k.cap('head', 'head', 0, 0, .095, .052, 'coat-head', 4, dx1=-.005)
        k.cap('muzzle', 'head', 0, 0, .05, .046, 'muzzle-skin', 4.2, x0=k.t('head')[0] - .09, dx1=-.005)
        for sd, sg in (('n', -1), ('f', 1)):
            k.cap(f'{NM[sd]}-ear', f'ear-{sd}', sg * .07, sg * .07, .03, .026, 'ears', 4.5, dx0=-.02, dx1=.02)
        if T == 50:
            for sg in (-1, 1):
                k.cap(f'eye-{"n" if sg < 0 else "f"}', 'head', sg * .09, sg * .09, .012, .012, 'eyes', 5, x0=hx + .2, x1=hx + .22)
    elif T == 200:
        k.cap('tail-1', 'tail-1', 0, 0, .055, .05, 'tail', 2.8, x1=k.t('tail-2')[0])
        k.cap('tail-2', 'tail-3', 0, 0, .05, .04, 'tail', 2.8, x0=k.o('tail-3')[0], x1=k.t('tail-4')[0] - .03)
        k.cap('neck-1', 'neck-1', 0, 0, .14, .12, 'coat-trunk', 3.3)
        k.cap('neck-2', 'neck-2', 0, 0, .12, .1, ['coat-trunk', 'mane'], 3.3, dx1=.17)
        k.cap('head', 'head', 0, 0, .095, .05, ['coat-head', 'muzzle-skin', 'eyes'], 4, dx1=-.005)
        for sd, sg in (('n', -1), ('f', 1)):
            k.cap(f'{NM[sd]}-ear', f'ear-{sd}', sg * .07, sg * .07, .03, .026, 'ears', 4.5, dx0=-.02, dx1=.02)
    else:
        k.cap('tail-upper', 'tail-2', 0, 0, .055, .05, 'tail', 2.8, x0=k.o('tail-1')[0])
        k.cap('tail-lower', 'tail-4', 0, 0, .045, .04, 'tail', 2.8, x0=k.o('tail-3')[0], dx1=-.03)
        k.cap('neck-a', 'neck-1', 0, 0, .14, .12, 'coat-trunk', 3.3)
        k.cap('neck-b', 'neck-2', 0, 0, .12, .1, ['coat-trunk', 'mane'], 3.3, dx1=.17)
        k.cap('head', 'head', 0, 0, .095, .05, ['coat-head', 'muzzle-skin', 'eyes'], 4, dx1=-.005)
        k.cap('ear-n', 'ear-n', -.07, -.07, .035, .03, 'ears', 4.5, dx0=-.025, dx1=.025)
        k.cap('ear-f', 'ear-f', .07, .07, .035, .03, 'ears', 4.5, dx0=-.025, dx1=.025)


TEXT = {
    50: ("the barrel and rump are ellipses and capsules at the true 0.60 m width, the neck and head are tapered columns bound to their bones with followTip so a lowered head drops with its neck, every leg segment is its own column (front) or capsule (plan) so strides and bending knees stretch correctly, the hind legs stand wider apart than the forelegs and are drawn behind them, and ears, eyes, forelock, mane and nostrils are added; from above the barrel hides the legs and only swinging hooves leave it.",
         "Rumpf und Hinterhand sind Ellipsen und Kapseln in der wahren Breite von 0,60 m, Hals und Kopf sind verjüngte Säulen, die mit followTip an ihre Knochen gebunden sind und beim gesenkten Kopf mit dem Hals mitwandern, jedes Beinglied ist eine eigene Säule (Vorderansicht) oder Kapsel (Grundriss), sodass Schritte und gebeugte Gelenke sich richtig strecken, die Hinterbeine stehen weiter auseinander als die Vorderbeine und liegen dahinter, dazu kommen Ohren, Augen, Stirnschopf, Mähne und Nüstern; von oben verdeckt der Rumpf die Beine, nur schwingende Hufe treten hervor."),
    100: ("the same bone-bound masses without detail lines: barrel and rump as one ellipse or capsule each, the neck in three columns, the head with muzzle, eyes and two ears, and every leg segment as its own followTip column or capsule.",
          "dieselben knochengebundenen Massen ohne Detaillinien: Rumpf und Hinterhand je als eine Ellipse oder Kapsel, der Hals in drei Säulen, der Kopf mit Maul, Augen und zwei Ohren und jedes Beinglied als eigene followTip-Säule oder -Kapsel."),
    200: ("main masses only: rump, chest, two neck pieces with the mane merged in, one head mass and two ears; each leg in three bone-bound pieces (upper, forearm or gaskin, and a lower piece that carries cannon, pastern and hoof).",
          "nur die Hauptmassen: Hinterhand, Brust, zwei Halsstücke mit eingeschmolzener Mähne, eine Kopfmasse und zwei Ohren; jedes Bein in drei knochengebundenen Stücken (oben, Unterarm oder Unterschenkel und ein unteres Stück mit Röhrbein, Fessel und Huf)."),
    500: ("a composed silhouette: rump and chest, two neck pieces, one head, two ear marks, two tail pieces, and each leg as an upper, a middle and a thin lower piece, bound to the same bones as the side view.",
          "eine zusammengesetzte Silhouette: Hinterhand und Brust, zwei Halsstücke, ein Kopf, zwei Ohrenmarken, zwei Schweifstücke und jedes Bein als oberes, mittleres und dünnes unteres Stück, an dieselben Knochen gebunden wie in der Seitenansicht."),
}


def build(doc):
    a = doc['asset']
    res = {}
    txt = {}
    for sc in (50, 100, 200, 500):
        if a['representations'][str(sc)]['status'] != 'authored':
            continue
        kf = K(a, sc)
        front(kf, sc)
        kp = K(a, sc)
        plan(kp, sc)
        res[sc] = {'front': kf.shapes, 'plan': kp.shapes}
        txt[sc] = TEXT[sc]
    return finish(doc, res, txt)
