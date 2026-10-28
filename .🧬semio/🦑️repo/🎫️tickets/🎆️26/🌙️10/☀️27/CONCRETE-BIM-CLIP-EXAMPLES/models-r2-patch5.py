import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert old in s, old[:80]
    s = s.replace(old, new)


sub('''FadeIn(heads[1]), Indicate(strikes, color=WARN, scale_factor=1.0), FadeIn(same)]''', '''FadeIn(heads[1]), FadeIn(same)]''')
sub('''(self.tr("support: ", "Lagerung: ") + support["fixity"], GOOD)''', '''(self.tr("support: fixed", "Lagerung: eingespannt"), GOOD)''')
sub('''self._counter(kinds, (0.6, -2.3, 0)''', '''self._counter(kinds, (-0.35, -2.3, 0)''')
sub('''(0.75, -2.3, 0), size=17.0, color=INK, anchor="l", width=5.5))''', '''(-0.2, -2.3, 0), size=17.0, color=INK, anchor="l", width=5.5))''')
p.write_text(s, encoding="utf-8")
print("ok")
