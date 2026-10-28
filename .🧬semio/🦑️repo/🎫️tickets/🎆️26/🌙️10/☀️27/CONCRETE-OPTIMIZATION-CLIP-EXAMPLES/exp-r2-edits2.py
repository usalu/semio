REPLACEMENTS = [
    ('        ghost = ghost() if callable(ghost) else ghost\n', '        ghost = ghost() if callable(ghost) else ghost\n        demand = demand() if callable(demand) else demand\n'),
    ('chart = self._chart(x_range=(0, 1.2, .1), x_ticks=(0, .5, 1.0)', 'chart = self._chart(x_range=(-.2, 1.2, .1), x_ticks=(0, .5, 1.0)'),
    ('        line_label = self._near(self._t(self.tr("straight line through the two levels", "Gerade durch die zwei Stufen"), (0, 0, 0), 14, MUTED, 2.4), axes.c2p(.5, (low + high) / 2), ("upright", "up", "downright"), gap=.3)\n',
     '        key_x, key_y = axes.c2p(.45, 500)[0], axes.c2p(.45, 500)[1] - .12\n'
     '        key_straight = _mn.VGroup(_room.dashed((key_x, key_y, 0), (key_x + .4, key_y, 0), MUTED, 2.6), self._at(self.tr("straight line through the two levels", "Gerade durch die zwei Stufen"), (key_x + .5, key_y, 0), 14, MUTED, 3.0))\n'
     '        key_true = _mn.VGroup(_mn.Line((key_x, key_y - .3, 0), (key_x + .4, key_y - .3, 0), color=DATA, stroke_width=3), self._at(self.tr("simulated rooms at 50 % window", "simulierte Räume bei 50 % Fenster"), (key_x + .5, key_y - .3, 0), 14, DATA, 3.0))\n'),
    ('_mn.FadeIn(end_tags), _mn.FadeIn(line_label), run_time=1.2)', '_mn.FadeIn(end_tags), _mn.FadeIn(key_straight), run_time=1.2)'),
    ('labels=True, ghost=lambda: self.ghost_depth.get_value(), reach=2.4, demand=True, daylight=True)', 'labels=True, ghost=lambda: self.ghost_depth.get_value(), reach=2.4, demand=lambda: grow.get_value() > .02, daylight=True)'),
    ('        true_label = self._near(self._t(self.tr("simulated rooms at 50 % window", "simulierte Räume bei 50 % Fenster"), (0, 0, 0), 14, DATA, 2.4), axes.c2p(.62, self._cooling({**centre["design"], "overhang": .62})), ("upright", "right", "up"), gap=.3, avoid=[[axes.c2p(0, low), axes.c2p(1, high)]])\n', ''),
    ('_mn.FadeIn(trace), _mn.FadeIn(true_label), sweep, seconds=6)', '_mn.FadeIn(trace), _mn.FadeIn(key_true), sweep, seconds=6)'),
    ('self.stage_end = _mn.VGroup(share_note, true, trace, true_label)', 'self.stage_end = _mn.VGroup(share_note, true, trace, key_true)'),
    ('(3.9, 1.65, 0), 16, INK, 6.4),\n                             self._t(self.tr("measured kWh/a"', '(3.9, 1.65, 0), 16, INK, 6.0),\n                             self._t(self.tr("measured kWh/a"'),
]
