import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
old = '''        self.add(fills, self.gridlines)
        self.tint, self.active = self._selection_shapes(self.state["selected"])
        self.add(self.tint)
'''
assert old in s
s = s.replace(old, '''        self.tint, self.active = self._selection_shapes(self.state["selected"])
        self.add(fills, self.tint, self.gridlines)
''')
open(p, "w", encoding="utf-8").write(s)
