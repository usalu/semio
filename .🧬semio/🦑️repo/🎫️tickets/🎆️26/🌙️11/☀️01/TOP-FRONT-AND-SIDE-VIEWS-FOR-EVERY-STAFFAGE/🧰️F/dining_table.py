from lib import *


def build(s, sc):
    F, P = 'front', 'plan'
    if sc == '500':
        s.rect(F, 'slab-left', -0.45, 0.64, 0.9, 0.11, like='slab-left')
        s.rect(F, 'leaf-bar', -0.44, 0.69, 0.88, 0.03, like='leaf-bar', z=5)
        for i, g in enumerate((1, -1)):
            s.poly(F, f'leg-{i}', [(g * 0.375, 0.7), (g * 0.375, 0.0)], like='leg-l', closed=False)
        s.rect(P, 'slab-left', 0.0, -0.45, 0.8, 0.9, like='slab-left')
        s.rect(P, 'slab-right', 0.8, -0.45, 0.8, 0.9, like='slab-right')
        s.rect(P, 'leaf-bar', 0.5, -0.45, 0.6, 0.9, like='leaf-bar', z=5)
        return ('Front view: the end of the table as one slab with the leaf bar and the two legs as lines. Plan: the two table halves and the leaf between them as three plates; the leaf stays hidden under the halves until they are pulled apart.',
                'Vorderansicht: die Stirnseite des Tisches als eine Platte mit der Auszugsplatte und den zwei Beinen als Linien. Grundriss: die beiden Tischhälften und die Einlegeplatte dazwischen als drei Flächen; die Einlegeplatte bleibt unter den Hälften verborgen, bis diese auseinandergezogen werden.')
    if sc == '200':
        s.rect(F, 'slab-left', -0.45, 0.64, 0.9, 0.11, like='slab-left')
        s.rect(F, 'leaf-bar', -0.44, 0.69, 0.88, 0.03, like='leaf-bar', z=5)
        for i, g in enumerate((1, -1)):
            s.poly(F, f'leg-{i}', [(g * 0.4, 0.0), (g * 0.35, 0.0), (g * 0.34, 0.64), (g * 0.41, 0.64)], like='leg-l')
        s.rect(P, 'slab-left', 0.0, -0.45, 0.8, 0.9, like='slab-left')
        s.rect(P, 'slab-right', 0.8, -0.45, 0.8, 0.9, like='slab-right')
        s.rect(P, 'leaf-bar', 0.5, -0.45, 0.6, 0.9, like='leaf-bar', z=5)
        return ('Front view: the slab with the apron below it, the two tapered legs and the leaf bar that rises into the top. Plan: the two halves and the leaf plate with its edges; the leaf shows only in the gap.',
                'Vorderansicht: die Platte mit der Zarge darunter, die zwei verjüngten Beine und die Auszugsplatte, die in die Tischplatte hochfährt. Grundriss: die beiden Hälften und die Einlegeplatte mit ihren Kanten; die Einlegeplatte zeigt sich nur in der Lücke.')
    if sc == '100':
        s.rect(F, 'top-l', -0.45, 0.72, 0.9, 0.03, like='top-l')
        s.rect(F, 'apron-l', -0.38, 0.64, 0.76, 0.08, like='apron-l')
        s.rect(F, 'leaf', -0.44, 0.69, 0.88, 0.03, like='leaf', z=5.5)
        for i, g in enumerate((1, -1)):
            s.poly(F, f'leg-{i}', [(g * 0.4, 0.0), (g * 0.35, 0.0), (g * 0.34, 0.64), (g * 0.41, 0.64)], like='leg-l')
        s.rect(P, 'top-l', 0.0, -0.45, 0.8, 0.9, like='top-l')
        s.rect(P, 'top-r', 0.8, -0.45, 0.8, 0.9, like='top-r')
        s.rect(P, 'leaf', 0.5, -0.45, 0.6, 0.9, like='leaf', z=5)
        for i, y in enumerate((-0.15, 0.15)):
            s.poly(P, f'plank-l-{i}', [(0.02, y), (0.78, y)], like=None, closed=False, bone='top-left', role='texture', z=6.5, filled=False, tone=0.4, region='table-top', material='oak-timber')
            s.poly(P, f'plank-r-{i}', [(0.82, y), (1.58, y)], like=None, closed=False, bone='top-right', role='texture', z=6.5, filled=False, tone=0.4, region='table-top', material='oak-timber')
        return ('Front view: the end of the table with top, apron, the leaf under the top and the two tapered legs. Plan: the two table halves with their plank joints and the leaf plate in the gap, drawn below the halves.',
                'Vorderansicht: die Stirnseite mit Platte, Zarge, der Auszugsplatte unter der Tischplatte und den zwei verjüngten Beinen. Grundriss: die beiden Tischhälften mit ihren Brettfugen und die Einlegeplatte in der Lücke, unterhalb der Hälften gezeichnet.')
    s.poly(F, 'top-l', [(-0.45, 0.728), (-0.442, 0.72), (0.442, 0.72), (0.45, 0.728), (0.45, 0.75), (-0.45, 0.75)], like='top-l')
    s.rect(F, 'apron-l', -0.38, 0.64, 0.76, 0.08, like='apron-l')
    s.poly(F, 'leaf', [(-0.44, 0.69), (0.44, 0.69), (0.44, 0.716), (0.434, 0.72), (-0.434, 0.72), (-0.44, 0.716)], like='leaf', z=5.5)
    for i, g in enumerate((1, -1)):
        s.poly(F, f'leg-{i}', [(g * 0.4, 0.0), (g * 0.35, 0.0), (g * 0.34, 0.56), (g * 0.34, 0.64), (g * 0.41, 0.64), (g * 0.41, 0.56)], like='leg-l')
        s.poly(F, f'shoulder-{i}', [(g * 0.34, 0.56), (g * 0.41, 0.56)], like='shoulder-l', closed=False)
    s.poly(P, 'top-l', [(0.0, -0.442), (0.008, -0.45), (0.792, -0.45), (0.8, -0.442), (0.8, 0.442), (0.792, 0.45), (0.008, 0.45), (0.0, 0.442)], like='top-l')
    s.poly(P, 'top-r', [(0.8, -0.442), (0.808, -0.45), (1.592, -0.45), (1.6, -0.442), (1.6, 0.442), (1.592, 0.45), (0.808, 0.45), (0.8, 0.442)], like='top-r')
    s.poly(P, 'leaf', [(0.5, -0.45), (1.1, -0.45), (1.1, 0.45), (0.5, 0.45)], like='leaf', z=5)
    for i, y in enumerate((-0.15, 0.15)):
        s.poly(P, f'plank-l-{i}', [(0.02, y), (0.78, y)], like='shoulder-l', closed=False, bone='top-left', z=6.5, role='texture', material='oak-timber', region='table-top')
        s.poly(P, f'plank-r-{i}', [(0.82, y), (1.58, y)], like='shoulder-r', closed=False, bone='top-right', z=6.5, role='texture', material='oak-timber', region='table-top')
        s.poly(P, f'plank-leaf-{i}', [(0.52, y), (1.08, y)], like='shoulder-l', closed=False, bone='leaf', z=5.5, role='texture', material='oak-timber', region='leaf-top')
    return ('Front view: the end of the table with the chamfered top, the end apron, the leaf under the top and the two tapered legs with their shoulder lines. Plan: the two halves with chamfered edges and plank joints, and the leaf plate with its own joints in the gap.',
            'Vorderansicht: die Stirnseite mit der gefasten Platte, der Stirnzarge, der Auszugsplatte unter der Tischplatte und den zwei verjüngten Beinen mit ihren Schulterlinien. Grundriss: die beiden Hälften mit gefasten Kanten und Brettfugen und die Einlegeplatte mit eigenen Fugen in der Lücke.')


if __name__ == '__main__':
    run('dining-table', build)
