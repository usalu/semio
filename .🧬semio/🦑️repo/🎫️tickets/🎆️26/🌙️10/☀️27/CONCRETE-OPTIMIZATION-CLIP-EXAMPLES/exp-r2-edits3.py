REPLACEMENTS = [
    ('            ghost_top = self._t(self.number(bare["response"], 1), base + UP * (bare["response"] * unit + .2), 13, MUTED, 1.0)\n',
     '            ghost_top = _mn.VGroup(self._t(self._kwh(bare["response"]), (0, 0, 0), 14, MUTED, 1.3), self._t(self.tr("without overhang", "ohne Vordach"), (0, 0, 0), 14, MUTED, 1.5)).arrange(DOWN, buff=.04)\n'
     '            _room.place(ghost_top, base + UP * (bare["response"] * unit + .42), "center")\n'),
    ('self._ghost_bar(base, bare["response"], height=2.9, width=.4), ghost_top,\n                       self._t(self.tr("without overhang", "ohne Vordach"), base + UP * bare["response"] * unit + RIGHT * .95 + UP * .2, 13, MUTED, 1.4),\n',
     'self._ghost_bar(base, bare["response"], height=2.9, width=.4), ghost_top,\n'),
]
