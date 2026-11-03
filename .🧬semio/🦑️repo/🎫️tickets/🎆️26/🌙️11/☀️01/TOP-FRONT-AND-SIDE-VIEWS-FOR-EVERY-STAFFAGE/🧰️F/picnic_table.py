from lib import *


def build(s, sc):
    F, P = 'front', 'plan'
    for view in (F, P):
        s.solid(view, 'bench-r', 'bench-r', -0.85, 0.85, z=5)
        s.solid(view, 'bench-l', 'bench-l', -0.85, 0.85, z=4.9)
    if sc == '500':
        s.rect(F, 'slab', -0.9, 0.65, 1.8, 0.1, like='slab')
        for i, g in enumerate((1, -1)):
            s.poly(F, f'leg-{i}', [(g * 0.8, 0.0), (g * 0.8, 0.66)], like='leg-l', closed=False)
        s.rect(P, 'slab', -0.35, -0.9, 0.7, 1.8, like='slab')
        for i, g in enumerate((1, -1)):
            s.poly(P, f'leg-l-{i}', [(-0.22, g * 0.8), (-0.69, g * 0.8)], like='leg-l', closed=False)
            s.poly(P, f'leg-r-{i}', [(0.22, g * 0.8), (0.69, g * 0.8)], like='leg-r', closed=False)
        return ('Front view: the long side with the table slab, the two A-frame legs as lines and the benches swinging up with their bones. Plan: the table slab with the two benches either side, extruded from their side profile so they swing up in both views, and the splayed frame legs as lines.',
                'Vorderansicht: die Längsseite mit Tischplatte, den zwei A-Rahmen-Beinen als Linien und den Bänken, die mit ihren Knochen hochschwingen. Grundriss: die Tischplatte mit den beiden Bänken zu beiden Seiten, gezeichnet aus ihrem Seitenprofil gezogen, so schwingen sie in beiden Ansichten hoch, und die gespreizten Rahmenbeine als Linien.')
    if sc == '200':
        s.rect(F, 'slab', -0.9, 0.665, 1.8, 0.085, like='slab')
        for i, g in enumerate((1, -1)):
            s.rect(F, f'leg-{i}', g * 0.8 - 0.025, 0.0, 0.05, 0.71, like='leg-l')
            s.rect(F, f'beam-{i}', g * 0.8 - 0.025, 0.35, 0.05, 0.05, like='beam')
        for i, g in enumerate((1, -1)):
            s.rect(F, f'hinge-{i}', g * 0.72 - 0.05, 0.34, 0.1, 0.1, like='hinge-l')
        s.rect(P, 'slab', -0.35, -0.9, 0.7, 1.8, like='slab')
        for i, g in enumerate((1, -1)):
            s.poly(P, f'leg-l-{i}', [(-0.25, g * 0.775), (-0.19, g * 0.775), (-0.66, g * 0.775), (-0.72, g * 0.775)], like='leg-l')
            s.poly(P, f'leg-r-{i}', [(0.25, g * 0.775), (0.19, g * 0.775), (0.66, g * 0.775), (0.72, g * 0.775)], like='leg-r')
        return ('Front view: the long side with the slab, the two A-frames seen edge-on with their cross beams, the hinges and the benches swinging up with their bones. Plan: the slab, the benches extruded from their side profile so they swing up in both views and the frame legs.',
                'Vorderansicht: die Längsseite mit Tischplatte, den zwei A-Rahmen von der Kante mit ihren Querbalken, den Scharnieren und den Bänken, die mit ihren Knochen hochschwingen. Grundriss: die Tischplatte, die Bänke aus ihrem Seitenprofil gezogen, so schwingen sie in beiden Ansichten hoch, und die Rahmenbeine.')
    if sc == '100':
        s.rect(F, 'top', -0.9, 0.71, 1.8, 0.04, like='top')
        for i, g in enumerate((1, -1)):
            s.rect(F, f'top-beam-{i}', g * 0.8 - 0.0225, 0.665, 0.045, 0.045, like='top-beam')
            s.rect(F, f'leg-{i}', g * 0.8 - 0.0225, 0.0, 0.045, 0.665, like='leg-l')
            s.rect(F, f'beam-{i}', g * 0.8 - 0.0225, 0.355, 0.045, 0.045, like='beam', z=3.5)
            s.rect(F, f'hinge-{i}', g * 0.72 - 0.05, 0.34, 0.1, 0.1, like='hinge-l')
        s.rect(P, 'top', -0.35, -0.9, 0.7, 1.8, like='top')
        for i, g in enumerate((1, -1)):
            s.poly(P, f'leg-l-{i}', [(-0.25, g * 0.7775), (-0.19, g * 0.7775), (-0.19, g * 0.8225), (-0.25, g * 0.8225)], like='leg-l') if False else None
            s.rect(P, f'leg-l-{i}', -0.72, g * 0.8 - 0.0225, 0.53, 0.045, like='leg-l')
            s.rect(P, f'leg-r-{i}', 0.19, g * 0.8 - 0.0225, 0.53, 0.045, like='leg-r')
            s.rect(P, f'top-beam-{i}', -0.3, g * 0.8 - 0.0225, 0.6, 0.045, like='top-beam', z=2.9)
        return ('Front view: the slab, the two A-frames seen edge-on with top beam, leg, cross beam and hinge, and the two benches swinging up with their bones. Plan: the slab, the benches extruded from their side profile so they swing up in both views, and the frame legs and top beams below.',
                'Vorderansicht: die Tischplatte, die zwei A-Rahmen von der Kante mit Kopfbalken, Bein, Querbalken und Scharnier und die zwei Bänke, die mit ihren Knochen hochschwingen. Grundriss: die Tischplatte, die Bänke aus ihrem Seitenprofil gezogen, so schwingen sie in beiden Ansichten hoch, und darunter die Rahmenbeine und Kopfbalken.')
    s.poly(F, 'top', [(-0.9, 0.748), (-0.9, 0.718), (-0.892, 0.71), (0.892, 0.71), (0.9, 0.718), (0.9, 0.748)], like='top')
    for i, g in enumerate((1, -1)):
        s.rect(F, f'top-beam-{i}', g * 0.8 - 0.0225, 0.665, 0.045, 0.045, like='top-beam')
        s.rect(F, f'leg-{i}', g * 0.8 - 0.0225, 0.0, 0.045, 0.665, like='leg-l')
        s.rect(F, f'beam-{i}', g * 0.8 - 0.0225, 0.355, 0.045, 0.045, like='beam', z=3.5)
        s.rect(F, f'hinge-{i}', g * 0.72 - 0.05, 0.34, 0.1, 0.1, like='hinge-l', r=0.004)
        s.ell(F, f'screw-{i}', g * 0.72, 0.4, 0.009, 0.009, like='screw-l')
    s.rect(P, 'top', -0.35, -0.9, 0.7, 1.8, like='top', r=0.008)
    for i, x in enumerate((-0.21, -0.07, 0.07, 0.21)):
        s.poly(P, f'top-plank-{i}', [(x, -0.89), (x, 0.89)], like='top', closed=False, role='texture', filled=False, tone=0.4, z=6.5)
    for i, g in enumerate((1, -1)):
        s.rect(P, f'leg-l-{i}', -0.72, g * 0.8 - 0.0225, 0.53, 0.045, like='leg-l')
        s.rect(P, f'leg-r-{i}', 0.19, g * 0.8 - 0.0225, 0.53, 0.045, like='leg-r')
        s.rect(P, f'top-beam-{i}', -0.3, g * 0.8 - 0.0225, 0.6, 0.045, like='top-beam', z=2.9)
    return ('Front view: the slab with its chamfer, the two A-frames seen edge-on with top beam, leg, cross beam, hinge plate and screw, and the two rounded benches swinging up with their bones. Plan: the slab with plank joints, the benches extruded from their side profile so they swing up in both views, and below them the frame legs and top beams, which show as soon as the benches are folded away.',
            'Vorderansicht: die gefaste Tischplatte, die zwei A-Rahmen von der Kante mit Kopfbalken, Bein, Querbalken, Scharnierplatte und Schraube und die zwei abgerundeten Bänke, die mit ihren Knochen hochschwingen. Grundriss: die Tischplatte mit Brettfugen, die Bänke aus ihrem Seitenprofil gezogen, so schwingen sie in beiden Ansichten hoch, und darunter die Rahmenbeine und Kopfbalken, die sichtbar werden, sobald die Bänke weggeklappt sind.')


if __name__ == '__main__':
    run('picnic-table', build)
