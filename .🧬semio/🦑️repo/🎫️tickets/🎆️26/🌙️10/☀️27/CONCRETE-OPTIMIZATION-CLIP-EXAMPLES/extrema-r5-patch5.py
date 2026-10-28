import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()
def swap(old, new):
    global s
    assert s.count(old) == 1, old[:60]
    s = s.replace(old, new)
swap('"Turning south, the low winter sun heats the room for free, so heating electricity falls."', '"Turning south, the winter sun heats the room for free: heating falls."')
swap('"Bei 450° zeigt die Fassade wieder nach Osten, daher endet die Kurve am Anfangswert."', '"Bei 450° zeigt die Fassade wieder nach Ost, die Kurve schließt sich."')
swap('"Wir minimieren ihn: Kleiner ist besser. Die Werte stammen aus einem vereinfachten Jahresmodell."', '"Wir minimieren ihn, denn kleiner ist besser. Die Werte stammen aus einem stark vereinfachten Jahresmodell."')
swap('"Now the opposite: how many kWh/a and CHF/a do we save against an east façade?"', '"Now the opposite: how much do we save against an east façade, in kWh/a and CHF/a?"')
swap('"E minimieren heißt die Einsparung S maximieren', '"E minimieren heißt die Einsparung S maximieren') if False else None
swap('"Minimizing E equals maximizing the saving S: valleys become peaks, the curve flips."', '"Minimizing E equals maximizing the saving S: valleys turn into peaks and the curve flips over."')
open(p, "w", encoding="utf-8").write(s)
