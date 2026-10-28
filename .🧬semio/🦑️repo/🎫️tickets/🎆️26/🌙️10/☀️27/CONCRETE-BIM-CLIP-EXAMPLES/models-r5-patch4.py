import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, (s.count(old), old[:100])
    s = s.replace(old, new)


sub('''self.scale_bar((-1.65, -2.4, 0), scale, 5.0, steps=2))''', '''self.scale_bar((-0.45, -2.3, 0), scale, 5.0, steps=2))''')
sub('''    "IfcRamp:element": ("Ramp", "Rampe"), "IfcRampFlight:element": ("Flight", "Lauf"), "IfcSpace:element": ("Space", "Raum"), "IfcSlab.ROOF:element": ("Roof slab", "Dachdecke"),
    "IfcSlab.FLOOR:voids": ("Ramp void", "Rampenloch"), "IfcColumn:Pset_ColumnCommon.FireRating": ("Fire rating", "Feuerwid."), "IfcColumn:Pset_ColumnCommon.IsExternal": ("External", "Außen")}''',
    '''    "IfcRamp:element": ("Ramp", "Rampe"), "IfcRampFlight:element": ("Flight", "Lauf"), "IfcSpace:element": ("Space", "Raum"), "IfcSlab.ROOF:element": ("Roof", "Dach"),
    "IfcSlab.FLOOR:voids": ("Void", "Loch"), "IfcColumn:Pset_ColumnCommon.FireRating": ("Fire", "Feuer"), "IfcColumn:Pset_ColumnCommon.IsExternal": ("External", "Außen")}''')
sub('''"IfcOpeningElement:element": ("Opening", "Aussparung")''', '''"IfcOpeningElement:element": ("Opening", "Öffnung")''')
sub('''"IfcCurtainWall:element": ("Glass wall", "Glaswand")''', '''"IfcCurtainWall:element": ("Glass", "Glas")''')
sub('''        base, pitch, unit = -0.5, 4.5 / len(groups), 0.02''', '''        base, pitch, unit = -0.5, 3.8 / len(groups), 0.014''')
sub('''            x = -2.2 + pitch * (index + 0.5)''', '''            x = -2.05 + pitch * (index + 0.5)''')
sub('''Rectangle(width=0.2, height=max(group["count"] * unit * fill.get_value(), 1e-3)''', '''Rectangle(width=0.17, height=max(group["count"] * unit * fill.get_value(), 1e-3)''')
sub('''            name.scale_to_fit_height(min(name.height, 0.8))''', '''            name.scale_to_fit_height(min(name.height, 0.78))''')

# services second line
sub('''                     self.label(f"its floor drains are not modelled; the {drains} roof drains belong to the roof level", f"seine Bodenabläufe sind nicht modelliert; die {drains} Dachabläufe gehören zur Dachebene",
                                (1.1, -0.78, 0), size=15.5, color=MUTED, anchor="l", width=5.4))''',
    '''                     *self._lines([self.tr("its floor drains are not modelled;", "seine Bodenabläufe sind nicht modelliert;"),
                                   self.tr(f"the {drains} roof drains belong to the roof level", f"die {drains} Dachabläufe gehören zur Dachebene")], (1.1, -0.76, 0), size=15.5, color=MUTED, pitch=0.22, width=5.4))''')
sub('''(0.95, -1.3, 0), width=5.6)
        pipes''', '''(0.95, -1.5, 0), width=5.6)
        pipes''')

# check card layout
sub('''(RIGHT, -0.95, 0), size=18.0, color=INK, width=5.9),''', '''(RIGHT, -0.92, 0), size=18.0, color=INK, width=5.9),''')
sub('''f"Abstand {clearance} m < gefordert {required} m{STAR} · es fehlen {short} m"), (RIGHT, -1.28, 0)''', '''f"Abstand {clearance} m < gefordert {required} m{STAR} · es fehlen {short} m"), (RIGHT, -1.2, 0)''')
sub('''(RIGHT, -1.58, 0), size=16.2, color=MUTED, width=5.9),
                      self._legend((RIGHT - 2.95, -2.47, 0), assumed=True))''', '''(RIGHT, -1.45, 0), size=16.2, color=MUTED, width=5.9),
                      self._legend((RIGHT - 2.95, -1.69, 0), assumed=True))''')
sub('''self.guid_chip(clash["a_tag"], (1.95, -2.0, 0), color=SERV)''', '''self.guid_chip(clash["a_tag"], (1.95, -2.17, 0), color=SERV)''')
sub('''self.guid_chip(clash["b_tag"], (5.15, -2.0, 0), color=STRUCT)''', '''self.guid_chip(clash["b_tag"], (5.15, -2.17, 0), color=STRUCT)''')

# midsurface reach
sub('''zoom, cx, cy, reach, bottom = 1.6, 3.9, 0.45, 1.0, soffit - 0.8''', '''zoom, cx, cy, reach, bottom = 1.6, 3.9, 0.45, 0.85, soffit - 0.8''')

p.write_text(s, encoding="utf-8")
print("part 4 ok")
