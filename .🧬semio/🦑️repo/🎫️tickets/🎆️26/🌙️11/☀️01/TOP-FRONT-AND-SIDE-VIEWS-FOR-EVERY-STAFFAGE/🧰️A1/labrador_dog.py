from legs import *
ID = 'labrador-dog'

CF, CH = 'coat-forelimbs', 'coat-hindlimbs'
FORE = [seg('scapula-f{s}', .085, .085, .05, .05, CF, 'scapula'), seg('humerus-f{s}', .085, .08, .05, .04, CF, 'humerus'),
        seg('forearm-f{s}', .08, .075, .035, .03, CF, 'forearm'), seg('metacarpus-f{s}', .075, .075, .028, .026, CF, 'wrist'),
        seg('paw-f{s}', .075, .075, .036, .04, [CF, 'pads'], 'forepaw')]
HIND = [seg('thigh-h{s}', .085, .085, .055, .045, CH, 'thigh'), seg('shank-h{s}', .085, .08, .045, .033, CH, 'shank'),
        seg('metatarsus-h{s}', .08, .08, .03, .027, CH, 'hock'), seg('paw-h{s}', .08, .08, .036, .04, [CH, 'pads'], 'hind-paw')]
GF = {200: [(1, [0, 1], 'fore-upper'), (2, [2], 'fore-forearm'), (3, [3, 4], 'fore-lower')], 500: [(1, [0, 1], 'fore-upper'), (2, [2], 'fore-leg'), (3, [3, 4], 'fore-foot')]}
GH = {200: [(0, [0], 'hind-thigh'), (1, [1], 'hind-shank'), (2, [2, 3], 'hind-lower')], 500: [(0, [0], 'hind-thigh'), (1, [1], 'hind-leg'), (2, [2, 3], 'hind-foot')]}


def tail_front(k, T):
    for n, a in (('tail-1', .034), ('tail-2', .032), ('tail-3', .029)):
        k.ell(n, n, 0, k.o(n)[1], a, .036, 'coat-tail', 2.8)
    if T < 500:
        k.col('tail-tip', 'tail-3', 0, 0, .02, .012, 'coat-tail', 2.8)


def front(k, T):
    for sd, sg in (('n', -1), ('f', 1)):
        leg_front(k, T, sd, sg, FORE, GF, 1.5, thin=1.0)
        leg_front(k, T, sd, sg, HIND, GH, 1.0)
    k.ell('hindbody', 'lumbar', 0, -.03, .125, .112, 'coat-trunk', 3)
    k.ell('chest', 'thorax', 0, -.072, .14, .165, 'coat-trunk', 3.2)
    tail_front(k, T)
    oy, ty = k.o('neck')[1], k.t('neck')[1]
    k.col('neck', 'neck', 0, 0, .09, .075, 'coat-trunk', 3.3, y0=oy + .16, y1=ty - .19)
    hy = k.o('head')[1]
    if T <= 100:
        k.ell('jaw', 'jaw', 0, .075, .04, .022, ['coat-head', 'tongue'], 3.9)
        k.ell('head', 'head', 0, .1425, .062, .057, 'coat-head', 4)
        k.ell('nose', 'head', 0, .134, .03, .026, 'nose-skin', 4.2)
        for sd, sg in (('n', -1), ('f', 1)):
            k.col(f'{NM[sd]}-ear', f'ear-{sd}', sg * .072, sg * .085, .022, .028, 'ears', 4.5 if sd == 'n' else 4.4, y0=.193, y1=.058)
        if T == 50:
            for sg in (-1, 1):
                k.ell(f'eye-{"n" if sg < 0 else "f"}', 'head', sg * .035, .16, .008, .006, None, 5, role='detail')
    elif T == 200:
        k.ell('jaw', 'jaw', 0, .075, .04, .022, ['coat-head', 'tongue'], 3.9)
        k.ell('head', 'head', 0, .1425, .062, .057, ['coat-head', 'nose-skin', 'tongue'], 4)
        for sd, sg in (('n', -1), ('f', 1)):
            k.col(f'{NM[sd]}-ear', f'ear-{sd}', sg * .072, sg * .085, .022, .028, 'ears', 4.5, y0=.193, y1=.058)
    else:
        k.ell('head', 'head', 0, .1425, .062, .057, ['coat-head', 'ears', 'nose-skin', 'tongue'], 4)


def plan(k, T):
    for sd, sg in (('n', -1), ('f', 1)):
        leg_plan(k, T, sd, sg, FORE, GF, 1.5, toe=.02)
        leg_plan(k, T, sd, sg, HIND, GH, 1.0, toe=.02)
    k.cap('hindbody', 'lumbar', 0, 0, .1, .13, 'coat-trunk', 3, x0=-.025, x1=.27)
    k.cap('chest', 'thorax', 0, 0, .14, .12, 'coat-trunk', 3.2, x0=.4, x1=.5)
    k.cap('neck', 'neck', 0, 0, .075, .065, 'coat-trunk', 3.3)
    hx, tx = k.o('head')[0], k.t('head')[0]
    if T <= 100:
        for n, (a, b) in zip(['tail-1', 'tail-2', 'tail-3'], [(.032, .03), (.03, .027), (.027, .02)]):
            k.cap(n, n, 0, 0, a, b, 'coat-tail', 2.8)
        k.cap('jaw', 'jaw', 0, 0, .04, .03, ['coat-head', 'tongue'], 3.9, x0=k.o('jaw')[0], x1=k.t('jaw')[0] - .02)
        k.cap('head', 'head', 0, 0, .06, .036, 'coat-head', 4, dx1=0)
        k.cap('nose', 'head', 0, 0, .03, .028, 'nose-skin', 4.2, x0=tx - .02, x1=tx + .005)
        for sd, sg in (('n', -1), ('f', 1)):
            k.cap(f'{NM[sd]}-ear', f'ear-{sd}', sg * .068, sg * .075, .03, .028, 'ears', 4.5, dx0=-.02, dx1=.03)
        if T == 50:
            for sg in (-1, 1):
                k.cap(f'eye-{"n" if sg < 0 else "f"}', 'head', sg * .036, sg * .036, .008, .008, None, 5, x0=hx + .13, x1=hx + .14, role='detail')
    else:
        k.cap('tail', 'tail-2', 0, 0, .03, .022, 'coat-tail', 2.8, x0=k.o('tail-1')[0], x1=k.t('tail-3')[0] + (.04 if T == 500 else 0))
        if T == 200:
            k.cap('jaw', 'jaw', 0, 0, .04, .03, ['coat-head', 'tongue'], 3.9, x0=k.o('jaw')[0], x1=k.t('jaw')[0] - .02)
            k.cap('head', 'head', 0, 0, .06, .036, ['coat-head', 'nose-skin', 'tongue'], 4)
            for sd, sg in (('n', -1), ('f', 1)):
                k.cap(f'{NM[sd]}-ear', f'ear-{sd}', sg * .068, sg * .075, .03, .028, 'ears', 4.5, dx0=-.02, dx1=.03)
        else:
            k.cap('head', 'head', 0, 0, .06, .036, ['coat-head', 'ears', 'nose-skin', 'tongue'], 4)


TEXT = {
    50: ("trunk and chest are an ellipse and a capsule at the true 0.28 m width, the neck is a followTip column that dips with the sniffing head, every leg segment is its own column (front) or capsule (plan) so trot and walk stretch correctly, the hind legs stand slightly wider than the forelegs, the tail has three pieces that stay horizontal until the dog wags or raises it, and the head carries a jaw with the tongue, a nose, hanging ears and eyes; from above the trunk hides the legs.",
         "Rumpf und Brust sind eine Ellipse und eine Kapsel in der wahren Breite von 0,28 m, der Hals ist eine followTip-Säule, die mit dem schnüffelnden Kopf nach unten geht, jedes Beinglied ist eine eigene Säule (Vorderansicht) oder Kapsel (Grundriss), sodass Trab und Schritt sich richtig strecken, die Hinterbeine stehen etwas weiter auseinander als die Vorderbeine, der Schwanz hat drei Stücke, die waagerecht bleiben, bis der Hund wedelt oder ihn hebt, und der Kopf trägt einen Unterkiefer mit Zunge, eine Nase, Hängeohren und Augen; von oben verdeckt der Rumpf die Beine."),
    100: ("the same bone-bound masses without detail lines: trunk and chest, a neck column, the head with jaw, nose and two ears, three tail pieces and every leg segment as its own followTip column or capsule.",
          "dieselben knochengebundenen Massen ohne Detaillinien: Rumpf und Brust, eine Halssäule, der Kopf mit Unterkiefer, Nase und zwei Ohren, drei Schwanzstücke und jedes Beinglied als eigene followTip-Säule oder -Kapsel."),
    200: ("main masses only: hindquarters, chest, one neck column, one head mass with jaw and two ears, one tail piece, and each leg in three bone-bound pieces (upper arm or thigh, forearm or shank, and a lower piece with wrist and paw).",
          "nur die Hauptmassen: Hinterhand, Brust, eine Halssäule, eine Kopfmasse mit Unterkiefer und zwei Ohren, ein Schwanzstück und jedes Bein in drei knochengebundenen Stücken (Oberarm oder Oberschenkel, Unterarm oder Unterschenkel und ein unteres Stück mit Handwurzel und Pfote)."),
    500: ("a composed silhouette: hindquarters and chest, neck, head with jaw and two ear marks, one tail piece, and each leg as an upper piece, a leg piece and a foot, bound to the same bones as the side view.",
          "eine zusammengesetzte Silhouette: Hinterhand und Brust, Hals, Kopf mit Unterkiefer und zwei Ohrenmarken, ein Schwanzstück und jedes Bein als oberes Stück, Beinstück und Fuß, an dieselben Knochen gebunden wie in der Seitenansicht."),
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
