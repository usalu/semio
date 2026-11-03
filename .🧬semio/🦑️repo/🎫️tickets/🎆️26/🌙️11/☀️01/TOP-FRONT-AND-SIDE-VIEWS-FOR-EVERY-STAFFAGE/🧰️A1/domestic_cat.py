from legs import *
ID = 'domestic-cat'

CF, CH = 'coat-forelimbs', 'coat-hindlimbs'
FORE = [seg('scapula-f{s}', .035, .035, .022, .022, CF, 'scapula'), seg('humerus-f{s}', .035, .034, .022, .018, CF, 'humerus'),
        seg('forearm-f{s}', .034, .032, .015, .012, CF, 'forearm'), seg('metacarpus-f{s}', .032, .032, .011, .011, CF, 'wrist'),
        seg('paw-f{s}', .032, .032, .016, .018, [CF, 'pads'], 'fore-paw')]
HIND = [seg('thigh-h{s}', .036, .036, .03, .025, CH, 'thigh'), seg('shank-h{s}', .036, .035, .02, .014, CH, 'shank'),
        seg('metatarsus-h{s}', .035, .035, .011, .011, CH, 'hock'), seg('paw-h{s}', .035, .035, .016, .018, [CH, 'pads'], 'hind-paw')]
GF = {200: [(1, [0, 1], 'fore-upper'), (2, [2], 'fore-lower'), (3, [3, 4], 'fore-foot')]}
GH = {200: [(0, [0], 'hind-thigh'), (1, [1], 'hind-lower'), (2, [2, 3], 'hind-foot')]}
TAIL = [('tail-1', .02, .019), ('tail-2', .019, .018), ('tail-3', .018, .017), ('tail-4', .017, .016), ('tail-5', .016, .012)]


def tail_beads(k):
    for n, a, b in TAIL:
        k.ell(n, n, 0, k.o(n)[1], a + .002, .032, 'coat-tail', 2.8)
    k.col('tail-tip', 'tail-5', 0, 0, .014, .01, 'coat-tail', 2.8)


def front(k, T):
    th = 1.15 if T == 200 else 1.0
    for sd, sg in (('n', -1), ('f', 1)):
        leg_front(k, T, sd, sg, FORE, GF, 1.5, ground=-.003, thin=th)
        leg_front(k, T, sd, sg, HIND, GH, 1.0, ground=-.003, thin=th)
    if T <= 100:
        k.ell('hindbody', 'lumbar', 0, -.004, .058, .053, 'coat-trunk', 3)
        k.ell('midbody', 'thorax-a', 0, -.012, .062, .062, 'coat-trunk', 3.1)
        tail_beads(k)
    else:
        k.ell('hindbody', 'lumbar', 0, -.004, .058, .053, 'coat-trunk', 3)
        tail_beads(k)
    k.ell('chest', 'thorax-b', 0, -.011, .065, .062, 'coat-trunk', 3.2)
    oy, ty = k.o('neck')[1], k.t('neck')[1]
    k.col('neck', 'neck', 0, 0, .042, .04, 'coat-trunk', 3.3, y0=oy + .078, y1=ty - .07)
    k.ell('jaw', 'jaw', 0, .02, .026, .014, ['coat-head', 'tongue'], 3.9)
    if T <= 100:
        k.ell('head', 'head', 0, .058, .046, .042, 'coat-head', 4)
        k.ell('nose', 'head', 0, .047, .009, .007, 'nose-skin', 4.2)
        for sd, sg in (('n', -1), ('f', 1)):
            k.col(f'{NM[sd]}-ear', f'ear-{sd}', sg * .028, sg * .032, .02, .006, 'ears', 4.5 if sd == 'n' else 4.4, y0=.074, y1=.128)
            k.ell(f'{NM[sd]}-eye', 'head', sg * .022, .064, .008, .006, 'eyes', 5, role='detail' if T == 50 else 'body')
    else:
        k.ell('head', 'head', 0, .058, .046, .042, ['coat-head', 'nose-skin', 'eyes'], 4)
        for sd, sg in (('n', -1), ('f', 1)):
            k.col(f'{NM[sd]}-ear', f'ear-{sd}', sg * .028, sg * .032, .02, .006, 'ears', 4.5, y0=.074, y1=.128)


def plan(k, T):
    th = 1.15 if T == 200 else 1.0
    for sd, sg in (('n', -1), ('f', 1)):
        leg_plan(k, T, sd, sg, FORE, GF, 1.5, toe=.012, thin=th)
        leg_plan(k, T, sd, sg, HIND, GH, 1.0, toe=.012, thin=th)
    k.cap('hindbody', 'lumbar', 0, 0, .055, .062, 'coat-trunk', 3, x0=.0, x1=.14)
    if T <= 100:
        k.cap('midbody', 'thorax-a', 0, 0, .062, .065, 'coat-trunk', 3.1, x0=.18, x1=.24)
        for n, a, b in TAIL:
            k.cap(n, n, 0, 0, a, b, 'coat-tail', 2.8, dx1=.012 if n == 'tail-5' else 0)
    else:
        k.cap('tail-a', 'tail-2', 0, 0, .02, .018, 'coat-tail', 2.8, x0=k.o('tail-1')[0])
        k.cap('tail-b', 'tail-4', 0, 0, .017, .012, 'coat-tail', 2.8, x0=k.o('tail-3')[0], x1=k.t('tail-5')[0] - .01)
    k.cap('chest', 'thorax-b', 0, 0, .062, .06, 'coat-trunk', 3.2, x0=.27, x1=.28)
    k.cap('neck', 'neck', 0, 0, .042, .04, 'coat-trunk', 3.3)
    hx = k.o('head')[0]
    k.cap('jaw', 'jaw', 0, 0, .026, .02, ['coat-head', 'tongue'], 3.9, x0=k.o('jaw')[0], x1=k.t('jaw')[0] - .01)
    if T <= 100:
        k.cap('head', 'head', 0, 0, .046, .03, 'coat-head', 4, x1=hx + .083)
        k.cap('nose', 'head', 0, 0, .012, .012, 'nose-skin', 4.2, x0=hx + .082, x1=hx + .085)
        for sd, sg in (('n', -1), ('f', 1)):
            k.cap(f'{NM[sd]}-ear', f'ear-{sd}', sg * .03, sg * .03, .014, .012, 'ears', 4.5, dx0=-.01, dx1=.01)
            k.cap(f'{NM[sd]}-eye', 'head', sg * .022, sg * .022, .007, .007, 'eyes', 5, x0=hx + .07, x1=hx + .074, role='detail' if T == 50 else 'body')
    else:
        k.cap('head', 'head', 0, 0, .046, .03, ['coat-head', 'nose-skin', 'eyes'], 4, x1=hx + .083)
        for sd, sg in (('n', -1), ('f', 1)):
            k.cap(f'{NM[sd]}-ear', f'ear-{sd}', sg * .03, sg * .03, .014, .012, 'ears', 4.5, dx0=-.01, dx1=.01)


TEXT = {
    50: ("the body is ellipses and capsules at the true 0.13 m width, the neck a followTip column that stretches when the cat lowers its head to stretch, every leg segment is its own column (front) or capsule (plan) so walking, stalking and the deep stretch read correctly, the five tail pieces are columns with margins so they follow a raised tail, and the head has a jaw with tongue, a nose, two ears and eyes; from above the body hides the legs.",
         "Der Körper besteht aus Ellipsen und Kapseln in der wahren Breite von 0,13 m, der Hals ist eine followTip-Säule, die sich beim Strecken mit gesenktem Kopf dehnt, jedes Beinglied ist eine eigene Säule (Vorderansicht) oder Kapsel (Grundriss), sodass Gehen, Anschleichen und die tiefe Streckung richtig lesbar sind, die fünf Schwanzstücke sind Säulen mit Rand und folgen einem erhobenen Schwanz, und der Kopf hat einen Unterkiefer mit Zunge, eine Nase, zwei Ohren und Augen; von oben verdeckt der Körper die Beine."),
    100: ("the same bone-bound masses without detail lines: hindquarters, mid body and chest, a neck column, the head with jaw, nose, two ears and eyes, five tail pieces and every leg segment as its own followTip column or capsule.",
          "dieselben knochengebundenen Massen ohne Detaillinien: Hinterhand, Mittelkörper und Brust, eine Halssäule, der Kopf mit Unterkiefer, Nase, zwei Ohren und Augen, fünf Schwanzstücke und jedes Beinglied als eigene followTip-Säule oder -Kapsel."),
    200: ("main masses only: hindquarters, chest, neck, one head mass with jaw and two ears, two tail pieces, and each leg in three bone-bound pieces (upper arm or thigh, lower leg, foot) drawn 1.15 times wider than life so they stay above the minimum line width.",
          "nur die Hauptmassen: Hinterhand, Brust, Hals, eine Kopfmasse mit Unterkiefer und zwei Ohren, zwei Schwanzstücke und jedes Bein in drei knochengebundenen Stücken (Oberarm oder Oberschenkel, Unterschenkel, Fuß), 1,15-mal breiter als in Wirklichkeit gezeichnet, damit sie über der Mindestlinienbreite bleiben."),
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
