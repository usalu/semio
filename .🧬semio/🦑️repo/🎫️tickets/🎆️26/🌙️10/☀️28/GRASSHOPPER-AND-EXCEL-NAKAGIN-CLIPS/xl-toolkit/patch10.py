import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
old = "        rows, x_value = {}, left + 2.05\n"
assert old in src
src = src.replace(old, "        rows, x_value = {}, left + 0.5 + max(_label(ui[key][language], S_SMALL, CELL_INK).width for key in ('address', 'stored', 'shown', 'formula', 'format'))\n")
open(path, "w", encoding="utf-8").write(src)
p2 = sys.argv[2]
s = open(p2, encoding="utf-8").read()
s = s.replace('values[(name, "B9")]', 'values[(name, "B14")]').replace('unit="m²", title=self.tr("Net floor area", "Nettogrundfläche")', 'unit="kWh/a", title=self.tr("Annual heating demand", "Jahresheizwärmebedarf")')
open(p2, "w", encoding="utf-8").write(s)
