import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)


rep("Jeder Slider ist ein Maß:", "Jeder Schieberegler ist ein Maß:")
rep("Zieht man einen Slider, folgt", "Zieht man einen Schieberegler, folgt")
rep("fügt dem Ribbon einen Tab hinzu.", "fügt dem Ribbon einen Reiter hinzu.")
rep("Mit 2,5 statt Text ist sie wieder grau.", "Mit 2.5 statt tall ist sie grau.")
rep("Mit der Zahl 2,5 in Z kehrt die Box zurück.", "Mit dem Text 2.5 in Z kehrt die Box zurück.")
rep('self._rows(_gh.evaluate(series_defn, {"cnt": count()}).tree("sp"))', 'self._rows(_gh.evaluate(series_defn, {"cnt": count()}).tree("sp"), " m")')
rep('''        group = _engine.VGroup(w_slider, l_slider, text, box, panel, bubble, *wires)''', '''        gloss = _engine.VGroup()
        if notes and self.language() == "de":
            german = "Datenumwandlung von Text zu Zahl fehlgeschlagen" if kind == "warning" else "Eingabeparameter Z hat keine Daten erhalten"
            gloss = _engine.VGroup(self._note("Grasshopper-Meldung (englisch)", _at(2.1, -1.2), color=SOFT), self._note(german, _at(2.1, -1.5)))
        group = _engine.VGroup(w_slider, l_slider, text, box, panel, bubble, gloss, *wires)''')
open(p, "w", encoding="utf-8").write(s)
