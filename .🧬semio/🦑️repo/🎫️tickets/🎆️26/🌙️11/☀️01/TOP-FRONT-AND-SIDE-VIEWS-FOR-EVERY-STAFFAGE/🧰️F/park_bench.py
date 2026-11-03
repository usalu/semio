from lib import *

K = 0.2093


def xc(y):
    return 0.19 + K * (y - 0.4)


def post(y0, y1, w):
    return [(xc(y0) - w, y0), (xc(y0) + w, y0), (xc(y1) + w, y1), (xc(y1) - w, y1)]


def band(y0, y1, a=0.045, b=0.012):
    return [(xc(y0) - a, y0), (xc(y0) - b, y0), (xc(y1) - b, y1), (xc(y1) - a, y1)]


SLATS = [(0.52, 0.6), (0.63, 0.71), (0.74, 0.82)]


def build(s, sc):
    F, P = 'front', 'plan'
    if sc == '500':
        s.rect(F, 'l-foot', -0.3, 0.0, 0.55, 0.05, like='l-frame')
        s.rect(F, 'l-front-leg', -0.29, 0.0, 0.06, 0.634, like='l-frame')
        s.poly(F, 'l-rear-post', post(0.0, 0.85, 0.03), like='l-frame')
        s.rect(F, 'seat-back-block', -0.3, 0.405, 0.49, 0.045, like='seat-back-block')
        s.poly(F, 'back-block', band(0.52, 0.82), like='seat-back-block', bone='backrest')
        s.rect(P, 'seat-back-block', 0.0, -0.3, 1.8, 0.6, like='seat-back-block')
        return ('Front view: the end frame as foot, front leg and leaning rear post, the seat board and the leaning backrest band. Plan: seat, backrest and armrests as one plate.',
                'Vorderansicht: der Endrahmen aus Fuß, Vorderbein und geneigter Rückenstütze, das Sitzbrett und das geneigte Lehnenband. Grundriss: Sitz, Rückenlehne und Armlehnen als eine Fläche.')
    if sc == '200':
        s.rect(F, 'l-foot', -0.3, 0.0, 0.55, 0.05, like='l-frame', r=0.01)
        s.rect(F, 'l-front-leg', -0.285, 0.05, 0.05, 0.36, like='l-frame')
        s.rect(F, 'l-rear-leg', 0.165, 0.05, 0.05, 0.36, like='l-frame')
        s.rect(F, 'l-arm-post', -0.276, 0.4, 0.032, 0.2, like='l-frame')
        s.rect(F, 'l-armrest', -0.3, 0.6, 0.44, 0.034, like='l-frame', r=0.012)
        s.poly(F, 'l-rear-post', post(0.4, 0.83, 0.016), like='l-frame')
        s.rect(F, 'seat', -0.3, 0.405, 0.49, 0.045, like='seat')
        s.poly(F, 'back-band', band(0.52, 0.82), like='back-band')
        s.poly(F, 'back-joint-1', [(xc(0.615) - 0.045, 0.615), (xc(0.615) - 0.012, 0.615)], like='back-joint-1', closed=False)
        s.poly(F, 'back-joint-2', [(xc(0.725) - 0.045, 0.725), (xc(0.725) - 0.012, 0.725)], like='back-joint-2', closed=False)
        s.rect(P, 'seat', 0.0, -0.3, 1.8, 0.49, like='seat')
        s.rect(P, 'back-band', 0.0, 0.18, 1.8, 0.075, like='back-band')
        for n, x in (('l', 0.075), ('r', 1.635)):
            s.rect(P, f'{n}-armrest', x, -0.3, 0.09, 0.44, like=f'{n}-frame', bone='end-left' if n == 'l' else 'end-right', z=5)
        for n, x in (('l', 0.12), ('r', 1.68)):
            s.ell(P, f'{n}-post', x, xc(0.8285), 0.0155, 0.0155, like=f'{n}-frame', bone='end-left' if n == 'l' else 'end-right', z=5.5)
        return ('Front view: the end frame as foot, two legs, arm post, armrest and the leaning rear post, the seat board and the leaning backrest band with its two joints. Plan: the seat board, the backrest band behind it, the two armrests and the tops of the posts.',
                'Vorderansicht: der Endrahmen aus Fuß, zwei Beinen, Armlehnenstütze, Armlehne und geneigter Rückenstütze, das Sitzbrett und das geneigte Lehnenband mit seinen zwei Fugen. Grundriss: das Sitzbrett, das Lehnenband dahinter, die zwei Armlehnen und die Pfostenköpfe.')
    s.rect(F, 'l-foot', -0.3, 0.0, 0.55, 0.05, like='l-foot' if 'l-foot' in s.elev else 'l-foot-leg', r=0.01)
    s.poly(F, 'l-front-leg', [(-0.288, 0.04), (-0.232, 0.04), (-0.236, 0.37), (-0.284, 0.37)], like='l-leg' if 'l-leg' in s.elev else 'l-foot-leg')
    s.poly(F, 'l-rear-leg', [(0.162, 0.04), (0.218, 0.04), (0.214, 0.37), (0.166, 0.37)], like='l-leg' if 'l-leg' in s.elev else 'l-foot-leg')
    s.rect(F, 'l-bearer', -0.29, 0.37, 0.51, 0.036, like='l-bearer')
    s.rect(F, 'l-brace', -0.26, 0.19, 0.45, 0.022, like='l-brace', **({'r': 0.004} if sc == '50' else {}))
    if sc == '100':
        s.poly(F, 'l-arm-post', [(-0.276, 0.4), (-0.276, 0.6)], like='l-arm-post', closed=False)
        s.poly(F, 'l-arm-post-b', [(-0.244, 0.4), (-0.244, 0.6)], like='l-arm-post-b', closed=False)
    else:
        s.rect(F, 'l-arm-post', -0.276, 0.4, 0.032, 0.2, like='l-arm-post')
    s.rect(F, 'l-armrest', -0.3, 0.6, 0.44, 0.034, like='l-armrest', r=0.013)
    s.poly(F, 'l-post', post(0.4, 0.8285, 0.0155), like='l-post')
    s.ell(F, 'l-finial', xc(0.8325), 0.8325, 0.0185, 0.0175, like='l-finial')
    s.rect(F, 'seat-front', -0.3, 0.405, 0.49, 0.045, like='seat-front', r=0.008)
    if sc == '100':
        s.poly(F, 'back-band', band(0.52, 0.82), like='back-band')
        s.poly(F, 'back-joint-1', [(xc(0.615) - 0.045, 0.615), (xc(0.615) - 0.012, 0.615)], like='back-joint-1', closed=False)
        s.poly(F, 'back-joint-2', [(xc(0.725) - 0.045, 0.725), (xc(0.725) - 0.012, 0.725)], like='back-joint-2', closed=False)
        s.rect(P, 'seat-front', 0.0, -0.3, 1.8, 0.49, like='seat-front', r=0.008)
        s.rect(P, 'back-band', 0.0, 0.18, 1.8, 0.075, like='back-band', r=0.012)
        for n, x in (('l', 0.075), ('r', 1.635)):
            s.rect(P, f'{n}-armrest', x, -0.3, 0.09, 0.44, like=f'{n}-armrest', r=0.012, z=5)
        for n, x in (('l', 0.12), ('r', 1.68)):
            s.ell(P, f'{n}-finial', x, xc(0.8325), 0.0185, 0.0185, like=f'{n}-finial')
            s.ell(P, f'{n}-post', x, xc(0.8285), 0.0135, 0.0135, like=f'{n}-post', z=3.05)
        return ('Front view: the end frame with foot, tapered legs, bearer, brace, the two arm posts, armrest and the leaning rear post with its finial, the seat board and the leaning backrest band with its joints. Plan: the seat board, the backrest band with its edge, the two armrests and the post finials.',
                'Vorderansicht: der Endrahmen mit Fuß, verjüngten Beinen, Auflager, Strebe, den zwei Armlehnenstützen, Armlehne und der geneigten Rückenstütze mit Knauf, das Sitzbrett und das geneigte Lehnenband mit seinen Fugen. Grundriss: das Sitzbrett, das Lehnenband mit seiner Kante, die zwei Armlehnen und die Pfostenknäufe.')
    s.poly(F, 'seat-chamfer', [(-0.288, 0.441), (0.178, 0.441)], like='seat-chamfer', closed=False)
    s.poly(F, 'l-web-rim', [(-0.27, 0.15), (-0.271, 0.36)], like='l-web-rim', closed=False)
    s.poly(F, 'l-web-rim-b', [(-0.249, 0.15), (-0.25, 0.36)], like='l-web-rim-b', closed=False)
    s.poly(F, 'l-foot-bead', [(-0.29, 0.016), (0.24, 0.016)], like='l-foot-bead', closed=False)
    s.path(F, 'l-arm-scroll', 'M -0.29 0.612 C -0.29 0.585 -0.26 0.58 -0.26 0.55', like='l-arm-scroll')
    for i, y in enumerate(SLATS):
        s.poly(F, f'back-slat-{i + 1}', band(y[0], y[1]), like=f'back-slat-{i + 1}')
        s.poly(F, f'back-slat-{i + 1}-chamfer', [(xc(y[1] - 0.008) - 0.045, y[1] - 0.008), (xc(y[1] - 0.008) - 0.012, y[1] - 0.008)], like=f'back-slat-{i + 1}-chamfer', closed=False)
        s.ell(F, f'back-slat-{i + 1}-bolt', xc((y[0] + y[1]) / 2) - 0.0, (y[0] + y[1]) / 2, 0.0038, 0.0038, like=f'back-slat-{i + 1}-bolt-0')
    for i, (bx, by) in enumerate(((-0.28, 0.008), (0.23, 0.008), (-0.02, 0.388))):
        s.ell(F, f'l-bolt-{i}', bx, by, 0.0045, 0.0045, like=f'l-bolt-{i}')
    s.rect(P, 'seat-front', 0.0, -0.3, 1.8, 0.49, like='seat-front', r=0.008)
    s.poly(P, 'seat-chamfer', [(0.012, -0.288), (1.788, -0.288)], like='seat-chamfer', closed=False)
    for i, y in enumerate(SLATS):
        s.rect(P, f'back-slat-{i + 1}', 0.0, xc(y[0]) - 0.045, 1.8, 0.033, like=f'back-slat-{i + 1}', r=0.01)
        s.poly(P, f'back-slat-{i + 1}-chamfer', [(0.012, xc(y[0]) - 0.0425), (1.788, xc(y[0]) - 0.0425)], like=f'back-slat-{i + 1}-chamfer', closed=False)
    for n, x in (('l', 0.075), ('r', 1.635)):
        s.rect(P, f'{n}-armrest', x, -0.3, 0.09, 0.44, like=f'{n}-armrest', r=0.013, z=5.2)
        s.path(P, f'{n}-arm-scroll', f'M {x + 0.045} -0.285 L {x + 0.045} 0.12', like=f'{n}-arm-scroll')
    for n, x in (('l', 0.12), ('r', 1.68)):
        s.ell(P, f'{n}-finial', x, xc(0.8325), 0.0185, 0.0185, like=f'{n}-finial')
        s.ell(P, f'{n}-post', x, xc(0.8285), 0.0135, 0.0135, like=f'{n}-post', z=3.05)
    return ('Front view: the end frame with its cast foot, bead, tapered legs with web rims, bearer, brace, arm posts, armrest with scroll and the leaning rear post with finial, the seat board with chamfer and the three leaning backrest slats with their bolts. Plan: the seat board with chamfer, the three slats that step back as they lean, the two armrests with their centre line and the post finials.',
            'Vorderansicht: der Endrahmen mit gegossenem Fuß, Wulst, verjüngten Beinen mit Stegrändern, Auflager, Strebe, Armlehnenstützen, Armlehne mit Volute und der geneigten Rückenstütze mit Knauf, das Sitzbrett mit Fase und die drei geneigten Lehnenlatten mit ihren Schrauben. Grundriss: das Sitzbrett mit Fase, die drei Latten, die beim Neigen nach hinten versetzt sind, die zwei Armlehnen mit ihrer Mittellinie und die Pfostenknäufe.')


if __name__ == '__main__':
    run('park-bench', build)
