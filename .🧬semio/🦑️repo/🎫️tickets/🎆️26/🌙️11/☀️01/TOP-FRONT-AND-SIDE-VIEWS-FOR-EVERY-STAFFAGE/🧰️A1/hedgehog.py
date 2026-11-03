from kit import *
ID = 'hedgehog'
BELLY = ['belly-fur', 'limbs-fur', 'paws-skin']


def front(k, T):
    dome = 'spines-dome' if T == 50 else ['spines-dome', 'belly-fur', 'limbs-fur', 'paws-skin']
    k.ell('dome', 'trunk', 0, .015, .075, .048, dome, 3.5)
    k.ell('skirt-rear', 'coat-r', 0, -.03, .072, .05, 'spines-skirt', 1)
    k.ell('skirt-front', 'coat-f', 0, -.03, .072, .05, 'spines-skirt', 3.45)
    if T == 50:
        k.ell('belly', 'trunk', 0, -.042, .06, .02, BELLY, 3)
        for sd, sg in (('n', -1), ('f', 1)):
            nme = 'near' if sd == 'n' else 'far'
            k.col(f'{nme}-fore', f'fore-{sd}', sg * .035, sg * .035, .014, .014, 'limbs-fur', 3.2)
            k.col(f'{nme}-forepaw', f'forepaw-{sd}', sg * .035, sg * .035, .014, .014, 'paws-skin', 3.2)
            k.col(f'{nme}-hind', f'hind-{sd}', sg * .042, sg * .042, .014, .014, 'limbs-fur', 1.1)
            k.col(f'{nme}-hindpaw', f'hindpaw-{sd}', sg * .042, sg * .042, .014, .014, 'paws-skin', 1.1)
    else:
        for sd, sg in (('n', -1), ('f', 1)):
            nme = 'near' if sd == 'n' else 'far'
            k.col(f'{nme}-fore', f'fore-{sd}', sg * .035, sg * .035, .014, .014, ['limbs-fur', 'paws-skin'], 3.2, y1=k.t(f'forepaw-{sd}')[1])
            k.col(f'{nme}-hind', f'hind-{sd}', sg * .042, sg * .042, .014, .014, ['limbs-fur', 'paws-skin'], 1.1, y1=k.t(f'hindpaw-{sd}')[1])
    k.col('neck', 'neck', 0, 0, .036, .034, 'face-fur', 1.9, y0=-.03, y1=.015)
    k.ell('head', 'head', 0, -.01, .035, .031, ['face-fur', 'nose'], 4)


def plan(k, T):
    dome = 'spines-dome' if T == 50 else ['spines-dome', 'belly-fur', 'limbs-fur', 'paws-skin']
    k.cap('dome', 'trunk', 0, 0, .072, .075, dome, 3.5, x0=.035, x1=.083)
    k.ell('skirt-rear', 'coat-r', -.03, 0, .03, .062, 'spines-skirt', 3.6)
    k.ell('skirt-front', 'coat-f', .14, 0, .03, .062, 'spines-skirt', 3.6)
    for sd, sg in (('n', -1), ('f', 1)):
        nme = 'near' if sd == 'n' else 'far'
        if T == 50:
            k.cap(f'{nme}-fore', f'fore-{sd}', sg * .035, sg * .035, .014, .014, 'limbs-fur', 1.5)
            k.cap(f'{nme}-forepaw', f'forepaw-{sd}', sg * .035, sg * .035, .014, .016, 'paws-skin', 1.5, dx1=.012)
            k.cap(f'{nme}-hind', f'hind-{sd}', sg * .042, sg * .042, .014, .014, 'limbs-fur', 1.1)
            k.cap(f'{nme}-hindpaw', f'hindpaw-{sd}', sg * .042, sg * .042, .014, .016, 'paws-skin', 1.1, dx1=.012)
        else:
            k.cap(f'{nme}-fore', f'fore-{sd}', sg * .035, sg * .035, .014, .016, ['limbs-fur', 'paws-skin'], 1.5, x1=k.t(f'forepaw-{sd}')[0] + .012)
            k.cap(f'{nme}-hind', f'hind-{sd}', sg * .042, sg * .042, .014, .016, ['limbs-fur', 'paws-skin'], 1.1, x1=k.t(f'hindpaw-{sd}')[0] + .012)
    k.cap('neck', 'neck', 0, 0, .03, .028, 'face-fur', 1.9)
    hx = k.o('head')[0]
    k.cap('head', 'head', 0, 0, .03, .018, ['face-fur', 'nose'], 4, x0=hx + .006, x1=hx + .04)


TEXT = {
    50: ("the spiny dome is an ellipse and a capsule at the true 0.15 m width, the front and rear spine skirts are separate shapes bound to their coat bones so they lift when the animal rolls up, the face is a rounded head on a short neck, and each of the four legs is a limb column (front) or capsule (plan) plus a paw; from above the dome and its skirts cover the legs.",
         "Die Stachelkuppel ist eine Ellipse und eine Kapsel in der wahren Breite von 0,15 m, die vorderen und hinteren Stachelschürzen sind getrennte Formen an ihren Fellknochen und heben sich beim Einrollen, das Gesicht ist ein runder Kopf auf kurzem Hals, und jedes der vier Beine ist eine Beinsäule (Vorderansicht) oder Kapsel (Grundriss) mit Pfote; von oben bedecken Kuppel und Schürzen die Beine."),
    100: ("one dome mass carrying the belly, limb and paw regions, two spine skirts, the head with its neck, and the four legs as one stroke-like piece each.",
          "eine Kuppelmasse, die Bauch-, Bein- und Pfotenregion trägt, zwei Stachelschürzen, der Kopf mit seinem Hals und die vier Beine als je ein strichartiges Stück."),
}


def build(doc):
    a = doc['asset']
    res, txt = {}, {}
    for sc in (50, 100):
        if a['representations'][str(sc)]['status'] != 'authored':
            continue
        kf = K(a, sc); front(kf, sc)
        kp = K(a, sc); plan(kp, sc)
        res[sc] = {'front': kf.shapes, 'plan': kp.shapes}
        txt[sc] = TEXT[sc]
    return finish(doc, res, txt)
