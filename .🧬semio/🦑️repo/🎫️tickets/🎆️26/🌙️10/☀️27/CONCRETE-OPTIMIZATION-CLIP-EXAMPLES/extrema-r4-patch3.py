import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()
def swap(a, b):
    global s
    assert s.count(a) == 1, a[:60]
    s = s.replace(a, b)
swap('({"wwr": .8}, self.tr("window share 80 %", "Fensteranteil 80 %"), DESIGN)]', '({"overhang": .6}, self.tr("overhang 0.6 m", "Vordach 0,6 m"), DESIGN)]')
swap('"Another window share moves the optimum\'s value, here not its direction; other changes need a new search.", "Ein anderer Fensteranteil verschiebt den Optimalwert, hier nicht die Richtung; Weiteres braucht eine neue Suche."',
     '"Another window share moves the optimum\'s value; an overhang moves its direction as well.", "Ein anderer Fensteranteil verschiebt den Optimalwert; ein Vordach verschiebt auch die Richtung."')
open(p, "w", encoding="utf-8").write(s)
