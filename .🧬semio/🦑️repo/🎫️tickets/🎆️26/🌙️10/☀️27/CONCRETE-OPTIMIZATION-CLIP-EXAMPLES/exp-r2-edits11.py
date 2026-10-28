REPLACEMENTS = [
    ('def _formula_rows(self, rows, point, *, size=17, width=6.6, align="center", trackers=None):', 'def _formula_rows(self, rows, point, *, size=17, width=6.2, align="center", trackers=None):'),
    ('(0, 0, 0), 14, _room.SUN, 1.9), (centre[0] + 1.5, centre[1] + .9, 0), "right")', '(0, 0, 0), 14, _room.SUN, 2.4), (centre[0] + 1.5, centre[1] + .9, 0), "right")'),
    ('MIXED, 1.2), self._t(self._kwh(end_value - SHOWN[3]), (0, 0, 0), 14, MIXED, 1.2)).arrange(DOWN, buff=.04, aligned_edge=LEFT)', 'MIXED, 1.4), self._t(self._kwh(end_value - SHOWN[3]), (0, 0, 0), 14, MIXED, 1.4)).arrange(DOWN, buff=.08, aligned_edge=LEFT)'),
    ('"? kann sich krümmen"), (0, 0, 0), 16, MIXED, 1.6)', '"? kann sich krümmen"), (0, 0, 0), 16, MIXED, 2.4)'),
    ('"WWR 50 %: Kühlstrom über der Vordachtiefe"), INK, 17), top=1.2)', '"WWR 50 %: Kühlstrom über der Vordachtiefe"), INK, 17), top=1.4)'),
    ('self.tr("same inputs → same output: no scatter, no noise estimate", "gleiche Eingaben → gleiche Ausgabe: keine Streuung, keine Rauschschätzung"), (3.9, 1.65, 0), 16, INK, 6.4)',
     'self.tr("same inputs → same output:\\nno scatter, no noise estimate", "gleiche Eingaben → gleiche Ausgabe:\\nkeine Streuung, keine Rauschschätzung"), (3.9, 1.6, 0), 16, INK, 6.2)'),
    ('(corner[0] - .8, corner[1] + .84, 0), 15, OPEN, 1.9)', '(corner[0] + .2, corner[1] + .43, 0), 15, OPEN, 1.9)'),
    ('(corner[0] - .8, corner[1] + .58, 0), 15, SHADED, 1.9)', '(corner[0] + .2, corner[1] + .2, 0), 15, SHADED, 1.9)'),
    ('        corner = self._at_plot(50, 0)\n', '        corner = self._at_plot(10, YMAX)\n'),
    ('key_x, key_y = self._at_plot(50, 0)[0] - .8, self._at_plot(50, 0)[1] + .32', 'key_x, key_y = self._at_plot(10, YMAX)[0] + .2, self._at_plot(10, YMAX)[1] - .03'),
    ('self._at(self.tr("if the lines were parallel", "wenn die Linien parallel wären"), (key_x + .5, key_y, 0), 15, OPEN, 2.9)', 'self._at(self.tr("if parallel", "wenn parallel"), (key_x + .5, key_y, 0), 15, OPEN, 1.7)'),
]
