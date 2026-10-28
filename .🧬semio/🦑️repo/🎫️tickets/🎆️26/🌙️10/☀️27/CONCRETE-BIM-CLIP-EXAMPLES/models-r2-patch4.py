import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert old in s, old[:80]
    s = s.replace(old, new)


sub('''zoom = 2.0 / (ring_radius + distance + piloti["radius"])''', '''zoom = 2.5 / (ring_radius + distance + piloti["radius"])''')
sub('''mark = np.array([centre[0] - radius + 0.55, middle[1] + 0.55, 0.0])''', '''mark = np.array([centre[0] - radius + 0.37, middle[1] + 0.45, 0.0])''')
sub('''self.text(self.quantity(clash["clearance"], "m"), mark, size=17.0, color=WARN),''', '''self.text(self.quantity(clash["clearance"], "m"), mark, size=16.2, color=WARN),''')
sub('''segments([(mark + (0.34, -0.06, 0), middle + (0.0, 0.0, 0))], WARN, 1.0, 0.8))''', '''segments([(mark + (0.3, -0.08, 0), middle + (0.0, 0.0, 0))], WARN, 1.0, 0.8))''')
p.write_text(s, encoding="utf-8")
print("ok")
