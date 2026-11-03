from lib import *

YS = [0.36 + 0.3 * i for i in range(5)]


def build(s, sc):
    F, P = 'front', 'plan'
    if sc == '500':
        s.rect(F, 'body', -0.15, 0.06, 0.3, 1.74, like='body')
        s.rect(F, 'plinth', -0.12, 0.0, 0.25, 0.08, like='plinth')
        for n, y in enumerate(YS, 1):
            s.poly(F, f'shelf-{n}', [(-0.145, y + 0.009), (0.132, y + 0.009)], like=f'shelf-{n}-l', closed=False, bone=f'shelf-{n}', z=5)
        s.rect(P, 'body', 0.0, -0.15, 0.8, 0.3, like='body')
        return ('Front view: an end section through the carcass, one filled body with the plinth set back and the five shelves as lines that move with their bone. Plan: the top panel as one plate.',
                'Vorderansicht: ein Schnitt durch den Korpus von der Schmalseite, ein gefüllter Körper mit zurückgesetztem Sockel und die fünf Böden als Linien, die mit ihrem Knochen wandern. Grundriss: die Deckplatte als eine Fläche.')
    if sc == '200':
        s.rect(F, 'body', -0.15, 0.06, 0.3, 1.74, like='body')
        s.rect(F, 'plinth', -0.12, 0.0, 0.25, 0.08, like='plinth')
        for n, y in enumerate(YS, 1):
            s.poly(F, f'shelf-{n}', [(-0.145, y + 0.009), (0.132, y + 0.009)], like=f'shelf-{n}-l', closed=False, bone=f'shelf-{n}', z=5)
        s.rect(P, 'body', 0.0, -0.15, 0.8, 0.3, like='body')
        s.poly(P, 'front-edge', [(0.018, -0.132), (0.782, -0.132)], like='shelf-1-l', closed=False, bone='frame', z=6, role='detail')
        return ('Front view: an end section through the carcass with the set-back plinth and the five shelf boards as lines at their pin positions. Plan: the top panel with its front edge line.',
                'Vorderansicht: ein Schnitt durch den Korpus von der Schmalseite mit zurückgesetztem Sockel und den fünf Fachböden als Linien auf ihren Stiftpositionen. Grundriss: die Deckplatte mit der Linie der Vorderkante.')
    if sc == '100':
        s.rect(F, 'side', -0.15, 0.06, 0.3, 1.74, like='side-l', tone=0.85, z=1)
        s.rect(F, 'back', 0.132, 0.07, 0.018, 1.708, like='back', z=2)
        s.rect(F, 'top', -0.15, 1.778, 0.3, 0.022, like='top')
        s.rect(F, 'bottom', -0.15, 0.06, 0.3, 0.022, like='bottom')
        s.rect(F, 'plinth', -0.12, 0.0, 0.25, 0.075, like='plinth')
        for n, y in enumerate(YS, 1):
            s.rect(F, f'shelf-{n}', -0.145, y, 0.277, 0.022, like=f'shelf-{n}-l', bone=f'shelf-{n}')
        s.rect(P, 'top', 0.0, -0.15, 0.8, 0.3, like='top')
        s.poly(P, 'front-edge', [(0.022, -0.128), (0.778, -0.128)], like='shelf-1-l', closed=False, bone='frame', z=7, role='detail', filled=False)
        return ('Front view: the section through the carcass as a pale side panel with the cut top, bottom, back panel and plinth, and the five shelves as boards that ride up and down with their bones. Plan: the top panel with its front edge line.',
                'Vorderansicht: der Schnitt durch den Korpus als helle Seitenwand mit angeschnittenem Deckel, Boden, Rückwand und Sockel und den fünf Fachböden als Bretter, die mit ihren Knochen auf und ab fahren. Grundriss: die Deckplatte mit der Linie der Vorderkante.')
    s.rect(F, 'side', -0.15, 0.06, 0.3, 1.74, like='side-l', tone=0.85, z=1)
    s.rect(F, 'back', 0.132, 0.07, 0.018, 1.712, like='back', z=2)
    s.rect(F, 'top', -0.15, 1.782, 0.3, 0.018, like='top')
    s.rect(F, 'bottom', -0.15, 0.06, 0.3, 0.018, like='bottom')
    s.rect(F, 'plinth', -0.12, 0.0, 0.25, 0.075, like='plinth')
    for n, y in enumerate(YS, 1):
        s.rect(F, f'shelf-{n}', -0.145, y, 0.277, 0.018, like=f'shelf-{n}-l', bone=f'shelf-{n}')
        s.rect(F, f'pin-{n}-front', -0.12, y - 0.012, 0.012, 0.013, like=f'pin-{n}-l', bone=f'shelf-{n}')
        s.rect(F, f'pin-{n}-back', 0.1, y - 0.012, 0.012, 0.013, like=f'pin-{n}-l', bone=f'shelf-{n}')
    s.rect(P, 'top', 0.0, -0.15, 0.8, 0.3, like='side-l', z=6, region='carcass', tone=0.7)
    s.poly(P, 'front-edge', [(0.018, -0.132), (0.782, -0.132)], like='shelf-1-l', closed=False, bone='frame', z=7, role='detail', filled=False)
    s.poly(P, 'back-edge', [(0.018, 0.132), (0.782, 0.132)], like='shelf-1-l', closed=False, bone='frame', z=7, role='detail', filled=False)
    return ('Front view: an end section through the carcass: the pale side panel with the cut top, bottom, back panel and plinth, the five shelves with two pins each that move with their bones. Plan: the top panel seen from above with the lines of the front and back inset.',
            'Vorderansicht: ein Schnitt durch den Korpus von der Schmalseite: die helle Seitenwand mit angeschnittenem Deckel, Boden, Rückwand und Sockel sowie die fünf Fachböden mit je zwei Stiften, die mit ihren Knochen wandern. Grundriss: die Deckplatte von oben mit den Linien der eingerückten Vorder- und Rückkante.')


if __name__ == '__main__':
    run('bookshelf', build)
