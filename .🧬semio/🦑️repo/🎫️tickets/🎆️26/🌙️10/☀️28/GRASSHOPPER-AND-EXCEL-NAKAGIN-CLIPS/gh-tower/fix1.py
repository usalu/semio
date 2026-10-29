import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)


rep('"capsules": ("mv", "cap", "ori", "st", "stl", "stc"),', '"capsules": ("capp", "ori", "st", "stl", "stc"),')
i = s.index('    "planes": ("z0"')
j = s.index("\n", i)
s = s[:i] + s[j + 1:]
rep('''    def _plane(self, plane, extra=(0.0, 0.0, 0.0)):
        """🧭 An engine plane as drawable axes, moved into tower B."""
        shift = np.array([self.B.center[0], self.B.center[1], 0.0]) + np.asarray(extra)
        return''', '''    def _plane(self, plane, shift=(0.0, 0.0, 0.0)):
        """🧭 An engine plane as drawable axes."""
        shift = np.asarray(shift)
        return''')
rep('"cap": (-5.4, 1.2), "xy": (-5.4, -0.4), "mv": (-2.5, 0.7), "ori": (0.2, 0.2)}, group="capsules", bridges=(("cap", "Brep", "ori", "G"),))',
    '"capp": (-5.4, 1.2), "xy": (-5.4, -0.4), "slots": (-2.5, 0.7), "ori": (0.2, 0.2)}, group="capsules")')
rep("def _widget(self, oid, at, sol, over, width=None, selected=False):", "def _widget(self, oid, at, sol, over, width=None, selected=False, lines=None):")
rep("return self.gh_panel(self._lines(tree), at=at,", "return self.gh_panel(lines or self._lines(tree), at=at,")
rep("def _net(self, layout, *, over=None, widths=None, selected=(), bridges=(), group=None):", "def _net(self, layout, *, over=None, widths=None, selected=(), bridges=(), group=None, panels=None):")
rep("self._widget(oid, at, sol, over, widths.get(oid), oid in selected) for", "self._widget(oid, at, sol, over, widths.get(oid), oid in selected, (panels or {}).get(oid)) for")
rep('''        counts = {key: len(MEMBERS[key]) + (len(_gh.canvas("capsule").objects) if key == "capsules" else 0) for key in GROUP_KEYS}''',
    '''        named = {oid for key in MEMBERS for oid in MEMBERS[key]}
        sizes = {**{key: len(MEMBERS[key]) for key in MEMBERS}, "planes": len(self._defn.objects) - len(named)}
        counts = {key: sizes[key] + (len(_gh.canvas("capsule").objects) if key == "capsules" else 0) for key in GROUP_KEYS}''')
rep('''net_live = self.live(lambda: self._net(count_layout, over={"n": int(round(k.get_value()))}, widths={"stl": 1.2}, group="capsules",
                                                bridges=(("ser", "Series", "ori", "B"),)), key=lambda: int(round(k.get_value())))''',
    '''lengths = self._solve().items("st", "Length")
        shown = lambda: int(round(k.get_value()))
        net_live = self.live(lambda: self._net(count_layout, over={"n": shown()}, widths={"stl": 1.2}, group="capsules", bridges=(("ser", "Series", "ori", "Target"),),
                                                panels={"stc": {(0,): [str(shown())]}, "stl": {(0,): [str(v) for v in lengths[:shown()]]}}), key=shown)''')
rep('''f"{int(round(k.get_value()))} × 4 × 2 = {int(round(k.get_value())) * 8}"''', '''f"{int(round(k.get_value()))} × 4 × 2 = {sum(lengths[:int(round(k.get_value()))])}"''')
open(p, "w", encoding="utf-8").write(s)
